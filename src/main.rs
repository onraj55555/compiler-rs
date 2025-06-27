use lexer::{self, Lexer};
use parser::{self, Parser};

fn main() {
    let mut lexer = Lexer::new("./code");
    lexer.tokenise();

    println!("{:?}", lexer);

    let mut parser = Parser::new(lexer.get_tokens());
    parser.parse();
}
