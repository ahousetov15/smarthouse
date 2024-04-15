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
    fn new(params: Room) -> Self {
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
    fn new(name: &str, device_storage: &DeviceStorage) -> Self {
        let mut rooms_map = HashMap::<String, Room>::new();
        for (room_name, devices) in device_storage.room_map.iter() {
            let mut room = Room::new(Room {
                name: room_name.to_string(),
                devices: HashSet::<String>::new(),
            });
            for device in devices.iter() {
                room.devices.insert(device.name().to_string());
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

    fn get_device_report(&self, room_name: &str, device_name: &str) -> Option<String> {
        match self.room_map.get(room_name) {
            Some(device_vec) => {
                println!("Пробую найти устройство: '{}'", device_name);
                let need_device = device_vec.iter().find(|&x| *x.name() == *device_name);
                match need_device {
                    Some(device) => Some(device.report()),
                    _ => {
                        println!("По имени {} устройств не найдено", device_name);
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
    fn get_device_info(
        &self,
        room_name: &str,
        device_name: &str,
        storage: &DeviceStorage,
    ) -> Option<String>; // Получаем информацию о том, ГДЕ ИМЕННО В ДОМЕ находится девайс
}

impl SmarthouseInterface for Smarthouse {
    fn get_device_info(
        &self,
        room_name: &str,
        device_name: &str,
        storage: &DeviceStorage,
    ) -> Option<String> {
        println!("Данные по дому: {:#?}", self.name.to_string());
        match self.rooms.get(room_name) {
            Some(room_struct) => {
                println!("Обнаружена комната: {:#?}", room_struct);
                if room_struct.devices.contains(device_name) {
                    storage.get_device_report(room_name, device_name)
                } else {
                    println!("Такого устройства: '{:?}' в доме нет.", device_name);
                    None
                }
            }
            _ => {
                println!("Такой комнаты: '{:?}' в доме нет.", room_name);
                None
            }
        }
    }
}

fn main() {
    // Создаем набор устройств для спальни
    let bedroom_device: Vec<Box<dyn DeviceInterface>> = vec![
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
    let kitche_device: Vec<Box<dyn DeviceInterface>> = vec![
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
    let smarthouse = Smarthouse::new("Домашная работа 3", &storage);
    let report = smarthouse.get_device_info("Bedroom", "Розетка у ванны в спальне", &storage);
    println!("Отчет: {:#?}", report);
    let report = smarthouse.get_device_info("Bedroom", "Розетка в спальне", &storage);
    println!("Отчет: {:#?}", report);
    let report = smarthouse.get_device_info("Bedroom", "Термометр в спальне", &storage);
    println!("Отчет: {:#?}", report);
    let report = smarthouse.get_device_info("Kitchen", "Розетка над столом кухни", &storage);
    println!("Отчет: {:#?}", report);
    let report = smarthouse.get_device_info("Kitchen", "Розетка у плиты", &storage);
    println!("Отчет: {:#?}", report);
}
