use std::env;
use std::cmp::{max, min};

pub struct Xorshift16 {
    state: u16,
}

impl Xorshift16 {
    pub fn new(seed: u16) -> Self { Self { state: seed } }
    pub fn next(&mut self) -> u16 {
        let mut x = self.state;
        x ^= x << 5;
        x ^= x >> 3;
        x ^= x << 1;
        self.state = x;
        x
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let length: usize = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let unique_symbols = max(2, min(length - 1, 8));

    let letters = ['a','b','c','d','e','f','g','h','i','j'];
    let chosen: Vec<char> = letters[..unique_symbols].to_vec();

    let mut rng = Xorshift16::new(
        ((std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_nanos() as u16)
         + length as u16)
        % 65535
    );

    let mut digits_pool: Vec<u8> = (0..10).collect();
    for i in (1..digits_pool.len()).rev() {
        let j = (rng.next() as usize) % (i + 1);
        digits_pool.swap(i, j);
    }

    let letter_digits: Vec<u8> = digits_pool.into_iter().take(unique_symbols).collect();

    let pattern: Vec<usize> = (0..length).map(|i| i % unique_symbols).collect();

    for &idx in &pattern {
        print!("{}{}", chosen[idx], letter_digits[idx]);
    }
    println!();
}