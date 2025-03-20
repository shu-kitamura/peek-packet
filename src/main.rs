mod device;

fn main() {
    if let Some(nic) = device::get_network_interface("eth0") {
         println!("{:?}", nic);
    };
}
