// src/lib.rs
pub use std::collections::{HashMap, HashSet};
pub extern crate pretty_env_logger;
#[macro_use] extern crate log;
mod smarthouse;
mod storage;
pub use smarthouse::enums::SocketState;
pub use smarthouse::devices::{Socket, Thermometer};
pub use smarthouse::traits::device_interface::DeviceInterface;
pub use smarthouse::Smarthouse;
pub use smarthouse::traits::device_interface::SmarthouseInterface;
pub use storage::device_storage::DeviceStorage;