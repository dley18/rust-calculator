use std::io;
mod lexer;
mod parser;
mod token;

fn print_prompt() {
    println!("--- Calculator v1.0 ---");
    println!("Ask me any math equation below:");
}

fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    print_prompt();
    let input = get_user_input();
    let tokens = lexer::tokenize(input);
    println!("Result = {}", parser::parse_tokens(tokens));
}
