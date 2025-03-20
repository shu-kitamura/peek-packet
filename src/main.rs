mod device;
mod handler;

use handler::handle_ethernet_frame;
use pnet_datalink::{
    NetworkInterface,
    Channel::Ethernet,
};

fn main() {
    let nic: NetworkInterface  = match device::get_network_interface("eth0") {
        Some(iface) => iface,
        None => return
    };

    let (_, mut rx) = match pnet_datalink::channel(&nic, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type."),
        Err(e) => panic!("Failed to create channel. {}", e),
    };

    loop {
        match rx.next() {
            Ok(bytes) => handle_ethernet_frame(bytes),
            Err(e) => panic!("{e}")
        }
    }
}
