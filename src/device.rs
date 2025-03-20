use pnet_datalink::NetworkInterface;

pub fn get_network_interface(name: &str) -> Option<NetworkInterface> {
    for iface in pnet_datalink::interfaces() {
        if iface.name == name {
            return Some(iface)
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::get_network_interface;

    #[test]
    fn get_nic() {
        assert!(get_network_interface("eth0").is_some());
        assert!(get_network_interface("not_exist_nic").is_none());
    }
}