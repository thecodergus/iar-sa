use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lib::cooling_schedule::um;
use lib::principal::simulated_annealing;

mod lib;

fn main() -> io::Result<()> {
    let path = "uf20-01.cnf"; // Substitua pelo caminho do seu arquivo
    let clauses = lib::arquivo::parse_cnf_file(path)?;

    let solucao_aleatoria: Vec<bool> = lib::vetores::random_bool_vector(20);

    let temperatura: f64 = 1000.0;
    let alfa: f64 = 0.95;
    let maximo_interacoes: usize = 10_000;

    let (melhor, historico) = simulated_annealing(
        solucao_aleatoria,
        clauses,
        temperatura,
        alfa,
        maximo_interacoes,
        &um,
    );

    println!("{:?}", historico);
    println!("{:?}", melhor);

    Ok(())
}
