use std::{fmt, net::Ipv4Addr};

use pnet::packet::{arp::{ArpHardwareType, ArpHardwareTypes, ArpOperation, ArpOperations, ArpPacket}, ethernet::EtherType};
use pnet_datalink::MacAddr;

pub struct ArpHeader {
    hardware_type: ArpHardwareType,
    protocol: EtherType,
    sender_mac_addr: MacAddr,
    sender_ip_addr: Ipv4Addr,
    target_mac_addr: MacAddr,
    target_ip_addr: Ipv4Addr,
    operation: ArpOperation,
}

impl<'a> From<ArpPacket<'a>> for ArpHeader {
    fn from(arp_packet: ArpPacket) -> Self {
        ArpHeader {
            hardware_type: arp_packet.get_hardware_type(),
            protocol: arp_packet.get_protocol_type(),
            sender_mac_addr: arp_packet.get_sender_hw_addr(),
            sender_ip_addr: arp_packet.get_sender_proto_addr(),
            target_mac_addr: arp_packet.get_target_hw_addr(),
            target_ip_addr: arp_packet.get_target_proto_addr(),
            operation: arp_packet.get_operation()
        }
    }
}

impl fmt::Display for ArpHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from("ARP header\n") 
                        + &format!("\tHardware type\t\t: {}\n", self.to_string_hw_type())
                        + &format!("\tProtocol\t\t: {}\n", self.to_string_protocol())
                        + &format!("\tSender Mac Address\t: {}\n", self.to_string_sender_mac_addr())
                        + &format!("\tTarget Mac Address\t: {}\n", self.to_string_target_mac_addr())
                        + &format!("\tSender IP Address\t: {}\n", self.to_string_sender_ip_addr())
                        + &format!("\tTarget IP Address\t: {}\n", self.to_string_target_ip_addr())
                        + &format!("\tOperation\t\t: {}\n", self.to_string_operation());
        write!(f, "{}", s)
    }
}

impl ArpHeader {
    fn to_string_hw_type(&self) -> String {
        match self.hardware_type {
            ArpHardwareTypes::Ethernet => String::from("Ethernet"),
            ArpHardwareType(n) => format!("known number '{n}'")
        }
    }

    fn to_string_protocol(&self) -> String {
        self.protocol.to_string()
    }

    fn to_string_sender_mac_addr(&self) -> String {
        self.sender_mac_addr.to_string()
    }

    fn to_string_target_mac_addr(&self) -> String {
        self.target_mac_addr.to_string()
    }

    fn to_string_sender_ip_addr(&self) -> String {
        self.sender_ip_addr.to_string()
    }

    fn to_string_target_ip_addr(&self) -> String {
        self.target_ip_addr.to_string()
    }

    fn to_string_operation(&self) -> String {
        match self.operation {
            ArpOperations::Reply => String::from("Reply"),
            ArpOperations::Request => String::from("Request"),
            ArpOperation(n) => format!("known operation '{n}'")
        }
    }
}
