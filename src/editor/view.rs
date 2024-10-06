use std::{cmp::min, io::Error};

use super::{
    command::{Edit, Move},
    Col, DocumentStatus, Line, Position, Row, Size, Terminal, UIComponent, NAME, VERSION,
};
mod buffer;
use buffer::Buffer;
mod location;
use location::Location;
mod fileinfo;
use fileinfo::FileInfo;
mod searchinfo;
use searchinfo::SearchInfo;

#[derive(Default, Eq, PartialEq, Clone, Copy)]
pub enum SearchDirection {
    #[default]
    Forward,
    Backward,
}

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    text_location: Location,
    scroll_offset: Position,
    search_info: Option<SearchInfo>,
}

impl View {
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            total_lines: self.buffer.height(),
            current_line_idx: self.text_location.line_idx,
            file_name: format!("{}", self.buffer.file_info),
            is_modified: self.buffer.dirty,
        }
    }

    pub const fn is_file_loaded(&self) -> bool {
        self.buffer.is_file_loaded()
    }

    pub fn enter_search(&mut self) {
        self.search_info = Some(SearchInfo {
            prev_location: self.text_location,
            prev_scroll_offset: self.scroll_offset,
            query: None,
        });
    }
    pub fn exit_search(&mut self) {
        self.search_info = None;
    }
    pub fn dismiss_search(&mut self) {
        if let Some(search_info) = &self.search_info {
            self.text_location = search_info.prev_location;
            self.scroll_offset = search_info.prev_scroll_offset;
            self.scroll_text_location_into_view(); // ensure the previous location is still visible even if the terminal has been resized during search.
        }
        self.search_info = None;
    }

    pub fn search(&mut self, query: &str) {
        if let Some(search_info) = &mut self.search_info {
            search_info.query = Some(Line::from(query));
        }
        self.search_in_direction(self.text_location, SearchDirection::default());
    }

    // Attempts to get the current search query - for scenarios where the search query absolutely must be there.
    // Panics if not present in debug, or if search info is not present in debug
    // Returns None on release.
    fn get_search_query(&self) -> Option<&Line> {
        let query = self
            .search_info
            .as_ref()
            .and_then(|search_info| search_info.query.as_ref());

        debug_assert!(
            query.is_some(),
            "Attempting to search with malformed searchinfo present"
        );
        query
    }

    fn search_in_direction(&mut self, from: Location, direction: SearchDirection) {
        if let Some(location) = self.get_search_query().and_then(|query| {
            if query.is_empty() {
                None
            } else if direction == SearchDirection::Forward {
                self.buffer.search_forward(query, from)
            } else {
                self.buffer.search_backward(query, from)
            }
        }) {
            self.text_location = location;
            self.center_text_location();
        };
    }
    pub fn search_next(&mut self) {
        let step_right = self
            .get_search_query()
            .map_or(1, |query| min(query.grapheme_count(), 1));

        let location = Location {
            line_idx: self.text_location.line_idx,
            grapheme_idx: self.text_location.grapheme_idx.saturating_add(step_right), //Start the new search behind the current match
        };
        self.search_in_direction(location, SearchDirection::Forward);
    }
    pub fn search_prev(&mut self) {
        self.search_in_direction(self.text_location, SearchDirection::Backward);
    }

    pub fn load(&mut self, file_name: &str) -> Result<(), Error> {
        let buffer = Buffer::load(file_name)?;
        self.buffer = buffer;
        self.set_needs_redraw(true);
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.buffer.save()
    }
    pub fn save_as(&mut self, file_name: &str) -> Result<(), Error> {
        self.buffer.save_as(file_name)
    }


    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Edit::Insert(character) => self.insert_char(character),
            Edit::Delete => self.delete(),
            Edit::DeleteBackward => self.delete_backward(),
            Edit::InsertNewline => self.insert_newline(),
        }
    }
    pub fn handle_move_command(&mut self, command: Move) {
        let Size { height, .. } = self.size;
        // This match moves the positon, but does not check for all boundaries.
        // The final boundarline checking happens after the match statement.
        match command {
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::PageUp => self.move_up(height.saturating_sub(1)),
            Move::PageDown => self.move_down(height.saturating_sub(1)),
            Move::StartOfLine => self.move_to_start_of_line(),
            Move::EndOfLine => self.move_to_end_of_line(),
        }
        self.scroll_text_location_into_view();
    }

    fn insert_newline(&mut self) {
        self.buffer.insert_newline(self.text_location);
        self.handle_move_command(Move::Right);
        self.set_needs_redraw(true);
    }
    fn delete_backward(&mut self) {
        if self.text_location.line_idx != 0 || self.text_location.grapheme_idx != 0 {
            self.handle_move_command(Move::Left);
            self.delete();
        }
    }
    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.set_needs_redraw(true);
    }
    fn insert_char(&mut self, character: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);
        self.buffer.insert_char(character, self.text_location);
        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            //move right for an added grapheme (should be the regular case)
            self.handle_move_command(Move::Right);
        }
        self.set_needs_redraw(true);
    }


    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);
        // hide the welcome message if it doesn't fit entirely.
        if remaining_width < len {
            return "~".to_string();
        }
        format!("{:<1}{:^remaining_width$}", "~", welcome_message)
    }


    fn scroll_vertically(&mut self, to: Row) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }
    fn scroll_horizontally(&mut self, to: Col) {
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        if offset_changed {
            self.set_needs_redraw(true);
        }
    }
    fn scroll_text_location_into_view(&mut self) {
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }
    fn center_text_location(&mut self) {
        let Size { height, width } = self.size;
        let Position { row, col } = self.text_location_to_position();
        let vertical_mid = height.div_ceil(2);
        let horizontal_mid = width.div_ceil(2);
        self.scroll_offset.row = row.saturating_sub(vertical_mid);
        self.scroll_offset.col = col.saturating_sub(horizontal_mid);
        self.set_needs_redraw(true);
    }


    pub fn caret_position(&self) -> Position {
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        let row = self.text_location.line_idx;
        debug_assert!(row.saturating_sub(1) <= self.buffer.lines.len());
        let col = self
            .buffer
            .lines
            .get(row)
            .map_or(0, |line| line.width_until(self.text_location.grapheme_idx));
        Position { col, row }
    }



    fn move_up(&mut self, step: usize) {
        self.text_location.line_idx = self.text_location.line_idx.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }
    fn move_down(&mut self, step: usize) {
        self.text_location.line_idx = self.text_location.line_idx.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }
    // clippy::arithmetic_side_effects: This function performs arithmetic calculations
    // after explicitly checking that the target value will be within bounds.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_right(&mut self) {
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_idx < line_width {
            self.text_location.grapheme_idx += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }
    // clippy::arithmetic_side_effects: This function performs arithmetic calculations
    // after explicitly checking that the target value will be within bounds.
    #[allow(clippy::arithmetic_side_effects)]
    fn move_left(&mut self) {
        if self.text_location.grapheme_idx > 0 {
            self.text_location.grapheme_idx -= 1;
        } else if self.text_location.line_idx > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }
    fn move_to_start_of_line(&mut self) {
        self.text_location.grapheme_idx = 0;
    }
    fn move_to_end_of_line(&mut self) {
        self.text_location.grapheme_idx = self
            .buffer
            .lines
            .get(self.text_location.line_idx)
            .map_or(0, Line::grapheme_count);
    }

    // Ensures self.location.grapheme_idx points to a valid grapheme index by snapping it to the left most grapheme if appropriate.
    // Doesn't trigger scrolling.
    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_idx = self
            .buffer
            .lines
            .get(self.text_location.line_idx)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_idx)
            });
    }
    // Ensures self.location.line_idx points to a valid line index by snapping it to the bottom most line if appropriate.
    // Doesn't trigger scrolling.
    fn snap_to_valid_line(&mut self) {
        self.text_location.line_idx = min(self.text_location.line_idx, self.buffer.height());
    }

}

impl UIComponent for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }
    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, origin_row: usize) -> Result<(), Error> {
        let Size { height, width } = self.size;
        let end_y = origin_row.saturating_add(height);
        let top_third = height.div_ceil(3);
        let scroll_top = self.scroll_offset.row;
        for current_row in origin_row..end_y {
            // to get the correct line index, we have to take current_row (the absolute row on screen),
            // subtract origin_row to get the current row relative to the view (ranging from 0 to self.size.height)
            // and add the scroll offset.
            let line_idx = current_row
                .saturating_sub(origin_row)
                .saturating_add(scroll_top);
            if let Some(line) = self.buffer.lines.get(line_idx) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right))?;
            } else if current_row == top_third && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        Ok(())
    }
}