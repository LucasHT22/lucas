use crate::interpreter::Interpreter;
use std::io::{self, Write};

pub struct Repl {
    interpreter: Interpreter,
    history: Vec<String>,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        self.print_welcome();

        loop {
            print!(">>> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let input = input.trim();

                    if input.is_empty() {
                        continue;
                    }

                    if input == "sair" || input == "exit" {
                        self.print_goodbye();
                        break;
                    }

                    if input == "ajuda" || input == "help" || input == "quit" {
                        self.print_help();
                        continue;
                    }

                    if input == "limpar" || input == "clear" {
                        self.clear_screen();
                        continue;
                    }

                    if input == "historico" || input == "history" {
                        self.print_history();
                        continue;
                    }

                    if input == "variaveis" || input == "vars" {
                        self.print_variables();
                        continue;
                    }

                    self.history.push(input.to_string());
                    self.execute(input);
                }
                Err(error) => {
                    eprintln!("Erro ao ler entrada: {}", error);
                    break;
                }
            }
        }
    }

    fn execute(&mut self, code: &str) {
        let code = if !code.ends_with(';') && !code.starts_with("variavel") && !code.starts_with("funcao") && !code.starts_with("se") && !code.starts_with("enquanto") && !code.starts_with("para") {
            format!("imprimir({});", code)
        } else {
            code.to_string()
        };

        let tokens = crate::lexer::Lexer::new(&code).tokenize();
        let mut parser = crate::parser::Parser::new(tokens);
        let stmts = parser.parse();

        for stmt in stmts {
            if let Err(e) = self.interpreter.execute(&stmt) {
                eprintln!("Erro: {}", e);
            }
        }
    }

    fn print_welcome(&self) {
        println!("\n╔═══════════════════════════════════════════════════╗");
        println!("║            Lucas Language REPL v0.1.0             ║");
        println!("╚═══════════════════════════════════════════════════╝");
        println!("\nDigite 'ajuda' para ver comandos disponíveis");
        println!("Digite 'sair' para encerrar\n");
    }

    fn print_goodbye(&self) {
        println!("\nVolte logo!! Obrigado por usar Lucas Language!");
    }

    fn print_help(&self) {
        println!("\nComandos Disponíveis:");
        println!("  ajuda, help        - Mostra esta mensagem");
        println!("  sair, exit, quit   - Sai do REPL");
        println!("  limpar, clear      - Limpa a tela");
        println!("  historico, history - Mostra histórico de comandos");
        println!("  variaveis, vars    - Mostra variáveis definidas");
        println!("\nExemplos:");
        println!("  >>> 2 + 2");
        println!("  >>> variavel x = 10");
        println!("  >>> imprimir(x * 2)");
        println!("  >>> funcao somar(a, b) {{ retornar a + b; }}");
        println!();
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    fn print_history(&self) {
        if self.history.is_empty() {
            println!("Histórico vazio");
            return;
        }

        println!("\nHistórico de Comandos:");
        for (i, cmd) in self.history.iter().enumerate() {
            println!("  [{}] {}", i + 1, cmd);
        }
        println!();
    }

    fn print_variables(&self) {
        println!("\nVariáveis Globais:");

        let globals = self.interpreter.globals.borrow();
        let vars = globals.get_all_variables();

        if vars.is_empty() {
            println!("  (nenhuma variável definida)");
        } else {
            for (name, value) in vars {
                println!("  {} = {}", name, value.to_string_repr());
            }
        }
        println!();
    }
}