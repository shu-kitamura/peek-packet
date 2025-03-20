use pnet_datalink::NetworkInterface;

pub fn get_network_interface(name: &str) -> Option<NetworkInterface> {
    for iface in pnet_datalink::interfaces() {
        if iface.name == name {
            return Some(iface)
        }
    }
    None
}