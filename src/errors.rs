use colored::*;

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub linha: usize,
    pub coluna: usize,
    pub fonte: String,
}

#[derive(Debug)]
pub enum ErrorType {
    LexicoError,
    SintaticoError,
    RuntimeError
}

pub struct LucasError {
    pub tipo: ErrorType,
    pub mensagem: String,
    pub localizacao: Option<SourceLocation>,
    pub sugestao: Option<String>,
}

impl LucasError {
    pub fn novo(tipo: ErrorType, mensagem: String) -> Self {
        Self {
            tipo,
            mensagem,
            localizacao: None,
            sugestao: None,
        }
    }

    pub fn com_localizacao(mut self, loc: SourceLocation) -> Self {
        self.localizacao = Some(loc);
        self
    }

    pub fn com_sugestao(mut self, sugestao: String) -> Self {
        self.sugestao = Some(sugestao);
        self
    }

    pub fn exibir(&self) {
        let tipo_str = match self.tipo {
            ErrorType::LexicoError => "Erro Léxico".red().bold(),
            ErrorType::SintaticoError => "Erro Sintático".yellow().bold(),
            ErrorType::RuntimeError => "Erro de Execução".red().bold(),
        };

        println!("\n{} {}" "X".red(), tipo_str);

        if let Some(ref loc) = self.localizacao {
            self.exibir_com_contexto(loc);
        } else {
            println!("  {}", self.mensagem.white());
        }

        if let Some(ref sug) = self.sugestao {
            println!("\n{} {}", "💡".yellow(), sug.cyan());
        }

        println!();
    }

    fn exibir_com_contexto(&self, loc: &SourceLocation) {
        println!(" na linha {}, coluna {}:", loc.linha.to_string().cyan(), loc.coluna.to_string().cyan());
        println!();

        let linhas: Vec<&str> = loc.fonte.lines().collect();
        let linha_idx = loc.linha.saturating_sub(1);

        if linha_idx > 0 {
            println!(" {} | {}", format!("{:3}", linha_idx).dimmed(), linhas[linha_idx - 1].dimmed());
        }

        if linha_idx < linhas.len() {
            println!(" {} | {}", format!("{:3}", loc.linha).cyan().bold(), linhas[linha_idx]);
        
            let espacos = " ".repeat(loc.coluna + 6);
            println!(" {} {} {}", espacos, "^".red().bold(), self.mensagem.red());
        }

        if linha_idx + 1 < linhas.len() {
            println!(" {} | {}", format!("{:3}", linha_idx + 2).dimmed(), linhas[linha_idx + 1].dimmed());
        }
    }
}

pub fn sugerir_similar(nome: &str, disponiveis: &[String]) -> Option<String> {
    let mut melhor_match: Option<(String, usize)> = None;

    for disponivel in disponiveis {
        let distancia = levenshtein_distance(nome, disponivel);
        if distancia <= 2 {
            match melhor_match {
                None => melhor_match = Some((disponivel.clone(), distancia)),
                Some((_, d)) if distancia < d => {
                    melhor_match = Some((disponivel.clone(), distancia))
                }
                _ => {}
            }
        }
    }

    melhor_match.map(|(nome, _)| format!("Você quis dizer '{}'?", nome))
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

pub fn erro(msg &str) {
    LucasError::novo(ErrorType::RuntimeError, msg.to_string()).exibir();
}