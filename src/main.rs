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
        //println!("{:?} розетка", self.name.as_str());
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
        self.power.to_string()
    }

    /// Получить отчет о потреблемой мощности
    fn report(&self) -> String {
        format!(" - Розетка: '{}' на {} ватт\n", self.name, self.power)
    }
}

impl DeviceInterface for Thermometer {
    ///Получить текущую температуру
    fn get(&self) -> String {
        self.temperature.to_string()
    }

    fn name(&self) -> &str {
        //println!("{:?} термометр", self.name.as_str());
        self.name.as_str()
    }

    fn interact(&mut self) {
        println!("SmarthouseInterface<Thermometer>::interact был вызыван");
    }

    /// Получить отчет о термометре и температура
    fn report(&self) -> String {
        " - Термометр: '{self.name}' с показанием +{self.temperature} градусов по цельсию\n"
            .to_string()
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

trait SmarthouseInterface {
    fn get_device_info(
        &self,
        room_name: &str,
        device_name: &str,
        storage: &DeviceStorage,
    ) -> Option<String>; // Получаем информацию о том, ГДЕ ИМЕННО В ДОМЕ находится девайс

    fn get_roooms_list(&self) -> String; // Получаем список комнат прям строкой
    fn get_rooms_devices_list(&self, room_name: &str) -> String; // Получаем список устройств прямо строкой
    fn full_report(&self, storage: &DeviceStorage) -> String;
}

impl SmarthouseInterface for Smarthouse {
    fn get_device_info(
        &self,
        room_name: &str,
        device_name: &str,
        storage: &DeviceStorage,
    ) -> Option<String> {
        match self.rooms.get(room_name) {
            Some(room_struct) => {
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

    fn get_roooms_list(&self) -> String {
        let mut room_list = Vec::new();
        for room_name in &self.rooms {
            room_list.push(room_name.0.as_ref());
        }
        format!(
            "В доме '{}' присутствуют следующие комнаты:\n - {}\n",
            self.name,
            room_list.join("\n - ")
        )
    }

    fn get_rooms_devices_list(&self, room_name: &str) -> String {
        match self.rooms.get(room_name) {
            Some(room_struct) => {
                let mut room_devices_list = Vec::<&str>::new();
                for device_name in &room_struct.devices {
                    room_devices_list.push(device_name);
                }
                format!(
                    "В комнате '{}' дома '{}' присутствуют следующие устройства:\n - {}\n",
                    room_name,
                    self.name,
                    room_devices_list.join("\n - ")
                )
            }
            _ => "Комнаты с именем '{room_name}' не найдено.".to_string(),
        }
    }

    fn full_report(&self, storage: &DeviceStorage) -> String {
        let mut full_report = format!("\n*** Полный отчет о состоянии дома '{}' ***\n", self.name);
        full_report += &self.get_roooms_list();
        for room in &self.rooms {
            full_report += "\nУстройства в комнате '{room.0}':\n";
            full_report += &self.get_rooms_devices_list(room.0.as_ref());
            full_report += "\nДанные по устройсвам в '{room.0}':\n";
            for device_name in &room.1.devices {
                if let Some(device_report) = self.get_device_info(room.0, device_name, storage) {
                    full_report += &device_report;
                }
            }
        }
        full_report += "\n*** Отчет о доме окончен ***\n";
        full_report
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
    // let report = smarthouse.get_device_info("Bedroom", "Розетка у ванны в спальне", &storage);
    // println!("Отчет: {:#?}", report);
    // let report = smarthouse.get_device_info("Bedroom", "Розетка в спальне", &storage);
    // println!("Отчет: {:#?}", report);
    // let report = smarthouse.get_device_info("Bedroom", "Термометр в спальне", &storage);
    // println!("Отчет: {:#?}", report);
    // let report = smarthouse.get_device_info("Kitchen", "Розетка над столом кухни", &storage);
    // println!("Отчет: {:#?}", report);
    // let report = smarthouse.get_device_info("Kitchen", "Розетка у плиты", &storage);
    // println!("Отчет: {:#?}", report);
    // print!("Отчет: {}", smarthouse.get_roooms_list());
    // print!("Отчет: {}", smarthouse.get_rooms_devices_list("Kitchen"));
    // print!("Отчет: {}", smarthouse.get_rooms_devices_list("Bedroom"));
    println!("{}", smarthouse.full_report(&storage));
}
