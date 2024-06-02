use crate::storage::errors::{NoDeviceError, NoRoomError, RoomNotAddedError, DeviceNotAddedError, RoomAddWithoutDevices};

#[derive(Debug)]
pub enum DeviceStorageGetInfoErrors {
    NoDevice(NoDeviceError),
    NoRoom(NoRoomError)
}

#[derive(Debug)]
pub enum DeviceStorageAddOrDeleteErrors {
    RoomNotAdd(RoomNotAddedError),
    DeviceNotAdd(DeviceNotAddedError),
    RoomWithoutDevice(RoomAddWithoutDevices)
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

impl From<RoomNotAddedError> for DeviceStorageAddOrDeleteErrors {
    fn from(err: RoomNotAddedError) -> Self {
        DeviceStorageAddOrDeleteErrors::RoomNotAdd(err)
    }
}

impl From<RoomAddWithoutDevices> for DeviceStorageAddOrDeleteErrors {
    fn from(err: RoomAddWithoutDevices) -> Self {
        DeviceStorageAddOrDeleteErrors::RoomWithoutDevice(err)
    }
}

impl From<DeviceNotAddedError> for DeviceStorageAddOrDeleteErrors {
    fn from(err: DeviceNotAddedError) -> Self {
        DeviceStorageAddOrDeleteErrors::DeviceNotAdd(err)
    }
}