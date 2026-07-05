use crate::token::{Token, TokenType};
use crate::values::Literal;
use crate::expr::Expr;
use crate::lox::ErrorReporter;

#[derive(Debug, Clone, Copy)]
pub struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        self.expression(reporter)
    }

    // Grammar rules based on the Lox language specification with ternary and comma operators
    // expression     → comma ;
    // comma          → ternary ( "," ternary )* ;
    // ternary        → equality ( "?" expression ":" ternary )? ;
    // equality       → comparison ( ( "!=" | "==" ) comparison )* ;
    // comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    // term           → factor ( ( "-" | "+" ) factor )* ;
    // factor         → unary ( ( "/" | "*" ) unary )* ;
    // unary          → ( "!" | "-" ) unary
    //                | primary ;
    // primary        → NUMBER | STRING | "true" | "false" | "nil"
    //                | "(" expression ")"
    // -- below functions are not implemented for primary, error handling is still general --
    //                | ( "!=" | "==" ) comparison
    //                | ( ">" | ">=" | "<" | "<=" ) term
    //                | ( "+" ) factor
    //                | ( "/" | "*" ) unary ;

    // Grammar entry point
    fn expression<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        self.comma(reporter)
    }

    fn comma<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        let mut expr = self.ternary(reporter)?;

        while self.match_token(&[TokenType::Comma]) {
            let operator = self.previous().clone();
            let right = self.ternary(reporter)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn ternary<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        let mut expr = self.equality(reporter)?;

        if self.match_token(&[TokenType::Question]) {
            let then_branch = self.expression(reporter)?;
            self.consume(TokenType::Colon, "Expect ':' after then branch of ternary expression.", reporter)?;
            let else_branch = self.ternary(reporter)?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            };
        }

        Ok(expr)
    }

    // Binary precedence levels
    fn equality<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        let mut expr = self.comparison(reporter)?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison(reporter)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        let mut expr = self.term(reporter)?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term(reporter)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        let mut expr = self.factor(reporter)?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor(reporter)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        let mut expr = self.unary(reporter)?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary(reporter)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Unary and primary expressions
    fn unary<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary(reporter)?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary(reporter)
    }

    fn primary<R>(&mut self, reporter: &mut R) -> Result<Expr, ParseError>
    where
        R: ErrorReporter,
    {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal { value: Some(Literal::Bool(false)) });
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal { value: Some(Literal::Bool(true)) });
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal { value: Some(Literal::Nil) });
        }
        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let literal = match self.previous().literal() {
                Some(lit) => lit.clone(),
                None => panic!("Expected a literal value."),
            };
            return Ok(Expr::Literal { value: Some(literal) });
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression(reporter)?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.", reporter)?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        // error productions

        Err(self.error(self.peek(), "Expected expression.", reporter))
    }

    // Token stream helpers
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type() == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type() == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume<R>(&mut self, token_type: TokenType, message: &str, reporter: &mut R) -> Result<&Token, ParseError>
    where
        R: ErrorReporter,
    {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message, reporter))
    }

    // Error helper
    fn error<R>(&self, token: &Token, message: &str, reporter: &mut R) -> ParseError
    where
        R: ErrorReporter,
    {
        reporter.error_token(token, message);
        ParseError
    }

    fn synchronize<R>(&mut self, reporter: &mut R)
    where
        R: ErrorReporter,
    {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type() == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}