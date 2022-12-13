
use std::io::stdin;

pub fn read_input() -> String{
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    name
}