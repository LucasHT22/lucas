#[derive(Debug, Clone)]
pub enum Expr {
    Numero(f64),
    Texto(String),
    Bool(bool),
    Nulo,
    Var(String),
    Array(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Atribuir(String, Box<Expr>),
    AtribuirIndex(Box<Expr>, Box<Expr>, Box<Expr>),
    Unario(UnarioOp, Box<Expr>),
    Binario(Box<Expr>, BinOp, Box<Expr>),
    Chamada(Box<Expr>, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    Imprimir(Expr),
    VarDecl(String, Expr),
    Bloco(Vec<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    While(Expr, Box<Stmt>),
    For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
    FuncDecl(String, Vec<String>, Vec<Stmt>),
    Return(Option<Expr>),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub enum UnarioOp {
    Neg,
    Nao,
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}