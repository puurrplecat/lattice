use super::Matrix;
use std::string::FromUtf8Error;
use crate::{FIELD, FIELD::*};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn decrypt_file(path: &Path, name: String, private_key: &[i64], field: FIELD) -> String {
    let private_key = Matrix::from_slice(private_key, private_key.len());
    let file = BufReader::new(File::open(path.join(name)).expect("No such account found"));
    let mut lines = file.lines();
    let modulus = lines.next().unwrap().unwrap().parse().unwrap();
    let username = decrypt_field(&mut lines, &private_key, modulus);
    let password = decrypt_field(&mut lines, &private_key, modulus);
    match field {
        USERNAME => username,
        PASSWORD => password,
    }
}

fn decrypt_field(lines: &mut Lines<BufReader<File>>, private_key: &Matrix, modulus: i64) -> String {
    let mut string = String::new();
    // str_len is number of lines to read
    let str_len: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..str_len {
        let matrix = Matrix::read_matrix(lines, 8);
        let result = Matrix::read_matrix(lines, 8);
        let s = decrypt_char(matrix, result, private_key, modulus).unwrap();
        string.push_str(&s);
    }
    string
}

pub fn decrypt_char(encrypt_matrix: Matrix, encrypt_result: Matrix, priv_key: &Matrix, modulus: i64) -> Result<String, FromUtf8Error> {
    let mut decrypt_result = &encrypt_matrix * priv_key;
    decrypt_result.map(|x| x % modulus);
    let mut i = -1;
    let decrypt_string = decrypt_result.matrix
        .iter()
        .zip(encrypt_result.matrix.iter())
        .map(|(x, y)| decrypt_bit(x[0], y[0], modulus))
        .fold(0, |acc, x| {i += 1; acc ^ (x << 7 >> i)});

    std::string::String::from_utf8(vec![decrypt_string])
}

fn decrypt_bit(ans: i64, v: i64, modulus: i64) -> u8 {
    let val = i64::abs(v - ans) % modulus; // always positive modulus.
    let dist_from_0 = std::cmp::min(modulus - val, val); // not sure if modulus - val is needed
    let dist_from_mid = i64::abs(modulus/2 - val);
    if dist_from_0 < dist_from_mid {
        0
    } else {
        1
    }
}

