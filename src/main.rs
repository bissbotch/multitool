use std::str::FromStr;
use std::env;

fn main() {
    let mut args = Vec::new();

    for arg in env::args().skip(1) {
        args.push(u64::from_str(&arg)
            .expect("Error parsing arguments"));
    }

    if args.len() == 0 {
        eprintln!("usage: broot USER PASSFILE");
        std::process::exit(1);
    }
}




