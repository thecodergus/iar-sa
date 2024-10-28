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
    estado_inicial: Vec<bool>,
    sat: Vec<Vec<i32>>,
    temperatura_inicial: f64,
    alfa: f64,
    maximo_interacoes: usize,
    fn_temperatura: &dyn Fn(f64, f64, f64, f64) -> f64,
) -> (Vec<bool>, Vec<Output>) {
    let mut historico: Vec<Output> = Vec::new();
    let mut estado: Vec<bool> = estado_inicial;
    let mut energia: f64 = funcao_objetivo(&sat, &estado);
    let mut temperatura: f64 = temperatura_inicial;
    let mut melhor_estado: Vec<bool> = estado.clone();
    let mut melhor_energia: f64 = energia;
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let mut contador: usize = 0;

    historico.push(Output {
        fo: energia,
        interacao: contador,
        temperatura,
        trues: somar_trues(&sat, &melhor_estado),
    });

    println!(
        "Iteração: {} | Temperatura: {:.4} | Energia: {:.4} | Trues: {}",
        0,
        temperatura,
        energia,
        somar_trues(&sat, &melhor_estado)
    );

    while temperatura >= 1e-4 {
        for interacao in 1..=maximo_interacoes {
            let proximo_estado: Vec<bool> = bit_flip_with_probability(&estado, 0.05);
            let nova_energia: f64 = funcao_objetivo(&sat, &proximo_estado);
            let de: f64 = nova_energia - energia;

            if de < 0.0 || rng.gen::<f64>() <= (-de / temperatura).exp() {
                estado = proximo_estado;
                energia = nova_energia;
            }

            if energia < melhor_energia {
                melhor_estado = estado.clone();
                melhor_energia = energia;
            }
        }
        // temperatura = fn_temperatura(temperatura, de, alfa, interacao);
        temperatura *= alfa;
        contador += 1;

        historico.push(Output {
            fo: energia,
            interacao: contador,
            temperatura,
            trues: somar_trues(&sat, &melhor_estado),
        });

        println!(
            "Iteração: {} | Temperatura: {:.4} | Energia: {:.4} | Trues: {}",
            contador,
            temperatura,
            energia,
            somar_trues(&sat, &melhor_estado)
        );
    }

    (melhor_estado, historico)
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
