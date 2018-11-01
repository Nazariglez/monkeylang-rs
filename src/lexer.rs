use crate::token::*;

const ZERO_CHAR:char = 0u8 as char;

fn is_letter(ch: char) -> bool {
    'a' <= ch && ch <= 'z' || 'A' <= ch && ch <= 'Z' || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Lexer {
        let mut lexer = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            ch: ZERO_CHAR
        };

        lexer.read_char();

        lexer 
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = ZERO_CHAR;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
    
    fn read_number(&mut self) -> String {
        let position = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }


        let v = self.input.as_bytes()[position..self.position].iter()
            .map(|v| *v)
            .collect::<Vec<_>>();

        String::from_utf8(v).unwrap_or("".to_string())
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        
        let v = self.input.as_bytes()[position..self.position].iter()
            .map(|v| *v)
            .collect::<Vec<_>>();

        String::from_utf8(v).unwrap_or("".to_string())
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            ZERO_CHAR
        } else {
            self.input.as_bytes()[self.read_position] as char
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        
        let token = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!(
                        "{}{}",
                        String::from_utf8(vec![ch as u8]).unwrap_or("".to_string()),
                        String::from_utf8(vec![self.ch as u8]).unwrap_or("".to_string()),
                    );

                    Token::new(TokenType::Equal, &literal)
                } else {
                    Token::from_char(TokenType::Assign, self.ch)
                }
            },
            '+' => Token::from_char(TokenType::Plus, self.ch),
            '-' => Token::from_char(TokenType::Minus, self.ch),
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!(
                        "{}{}",
                        String::from_utf8(vec![ch as u8]).unwrap_or("".to_string()),
                        String::from_utf8(vec![self.ch as u8]).unwrap_or("".to_string()),
                    );

                    Token::new(TokenType::NotEqual, &literal)
                } else {
                    Token::from_char(TokenType::Bang, self.ch)
                }
            },
            '/' => Token::from_char(TokenType::Slash, self.ch),
            '*' => Token::from_char(TokenType::Asterisk, self.ch),
            '<' => Token::from_char(TokenType::LT, self.ch),
            '>' => Token::from_char(TokenType::GT, self.ch),
            ';' => Token::from_char(TokenType::Semicolon, self.ch),
            '(' => Token::from_char(TokenType::Lparen, self.ch),
            ')' => Token::from_char(TokenType::Rparen, self.ch),
            ',' => Token::from_char(TokenType::Comma, self.ch),
            '{' => Token::from_char(TokenType::Lbrace, self.ch),
            '}' => Token::from_char(TokenType::Rbrace, self.ch),
            ZERO_CHAR => Token::new(TokenType::EOF, ""),
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    return Token::new(lookup_ident(&literal), &literal);
                } else if is_digit(self.ch) {
                    return Token::new(TokenType::Int, &self.read_number());
                } else {
                    Token::from_char(TokenType::Illegal, self.ch)
                }
            },
        };

        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = vec![
            (TokenType::Assign, "="),
            (TokenType::Plus, "+"),
            (TokenType::Lparen, "("),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Rbrace, "}"),
            (TokenType::Comma, ","),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (i, (e_tok, e_lit)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            
            assert_eq!(tok.typ, *e_tok, "Wrong tokentype. {}: expected={:?}, got={:?}", i, e_tok, tok.typ);
            assert_eq!(tok.literal, *e_lit, "Wrong literal. {}: expected={}, got={}", i, e_lit, tok.literal);
        }
    }

    #[test]
    fn test_next_token_2() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (i, (e_tok, e_lit)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.typ, *e_tok, "Wrong tokentype. {}: expected={:?}, got={:?}", i, e_tok, tok.typ);
            assert_eq!(tok.literal, *e_lit, "Wrong literal. {}: expected={}, got={}", i, e_lit, tok.literal);
        }
    }
    
    #[test]
    fn test_next_token_3() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;
        "#;

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),            
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::GT, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (i, (e_tok, e_lit)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.typ, *e_tok, "Wrong tokentype. {}: expected={:?}, got={:?}", i, e_tok, tok.typ);
            assert_eq!(tok.literal, *e_lit, "Wrong literal. {}: expected={}, got={}", i, e_lit, tok.literal);
        }
    }

    #[test]
    fn test_next_token_4() {
        let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        let tests = vec![
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Semicolon, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::Lparen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::Rparen, ")"),
            (TokenType::Semicolon, ";"),            
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::GT, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semicolon, ";"),
            (TokenType::If, "if"),
            (TokenType::Lparen, "("),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::Rparen, ")"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::Lbrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semicolon, ";"),
            (TokenType::Rbrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Equal, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semicolon, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEqual, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semicolon, ";"),
            (TokenType::EOF, ""),
        ];

        let mut lexer = Lexer::new(input);
        for (i, (e_tok, e_lit)) in tests.iter().enumerate() {
            let tok = lexer.next_token();
            assert_eq!(tok.typ, *e_tok, "Wrong tokentype. {}: expected={:?}, got={:?}", i, e_tok, tok.typ);
            assert_eq!(tok.literal, *e_lit, "Wrong literal. {}: expected={}, got={}", i, e_lit, tok.literal);
        }
    }
}
