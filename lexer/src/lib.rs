use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq)]
pub enum TokenType {
    Id(String),
    Num(String),
    Plus,
    Min,
    Mul,
    Div,
    Mod,
    Eq,
    Eqeq,
    Lt,
    Lteq,
    Gt,
    Gteq,
    Lbra,
    Rbra,
    Lang,
    Rang,
    Lcur,
    Rcur,
    Semi,
    If,
    Else,
    While,
    For,
    Return,
    Int,
    Void,
    Invalid,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            TokenType::Semi => format!(";"),
            TokenType::Rcur => format!("}}"),
            TokenType::Lcur => format!("{{"),
            TokenType::Lang => format!("["),
            TokenType::Rang => format!("]"),
            TokenType::Lbra => format!("("),
            TokenType::Rbra => format!(")"),
            TokenType::Lt => format!("<"),
            TokenType::Lteq => format!("<="),
            TokenType::Gt => format!(">"),
            TokenType::Gteq => format!(">="),
            TokenType::Eq => format!("="),
            TokenType::Eqeq => format!("=="),
            TokenType::Plus => format!("+"),
            TokenType::Min => format!("-"),
            TokenType::Mul => format!("*"),
            TokenType::Div => format!("/"),
            TokenType::Mod => format!("%"),
            TokenType::Id(id) => format!("id:{}", id),
            TokenType::Num(num) => format!("num:{}", num),
            TokenType::If => format!("if"),
            TokenType::Else => format!("else"),
            TokenType::While => format!("while"),
            TokenType::For => format!("for"),
            TokenType::Return => format!("ret"),
            TokenType::Int => format!("int"),
            TokenType::Void => format!("void"),
            TokenType::Invalid => format!("INVALID"),
        };
        write!(f, "{}", t)
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ {}:{}:{:?} ]",
            self.line_nr, self.line_index, self.token_type
        )
    }
}

/*
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.line_nr == other.line_nr
            && self.line_index == other.line_index
            && self.token_type == other.token_type
    }
}
*/

#[derive(PartialEq)]
struct Token {
    token_type: TokenType,
    line_nr: usize,
    line_index: usize,
}

impl Token {
    fn new(token_type: TokenType, line_nr: usize, line_index: usize) -> Self {
        Self {
            token_type,
            line_nr,
            line_index,
        }
    }
}

pub struct Lexer {
    path: String,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            tokens: Vec::new(),
        }
    }

    pub fn tokenise(&mut self) {
        let file = File::open(self.path.clone()).unwrap();
        let lines = BufReader::new(file).lines();

        for (nr, line) in lines.map_while(Result::ok).enumerate() {
            let tokens = Lexer::tokenise_line(&line, nr);
            for token in tokens {
                self.tokens.push(token);
            }
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn tokenise_line(line: &str, line_nr: usize) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut i = 0;

        loop {
            let token_type; //dev/ = TokenType::Invalid;
            let index; // = 0;
            let c = line.chars().nth(i);
            let c = match c {
                Some(c) => c,
                None => break,
            };

            if c.is_whitespace() {
                i += 1;
                continue;
            }
            // Skip
            else if c == ';' {
                token_type = TokenType::Semi;
                index = i;
                i += 1;
            } else if c == '(' {
                token_type = TokenType::Lbra;
                index = i;
                i += 1;
            } else if c == ')' {
                token_type = TokenType::Rbra;
                index = i;
                i += 1;
            } else if c == '[' {
                token_type = TokenType::Lang;
                index = i;
                i += 1;
            } else if c == ']' {
                token_type = TokenType::Rang;
                index = i;
                i += 1;
            } else if c == '{' {
                token_type = TokenType::Lcur;
                index = i;
                i += 1;
            } else if c == '}' {
                token_type = TokenType::Rcur;
                index = i;
                i += 1;
            } else if c == '+' {
                token_type = TokenType::Plus;
                index = i;
                i += 1;
            } else if c == '-' {
                token_type = TokenType::Min;
                index = i;
                i += 1;
            } else if c == '*' {
                token_type = TokenType::Mul;
                index = i;
                i += 1;
            } else if c == '/' {
                let c = line.chars().nth(i + 1);

                match c {
                    None => {
                        token_type = TokenType::Div;
                        index = i;
                        i += 1;
                    }
                    Some(c) => {
                        if c == '/' {
                            break;
                        } // Comment
                        token_type = TokenType::Div;
                        index = i;
                        i += 1;
                    }
                }
            } else if c == '%' {
                token_type = TokenType::Mod;
                index = i;
                i += 1;
            } else if c == '=' {
                let c = line.chars().nth(i + 1);

                match c {
                    None => {
                        token_type = TokenType::Eq;
                        index = i;
                        i += 1;
                    }
                    Some(c) => {
                        if c == '=' {
                            token_type = TokenType::Eqeq;
                            index = i;
                            i += 2;
                        } else {
                            token_type = TokenType::Eq;
                            index = i;
                            i += 1;
                        }
                    }
                }
            } else if c == '<' {
                let c = line.chars().nth(i + 1);

                match c {
                    None => {
                        token_type = TokenType::Lt;
                        index = i;
                        i += 1;
                    }
                    Some(c) => {
                        if c == '=' {
                            token_type = TokenType::Lteq;
                            index = i;
                            i += 2;
                        } else {
                            token_type = TokenType::Lt;
                            index = i;
                            i += 1;
                        }
                    }
                }
            } else if c == '>' {
                let c = line.chars().nth(i + 1);

                match c {
                    None => {
                        token_type = TokenType::Gt;
                        index = i;
                        i += 1;
                    }
                    Some(c) => {
                        if c == '=' {
                            token_type = TokenType::Gteq;
                            index = i;
                            i += 2;
                        } else {
                            token_type = TokenType::Gt;
                            index = i;
                            i += 1;
                        }
                    }
                }
            }
            // Identifier
            else if c.is_alphabetic() || c == '_' {
                let mut id = String::new();
                id.push(c);
                index = i;
                i += 1;
                loop {
                    let c = line.chars().nth(i);
                    let c = match c {
                        None => break,
                        Some(c) => c,
                    };
                    if c.is_alphanumeric() || c == '_' {
                        id.push(c);
                        i += 1;
                    } else {
                        break;
                    }
                }
                if "if" == &id {
                    token_type = TokenType::If;
                } else if "else" == &id {
                    token_type = TokenType::Else;
                } else if "while" == &id {
                    token_type = TokenType::While;
                } else if "for" == &id {
                    token_type = TokenType::For;
                } else if "int" == &id {
                    token_type = TokenType::Int;
                } else if "void" == &id {
                    token_type = TokenType::Void;
                } else if "return" == &id {
                    token_type = TokenType::Return;
                } else {
                    token_type = TokenType::Id(id);
                }
            }
            // Numeric (integer only)
            else if c.is_numeric() {
                let mut num = String::new();
                num.push(c);
                index = i;
                i += 1;
                loop {
                    let c = line.chars().nth(i);
                    let c = match c {
                        None => break,
                        Some(c) => c,
                    };
                    if c.is_numeric() {
                        num.push(c);
                        i += 1;
                    } else {
                        break;
                    }
                }
                token_type = TokenType::Num(num);
            } else {
                panic!("Invalid character {}", c);
            }

            tokens.push(Token::new(token_type, line_nr, index));
        }

        tokens
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn file_1dotc() {
        let path = "test_files/1.c";
        let result = vec![
            Token::new(TokenType::Int, 0, 0),
            Token::new(TokenType::Id(String::from("double")), 0, 4),
            Token::new(TokenType::Lbra, 0, 10),
            Token::new(TokenType::Int, 0, 11),
            Token::new(TokenType::Id(String::from("v")), 0, 15),
            Token::new(TokenType::Rbra, 0, 16),
            Token::new(TokenType::Lcur, 0, 18),
            Token::new(TokenType::Return, 1, 1),
            Token::new(TokenType::Id(String::from("v")), 1, 8),
            Token::new(TokenType::Mul, 1, 10),
            Token::new(TokenType::Num(String::from("2")), 1, 12),
            Token::new(TokenType::Semi, 1, 13),
            Token::new(TokenType::Rcur, 2, 0),
        ];
        let mut lexer = Lexer::new(&path);
        lexer.tokenise();

        assert_eq!(lexer.tokens.len(), result.len());
        for i in 0..result.len() {
            assert_eq!(result[i], lexer.tokens[i]);
        }
    }
}
