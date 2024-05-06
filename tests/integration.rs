use smarthouse::*;

#[test]
fn main_smarthouse_test() {
    // Создаем имена для наших устройств в разных комнатах
    let socket_bedroom_name = String::from("Розетка в спальне");
    let socket_bedroom_bathroom_name = String::from("Розетка у ванны в спальне");
    let thermo_bedroom_name = String::from("Термометр в спальне");
    let socket_kitchen_name = String::from("Розетка над столом кухни");
    let socket_kitchen_stove_name = String::from("Розетка у плиты");
    
    // Создаем имена наших комнат
    let bedroom_name = String::from("Спальня");
    let kitchen_name = String::from("Кухня");

    // Создаем набор устройств для спальни
    let bedroom_device: Vec<Box<dyn DeviceInterface>> = vec![
        Box::new(Socket::new(Socket {
            name: socket_bedroom_name.clone(),
            power: 220.0,
            state: SocketState::IsOff,
        })),
        Box::new(Socket::new(Socket {
            name: socket_bedroom_bathroom_name.clone(),
            power: 210.0,
            state: SocketState::IsOn,
        })),
        Box::new(Thermometer::new(Thermometer {
            name: thermo_bedroom_name.clone(),
            temperature: 22,
        })),
    ];
    let kitche_device: Vec<Box<dyn DeviceInterface>> = vec![
        Box::new(Socket::new(Socket {
            name: socket_kitchen_name.clone(),
            power: 220.0,
            state: SocketState::IsOff,
        })),
        Box::new(Socket::new(Socket {
            name: socket_kitchen_stove_name.clone(),
            power: 210.0,
            state: SocketState::IsOn,
        })),
    ];
    let mut storage: DeviceStorage = DeviceStorage::new(DeviceStorage {
        room_map: HashMap::new(),
    });
    storage
        .room_map
        .insert(bedroom_name.clone(), bedroom_device);
    storage
        .room_map
        .insert(kitchen_name.clone(), kitche_device);

    // Создали умный дом
    let smarthouse = Smarthouse::new("Домашная работа 3", &storage);

    // Проверяем список комнат
    assert!(smarthouse.get_roooms_list().contains(&bedroom_name));
    assert!(smarthouse.get_roooms_list().contains(&kitchen_name));

    // Проверяем список устройств в спальне
    assert!(smarthouse.get_rooms_devices_list(&bedroom_name).contains(&socket_bedroom_name));
    assert!(smarthouse.get_rooms_devices_list(&bedroom_name).contains(&socket_bedroom_bathroom_name));
    assert!(smarthouse.get_rooms_devices_list(&bedroom_name).contains(&thermo_bedroom_name));

    // Проверяем список устройств на кухне
    assert!(smarthouse.get_rooms_devices_list(&kitchen_name).contains(&socket_kitchen_name));
    assert!(smarthouse.get_rooms_devices_list(&kitchen_name).contains(&socket_kitchen_stove_name));

    // Проверяем отчеты по устройствам
    let exp_msg = format!("'{}' не найдено в {}", socket_bedroom_name, bedroom_name);
    assert!(smarthouse.get_device_info(&bedroom_name, &socket_bedroom_name, &storage).expect(&exp_msg).contains(&socket_bedroom_name));
    assert!(smarthouse.get_device_info(&bedroom_name, &socket_bedroom_name, &storage).expect(&exp_msg).contains("Розетка"));
    assert!(smarthouse.get_device_info(&bedroom_name, &socket_bedroom_name, &storage).expect(&exp_msg).contains("ватт"));
    assert!(smarthouse.get_device_info(&bedroom_name, &socket_bedroom_bathroom_name, &storage).expect(&exp_msg).contains(&socket_bedroom_bathroom_name));
    assert!(smarthouse.get_device_info(&bedroom_name, &socket_bedroom_bathroom_name, &storage).expect(&exp_msg).contains("Розетка"));
    assert!(smarthouse.get_device_info(&bedroom_name, &socket_bedroom_bathroom_name, &storage).expect(&exp_msg).contains("ватт"));
    assert!(smarthouse.get_device_info(&bedroom_name, &thermo_bedroom_name, &storage).expect(&exp_msg).contains(&thermo_bedroom_name));
    assert!(smarthouse.get_device_info(&bedroom_name, &thermo_bedroom_name, &storage).expect(&exp_msg).contains("Термометр"));
    assert!(smarthouse.get_device_info(&bedroom_name, &thermo_bedroom_name, &storage).expect(&exp_msg).contains("градусов по цельсию"));
    assert_eq!(smarthouse.get_device_info(&kitchen_name, format!("Неведомая хрень").as_str(), &storage), None);
}