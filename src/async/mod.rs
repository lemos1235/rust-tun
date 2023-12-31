//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

//! Async specific modules.

use crate::error;

use crate::configuration::Configuration;
#[cfg(not(target_os = "windows"))]
use crate::platform::create;

#[cfg(not(target_os = "windows"))]
mod device;
#[cfg(not(target_os = "windows"))]
pub use self::device::{AsyncDevice, AsyncQueue};

mod codec;
pub use self::codec::{TunPacket, TunPacketCodec};

/// Create a TUN device with the given name.
#[cfg(not(target_os = "windows"))]
pub fn create_as_async(configuration: &Configuration) -> Result<AsyncDevice, error::Error> {
    let device = create(&configuration)?;
    AsyncDevice::new(device).map_err(|err| err.into())
}

#[cfg(target_os = "windows")]
mod device_windows;
#[cfg(target_os = "windows")]
pub use device_windows::AsyncDevice;

#[cfg(target_os = "windows")]
pub fn create_as_async(configuration: &Configuration) -> Result<AsyncDevice, error::Error> {
    AsyncDevice::new(configuration).map_err(|err| err.into())
}
