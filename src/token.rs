/// List of tokens that generate by tokenizer
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Add,
    Sub,
    Mul,
    Divide,
    Caret,
    LeftParen,
    RightParen,
    Num(f64),
    Eof,
}

#[derive(Debug, PartialEq, PartialOrd)]
/// Determine
pub enum OperPrec {
    DefaultZero,
    AddSub,
    MulDiv,
    Power,
    Negative,
}

impl Token {
    pub fn get_oper_prec(&self) -> OperPrec {
        use self::OperPrec::*;
        use self::Token::*;

        match *self {
            Add | Sub => AddSub,
            Mul | Divide => MulDiv,
            Caret => Power,
            _ => DefaultZero,
        }
    }
}
