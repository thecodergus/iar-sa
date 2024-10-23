use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
