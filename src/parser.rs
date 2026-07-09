use crate::token::{Token, TokenType};
use crate::values::Literal;
use crate::expr::Expr;
use crate::stmt::Stmt;
use crate::errors::ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    errors: Vec<ParseError>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, errors: Vec::new(), current: 0 }
    }

    pub fn parse_prompt_line(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let checkpoint = self.current;
        
        if let Ok(expr) = self.expression() {
            // must consume everything up to EOF to count as "just an expression".
            if self.check(&TokenType::Semicolon) {
                self.advance();
            }
            if self.is_at_end() {
                return Ok(vec![Stmt::Print{ expression: Box::new(expr) }]);
            }
        }
        // backtrack and parse normally.
        self.current = checkpoint;
        self.parse()
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }

        if self.errors.is_empty() { Ok(statements) } else { Err(self.errors.clone()) }
    }

    // refer to grammar.md for grammar rules

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?.clone();

        let mut initializer = Expr::Literal { value: Literal::Nil };
        if self.match_token(&[TokenType::Equal]) {
            initializer = self.expression()?;
        }

        self.consume(TokenType::Semicolon, "Expect ';' after variable declaration.")?;

        Ok(Stmt::Var { name, initializer: Box::new(initializer) })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        } else if self.match_token(&[TokenType::LeftBrace]) {
            let statements = self.block()?;
            return Ok(Stmt::Block { statements });
        } else if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        } else if self.match_token(&[TokenType::While]) {
            return self.while_statement();
        } else if self.match_token(&[TokenType::For]) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_token(&[TokenType::Semicolon]) {
            None
        } else if self.match_token(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = if !self.check(&TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment = if !self.check(&TokenType::RightParen) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = Box::new(self.statement()?);

        //desugar into while loop

        if let Some(inc) = increment {
            body = Box::new(Stmt::Block {
                statements: vec![
                    *body,
                    Stmt::Expression { expression: Box::new(inc) },
                ],
            });
        }

        let condition = condition.unwrap_or(Expr::Literal { value: Literal::Bool(true) });

        body = Box::new(Stmt::While {
            condition: Box::new(condition),
            body,
        });

        if let Some(init) = initializer {
            body = Box::new(Stmt::Block {
                statements: vec![
                    init,
                    *body,
                ],
            });
        }

        Ok(*body)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If { condition: Box::new(condition), then_branch, else_branch })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after while condition.")?;

        let body = Box::new(self.statement()?);
        Ok(Stmt::While { condition: Box::new(condition), body })
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression {
            expression: Box::new(expr),
        })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    // Grammar entry point
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.assignment()?;

        while self.match_token(&[TokenType::Comma]) {
            let operator = self.previous().clone();
            let right = self.assignment()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(self.error(&equals, "Invalid assignment target."));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.ternary()?;

        while self.match_token(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.ternary()?;
            expr = Expr::Logical {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        if self.match_token(&[TokenType::Question]) {
            let then_branch = self.expression()?;
            self.consume(TokenType::Colon, "Expect ':' after then branch of ternary expression.")?;
            let else_branch = self.ternary()?;
            expr = Expr::Ternary {
                condition: Box::new(expr),
                then_branch: Box::new(then_branch),
                else_branch: Box::new(else_branch),
            };
        }

        Ok(expr)
    }

    // Binary precedence levels
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    // Unary and primary expressions
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::False]) {
            return Ok(Expr::Literal { value: Literal::Bool(false) });
        }
        if self.match_token(&[TokenType::True]) {
            return Ok(Expr::Literal { value: Literal::Bool(true) });
        }
        if self.match_token(&[TokenType::Nil]) {
            return Ok(Expr::Literal { value: Literal::Nil });
        }
        if self.match_token(&[TokenType::Number, TokenType::String]) {
            let literal = self.previous().literal()
                .cloned()
                .ok_or_else(|| self.error(self.previous(), "Expected a literal value."))?;
            return Ok(Expr::Literal { value: literal });
        }
        if self.match_token(&[TokenType::Identifier]) {
            return Ok(Expr::Variable { name: self.previous().clone() });
        }
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        // error productions

        Err(self.error(self.peek(), "Expected expression."))
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

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    // Error helper
    fn error(&self, token: &Token, message: &str) -> ParseError {
        ParseError {
            token: token.clone(),
            message: message.to_string(),
        }
    }

    fn synchronize(&mut self) {
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