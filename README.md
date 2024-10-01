# Quik Text Editor

## Project Status

### Done
- Navigation
- File Handling
- File Status

### Work in Progress
- Search
- Syntax Highlighting

## Overview
Quik is a terminal-based text editor developed in Rust, designed to provide a minimal yet functional editing experience. Quik implements basic features expected from a text editor, along with enhancements like syntax highlighting and a search functionality.

## Table of Contents
- [Features](#features)
- [Getting Started](#getting-started)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Features
- **Minimal Interface**: A clean and straightforward interface for efficient text editing.
- **Syntax Highlighting**: Supports basic syntax highlighting for various programming languages.
- **Search Functionality**: Allows users to search for specific text within their documents.
- **Cross-Platform**: Built using Rust, Quik runs on any platform that supports Rust.

## Getting Started
To get started with Quik, you'll need to have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

### Prerequisites
- Rust
- Cargo (comes with Rust installation)

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/pavanmanishd/quik.git
   cd quik
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the editor:
   ```bash
   ./target/release/quik
   ```

## Usage
Once you have Quik running, you can:
- Open a file by passing the filename as an argument: 
  ```bash
  ./target/release/quik myfile.txt
  ```

- Use the arrow keys to navigate and start editing your text.
- Press `Ctrl + Q` to quit the editor.

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE.md) file for details.
