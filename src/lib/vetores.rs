use rand::seq::index::sample;
use rand::thread_rng;
use rand::Rng;

pub fn generate_random_vector(n: usize) -> Vec<i32> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-100..=100)).collect()
}

pub fn random_bool_vector(n: usize) -> Vec<bool> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen()).collect()
}

pub fn bit_flip_with_probability(vetor: &Vec<bool>, p: f64) -> Vec<bool> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

    vetor
        .iter()
        .map(|&bit| {
            // Gera um número aleatório entre 0.0 e 1.0 e inverte o bit se for menor que `p`
            if rng.gen_range(0.0..=1.0) < p {
                !bit
            } else {
                bit
            }
        })
        .collect()
}

pub fn change_for_boolean(values: &Vec<Vec<i32>>, booleans: &Vec<bool>) -> Vec<Vec<bool>> {
    values
        .iter()
        .map(|vector| {
            vector
                .iter()
                .map(|value| {
                    if *value > 0 {
                        *booleans
                            .get((*value).abs() as usize - 1)
                            .expect("Erro ao buscar o valor booleano")
                    } else {
                        !booleans
                            .get((*value).abs() as usize - 1)
                            .expect("Erro ao buscar o valor booleano")
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}
