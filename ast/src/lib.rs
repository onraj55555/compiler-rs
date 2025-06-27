pub struct Program {
    declarations: Vec<Declaration>,
}

pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
}

pub struct FunctionDeclaration {
    name: String,
    parameters: Vec<FunctionParameterDeclaration>,
    body: Vec<Statement>,
}

pub struct FunctionParameterDeclaration {
    name: String,
    datatype: Type,
}

pub enum Statement {
    DeclarationStatement(DeclarationStatement),
    VariableAssignmentStatement(VariableAssignmentStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(Expression),
    Expression(Expression),
}

pub struct WhileStatement {
    condition: Expression,
    body: Vec<Statement>,
}

// if condition is None -> else statement
pub struct IfStatement {
    condition: Option<Expression>,
    body: Vec<Statement>,
    tail_conditions: Option<Box<IfStatement>>,
}

pub struct DeclarationStatement {
    variable: String,
    datatype: Type,
    value: Expression,
}

impl DeclarationStatement {
    fn new(variable: String, datatype: Type, value: Expression) -> Self {
        Self {
            variable,
            datatype,
            value,
        }
    }
}

pub struct VariableAssignmentStatement {
    variable: String,
    value: Expression,
}

pub enum Type {
    SimpleType(SimpleType),
}

pub enum SimpleType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Void,
}

pub enum Expression {
    BinOpExpression(BinOpExpression),
    LiteralExpression(LiteralExpression),
    VariableReferenceExpression(String),
    FunctionCallExpression(FunctionCallExpression),
}

pub struct BinOpExpression {
    left: Box<Expression>,
    op: Operator,
    right: Box<Expression>,
}

pub enum Operator {
    Plus,
    Min,
    Mul,
    Div,
    Mod,
}

pub struct FunctionCallExpression {
    name: String,
    parameters: Vec<Expression>,
}

pub enum LiteralExpression {
    I8(String),
    I16(String),
    I32(String),
    I64(String),
    U8(String),
    U16(String),
    U32(String),
    U64(String),
}
