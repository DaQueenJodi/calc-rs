use crate::tokens::{Token, TokenFlavor, TokenValue};

pub struct Tokenizer {
    text: String,
    pos: usize,
    curr_char: Option<char>,
}

impl Tokenizer {
    pub fn new(text: String) -> Tokenizer {
        Tokenizer {
            text: text.clone(),
            pos: 0,
            curr_char: text.chars().nth(0),
        }
    }
    pub fn next_token(&mut self) -> Token {
        let curr_char = match self.curr_char {
            Some(c) => c,
            None => return Token::new(TokenFlavor::EOF, None),
        };

        if curr_char.is_digit(10) {
            return self.get_num();
        }

        if curr_char == ' ' {
            // skip white space
            self.next_char();
            return self.next_token();
        }

        let flavor = match curr_char {
            '+' => TokenFlavor::ADD,
            '-' => TokenFlavor::MINUS, // could be a negative number or subtraction
            '/' => TokenFlavor::DIV,
            '*' => TokenFlavor::MUL,
            _ => panic!("didnt expect character: {}", curr_char),
        };
        return Token::new(flavor, None);
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token();
            tokens.push(token);
            if token.flavor == TokenFlavor::EOF {
                break;
            }
            self.next_char();
        }
        tokens
    }
}
impl Tokenizer {
    fn next_char(&mut self) {
        //println!("welp");
        self.pos += 1;
        self.curr_char = self.text.chars().nth(self.pos)
    }
    fn get_num(&mut self) -> Token {
        let mut nums: Vec<char> = Vec::new();
        while let Some(c) = self.curr_char {
            //println!("welp");
            if c.is_numeric() {
                nums.push(c);
            } else {
                break;
            }
            self.next_char();
        }
        let num = nums.into_iter().collect::<String>().parse().unwrap();
        let token_value = TokenValue::NUM(num);
        Token::new(TokenFlavor::NUMBER, Some(token_value))
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    curr_token: Token,
}

impl Parser {
    fn eat(&mut self, compare: TokenFlavor) {
        let flavor = self.curr_token.flavor;
        if flavor == compare {
            self.next_token();
            return;
        }
        panic!("expected: '{compare:?}, got: {flavor:?}'")
    }
    fn term(&mut self) -> TokenValue {
        let token = self.curr_token;
        self.eat(TokenFlavor::NUMBER);
        token.value.unwrap()
    }
    fn next_token(&mut self) {
        self.pos += 1;
        self.curr_token = self.tokens[self.pos]
    }
    pub fn expr(&mut self) -> i128 {
        let mut result = self.term().get_num();
        loop {
            match self.curr_token.flavor {
                TokenFlavor::MINUS => {
                    self.eat(TokenFlavor::MINUS);
                    result -= self.term().get_num();
                }
                TokenFlavor::ADD => {
                    self.eat(TokenFlavor::ADD);
                    result += self.term().get_num();
                }
                TokenFlavor::MUL => {
                    self.eat(TokenFlavor::MUL);
                    result *= self.term().get_num();
                }
                TokenFlavor::DIV => {
                    self.eat(TokenFlavor::DIV);
                    result /= self.term().get_num();
                }
                TokenFlavor::EOF => break, // end on EOF
                _ => panic!("expected operator, got {:?}", self.curr_token.flavor),
            }
        }
        result
    }
}
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.clone(),
            pos: 0,
            curr_token: tokens[0],
        }
    }
}
