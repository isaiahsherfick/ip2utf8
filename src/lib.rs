use std::{io::{self,Write},u32,fmt::{Display,Formatter}};

pub trait ToCodePoint {
    fn to_code_point(&self) -> u32;
}

impl ToCodePoint for char {
    fn to_code_point(&self) -> u32 {
        let escaped = self.escape_unicode().to_string();
        let code_point = escaped.replace(&['\\', '{', '}', 'u'], "");
        u32::from_str_radix(&code_point, 16).unwrap()
    }
}

#[derive(Debug)]
pub enum Error {
    InvalidIpv4Address(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::InvalidIpv4Address(s) => {
                write!(f, "{} is not a valid ipv4 address.",s)?;
                Ok(())
            }
        }
    }
}

impl std::error::Error for Error {}

pub fn utf8_to_ipv4(input: &str) -> String {
    let mut non_zero_bytes: Vec<u8> = vec![];
    for c in input.chars() {
        let bytes = c.to_code_point().to_le_bytes();
        for byte in bytes {
            if byte != 0 {
                non_zero_bytes.push(byte);
            }
        }
    }
    while non_zero_bytes.len() < 4 {
        non_zero_bytes.push(0);
    }
    return format!(
        "{}.{}.{}.{}",
        non_zero_bytes[0], non_zero_bytes[1], non_zero_bytes[2], non_zero_bytes[3]
    );
}

pub fn is_valid_ipv4_address(addr: &str) -> bool {
    let split: Vec<&str> = addr.split(".").collect();
    if split.len() != 4 {
        writeln!(io::stdout(), "addr is not valid: {}",addr).unwrap();
        return false;
    }
    for octet in split {
        match octet.parse::<u8>() {
            Ok(v) => v,
            Err(_) => {
                return false;
            }
        };
    }
    true
}

pub fn ipv4_to_utf8(input: &str) -> Result<String, Error> {
    if !is_valid_ipv4_address(input) {
        return Err(Error::InvalidIpv4Address(input.into()));
    }
    let octets: Vec<&str> = input.split(".").collect();
    let mut solution = String::new();
    for octet in octets {
        let val = octet.parse::<u32>().unwrap();
        let c = char::from_u32(val).unwrap();
        println!("octet={val}, to char={c}");
        solution += &format!("{}",c);
    }
    Ok(solution)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_is_valid_ipv4_address() {
        let valid_addrs = vec![
            "192.168.2.1",
            "255.255.255.255",
            "8.8.8.8",
            "10.10.1.2",
            "144.185.222.255",
            "0.0.0.0",
        ];
        for addr in &valid_addrs {
            assert!(is_valid_ipv4_address(addr));
        }
        let invalid_addrs = vec![
            "600",
            "hello",
            "123",
            "999999999",
            "414321trega",
            "192.168.243.424.242",
            "256.192.192.192",
            "123123.123123",
        ];
        for addr in &invalid_addrs {
            assert!(!is_valid_ipv4_address(addr));
        }
    }
}
