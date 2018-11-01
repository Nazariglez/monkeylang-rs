use crate::token::*;
use crate::lexer::Lexer;
use crate::ast;

struct Parser {
    l: Lexer,
    
    errors: Vec<String>,

    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut p = Parser {
            l: lexer,
            errors: vec![],
            cur_token: Token::default(),
            peek_token: Token::default(),
        };

        p.next_token();
        p.next_token();

        p
    }

    fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, typ: &TokenType) {
        self.errors.push(format!("expected next token to be '{:?}', got '{:?}' instead", typ, self.peek_token.typ));
    }

    fn next_token(&mut self) {
        let mut token = self.l.next_token();
        std::mem::swap(&mut token, &mut self.peek_token);
        self.cur_token = token;
    }

    fn parse_program(&mut self) -> Result<ast::Program, String> {
        let mut program = ast::Program::new();
        while self.cur_token.typ != TokenType::EOF {
            while let Some(stmt) = self.parse_statement() {
                program.statements.push(stmt);
            }
            self.next_token();
        }

        Ok(program)
        //Err("Invalid parse_program".to_string())
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.cur_token.typ {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Return => self.parse_return_statement(),
            _ => None
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::Statement> {
        //let token = self.cur_token.clone();
        
        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let cur_token = self.cur_token.clone();
        let val = self.cur_token.literal.clone();
        let ident = ast::Identifier::new(cur_token, &val);

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(
            ast::Statement::Let(
                ident, 
                ast::Expression::Identifier(
                    ast::Identifier::new(Token::new(TokenType::Ident, &val), &val)
                )
            )
        )
    }

    fn parse_return_statement(&mut self) -> Option<ast::Statement> {
        self.next_token();
        while !self.cur_token_is(TokenType::Semicolon) {
            self.next_token();
        }

        Some(
            ast::Statement::Return(
                ast::Expression::Identifier(
                    ast::Identifier::new(Token::new(TokenType::String, ""), "") //TODO don't use strings as empty values
                )
            )
        )
    }

    fn cur_token_is(&self, typ: TokenType) -> bool {
        self.cur_token.typ == typ
    }

    fn peek_token_is(&self, typ: &TokenType) -> bool {
        self.peek_token.typ == *typ
    }

    fn expect_peek(&mut self, typ: TokenType) -> bool {
        if self.peek_token_is(&typ) {
            self.next_token();
            true
        } else {
            self.peek_error(&typ);
            false
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_let_statements() {
        let input = r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let r_program = p.parse_program();
        if let Err(err) = check_parser_errors(&p) {
            panic!(err);
        }

        match r_program {
            Ok(program) => {
                assert!(program.statements.len() == 3, "program.statements does not contain 3 statements. got={}", program.statements.len());
                
                let tests = vec![
                    "x",
                    "y",
                    "foobar"
                ];

                for (i, e_ident) in tests.iter().enumerate() {
                    match program.statements.get(i) {
                        None => panic!("Invalid statement index"),
                        Some(stmt) => {
                            match stmt {
                                ast::Statement::Let(ident, _expression) => {
                                    assert_eq!(stmt.token_literal(), "let", "token_litral is not 'let'. got={}", stmt.token_literal());
                                    assert_eq!(*e_ident, ident.value, "Expected identifier={}. got={}", e_ident, ident.value);
                                    assert_eq!(*e_ident, ident.token_literal(), "Expected identifier={}. got={}", e_ident, ident.token_literal());
                                },
                                _ => panic!("Invalid ast.Statement, expected 'let'")
                            }
                        }
                    }
                }
                
            },
            Err(e) => {
                panic!("{}", e);
            }
        }
    }

    fn check_parser_errors(p: &Parser) -> Result<(), String> {
        if p.errors.len() == 0 {
            return Ok(());
        }

        let mut err = vec![format!("parser has {} errors", p.errors.len())];
        for msg in &p.errors {
            err.push(format!("parser error: {}", msg));
        }

        let error_msg = err.join("\n");
        Err(error_msg)
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
            return 5;
            return 10;
            return 993322;
        "#;

        let l = Lexer::new(input);
        let mut p = Parser::new(l);

        let r_program = p.parse_program();
        if let Err(err) = check_parser_errors(&p) {
            panic!(err);
        }

        match r_program {
            Err(e) => panic!("{}", e),
            Ok(program) => {
                assert!(program.statements.len() == 3, "program.statements does not contain 3 statements. got={}", program.statements.len());

                for stmt in &program.statements {
                    assert_eq!(stmt.token_literal(), "return", "token_litral is not 'return'. got={}", stmt.token_literal());
                }
            }
        }
    }
}
