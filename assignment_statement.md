# Examples
fn some_function(a: int, b:int) -> int {
    return a + b;
}

fn print(a: int) -> void {
    // print an integer
}

fn main() -> int {
    let c : int = some_function(1, 2);
    print(c);
}

# Grammar
## Function
function -> "fn" identifier "(" parameter_list ")" "->" type "{" body "}"
parameter_list -> e | identifier ":" "int" | identifier ":" "int" "," parameter_list
body -> declaration_statement | assignment_statement | if_statement | while_statement | return_statement | function_call_statement
declaration_statement -> "let" identifier ":" "int" "=" expression ";"
assignment_statement -> identifier "=" expression ";"
