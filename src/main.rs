use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod lib;

fn main() -> io::Result<()> {
    let path = "uf20-01.cnf"; // Substitua pelo caminho do seu arquivo
    let clauses = lib::arquivo::parse_cnf_file(path)?;

    // Exibir as cláusulas como vetor de vetores
    for clause in &clauses {
        println!("{:?}", clause);
    }

    Ok(())
}
