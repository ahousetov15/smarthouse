pub mod device_interface {
    use crate::smarthouse::devices::devices::{Socket, Thermometer};
    
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

}