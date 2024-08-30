use super::scanner::{Token, TokenType};
use super::atom::Atom;
use std::any::{Any, TypeId};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Atom),
    Unary(Box<Token>, Box<Expr>)
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, exp: &Expr) -> T; 
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, exp: &Expr) -> String {
        match exp {
            Expr::Binary(e1, t, e2) => format!("[({}) {} {}]", t, self.visit_expr(e1), self.visit_expr(e2)),
            Expr::Grouping(e) => format!("(group {} )", self.visit_expr(e)),
            Expr::Literal(n) => format!("{:?}", n),
            Expr::Unary(t, e) => format!("( {} {} )", t, self.visit_expr(e))
        }
    }
}

struct Interpreter;

//dynamic heap allocated Any type
impl Visitor<Atom> for Interpreter {
    fn visit_expr(&mut self, exp: &Expr) -> Atom {
        match exp {
            Expr::Literal(atom) => atom.clone(),
            Expr::Grouping(expr) => evaluate(*expr.clone()),
            Expr::Binary(lhs, op, rhs) => {
                match op.token_type {
                    TokenType::PLUS => Atom::add(evaluate(*lhs.clone()), evaluate(*rhs.clone())).unwrap(),
                    TokenType::MINUS => Atom::sub(evaluate(*lhs.clone()), evaluate(*rhs.clone())).unwrap(),
                    TokenType::STAR => Atom::mult(evaluate(*lhs.clone()), evaluate(*rhs.clone())).unwrap(),
                    _ => todo!()
                }
            }
            _ => todo!()
        }
    }
}

pub fn evaluate(expr: Expr) -> Atom {
    Interpreter{}.visit_expr(&expr)
}
