use crate::ast::{Expr, Stmt, UnarioOp, BinOp};
use crate::environment::{Environment, EnvRef};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Text(String),
    Bool(bool),
    Nil,
    Function(Rc<Function>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::Function(_) => true,
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Number(n) => {
                if (n.fract() - 0.0).abs() < 1e-10 { format!("{}", *n as i64) } else { format!("{}", n) }
            }
            Value::Text(s) => s.clone(),
            Value::Bool(b) => format!("{}", b),
            Value::Nil => "nulo".into(),
            Value::Function(f) => format!("<fn {}>", f.name),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
    pub closure: EnvRef,
}

impl Function {
    pub fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, String> {
        if args.len() != self.params.len() {
            return Err(format!("Esperado {} argumentos mas recebeu {}", self.params.len(), args.len()));
        }
        let env = Rc::new(RefCell::new(Environment::with_enclosing(self.closure.clone())));
        for (p, a) in self.params.iter().zip(args.into_iter()) {
            env.borrow_mut().define(p.clone(), a);
        }
        let prev_env = interpreter.env.clone();
        interpreter.env = env.clone();
        let mut ret_val: Option<Value> = None;
        for stmt in &self.body {
            match interpreter.execute(stmt) {
                Ok(Some(v)) => { ret_val = Some(v); break; }
                Ok(None) => {}
                Err(e) => { interpreter.env = prev_env; return Err(e); }
            }
        }
        interpreter.env = prev_env;
        Ok(ret_val.unwrap_or(Value::Nil))
    }
}

pub struct Interpreter {
    pub globals: EnvRef,
    pub env: EnvRef,
}

impl Interpreter {
    pub fn new() -> Self { 
        let g = Rc::new(RefCell::new(Environment::new()));
        let it = Self { globals: g.clone(), env: g.clone() };
        {
            let print_fn = Rc::new(Function {
                name: "imprimir".into(),
                params: vec!["x".into()],
                body: vec![],
                closure: it.globals.clone(),
            });
            it.globals.borrow_mut().define("imprimir".into(), Value::Function(print_fn));
        }
        it
    }

    pub fn run(&mut self, source: &str) {
        let tokens = crate::lexer::Lexer::new(source).tokenize();
        let mut parser = crate::parser::Parser::new(tokens);
        let stmts = parser.parse();
        for s in stmts {
            if let Err(e) = self.execute(&s) {
                eprintln!("Erro: {}", e);
                break;
            }
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::ExprStmt(e) => { self.evaluate(e)?; Ok(None) }
            Stmt::Imprimir(e) => {
                let v = self.evaluate(e)?;
                println!("{}", v.to_string_repr());
                Ok(None)
            }
            Stmt::VarDecl(name, init) => {
                let v = self.evaluate(init)?;
                self.env.borrow_mut().define(name.clone(), v);
                Ok(None)
            }
            Stmt::Bloco(stmts) => {
                let new_env = Rc::new(RefCell::new(Environment::with_enclosing(self.env.clone())));
                let prev = self.env.clone();
                self.env = new_env;
                for s in stmts {
                    let r = self.execute(s)?;
                    if r.is_some() {
                        self.env = prev;
                        return Ok(r);
                    }
                }
                self.env = prev;
                Ok(None)
            }
            Stmt::If(cond, then_branch, else_branch) => {
                let c = self.evaluate(cond)?;
                if c.is_truthy() {
                    return self.execute(then_branch);
                } else if let Some(eb) = else_branch {
                    return self.execute(eb);
                } else { 
                    Ok(None) 
                }
            }
            Stmt::While(cond, body) => {
                while self.evaluate(cond)?.is_truthy() {
                    if let Some(v) = self.execute(body)? { return Ok(Some(v)); }
                }
                Ok(None)
            }
            Stmt::FuncDecl(name, params, body) => {
                let func = Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.env.clone(),
                };
                self.env.borrow_mut().define(name.clone(), Value::Function(Rc::new(func)));
                Ok(None)
            }
            Stmt::Return(expr_opt) => {
                if let Some(e) = expr_opt { let v = self.evaluate(e)?; return Ok(Some(v)); }
                else { return Ok(Some(Value::Nil)); }
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Numero(n) => Ok(Value::Number(*n)),
            Expr::Texto(s) => Ok(Value::Text(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Nulo => Ok(Value::Nil),
            Expr::Var(name) => self.env.borrow().get(name),
            Expr::Unario(op, right) => {
                let r = self.evaluate(right)?;
                match op {
                    UnarioOp::Neg => {
                        if let Value::Number(n) = r { Ok(Value::Number(-n)) } else { Err("Operador unário '-' espera número".into()) }
                    }
                    UnarioOp::Nao => Ok(Value::Bool(!r.is_truthy()))
                }
            }
            Expr::Binario(left, op, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                match op {
                    BinOp::Add => {
                        match (l, r) {
                            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                            (Value::Text(a), Value::Text(b)) => Ok(Value::Text(a + &b)),
                            (Value::Text(a), b2) => Ok(Value::Text(a + &b2.to_string_repr())),
                            (a2, Value::Text(b)) => Ok(Value::Text(a2.to_string_repr() + &b)),
                            _ => Err("Operador '+' inválido para operandos".into())
                        }
                    }
                    BinOp::Sub => {
                        if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Number(a - b)) } else { Err("'-' espera números".into()) }
                    }
                    BinOp::Mul => {
                        if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Number(a * b)) } else { Err("'*' espera números".into()) }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Number(a / b)) } else { Err("'/' espera números".into()) }
                    }
                    BinOp::Eq => Ok(Value::Bool(self.is_equal(&l, &r))),
                    BinOp::Neq => Ok(Value::Bool(!self.is_equal(&l, &r))),
                    BinOp::Lt => if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Bool(a < b)) } else { Err("'<' espera números".into()) },
                    BinOp::Gt => if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Bool(a > b)) } else { Err("'>' espera números".into()) },
                    BinOp::Le => if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Bool(a <= b)) } else { Err("'<=' espera números".into()) },
                    BinOp::Ge => if let (Value::Number(a), Value::Number(b)) = (l, r) { Ok(Value::Bool(a >= b)) } else { Err("'>=' espera números".into()) },
                    BinOp::And => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
                    BinOp::Or => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
                }
            }
            Expr::Chamada(callee_expr, args_exprs) => {
                let callee = self.evaluate(callee_expr)?;
                let mut args_vals = Vec::new();
                for a in args_exprs { args_vals.push(self.evaluate(a)?); }
                match callee {
                    Value::Function(f_rc) => {
                        let f = f_rc.clone();
                        if f.name == "imprimir" && f.params.len() <= 1 {
                            if args_vals.len() >= 1 {
                                for (i, v) in args_vals.iter().enumerate() {
                                    if i>0 { print!(" "); }
                                    print!("{}", v.to_string_repr());
                                }
                                println!();
                                return Ok(Value::Nil);
                            } else {
                                println!();
                                return Ok(Value::Nil);
                            }
                        }
                        f.call(self, args_vals)
                    }
                    _ => Err("Tentativa de chamar algo que não é função".into())
                }
            }
        }
    }

    fn is_equal(&self, a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Number(x), Value::Number(y)) => (x - y).abs() < 1e-10,
            (Value::Text(x), Value::Text(y)) => x == y,
            (Value::Bool(x), Value::Bool(y)) => x == y,
            (Value::Nil, Value::Nil) => true,
            _ => false
        }
    }
}