mod term_print;
mod cmds;

use std::{env};
use std::path::PathBuf;
use std::process::Command;

use term_print::ansi;
use cmds::{boot, fetch_cmd};



fn get_curr_path() -> Option<PathBuf> {
    env::current_dir().ok()
}

fn get_exec_path() -> Option<PathBuf> {
    env::current_exe().ok()
}

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
            println!("Run {}boot help{} to get info on commands", ansi::ITALIC, ansi::RESET);
        }
        // Git
        boot::Command::Commit { message } => {

        }
        // Errors
        boot::Command::Invalid => {
            println!("{}{}Error: Command not valid!{}", ansi::BOLD, ansi::RED, ansi::RESET);
        }
        boot::Command::IncorrectUsage => {
            println!("{}{}Error: Incorrect usage!{}", ansi::BOLD, ansi::RED, ansi::RESET)
        }
        _other => {
            println!("[Stable] OK")
        }
    };
}
