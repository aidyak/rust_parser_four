#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    EOF,
}

struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer { input, position: 0 }
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        if current_char.is_digit(10) {
            let number = self.consume_number();
            return Token::Number(number);
        }

        match current_char {
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                Token::Minus
            }
            '*' => {
                self.position += 1;
                Token::Star
            }
            '/' => {
                self.position += 1;
                Token::Slash
            }
            '(' => {
                self.position += 1;
                Token::LParen
            }
            ')' => {
                self.position += 1;
                Token::RParen
            }
            _ => panic!("Invalid character found: {}", current_char),
        }
    }

    fn consume_number(&mut self) -> i64 {
        let start = self.position;
        while self.position < self.input.len() {
            if !self.input.chars().nth(self.position).unwrap().is_digit(10) {
                break;
            }
            self.position += 1;
        }
        let number_str = &self.input[start..self.position];
        number_str.parse().unwrap()
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input.chars().nth(self.position).unwrap().is_whitespace() {
            self.position += 1;
        }
    }
}

// Parser用の構造体
struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        let current_token = lexer.next_token();
        Parser { lexer, current_token }
    }

    fn parse_expression(&mut self) -> i64 {
        let mut result = self.parse_term();

        loop {
            match self.current_token {
                Token::Plus => {
                    self.eat(Token::Plus);
                    result += self.parse_term();
                }
                Token::Minus => {
                    self.eat(Token::Minus);
                    result -= self.parse_term();
                }
                _ => break,
            }
        }

        result
    }

    fn parse_term(&mut self) -> i64 {
        let mut result = self.parse_factor();

        loop {
            match self.current_token {
                Token::Star => {
                    self.eat(Token::Star);
                    result *= self.parse_factor();
                }
                Token::Slash => {
                    self.eat(Token::Slash);
                    result /= self.parse_factor();
                }
                _ => break,
            }
        }

        result
    }

    fn parse_factor(&mut self) -> i64 {
        match self.current_token {
            Token::Number(num) => {
                self.eat(Token::Number(num));
                num
            }
            Token::LParen => {
                self.eat(Token::LParen);
                let result = self.parse_expression();
                self.eat(Token::RParen);
                result
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn eat(&mut self, expected_token: Token) {
        if self.current_token == expected_token {
            self.current_token = self.lexer.next_token();
        } else {
            panic!("Expected token {:?}, found {:?}", expected_token, self.current_token);
        }
    }

    pub fn evaluate(&mut self) -> i64 {
        self.parse_expression()
    }
}

fn main() {
    let input = String::from("3 + 4 * (10 - 2)");
    let mut lexer = Lexer::new(input);
    let mut parser = Parser::new(&mut lexer);

    let result = parser.evaluate();
    println!("Result: {}", result);
}
