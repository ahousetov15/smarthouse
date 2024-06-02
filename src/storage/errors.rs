use std::fmt::{self};

#[derive(Debug)]
pub struct NoDeviceError {
    pub room_name: String,
    pub device_name: String,
}

#[derive(Debug)]
pub struct NoRoomError {
    pub room_name: String,
}

#[derive(Debug)]
pub struct RoomNotAddedError {
    pub room_name: String,
}

#[derive(Debug)]
pub struct DeviceNotAddedError {
    pub room_name: String,
    pub device_name: String,
}


#[derive(Debug)]
pub struct RoomAddWithoutDevices {
    pub room_name: String
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

impl fmt::Display for RoomNotAddedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Комната '{}' не добавлена", self.room_name)
    }
}

impl fmt::Display for DeviceNotAddedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Устройство '{}' в комнате '{}' не добавлено", self.device_name, self.room_name)
    }
}

impl fmt::Display for RoomAddWithoutDevices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Комната '{}' добавлена. Список устройств пуст.", self.room_name)
    }
}



enum DeviceStorageGetInfoErrors {
    NoDevice(NoDeviceError),
    NoRoom(NoRoomError)
}

impl From<NoDeviceError> for DeviceStorageGetInfoErrors {
    fn from(err: NoDeviceError) -> Self {
        DeviceStorageGetInfoErrors::NoDevice(err)
    }
}

impl From<NoRoomError> for DeviceStorageGetInfoErrors {
    fn from(err: NoRoomError) -> Self {
        DeviceStorageGetInfoErrors::NoRoom(err)
    }
}

enum DeviceStorageAddOrDeleteErrors {
    RoomNotAdd(RoomNotAddedError),
    DeviceNotAdd(DeviceNotAddedError),
    RoomWithoutDevice(RoomAddWithoutDevices)
}

impl From<RoomNotAddedError> for DeviceStorageAddOrDeleteErrors {
    fn from(err: RoomNotAddedError) -> Self {
        DeviceStorageAddOrDeleteErrors::RoomNotAdd(err)
    }
}


impl fmt::Display for DeviceStorageAddOrDeleteErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceStorageAddOrDeleteErrors::RoomNotAdd(error) => write!(f, "{}", error),
            DeviceStorageAddOrDeleteErrors::DeviceNotAdd(error) => write!(f, "{}", error),
            DeviceStorageAddOrDeleteErrors::RoomWithoutDevice(error) => write!(f, "{}", error),
        }
    }
}