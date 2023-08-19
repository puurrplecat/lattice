use super::{Matrix, Equation, Row};
use super::rng;
use std::path::Path;
use std::fs::File;
use std::io::{Write, Result};

// HOLY SHIT CLOSURES CAN CAPTURE LIFETIMES, PLUS THE LIFETIMES ARE ELIDED
fn encrypt_bit(equation: &Equation) -> impl '_ + FnMut(u8) -> (Row, i64) {
    let mut rng = rng(17709312065);
    move |bit| {
        // closure owns the rng. So it is random everytime this closure is called.
        let start = rng() % (equation.get_height() / 2);
        let mut row = Row::new_zeroed(equation.get_width());
        let mut equals_to = 0;
        // Adds some random rows together. This makes it harder than using a row directly from the public key.
        // (Is it hard to find which rows were added together? probably, due to the added errors)
        for i in start..start + rng() % 20 {
            row += &equation.matrix[i];
            equals_to += equation.result[i][0];
        }
        row %= equation.modulus;

        match bit {
            1 => { (row, (equals_to + equation.modulus / 2) % equation.modulus) }
            0 => { (row, equals_to % equation.modulus)                      }
            _ => { panic!("Cannot encrypt a non binary value")          }
        }
    }
}

// a char is 8 bits (1 byte). Since learning with errors encrypts bits by multiplying one row with the private key
// and comparing with the encrypted value, we multiply a matrix of many rows with the private key.
pub fn encrypt_char(equation: &Equation, char: String, bit_encrypt: &mut impl FnMut(u8) -> (Row, i64)) -> (Matrix, Matrix) {
    let byte = char.as_bytes();
    let mut new_matrix = Matrix::matrix_builder(8, equation.get_width());
    let mut result = Matrix::matrix_builder(8, 1); // column vector

    for i in (0..8).rev() {
        let bit = (byte[0] & 1 << i) << (7 - i) >> 7; // extracts the each bit from left to right
        let (row, encrypt_value) = bit_encrypt(bit);

        new_matrix.push_row(row);
        result.push_value(encrypt_value);
    }
    (new_matrix, result)
}

pub fn encrypt_string(equation: &Equation, name: String, username: String, password: String, path: &Path) -> Result<usize> {
    let mut file = File::create(path.join(name)).expect("File jumped off a cliff");
    writeln!(file, "{}", equation.modulus)?;
    // the function to encrypt. We create it here so the closure uses the same rng in both username and password. I.e.,
    // the closure is not recreated and the rng restarted if created in the encrypt_char function
    let mut bit_encrypt = encrypt_bit(equation); 

    // length of username/password printed to facilitate decryption
    writeln!(file, "{}", username.len())?;
    for i in username.chars() {
        let (vec, result) = encrypt_char(equation, i.to_string(), &mut bit_encrypt);
        writeln!(file, "{}", vec)?;
        writeln!(file, "{}", result)?;
    }
    writeln!(file, "{}", password.len())?;
    for i in password.chars() {
        let (vec, result) = encrypt_char(equation, i.to_string(), &mut bit_encrypt);
        writeln!(file, "{}", vec)?;
        writeln!(file, "{}", result)?;
    }
    Ok(0)
}