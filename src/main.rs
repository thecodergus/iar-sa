use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use lib::arquivo::{gerar_grafico_convergencia, gerar_grafico_temperatura};
use lib::cooling_schedule::{dois, um};
use lib::principal::{simulated_annealing, Output};

mod lib;

use csv::Writer;
use rayon::prelude::*;
use std::error::Error;
use std::sync::Arc;

// Função para salvar os resultados em um arquivo CSV
fn salvar_resultados_csv(path: &str, resultados: &[(usize, Output)]) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(path)?;

    // Escreve o cabeçalho do CSV
    wtr.write_record(&["Tamanho_Vetor", "Temperatura", "Interacao", "FO", "Trues"])?;

    // Escreve cada linha do CSV com os valores de cada `Output`
    for (tamanho_vetor, output) in resultados {
        wtr.write_record(&[
            tamanho_vetor.to_string(),
            output.temperatura.to_string(),
            output.interacao.to_string(),
            output.fo.to_string(),
            output.trues.to_string(),
        ])?;
    }

    // Garantir que os dados sejam escritos no disco
    wtr.flush()?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Lista dos arquivos que serão usados como entrada
    let paths = vec!["uf20-01.cnf", "uf100-01.cnf", "uf250-01.cnf"];

    // Mapeamento do tamanho do vetor com base no nome do arquivo
    let tamanho_vetores = vec![
        ("uf20-01.cnf", 20),
        ("uf100-01.cnf", 100),
        ("uf250-01.cnf", 250),
    ]
    .into_iter()
    .collect::<std::collections::HashMap<&str, usize>>();

    // Configurações para a execução do simulated annealing
    let temperatura: f64 = 1000.0;
    let alfa: f64 = 0.90;
    let maximo_interacoes: usize = 5_000_000;
    let sa_max: usize = 100_000;

    // Usamos Arc para permitir o compartilhamento dos parâmetros entre threads
    let temperatura = Arc::new(temperatura);
    let alfa = Arc::new(alfa);
    let maximo_interacoes = Arc::new(maximo_interacoes);
    let sa_max = Arc::new(sa_max);

    // Vetor para armazenar o último `Output` de cada execução
    let mut resultados_finais = Vec::new();

    // Rodar 30 execuções com 10 threads em paralelo usando rayon
    (0..30)
        .into_par_iter()
        .map(|i| {
            // Determina qual arquivo usar baseado no índice
            let path = paths[i % paths.len()];

            // Obtém o tamanho do vetor booleano com base no nome do arquivo
            let &tamanho_vetor = tamanho_vetores.get(path).unwrap_or(&250);

            // Parse do arquivo de cláusulas
            let clauses = lib::arquivo::parse_cnf_file(path)?;

            // Gera uma solução aleatória com o tamanho adequado
            let solucao_aleatoria: Vec<bool> = lib::vetores::random_bool_vector(tamanho_vetor);

            // Executa o simulated annealing
            let (melhor, historico) = simulated_annealing(
                &dois,
                *alfa,
                *sa_max,
                *maximo_interacoes,
                *temperatura,
                solucao_aleatoria,
                clauses,
            );

            // Gerar os gráficos de convergência e temperatura para cada execução
            let convergencia_path = format!("{} - convergencia_{}.png", tamanho_vetor, i + 1);
            let temperatura_path = format!("{} - temperatura_{}.png", tamanho_vetor, i + 1);

            if let Err(e) = gerar_grafico_convergencia(historico.clone(), &convergencia_path) {
                eprintln!(
                    "Erro ao gerar o gráfico de convergência para execução {}: {}",
                    i + 1,
                    e
                );
            } else {
                println!(
                    "Gráfico de convergência gerado com sucesso para execução {}.",
                    i + 1
                );
            }

            if let Err(e) = gerar_grafico_temperatura(historico.clone(), &temperatura_path) {
                eprintln!(
                    "Erro ao gerar o gráfico de temperatura para execução {}: {}",
                    i + 1,
                    e
                );
            } else {
                println!(
                    "Gráfico de temperatura gerado com sucesso para execução {}.",
                    i + 1
                );
            }

            // Captura o último item do `historico` e adiciona o tamanho do vetor
            let ultimo_output = historico.last().cloned();
            Ok((ultimo_output, tamanho_vetor)) // Retorna o último output e o tamanho do vetor
        })
        .collect::<Result<Vec<_>, io::Error>>()?
        .into_iter()
        .for_each(|(output, tamanho_vetor)| {
            if let Some(out) = output {
                resultados_finais.push((tamanho_vetor, out));
            }
        });

    // Escreve os resultados finais no arquivo CSV
    salvar_resultados_csv("resultados.csv", &resultados_finais)?;

    println!("Resultados finais salvos em 'resultados.csv'.");

    Ok(())
}
