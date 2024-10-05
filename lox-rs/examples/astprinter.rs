extern crate lox_rs;
use lox_rs::lox;

use lox::ast::{Expr, AstPrinter, ExprVisitor};
use lox::scanner::{Token, TokenType};

fn main() {
    let expr = Expr::Binary(
                        Box::new(Expr::Unary(Box::new(Token { token_type: TokenType::MINUS, lexeme: "-".into(), line: 55 }), Box::new(Expr::NumberLiteral(3.14)))),
                        Box::new(Token{ token_type: TokenType::STAR, lexeme: "*".into(), line:13 }),
                        Box::new(Expr::Grouping(Box::new(Expr::NumberLiteral(52.0)))));

        println!("{}", AstPrinter{}.visit_expr(&expr));
                          
}
