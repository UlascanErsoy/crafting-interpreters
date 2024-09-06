use super::errors::LanguageError;
use super::scanner::{Token, TokenType};
use std::fmt;
use std::convert::From;

#[derive(Debug, Clone)]
pub enum Atom {
    String(String),
    Number(f64),
    Bool(bool),
    Nil
}

impl PartialEq for Atom {
    
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Atom::String(lhs), Atom::String(rhs)) => lhs == rhs,
            (Atom::Number(lhs), Atom::Number(rhs)) => lhs == rhs,
            (Atom::Nil, Atom::Nil) => true,
            _ => false
        }
    }
}

impl From<Token> for Atom {
    fn from(token: Token) -> Self {
        match token.token_type {
            TokenType::STRING(s) => Self::String(s),
            TokenType::NUMBER(n) => Self::Number(n),
            TokenType::TRUE  => Self::Bool(true),
            TokenType::FALSE => Self::Bool(false),
            TokenType::NIL => Self::Nil,
            _ => panic!("This should not be possible!")
        }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atom::String(_) => write!(f, "String"),
            Atom::Number(_) => write!(f, "Number"),
            Atom::Bool(_) => write!(f, "Bool"),
            Atom::Nil => write!(f, "Nil")
        }
    }
}

impl Atom {
    pub fn add(lhs: Self, rhs: Self) -> Result<Self, LanguageError> {
        match (lhs, rhs) {
            (Atom::Number(l) , Atom::Number(r)) => Ok(Atom::Number(l + r)),
            (Atom::Number(l) , Atom::Bool(r)) => Ok(Atom::Number(l + if r {1_f64} else {0_f64})),
            (Atom::Bool(l) , Atom::Number(r)) => Ok(Atom::Number(if l {1_f64} else {0_f64} + r)),
            (Atom::Number(l) , Atom::String(r)) => Ok(Atom::String(format!("{}{}", l,r))),
            (Atom::String(l) , Atom::Number(r)) => Ok(Atom::String(format!("{}{}", l,r))),
            (Atom::String(l) , Atom::String(r)) => Ok(Atom::String(format!("{}{}", l,r))),
            (Atom::Bool(l) , Atom::String(r)) => Ok(Atom::String(format!("{}{}", l,r))),
            (Atom::String(l) , Atom::Bool(r)) => Ok(Atom::String(format!("{}{}", l,r))),
            (Atom::Nil , Atom::String(r)) => Ok(Atom::String(format!("nil{}", r))),
            (Atom::String(l), Atom::Nil) => Ok(Atom::String(format!("{}nil", l))),
            (l,r) => Err(LanguageError::TypeError(format!("Addition not supported for {} and {}",l,r))) 

        }
    }

    pub fn sub(lhs: Self, rhs: Self) -> Result<Self, LanguageError> {
        match (lhs, rhs) {
            (Atom::Number(l) , Atom::Number(r)) => Ok(Atom::Number(l - r)),
            (Atom::Number(l) , Atom::Bool(r)) => Ok(Atom::Number(l - if r {1_f64} else {0_f64})),
            (Atom::Bool(l) , Atom::Number(r)) => Ok(Atom::Number(if l {1_f64} else {0_f64} - r)),
            (l,r) => Err(LanguageError::TypeError(format!("Subtraction not supported for {} and {}",l,r))) 
        }

    }

    pub fn mult(lhs: Self, rhs: Self) -> Result<Self, LanguageError> {
        match (lhs, rhs) {
            (Atom::Number(l) , Atom::Number(r)) => Ok(Atom::Number(l * r)),
            (Atom::Number(l) , Atom::Bool(r)) => Ok(Atom::Number(l * if r {1_f64} else {0_f64})),
            (Atom::Bool(l) , Atom::Number(r)) => Ok(Atom::Number(if l {1_f64} else {0_f64} * r)),
            (l,r) => Err(LanguageError::TypeError(format!("Multiplication not supported for {} and {}",l,r))) 
        }

    }

    pub fn divide(lhs: Self, rhs: Self) -> Result<Self, LanguageError> {
        match (lhs, rhs) {
            (Atom::Number(l) , Atom::Number(r)) => Ok(Atom::Number(l / r)),
            (Atom::Number(l) , Atom::Bool(r)) => Ok(Atom::Number(l / if r {1_f64} else {0_f64})),
            (Atom::Bool(l) , Atom::Number(r)) => Ok(Atom::Number(if l {1_f64} else {0_f64} / r)),
            (l,r) => Err(LanguageError::TypeError(format!("Division not supported for {} and {}",l,r))) 
        }

    }

    pub fn comp(t: &TokenType, lhs: Self, rhs: Self) -> Result<Self, LanguageError> {
        if let (Atom::Number(l) , Atom::Number(r)) = (&lhs, &rhs) {
            match t {
                TokenType::LESSEQUAL => Ok(Atom::Bool(l <= r)),
                TokenType::GREATEREQUAL => Ok(Atom::Bool(l >= r)),
                TokenType::GREATER => Ok(Atom::Bool(l > r)),
                TokenType::LESS => Ok(Atom::Bool(l < r)),
                _ => unreachable!()
            }
        }else{
            Err(LanguageError::TypeError(format!("{:?} not supported between {} and {}", t, lhs, rhs)))
        }
    }


}
