#[derive(Debug, PartialEq, Clone)]
pub enum AssignOperator {
    Equals
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Plus
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Program {
        lines: Box<Vec<Expression>>,
    },
    Assign {
        operator: AssignOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Var {
        value: String,
    },
    Lambda {
        vars: Vec<String>,
        body: Box<Expression>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        args: Box<Vec<Expression>>,
    },
    Num {
        value: i64,
    },
    String {
        value: String,
    },
}
