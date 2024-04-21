enum IpAddrKind {
	IPv4,
	IPv6,
}

struct IpAddr {
	kind: IpAddrKind,
	address: String,
}


fn main() {
	// The :: is a namespace syntax
	let ip4 = IpAddrKind::IPv4;
	let ip6 = IpAddrKind::IPv6;

	let _home = IpAddr {
		kind: ip4,
		address: String::from("127.0.0.1"),
	};

	let _loopback = IpAddr{
		kind: ip6,
		address: String::from("::1"),
	};
}
