use std::io;
extern crate regex;

use regex::Regex;

fn main() {

    loop {
        println!("Enter input:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        println!("{}", Expression::new(&input).calculate(4));
    }
}

enum Token {
    Number(i64),
    Plus,
    Sub,
    Mul,
    Div,
    Pow,
    LeftParen,
    RightParen,
}
impl Token {
    fn precedence(&self) -> usize {
        match *self {
            Token::Plus | Token::Sub => 1,
            Token::Mul | Token::Div => 2,
            Token::Pow => 3,
            _ => 0,
        }
    }
}

struct Expression {
    regex: Regex,
    tokens: Vec<Token>
}

impl Expression {
    fn new(expression: String) -> Function {
        let local_regex = Regex::new(r"^[0-9]+").expect("Unable to create the regex");
        Function {
            regex : local_regex,
            tokens : tokenize(local_regex, &mut expression)
        }
    }

    fn calculate(&self, x: usize) -> i64 {

    }

    /// Tokenizes the input string into a Vec of Tokens.
    priv fn tokenize(regex: Regex, mut input: &str) -> Vec<Token> {
        let mut result = vec![];

        loop {
            input = input.trim_left();
            if input.is_empty() { break }

            let (token, rest) = match regex.find(input) {
                Some((_, end)) => {
                    let (num, rest) = input.split_at(end);
                }
            }
        }

    }
}

struct Parenthesis {

}
