fn route(_ip_kind: IpAddrKind){

}

enum IpAddrKind {
	IPv4,
	IPv6,
}

fn main() {
	// The :: is a namespace syntax
	let ip4 = IpAddrKind::IPv4;
	let ip6 = IpAddrKind::IPv6;

	route(ip4);
	route(ip6);
}
