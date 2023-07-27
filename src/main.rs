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
    println!("the encrypt {}, truth is {}", v, ans);
    println!("{}", mods(v - ans, MOD));
    // distance function in modulus field
    if mod_dist(v - ans, 0) < mod_dist(v - ans, MOD / 2) {
        0
    } else {
        1
    }
}

fn mod_dist(a: i64, b: i64) -> i64 {
    mods(i64::abs(b - a), MOD)
}

fn mods(a: i64, b: i64) -> i64 {
    if a % b < 0 {
        a + b
    } else {
        a % b
    }
}

// How does LEARNING WITH ERRORS work. 
// It is proven to be as hard as solving the closest vector problem.

// Suppose I have a system of equations of 10 variables. The more equations the better (i'm guessing).
// A private key is a particular solution to this system of linear equations under some modulus.
// In fact, you can first generate the private key, then finalise the system of equations with a simple multiply to a matrix of coefficients.
// Then you introduce some error (best if there is a even mix of positive and negative as we'll see later).

// To encrypt a bit, the user takes the system of equations, sums a few of them and call it N. This means the errors are cumulatively summed. If the errors don't have a spread of positive or negative, it can exceed the modulus we are working under and it doesn't work. To be honest I am not 100% clear if that's the reason.

// Then if we want to encrypt 1, we add half the modulus to what N equals. To encrypt 0, we do nothing

// Now decryption, if given N, it is very hard to tell if this is 0 or 1. But with the private key, we substitute it into N
// to get what the equation should equal, and we can easily tell if half of the mod was added or not. It is extremely hard without the private key to find what it should be.

// This is similar to the closest vector problem in that it is impossible to find the closest vector to the error filled vector (as this will be our private key). and without the private key, you cannot decrypt every bit sensibly to get information.

// This is a simple example of post quantum cryptography 
