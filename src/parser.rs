use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens[self.pos].clone();
        self.pos += 1;
        token
    }

    fn expr(&mut self) -> f64 {
        let mut left_value = self.term();
        while self.peek() == &Token::Plus || self.peek() == &Token::Minus {
            let token: Token = self.consume();
            let right_value = self.term();
            if matches!(token, Token::Plus) {
                left_value = left_value + right_value
            } else {
                left_value = left_value - right_value
            }
        }
        left_value
    }

    fn term(&mut self) -> f64 {
        let mut left_value = self.factor();
        while self.peek() == &Token::Multiply || self.peek() == &Token::Divide {
            let token: Token = self.consume();
            let right_value = self.factor();
            if matches!(token, Token::Multiply) {
                left_value = left_value * right_value
            } else {
                left_value = left_value / right_value
            }
        }
        left_value
    }

    fn factor(&mut self) -> f64 {
        match self.consume() {
            Token::Number(num) => num,
            Token::Negate => -(self.factor()),
            Token::LParen => {
                let res = self.expr();
                assert!(
                    self.peek() == &Token::RParen,
                    "Expected Closing Parenthesis"
                );
                self.consume();
                res
            }
            _ => panic!("Unexpected Token"),
        }
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> f64 {
    let mut parser = Parser::new(tokens);
    parser.expr()
}
