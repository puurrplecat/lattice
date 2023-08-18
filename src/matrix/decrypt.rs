use super::{Matrix, Equation, Row};
use std::string::FromUtf8Error;
use super::rng;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};

pub fn decrypt_file(public_key: Equation, path: &Path, name: String, private_key: &[i64]) -> String {
    let private_key = Matrix::from_slice(private_key, 10);
    let file = BufReader::new(File::open(path.join(name)).expect("No such account found"));
    let mut lines = file.lines().peekable();
    let modulus: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut string = String::new();
    while lines.peek().is_some() {
        let matrix = Matrix::from_slice(&lines.next().unwrap().unwrap()
                                              .split_whitespace()
                                              .map(|x| x.parse().unwrap())
                                              .collect::<Vec<i64>>(), 8);
        let result = Matrix::from_slice(&lines.next().unwrap().unwrap()
                                              .split_whitespace()
                                              .map(|x| x.parse().unwrap())
                                              .collect::<Vec<i64>>(), 8);
        let s = decrypt_char(matrix, result, &private_key, modulus).unwrap();
        string.push_str(&s);
    }
    string
}

fn decrypt_bit(ans: i64, v: i64, modulus: i64) -> u8 {
    let val = i64::abs(v - ans); // always positive modulus.
    let dist_from_0 = std::cmp::min(modulus - val, val - 0); // not sure if modulus - val is needed
    let dist_from_mid = i64::abs(modulus/2 - val);
    if dist_from_0 <dist_from_mid {
        0
    } else {
        1
    }
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