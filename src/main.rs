mod term_print;
mod cmds;

use std::env::set_current_dir;
use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use term_print::ansi;
use cmds::{boot, fetch_cmd};

const VERSION: &str = "0.1.0";
const BOOT_DIR: &str = "/Users/donti/boot-rs/";
const BOOT_PROJECTS_DIR: &str = "/Users/donti/boot-rs/projects/";
const GITHUB_LINK: &str = "https://github.com/stelleron/";


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
        // Basic
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
        boot::Command::Help => {
            println!("{}{}{}Boot - List of Commands{}", ansi::GREEN, ansi::BOLD, ansi::UNDERLINE, ansi::RESET);

            println!("{}{}Basic Commands{}", ansi::BOLD, ansi::UNDERLINE, ansi::RESET);
            println!("{}help{}: Show this information", ansi::BOLD, ansi::RESET);
            println!("{}version{}: Print current version", ansi::BOLD, ansi::RESET);
            println!("{}update{}: Recompile Boot", ansi::BOLD, ansi::RESET);
            println!("{}selfopen{}: Open this in VSCode", ansi::BOLD, ansi::RESET);

            println!("\n{}{}Project Commands{}", ansi::BOLD, ansi::UNDERLINE, ansi::RESET);
            println!("{}new{}: Create a new C/C++ project", ansi::BOLD, ansi::RESET);
            println!("{}open{}: Open a project in VSCode", ansi::BOLD, ansi::RESET);
            println!("{}delete{}: Delete a project", ansi::BOLD, ansi::RESET);
            println!("{}listproj{}: List all stored projects", ansi::BOLD, ansi::RESET);

            println!("\n{}{}Build Commands{}", ansi::BOLD, ansi::UNDERLINE, ansi::RESET);
            println!("{}build{}: Build your current C/C++ project", ansi::BOLD, ansi::RESET);
            println!("{}run{}: Run your current C/C++ project", ansi::BOLD, ansi::RESET);
            println!("{}test{}: Build + Run your project", ansi::BOLD, ansi::RESET);

            println!("\n{}{}Git Commands{}", ansi::BOLD, ansi::UNDERLINE, ansi::RESET);
            println!("{}commit{}: Commit to Git and push changes", ansi::BOLD, ansi::RESET);
            println!("{}clone{}: Clone a project from your GitHub", ansi::BOLD, ansi::RESET);
        }
        boot::Command::Version => {
            println!("{}Boot Version {}{}", ansi::BOLD, VERSION, ansi::RESET);
        }
        boot::Command::Update => {
            set_current_dir(BOOT_DIR).expect("Unable to set current directory!");
            Command::new("cargo")
                    .arg("build")
                    .status()
                    .expect("Unable to recompile Boot!");
        }
        boot::Command::SelfOpen => {
            set_current_dir(BOOT_DIR).expect("Unable to set current directory!");
            Command::new("code")
                    .arg(".")
                    .status()
                    .expect("Unable to open Boot!");
        }
        // Project
        boot::Command::Create { name } => {
            set_current_dir(BOOT_PROJECTS_DIR).expect("Unable to set current directory!");

            fs::create_dir(&name).expect("Unable to create new project!");
            set_current_dir(&name).expect("Unable to set current directory!");

            let mut bootfile = fs::File::create_new(".boot").expect("Error in creating file!");
            writeln!(bootfile, "./build/{}", name).expect("Failed to write to file");

            fs::File::create_new("premake5.lua").expect("Error in creating files!");
            fs::File::create_new(".gitignore").expect("Error in creating files!");

            fs::create_dir("src").expect("Error in creating directory!");
            fs::File::create_new("src/main.cpp").expect("Error in creating files!");
            fs::create_dir("external").expect("Error in creating directory!");
            fs::create_dir("external/include").expect("Error in creating directory!");
            fs::create_dir("external/lib").expect("Error in creating directory!");
            fs::create_dir("build").expect("Error in creating directory!");

            Command::new("git")
                    .arg("init")
                    .status()
                    .expect("Unable to initialize Git!");

            Command::new("code")
                    .arg(".")
                    .status()
                    .expect("Unable to open your project!");
        }
        boot::Command::Open { name } => {
            set_current_dir(BOOT_PROJECTS_DIR).expect("Unable to set current directory!");
            Command::new("code")
                    .arg(&name)
                    .status()
                    .expect("Unable to open your project!");
        }
        // Git
        boot::Command::Commit { message } => {
            // git add .
            let status = Command::new("git")
                .arg("add")
                .arg(".")
                .status()
                .expect("Failed to run git add");
            assert!(status.success(), "git add failed");

            // git commit -m "message"
            let status = Command::new("git")
                .arg("commit")
                .arg("-m")
                .arg(message)
                .status()
                .expect("Failed to run git commit");
            assert!(status.success(), "git commit failed");

            // git push
            let status = Command::new("git")
                .arg("push")
                .status()
                .expect("Failed to run git push");
            assert!(status.success(), "git push failed");
        }
        boot::Command::Clone { repo } => {
            set_current_dir(BOOT_PROJECTS_DIR).expect("Unable to set current directory!");
            let mut repo_str = GITHUB_LINK.to_string();
            repo_str.push_str(&repo);
            Command::new("git")
                    .arg("clone")
                    .arg(repo_str)
                    .status()
                    .expect("Failed to clone repo!");

        }
        // Errors
        boot::Command::Invalid => {
            println!("{}{}Error: Command not valid!{}", ansi::BOLD, ansi::RED, ansi::RESET);
        }
        boot::Command::IncorrectUsage => {
            println!("{}{}Error: Incorrect usage!{}", ansi::BOLD, ansi::RED, ansi::RESET);
        }
        _other => {
            println!("{}{}Error: Not implemented yet!{}", ansi::BOLD, ansi::RED, ansi::RESET);
        }
    };
}
