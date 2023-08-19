mod matrix;
use matrix::keygen;
use matrix::encrypt;
use matrix::decrypt;
use std::path::Path;

use matrix::Equation;

const PATH: &str = "/Users/andy/passwords";

macro_rules! is {($boolexpr:expr, $error:expr) => {($boolexpr).then_some(()).expect($error)}}

use FIELD::*;
pub enum FIELD {
    USERNAME,
    PASSWORD,
}

fn main() {
    let path = Path::new(PATH);
    let mut args: Vec<String> = std::env::args().collect();
    is!(args.len() >= 2, "Too little args given");
    let master_password: &[i64] = &std::mem::take(&mut args[1])
                                  .as_bytes()
                                  .iter()
                                  .map(|&x| x as i64)
                                  .collect::<Vec<i64>>();
    match args[2].as_str() {
        "keygen" => {
            is!(args.len() >= 3, "Too little args given for keygen instruction");
            match keygen::write_keys(path, master_password, 314159265, 2521) {
                Ok(()) => (),
                Err(e) => panic!("I think writing failed. Kind: {}", e),
            }
        }
        "encrypt" => {
            is!(args.len() >= 6, "Too little args given for encrypt instruction");
            let equation = Equation::read_equation_from_file(path);
            let name = std::mem::take(&mut args[3]);
            let username = std::mem::take(&mut args[4]);
            let password = std::mem::take(&mut args[5]);
            match encrypt::encrypt_string(&equation, name, username, password, path) {
                Ok(_) => (),
                Err(e) => panic!("encrypt error of kind {}", e),
            };
        }
        "decrypt" => {
            let mut field = USERNAME;
            if args[4] == "username" {
            } else if args[4] == "password" {
                field = PASSWORD;                
            } else {
                panic!("Not a valid field to decrypt");
            }
            let string = decrypt::decrypt_file(path, std::mem::take(&mut args[3]), master_password, field);
            print!("{string}");
        }
        _ => ()
    }
}

