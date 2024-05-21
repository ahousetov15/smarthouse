pub mod errors;
pub mod enums;
pub mod device_storage {
    // use crate::log::{debug, error};
    use crate::log::{debug};
    use crate::smarthouse::traits::device_interface::DeviceInterface;
    use crate::storage::enums::DeviceStorageErrors;
    use crate::storage::errors::{NoDeviceError, NoRoomError};
    use std::collections::HashMap;

    /// Хранилище устройств
    //pub struct DeviceStorage<'dev_strg_time> {
    pub struct DeviceStorage {
        pub room_map: HashMap<String, Vec<Box<dyn DeviceInterface>>>,
    }

    //impl DeviceStorage<'dev_strg_time> {
    impl DeviceStorage {
        pub fn new(param: DeviceStorage) -> Self {
            Self {
                room_map: param.room_map,
            }
        }

        // pub fn get_device_report(&self, room_name: &str, device_name: &str) -> Option<String> {
        pub fn get_device_report(&self, room_name: &str, device_name: &str) -> Result<String, DeviceStorageErrors> {
            match self.room_map.get(room_name) {
                Some(device_vec) => {
                    debug!("Пробую найти устройство: {:?}", device_name);
                    let need_device = device_vec.iter().find(|&x| *x.get_name() == *device_name);
                    match need_device {
                        Some(device) => Ok(device.report()),
                        _ => Err(DeviceStorageErrors::NoDevice(NoDeviceError {
                                room_name: room_name.to_string(),
                                device_name: device_name.to_string(),
                            }))
                    }
                }
                _ => {
                    Err(DeviceStorageErrors::NoRoom(NoRoomError {
                        room_name: room_name.to_string(),
                    }))
                    // error!("Такой комнаты в доме нет.");
                    // None
                }
            }
        }

        // pub fn add_device<T: DeviceInterface>(&mut self, room_name: String, device: T) {
        //     //let new_device: &dyn DeviceInterface = device;
        //     //self.room_map.insert(room_name, device);
        //     match self.room_map.get_mut(&room_name) {
        //         Some(device_vec) => device_vec.push(Box::new(device)),
        //         _ => {
        //             let mut new_devices_vec: Vec<Box<dyn DeviceInterface>> = Vec::new();
        //             new_devices_vec.push(Box::new(device));
        //             self.room_map.insert(room_name, new_devices_vec);
        //         }
        //     }
        // }
    }
}
