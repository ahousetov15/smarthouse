// src/main.rs

///Состояния нашей розетки
enum SocketState {
    IsOn,  //Включено
    IsOff, //Выключено
}

struct Socket {
    power: f32,         //Величина в ваттах
    state: SocketState, //Состояние розетки (Вкл./Выкл.)
}

struct Thermometer {
    temperature: i32, // Температура в градусах цельсия

}

/// Для хранения разных типов устройств в одном контейнере,
/// создаем некий тип-перечисление типов.
enum Device {
    Socket(Socket),
    Thermometer(Thermometer),
}

///Наш умный дом
struct Smarthouse {
    name: String,             // Имя дома
    socket: Socket,           //Розетка
    thermometer: Thermometer, //Термометр
}

/// Команата
struct Room {
    name: String,
    decives: Vec<Device>,
}

// trait SocketInterface {
//    fn turn_switch(&mut self, state: &bool); //Повернуть выключатель (во Вкл. или Выкл.)
//    fn get_power(&self); // Получить потребляемую мощность
// }

trait ThermometerInterface {
    fn get_temparature(&self); //Получить текущую температуру
}

///Интерфейс для умного дома
// trait SmarthouseInterface {
//    fn new() -> Self; //Конструктор для инициализации со значениями по умолчанию
//    fn temperature(&self); //Получить текущую температуру
//    fn turn_switch(&mut self, turn: &bool); //Повернуть выключатель
//    fn socket_power(&self); //Получить потребляемую мощность
// }
trait SmarthouseInterface {
    fn new() where Self: Sized; //Конструктор для инициализации со значениями по умолчанию
    fn get(&self); //Получить текущую температуру
    fn interact(&mut self, turn: &bool); //Повернуть выключатель
}

///Интерфейс для розетки
impl SmarthouseInterface for Socket {
    fn new() {
        Socket{
            power: 0.0,
            state: SocketState::IsOff,
        }
    }

    /// Повернуть выключатель (во Вкл. или Выкл.)
    fn interact(&mut self, state: &bool) {
        if *state {
            self.state = SocketState::IsOn;
            println!("Socket is switched on");
        } else {
            self.state = SocketState::IsOff;
            println!("Socket is switched off");
        }
    }

    /// Получить потребляемую мощность
    fn get(&self) {
        let value = self.power;
        println!("Socket power is {value} vatt");
    }
}

///Интерфейс для термометра
impl SmarthouseInterface for Thermometer {
    fn new() -> () {
        Thermometer{
            temperature: 23,
        }
    }

    ///Получить текущую температуру
    fn get(&self) {
        let current_temperature = self.temperature;
        println!("Current temperature: {current_temperature:?}");
    }

    fn interact(&mut self, turn: &bool) {
        println!("Turned {}", if *turn { "on" } else { "off" });
    }
}

///! Имплементация интерфейса для умного дома
// impl SmarthouseInterface for Smarthouse {
//     /// Конструктор для инициализации со значениями по умолчанию
//     fn new() -> Self {
//         Self {
//             socket: Socket {
//                 power: 3.6,
//                 state: SocketState::IsOff,
//             },
//             thermometer: Thermometer { temperature: 23 },
//         }
//     }

//     /// Получить текущую температуру
//     fn temperature(&self) {
//         self.thermometer.get_temparature();
//     }

//     /// Повернуть выключатель
//     fn turn_switch(&mut self, turn: &bool) {
//         self.socket.turn_switch(turn);
//     }

//     /// Получить потребляемую мощность
//     fn socket_power(&self) {
//         self.socket.get_power();
//     }
// }

fn main() {
    // let mut smarthouse = Smarthouse::new(); //Создание экземпляра умного дома
    // smarthouse.socket_power(); //Получить потребляемую мощность
    // smarthouse.turn_switch(&true); //Повернуть выключатель
    // smarthouse.turn_switch(&false); //Повернуть выключатель
    // smarthouse.temperature(); //Получить текущую температуру
}
