use std::io::{self, Read};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

pub struct Editor {

}


impl Editor {
    pub fn default() -> Self {
        Editor{}
    }

    pub fn run(&self) {
        enable_raw_mode().unwrap();
        for b in io::stdin().bytes() {
                match b {
                    Ok(b) => {
                        let c = b as char;
                        if c.is_control() {
                            println!("Binary: {0:08b} Ascii: {0:#03}\r",b);
                        } else {
                            println!("Binary: {0:08b} Ascii: {0:#03} Character: {1:#?}\r",b,c);
                        }
                        if c == 'q' {
                            disable_raw_mode().unwrap();
                            break;
                        }
                    },
                    Err(err) => println!("Error : {}",err)
                }
        }
    }
}