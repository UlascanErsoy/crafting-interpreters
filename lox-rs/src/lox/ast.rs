use super::scanner::Token;

pub enum Expr {
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Grouping(Box<Expr>),
    NumberLiteral(f64),
    StringLiteral(String),
    Unary(Box<Token>, Box<Expr>)
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, exp: &Expr) -> T; 
}

pub struct AstPrinter;

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, exp: &Expr) -> String {
        match exp {
            Expr::Binary(e1, t, e2) => format!("{} {} {}", t, self.visit_expr(e1), self.visit_expr(e2)),
            Expr::Grouping(e) => format!("(group {} )", self.visit_expr(e)),
            Expr::NumberLiteral(n) => format!("{}", n),
            Expr::StringLiteral(s) => format!("{}", s),
            Expr::Unary(t, e) => format!("( {} {} )", t, self.visit_expr(e))
        }
    }
}
