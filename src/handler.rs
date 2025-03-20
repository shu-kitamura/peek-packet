use pnet::packet::{
    arp::ArpPacket, ethernet::{EtherTypes, EthernetPacket}, ip::IpNextHeaderProtocols, ipv4::Ipv4Packet, ipv6::Ipv6Packet, Packet
};

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
        EtherTypes::Ipv4 => match Ipv4Packet::new(ether_frame.payload()) {
            Some(packet) => handle_ipv4_packet(packet),
            None => return
        },
        EtherTypes::Ipv6 => match Ipv6Packet::new(ether_frame.payload()) {
            Some(packet) => handle_ipv6_packet(packet),
            None => return
        },
        _ => return
    };
}

fn handle_arp_packet(arp_packet: ArpPacket) {
    match arp_packet.get_protocol_type() {
        EtherTypes::Ipv4 => println!("{:?}", arp_packet),
        _ => return
    }
}

fn handle_ipv4_packet(ipv4_packet: Ipv4Packet) {
    match ipv4_packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => println!("TCP"),
        IpNextHeaderProtocols::Udp => println!("UDP"),
        IpNextHeaderProtocols::Icmp => println!("ICMP"),
        _ => return
    }
}

fn handle_ipv6_packet(ipv6_packet: Ipv6Packet) {
    match ipv6_packet.get_next_header() {
        IpNextHeaderProtocols::Tcp => println!("TCP"),
        IpNextHeaderProtocols::Udp => println!("UDP"),
        IpNextHeaderProtocols::Icmp => println!("ICMP"),
        _ => return
    }
}