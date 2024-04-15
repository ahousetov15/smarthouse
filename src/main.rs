// src/main.rs

use std::collections::{HashMap, HashSet};

///Состояния нашей розетки
#[derive(Clone)]
enum SocketState {
    IsOn,  //Включено
    IsOff, //Выключено
}

#[derive(Clone)]
struct Socket {
    name: String,       //Имя девайса
    power: f32,         //Величина в ваттах
    state: SocketState, //Состояние розетки (Вкл./Выкл.)
}

impl Socket {
    fn new(params: Socket) -> Self {
        Self {
            name: params.name,
            power: params.power,
            state: params.state,
        }
    }
}

#[derive(Clone)]
struct Thermometer {
    name: String,     // Имя девайса
    temperature: i32, // Температура в градусах цельсия
}

impl Thermometer {
    fn new(param: Thermometer) -> Self {
        Self {
            name: param.name,
            temperature: param.temperature,
        }
    }
}

// /// Для хранения разных типов устройств в одном контейнере,
// /// создаем некий тип-перечисление типов.
// #[derive(Clone)]
// enum Device {
//     Socket(Socket),
//     Thermometer(Thermometer),
// }

/// Команата
#[derive(Debug, Clone)]
struct Room {
    name: String,
    devices: HashSet<String>,
}

impl Room {
    fn new(params: Room) -> Self{
        Self {
            name: params.name,
            devices: params.devices,
        }
    }
}

trait DeviceInterface {
    fn name(&self) -> &str;
    fn get(&self) -> String; //Получить список оборудования/помещений в комнате/доме
    fn interact(&mut self); //Повернуть выключатель
    fn report(&self) -> String;
}

impl DeviceInterface for Socket {
    fn name(&self) -> &str {
        println!("{:?} розетка", self.name.as_str());
        self.name.as_str()
    }

    /// Повернуть выключатель (во Вкл. или Выкл.)
    fn interact(&mut self) {
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

    /// Получить потребляемую мощность
    fn get(&self) -> String {
        let power_str = self.power.to_string();
        power_str
    }

    /// Получить отчет о потреблемой мощности
    fn report(&self) -> String {
        format!(" - Розетка: '{}' на {} ватт\n", self.name, self.power)
    }
}

impl DeviceInterface for Thermometer {
    ///Получить текущую температуру
    fn get(&self) -> String {
        let temp_str = self.temperature.to_string();
        temp_str
    }

    fn name(&self) -> &str {
        println!("{:?} термометр", self.name.as_str());
        self.name.as_str()
    }

    fn interact(&mut self) {
        println!("SmarthouseInterface<Thermometer>::interact был вызыван");
    }

    /// Получить отчет о термометре и температура
    fn report(&self) -> String {
        format!(
            " - Термометр: '{}' с показанием +{} градусов по цельсию\n",
            self.name, self.temperature
        )
    }
}

/// Наш умный дом
struct Smarthouse {
    name: String, // Имя дома
    rooms: HashMap<String, Room>,
}

impl Smarthouse {
    fn new(name: &str, device_storage: &DeviceStorage) {
        let rooms_map = HashMap::<String, Room>;
        for (room_name, devices) in device_storage.room_map.iter() {
            let mut room = Room::new(Room{name: *room_name, decives: HashSet<String>});
        }
    }
}

/// Хранилище устройств
struct DeviceStorage {
    room_map: HashMap<String, Vec<Box<dyn DeviceInterface>>>,
}

impl DeviceStorage {
    fn new(param: DeviceStorage) -> Self {
        Self {
            room_map: param.room_map,
        }
    }

    fn get_device_report(&self, room: &str, name: &str) -> Option<String> {
        match self.room_map.get(room) {
            Some(device_vec) => {
                println!("Try to find: {}", name);
                let need_device = device_vec.iter().find(|&x| *x.name() == *name);
                match need_device {
                    Some(device) => Some(device.report()),
                    _ => {
                        println!("По имени {} ничего не найдено", name);
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

trait SmarthouseInterface {
    //fn get_device_info(&self, room: &str, name: &str) -> Option<String>; // Получаем информацию о том, ГДЕ ИМЕННО В ДОМЕ находится девайс
    fn get_device_info(&self, room: &str) -> Option<String>; // Получаем информацию о том, ГДЕ ИМЕННО В ДОМЕ находится девайс
}

impl SmarthouseInterface for Smarthouse {
    //fn get_device_info(&self, room: &str, name: &str) -> Option<String> {
    fn get_device_info(&self, room: &str) -> Option<String> {
        match self.rooms.get(room) {
            Some(ref room_struct) => {
                println!(
                    "В данной комнате вот такой набор оборудования: {:#?}",
                    room_struct
                );
                Some(format!("{:#?}", room_struct))
            }
            _ => {
                println!("Такой комнаты в доме нет.");
                None
            }
        }
    }
}

// ///Интерфейс для умного дома
// trait SmarthouseInterface<T> {
//     type GetResult;

//     // fn new(params: T) -> T
//     // where
//     //     Self: Sized;
//     fn name(&self) -> &str;
//     fn get(&self) -> Self::GetResult; //Получить список оборудования/помещений в комнате/доме
//                                       //fn report(&self) -> String; // Получить отчет об оборудовании в помещении/список комнат и оборудования в них
//     fn interact(&mut self); //Повернуть выключатель
// }

// ///Интерфейс для розетки
// impl SmarthouseInterface<Socket> for Socket {
//     type GetResult = Vec<f32>;

// // fn new(params: Socket) -> Self {
// //     Self {
// //         name: params.name,
// //         power: params.power,
// //         state: params.state,
// //     }
// // }

// fn name(&self) -> &str {
//     println!("{:?} розетка", self.name.as_str());
//     self.name.as_str()
// }

// /// Повернуть выключатель (во Вкл. или Выкл.)
// fn interact(&mut self) {
//     self.state = match self.state {
//         SocketState::IsOn => {
//             println!("Розетка выключена.");
//             SocketState::IsOff
//         }
//         SocketState::IsOff => {
//             println!("Розетка включена.");
//             SocketState::IsOn
//         }
//     }
// }

// /// Получить потребляемую мощность
// fn get(&self) -> Self::GetResult {
//     let power_vec = vec![self.power];
//     power_vec
// }

// // /// Получить отчет о потреблемой мощности
// // fn report(&self) -> String {
// //     format!(" - Розетка: '{}' на {} ватт\n", self.name, self.power)
// // }
// }

// ///Интерфейс для термометра
// impl SmarthouseInterface<Thermometer> for Thermometer {
//     type GetResult = Vec<i32>;

//     // fn new(param: Thermometer) -> Self {
//     //     Self {
//     //         name: param.name,
//     //         temperature: param.temperature,
//     //     }
//     // }

//     ///Получить текущую температуру
//     fn get(&self) -> Self::GetResult {
//         let temperature_vec = vec![self.temperature];
//         temperature_vec
//     }

//     fn name(&self) -> &str {
//         println!("{:?} термометр", self.name.as_str());
//         self.name.as_str()
//     }

//     fn interact(&mut self) {
//         println!("SmarthouseInterface<Thermometer>::interact был вызыван");
//     }

//     // /// Получить отчет о термометре и температура
//     // fn report(&self) -> String {
//     //     format!(
//     //         " - Термометр: '{}' с показанием +{} градусов по цельсию\n",
//     //         self.name, self.temperature
//     //     )
//     // }
// }

// impl SmarthouseInterface<Room> for Room {
//     type GetResult = Vec<Device>;

//     // fn new(param: Room) -> Self {
//     //     Self {
//     //         name: param.name,
//     //         decives: param.decives,
//     //     }
//     // }

//     fn interact(&mut self) {
//         println!("SmarthouseInterface<Room>::interact был вызван");
//     }

//     fn name(&self) -> &str {
//         println!("{:?} комната", self.name.as_str());
//         self.name.as_str()
//     }

//     // fn get(&self) -> Self::GetResult {
//     //     self.decives.clone()
//     // }

//     // fn report(&self) -> String {
//     //     let mut output = format!("**Комната:{}**\n", self.name);
//     //     if !self.decives.is_empty() {
//     //         output += "**Устройсва:**\n";
//     //         for d in &self.decives {
//     //             match d {
//     //                 Device::Socket(socket) => {
//     //                     output += &socket.report();
//     //                     //output += format!(" - Розетка: {}\n", socket.name).as_str();
//     //                 }
//     //                 Device::Thermometer(thermometer) => {
//     //                     output += &thermometer.report();
//     //                     //output += format!(" - Термометр: {}\n", thermometer.name).as_str();
//     //                 }
//     //             }
//     //         }
//     //     } else {
//     //         output += "**В комнате не обнаружено устройств.**\n\n"
//     //     }
//     //     output += "****\n\n";
//     //     output
//     // }
// }

// impl SmarthouseInterface<Smarthouse> for Smarthouse {
//     type GetResult = Vec<Room>;

//     // fn new(param: Smarthouse) -> Self {
//     //     Self {
//     //         name: param.name,
//     //         rooms: param.rooms,
//     //     }
//     // }

//     fn name(&self) -> &str {
//         println!("{:?} house", self.name.as_str());
//         self.name.as_str()
//     }

//     fn interact(&mut self) {
//         println!("SmarthouseInterface<Smarthouse>::interact был вызван");
//     }

//     // fn report(&self) -> String {
//     //     let mut output = format!("**Дом:{}**\n", self.name);
//     //     if !self.rooms.is_empty() {
//     //         output += "**Комнаты:**\n";
//     //         for room in &self.rooms {
//     //             output += &room.report();
//     //         }
//     //     } else {
//     //         output += "**В доме не обнаружено комнат.**\n\n"
//     //     }
//     //     output += "****\n\n";
//     //     output
//     // }

//     // fn get(&self) -> Self::GetResult {
//     //     self.rooms.clone()
//     // }
// }

fn main() {
    // Создаем набор устройств для спальни
    let mut bedroom_device: Vec<Box<dyn DeviceInterface>> = vec![
        Box::new(Socket::new(Socket {
            name: "Розетка в спальне".to_string(),
            power: 220.0,
            state: SocketState::IsOff,
        })),
        Box::new(Socket::new(Socket {
            name: "Розетка у ванны в спальне".to_string(),
            power: 210.0,
            state: SocketState::IsOn,
        })),
        Box::new(Thermometer::new(Thermometer {
            name: "Термометр в спальне".to_string(),
            temperature: 22,
        })),
    ];
    let mut kitche_device: Vec<Box<dyn DeviceInterface>> = vec![
        Box::new(Socket::new(Socket {
            name: "Розетка над столом кухни".to_string(),
            power: 220.0,
            state: SocketState::IsOff,
        })),
        Box::new(Socket::new(Socket {
            name: "Розетка у плиты".to_string(),
            power: 210.0,
            state: SocketState::IsOn,
        })),
    ];
    let mut storage: DeviceStorage = DeviceStorage::new(DeviceStorage {
        room_map: HashMap::new(),
    });
    storage
        .room_map
        .insert("Bedroom".to_string(), bedroom_device);
    storage
        .room_map
        .insert("Kitchen".to_string(), kitche_device);

    // let mut room_1_sockets: Vec = vec![
    //     Socket {
    //         name: "Розетка_у_ванной".to_string(),
    //         power: 220.1,
    //         state: SocketState::IsOn,
    //     },
    //     Socket {
    //         name: "Розетка_у_кровати".to_string(),
    //         power: 220.0,
    //         state: SocketState::IsOff,
    //     },
    // ];
    // let bedroom_device: Vec<dyn SmarthouseInterface> = Vec::new();

    // let bedroom_devices: Vec<dyn SmarthouseInterface<T>> = vec![Socket::new(Socket {
    //     name: "Розетка_у_ванной".to_string(),
    //     power: 220.1,
    //     state: SocketState::IsOn,
    // })];

    // bedroom_sockets.push(Socket::new(Socket {
    //     name: "Розетка_у_ванной".to_string(),
    //     power: 220.1,
    //     state: SocketState::IsOn,
    // }))
    // // Добавляем розетки
    // bedroom_sockets.push(Socket::new(Socket {
    //     name: "Розетка 1".to_string(),
    //     power: 100.0,
    //     state: SocketState::IsOn,
    // }));
    // bedroom_sockets.push(Socket::new(Socket {
    //     name: "Розетка 2".to_string(),
    //     power: 200.0,
    //     state: SocketState::IsOff,
    // }));

    // let bedroow_sockets: Vec<dyn SmarthouseInterface<Socket>> = vec![
    //     Socket::new(Socket {
    //         name: "Розетка_у_кровати_спальня".to_string(),
    //         power: 220.0,
    //         state: SocketState::IsOff,
    //     }),
    //     Socket::new(Socket {
    //         name: "Розетка_у_окна_спальня".to_string(),
    //         power: 220.0,
    //         state: SocketState::IsOn,
    //     }),
    // ];
}
