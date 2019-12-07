enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddrKind {
    fn output(&self) {
        match self {
            IpAddrKind::V4(a, b, c, d) => println!("The IP Address is {}.{}.{}.{}", a, b, c, d),
            IpAddrKind::V6(ip_str) => println!("The IP Address is {}", ip_str),
        }
    }
}

fn main() {
    let home_ip: IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let loopback: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    let some_u8_value = Some(5u8);
    home_ip.output();
    loopback.output();
    let ans = plus_one(some_u8_value);
    println!("Hello, world! {}", ans.unwrap());
}

fn plus_one(x: Option<u8>) -> Option<u8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}