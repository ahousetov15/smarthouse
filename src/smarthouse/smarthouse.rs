pub mod smarthouse {
    use crate::smarthouse::rooms::rooms::Room;
    use crate::storage::device_storage::device_storage::DeviceStorage;
    use std::collections::HashMap;
    use std::collections::HashSet;


    /// Наш умный дом
    pub struct Smarthouse {
        pub name: String, // Имя дома
        pub rooms: HashMap<String, Room>,
    }

    impl Smarthouse {
        pub fn new(name: &str, device_storage: &DeviceStorage) -> Self {
            let mut rooms_map = HashMap::<String, Room>::new();
            for (room_name, devices) in device_storage.room_map.iter() {
                let mut room = Room::new(Room {
                    name: room_name.to_string(),
                    devices: HashSet::<String>::new(),
                });
                for device in devices.iter() {
                    room.devices.insert(device.get_name().to_string());
                }
                println!("Сформировали новую комнату: {:#?}", room);
                rooms_map.insert(room_name.to_string(), room);
            }
            Self {
                name: name.to_string(),
                rooms: rooms_map,
            }
        }
    }
}