pub mod devices{
    use super::super::enums::SocketState;

    #[derive(Clone)]
    pub struct Socket {
        pub name: String,       //Имя девайса
        pub power: f32,         //Величина в ваттах
        pub state: SocketState, //Состояние розетки (Вкл./Выкл.)
    }

    impl Socket {
        pub fn new(params: Socket) -> Self {
            Self {
                name: params.name,
                power: params.power,
                state: params.state,
            }
        }


        /// Повернуть выключатель (во Вкл. или Выкл.)
        pub fn interact(&mut self){
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
    }

    #[derive(Clone)]
    pub struct Thermometer {
        pub name: String,     // Имя девайса
        pub temperature: i32, // Температура в градусах цельсия
    }

    impl Thermometer {
        pub fn new(param: Thermometer) -> Self {
            Self {
                name: param.name,
                temperature: param.temperature,
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::super::enums::SocketState;
    use crate::smarthouse::devices::devices::{Socket, Thermometer};
    #[test]
    fn test_socket() {
        let mut socket = Socket::new(Socket {
            name: "Розетка".to_string(),
            power: 220.0,
            state: SocketState::IsOff
        });
        assert_eq!(socket.state, SocketState::IsOff);
        socket.interact();
        assert_eq!(socket.state, SocketState::IsOn);
        assert_eq!(socket.name, "Розетка".to_string());
        assert_eq!(socket.power, 220.0);
        socket.power = 210.0;
        assert_eq!(socket.power, 210.0);
    }

    #[test]
    fn test_thermometer() {
        let mut therm: Thermometer = Thermometer::new(Thermometer {
            name: "Термометр".to_string(),
            temperature: 22
        });
        assert_eq!(therm.temperature, 22);
        therm.temperature = 23;
        assert_eq!(therm.temperature, 23);
        let new_name = String::from("ЯДрачистыйИзумруд"); 
        therm.name = "ЯДрачистыйИзумруд".to_string();
        assert_eq!(therm.name, new_name);
    }
}