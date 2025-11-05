use crate::tokens::{Token, TokenType};
use crate::keywords::palavras_chave;

pub struct Lexer {
    src: Vec<char>,
    pos: usize,
    linha: usize,
    coluna: usize,
}

impl Lexer {
    pub fn new(s: &str) -> Self {
        Self {
            src: s.chars().collect(),
            pos: 0,
            linha: 1,
            coluna: 1,
        }
    }

    fn espiar(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn avancar(&mut self) -> Option<char> {
        let c = self.espiar();
        if c.is_some() {
            self.pos += 1;
            self.coluna += 1;
        }
        c
    }

    fn combinar(&mut self, esperado: char) -> bool {
        if let Some(c) = self.espiar() {
            if c == esperado {
                self.pos += 1;
                self.coluna += 1;
                return true;
            }
        }
        false
    }

    fn pular_espacos(&mut self) {
        loop {
            match self.espiar() {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.avancar();
                }
                Some('\n') => {
                    self.linha += 1;
                    self.coluna = 1;
                    self.avancar();
                }
                Some('/') => {
                    if let Some(next) = self.src.get(self.pos + 1) {
                        if *next == '/' {
                            while let Some(ch) = self.espiar() {
                                if ch == '\n' {
                                    break;
                                }
                                self.avancar();
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        use TokenType::*;
        let mut tokens: Vec<Token> = Vec::new();

        while self.espiar().is_some() {
            self.pular_espacos();

            if self.espiar().is_none() {
                break;
            }

            let ch = self.espiar().unwrap();
            let linha = self.linha;

            let token = match ch {
                '+' => {
                    self.avancar();
                    Token::new(Mais, "+".into(), linha)
                }
                '-' => {
                    self.avancar();
                    Token::new(Menos, "-".into(), linha)
                }
                '*' => {
                    self.avancar();
                    Token::new(Multiplica, "*".into(), linha)
                }
                '/' => {
                    self.avancar();
                    Token::new(Divide, "/".into(), linha)
                }
                '(' => {
                    self.avancar();
                    Token::new(AbrePar, "(".into(), linha)
                }
                ')' => {
                    self.avancar();
                    Token::new(FechaPar, ")".into(), linha)
                }
                '{' => {
                    self.avancar();
                    Token::new(AbreChave, "{".into(), linha)
                }
                '}' => {
                    self.avancar();
                    Token::new(FechaChave, "}".into(), linha)
                }
                '[' => {
                    self.avancar();
                    Token::new(AbreColchete, "[".into(), linha)
                }
                ']' => {
                    self.avancar();
                    Token::new(FechaColchete, "]".into(), linha)
                }
                ';' => {
                    self.avancar();
                    Token::new(PontoVirgula, ";".into(), linha)
                }
                ',' => {
                    self.avancar();
                    Token::new(Virgula, ",".into(), linha)
                }
                '=' => {
                    self.avancar();
                    if self.combinar('=') {
                        Token::new(IgualIgual, "==".into(), linha)
                    } else {
                        Token::new(Igual, "=".into(), linha)
                    }
                }
                '!' => {
                    self.avancar();
                    if self.combinar('=') {
                        Token::new(Diferente, "!=".into(), linha)
                    } else {
                        Token::new(Nao, "!".into(), linha)
                    }
                }
                '<' => {
                    self.avancar();
                    if self.combinar('=') {
                        Token::new(MenorIgual, "<=".into(), linha)
                    } else {
                        Token::new(Menor, "<".into(), linha)
                    }
                }
                '>' => {
                    self.avancar();
                    if self.combinar('=') {
                        Token::new(MaiorIgual, ">=".into(), linha)
                    } else {
                        Token::new(Maior, ">".into(), linha)
                    }
                }
                '"' => {
                    self.avancar();
                    let mut s = String::new();
                    while let Some(ch2) = self.espiar() {
                        if ch2 == '"' {
                            break;
                        }
                        if ch2 == '\n' {
                            self.linha += 1;
                            self.coluna = 1;
                        }
                        s.push(ch2);
                        self.avancar();
                    }
                    if self.espiar() == Some('"') {
                        self.avancar();
                    }
                    let lexema = format!("\"{}\"", s);
                    Token::new(Texto(s), lexema, linha)
                }
                d if d.is_ascii_digit() => {
                    let mut num = String::new();
                    while let Some(nd) = self.espiar() {
                        if nd.is_ascii_digit() || nd == '.' {
                            num.push(nd);
                            self.avancar();
                        } else {
                            break;
                        }
                    }
                    let valor = num.parse::<f64>().unwrap_or(0.0);
                    Token::new(Numero(valor), num, linha)
                }
                a if a.is_alphabetic() || a == '_' => {
                    let mut id = String::new();
                    while let Some(ch2) = self.espiar() {
                        if ch2.is_alphanumeric() || ch2 == '_' {
                            id.push(ch2);
                            self.avancar();
                        } else {
                            break;
                        }
                    }

                    let mapa = palavras_chave();
                    if let Some(tt) = mapa.get(id.as_str()) {
                        Token::new(tt.clone(), id.clone(), linha)
                    } else {
                        Token::new(Ident(id.clone()), id, linha)
                    }
                }
                _ => {
                    self.avancar();
                    continue;
                }
            };

            tokens.push(token);
        }

        tokens.push(Token::new(Fim, "".into(), self.linha));
        tokens
    }
}