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

    return sat_booleano
        .iter()
        .filter(|v| **v)
        .collect::<Vec<&bool>>()
        .len() as f64
        / sat_booleano.len() as f64;
}

pub fn simulated_annealing(
    estado_incial: Vec<bool>,
    sat: Vec<Vec<i32>>,
    temperatura_inicial: f64,
    alfa: f64,
    maximo_interacoes: usize,
    fn_temperatura: &dyn Fn(f64, f64, f64, f64) -> f64,
) -> (Vec<bool>, Vec<Output>) {
    let mut historico: Vec<Output> = vec![];
    let mut estado: Vec<bool> = estado_incial.clone();
    let mut energia: f64 = funcao_objetivo(&sat, &estado);
    let mut temperatura: f64 = temperatura_inicial;
    let mut melhor_estado: Vec<bool> = estado.clone();
    let mut rng = thread_rng();

    historico.push(Output {
        fo: energia,
        interacao: 0,
        temperatura: temperatura.clone(),
        trues: somar_trues(&sat, &melhor_estado),
    });

    println!(
        "Temperatura: {} | Energia: {} | Trues: {}",
        temperatura,
        energia,
        somar_trues(&sat, &melhor_estado)
    );

    for _ in 0..maximo_interacoes {
        let proximo_estado: Vec<bool> = bit_flip_with_probability(&estado, 5e-2);
        let nova_energia: f64 = funcao_objetivo(&sat, &proximo_estado);

        estado = {
            let de: f64 = nova_energia - energia;

            if de < 0.0 || rng.gen_range(0.0..=1.0) <= f64::consts::E.powf(-de / temperatura) {
                energia = nova_energia;
                proximo_estado
            } else {
                estado
            }
        };

        if funcao_objetivo(&sat, &melhor_estado) < funcao_objetivo(&sat, &estado) {
            melhor_estado = estado.clone();
        }

        temperatura *= alfa;

        historico.push(Output {
            fo: energia,
            interacao: 0,
            temperatura: temperatura.clone(),
            trues: somar_trues(&sat, &melhor_estado),
        });

        println!(
            "Temperatura: {} | Energia: {} | Trues: {}",
            temperatura,
            energia,
            somar_trues(&sat, &melhor_estado)
        );

        if temperatura <= 1e-4 {
            break;
        }
    }

    return (melhor_estado, historico);
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
