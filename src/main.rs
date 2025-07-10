mod term_print;

use std::env;
use term_print::ansi;

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
        println!("{}Boot -- Your Personal Project Manager CLI Tool {}", ansi::BOLD, ansi::RESET);
        println!("Run {} boot help {} to get info on commands", ansi::ITALIC, ansi::RESET);
    }
    // == 1 ARGUMENT
    // == 2+ ARGUMENTS
}
