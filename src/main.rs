use std::io;

fn main(){
    
    let mut input_long_str = String::new();

    io::stdin()
        .read_line(&mut input_long_str)
        .expect("BulbaZavr");
    let input_long: u32 = input_long_str.trim().parse().expect("BulbaZavr");
}