use std::fmt::Debug;

use ast::*;
use lexer::{self, Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
    program: Option<Program>,
    errors: Vec<String>,
}

impl Debug for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.program)
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut parser = Parser {
            tokens,
            index: 0,
            program: None,
            errors: Vec::new(),
        };
        parser.parse();
        parser
    }

    fn eat(&mut self, token_type: TokenType) {
        if self.index >= self.tokens.len() {
            panic!("Out of bounds!");
        }

        let t = self.next();

        if !(std::mem::discriminant(&token_type) == std::mem::discriminant(t.get_type())) {
            // Do not advance the index, this simulates as the correct token being eaten
            self.error(vec![token_type]);
        } else {
            self.index += 1;
        }
    }

    fn error(&mut self, expected: Vec<TokenType>) {
        let t = self.next();
        let mut error_msg = format!(
            "Wrong token at {}:{}, expected ",
            t.get_line_nr(),
            t.get_line_index()
        );

        for i in 0..(expected.len() - 1) {
            error_msg.push_str(format!("\"{}\", ", expected[i].debug_type()).as_str());
        }

        error_msg
            .push_str(format!("or \"{}\"", expected[expected.len() - 1].debug_type()).as_str());

        self.errors.push(error_msg);
    }

    fn next(&self) -> &Token {
        &self.tokens[self.index]
    }

    fn next_next(&self) -> &Token {
        &self.tokens[self.index + 1]
    }

    fn parse(&mut self) {
        self.program = Some(self.program());
        println!("Parsing succeeded YAY!");
    }

    // program -> function_list "$"
    fn program(&mut self) -> Program {
        let mut declarations = Vec::new();
        match *self.next().get_type() {
            // fn
            TokenType::Fn => {
                declarations.append(&mut self.function_list());
                self.eat(TokenType::End);
            }
            _ => {
                self.error(vec![TokenType::Fn]);
            }
        };
        Program::new(declarations)
    }

    // function_list -> function function_list | e
    fn function_list(&mut self) -> Vec<Declaration> {
        let mut function_declarations = Vec::new();
        match self.next().get_type() {
            // fn
            TokenType::Fn => {
                function_declarations.push(Declaration::FunctionDeclaration(self.function()));
                function_declarations.append(&mut self.function_list());
            }
            // e
            _ => {}
        };
        function_declarations
    }

    // function -> "fn" identifier "(" parameter_declaration_list ")" "->" type "{" body "}"
    fn function(&mut self) -> FunctionDeclaration {
        self.eat(TokenType::Fn);
        let name = self.parse_identifier();
        self.eat(TokenType::Lbra);
        let parameters = self.parameter_declaration_list();
        self.eat(TokenType::Rbra);
        self.eat(TokenType::Arrow);
        let return_type = self.parse_type();
        self.eat(TokenType::Lcur);
        let body = self.parse_body();
        self.eat(TokenType::Rcur);
        FunctionDeclaration::new(name, parameters, return_type, body)
    }

    // paramter_declaration_list -> parameter_declaration parameter_declaration_tail | e
    fn parameter_declaration_list(&mut self) -> Vec<FunctionParameterDeclaration> {
        let mut function_parameter_declarations = Vec::new();
        match self.next().get_type() {
            // identifier
            TokenType::Id(_) => {
                function_parameter_declarations.push(self.parameter_declaration());
                function_parameter_declarations.append(&mut self.parameter_declaration_tail());
            }
            // e
            _ => {}
        };
        function_parameter_declarations
    }

    // parameter_declaration_tail -> "," parameter_declaration parameter_declaration_tail | e
    fn parameter_declaration_tail(&mut self) -> Vec<FunctionParameterDeclaration> {
        let mut function_parameter_declarations = Vec::new();
        match self.next().get_type() {
            TokenType::Comma => {
                self.eat(TokenType::Comma);
                function_parameter_declarations.push(self.parameter_declaration());
                function_parameter_declarations.append(&mut self.parameter_declaration_tail());
            }
            _ => {}
        };
        function_parameter_declarations
    }

    // parameter_declaration -> identifier ":" type
    fn parameter_declaration(&mut self) -> FunctionParameterDeclaration {
        let name = self.parse_identifier();
        self.eat(TokenType::Colon);
        let datatype = self.parse_type();
        FunctionParameterDeclaration::new(name, datatype)
    }

    // type,  TODO: add support for custom defined types in the future
    fn parse_type(&mut self) -> Type {
        let (eat, datatype) = match self.next().get_type() {
            TokenType::I8 => (TokenType::I8, SimpleType::I8),
            TokenType::I16 => (TokenType::I16, SimpleType::I16),
            TokenType::I32 => (TokenType::I32, SimpleType::I32),
            TokenType::I64 => (TokenType::I64, SimpleType::I64),
            TokenType::U8 => (TokenType::U8, SimpleType::U8),
            TokenType::U16 => (TokenType::U16, SimpleType::U16),
            TokenType::U32 => (TokenType::U32, SimpleType::U32),
            TokenType::U64 => (TokenType::U64, SimpleType::U64),
            TokenType::Void => (TokenType::Void, SimpleType::Void),
            _ => {
                self.error(vec![TokenType::I8]); // Any of the simple types can be chosen, they all fall
                // under "type"
                return Type::SimpleType(SimpleType::U64);
            }
        };

        self.eat(eat);
        Type::SimpleType(datatype)
    }

    fn parse_identifier(&mut self) -> String {
        let value = match self.next().get_type() {
            TokenType::Id(identifier) => identifier.clone(),
            _ => {
                self.error(vec![TokenType::Id(String::new())]);
                return String::new();
            }
        };
        self.eat(TokenType::Id(String::new()));
        value
    }

    // body -> statement body | e
    fn parse_body(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        match self.next().get_type() {
            TokenType::Let
            | TokenType::Id(_)
            | TokenType::Num(_)
            | TokenType::Return
            | TokenType::Lbra
            | TokenType::If
            | TokenType::While => {
                statements.push(self.parse_statement());
                statements.append(&mut self.parse_body());
            }
            _ => {}
        }
        statements
    }

    // statement -> block_statement | non_block_statement ";"
    fn parse_statement(&mut self) -> Statement {
        match self.next().get_type() {
            TokenType::Let
            | TokenType::Id(_)
            | TokenType::Return
            | TokenType::Num(_)
            | TokenType::Lbra => {
                let statement = self.parse_non_block_statement();
                self.eat(TokenType::Semi);
                statement
            }
            TokenType::If | TokenType::While => self.parse_block_statement(),
            _ => {
                self.error(vec![
                TokenType::Let,
                TokenType::Id(String::new()),
                TokenType::Return,
                TokenType::Num(String::new()),
                TokenType::Lbra,
                TokenType::If,
                TokenType::While,
            ]);
                return Statement::
            },
        }
    }

    // block_statement -> if_statement | while_statement
    fn parse_block_statement(&mut self) -> Statement {
        match self.next().get_type() {
            TokenType::If => Statement::IfStatement(self.parse_if_statement()),
            TokenType::While => Statement::WhileStatement(self.parse_while_statement()),
            _ => self.error(vec![TokenType::If, TokenType::While]),
        }
    }

    // non_block_statement -> declaration_statement | assignment_statement | return_statement |
    // expression
    // This has a little cheat which makes it LL(2)
    fn parse_non_block_statement(&mut self) -> Statement {
        match self.next().get_type() {
            TokenType::Let => Statement::DeclarationStatement(self.parse_declaration_statement()),
            TokenType::Id(_) => match self.next_next().get_type() {
                TokenType::Eq => {
                    Statement::VariableAssignmentStatement(self.parse_assignment_statement())
                }
                _ => Statement::Expression(self.parse_expression()),
            },
            TokenType::Return => Statement::ReturnStatement(self.parse_return_statement()),
            TokenType::Num(_) | TokenType::Lbra => Statement::Expression(self.parse_expression()),
            _ => self.error(vec![
                TokenType::Let,
                TokenType::Id(String::new()),
                TokenType::Return,
                TokenType::Num(String::new()),
            ]),
        }
    }

    // if_statement -> "if" expression "{" body "}" maybe_else_statement
    fn parse_if_statement(&mut self) -> IfStatement {
        self.eat(TokenType::If);
        let condition = self.parse_expression();
        self.eat(TokenType::Lcur);
        let body = self.parse_body();
        self.eat(TokenType::Rcur);
        let tail = self.parse_maybe_else_statement();
        IfStatement::new(condition, body, tail)
    }

    // maybe_else_statement -> "else" else_statement | e
    fn parse_maybe_else_statement(&mut self) -> Option<IfStatement> {
        match self.next().get_type() {
            TokenType::Else => {
                self.eat(TokenType::Else);
                Some(self.parse_else_statement())
            }
            _ => None,
        }
    }

    // else_statement -> if_statement | "{" body "}"
    fn parse_else_statement(&mut self) -> IfStatement {
        match self.next().get_type() {
            TokenType::If => self.parse_if_statement(),
            TokenType::Lcur => {
                self.eat(TokenType::Lcur);
                let body = self.parse_body();
                self.eat(TokenType::Rcur);
                IfStatement::make_else(body)
            }
            _ => self.error(vec![TokenType::If, TokenType::Lcur]),
        }
    }

    // while_statement -> "while" expression "{" body "}"
    fn parse_while_statement(&mut self) -> WhileStatement {
        self.eat(TokenType::While);
        let condition = self.parse_expression();
        self.eat(TokenType::Lcur);
        let body = self.parse_body();
        self.eat(TokenType::Rcur);
        WhileStatement::new(condition, body)
    }

    // declaration_statement -> "let" identifier ":" type "=" expression
    fn parse_declaration_statement(&mut self) -> DeclarationStatement {
        self.eat(TokenType::Let);
        let variable = self.parse_identifier();
        self.eat(TokenType::Colon);
        let datatype = self.parse_type();
        self.eat(TokenType::Eq);
        let value = self.parse_expression();
        DeclarationStatement::new(variable, datatype, value)
    }

    // assignment_statement -> lvalue "=" expression
    fn parse_assignment_statement(&mut self) -> VariableAssignmentStatement {
        let variable = self.parse_lvalue();
        self.eat(TokenType::Eq);
        let value = self.parse_expression();
        VariableAssignmentStatement::new(variable, value)
    }

    // lvalue -> identifier
    fn parse_lvalue(&mut self) -> String {
        match self.next().get_type() {
            TokenType::Id(_) => self.parse_identifier(),
            _ => self.error(vec![TokenType::Id(String::new())]),
        }
    }

    // return_statement -> "return" maybe_expression
    fn parse_return_statement(&mut self) -> Option<Expression> {
        self.eat(TokenType::Return);
        self.parse_maybe_expression()
    }

    // maybe_expression -> expression | e
    fn parse_maybe_expression(&mut self) -> Option<Expression> {
        match self.next().get_type() {
            TokenType::Semi => None,
            _ => Some(self.parse_expression()),
        }
    }

    // expression -> term expression'
    fn parse_expression(&mut self) -> Expression {
        // number, identifier, (
        let expression = self.parse_term();
        let expression = self.parse_expression_prime(expression);
        expression
    }

    // term -> factor term'
    fn parse_term(&mut self) -> Expression {
        // number, identifier, (
        let expression = self.parse_factor();
        let expression = self.parse_term_prime(expression);
        expression
    }

    // factor -> number | identifier | function_call | "(" expression ")"
    fn parse_factor(&mut self) -> Expression {
        // number, identifier, (
        match self.next().get_type() {
            TokenType::Num(_) => {
                let value = self.parse_number();
                Expression::LiteralExpression(LiteralExpression::I64(value))
            }
            TokenType::Id(_) => match self.next_next().get_type() {
                TokenType::Lbra => Expression::FunctionCallExpression(self.parse_function_call()),
                _ => {
                    let value = self.parse_identifier();
                    Expression::VariableReferenceExpression(value)
                }
            },
            TokenType::Lbra => {
                self.eat(TokenType::Lbra);
                let expression = self.parse_expression();
                self.eat(TokenType::Rbra);
                expression
            }
            _ => {
                self.error(vec![
                    TokenType::Num(String::new()),
                    TokenType::Id(String::new()),
                    TokenType::Lbra,
                ]);
            }
        }
    }

    fn parse_function_call(&mut self) -> FunctionCallExpression {
        let name = self.parse_identifier();
        let mut function_call_expression = FunctionCallExpression::new(name);
        self.eat(TokenType::Lbra);
        let parameters = self.parse_parameter_list();
        function_call_expression.add_parameters(parameters);
        self.eat(TokenType::Rbra);
        function_call_expression
    }

    // parameter_list -> expression parameter_list_tail
    fn parse_parameter_list(&mut self) -> Vec<Expression> {
        let mut parameters: Vec<Expression> = Vec::new();
        match self.next().get_type() {
            TokenType::Num(_) | TokenType::Id(_) | TokenType::Lbra => {
                let expression = self.parse_expression();
                parameters.push(expression);
                let mut rest_parameters = self.parse_parameter_list_tail();
                parameters.append(&mut rest_parameters);
            }
            _ => {}
        }
        parameters
    }

    // parameter_list_tail -> , expression parameter_list_tail
    fn parse_parameter_list_tail(&mut self) -> Vec<Expression> {
        let mut parameters: Vec<Expression> = Vec::new();
        match self.next().get_type() {
            TokenType::Comma => {
                self.eat(TokenType::Comma);
                let expression = self.parse_expression();
                parameters.push(expression);
                let mut rest_parameters = self.parse_parameter_list_tail();
                parameters.append(&mut rest_parameters);
            }
            _ => {}
        };
        parameters
    }

    fn parse_number(&mut self) -> String {
        let value = match self.next().get_type() {
            TokenType::Num(n) => n.clone(),
            _ => todo!(),
        };
        // number
        self.eat(TokenType::Num(String::new()));
        value
    }

    fn parse_term_prime(&mut self, left: Expression) -> Expression {
        let (eat, operator) = match self.next().get_type() {
            TokenType::Mul => (TokenType::Mul, Operator::Mul),
            TokenType::Div => (TokenType::Div, Operator::Div),
            TokenType::Mod => (TokenType::Mod, Operator::Mod),
            _ => return left,
        };

        self.eat(eat);
        let right = self.parse_factor();
        let bin_op_expression = BinOpExpression::new(left, right, operator);
        let expression = self.parse_term_prime(Expression::BinOpExpression(bin_op_expression));
        expression
    }

    fn parse_expression_prime(&mut self, left: Expression) -> Expression {
        let (eat, operator) = match self.next().get_type() {
            TokenType::Plus => (TokenType::Plus, Operator::Plus),
            TokenType::Min => (TokenType::Min, Operator::Min),
            _ => return left,
        };

        self.eat(eat);
        let right = self.parse_term();
        let bin_op_expression = BinOpExpression::new(left, right, operator);
        let expression =
            self.parse_expression_prime(Expression::BinOpExpression(bin_op_expression));
        expression
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
