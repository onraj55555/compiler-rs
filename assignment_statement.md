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
program -> function function_tail
function_tail -> function function_tail | e
function -> "fn" identifier "(" parameter_declaration_list ")" "->" type "{" body "}" | e
parameter_declaration_list -> parameter_declaration parameter_declaration_tail | e
parameter_declaration_tail -> "," parameter_declaration parameter_declaration_tail | e
parameter_declaration -> identifier ":" type
body -> statement ";" body | e
statement -> declaration_statement | assignment_statement | if_statement | while_statement | return_statement | expression
declaration_statement -> "let" identifier ":" type "=" expression
assignment_statement -> identifier "=" expression
if_statement -> "if" expression "{" body "}" if_statement_rest
if_statement_rest -> e | "else" if_statement | "else" "{" body "}"
while_statement -> "while" expression "{" body "}"
return_statement -> "return" expression
expression -> term expression_prime
expression_prime -> "+" term expression_prime | "-" term expression_prime | e
term -> factor term_prime
term_prime -> "*" factor term_prime | "/" factor term_prime | "%" factor term_prime | e
factor -> number | identifier | function_call | "(" expression ")"
function_call -> identifier "(" parameter_list ")"
parameter_list -> parameter parameter_tail | e
parameter_tail -> "," parameter parameter_tail | e
parameter -> expression
type -> "int"

## Rewritten
program -> function_list $
function_list -> function function_list | e
function -> "fn" identifier "(" parameter_declaration_list ")" "->" type "{" body "}"
parameter_declaration_list -> parameter_declaration "," parameter_declaration_list | e
parameter_declaration -> identifier ":" type
body -> statement body | e
statement -> block_statement | non_block_statement ";"
block_statement -> if_statement | while_statement
non_block_statement -> declaration_statement | assignment_statement | return_statement | expression
declaration_statement -> "let" identifier ":" type "=" expression
assignment_statement -> identifier "=" expression
return_statement -> "return" expression
expression -> term expression_prime
expression_prime -> "+" term expression_prime | "-" term expression_prime | e
term -> factor term_prime
term_prime -> "*" factor term_prime | "/" factor term_prime | "%" factor term_prime | e
factor -> number | maybe_function_call | "(" expression ")"
maybe_function_call -> identifier maybe_function_call_rest
maybe_function_call_rest -> e | "(" parameter_list ")"
parameter_list -> parameter "," parameter_list | e
parameter -> expression
type -> "int"
