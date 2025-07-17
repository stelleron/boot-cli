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
        // Errors
        IncorrectUsage,
        Invalid,
    }
}

fn expect_second_arg<F>(args: &Vec<String>, f: F) -> boot::Command
where
    F: Fn(String) -> boot::Command,
{
    args.get(2)
        .cloned()
        .map(f)
        .unwrap_or(boot::Command::IncorrectUsage)
}


pub fn fetch_cmd(args: Vec<String>) -> boot::Command {
    if args.len() < 2 {
        return boot::Command::Home;
    }
    match args[1].as_str() {
        // Basic
        "help" => boot::Command::Help,
        "version" => boot::Command::Version,
        "update" => boot::Command::Update,
        "selfopen" => boot::Command::SelfOpen,
        // Projects
        "create" => expect_second_arg(&args, |name| boot::Command::Create { name: name }),
        "open" => expect_second_arg(&args, |name| boot::Command::Open { name: name }),
        "delete" => expect_second_arg(&args, |name| boot::Command::Delete { name: name }),
        "listproj" => boot::Command::ListProjects,
        // Build
        "run" => boot::Command::Run,
        "build" => boot::Command::Build,
        "test" => boot::Command::Test,
        // Git
        "commit" => expect_second_arg(&args, |msg| boot::Command::Commit { message: msg }),
        "clone" => expect_second_arg(&args, |repo| boot::Command::Clone { repo: repo }),
        // Invalid
        _ => boot::Command::Invalid,
    }
}