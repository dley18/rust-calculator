use std::iter::Peekable;
use std::str::Chars;

use crate::token::Token;

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => {} // whitespace
            '0'..='9' => {
                tokens.push(Token::Number(consume_number(c, &mut chars)));
            }
            '+' => tokens.push(Token::Plus),

            '-' if matches!(chars.peek(), Some(&next) if next.is_numeric()) => {
                // Only a negative number if previous token was an operator or LParen
                // Or if there was no previous token

                let is_negative = matches!(
                    tokens.last(),
                    None | Some(Token::Plus)
                        | Some(Token::Minus)
                        | Some(Token::Multiply)
                        | Some(Token::Divide)
                        | Some(Token::LParen)
                );

                if is_negative {
                    tokens.push(Token::Number(consume_number(c, &mut chars)));
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '-' if matches!(chars.peek(), Some(&next) if next == '(') => {
                let is_negate = matches!(
                    tokens.last(),
                    None | Some(Token::Plus)
                        | Some(Token::Minus)
                        | Some(Token::Multiply)
                        | Some(Token::Divide)
                        | Some(Token::LParen)
                );

                if is_negate {
                    tokens.push(Token::Negate);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Multiply),
            '/' => tokens.push(Token::Divide),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            _ => {} // Anything we don't want
        }
    }
    tokens.push(Token::EOF);
    tokens
}

fn consume_number(first: char, chars: &mut Peekable<Chars>) -> f64 {
    let mut buffer = String::new();
    buffer.push(first);

    while let Some(&next) = chars.peek() {
        if next.is_numeric() || next == '.' {
            buffer.push(next);
            chars.next();
        } else {
            break;
        }
    }

    buffer.parse().unwrap()
}
