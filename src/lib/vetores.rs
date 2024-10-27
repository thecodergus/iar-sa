use rand::seq::index::sample;
use rand::thread_rng;
use rand::Rng;

pub fn generate_random_vector(n: usize) -> Vec<i32> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-100..=100)).collect()
}

pub fn bitflip_random(vec: &Vec<bool>, percentage: f64) -> Vec<bool> {
    if percentage <= 0.0 || percentage > 100.0 {
        println!("Porcentagem inválida. Deve estar entre 0 e 100.");
        return vec.clone(); // Retorna o vetor original se a porcentagem for inválida.
    }

    let len: usize = vec.len();
    let flip_count: usize = ((percentage / 100.0) * len as f64).round() as usize;

    if flip_count == 0 {
        return vec.clone(); // Se o número de flips calculado for 0, retorna o vetor original.
    }

    // Cria uma cópia do vetor para modificá-lo sem afetar o original
    let mut new_vec: Vec<bool> = vec.clone();

    // Seleciona índices aleatórios para modificar
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let indices_to_flip: rand::seq::index::IndexVec = sample(&mut rng, len, flip_count);

    for index in indices_to_flip.iter() {
        new_vec[index] = !new_vec[index]; // Realiza o bit-flip no índice selecionado
    }

    new_vec // Retorna o novo vetor com os bits alterados
}

pub fn change_for_boolean(values: &Vec<Vec<usize>>, booleans: &Vec<bool>) -> Vec<Vec<bool>> {
    values
        .iter()
        .map(|vector| {
            vector
                .iter()
                .map(|value| {
                    if *value > 0 {
                        *booleans
                            .get(*value - 1)
                            .expect("Erro ao buscar o valor booleano")
                    } else {
                        !booleans
                            .get(*value - 1)
                            .expect("Erro ao buscar o valor booleano")
                    }
                })
                .collect::<Vec<bool>>()
        })
        .collect()
}
