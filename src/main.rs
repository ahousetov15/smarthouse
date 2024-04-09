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

// /// Для хранения разных типов устройств в одном контейнере,
// /// создаем некий тип-перечисление типов.
// #[derive(Clone)]
// enum Device {
//     Socket(Socket),
//     Thermometer(Thermometer),
// }

/// Команата
#[derive(Clone)]
struct Room {
    name: String,
    // decives: Vec<Device>,
}

// ///Наш умный дом
// struct Smarthouse {
//     name: String,     // Имя дома
//     rooms: Vec<Room>, // Список комнат
// }

// trait ThermometerInterface {
//     fn get_temparature(&self); //Получить текущую температуру
// }

trait DeviceInterface<T> {
    type GetResult;

    fn name(&self) -> &str;
    fn get(&self) -> Self::GetResult; //Получить список оборудования/помещений в комнате/доме
    fn interact(&mut self); //Повернуть выключатель
    fn report(&self) -> String;
}

impl DeviceInterface<Socket> for Socket {
    type GetResult = f32;

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
        self.power.clone()
    }

    /// Получить отчет о потреблемой мощности
    fn report(&self) -> String {
        format!(" - Розетка: '{}' на {} ватт\n", self.name, self.power)
    }
}

impl DeviceInterface<Thermometer> for Thermometer {
    type GetResult = i32;

    ///Получить текущую температуру
    fn get(&self) -> Self::GetResult {
        self.temperature.clone()
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
    let bedroom_devices: Vec<dyn SmarthouseInterface<T>> = vec![Socket::new(Socket {
        name: "Розетка_у_ванной".to_string(),
        power: 220.1,
        state: SocketState::IsOn,
    })];
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
