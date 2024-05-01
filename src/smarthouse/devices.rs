pub mod devices{
    ///Состояния нашей розетки
    #[derive(Clone)]
    #[non_exhaustive]
    pub enum SocketState {
        IsOn,  //Включено
        IsOff, //Выключено
    }

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
        pub fn interact(&mut self) {
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