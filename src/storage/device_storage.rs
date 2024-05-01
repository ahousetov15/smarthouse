pub mod device_storage {
    use crate::smarthouse::traits::device_interface::DeviceInterface;
    use std::collections::HashMap;

    /// Хранилище устройств
    pub struct DeviceStorage {
        pub room_map: HashMap<String, Vec<Box<dyn DeviceInterface>>>,
    }

    impl DeviceStorage {
        pub fn new(param: DeviceStorage) -> Self {
            Self {
                room_map: param.room_map,
            }
        }

        pub fn get_device_report(&self, room_name: &str, device_name: &str) -> Option<String> {
            match self.room_map.get(room_name) {
                Some(device_vec) => {
                    println!("Пробую найти устройство: '{}'", device_name);
                    let need_device = device_vec.iter().find(|&x| *x.get_name() == *device_name);
                    match need_device {
                        Some(device) => Some(device.report()),
                        _ => {
                            println!("По имени {device_name} устройств не найдено");
                            None
                        }
                    }
                }
                _ => {
                    println!("Такой комнаты в доме нет.");
                    None
                }
            }
        }
    }   
}
