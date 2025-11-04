#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Numero(f64),
    Texto(String),
    Ident(String),
    Variavel,
    Funcao,
    Se,
    Senao,
    Enquanto,
    Para,
    Retornar,
    Imprimir,
    Verdadeiro,
    Falso,
    Nulo,
    E,
    Ou,
    Nao,
    Mais,
    Menos,
    Multiplica,
    Divide,
    Igual,
    IgualIgual,
    Diferente,
    Menor,
    MenorIgual,
    Maior,
    MaiorIgual,
    AbrePar,
    FechaPar,
    AbreChave,
    FechaChave,
    Virgula,
    PontoVirgula,
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