use rand::{thread_rng, Rng};

use super::vetores::{bitflip_random, change_for_boolean};

#[derive(Debug, Clone)]
pub struct Output {
    pub temperatura: f64,
    pub interacao: usize,
    pub fo: f64,
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
    mut s: Vec<bool>,
    sat: Vec<Vec<i32>>,
    temperatura_inicial: f64,
    alfa: f64,
    maximo_interacoes: usize,
    fn_temperatura: &dyn Fn(f64, f64, f64, f64) -> f64,
) -> (Vec<bool>, Vec<Output>) {
    let mut s_asterisco: Vec<bool> = s.clone();
    let mut iter_t: usize = 0;
    let mut contador: usize = 0;
    let mut temperatura: f64 = temperatura_inicial;
    let mut historico: Vec<Output> = vec![];

    while temperatura < 1e-4 {
        while iter_t < maximo_interacoes {
            iter_t += 1;
            contador += 1;

            let s_linha: Vec<bool> = bitflip_random(&s, 5e-2);
            let delta: f64 = funcao_objetivo(&sat, &s_linha) - funcao_objetivo(&sat, &s);

            if delta < 0.0 {
                s = s_linha.clone();

                if funcao_objetivo(&sat, &s_linha) < funcao_objetivo(&sat, &s) {
                    s_asterisco = s_linha;
                }
            } else {
                if thread_rng().gen_range(0.0..=1.0) < (-delta / temperatura).exp() {
                    s = s_linha;
                }
            }

            historico.push(Output {
                interacao: contador,
                fo: funcao_objetivo(&sat, &s_asterisco),
                temperatura,
            });
        }
        temperatura *= alfa;
        iter_t = 0;
    }

    return (s_asterisco, historico);
}
