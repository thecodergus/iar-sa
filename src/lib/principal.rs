use rand::Rng;

use super::vetores::{bitflip_random, change_for_boolean};

#[derive(Debug, Clone)]
pub struct Output {
    pub temperatura: f64,
    pub interacao: usize,
    pub clausulas_verdadeiras: usize,
}

pub fn funcao_objetivo(sat: &Vec<Vec<i32>>, booleanos: &Vec<bool>) -> f64 {
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
    sat: Vec<Vec<i32>>,
    mut temperatura: f64,
    alfa: f64,
    maximo_interacoes: usize,
    fn_temperatura: &dyn Fn(f64, f64, f64, f64) -> f64,
) -> (Vec<bool>, Vec<Output>) {
    let mut s: Vec<bool> = melhor_solucao.clone();

    let mut historico: Vec<Output> = vec![];
    let mut contador: usize = 0;

    while temperatura > 1e-4 {
        let mut iter: usize = 0;

        while iter < maximo_interacoes {
            let vizinho: Vec<bool> = bitflip_random(&s, 5e-2);
            let fo_vizinho = funcao_objetivo(&sat, &vizinho);
            let fo_s = funcao_objetivo(&sat, &s);
            let delta: f64 = fo_vizinho - fo_s;

            if delta < 0.0 || rand::thread_rng().gen::<f64>() < (-delta / temperatura).exp() {
                s = vizinho;
                if fo_vizinho < funcao_objetivo(&sat, &melhor_solucao) {
                    melhor_solucao = s.clone();
                }
            }

            iter += 1;
        }

        let melhor_fo = funcao_objetivo(&sat, &melhor_solucao);
        let s_fo = funcao_objetivo(&sat, &s);
        temperatura = fn_temperatura(temperatura, alfa, melhor_fo, s_fo);

        let clausulas_verdadeiras: usize = melhor_solucao
            .iter()
            .filter(|v| **v)
            .collect::<Vec<&bool>>()
            .len();

        historico.push(Output {
            temperatura,
            interacao: contador,
            clausulas_verdadeiras,
        });
        contador += 1;
    }

    return (melhor_solucao, historico);
}
