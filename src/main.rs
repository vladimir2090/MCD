use std::env;
use std::process::exit;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cmp::{max, min};

pub struct Xorshift8 {
    state: u8,
}

impl Xorshift8 {
    pub fn new(seed: u8) -> Self { Self { state: seed } }
    pub fn next(&mut self) -> u8 {
        let mut x = self.state;
        x ^= x << 3;
        x ^= x >> 1;
        x ^= x << 4;
        self.state = x;
        x
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length: usize = match args.get(1) {
        Some(s) => match s.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("Error: '{}' is not a valid integer", s);
                exit(1);
            }
        },
        None => 4,
    };

    let unique_symbols = max(2, min(length - 1, 8));
    let letters = ['a','b','c','d','e','f','g','h'];
    let chosen: Vec<char> = letters[..unique_symbols].to_vec();

    let seed = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => ((duration.as_nanos() as u8) + length as u8) % 255,
        Err(_) => 17,
    };
    let mut rng = Xorshift8::new(seed);

    let mut digits_pool: Vec<u8> = (0..10).collect();
    for i in (1..digits_pool.len()).rev() {
        let j = (rng.next() as usize) % (i + 1);
        digits_pool.swap(i, j);
    }
    
    let letter_digits: Vec<u8> = digits_pool.into_iter().take(unique_symbols).collect();

    for i in 0..length {
        let idx = i % unique_symbols;
        print!("{}{}", chosen[idx], letter_digits[idx]);
    }
    println!();
}