#![allow(dead_code)]

#[derive(Debug)]
enum TokenType {
    Number,     // 1, 2, 3... exc.
    Operator,   // + - * / exc.
    Function,   // sqrt, log, exc.
    Parenthesis
}

#[derive(Debug)]
struct Token {
    typ: TokenType,
    lex: String  // Data as a string
}

fn is_operator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '^' => true,
        _ => false
    }
}

fn is_decimal(c: char) -> bool {
    match c {
        ',' => true,
        '.' => true,
        _ => false
    }
}

fn is_parenthesis(c: char) -> bool {
    match c {
        '(' => true,
        ')' => true,
        _ => false
    }
}

fn lexer(data: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = data.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_numeric() {
            let mut buffer: String = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_numeric() || is_decimal(c){
                    buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token {
                typ: TokenType::Number,
                lex: buffer
            });
        } else if c.is_alphabetic() {
            let mut buffer: String = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphabetic() {
                    buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token {
                typ: TokenType::Function,
                lex: buffer
            });
        } else if is_operator(c) {
            tokens.push(Token {
                typ: TokenType::Operator,
                lex: String::from(c)
            });
            chars.next();
        } else if is_parenthesis(c) {
            tokens.push(Token {
                typ: TokenType::Parenthesis,
                lex: String::from(c)
            });
            chars.next();
        } else {
            chars.next();
        }
    }
    tokens
}


fn program_loop() {
    println!("\n----SpeedJunk™----\n");
    println!("Begin Calculation Or Type [ :q ] To Quit\n");
    
    let mut data: String = String::new();
    loop {
        data.clear();
        std::io::stdin().read_line(&mut data).unwrap();
        if data.trim() == ":q" {
            break;
        }
        let lexed: Vec<Token> = lexer(data.as_str());

        for token in lexed {
            println!("{:?}", token);
        }
    }
}

fn main() {
    program_loop();
}