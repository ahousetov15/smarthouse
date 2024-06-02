pub mod enums;
pub mod errors;
pub mod device_storage {
    use crate::log::debug;
    use crate::smarthouse::traits::device_interface::DeviceInterface;
    use crate::storage::enums::DeviceStorageAddOrDeleteErrors;
    use crate::storage::enums::DeviceStorageGetInfoErrors;
    use crate::storage::errors::{NoDeviceError, NoRoomError};
    use std::collections::HashMap;
    use std::sync::Arc;

    use super::errors::RoomAddWithoutDevices;
    use super::errors::RoomNotAddedError;

    /// Хранилище устройств
    //pub struct DeviceStorage<'dev_strg_time> {
    pub struct DeviceStorage {
        pub room_map: HashMap<String, Vec<Arc<dyn DeviceInterface>>>,
    }

    impl DeviceStorage {
        pub fn new(param: DeviceStorage) -> Self {
            Self {
                room_map: param.room_map,
            }
        }

        // pub fn get_device_report(&self, room_name: &str, device_name: &str) -> Option<String> {
        pub fn get_device_report(
            &self,
            room_name: &str,
            device_name: &str,
        ) -> Result<String, DeviceStorageGetInfoErrors> {
            match self.room_map.get(room_name) {
                Some(device_vec) => {
                    debug!("Пробую найти устройство: {:?}", device_name);
                    let need_device = device_vec.iter().find(|&x| *x.get_name() == *device_name);
                    match need_device {
                        Some(device) => Ok(device.report()),
                        _ => Err(DeviceStorageGetInfoErrors::NoDevice(NoDeviceError {
                            room_name: room_name.to_string(),
                            device_name: device_name.to_string(),
                        })),
                    }
                }
                _ => {
                    Err(DeviceStorageGetInfoErrors::NoRoom(NoRoomError {
                        room_name: room_name.to_string(),
                    }))
                    // error!("Такой комнаты в доме нет.");
                    // None
                }
            }
        }

        pub fn add_to_storage(
            &mut self,
            room_name: &str,
            device: Option<Arc<dyn DeviceInterface>>,
        ) -> Result<String, DeviceStorageAddOrDeleteErrors> {
            let room_name_copy = room_name.to_string().clone();
            match self.room_map.get_mut(&room_name_copy) {
                Some(device_vec) => {
                    if let Some(device) = device {
                        device_vec.push(device);
                        Ok(format!(
                            "В комнату {} добавлен устройство: {}",
                            room_name,
                            device_vec.last().unwrap().get_name()
                        ))
                    } else {
                        Err(DeviceStorageAddOrDeleteErrors::RoomNotAdd(
                            RoomNotAddedError {
                                room_name: room_name.to_string(),
                            },
                        ))
                    }
                }
                _ => match device {
                    Some(device) => {
                        let new_devices_vec: Vec<Arc<dyn DeviceInterface>> = vec![device];
                        let room_name_clone = room_name_copy.to_string();
                        self.room_map.insert(room_name_copy, new_devices_vec);
                        let device_name = self
                            .room_map
                            .get(room_name)
                            .unwrap()
                            .last()
                            .unwrap()
                            .get_name();
                        Ok(format!(
                            "Добавлена новая комната {} с устройством: {}",
                            room_name_clone, device_name
                        ))
                    }
                    _ => {
                        self.room_map.insert(room_name_copy, vec![]);
                        Err(DeviceStorageAddOrDeleteErrors::RoomWithoutDevice(RoomAddWithoutDevices{
                            room_name: room_name.to_string(),}))
                    }
                },
            }
        }

        pub fn remove_from_storage(
            &mut self,
            room_name: &str,
            device_name: Option<&str>,
        ) -> Result<String, DeviceStorageGetInfoErrors> {
            let room_name_copy = room_name.to_string().clone();
            match self.room_map.get_mut(&room_name_copy) {
                Some(device_vec) => {
                    if let Some(device_name_result) = device_name {
                        // if device_vec
                        //     .iter()
                        //     .find(|&x| x.get_name() == device_name_result)
                        //     .is_some()
                        if device_vec
                            .iter()
                            .any(|x| x.get_name() == device_name_result)              
                        {
                            device_vec.retain(|x| x.get_name() == device_name_result);
                            Ok(format!(
                                "Удалено устройство {} из комнаты {}",
                                device_name_result, room_name
                            ))
                        } else {
                            Err(DeviceStorageGetInfoErrors::NoDevice(NoDeviceError {
                                room_name: room_name.to_string(),
                                device_name: device_name_result.to_string(),
                            }))
                        }
                    } else {
                        self.room_map.remove(room_name);
                        Ok(format!("Удалена комната {}", room_name))
                    }
                }
                _ => Err(DeviceStorageGetInfoErrors::NoRoom(NoRoomError {
                    room_name: room_name.to_string(),
                })),
            }
        }
    }
}
