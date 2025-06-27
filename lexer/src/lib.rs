use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Clone)]
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
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Void,
    Fn,
    Colon,
    Arrow,
    End,
    Comma,
    Let,
    Invalid,
}

impl Debug for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = match self {
            TokenType::Semi => format!("SEMI"),
            TokenType::Rcur => format!("RCUR"),
            TokenType::Lcur => format!("LCUR"),
            TokenType::Lang => format!("LANG"),
            TokenType::Rang => format!("RANG"),
            TokenType::Lbra => format!("LBRA"),
            TokenType::Rbra => format!("RBRA"),
            TokenType::Lt => format!("LT"),
            TokenType::Lteq => format!("LTEQ"),
            TokenType::Gt => format!("GT"),
            TokenType::Gteq => format!("GTEQ"),
            TokenType::Eq => format!("EQ"),
            TokenType::Eqeq => format!("EQEQ"),
            TokenType::Plus => format!("PLUS"),
            TokenType::Min => format!("MIN"),
            TokenType::Mul => format!("MUL"),
            TokenType::Div => format!("DIV"),
            TokenType::Mod => format!("MOD"),
            TokenType::Id(id) => format!("ID:{}", id),
            TokenType::Num(num) => format!("NUM:{}", num),
            TokenType::If => format!("IF"),
            TokenType::Else => format!("ELSE"),
            TokenType::While => format!("WHILE"),
            TokenType::For => format!("FOR"),
            TokenType::Return => format!("RET"),
            TokenType::Void => format!("VOID"),
            TokenType::Invalid => format!("INVALID"),
            TokenType::Fn => format!("FN"),
            TokenType::Colon => format!("COLON"),
            TokenType::Arrow => format!("ARROW"),
            TokenType::End => format!("END"),
            TokenType::Comma => format!("COMMA"),
            TokenType::Let => format!("LET"),
            TokenType::I8 => format!("i8"),
            TokenType::I16 => format!("i16"),
            TokenType::I32 => format!("i32"),
            TokenType::I64 => format!("i64"),
            TokenType::U8 => format!("u8"),
            TokenType::U16 => format!("u16"),
            TokenType::U32 => format!("u32"),
            TokenType::U64 => format!("u64"),
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

#[derive(PartialEq, Clone)]
pub struct Token {
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

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn get_line_nr(&self) -> usize {
        self.line_nr
    }

    pub fn get_line_index(&self) -> usize {
        self.line_index
    }
}

pub struct Lexer {
    path: String,
    tokens: Vec<Token>,
}

impl Debug for Lexer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            write!(f, "{:?}", token).unwrap();
        }
        write!(f, "\n")
    }
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
        self.tokens.push(Token::new(TokenType::End, 0, 0));
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        self.tokens.clone()
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
                let c = line.chars().nth(i + 1);

                match c {
                    Some(c) => {
                        if c == '>' {
                            token_type = TokenType::Arrow;
                            index = i;
                            i += 2;
                        } else {
                            token_type = TokenType::Min;
                            index = i;
                            i += 1;
                        }
                    }
                    None => {
                        token_type = TokenType::Min;
                        index = i;
                        i += 1;
                    }
                }
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
            } else if c == ':' {
                token_type = TokenType::Colon;
                index = i;
                i += 1;
            } else if c == ',' {
                token_type = TokenType::Comma;
                index = i;
                i += 1;
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
                } else if "i8" == &id {
                    token_type = TokenType::I8;
                } else if "i16" == &id {
                    token_type = TokenType::I16;
                } else if "i32" == &id {
                    token_type = TokenType::I32;
                } else if "i64" == &id {
                    token_type = TokenType::I64;
                } else if "u8" == &id {
                    token_type = TokenType::U8;
                } else if "u16" == &id {
                    token_type = TokenType::U16;
                } else if "u32" == &id {
                    token_type = TokenType::U32;
                } else if "u64" == &id {
                    token_type = TokenType::U64;
                } else if "void" == &id {
                    token_type = TokenType::Void;
                } else if "return" == &id {
                    token_type = TokenType::Return;
                } else if "let" == &id {
                    token_type = TokenType::Let;
                } else if "fn" == &id {
                    token_type = TokenType::Fn;
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
            Token::new(TokenType::I32, 0, 0),
            Token::new(TokenType::Id(String::from("double")), 0, 4),
            Token::new(TokenType::Lbra, 0, 10),
            Token::new(TokenType::I32, 0, 11),
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
