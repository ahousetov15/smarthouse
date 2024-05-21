use std::fmt::{self};

pub struct NoDeviceError {
    pub room_name: String,
    pub device_name: String,
}

#[derive(Debug)]
pub struct NoRoomError {
    pub room_name: String,
}

impl fmt::Display for NoDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "'{}' в комнате '{}' не найдено", self.device_name, self.room_name)
    }
}

impl fmt::Display for NoRoomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Комната '{}' не найдена", self.room_name)
    }
}


/// Ручная реализация #[derive(Debug)]
impl fmt::Debug for NoDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NoDeviceError")
            .field("room_name", &self.room_name)
            .field("device_name", &self.device_name)
            .finish()
    }
}

enum DeviceStorageErrors {
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