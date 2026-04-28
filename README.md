# rust-calculator
A simple CLI calculator implemented in rust. 
Supports addition, subtraction, multiplication, division, parentheses, and negative numbers.

# How does it work?
### Lexer
- Reads raw input string and tokenizes it into a vector of typed tokens
- Handles multi-digit numbers, decimals, and unary minus
### Parser
- Consumes the token vector using a recursive descent parser
- Evaluates expression with correct operator precedence

# Usage
```cargo run```
