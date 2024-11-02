use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lib::arquivo::{gerar_grafico_convergencia, gerar_grafico_temperatura};
use lib::cooling_schedule::{dois, um};
use lib::principal::simulated_annealing;

mod lib;

fn main() -> io::Result<()> {
    let path = "uf250-01.cnf"; // Substitua pelo caminho do seu arquivo
    let clauses = lib::arquivo::parse_cnf_file(path)?;

    let solucao_aleatoria: Vec<bool> = lib::vetores::random_bool_vector(250);

    let temperatura: f64 = 1000.0;
    let alfa: f64 = 0.95;
    let maximo_interacoes: usize = 5_000_000;
    let sa_max: usize = 100_000;

    let (melhor, historico) = simulated_annealing(
        &dois,
        alfa,
        sa_max,
        maximo_interacoes,
        temperatura,
        solucao_aleatoria,
        clauses,
    );

    // println!("{:?}", historico);
    // println!("{:?}", melhor);

    // Gerar o gráfico de convergência
    if let Err(e) = gerar_grafico_convergencia(historico.clone(), "convergencia.png") {
        println!("Erro ao gerar o gráfico: {}", e);
    } else {
        println!("Gráfico de convergência gerado com sucesso.");
    }

    if let Err(e) = gerar_grafico_temperatura(historico, "temperatura.png") {
        println!("Erro ao gerar o gráfico: {}", e);
    } else {
        println!("Gráfico de temperatura gerado com sucesso.");
    }

    Ok(())
}
