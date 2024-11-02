use rand::{thread_rng, Rng};

use super::cooling_schedule;

use super::vetores::{bit_flip_with_probability, change_for_boolean};
use core::f64;

#[derive(Debug, Clone)]
pub struct Output {
    pub temperatura: f64,
    pub interacao: usize,
    pub fo: f64,
    pub trues: usize,
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
    f: &dyn Fn(f64, f64, f64, f64) -> f64,
    alfa: f64,
    sa_max: usize,
    max_interacoes: usize,
    t0: f64,
    mut s: Vec<bool>,
    sat: Vec<Vec<i32>>,
) -> (Vec<bool>, Vec<Output>) {
    let mut s_asterisco: Vec<bool> = s.clone();
    let mut iter_t: usize = 0;
    let mut t: f64 = t0;
    let mut historico: Vec<Output> = vec![];
    let mut contador: usize = 0;

    historico.push(Output {
        fo: funcao_objetivo(&sat, &s_asterisco),
        interacao: contador,
        temperatura: t,
        trues: somar_trues(&sat, &s_asterisco),
    });

    // println!(
    //     "Interação: {} | Temperatura: {:.5} | Energia: {:.5} | Numero de Trues: {}",
    //     contador,
    //     t,
    //     funcao_objetivo(&sat, &s_asterisco),
    //     somar_trues(&sat, &s_asterisco)
    // );

    while t > 1e-4 {
        while iter_t < sa_max {
            iter_t += 1;
            contador += 1;
            let s_linha: Vec<bool> = bit_flip_with_probability(&s, 0.05);
            let fo_s_linha: f64 = funcao_objetivo(&sat, &s_linha);
            let fo_s: f64 = funcao_objetivo(&sat, &s);
            let delta: f64 = fo_s_linha - fo_s;

            if delta < 0.0 {
                s = s_linha.clone();
                let fo_s_asterisco: f64 = funcao_objetivo(&sat, &s_asterisco);

                if fo_s_linha < fo_s_asterisco {
                    s_asterisco = s_linha;
                }
            } else {
                let x: f64 = rand::thread_rng().gen::<f64>();
                if x < (-delta / t).exp() {
                    s = s_linha;
                }
            }
        }
        t = f(contador as f64, t0, t * alfa, max_interacoes as f64);
        iter_t = 0;

        historico.push(Output {
            fo: funcao_objetivo(&sat, &s_asterisco),
            interacao: contador,
            temperatura: t,
            trues: somar_trues(&sat, &s_asterisco),
        });

        // println!(
        //     "Interação: {} | Temperatura: {:.5} | Energia: {:.5} | Numero de Trues: {}",
        //     contador,
        //     t,
        //     funcao_objetivo(&sat, &s_asterisco),
        //     somar_trues(&sat, &s_asterisco)
        // );
    }

    return (s_asterisco, historico);
}

fn somar_trues(sat: &Vec<Vec<i32>>, booleanos: &Vec<bool>) -> usize {
    return change_for_boolean(sat, booleanos)
        .iter()
        .map(|vector| vector.iter().any(|&i| i))
        .collect::<Vec<bool>>()
        .iter()
        .filter(|v| **v)
        .collect::<Vec<&bool>>()
        .len();
}
