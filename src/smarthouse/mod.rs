pub mod enums;
pub mod traits;

use crate::smarthouse::rooms::Room;
use crate::storage::device_storage::DeviceStorage;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::log::{debug};


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
                devices: HashSet::new(),
            });
            for device in devices.iter() {
                room.devices.insert(device.get_name().to_string());
            }
            debug!("Сформировали новую комнату: {:#?}", room);
            rooms_map.insert(room_name.to_string(), room);
        }
        Self {
            name: name.to_string(),
            rooms: rooms_map,
        }
    }
}

pub mod rooms {
    use std::collections::HashSet;

    /// Команата
    #[derive(Debug, Clone)]
    pub struct Room {
        pub name: String,
        pub devices: HashSet<String>,
    }

    impl Room {
        pub fn new(params: Room) -> Self {
            Self {
                name: params.name,
                devices: params.devices,
            }
        }
    }
}

pub mod devices{
    use crate::smarthouse::enums::SocketState;

    #[derive(Clone)]
    pub struct Socket {
        pub name: String,       //Имя девайса
        pub power: f32,         //Величина в ваттах
        pub state: SocketState, //Состояние розетки (Вкл./Выкл.)
    }

    impl Socket {
        pub fn new(params: Socket) -> Self {
            Self {
                name: params.name,
                power: params.power,
                state: params.state,
            }
        }


        /// Повернуть выключатель (во Вкл. или Выкл.)
        pub fn interact(&mut self){
            self.state = match self.state {
                SocketState::IsOn => {
                    println!("Розетка выключена.");
                    SocketState::IsOff
                }
                SocketState::IsOff => {
                    println!("Розетка включена.");
                    SocketState::IsOn
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct Thermometer {
        pub name: String,     // Имя девайса
        pub temperature: i32, // Температура в градусах цельсия
    }

    impl Thermometer {
        pub fn new(param: Thermometer) -> Self {
            Self {
                name: param.name,
                temperature: param.temperature,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::smarthouse::enums::SocketState;
    use crate::smarthouse::devices::{Socket, Thermometer};
    extern crate pretty_env_logger;
    // #[macro_use] extern crate log;

    fn init() {
        let _ = pretty_env_logger::init_timed();
    }

    #[test]
    fn test_socket() {
        init();
        let mut socket = Socket::new(Socket {
            name: "Розетка".to_string(),
            power: 220.0,
            state: SocketState::IsOff
        });
        assert_eq!(socket.state, SocketState::IsOff);
        socket.interact();
        assert_eq!(socket.state, SocketState::IsOn);
        assert_eq!(socket.name, "Розетка".to_string());
        assert_eq!(socket.power, 220.0);
        socket.power = 210.0;
        assert_eq!(socket.power, 210.0);
    }

    #[test]
    fn test_thermometer() {
        //init();
        let mut therm: Thermometer = Thermometer::new(Thermometer {
            name: "Термометр".to_string(),
            temperature: 22
        });
        assert_eq!(therm.temperature, 22);
        therm.temperature = 23;
        assert_eq!(therm.temperature, 23);
        let new_name = String::from("ЯДрачистыйИзумруд"); 
        therm.name = "ЯДрачистыйИзумруд".to_string();
        assert_eq!(therm.name, new_name);
    }
}