use ip2utf8::utf8_to_ipv4;
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        // println!("0.0.0.0");
        println!("{}",String::from("0.0.0.0"));
        return;
    }

    let input = args[1].clone();

    println!("{}",utf8_to_ipv4(&input));
}
