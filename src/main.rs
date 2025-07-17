mod term_print;
mod cmds;

use std::{default, env};
use term_print::ansi;
use cmds::{boot, fetch_cmd};

fn main() {
    // Receive arguments
    let args: Vec<String> = env::args().collect();
    let cmd = fetch_cmd(args);
    match cmd {
        boot::Command::Home => {
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
        boot::Command::Invalid => {
            println!("{}{}Error: Command not valid!{}", ansi::BOLD, ansi::RED, ansi::RESET);
        }
        other => {

        }
    };
}
