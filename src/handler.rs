use pnet::packet::ethernet::{EtherType, EtherTypes, EthernetPacket};

pub fn handle_ethernet_frame(bytes: &[u8]) {
    let ether_frame = match EthernetPacket::new(bytes) {
        Some(eth) => eth,
        None => return
    };

    match ether_frame.get_ethertype() {
        EtherTypes::Arp => println!("Arp"),
        EtherTypes::Ipv4 => println!("IPv4"),
        EtherTypes::Ipv6 => println!("IPv6"),
        _ => return
    };
}