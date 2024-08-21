use std::mem::discriminant;
use super::scanner::{TokenType, Token};
use super::ast::Expr;
use super::errors::LanguageError;

pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    current: usize,
    errs: Vec<LanguageError>
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, current: 0, errs: Vec::new()}
    }

    pub fn parse(&mut self) -> Result<Expr,String> {
       if self.errs.len()  == 0 {
           Ok(self.expr())
       }else{
           Err("Error for some reason".into())
       }
    }

    fn expr(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr: Expr = self.comparison();
        while 
            self.tmatch(&[TokenType::BANGEQUAL, TokenType::EQUALEQUAL]){
                let op: Token = self.prev();
                let rhs: Expr = self.comparison();

                expr = Expr::Binary(
                                Box::new(expr),
                                Box::new(op),
                                Box::new(rhs));
            }

        expr

    }

    fn comparison(&mut self) -> Expr {
        let mut expr: Expr = self.term();

        while 
            self.tmatch(&[TokenType::GREATER, 
                          TokenType::GREATEREQUAL,
                          TokenType::LESS,
                          TokenType::LESSEQUAL]) {
                let op: Token = self.prev();
                let rhs: Expr = self.term();

                expr = Expr::Binary(
                                Box::new(expr),
                                Box::new(op),
                                Box::new(rhs));
            }

        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr: Expr = self.factor();

        while
            self.tmatch(&[TokenType::MINUS,
                          TokenType::PLUS]) {
                let op: Token = self.prev();
                let rhs: Expr = self.factor();

                expr = Expr::Binary(
                                Box::new(expr),
                                Box::new(op),
                                Box::new(rhs));
            }

        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr: Expr = self.unary();
        while 
            self.tmatch(&[TokenType::SLASH, TokenType::STAR]) {
                let op: Token = self.prev();
                let rhs: Expr = self.unary();

                expr = Expr::Binary(
                                Box::new(expr),
                                Box::new(op),
                                Box::new(rhs));
            }

        expr
    }

    fn unary(&mut self) -> Expr {
        if self.tmatch(&[TokenType::BANG, TokenType::MINUS]) {
            let op: Token = self.prev();
            let rhs: Expr = self.unary();

            return Expr::Unary(
                        Box::new(op),
                        Box::new(rhs));
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        let token = self.peek();
        self.advance();

        match token.token_type {
            TokenType::FALSE => Expr::BoolLiteral(false),
            TokenType::TRUE => Expr::BoolLiteral(true),
            TokenType::NIL => Expr::NIL,
            TokenType::NUMBER(num) => Expr::NumberLiteral(num),
            TokenType::STRING(str) => Expr::StringLiteral(str),
            TokenType::LEFTPAREN => {
                let expr = self.expr();
                match self.consume(&TokenType::RIGHTPAREN) {
                    Ok(_) => {},
                    Err(err) => self.errs.push(LanguageError::ParserError(err))
                }
                Expr::Grouping(Box::new(expr))
            }
            _ => unreachable!()
        }
    }

    fn consume(&mut self, ttype: &TokenType) -> Result<Token, String> {
        if self.check(ttype) {
            return Ok(self.advance());
        }

        Err(format!("Expected token {:?}", ttype))
    }

    fn advance(&mut self) -> Token {
        if self.current < self.tokens.len() {
            self.current += 1;
        }

        self.peek()
    }

    fn prev(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn check(&self, ttype: &TokenType) -> bool {
        if self.current >= self.tokens.len() {
            return false;
        }
        
        discriminant(&self.peek().token_type) == discriminant(ttype)

    }

    fn tmatch (&mut self, ttypes: &[TokenType]) -> bool {
        for ttype in ttypes {
            if self.check(ttype) {

                self.advance();
                return true;
            }
        }

        return false;
    }
}
