use std::io::{self, BufRead, Write};

const STRINGS_WELCOME: &str = "ChatGPT CLI v0.00\nType 'help' to see commands";
const STRINGS_HELP: &str = "ChatGPT CLI help page\nquit - close application";
const STRINGS_UNKNOWN: &str = "Unknown command";

fn write_line(msg: &str) {
    io::stdout()
        .lock()
        .write_all((msg.to_owned() + "\n").as_bytes())
        .unwrap();
}

fn read_line() -> String {
    let mut input = String::with_capacity(256);
    io::stdin().lock().read_line(&mut input).unwrap();
    input.truncate(input.trim().len());
    input
}

fn main() {
    write_line(STRINGS_WELCOME);
    loop {
        let input = read_line();
        match input.as_str() {
            "help" => write_line(STRINGS_HELP),
            "quit" => break,
            _ => write_line(STRINGS_UNKNOWN),
        }
    }
}
