use crate::lexer::Lexer;
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

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
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
        let mut lexer = Lexer::new(source.to_string());
        let tokens = lexer.scan_tokens(|line, message| self.error(line, message));

        for token in tokens {
            println!("{}", token);
        }
    }
}