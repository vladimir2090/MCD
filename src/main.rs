use std::io;
use rand::rng;

fn main(){
    
    let mut input_long_str = String::new();
    let mut tips_code = String::new();

    io::stdin()
        .read_line(&mut input_long_str)
        .expect("BulbaZavr");
    let input_long: u32 = input_long_str.trim().parse().expect("BulbaZavr");

    io::stdin()
        .read_line(&mut tips_code)
        .expect("BulbaZavr");
    let tips_code: String = tips_code.trim();

    
}