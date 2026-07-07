#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    FloatLit(f64),
    StringLit(String),
    Identifier(String),
    Boolean(bool),
    FuncCall {
        name: String,
        args: Vec<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: Op,
        right: Box<Expr>,
    },
}
#[derive(Debug, Clone)]
pub enum Op {
    Add,     // +
    Sub,     // -
    Mul,     // *
    Div,     // /
    Percent, // %
    Eq,      // ==
    Neq,     // !=
    Lt,      // <
    Gt,      // >
    Le,      // <=
    Ge,      // >=
}
#[derive(Debug, Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    I128,
    F32,
    F64,
    Bool,
    Void,
    Ptr(Box<Type>),
    Custom(String),
}

#[derive(Debug, Clone)]
pub enum AST {
    VarDecl {
        name: String,
        value: Expr,
        ty: Option<Type>,
    },
    FuncDecl {
        name: String,
        args: Vec<(String, Type)>,
        body: Vec<AST>,
        return_type: Option<Type>,
    },
    Return {
        value: Expr,
    },
    Loop {
        body: Vec<AST>,
    },
    ExprStmt(Expr),

    AsmBlock {
        assembly: String,
    },
    ExternFn {
        name: String,
        args: Vec<(String, Type)>,
        return_type: Option<Type>,
    },
    If {
        condition: Expr,
        then_branch: Vec<AST>,
        else_branch: Option<Vec<AST>>,
    },
    While {
        condition: Expr,
        body: Vec<AST>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Vec<AST>,
    },
    Break
}
