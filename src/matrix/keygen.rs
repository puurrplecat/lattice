use super::{Matrix, Equation, rng};
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn write_keys(path: &Path, private_key: &[i64], seed: usize, modulus: i64) {
    let mut file = File::create(path.join("public_key.txt")).expect("File died?");
    let equation = generate_keys(private_key, seed, 20, 10, modulus);
    writeln!(file, "{}",  equation).unwrap();
}

fn generate_keys(private_key: &[i64], seed: usize, height: usize, width: usize, modulus: i64) -> Equation {
    assert!(private_key.len() == width);
    let mut rng = rng(seed);
    let mut matrix: Vec<i64> = vec![0; height * width];
    matrix.fill_with(|| rng() as i64 % modulus);
    let public_key = Matrix::from_slice(&matrix, height);


    let private_key = Matrix::from_slice(private_key, width);
    let mut result = (&public_key * private_key) % modulus;

    add_noise(&mut result, modulus);
    Equation::from_matrices(public_key, result, modulus)
}

fn add_noise(m: &mut Matrix, modulus: i64) {
    let mut rng = rng(714014738);
    for i in 0..m.height {
        let v = rng() as i64 % modulus / 10;
        let sign = if rng() % 2 == 1 {1} else {-1};
        m[i][0] += sign * v;
        // changes sign to subtract or add error, this ensures on average the error does not exceed the mod
    }
}
