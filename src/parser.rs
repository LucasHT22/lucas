use crate::tokens::{Token, TokenType};
use crate::ast::{Expr, Stmt, UnarioOp, BinOp};
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens: tokens.into_iter().peekable() }
    }

    fn peek(&mut self) -> Option<Token> { self.tokens.peek().cloned() }
    fn advance(&mut self) -> Option<Token> { self.tokens.next() }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = vec![];
        while let Some(t) = self.peek() {
            if t.tipo == TokenType::Fim { break; }
            if let Some(s) = self.declaration() { stmts.push(s); } else { break; }
        }
        stmts
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if let Some(t) = self.peek() {
            match t.tipo {
                TokenType::Variavel => { self.advance(); return self.var_declaration(); }
                TokenType::Funcao => { self.advance(); return self.func_declaration(); }
                _ => {}
            }
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = if let Some(tok) = self.advance() {
            if let TokenType::Ident(_) = tok.tipo { tok.lexema } else { return None; }
        } else { return None; };

        if let Some(tok) = self.advance() {
            if tok.tipo != TokenType::Igual { return None; }
        } else { return None; }

        let init = self.expression();

        if let Some(peek) = self.peek() {
            if peek.tipo == TokenType::PontoVirgula { self.advance(); }
        }
        Some(Stmt::VarDecl(name, init))
    }

    fn func_declaration(&mut self) -> Option<Stmt> {
        let name = if let Some(tok) = self.advance() {
            if let TokenType::Ident(_) = tok.tipo { tok.lexema } else { return None; }
        } else { return None; };

        let tok = self.advance(); if tok.is_none() { return None; }
        if tok.unwrap().tipo != TokenType::AbrePar { return None; }

        let mut params = Vec::new();
        if let Some(p) = self.peek() {
            if p.tipo != TokenType::FechaPar {
                loop {
                    if let Some(tok) = self.advance() {
                        if let TokenType::Ident(_) = tok.tipo { params.push(tok.lexema); } else { return None }
                    } else { return None; }
                    if let Some(peek) = self.peek() {
                        if peek.tipo == TokenType::Virgula { self.advance(); continue; } else { break; }
                    }
                }
            }
        }
        if let Some(tok) = self.advance() {
            if tok.tipo != TokenType::FechaPar { return None; }
        } else { return None; }

        let body = if let Some(Stmt::Bloco(stmts)) = self.block() { stmts } else { Vec::new() };
        Some(Stmt::FuncDecl(name, params, body))
    }

    fn statement(&mut self) -> Option<Stmt> {
        if let Some(tok) = self.peek() {
            match tok.tipo {
                TokenType::Imprimir => { 
                    self.advance(); 
                    let expr = self.expression(); 
                    if let Some(s) = self.peek() { 
                        if s.tipo == TokenType::PontoVirgula { self.advance(); } 
                    } 
                    return Some(Stmt::Imprimir(expr)); 
                }
                TokenType::AbreChave => return self.block(),
                TokenType::Se => {
                    self.advance();
                    if let Some(t) = self.advance() { if t.tipo != TokenType::AbrePar { return None; } } else { return None; }
                    let cond = self.expression();
                    if let Some(t) = self.advance() { if t.tipo != TokenType::FechaPar { return None; } } else { return None; }
                    let then_branch = self.statement().map(Box::new)?;
                    let mut else_branch = None;
                    if let Some(peek) = self.peek() {
                        if peek.tipo == TokenType::Senao { self.advance(); if let Some(s) = self.statement() { else_branch = Some(Box::new(s)); } }
                    }
                    return Some(Stmt::If(cond, then_branch, else_branch));
                }
                TokenType::Enquanto => {
                    self.advance();
                    if let Some(t) = self.advance() { if t.tipo != TokenType::AbrePar { return None; } } else { return None; }
                    let cond = self.expression();
                    if let Some(t) = self.advance() { if t.tipo != TokenType::FechaPar { return None; } } else { return None; }
                    let body = self.statement().map(Box::new)?;
                    return Some(Stmt::While(cond, body));
                }
                TokenType::Para => {
                    self.advance();
                    if let Some(t) = self.advance() { if t.tipo != TokenType::AbrePar { return None; } } else { return None; }

                    let init = if let Some(peek) = self.peek() {
                        if peek.tipo == TokenType::PontoVirgula { self.advance(); None } else { let s = self.declaration(); if let Some(p) = self.peek() { if p.tipo==TokenType::PontoVirgula { self.advance(); } } s }
                    } else { None };

                    let cond = if let Some(peek) = self.peek() {
                        if peek.tipo == TokenType::PontoVirgula { None } else { Some(self.expression()) }
                    } else { None };

                    let incr = if let Some(peek) = self.peek() {
                        if peek.tipo == TokenType::FechaPar { None } else { Some(self.expression()) }
                    } else { None };

                    if let Some(t) = self.advance() { if t.tipo != TokenType::FechaPar {return None; } } else { return None; }
                    let body = self.statement().map(|s| s).unwrap_or(Stmt::Bloco(vec![]));

                    let mut stmts = Vec::new();
                    if let Some(init_stmt) = init { stmts.push(init_stmt); }
                    let while_cond = cond.unwrap_or(Expr::Bool(true));

                    let mut inner = Vec::new();
                    inner.push(body);
                    if let Some(inc_expr) = incr {
                        inner.push(Stmt::ExprStmt(inc_expr));
                    }

                    let while_stmt = Stmt::While(while_cond, Box::new(Stmt::Bloco(inner)));
                    stmts.push(while_stmt);
                    return Some(Stmt::Bloco(stmts));
                }
                TokenType::Retornar => {
                    self.advance();

                    let expr = if let Some(p) = self.peek() { if p.tipo == TokenType::PontoVirgula { None } else { Some(self.expression()) } } else { None };
                    if let Some(p) = self.peek() { if p.tipo == TokenType::PontoVirgula { self.advance(); } }
                    return Some(Stmt::Return(expr));
                }
                _ => {}
            }
        }
        let expr = self.expression();
        if let Some(p) = self.peek() { if p.tipo == TokenType::PontoVirgula { self.advance(); } }
        Some(Stmt::ExprStmt(expr))
    }

    fn block(&mut self) -> Option<Stmt> {
        if let Some(t) = self.advance() {
            if t.tipo != TokenType::AbreChave { return None; }
        } else { return None }
        let mut stmts = Vec::new();
        while let Some(peek) = self.peek() {
            if peek.tipo == TokenType::FechaChave { self.advance(); return Some(Stmt::Bloco(stmts)); }
            if peek.tipo == TokenType::Fim { return None; }
            if let Some(s) = self.declaration() { stmts.push(s); } else { break; }
        }
        None
    }

    fn expression(&mut self) -> Expr {
        self.or()
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();
        while let Some(peek) = self.peek() {
            if peek.tipo == TokenType::Ou { self.advance(); let right = self.and(); expr = Expr::Binario(Box::new(expr), BinOp::Or, Box::new(right)); } else { break; }
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();
        while let Some(peek) = self.peek() {
            if peek.tipo == TokenType::E { self.advance(); let right = self.equality(); expr = Expr::Binario(Box::new(expr), BinOp::And, Box::new(right)); } else { break; }
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        loop {
            if let Some(peek) = self.peek() {
                match peek.tipo {
                    TokenType::IgualIgual => { self.advance(); let right = self.comparison(); expr = Expr::Binario(Box::new(expr), BinOp::Eq, Box::new(right)); }
                    TokenType::Diferente => { self.advance(); let right = self.comparison(); expr = Expr::Binario(Box::new(expr), BinOp::Neq, Box::new(right)); }
                    _ => break
                }
            } else { break; }
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        loop {
            if let Some(peek) = self.peek() {
                match peek.tipo {
                    TokenType::Maior => { self.advance(); let r = self.term(); expr = Expr::Binario(Box::new(expr), BinOp::Gt, Box::new(r)); }
                    TokenType::MaiorIgual => { self.advance(); let r = self.term(); expr = Expr::Binario(Box::new(expr), BinOp::Ge, Box::new(r)); }
                    TokenType::Menor => { self.advance(); let r = self.term(); expr = Expr::Binario(Box::new(expr), BinOp::Lt, Box::new(r)); }
                    TokenType::MenorIgual => { self.advance(); let r = self.term(); expr = Expr::Binario(Box::new(expr), BinOp::Le, Box::new(r)); }
                    _ => break
                }
            } else { break; }
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        loop {
            if let Some(peek) = self.peek() {
                match peek.tipo {
                    TokenType::Mais => { self.advance(); let r = self.factor(); expr = Expr::Binario(Box::new(expr), BinOp::Add, Box::new(r)); }
                    TokenType::Menos => { self.advance(); let r = self.factor(); expr = Expr::Binario(Box::new(expr), BinOp::Sub, Box::new(r)); }
                    _ => break
                }
            } else { break; }
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        loop {
            if let Some(peek) = self.peek() {
                match peek.tipo {
                    TokenType::Multiplica => { self.advance(); let r = self.unary(); expr = Expr::Binario(Box::new(expr), BinOp::Mul, Box::new(r)); }
                    TokenType::Divide => { self.advance(); let r = self.unary(); expr = Expr::Binario(Box::new(expr), BinOp::Div, Box::new(r)); }
                    _ => break
                }
            } else { break; }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if let Some(peek) = self.peek() {
            match peek.tipo {
                TokenType::Menos => { self.advance(); let right = self.unary(); return Expr::Unario(UnarioOp::Neg, Box::new(right)); }
                TokenType::Nao => { self.advance(); let right = self.unary(); return Expr::Unario(UnarioOp::Nao, Box::new(right)); }
                _ => {}
            }
        }
        self.call()
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if let Some(peek) = self.peek() {
                if peek.tipo == TokenType::AbrePar {
                    self.advance();
                    let mut args = Vec::new();
                    if let Some(p2) = self.peek() {
                        if p2.tipo != TokenType::FechaPar {
                            loop {
                                let a = self.expression();
                                args.push(a);
                                if let Some(nxt) = self.peek() {
                                    if nxt.tipo == TokenType::Virgula { self.advance(); continue; }
                                }
                                break;
                            }
                        }
                    }
                    if let Some(t) = self.advance() { if t.tipo != TokenType::FechaPar { } }
                    expr = Expr::Chamada(Box::new(expr), args);
                    continue;
                }
            }
            break;
        }
        expr
    }

    fn primary(&mut self) -> Expr {
        if let Some(tok) = self.advance() {
            match tok.tipo {
                TokenType::Numero(n) => Expr::Numero(n),
                TokenType::Texto(s) => Expr::Texto(s),
                TokenType::Verdadeiro => Expr::Bool(true),
                TokenType::Falso => Expr::Bool(false),
                TokenType::Nulo => Expr::Nulo,
                TokenType::Ident(name) => Expr::Var(name),
                TokenType::AbrePar => {
                    let e = self.expression();
                    if let Some(t) = self.advance() { if t.tipo != TokenType::FechaPar { } }
                    e
                }
                _ => Expr::Nulo,
            }
        } else {
            Expr::Nulo
        }
    }
}