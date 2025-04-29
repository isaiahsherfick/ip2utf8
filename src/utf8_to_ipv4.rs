use std::io::{self, Write};
use utf8toipv4::utf8_to_ipv4;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        let _ = writeln!(io::stdout(), "{}", String::from("0.0.0.0"));
        return;
    }

    let input = args[1].clone();

    let _ = writeln!(io::stdout(), "{}", utf8_to_ipv4(&input));
}
