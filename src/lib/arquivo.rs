use plotters::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use super::principal::Output;

pub fn parse_cnf_file<P: AsRef<Path>>(filename: P) -> io::Result<Vec<Vec<i32>>> {
    let file: File = File::open(filename)?;
    let reader: io::BufReader<File> = io::BufReader::new(file);

    let mut clauses: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line: String = line?;
        // Ignora linhas de comentários e de metadados que começam com 'c' ou 'p'
        if line.starts_with('c') || line.starts_with('p') {
            continue;
        }

        // Divide a linha em partes e converte para i32, ignorando o '0' no final
        let clause: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .filter(|&num| num != 0) // Remove o zero que marca o final de cada cláusula
            .collect();

        if !clause.is_empty() {
            clauses.push(clause);
        }
    }

    Ok(clauses)
}

pub fn gerar_grafico_convergencia(
    dados: Vec<Output>,
    caminho_arquivo: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determinar o tamanho da imagem
    let largura = 800;
    let altura = 600;

    // Criar o backend do gráfico
    let root = BitMapBackend::new(caminho_arquivo, (largura, altura)).into_drawing_area();
    root.fill(&WHITE)?;

    // Definir as margens do gráfico
    let root = root.margin(50, 50, 50, 50);

    // Encontrar os valores máximos e mínimos para os eixos
    let max_iteracao = dados.iter().map(|d| d.interacao).max().unwrap_or(0);
    let min_fo = dados.iter().map(|d| d.fo).fold(f64::INFINITY, f64::min);
    let max_fo = dados.iter().map(|d| d.fo).fold(f64::NEG_INFINITY, f64::max);

    // Configurar o gráfico
    let mut chart = ChartBuilder::on(&root)
        .caption("Convergência do Simulated Annealing", ("sans-serif", 30))
        .x_label_area_size(40)
        .y_label_area_size(50)
        .build_cartesian_2d(0usize..max_iteracao, min_fo..max_fo)?;

    // Desenhar malhas de fundo
    chart
        .configure_mesh()
        .x_desc("Iterações")
        .y_desc("Valor da Função Objetivo")
        .draw()?;

    // Preparar os dados para o gráfico
    let pontos: Vec<(usize, f64)> = dados.iter().map(|d| (d.interacao, d.fo)).collect();

    // Desenhar a linha de convergência
    chart
        .draw_series(LineSeries::new(pontos, &RED))?
        .label("FO por Iteração")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // Desenhar a legenda
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
