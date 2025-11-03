use std::collections::HashMap;
use crate::tokens::TokenType;

pub fn palavras_chave() -> HashMap<&'static str, TokenType> {
    use TokenType::*;
    HashMap::from([
        ("variavel", Variavel),
        ("se", Se),
        ("senao", Senao),
        ("enquanto", Enquanto),
        ("para", Para),
        ("funcao", Funcao),
        ("retornar", Retornar),
        ("verdadeiro", Verdadeiro),
        ("falso", Falso),
        ("imprimir", Imprimir),
        ("e", E),
        ("ou", Ou),
        ("nao", Nao),
        ("nulo", Nulo),
    ])
}