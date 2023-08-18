mod matrix;
use matrix::keygen;
use matrix::encrypt;
use matrix::decrypt;
use std::path::Path;

use matrix::Equation;

const PATH: &str = "/Users/andy/passwords";

macro_rules! is {($boolexpr:expr, $error:expr) => {($boolexpr).then_some(()).expect($error)}}

fn main() {
    let sample_private_key = &[1,2,3,4,5,6,7,8,9,10];
    let path = Path::new(PATH);
    let mut args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    is!(args.len() >= 2, "Too little args given");
    // args[1] will be master password
    match args[2].as_str() {
        "keygen" => {
            is!(args.len() >= 3, "Too little args given for keygen instruction");
            keygen::write_keys(path, sample_private_key, 10, 211);
        }
        "encrypt" => {
            is!(args.len() >= 6, "Too little args given for encrypt instruction");
            let equation = Equation::read_equation_from_file(path);
            let name = std::mem::take(&mut args[3]);
            let username = std::mem::take(&mut args[4]);
            let password = std::mem::take(&mut args[5]);
            encrypt::encrypt_string(&equation, name, username, password, path);
        }
        "decrypt" => {
            let equation = Equation::read_equation_from_file(path);
            let username = decrypt::decrypt_file(equation, path, std::mem::take(&mut args[3]), sample_private_key);
            println!("{}", username);
        }
        _ => ()
    }
}

