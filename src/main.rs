#![allow(dead_code)]

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

#[derive(Debug)]
#[derive(Clone)]
enum TokenType {
    Number,     // 1, 2, 3... exc.
    BinaryOp,   // + - * / exc.
    UnaryOp,   // ^ - exc.
    Function,   // sqrt, log, exc.
    Parenthesis
}

#[derive(Debug)]
#[derive(Clone)]
struct Token {
    typ: TokenType,
    lex: String  // Data as a string
}

impl Token {
    fn copy(&self) -> Token {
        Token {
            typ: self.typ.clone(),
            lex: self.lex.clone(),
        }
    }
}

#[derive(Debug)]
enum ParseNode {
    Binary (char, Box<ParseNode>, Box<ParseNode>),
    Unary (char, Box<ParseNode>),
    Number(f64),
    Null
}

#[derive(Debug)]
struct Parser;

// Wtf
impl Parser {
    /*
    fn parser(&self, tokens: &[Token]) -> Box<ParseNode> {
        self.parse_expression(tokens)
    }
    fn parse_factor(&self, tokens: &[Token]) -> Box<ParseNode> {  // Number, Parenthesis

    }
    fn parse_term(&self, tokens: &[Token]) -> Box<ParseNode> {  // Multiply, Divide

    }
    fn parse_expression(&self, tokens: &[Token]) -> Box<ParseNode> {  // Add, Subtract

    }
     */
}

fn lexer(data: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = data.chars().peekable();
    let mut last_tok = Token {typ: TokenType::Number, lex: "0".to_string()};

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
            let mut new_tok: Token = Token {
                typ: TokenType::Number,
                lex: buffer
            };
            last_tok = new_tok.clone();
            tokens.push(new_tok);
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
            let mut new_tok: Token = Token {
                typ: TokenType::Function,
                lex: buffer
            };
            last_tok = new_tok.clone();
            tokens.push(new_tok);
        } else if is_operator(c) {
            if c == '-' 
            && (matches!(last_tok.typ, TokenType::BinaryOp)
            || (matches!(last_tok.typ, TokenType::Parenthesis)
            && last_tok.lex == "(")) {
                let mut new_tok: Token = Token {
                    typ: TokenType::UnaryOp,
                    lex: String::from(c)
                };
                last_tok = new_tok.clone();
                tokens.push(new_tok);
            } else {
                let mut new_tok: Token = Token {
                    typ: TokenType::BinaryOp,
                    lex: String::from(c)
                };
                last_tok = new_tok.clone();
                tokens.push(new_tok);
            }
            chars.next();
        } else if is_parenthesis(c) {
            let mut new_tok: Token = Token {
                typ: TokenType::Parenthesis,
                lex: String::from(c)
            };
            last_tok = new_tok.clone();
            tokens.push(new_tok);
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