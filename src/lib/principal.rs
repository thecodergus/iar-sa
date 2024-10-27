use rand::Rng;

use super::vetores::{bitflip_random, change_for_boolean};
use std::f64::consts::E;

pub fn funcao_objetivo(sat: &Vec<Vec<usize>>, booleanos: &Vec<bool>) -> f64 {
    let sat_booleano: Vec<bool> = change_for_boolean(sat, booleanos)
        .iter()
        .map(|vector| vector.iter().any(|&i| i))
        .collect::<Vec<bool>>();

    return 1.0
        - (sat_booleano
            .iter()
            .filter(|v| **v)
            .collect::<Vec<&bool>>()
            .len() as f64
            / sat_booleano.len() as f64);
}

pub fn simulated_annealing(
    mut melhor_solucao: Vec<bool>,
    sat: Vec<Vec<usize>>,
    mut temperatura: f64,
    alfa: f64,
    maximo_interacoes: usize,
) -> Vec<bool> {
    let mut s: Vec<bool> = melhor_solucao.clone();
    while temperatura > 1e-4 {
        let mut iter: usize = 0;

        while iter < maximo_interacoes {
            iter += 1;

            let vizinho: Vec<bool> = bitflip_random(&s, 0.15);
            let delta: f64 = funcao_objetivo(&sat, &vizinho) - funcao_objetivo(&sat, &s);

            if delta < 0.0 || rand::thread_rng().gen::<f64>() < (-delta / temperatura).exp() {
                s = vizinho;

                if funcao_objetivo(&sat, &s) < funcao_objetivo(&sat, &melhor_solucao) {
                    melhor_solucao = s.clone();
                }
            }
        }

        temperatura *= alfa;
    }

    return melhor_solucao;
}
