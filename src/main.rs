use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lib::arquivo::gerar_grafico_convergencia;
use lib::cooling_schedule::um;
use lib::principal::simulated_annealing;

mod lib;

fn main() -> io::Result<()> {
    let path = "uf100-01.cnf"; // Substitua pelo caminho do seu arquivo
    let clauses = lib::arquivo::parse_cnf_file(path)?;

    let solucao_aleatoria: Vec<bool> = lib::vetores::random_bool_vector(100);

    let temperatura: f64 = 1000.0;
    let alfa: f64 = 0.95;
    let maximo_interacoes: usize = 200_000;

    let (melhor, historico) = simulated_annealing(
        solucao_aleatoria,
        clauses,
        temperatura,
        alfa,
        maximo_interacoes,
        &um,
    );

    // println!("{:?}", historico);
    // println!("{:?}", melhor);

    // Gerar o gráfico de convergência
    if let Err(e) = gerar_grafico_convergencia(historico, "convergencia.png") {
        println!("Erro ao gerar o gráfico: {}", e);
    } else {
        println!("Gráfico de convergência gerado com sucesso.");
    }

    Ok(())
}
