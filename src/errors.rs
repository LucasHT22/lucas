pub fn erro(msg: &str, linha: usize) {
    eprintln!("Erro na linha {}: {}", linha, msg);
}