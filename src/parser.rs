use crate::errors::ParseError;
use crate::Node;
use crate::OperPrec;
use crate::Token;
use crate::Tokenizer;

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    cur_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(expr: &'a str) -> Result<Self, ParseError> {
        let mut lexer = Tokenizer::new(expr);
        let cur_token = match lexer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidCharactor),
        };

        Ok(Parser {
            tokenizer: lexer,
            cur_token,
        })
    }

    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let ast = self.generate_ast(OperPrec::DefaultZero);

        match ast {
            Ok(ast) => Ok(ast),
            Err(e) => Err(e),
        }
    }

    fn get_next_token(&mut self) -> Result<(), ParseError> {
        let next_token = match self.tokenizer.next() {
            Some(token) => token,
            None => return Err(ParseError::InvalidCharactor),
        };

        self.cur_token = next_token;
        Ok(())
    }

    fn generate_ast(&mut self, oper_perc: OperPrec) -> Result<Node, ParseError> {
        let mut left_expr = self.parse_number()?;

        while oper_perc < self.cur_token.get_oper_prec() {
            if self.cur_token == Token::Eof {
                break;
            }
            let right_expr = self.convert_token_to_node(left_expr)?;
            left_expr = right_expr;
        }
        Ok(left_expr)
    }

    fn parse_number(&mut self) -> Result<Node, ParseError> {
        let token = self.cur_token.clone();
        match token {
            Token::Sub => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::Negative)?;

                Ok(Node::Negative(Box::new(expr)))
            }
            Token::Num(i) => {
                self.get_next_token()?;
                Ok(Node::Number(i))
            }
            Token::LeftParen => {
                self.get_next_token()?;
                let expr = self.generate_ast(OperPrec::DefaultZero)?;
                self.check_paren(Token::RightParen)?;
                if self.cur_token == Token::LeftParen {
                    let right = self.generate_ast(OperPrec::MulDiv)?;
                    return Ok(Node::Multiply(Box::new(expr), Box::new(right)));
                }
                Ok(expr)
            }
            _ => Err(ParseError::UnableToParse),
        }
    }

    fn convert_token_to_node(&mut self, left_expr: Node) -> Result<Node, ParseError> {
        match self.cur_token {
            Token::Add => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Add(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Sub => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::AddSub)?;
                Ok(Node::Subtract(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Mul => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Multiply(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Divide => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Divide(Box::new(left_expr), Box::new(right_expr)))
            }
            Token::Caret => {
                self.get_next_token()?;
                let right_expr = self.generate_ast(OperPrec::MulDiv)?;
                Ok(Node::Caret(Box::new(left_expr), Box::new(right_expr)))
            }
            _ => Err(ParseError::InvalidOperator(self.cur_token.clone())),
        }
    }

    fn check_paren(&mut self, expected: Token) -> Result<(), ParseError> {
        if expected == self.cur_token {
            self.get_next_token()?;
            Ok(())
        } else {
            Err(ParseError::InvalidResult {
                expected,
                got: self.cur_token.clone(),
            })
        }
    }
}
