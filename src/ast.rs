use crate::span::Span;

#[derive(Debug, Clone)]
pub enum Op {
    Add(Span),
    Sub(Span),
    Mult(Span),
    Div(Span),
    CondEq(Span),
    CondG(Span),
    CondGEq(Span),
    CondL(Span),
    CondLEq(Span),
}

#[derive(Debug, Clone)]
pub enum VarType {
    Normal(Span, String),
    Array(Span, Box<VarType>, Option<usize>),
    Pointer(Span, Box<VarType>),
    Func(Span, Box<VarType>, Vec<ArgDecl>)
}

#[derive(Debug, Clone)]
pub struct ArgDecl {
    pub span: Span,
    pub name: Identifier,
    pub kind: VarType
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub span: Span,
    pub name: String
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(Span, u32),
    Identifier(Identifier),
    BinaryOp(Span, Box<Expression>, Op, Box<Expression>)
}

#[derive(Debug, Clone)]
pub enum Statement {
    Declaration {
        span: Span,
        vartype: VarType,
        target: Identifier,
        expr: Expression,
    },
    Assignment {
        span: Span,
        target: Expression,
        expr: Expression,
    },
    Expr {
        span: Span,
        expr: Expression,
    },
    Body {
        span: Span,
        content: Vec<Box<Statement>>
    },
    IfStatement {
        span: Span,
        cond: Expression,
        body: Box<Statement>,
        child: Box<Option<Statement>>,
    },
}