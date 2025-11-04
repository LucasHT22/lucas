#[derive(Debug, Clone, PartialEq)]

pub enum TokenType {
    Imprimir,
    Variavel,
    Funcao,
    Retornar,
    Se,
    Senao,
    Enquanto,
    Para,
    Verdadeiro,
    Falso,
    E,
    Ou,
    Nao,
    Nulo,
    Numero(f64),
    Texto(String),
    Ident(String),
    Igual,
    Mais,
    Menos,
    Multiplica,
    Divide,
    Maior,
    Menor,
    MaiorIgual,
    MenorIgual,
    IgualIgual,
    Diferente,
    AbrePar,
    FechaPar,
    AbreChave,
    FechaChave,
    PontoVirgula,
    Virgula,
    Fim,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tipo: TokenType,
    pub lexema: String,
    pub linha: usize,
}

impl Token {
    pub fn new(tipo: TokenType, lexema: String, linha: usize) -> Self {
        Self { tipo, lexema, linha }
    }
}