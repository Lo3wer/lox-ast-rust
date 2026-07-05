use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::evaluator::Evaluator;
use crate::ast_printer::AstPrinter;
use crate::token::{Token, TokenType};
use crate::values::Literal;
use std::fs;
use std::io::{self, BufRead, Write};
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

    pub fn error_token(&mut self, token: &Token, message: &str) {
        if token.token_type() == TokenType::Eof {
            self.report(token.line(), " at end", message);
        } else {
            self.report(token.line(), &format!(" at '{}'", token.lexeme()), message);
        }
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
        let tokens = lexer.scan_tokens(self);
        let mut parser = Parser::new(tokens);
        let expression = match parser.parse(self) {
            Ok(expression) => expression,
            Err(_) => return,
        };

        if self.had_error {
            return;
        }

        let evaluator = Evaluator::new();
        let result = evaluator.interpret(&expression);
        if let Some(literal) = result {
            let literal_string = match literal {
                crate::values::Literal::Number(n) => n.to_string(),
                crate::values::Literal::String(s) => s,
                crate::values::Literal::Bool(b) => b.to_string(),
                crate::values::Literal::Nil => "nil".to_string(),
            };
            println!("{}", literal_string);
        }
    }
}

pub trait ErrorReporter {
    fn error(&mut self, line: usize, message: &str);
    fn error_token(&mut self, token: &Token, message: &str);
}

impl ErrorReporter for Lox {
    fn error(&mut self, line: usize, message: &str) {
        Lox::error(self, line, message);
    }

    fn error_token(&mut self, token: &Token, message: &str) {
        Lox::error_token(self, token, message);
    }
}