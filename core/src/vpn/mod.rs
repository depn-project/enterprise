use std::io::Error;
use std::net::IpAddr;
use std::process::{Command, Output};

pub struct VPN {
    name: String,
}

impl VPN {
    pub fn new(&self, name: String) -> Self {
        VPN { name }
    }

    pub fn connect() -> Result<Output, Error> {
        Command::new("cmd")
            .args(["/C", "wireguard.exe /installtunnelservice C:\\Users\\Улад\\code\\depn\\wg-core.conf"])
            .output()
    }

    pub fn disconnect() -> Result<Output, Error> {
        Command::new("cmd")
            .args(["/C", "wireguard.exe /uninstalltunnelservice wg-core"])
            .output()
    }
}
