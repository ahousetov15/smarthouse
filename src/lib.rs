// src/lib.rs
pub use std::collections::{HashMap, HashSet};
mod smarthouse;
mod storage;
pub use smarthouse::enums::SocketState;
pub use smarthouse::devices::{Socket, Thermometer};
pub use smarthouse::traits::device_interface::DeviceInterface;
pub use smarthouse::Smarthouse;
pub use smarthouse::traits::device_interface::SmarthouseInterface;
pub use storage::device_storage::DeviceStorage;


// fn main() {
//     // Создаем набор устройств для спальни
//     let bedroom_device: Vec<Box<dyn DeviceInterface>> = vec![
//         Box::new(Socket::new(Socket {
//             name: "Розетка в спальне".to_string(),
//             power: 220.0,
//             state: SocketState::IsOff,
//         })),
//         Box::new(Socket::new(Socket {
//             name: "Розетка у ванны в спальне".to_string(),
//             power: 210.0,
//             state: SocketState::IsOn,
//         })),
//         Box::new(Thermometer::new(Thermometer {
//             name: "Термометр в спальне".to_string(),
//             temperature: 22,
//         })),
//     ];
//     let kitche_device: Vec<Box<dyn DeviceInterface>> = vec![
//         Box::new(Socket::new(Socket {
//             name: "Розетка над столом кухни".to_string(),
//             power: 220.0,
//             state: SocketState::IsOff,
//         })),
//         Box::new(Socket::new(Socket {
//             name: "Розетка у плиты".to_string(),
//             power: 210.0,
//             state: SocketState::IsOn,
//         })),
//     ];
//     let mut storage: DeviceStorage = DeviceStorage::new(DeviceStorage {
//         room_map: HashMap::new(),
//     });
//     storage
//         .room_map
//         .insert("Bedroom".to_string(), bedroom_device);
//     storage
//         .room_map
//         .insert("Kitchen".to_string(), kitche_device);
//     let smarthouse = Smarthouse::new("Домашная работа 3", &storage);
//     // let report = smarthouse.get_device_info("Bedroom", "Розетка у ванны в спальне", &storage);
//     // println!("Отчет: {:#?}", report);
//     // let report = smarthouse.get_device_info("Bedroom", "Розетка в спальне", &storage);
//     // println!("Отчет: {:#?}", report);
//     // let report = smarthouse.get_device_info("Bedroom", "Термометр в спальне", &storage);
//     // println!("Отчет: {:#?}", report);
//     // let report = smarthouse.get_device_info("Kitchen", "Розетка над столом кухни", &storage);
//     // println!("Отчет: {:#?}", report);
//     // let report = smarthouse.get_device_info("Kitchen", "Розетка у плиты", &storage);
//     // println!("Отчет: {:#?}", report);
//     // print!("Отчет: {}", smarthouse.get_roooms_list());
//     // print!("Отчет: {}", smarthouse.get_rooms_devices_list("Kitchen"));
//     // print!("Отчет: {}", smarthouse.get_rooms_devices_list("Bedroom"));
//     println!("{}", smarthouse.full_report(&storage));
// }
