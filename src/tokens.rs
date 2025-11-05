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
    Break,
    Continue,
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
    AbreColchete,
    FechaColchete,
    Virgula,
    PontoVirgula,
    Fim,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tipo: TokenType,
    pub lexema: String,
    pub linha: usize,
    pub coluna: usize,
}

impl Token {
    pub fn new(tipo: TokenType, lexema: String, linha: usize) -> Self {
        Self { tipo, lexema, linha, coluna: 0 }
    }
    
    pub fn with_column(tipo: TokenType, lexema: String, linha: usize, coluna: usize) -> Self {
        Self { tipo, lexema, linha, coluna }
    }
}