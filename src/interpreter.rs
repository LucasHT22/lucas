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
    Array(Rc<RefCell<Vec<Value>>>),
    Function(Rc<Function>),
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::Text(s) => !s.is_empty(),
            Value::Array(arr) => !arr.borrow().is_empty(),
            Value::Function(_) => true,
        }
    }

    pub fn to_string_repr(&self) -> String {
        match self {
            Value::Number(n) => {
                if (n.fract() - 0.0).abs() < 1e-10 { 
                    format!("{}", *n as i64) 
                } else { 
                    format!("{}", n) 
                }
            }
            Value::Text(s) => s.clone(),
            Value::Bool(b) => format!("{}", b),
            Value::Nil => "nulo".into(),
            Value::Array(arr) => {
                let items: Vec<String> = arr.borrow()
                    .iter()
                    .map(|v| v.to_string_repr())
                    .collect();
                format!("[{}]", items.join(", "))
            }
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
        
        let mut ret_val = Value::Nil;
        for stmt in &self.body {
            match interpreter.execute(stmt)? {
                Some(v) => { ret_val = v; break; }
                None => {}
            }
        }
        interpreter.env = prev_env;
        Ok(ret_val)
    }
}

pub struct Interpreter {
    pub globals: EnvRef,
    pub env: EnvRef,
    pub fonte: String,
}

impl Interpreter {
    pub fn new() -> Self { 
        let g = Rc::new(RefCell::new(Environment::new()));
        let it = Self { 
            globals: g.clone(), 
            env: g.clone(),
            fonte: String::new(),
        };
        
        it.globals.borrow_mut().define("imprimir".into(), Value::Function(Rc::new(Function {
            name: "imprimir".into(),
            params: vec![],
            body: vec![],
            closure: it.globals.clone(),
        })));
        
        it.globals.borrow_mut().define("comprimento".into(), Value::Function(Rc::new(Function {
            name: "comprimento".into(),
            params: vec!["x".into()],
            body: vec![],
            closure: it.globals.clone(),
        })));
        
        it.globals.borrow_mut().define("maiuscula".into(), Value::Function(Rc::new(Function {
            name: "maiuscula".into(),
            params: vec!["x".into()],
            body: vec![],
            closure: it.globals.clone(),
        })));
        
        it.globals.borrow_mut().define("minuscula".into(), Value::Function(Rc::new(Function {
            name: "minuscula".into(),
            params: vec!["x".into()],
            body: vec![],
            closure: it.globals.clone(),
        })));
    }

    pub fn run(&mut self, source: &str) {
        self.fonte = source.to_string();
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

    pub fn execute(&mut self, stmt: &Stmt) -> Result<FlowControl, String> {
        match stmt {
            Stmt::ExprStmt(e) => { 
                self.evaluate(e)?; 
                Ok(FlowControl::None) 
            }
            Stmt::Imprimir(e) => {
                let v = self.evaluate(e)?;
                println!("{}", v.to_string_repr());
                Ok(FlowControl::None)
            }
            Stmt::VarDecl(name, init) => {
                let v = self.evaluate(init)?;
                self.env.borrow_mut().define(name.clone(), v);
                Ok(FlowControl::None)
            }
            Stmt::Bloco(stmts) => {
                let new_env = Rc::new(RefCell::new(Environment::with_enclosing(self.env.clone())));
                let prev = self.env.clone();
                self.env = new_env;
                
                for s in stmts {
                    match self.execute(s)? {
                        FlowControl::None => {}
                        flow => {
                            self.env = prev;
                            return Ok(flow);
                        }
                    }
                }
                self.env = prev;
                Ok(FlowControl::None)
            }
            Stmt::If(cond, then_branch, else_branch) => {
                let c = self.evaluate(cond)?;
                if c.is_truthy() {
                    self.execute(then_branch)
                } else if let Some(eb) = else_branch {
                    self.execute(eb)
                } else { 
                    Ok(FlowControl::None)
                }
            }
            Stmt::While(cond, body) => {
                while self.evaluate(cond)?.is_truthy() {
                    match self.execute(body)? {
                        FlowControl::Break => break,
                        FlowControl::Continue => continue,
                        FlowControl::Return(v) => return Ok(FlowControl::Return(v)),
                        FlowControl::None => {}
                    }
                }
                Ok(FlowControl::None)
            }
            Stmt::For(init, cond, incr, body) => {
                let new_env = Rc::new(RefCell::new(Environment::with_enclosing(self.env.clone())));
                let prev = self.env.clone();
                self.env = new_env;
                
                if let Some(init_stmt) = init {
                    self.execute(init_stmt)?;
                }
                
                loop {
                    if let Some(cond_expr) = cond {
                        if !self.evaluate(cond_expr)?.is_truthy() {
                            break;
                        }
                    }
                    
                    match self.execute(body)? {
                        FlowControl::Break => break,
                        FlowControl::Continue => {},
                        FlowControl::Return(v) => {
                            self.env = prev;
                            return Ok(FlowControl::Return(v));
                        }
                        FlowControl::None => {}
                    }
                    
                    if let Some(incr_expr) = incr {
                        self.evaluate(incr_expr)?;
                    }
                }
                
                self.env = prev;
                Ok(FlowControl::None)
            }
            Stmt::FuncDecl(name, params, body) => {
                let func = Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.env.clone(),
                };
                self.env.borrow_mut().define(name.clone(), Value::Function(Rc::new(func)));
                Ok(FlowControl::None)
            }
            Stmt::Return(expr_opt) => {
                let v = if let Some(e) = expr_opt { 
                    self.evaluate(e)? 
                } else { 
                    Value::Nil 
                };
                Ok(FlowControl::Return(v))
            }
            Stmt::Break => Ok(FlowControl::Break),
            Stmt::Continue => Ok(FlowControl::Continue),
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Numero(n) => Ok(Value::Number(*n)),
            Expr::Texto(s) => Ok(Value::Text(s.clone())),
            Expr::Bool(b) => Ok(Value::Bool(*b)),
            Expr::Nulo => Ok(Value::Nil),
            Expr::Array(elementos) => {
                let mut arr = Vec::new();
                for elem in elementos {
                    arr.push(self.evaluate(elem)?);
                }
                Ok(Value::Array(Rc::new(RefCell::new(arr))))
            }
            Expr::Index(arr_expr, idx_expr) => {
                let arr_val = self.evaluate(arr_expr)?;
                let idx_val = self.evaluate(idx_expr)?;
                
                match (arr_val, idx_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let i = idx as usize;
                        let borrowed = arr.borrow();
                        if i < borrowed.len() {
                            Ok(borrowed[i].clone())
                        } else {
                            Err(format!("Índice {} fora dos limites (tamanho: {})", i, borrowed.len()))
                        }
                    }
                    (Value::Text(s), Value::Number(idx)) => {
                        let i = idx as usize;
                        if i < s.len() {
                            Ok(Value::Text(s.chars().nth(i).unwrap().to_string()))
                        } else {
                            Err(format!("Índice {} fora dos limites (tamanho: {})", i, s.len()))
                        }
                    }
                    _ => Err("Indexação requer array/string e índice numérico".into())
                }
            }
            Expr::Atribuir(name, value_expr) => {
                let value = self.evaluate(value_expr)?;
                self.env.borrow_mut().assign(name, value.clone())?;
                Ok(value)
            }
            Expr::AtribuirIndex(arr_expr, idx_expr, value_expr) => {
                let arr_val = self.evaluate(arr_expr)?;
                let idx_val = self.evaluate(idx_expr)?;
                let new_val = self.evaluate(value_expr)?;
                
                match (arr_val, idx_val) {
                    (Value::Array(arr), Value::Number(idx)) => {
                        let i = idx as usize;
                        let mut borrowed = arr.borrow_mut();
                        if i < borrowed.len() {
                            borrowed[i] = new_val.clone();
                            Ok(new_val)
                        } else {
                            Err(format!("Índice {} fora dos limites", i))
                        }
                    }
                    _ => Err("Atribuição de índice requer array e índice numérico".into())
                }
            }
            Expr::Var(name) => self.env.borrow().get(name),
            Expr::Unario(op, right) => {
                let r = self.evaluate(right)?;
                match op {
                    UnarioOp::Neg => {
                        if let Value::Number(n) = r { 
                            Ok(Value::Number(-n)) 
                        } else { 
                            Err("Operador unário '-' espera número".into()) 
                        }
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
                        if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                            Ok(Value::Number(a - b)) 
                        } else { 
                            Err("'-' espera números".into()) 
                        }
                    }
                    BinOp::Mul => {
                        if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                            Ok(Value::Number(a * b)) 
                        } else { 
                            Err("'*' espera números".into()) 
                        }
                    }
                    BinOp::Div => {
                        if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                            if b == 0.0 {
                                Err("Divisão por zero".into())
                            } else {
                                Ok(Value::Number(a / b))
                            }
                        } else { 
                            Err("'/' espera números".into()) 
                        }
                    }
                    BinOp::Eq => Ok(Value::Bool(self.is_equal(&l, &r))),
                    BinOp::Neq => Ok(Value::Bool(!self.is_equal(&l, &r))),
                    BinOp::Lt => if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                        Ok(Value::Bool(a < b)) 
                    } else { 
                        Err("'<' espera números".into()) 
                    },
                    BinOp::Gt => if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                        Ok(Value::Bool(a > b)) 
                    } else { 
                        Err("'>' espera números".into()) 
                    },
                    BinOp::Le => if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                        Ok(Value::Bool(a <= b)) 
                    } else { 
                        Err("'<=' espera números".into()) 
                    },
                    BinOp::Ge => if let (Value::Number(a), Value::Number(b)) = (l, r) { 
                        Ok(Value::Bool(a >= b)) 
                    } else { 
                        Err("'>=' espera números".into()) 
                    },
                    BinOp::And => Ok(Value::Bool(l.is_truthy() && r.is_truthy())),
                    BinOp::Or => Ok(Value::Bool(l.is_truthy() || r.is_truthy())),
                }
            }
            Expr::Chamada(callee_expr, args_exprs) => {
                let callee = self.evaluate(callee_expr)?;
                let mut args_vals = Vec::new();
                for a in args_exprs { 
                    args_vals.push(self.evaluate(a)?); 
                }
                
                match callee {
                    Value::Function(f_rc) => {
                        let f = f_rc.clone();
                        
                        if f.name == "imprimir" {
                            for (i, v) in args_vals.iter().enumerate() {
                                if i > 0 { print!(" "); }
                                print!("{}", v.to_string_repr());
                            }
                            println!();
                            return Ok(Value::Nil);
                        }
                        
                        if f.name == "comprimento" {
                            if args_vals.len() != 1 {
                                return Err("comprimento() espera 1 argumento".into());
                            }
                            match &args_vals[0] {
                                Value::Text(s) => return Ok(Value::Number(s.len() as f64)),
                                Value::Array(arr) => return Ok(Value::Number(arr.borrow().len() as f64)),
                                _ => return Err("comprimento() espera texto ou array".into())
                            }
                        }
                        
                        if f.name == "maiuscula" {
                            if args_vals.len() != 1 {
                                return Err("maiuscula() espera 1 argumento".into());
                            }
                            if let Value::Text(s) = &args_vals[0] {
                                return Ok(Value::Text(s.to_uppercase()));
                            }
                            return Err("maiuscula() espera texto".into());
                        }
                        
                        if f.name == "minuscula" {
                            if args_vals.len() != 1 {
                                return Err("minuscula() espera 1 argumento".into());
                            }
                            if let Value::Text(s) = &args_vals[0] {
                                return Ok(Value::Text(s.to_lowercase()));
                            }
                            return Err("minuscula() espera texto".into());
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