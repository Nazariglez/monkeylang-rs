use crate::token::*;

pub struct Identifier {
    pub tok: Token,
    pub value: String
}

impl Identifier {
    pub fn new(tok: Token, val: &str) -> Identifier {
        Identifier {
            tok: tok,
            value: val.to_string()
        }
    }

    pub fn token_literal(&self) -> &str {
        &self.tok.literal
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", self.value)
    }
}

pub enum Statement {
    Let(Identifier, Expression),
    Return(Expression),
    Expression(Expression),
}

impl Statement {
    pub fn token_literal(&self) -> String {
        match self {
            Statement::Let(_, _) => String::from("let"),
            Statement::Return(_)=> String::from("return"),
            Statement::Expression(exp) => exp.to_string(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Statement::Let(ident, exp) => {
                format!("{} {} = {};", self.token_literal(), ident, exp)
            },
            Statement::Return(exp) => {
                if exp.to_string() == "" {
                    format!("{};", self.token_literal())
                } else {
                    format!("{} {};", self.token_literal(), exp)
                }
            },
            Statement::Expression(exp) => exp.to_string()
        };

        write!(fmt, "{}", str)
    }
}


//pub type Expression = String;
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Expression::Identifier(ident) => ident.to_string(),
            Expression::Literal(l) => l.to_string(),
        };

        write!(fmt, "{}", str)
    }
}


pub enum Literal {
    Int(i32),
    String(String),
    Bool(bool)
}

impl std::fmt::Display for Literal {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Int(i) => write!(fmt, "{}", i),
            Literal::String(s) => write!(fmt, "{}", s),
            Literal::Bool(b) => write!(fmt, "{}", b)   
        }
    }
}



pub struct Program {
    pub statements: Vec<Statement>
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: vec![]
        }
    }

    pub fn token_literal(&self) -> String {
        if let Some(stmt) = self.statements.get(0) {
            stmt.token_literal()
        } else {
            "".to_string()
        }
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = vec![];
        for stmt in &self.statements {
            s.push(stmt.to_string());
        }

        write!(fmt, "{}", s.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        let program = Program {
            statements: vec![
                Statement::Let(
                    Identifier::new(Token::new(TokenType::Ident, "my_var"), "my_var"),
                    Expression::Identifier(Identifier::new(Token::new(TokenType::Ident, "another_var"), "another_var"))           
                ),
            ]
        };

        assert_eq!(program.to_string(), "let my_var = another_var;", "program.to_string() is wrong. got='{}'", program.to_string());
    }
}
