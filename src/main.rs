use std::env;

fn main() {
    // Receive arguments
    let args: Vec<String> = env::args().collect();

    // Handle additional arguments
    // == 0 ARGUMENTS
    if args.len() == 1 {
        println!(r#"  ____              _   "#);
        println!(r#" |  _ \            | |  "#);
        println!(r#" | |_) | ___   ___ | |_ "#);
        println!(r#" |  _ < / _ \ / _ \| __|"#);
        println!(r#" | |_) | (_) | (_) | |_ "#);
        println!(r#" |____/ \___/ \___/ \__|"#);
    }
    // == 1 ARGUMENT
    // == 2+ ARGUMENTS
}
