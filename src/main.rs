use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: mcd <pattern> <length>");
        return;
    }

    let _pattern = &args[1];

    let _length = args[2]
        .parse::<u64>()
        .unwrap_or_else(|_| {
            eprintln!("Invalid number: {}", args[2]);
            std::process::exit(1);
        });
}


fn generate(){
    //use Xorshift
}