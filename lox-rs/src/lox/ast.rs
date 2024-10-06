use super::errors::LanguageError;
use super::scanner::{Token, TokenType};
use super::atom::Atom;

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Atom),
    Unary(Box<Token>, Box<Expr>),
    Var(Box<String>)

}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Box<Expr>),
    Print(Box<Expr>),
    Var(Box<String>, Box<Expr>)
}

#[derive(Debug, Clone)]
pub struct VarDecl(Box<Expr>);

pub trait ExprVisitor<T> {
    fn visit_expr(&mut self, exp: &Expr) -> T; 
}

pub trait StmtVisitor {
    fn visit_stmt(&mut self, stmt: &Stmt); 
}


pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&mut self, exp: &Expr) -> String {
        match exp {
            Expr::Binary(e1, t, e2) => format!("[({}) {} {}]", t, self.visit_expr(e1), self.visit_expr(e2)),
            Expr::Grouping(e) => format!("(group {} )", self.visit_expr(e)),
            Expr::Literal(n) => format!("{:?}", n),
            Expr::Unary(t, e) => format!("( {} {} )", t, self.visit_expr(e)),
            Expr::Var(val) => format!("{}", val)
        }
    }
}

pub struct Environment {
    values: HashMap<String, Atom>
}

impl Environment {

    pub fn set(&mut self, lval: String, rval: Atom) {
        self.values.insert(lval, rval);
    }
    pub fn get(&self, lval: String) -> Atom {
        match self.values.get(&lval) {
            Some(rval) => rval.clone(),
            None => panic!("{} not defined", lval)
        }
    }
}

pub struct Interpreter {
    pub error: Option<LanguageError>,
    pub env: Environment
}

impl Default for Interpreter {
    fn default() -> Self { Interpreter { error: None , env: Environment{ values: HashMap::new() } }}
}

impl Interpreter {

    pub fn evaluate(&mut self, expr: Expr) -> Atom {
        if let Some(_) = &self.error {
            return Atom::Nil;
        }

        self.visit_expr(&expr)
    }

}

impl StmtVisitor for Interpreter {
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Print(expr) => println!("{:?}", self.evaluate(*expr.clone())),
            Stmt::Expr(expr) => {self.evaluate(*expr.clone());},
            Stmt::Var(name, expr) => {
                let rval: Atom = self.evaluate(*expr.clone());
                self.env.set(*name.clone(), rval)
            }
        }
    }
}

//dynamic heap allocated Any type
impl ExprVisitor<Atom> for Interpreter {
    fn visit_expr(&mut self, exp: &Expr) -> Atom {
        match exp {
            Expr::Literal(atom) => atom.clone(),
            Expr::Grouping(expr) => self.evaluate(*expr.clone()),
            Expr::Binary(lhs, op, rhs) => {
                let res = match &op.token_type {
                    TokenType::PLUS => Atom::add(self.evaluate(*lhs.clone()), self.evaluate(*rhs.clone())),
                    TokenType::MINUS => Atom::sub(self.evaluate(*lhs.clone()), self.evaluate(*rhs.clone())),
                    TokenType::STAR => Atom::mult(self.evaluate(*lhs.clone()), self.evaluate(*rhs.clone())),
                    TokenType::SLASH => Atom::divide(self.evaluate(*lhs.clone()), self.evaluate(*rhs.clone())),
                    TokenType::BANGEQUAL => Ok(Atom::Bool(self.evaluate(*lhs.clone()) != self.evaluate(*rhs.clone()))),
                    TokenType::EQUALEQUAL => Ok(Atom::Bool(self.evaluate(*lhs.clone()) == self.evaluate(*rhs.clone()))),
                    TokenType::LESSEQUAL | 
                    TokenType::GREATEREQUAL |
                    TokenType::GREATER  |
                    TokenType::LESS => Atom::comp(&op.token_type, self.evaluate(*lhs.clone()), self.evaluate(*rhs.clone())),
                    _ => todo!()
                }; 

                match res {
                   Ok(atom) => atom,
                   Err(err) => {
                       self.error = Some(err);
                       Atom::Nil 
                   }
                }
            },
            Expr::Var(var) => self.env.get(*var.clone()),
            _ => todo!()
        }
    }
}


