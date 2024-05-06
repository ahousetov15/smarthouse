pub mod device_interface {
    use crate::smarthouse::devices::devices::{Socket, Thermometer};
    use crate::smarthouse::smarthouse::smarthouse::Smarthouse;
    use crate::storage::device_storage::device_storage::DeviceStorage;
    
    pub trait DeviceInterface {
        fn get_name(&self) -> &str;
        fn get(&self) -> String; //Получить список оборудования/помещений в комнате/доме
        fn interact(&mut self);
        fn report(&self) -> String;
    }

    impl DeviceInterface for Socket {
        fn get_name(&self) -> &str {
            self.name.as_str()
        }

        /// Повернуть выключатель (во Вкл. или Выкл.)
        fn interact(&mut self) {
            self.interact()
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

        fn get_name(&self) -> &str {
            self.name.as_str()
        }

        fn interact(&mut self) {
            println!("SmarthouseInterface<Thermometer>::interact был вызыван");
        }

        /// Получить отчет о термометре и температура
        fn report(&self) -> String {
            format!(" - Термометр: '{}' с показанием +{} градусов по цельсию\n", self.name, self.temperature)
        }
    }

    pub trait SmarthouseInterface {
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
                _ => format!("Комнаты с именем '{:?}' не найдено.", room_name),
            }
        }

        fn full_report(&self, storage: &DeviceStorage) -> String {
            let mut full_report = format!("\n*** Полный отчет о состоянии дома '{}' ***\n", self.name);
            full_report += &self.get_roooms_list();
            for room in &self.rooms {
                full_report += format!("\nУстройства в комнате '{}':\n", room.0).as_str();
                full_report += &self.get_rooms_devices_list(room.0.as_ref());
                full_report += format!("\nДанные по устройсвам в '{}':\n", room.0).as_str();
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
}


#[cfg(test)]
mod tests {
    use super::super::enums::SocketState;
    use crate::smarthouse::devices::devices::{Socket, Thermometer};
    use crate::DeviceInterface;


    #[test]
    fn test_device_interface_socket() {
        let socket_name = "Розетка".to_string();
        let mut socket = Socket::new(Socket {
            name: socket_name.clone(),
            power: 220.0,
            state: SocketState::IsOff
        });
        
        let device_interface: &mut dyn DeviceInterface = &mut socket;
        assert_eq!(device_interface.get_name(), &socket_name);
        device_interface.interact();
        assert_eq!(device_interface.get(), socket.power.to_string());
        assert_eq!(socket.state, SocketState::IsOn);
    }

    #[test]
    fn test_device_interface_thermometer() {
        let therm_name = "Термометр".to_string();
        let mut therm = Thermometer::new(Thermometer {
            name: therm_name.clone(),
            temperature: 22
        });
        let device_interface: &mut dyn DeviceInterface = &mut therm;
        assert_eq!(device_interface.get_name(), &therm_name);
        assert_eq!(device_interface.get(), therm.temperature.to_string());
    }
}