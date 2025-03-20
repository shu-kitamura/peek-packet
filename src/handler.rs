use pnet::packet::{arp::ArpPacket, ethernet::{EtherTypes, EthernetPacket}, Packet};

pub fn handle_ethernet_frame(bytes: &[u8]) {
    let ether_frame = match EthernetPacket::new(bytes) {
        Some(eth) => eth,
        None => return
    };

    match ether_frame.get_ethertype() {
        EtherTypes::Arp => match ArpPacket::new(ether_frame.payload()) {
            Some(packet) => handle_arp_packet(packet),
            None => return
        },
        EtherTypes::Ipv4 => println!("IPv4"),
        EtherTypes::Ipv6 => println!("IPv6"),
        _ => return
    };
}

fn handle_arp_packet(arp_packet: ArpPacket) {
    match arp_packet.get_protocol_type() {
        EtherTypes::Ipv4 => println!("{:?}", arp_packet),
        _ => return
    }
}