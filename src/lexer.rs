use crate::tokens::{Token, TokenType};
use crate::keywords::palavras_chave;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    linha; usize,
}

impl Lexer {
    pub fn new(s: &str) -> Self {
        Self { src: s.chars().collect(), pos: 0, linha: 1 }
    }

    fn espiar(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn avancar(&mut self) -> Option<char> {
        let c = self.espiar();
        if c.is_some() { self.pos += 1; }
        c
    }

    fn combinar(&mut self, expected: char) -> bool {
        if let Some(c) = self.espiar() {
            if c == expected {
                self.pos += 1;
                return true;
            }
        }
        false
    }

    fn pular_espacos(&mut self) {
        loop {
            match self.espiar() {
                Some(' ') | Some('\r') | Some('\t') => { self.avancar(); }
                Some('\n') => { self.linha += 1; self.avancar(); }
                Some('/') => {
                    if let Some(next) = self.src.get(self.pos+1) {
                        if *next == '/' {
                            while let Some(ch) = self.espiar() {
                                if ch == '\n' { break; }
                                self.avancar();
                            }
                        } else { break; }
                    } else { break; }
                }
                _ => break
            }
        }
    }

    pub fn tokenize(&mut self) => Vec<Token> {
        use TokenType::*;
        let mut tokens: Vec<Token> = vec::new();

        while let Some(c) = self.avancar() {
            self.pular_espacos();
            let linha = self.linha;
            if let Some(ch) = self.espiar() {
                match ch {
                    '+' => { self.avancar(); tokens.push(Token { tipo: Mais, lexema: "+".into(), linha }); }
                    '-' => { self.avancar(); tokens.push(Token { tipo: Menos, lexema: "-".into(), linha }); }
                    '*' => { self.avancar(); tokens.push(Token { tipo: Multiplica, lexema: "*".into(), linha }); }
                    '/' => { self.avancar(); tokens.push(Token { tipo: Divide, lexema: "/".into(), linha }); }
                    '(' => { self.avancar(); tokens.push(Token { tipo: AbrePar, lexema: "(".into(), linha }); }
                    ')' => { self.avancar(); tokens.push(Token { tipo: FechaPar, lexema: ")".into(), linha }); }
                    '{' => { self.avancar(); tokens.push(Token { tipo: AbreChave, lexema: "{".into(), linha }); }
                    '}' => { self.avancar(); tokens.push(Token { tipo: FechaChave, lexema: "}".into(), linha }); }
                    ';' => { self.avancar(); tokens.push(Token { tipo: PontoVirgula, lexema: ";".into(), linha }); }
                    ',' => { self.avancar(); tokens.push(Token { tipo: Virgula, lexema: ",".into(), linha }); }
                    '=' => { 
                        self.avancar(); 
                        if self.combinar('=') { tokens.push(Token { tipo: IgualIgual, lexema: "==".into(), linha }); }
                        else { tokens.push(Token { tipo: Igual, lexema: "=".into(), linha }); }
                    }
                    '!' => {
                        self.avancar();
                        if self.combinar('=') { tokens.push(Token { tipo: Diferente, lexema: "!=".into(), linha }); }
                        else { }
                    }
                    '<' => {
                        self.avancar();
                        if self.combinar('=') { tokens.push(Token { tipo: MenorIgual, lexema: "<=".into(), linha }); }
                        else { tokens.push(Token { tipo: Menor, lexema: "<".into(), linha }); }
                    }
                    '>' => {
                        self.avancar();
                        if self.combinar('=') { tokens.push(Token { tipo: MaiorIgual, lexema: ">=0".into(), linha }); }
                        else { tokens.push(Token { tipo: Maior, lexema: ">".into(), linha }); }
                    }
                    '"' => {
                        self.avancar();
                        let mut s = String::new();
                        while let Some(ch2) = self.espiar() {
                            if ch2 == '"' { break; }
                            if ch2 == '\n' { self.linha += 1; }
                            s.push(ch2);
                            self.avancar();
                        }
                        if self.espiar() == Some('"') { self.avancar(); }
                        tokens.push(Token { tipo: Texto(s.clone()), lexema: s, linha });
                    }
                    d if d.is_ascii_digit() => {
                        let mut num = String::new();
                        while let Some(nd) = self.espiar() {
                            if nd.is_ascii_digit() || nd == '.' { num.push(nd); self.avancar(); } else { break; }
                        }
                        let val = num.parse::<f64>().unwrap_or(0.0);
                        tokens.push(Token { tipo: Numero(val), lexema: num, linha });
                    }
                    a if a.is_alphabetic() || a == '_' => {
                        let mut id = String::new();
                        while let Some(ch2) = self.espiar() {
                            if ch2.is_alphanumeric() || ch2 == '_' { id.push(ch2); self.avancar(); } else { break; }
                        }
                        let map = palavras_chave();
                        if let Some(tt) = map.get(id.as_str()) {
                            tokens.push(Token { tipo: tt.clone(), lexema: id, linha });
                        } else {
                            tokens.push(Token { tipo: TokenType::Ident(id.clone()), lexema: id, linha });
                        }
                    }
                    _ => {
                        self.avancar();
                    }
                }
            }
        }

        tokens.push(Token { tipo: TokenType::Fim, lexema: "".into(), linha: self.linha });
        tokens
    }
}