use super::{Matrix, Equation, Row};
use std::string::FromUtf8Error;
use super::rng;
use std::path::Path;
use std::fs::File;
use std::io::Write;

pub fn write_encrypt(path: &Path) {
    
}

// HOLY SHIT CLOSURES CAN CAPTURE LIFETIMES, PLUS THE LIFETIMES ARE ELIDED
fn encrypt_bit(equation: &Equation) -> impl '_ + FnMut(u8) -> (Row, i64) {
    let mut rng = rng(10);
    move |bit| {
        let mut random_index: Vec<usize> = vec![0; 4];
        // closure owns the rng. So it is random everytime this closure is called.
        random_index.fill_with(|| rng() % equation.get_height()); // NO CLOSURE WTH
        // Adds some random rows together. This makes it harder than using a row directly from the public key.
        // (Is it hard to find which rows were added together? probably, due to the added errors)
        let mut row = Row::new_zeroed(equation.get_width());
        let mut equals_to = 0;
        for i in random_index {
            row += &equation.matrix[i];
            equals_to += equation.result[i][0];
        }

        match bit {
            1 => { (row, (equals_to + equation.modulus / 2) % equation.modulus) }
            0 => { (row, equals_to % equation.modulus)                      }
            _ => { panic!("Cannot encrypt a non binary value")          }
        }
    }
}

pub fn encrypt_char(equation: &Equation, char: String) -> (Matrix, Matrix) {
    let byte = char.as_bytes();
    let mut bit_encrypt = encrypt_bit(equation);
    let mut new_matrix = Matrix::matrix_builder(8, equation.get_width());
    let mut result = Matrix::matrix_builder(8, 1); // column vector
    for i in (0..8).rev() {
        let bit = (byte[0] & 1 << i) << (7 - i) >> 7; // extracts the bit
        let (row, encrypt_value) = bit_encrypt(bit);
        new_matrix.push_row(row);
        result.push_value(encrypt_value);
    }
    (new_matrix, result)
}

pub fn encrypt_string(equation: &Equation, name: String, username: String, password: String, path: &Path) {
    let mut file = File::create(path.join(name)).expect("File jumped off a cliff");
    writeln!(file, "{}", equation.modulus).unwrap();
    for i in username.chars() {
        println!("encrypting {}", i);
        let (vec, result) = encrypt_char(equation, i.to_string());
        writeln!(file, "{}", vec).unwrap();
        writeln!(file, "{}", result).unwrap();
    }
}

fn decrypt_bit(ans: i64, v: i64, modulus: i64) -> u8 {
    let val = i64::abs((v - ans) % modulus); // always positive modulus.
    if std::cmp::min(modulus - val, val) < i64::abs(val - modulus / 2) {
        0
    } else {
        1
    }
}

pub fn decrypt_char(encrypt_matrix: Matrix, encrypt_result: Matrix, priv_key: Matrix, modulus: i64) -> Result<String, FromUtf8Error> {
    let mut decrypt_result = encrypt_matrix * priv_key;
    decrypt_result.map(|x| x % modulus);
    let mut i = -1;
    let decrypt_string = decrypt_result.matrix
    .iter()
    .zip(encrypt_result.matrix.iter())
    .map(|(x, y)| decrypt_bit(x[0], y[0], modulus))
    .fold(0, |acc, x| {i += 1; acc ^ (x << 7 >> i)});
    std::string::String::from_utf8(vec![decrypt_string])
}