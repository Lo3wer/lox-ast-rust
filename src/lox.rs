// use crate::lexer::Lexer;
use std::fs;
use std::io::{self, Write, BufRead};
use std::process;

pub struct Lox {
    had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false }
    }

    pub fn run_file(&mut self, path: &str) -> io::Result<()> {
        let contents = fs::read_to_string(path)?;
        self.run(&contents);
        if self.had_error {
            process::exit(65);
        }
        Ok(())
    }

    pub fn run_prompt(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut line = String::new();

        loop {
            print!("> ");
            io::stdout().flush()?;
            line.clear();
            if stdin.lock().read_line(&mut line)? == 0 {
                break;
            }
            self.run(line.trim_end());
            self.had_error = false; // reset per line, like the book does
        }
        Ok(())
    }

    fn run(&mut self, source: &str) {
        println!("{}", source);
        // later: scan, parse, and set self.had_error = true on failure
    }

    fn error(&mut self, line: usize, message: &str) {
        eprintln!("[line {}] Error: {}", line, message);
        self.had_error = true;
    }
}