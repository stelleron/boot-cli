// For styling terminal output
#[allow(unused)]
pub mod ansi {
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
