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

#[cfg(all(
    feature = "async",
    any(
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "windows",
    )
))]
pub use r#async::*;

pub use crate::address::IntoAddress;
pub use crate::configuration::{Configuration, Layer};
pub use crate::device::Device;
pub use crate::error::*;
pub use crate::platform::create;

mod address;
#[cfg(all(
    feature = "async",
    any(
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "android",
        target_os = "windows",
    )
))]
pub mod r#async;
mod configuration;
mod device;
mod error;
pub mod platform;

pub fn configure() -> Configuration {
    Configuration::default()
}
