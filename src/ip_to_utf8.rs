use utf8toipv4::ipv4_to_utf8;
use std::io::{self,Write};
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        let _ = writeln!(io::stdout(),"Usage: ipv4toutf8 [ipv4 address]").unwrap();
        return;
    }

    let input = args[1].clone();

    match ipv4_to_utf8(&input) {
        Ok(r) => {
            let _ = writeln!(io::stdout(),"{}",r).unwrap();
        }
        Err(r) => {
            let _ = writeln!(io::stdout(),"{}",r).unwrap();
        }
    }
}
