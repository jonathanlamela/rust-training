#[derive(Debug)]
enum IpType {
    Ipv4,
    Ipv6,
}

struct IpAddr {
    kind: IpType,
    address: String,
}

fn main() {
    let home = IpAddr {
        kind: IpType::Ipv4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpType::Ipv6,
        address: String::from("::1"),
    };
    println!("Home IP: {}, Type: {:?}", home.address, home.kind);
    println!(
        "Loopback IP: {}, Type: {:?}",
        loopback.address, loopback.kind
    );

    match home.kind {
        IpType::Ipv4 => println!("This is an IPv4 address."),
        IpType::Ipv6 => println!("This is an IPv6 address."),
    }

    //Uses of Some
    let some_ip = Some(IpAddr {
        kind: IpType::Ipv4,
        address: String::from("192.168.1.1"),
    });

    match some_ip {
        Some(IpAddr { kind, address }) => {
            println!("IP Address: {}, Type: {:?}", address, kind);
        }
        None => println!("No IP Address found."),
    }
}
