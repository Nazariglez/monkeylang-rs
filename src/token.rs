use std::fmt;

#[derive(Clone, Debug)]
pub struct Token {
    pub typ: TokenType,
    pub literal: String
}

impl Token {
    pub fn new(typ: TokenType, literal: &str) -> Token {
        Token {
            typ: typ,
            literal: literal.to_string()
        }
    }

    pub fn from_char(typ: TokenType, ch: char) -> Token {
        Token::new(typ, &ch.to_string())
    }

    pub fn default() -> Token {
        Token::new(TokenType::Illegal, "")
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Illegal,
    EOF,

    Ident,
    Int,
    String,

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,
    Equal,
    NotEqual,

    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            _Illegal => "Illegal",
            _EOF => "EOF",
            _Ident => "Ident",
            _ => "Unimplemented!"
        };

        write!(f, "{}", val)
    }
}

pub fn lookup_ident(ident: &str) -> TokenType {
    match ident {
        "fn" => TokenType::Function,
        "let" => TokenType::Let,
        "true" => TokenType::True,
        "false" => TokenType::False,
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "return" => TokenType::Return,
        _ => TokenType::Ident
    }
}
