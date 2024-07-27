use blinkscan::{self, create_network, get_default_interface};

pub fn scan() -> Vec<HostInfo> {
    let interface = get_default_interface().unwrap();
    let network = create_network(&interface);
    let mut hosts = Vec::new();
    for host in blinkscan::scan_network(network, std::time::Duration::from_secs(5)) {
        hosts.push(HostInfo {
            host: host.host,
            mac: host.mac,
            vendor: host.vendor,
        });
    }
    hosts
}

#[derive(Clone)]
pub struct HostInfo {
    pub host: String,
    pub mac: Option<String>,
    pub vendor: Option<String>,
}
