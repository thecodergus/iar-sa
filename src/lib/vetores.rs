use rand::seq::index::sample;
use rand::thread_rng;
use rand::Rng;

pub fn generate_random_vector(n: usize) -> Vec<i32> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(-100..=100)).collect()
}

pub fn bitflip_random(vec: &mut Vec<bool>, percentage: f64) {
    if percentage <= 0.0 || percentage > 100.0 {
        println!("Porcentagem inválida. Deve estar entre 0 e 100.");
        return;
    }

    let len: usize = vec.len();
    let flip_count: usize = ((percentage / 100.0) * len as f64).round() as usize;

    if flip_count == 0 {
        return; // Se o número de flips calculado for 0, não faça nada.
    }

    // Seleciona índices aleatórios para modificar
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let indices_to_flip: rand::seq::index::IndexVec = sample(&mut rng, len, flip_count);

    for index in indices_to_flip.iter() {
        vec[index] = !vec[index]; // Realiza o bit-flip no índice selecionado
    }
}
