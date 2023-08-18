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

pub fn encrypt_char(equation: &Equation, char: String, bit_encrypt: &mut impl FnMut(u8) -> (Row, i64)) -> (Matrix, Matrix) {
    let byte = char.as_bytes();
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
    let mut bit_encrypt = encrypt_bit(equation);
    // TODO USERNAME AND PASSWORD
    for i in username.chars() {
        println!("encrypting {}", i);
        let (vec, result) = encrypt_char(equation, i.to_string(), &mut bit_encrypt);
        writeln!(file, "{}", vec).unwrap();
        writeln!(file, "{}", result).unwrap();
    }
}