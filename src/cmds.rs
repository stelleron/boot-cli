use std::process::Command;

#[allow(unused)]
pub mod boot {
    pub enum Command {
        // Basic commands
        Home,
        Help,
        Version,
        Update,
        SelfOpen,
        // Project commands
        Create{name: String},
        Open{name: String},
        Delete{name: String},
        ListProjects,
        // Build commands
        Run,
        Build,
        Test,
        // Git commands
        Commit{message: String},
        Clone{repo: String},
        // Invalid
        Invalid,
    }
}

pub fn fetch_cmd(args: Vec<String>) -> boot::Command {
    if args.len() < 2 {
        return boot::Command::Home;
    }
    match args[1].as_str() {
        _ => boot::Command::Invalid,
    }
}