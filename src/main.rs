use std::env;

#[allow(unused)]
mod ansi {
    pub const RESET: &str     = "\x1b[0m";
    pub const BOLD: &str      = "\x1b[1m";
    pub const ITALIC: &str    = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";

    pub const BLACK: &str     = "\x1b[30m";
    pub const RED: &str       = "\x1b[31m";
    pub const GREEN: &str     = "\x1b[32m";
    pub const YELLOW: &str    = "\x1b[33m";
    pub const BLUE: &str      = "\x1b[34m";
    pub const MAGENTA: &str   = "\x1b[35m";
    pub const CYAN: &str      = "\x1b[36m";
    pub const WHITE: &str     = "\x1b[37m";
}
pub use ansi::{BOLD, RESET, ITALIC};

fn main() {
    // Receive arguments
    let args: Vec<String> = env::args().collect();

    // Handle additional arguments
    // == 0 ARGUMENTS
    if args.len() == 1 {
        println!(r"  ____                    __      ");
        println!(r" /\  _`\                 /\ \__   ");
        println!(r" \ \ \L\ \    ___     ___\ \ ,_\  ");
        println!(r"  \ \  _ <'  / __`\  / __`\ \ \/  ");
        println!(r"   \ \ \L\ \/\ \L\ \/\ \L\ \ \ \_ ");
        println!(r"    \ \____/\ \____/\ \____/\ \__\");
        println!(r"     \/___/  \/___/  \/___/  \/__/");
        println!("{BOLD}Boot -- Your Personal Project Manager CLI Tool {RESET}");
        println!("Run {ITALIC} boot help {RESET} to get info on commands");
    }
    // == 1 ARGUMENT
    // == 2+ ARGUMENTS
}
