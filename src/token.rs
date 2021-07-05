pub enum Token {
    Add,
    Sub,
    Mul,
    Divide,
    LeftParen,
    RightParen,
    Num(i64),
    EOF,
}
