// src/main.rs

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

#[derive(Clone)]
struct Thermometer {
    name: String,     // Имя девайса
    temperature: i32, // Температура в градусах цельсия
}

/// Для хранения разных типов устройств в одном контейнере,
/// создаем некий тип-перечисление типов.
#[derive(Clone)]
enum Device {
    Socket(Socket),
    Thermometer(Thermometer),
}

/// Команата
#[derive(Clone)]
struct Room {
    name: String,
    decives: Vec<Device>,
}

///Наш умный дом
struct Smarthouse {
    name: String,     // Имя дома
    rooms: Vec<Room>, // Список комнат
}

// trait ThermometerInterface {
//     fn get_temparature(&self); //Получить текущую температуру
// }

///Интерфейс для умного дома
trait SmarthouseInterface<T> {
    type GetResult;

    fn new(params: T) -> Self;
    fn name(&self) -> &str;
    fn get(&self) -> Self::GetResult; //Получить список оборудования/помещений в комнате/доме
    fn report(&self) -> String; // Получить отчет об оборудовании в помещении/список комнат и оборудования в них
    fn interact(&mut self); //Повернуть выключатель
}

///Интерфейс для розетки
impl SmarthouseInterface<Socket> for Socket {
    type GetResult = Vec<f32>;

    fn new(params: Socket) -> Self {
        Self {
            name: params.name,
            power: params.power,
            state: params.state,
        }
    }

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
    fn get(&self) -> Self::GetResult {
        let power_vec = vec![self.power];
        power_vec
    }

    /// Получить отчет о потреблемой мощности
    fn report(&self) -> String {
        format!(" - Розетка: '{}' на {} ватт\n", self.name, self.power)
    }
}

///Интерфейс для термометра
impl SmarthouseInterface<Thermometer> for Thermometer {
    type GetResult = Vec<i32>;

    fn new(param: Thermometer) -> Self {
        Self {
            name: param.name,
            temperature: param.temperature,
        }
    }

    ///Получить текущую температуру
    fn get(&self) -> Self::GetResult {
        let temperature_vec = vec![self.temperature];
        temperature_vec
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

impl SmarthouseInterface<Room> for Room {
    type GetResult = Vec<Device>;

    fn new(param: Room) -> Self {
        Self {
            name: param.name,
            decives: param.decives,
        }
    }

    fn interact(&mut self) {
        println!("SmarthouseInterface<Room>::interact был вызван");
    }

    fn name(&self) -> &str {
        println!("{:?} комната", self.name.as_str());
        self.name.as_str()
    }

    fn get(&self) -> Self::GetResult {
        self.decives.clone()
    }

    fn report(&self) -> String {
        let mut output = format!("**Комната:{}**\n", self.name);
        if !self.decives.is_empty() {
            output += "**Устройсва:**\n";
            for d in &self.decives {
                match d {
                    Device::Socket(socket) => {
                        output += &socket.report();
                        //output += format!(" - Розетка: {}\n", socket.name).as_str();
                    }
                    Device::Thermometer(thermometer) => {
                        output += &thermometer.report();
                        //output += format!(" - Термометр: {}\n", thermometer.name).as_str();
                    }
                }
            }
        } else {
            output += "**В комнате не обнаружено устройств.**\n\n"
        }
        output += "****\n\n";
        output
    }
}

impl SmarthouseInterface<Smarthouse> for Smarthouse {
    type GetResult = Vec<Room>;

    fn new(param: Smarthouse) -> Self {
        Self {
            name: param.name,
            rooms: param.rooms,
        }
    }

    fn name(&self) -> &str {
        println!("{:?} house", self.name.as_str());
        self.name.as_str()
    }

    fn interact(&mut self) {
        println!("SmarthouseInterface<Smarthouse>::interact был вызван");
    }

    fn report(&self) -> String {
        let mut output = format!("**Дом:{}**\n", self.name);
        if !self.rooms.is_empty() {
            output += "**Комнаты:**\n";
            for room in &self.rooms {
                output += &room.report();
            }
        } else {
            output += "**В доме не обнаружено комнат.**\n\n"
        }
        output += "****\n\n";
        output
    }

    fn get(&self) -> Self::GetResult {
        self.rooms.clone()
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
