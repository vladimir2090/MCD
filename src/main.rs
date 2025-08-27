use std::env;

pub struct Xorshift8 {
    state: u8,
}

impl Xorshift8 {
    pub fn new(seed: u8) -> Self {
        Self { state: seed }
    }

    pub fn next(&mut self) -> u8 {
        let mut x = self.state;
        x ^= x << 2;
        x ^= x >> 5;
        x ^= x << 1;
        self.state = x;
        x
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let length: u64 = if args.len() < 2 {
        4
    } else {
        match args[1].parse::<u64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: '{}' is not a valid integer", args[1]);
                std::process::exit(1); //err in input
            }
        }
    };
    println!("length: {}", length); //for test
    let mut rng = Xorshift8::new(123);

    for _ in 0..length {
        println!("Next digit: {}", rng.next());
    }
}