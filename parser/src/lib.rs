use ast::*;
use lexer::{self, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.index >= self.tokens.len() {
            panic!("Out of bounds!");
        }

        let t = self.next();

        if !(std::mem::discriminant(&token_type) == std::mem::discriminant(t.get_type())) {
            panic!("Wrong token at {}:{}", t.get_line_nr(), t.get_line_index());
        }

        self.index += 1;
    }

    fn next(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn next_next(&self) -> &Token {
        &self.tokens[self.index + 1]
    }

    pub fn parse(&mut self) {
        self.program();
        println!("Parsing succeeded YAY!");
    }

    // program -> function_list "$"
    fn program(&mut self) {
        match *self.next().get_type() {
            // fn
            TokenType::Fn => {
                self.function_list();
                self.eat(TokenType::End);
            }
            _ => {
                panic!("Unexpected token {:?}", self.next())
            }
        }
    }

    // function_list -> function function_list | e
    fn function_list(&mut self) {
        match self.next().get_type() {
            // fn
            TokenType::Fn => {
                self.function();
                self.function_list();
            }
            // e
            _ => {}
        };
    }

    // function -> "fn" identifier "(" parameter_declaration_list ")" "->" type "{" body "}"
    fn function(&mut self) {
        self.eat(TokenType::Fn);
        self.parse_identifier();
        self.eat(TokenType::Lbra);
        self.parameter_declaration_list();
        self.eat(TokenType::Rbra);
        self.eat(TokenType::Arrow);
        self.parse_type();
        self.eat(TokenType::Lcur);
        self.parse_body();
        self.eat(TokenType::Rcur);
    }

    // paramter_declaration_list -> parameter_declaration parameter_declaration_tail | e
    fn parameter_declaration_list(&mut self) {
        match self.next().get_type() {
            // identifier
            TokenType::Id(_) => {
                self.parameter_declaration();
                self.parameter_declaration_tail();
            }
            // e
            _ => {}
        };
    }

    // parameter_declaration_tail -> "," parameter_declaration parameter_declaration_tail | e
    fn parameter_declaration_tail(&mut self) {
        match self.next().get_type() {
            TokenType::Comma => {
                self.eat(TokenType::Comma);
                self.parameter_declaration();
                self.parameter_declaration_tail();
            }
            _ => {}
        };
    }

    // parameter_declaration -> identifier ":" type
    fn parameter_declaration(&mut self) {
        self.parse_identifier();
        self.eat(TokenType::Colon);
        self.parse_type();
    }

    // type,  TODO: add support for custom defined types in the future
    fn parse_type(&mut self) {
        match self.next().get_type() {
            TokenType::I8 => self.eat(TokenType::I8),
            TokenType::I16 => self.eat(TokenType::I16),
            TokenType::I32 => self.eat(TokenType::I32),
            TokenType::I64 => self.eat(TokenType::I64),
            TokenType::U8 => self.eat(TokenType::U8),
            TokenType::U16 => self.eat(TokenType::U16),
            TokenType::U32 => self.eat(TokenType::U32),
            TokenType::U64 => self.eat(TokenType::U64),
            _ => {
                panic!("Unexpected token: {:?}", self.next());
            }
        }
    }

    // TODO: find out what AST node this should be
    // identifier
    fn parse_identifier(&mut self) -> {
        self.eat(TokenType::Id(String::new()));
    }

    // body -> statement body | e
    fn parse_body(&mut self) {
        match self.next().get_type() {
            TokenType::Let
            | TokenType::Id(_)
            | TokenType::Num(_)
            | TokenType::Return
            | TokenType::Lbra
            | TokenType::If
            | TokenType::While => {
                self.parse_statement();
                self.parse_body();
            }
            _ => {}
        }
    }

    // statement -> block_statement | non_block_statement ";"
    fn parse_statement(&mut self) {
        match self.next().get_type() {
            TokenType::Let
            | TokenType::Id(_)
            | TokenType::Return
            | TokenType::Num(_)
            | TokenType::Lbra => {
                self.parse_non_block_statement();
                self.eat(TokenType::Semi)
            }
            TokenType::If | TokenType::While => self.parse_block_statement(),
            _ => panic!("Unexpected token {:?}", self.next()),
        }
    }

    // block_statement -> if_statement | while_statement
    fn parse_block_statement(&mut self) {
        match self.next().get_type() {
            TokenType::If => self.parse_if_statement(),
            TokenType::While => self.parse_while_statement(),
            _ => panic!("Unexpected token {:?}", self.next()),
        };
    }

    // non_block_statement -> declaration_statement | assignment_statement | return_statement |
    // expression
    // This has a little cheat which makes it LL(2)
    fn parse_non_block_statement(&mut self) -> Statement {
        match self.next().get_type() {
            TokenType::Let => self.parse_declaration_statement(),
            TokenType::Id(_) => match self.next_next().get_type() {
                TokenType::Eq => self.parse_assignment_statement(),
                _ => self.parse_expression(),
            },
            TokenType::Return => self.parse_return_statement(),
            TokenType::Num(_) | TokenType::Lbra => self.parse_expression(),
            _ => panic!("Unexpected token {:?}", self.next()),
        }
    }

    // if_statement -> "if" expression "{" body "}" maybe_else_statement
    fn parse_if_statement(&mut self) {
        self.eat(TokenType::If);
        self.parse_expression();
        self.eat(TokenType::Lcur);
        self.parse_body();
        self.eat(TokenType::Rcur);
        self.parse_maybe_else_statement();
    }

    // maybe_else_statement -> "else" else_statement | e
    fn parse_maybe_else_statement(&mut self) {
        match self.next().get_type() {
            TokenType::Else => {
                self.eat(TokenType::Else);
                self.parse_else_statement();
            }
            _ => {}
        };
    }

    // else_statement -> if_statement | "{" body "}"
    fn parse_else_statement(&mut self) {
        match self.next().get_type() {
            TokenType::If => {
                self.parse_if_statement();
            }
            TokenType::Lcur => {
                self.eat(TokenType::Lcur);
                self.parse_body();
                self.eat(TokenType::Rcur);
            }
            _ => panic!("Unexpected token {:?}", self.next()),
        }
    }

    // while_statement -> "while" expression "{" body "}"
    fn parse_while_statement(&mut self) {
        self.eat(TokenType::While);
        self.parse_expression();
        self.eat(TokenType::Lcur);
        self.parse_body();
        self.eat(TokenType::Rcur);
    }

    // declaration_statement -> "let" identifier ":" type "=" expression
    fn parse_declaration_statement(&mut self) -> DeclarationStatement {
        self.eat(TokenType::Let);
        self.parse_identifier();
        self.eat(TokenType::Colon);
        self.parse_type();
        self.eat(TokenType::Eq);
        self.parse_expression();
    }

    // assignment_statement -> lvalue "=" expression
    fn parse_assignment_statement(&mut self) {
        self.parse_lvalue();
        self.eat(TokenType::Eq);
        self.parse_expression();
    }

    // lvalue -> identifier
    fn parse_lvalue(&mut self) {
        match self.next().get_type() {
            TokenType::Id(_) => self.eat(TokenType::Id(String::new())),
            _ => panic!("Unexpected token {:?}", self.next()),
        }
    }

    // return_statement -> "return" maybe_expression
    fn parse_return_statement(&mut self) {
        self.eat(TokenType::Return);
        self.parse_maybe_expression();
    }

    // maybe_expression -> expression | e
    fn parse_maybe_expression(&mut self) {
        match self.next().get_type() {
            TokenType::Semi => {}
            _ => self.parse_expression(),
        }
    }

    // expression -> term expression'
    fn parse_expression(&mut self) {
        // number, identifier, (
        self.parse_term();
        self.parse_expression_prime();
    }

    // term -> factor term'
    fn parse_term(&mut self) {
        // number, identifier, (
        self.parse_factor();
        self.parse_term_prime();
    }

    // factor -> number | identifier | function_call | "(" expression ")"
    fn parse_factor(&mut self) {
        // number, identifier, (
        match self.next().get_type() {
            TokenType::Num(_) => {
                self.parse_number();
            }
            TokenType::Id(_) => match self.next_next().get_type() {
                TokenType::Lbra => self.parse_function_call(),
                _ => self.parse_identifier(),
            },
            TokenType::Lbra => {
                self.eat(TokenType::Lbra);
                self.parse_expression();
                self.eat(TokenType::Rbra);
            }
            _ => {
                panic!("Unexpected token {:?}", self.next())
            }
        }
    }

    fn parse_function_call(&mut self) {
        self.parse_identifier();
        self.eat(TokenType::Lbra);
        self.parse_parameter_list();
        self.eat(TokenType::Rbra)
    }

    // parameter_list -> expression parameter_list_tail
    fn parse_parameter_list(&mut self) {
        match self.next().get_type() {
            TokenType::Num(_) | TokenType::Id(_) | TokenType::Lbra => {
                self.parse_expression();
                self.eat(TokenType::Comma);
                self.parse_parameter_list_tail();
            }
            _ => {}
        }
    }

    // parameter_list_tail -> , expression parameter_list_tail
    fn parse_parameter_list_tail(&mut self) {
        match self.next().get_type() {
            TokenType::Comma => {
                self.eat(TokenType::Comma);
                self.parse_expression();
                self.parse_parameter_list_tail();
            }
            _ => {}
        }
    }

    fn parse_number(&mut self) -> LiteralExpression {
        let le = match self.next().get_type() {
            TokenType::Num(n) => LiteralExpression::U64(n.clone()),
            _ => todo!(),
        };
        // number
        self.eat(TokenType::Num(String::new()));
        le
    }

    fn parse_term_prime(&mut self) {
        match self.next().get_type() {
            TokenType::Mul => {
                self.eat(TokenType::Mul);
                self.parse_factor();
                self.parse_term_prime();
            }
            TokenType::Div => {
                self.eat(TokenType::Div);
                self.parse_factor();
                self.parse_term_prime();
            }
            TokenType::Mod => {
                self.eat(TokenType::Mod);
                self.parse_factor();
                self.parse_term_prime();
            }
            _ => {}
        }
    }

    fn parse_expression_prime(&mut self) {
        match self.next().get_type() {
            TokenType::Plus => {
                self.eat(TokenType::Plus);
                self.parse_term();
                self.parse_expression_prime();
            }
            TokenType::Min => {
                self.eat(TokenType::Min);
                self.parse_term();
                self.parse_expression_prime();
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dummy() {
        let mut parser = Parser::new(Vec::new());
        parser.parse();
    }
}

/*
E -> T E'
E' -> + T E' | - T E' | e
T -> F T'
T' -> * F T' | / F T' | % F T' | e
F -> ...
*/
