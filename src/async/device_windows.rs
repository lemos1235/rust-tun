use std::io;
use std::net::IpAddr;

use tokio_util::codec::Framed;

use ipnet::IpNet;
use tunio::platform::wintun::AsyncInterface;
use tunio::traits::{DriverT, InterfaceT};
use tunio::{DefaultAsyncInterface, DefaultDriver};

use crate::{Configuration, TunPacketCodec};

pub struct AsyncDevice {
    iface: AsyncInterface,
}

impl AsyncDevice {
    pub fn new(conf: &Configuration) -> io::Result<AsyncDevice> {
        let mut driver = DefaultDriver::new().unwrap();
        let if_config = DefaultAsyncInterface::config_builder()
            .name(conf.name.to_owned().unwrap_or(String::from("iface1")))
            .build()
            .unwrap();

        let iface = match conf.enabled {
            Some(true) => DefaultAsyncInterface::new_up(&mut driver, if_config).unwrap(),
            _ => DefaultAsyncInterface::new(&mut driver, if_config).unwrap(),
        };

        let iff = iface.handle();
        let mtu = match conf.mtu {
            Some(mtu) => mtu as u32,
            _ => 1500,
        };
        iff.set_mtu(mtu).map_err(|_| io::Error::last_os_error())?;
        match (conf.address, conf.netmask) {
            (Some(addr), Some(mask)) => {
                let ipnet = IpNet::with_netmask(IpAddr::from(addr), IpAddr::from(mask)).unwrap();
                iff.add_address(ipnet)
                    .map_err(|_| io::Error::last_os_error())?;
            }
            (Some(addr), _) => {
                let ipnet = IpNet::from(IpAddr::from(addr));
                iff.add_address(ipnet)
                    .map_err(|_| io::Error::last_os_error())?;
            }
            _ => {}
        };
        Ok(AsyncDevice { iface })
    }

    /// Consumes this AsyncDevice and return a Framed object (unified Stream and Sink interface)
    pub fn into_framed(self) -> Framed<AsyncInterface, TunPacketCodec> {
        let codec = TunPacketCodec::new(false, self.iface.handle().mtu().unwrap_or(1500) as i32);
        Framed::new(self.iface, codec)
    }
}
