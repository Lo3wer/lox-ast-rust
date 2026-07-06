use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::evaluator::Evaluator;
use crate::token::TokenType;
use crate::errors::{LexError, ParseError, RuntimeError};
use std::fs;
use std::io::{self, BufRead, Write};
use std::process;

pub struct Lox {
    had_error: bool,
    had_runtime_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Lox { had_error: false, had_runtime_error: false }
    }

    fn report(&mut self, line: usize, where_: &str, message: &str) {
        eprintln!("[line {line}] Error{where_}: {message}");
        self.had_error = true;
    }

    fn report_lex_error(&mut self, error: &LexError) {
        self.report(error.line, "", &error.message);
    }

    fn report_parse_error(&mut self, error: &ParseError) {
        let where_ = if error.token.token_type() == TokenType::Eof {
            " at end".to_string()
        } else {
            format!(" at '{}'", error.token.lexeme())
        };
        self.report(error.token.line(), &where_, &error.message);
    }

    fn report_runtime_error(&mut self, error: &RuntimeError) {
        self.report(error.token.line(), "", &error.message);
        self.had_runtime_error = true;
    }

    pub fn run_file(&mut self, path: &str) -> io::Result<()> {
        let contents = fs::read_to_string(path)?;
        self.run(&contents);
        if self.had_error {
            process::exit(65);
        }
        if self.had_runtime_error {
            process::exit(70);
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
            self.had_error = false;
            self.had_runtime_error = false;
        }
        Ok(())
    }

    fn run(&mut self, source: &str) {
        let mut lexer = Lexer::new(source.to_string());
        let (tokens, lex_errors) = lexer.scan_tokens();
        for error in &lex_errors {
            self.report_lex_error(error);
        }
        if !lex_errors.is_empty() {
            return;
        }

        let mut parser = Parser::new(tokens);
        let expression = match parser.parse() {
            Ok(expression) => expression,
            Err(error) => {
                self.report_parse_error(&error);
                return;
            }
        };

        let evaluator = Evaluator::new();
        match evaluator.interpret(&expression) {
            Ok(value) => println!("{}", value),
            Err(error) => self.report_runtime_error(&error),
        }
    }
}
