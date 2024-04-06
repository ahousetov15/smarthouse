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

///Наш умный дом
struct Smarthouse {
    outlet: Socket,           //Розетка
    thermometer: Thermometer, //Термометр
}

trait SocketInterface {
    fn turn_switch(&mut self, state: &bool); //Повернуть выключатель (во Вкл. или Выкл.)
    fn get_power(&self); // Получить потребляемую мощность
}

trait ThermometerInterface {
    fn get_temparature(&self); //Получить текущую температуру
}

///Интерфейс для умного дома
trait SmarthouseInterface {
    fn new() -> Self; //Конструктор для инициализации со значениями по умолчанию
    fn temperature(&self); //Получить текущую температуру
    fn turn_switch(&mut self, turn: &bool); //Повернуть выключатель
    fn outlet_power(&self); //Получить потребляемую мощность
}

///Интерфейс для розетки
impl SocketInterface for Socket {
    /// Повернуть выключатель (во Вкл. или Выкл.)
    fn turn_switch(&mut self, state: &bool) {
        if *state {
            self.state = SocketState::IsOn;
            println!("Socket is switched on");
        } else {
            self.state = SocketState::IsOff;
            println!("Socket is switched off");
        }
    }

    /// Получить потребляемую мощность
    fn get_power(&self) {
        let value = self.power;
        println!("Socket power is {value} vatt");
    }
}

///Интерфейс для термометра
impl ThermometerInterface for Thermometer {
    ///Получить текущую температуру
    fn get_temparature(&self) {
        let current_temperature = self.temperature;
        println!("Current temperature: {current_temperature:?}");
    }
}

///! Имплементация интерфейса для умного дома
impl SmarthouseInterface for Smarthouse {
    /// Конструктор для инициализации со значениями по умолчанию
    fn new() -> Self {
        Self {
            outlet: Socket {
                power: 3.6,
                state: SocketState::IsOff,
            },
            thermometer: Thermometer { temperature: 23 },
        }
    }

    /// Получить текущую температуру
    fn temperature(&self) {
        self.thermometer.get_temparature();
    }

    /// Повернуть выключатель
    fn turn_switch(&mut self, turn: &bool) {
        self.outlet.turn_switch(turn);
    }

    /// Получить потребляемую мощность
    fn outlet_power(&self) {
        self.outlet.get_power();
    }
}

fn main() {
    let mut smarthouse = Smarthouse::new(); //Создание экземпляра умного дома
    smarthouse.outlet_power(); //Получить потребляемую мощность
    smarthouse.turn_switch(&true); //Повернуть выключатель
    smarthouse.turn_switch(&false); //Повернуть выключатель
    smarthouse.temperature(); //Получить текущую температуру
}
