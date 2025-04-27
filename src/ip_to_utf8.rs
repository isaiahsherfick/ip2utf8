use ip2utf8::ipv4_to_utf8;
use std::io::{self,Write};
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        writeln!(io::stdout(),"Usage: ipv4toutf8 [ipv4 address]").unwrap();
        return;
    }

    let input = args[1].clone();

    match ipv4_to_utf8(&input) {
        Ok(r) => {
            writeln!(io::stdout(),"{}",r).unwrap()
        }
        Err(r) => {
            writeln!(io::stdout(),"{}",r).unwrap();
        }
    }
}
