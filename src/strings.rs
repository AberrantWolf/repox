use crossterm::style::{Color, Stylize};
use terminal_emoji::Emoji;

use crate::AppArgs;

pub struct Prefixes {
    pub stdout: String,
    pub stderr: String,
    pub info: String,
    pub warn: String,
    pub error: String,
    pub debug: String,
    pub trace: String,
}

fn wrap_string(s: &str, color: Color, emoji: Emoji, boring: bool) -> String {
    if boring {
        format!("[{}]", s)
    } else {
        format!("[{}{}]", s.with(color), emoji)
    }
}

impl Prefixes {
    pub fn new(args: &AppArgs) -> Self {
        Self {
            stdout: wrap_string("STDOUT ", Color::Cyan, Emoji("💬", ""), args.no_fun),
            stderr: wrap_string("STDERR ", Color::Red, Emoji("🚫", ""), args.no_fun),
            info: wrap_string("INFO   ", Color::Green, Emoji("ℹ️", ""), args.no_fun),
            warn: wrap_string("WARNING", Color::Yellow, Emoji("⚠️", ""), args.no_fun),
            error: wrap_string("ERROR  ", Color::Magenta, Emoji("❌", ""), args.no_fun),
            debug: wrap_string("DEBUG  ", Color::Blue, Emoji("🐞", ""), args.no_fun),
            trace: wrap_string("TRACE  ", Color::DarkGrey, Emoji("🕵️", ""), args.no_fun),
        }
    }
}
