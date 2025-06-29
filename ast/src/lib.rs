#[derive(Debug)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

impl Program {
    pub fn new(declarations: Vec<Declaration>) -> Self {
        Self { declarations }
    }
}

#[derive(Debug)]
pub enum Declaration {
    FunctionDeclaration(FunctionDeclaration),
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<FunctionParameterDeclaration>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

impl FunctionDeclaration {
    pub fn new(
        name: String,
        parameters: Vec<FunctionParameterDeclaration>,
        return_type: Type,
        body: Vec<Statement>,
    ) -> Self {
        Self {
            name,
            parameters,
            return_type,
            body,
        }
    }
}

#[derive(Debug)]
pub struct FunctionParameterDeclaration {
    pub name: String,
    pub datatype: Type,
}

impl FunctionParameterDeclaration {
    pub fn new(name: String, datatype: Type) -> Self {
        Self { name, datatype }
    }
}

#[derive(Debug)]
pub enum Statement {
    DeclarationStatement(DeclarationStatement),
    VariableAssignmentStatement(VariableAssignmentStatement),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    ReturnStatement(Option<Expression>),
    Expression(Expression),
}

#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

impl WhileStatement {
    pub fn new(condition: Expression, body: Vec<Statement>) -> Self {
        Self { condition, body }
    }
}

// if condition is None -> else statement
#[derive(Debug)]
pub struct IfStatement {
    pub condition: Option<Expression>,
    pub body: Vec<Statement>,
    pub tail_conditions: Option<Box<IfStatement>>,
}

impl IfStatement {
    pub fn new(condition: Expression, body: Vec<Statement>, tail: Option<IfStatement>) -> Self {
        let tail = match tail {
            Some(tail) => Some(Box::new(tail)),
            None => None,
        };
        Self {
            condition: Some(condition),
            body,
            tail_conditions: tail,
        }
    }

    pub fn make_else(body: Vec<Statement>) -> Self {
        Self {
            condition: None,
            body,
            tail_conditions: None,
        }
    }
}

#[derive(Debug)]
pub struct DeclarationStatement {
    pub variable: String,
    pub datatype: Type,
    pub value: Expression,
}

impl DeclarationStatement {
    pub fn new(variable: String, datatype: Type, value: Expression) -> Self {
        Self {
            variable,
            datatype,
            value,
        }
    }
}

#[derive(Debug)]
pub struct VariableAssignmentStatement {
    pub variable: String,
    pub value: Expression,
}

impl VariableAssignmentStatement {
    pub fn new(variable: String, value: Expression) -> Self {
        Self { variable, value }
    }
}

#[derive(Debug)]
pub enum Type {
    SimpleType(SimpleType),
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Expression {
    BinOpExpression(BinOpExpression),
    LiteralExpression(LiteralExpression),
    VariableReferenceExpression(String),
    FunctionCallExpression(FunctionCallExpression),
}

#[derive(Debug)]
pub struct BinOpExpression {
    pub left: Box<Expression>,
    pub op: Operator,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Min,
    Mul,
    Div,
    Mod,
}

impl BinOpExpression {
    pub fn new(left: Expression, right: Expression, op: Operator) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct FunctionCallExpression {
    pub name: String,
    pub parameters: Vec<Expression>,
}

impl FunctionCallExpression {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, parameter: Expression) {
        self.parameters.push(parameter)
    }

    pub fn add_parameters(&mut self, parameters: Vec<Expression>) {
        for parameter in parameters {
            self.add_parameter(parameter);
        }
    }
}

#[derive(Debug)]
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
