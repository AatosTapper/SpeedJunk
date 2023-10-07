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
    Parenthesis,
    Null
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

impl ParseNode {
    fn evaluate(&mut self) -> f64 {
        match self {
            ParseNode::Binary(operator, left_operand, right_operand) => {
                match operator {
                    '+' => return left_operand.evaluate() + right_operand.evaluate(),
                    '-' => return left_operand.evaluate() - right_operand.evaluate(),
                    '*' => return left_operand.evaluate() * right_operand.evaluate(),
                    '/' => return left_operand.evaluate() / right_operand.evaluate(),
                    '^' => return f64::powf(left_operand.evaluate(), right_operand.evaluate()),
                    _ => return 0.0
                }
            }
            ParseNode::Unary(operator, left_operand) => {
                match operator {
                    '-' => return -left_operand.evaluate(),
                    _ => return 0.0
                }
            }
            ParseNode::Number(num) => {
                *num
            }
            _ => return 0.0
        }
    }
}

#[derive(Debug)]
enum ParserError {
    SyntaxError(String),
    MathError(String)
}

#[derive(Debug)]
struct Parser {
    curr_index: usize,
    all_tokens: Vec<Token>
}

impl Parser {
    fn new() -> Self {
        Self {
            curr_index: 0,
            all_tokens: Vec::new(),
        }
    }

    fn create_ast(&mut self, tokens: Vec<Token>) -> Result<Box<ParseNode>, ParserError> {
        self.all_tokens = tokens;
        self.parse_expression()
    }

    fn parse_expression(&mut self) -> Result<Box<ParseNode>, ParserError> {
        let mut term = self.parse_term()?;
        let mut token = self.all_tokens[self.curr_index];
        while (matches!(token.lex.as_str(), "+") || matches!(token.lex.as_str(), "-"))
            && matches!(token.typ, TokenType::BinaryOp) {
            let mut operator: char = token.lex.chars().next().unwrap();
            self.next_token();
            let mut right_term = self.parse_term()?;
            term = Box::new(ParseNode::Binary(operator, term, right_term));
        }
        Ok(term)
    }

    fn parse_term(&mut self) -> Result<Box<ParseNode>, ParserError> {
        let mut factor = self.parse_factor()?;
        let mut token = self.all_tokens[self.curr_index];
        while matches!(token.lex.as_str(), "*") || matches!(token.lex.as_str(), "/") {
            let mut operator: char = token.lex.chars().next().unwrap();
            self.next_token();
            let mut right_factor = self.parse_factor()?;
            factor = Box::new(ParseNode::Binary(operator, factor, right_factor));
        }
        Ok(factor)
    }

    fn parse_factor(&mut self) -> Result<Box<ParseNode>, ParserError> {
        let token = self.all_tokens[self.curr_index];
        if matches!(token.typ, TokenType::Number) {
            let val: f64 = token.lex.parse().unwrap();
            self.next_token();
            Ok(Box::new(ParseNode::Number(val)))
        } else if matches!(token.lex.as_str(), "(") {
            self.next_token();
            let expression = self.parse_expression()?;
            let token = self.all_tokens[self.curr_index];
    
            if matches!(token.lex.as_str(), ")") {
                self.next_token(); // Move to the next token after the closing parenthesis
                Ok(expression)
            } else {
                Err(ParserError::SyntaxError("No Closing Parenthesis Found".to_string()))
            }
        } else {
            Err(ParserError::SyntaxError("Unexpected Token".to_string()))
        }
    }
    

    fn next_token(&self) {
        if self.curr_index < self.all_tokens.len() - 1 {
            self.curr_index += 1;
        } else {
            println!("Parser ran out of tokens");
        }
    }
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
    let mut parser: Parser = Parser::new();

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
        let ast: Result<Box<ParseNode>, ParserError> = parser.create_ast(lexed);

        match ast {
            Ok(mut result) => {
                println!("{}", result.evaluate());
                continue;
            }
            Err(error) => match error {
                ParserError::SyntaxError(message) => {
                    println!("Syntax error: {}", message);
                    continue;
                }
                ParserError::MathError(message) => {
                    println!("Math error: {}", message);
                    continue;
                }
            }
        }
    }
}

fn main() {
    program_loop();
}