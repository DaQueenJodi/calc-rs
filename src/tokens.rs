use std::fmt::Display;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub flavor: TokenFlavor,
    pub value: Option<TokenValue>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenFlavor {
    NUMBER,
    MINUS,
    MUL,
    DIV,
    ADD,

    EOF,
    NONE,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenValue {
    NUM(i128),
}

impl Token {
    pub fn new(flavor: TokenFlavor, value: Option<TokenValue>) -> Token {
        Token { flavor, value }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let formatted_value = match self.value {
            None => "".to_string(),
            Some(num) => "(".to_string() + &format!("{num}") + ")",
        };
        write!(f, "{:?}{formatted_value}", self.flavor) // TokenFlavor(TokenValue)
    }
}

impl Display for TokenValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TokenValue::NUM(num) => format!("{num}"),
        };
        write!(f, "{text}")
    }
}

impl TokenValue {
    pub fn get_num(&self) -> i128 {
        let TokenValue::NUM(num) = self;
        return *num;
    }
}
