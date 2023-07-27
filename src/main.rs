mod gauss_elim;
mod matrix;
mod encrypt;
use matrix::Matrix;
use matrix::Equation;

const SIZE: usize = 10;
const MOD: i64 = 157;
const ERROR_MOD: i64 = MOD / 10;
const MATRIX: [i64; SIZE * SIZE] = [67, 36, 60, 81, 149, 11, 38, 25, 107, 36, 31, 4, 57, 34, 104, 56, 35, 100, 56, 100, 145, 87, 29, 129, 44, 28, 18, 111, 91, 113, 5, 52, 67, 113, 92, 112, 76, 20, 152, 80, 90, 13, 76, 136, 147, 136, 101, 65, 123, 11, 47, 154, 147, 92, 89, 7, 39, 116, 127, 48, 12, 127, 94, 64, 50, 24, 7, 19, 142, 17, 92, 22, 79, 114, 16, 110, 156, 36, 0, 122, 29, 128, 114, 148, 73, 57, 49, 63, 132, 35, 153, 42, 57, 120, 123, 98, 34, 63, 129, 100];
const SOLS: [i64; SIZE] = [1660, 1650, 2416, 2104, 2143, 2641, 1804, 1807, 2458, 2521];

fn main() {
    let matrix: Matrix<SIZE, SIZE> = Matrix::from_slice(&MATRIX);
    let sols: Matrix<SIZE, 1> = Matrix::from_slice(&SOLS);
    let mut final_solution: Matrix<SIZE, 1> = matrix * sols;
    final_solution.map(|x| x % MOD);
    let mut equation: Equation<SIZE, SIZE> = Equation::from_matrices(matrix, final_solution, MOD);
    println!("{}", equation);
    equation.add_noise();
    println!("{}", equation);
    let to_encrypt = 1;
    println!("I will encrypt {}", to_encrypt);
    let encrypted = equation.encrypt(to_encrypt);
    println!("I managed to decrypt {}", decrypt(encrypted, Matrix::from_slice(&SOLS)))
}

fn decrypt((encrypt, v): (Matrix<1, SIZE>, i64), priv_key: Matrix<SIZE, 1>) -> i64 {
    let ans = (encrypt * priv_key)[0][0] % MOD;  
    // println!("the encrypt {}, truth is {}", v, ans);
    // println!("{} {}", mod_dist(v - ans, 0), mod_dist(v - ans, MOD / 2));
    let val = (v - ans).rem_euclid(MOD); // always positive modulus.
    // println!("{}", val);
    if val - 0 < i64::abs(val - MOD / 2) {
        0
    } else {
        1
    }
}
