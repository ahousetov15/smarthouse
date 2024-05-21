use crate::storage::errors::{NoDeviceError, NoRoomError};

pub enum DeviceStorageErrors {
    NoDevice(NoDeviceError),
    NoRoom(NoRoomError),
}

impl From<NoDeviceError> for DeviceStorageErrors {
    fn from(err: NoDeviceError) -> Self {
        DeviceStorageErrors::NoDevice(err)
    }
}

impl From<NoRoomError> for DeviceStorageErrors {
    fn from(err: NoRoomError) -> Self {
        DeviceStorageErrors::NoRoom(err)
    }
}