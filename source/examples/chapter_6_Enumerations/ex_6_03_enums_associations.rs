#[derive(Debug)]
enum IpAddrKind {
	IPv4(String),
	IPv6(String),
}

enum IpAddr {
	IPv4(u8,u8,u8,u8),
	IPv6(String),
}

fn main() {
	
	let home = IpAddrKind::IPv4(String::from("127.0.0.1"));
	let loopback = IpAddrKind::IPv6(String::from("::1"));
	let home2 = IpAddr::IPv4(127,0,0,1);
	let loopback2 = IpAddr::IPv6(String::from("::1"));
}