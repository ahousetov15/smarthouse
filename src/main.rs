// src/main.rs

///Состояния нашей розетки
enum OutletState {
    IsOn,  //Включено
    IsOff, //Выключено
}

struct Outlet {
    power: f32,         //Величина в ваттах
    state: OutletState, //Состояние розетки (Вкл./Выкл.)
}

struct Thermometer {
    temperature: i32, // Температура в градусах цельсия
}

trait OutletInterface {
    fn turn_switch(&mut self, state: &bool); //Повернуть выключатель (во Вкл. или Выкл.)
    fn get_power(&self); // Получить потребляемую мощность
}

trait ThermometerInterface {
    fn get_temparature(&self); //Получить текущую температуру
}

///Интерфейс для розетки
impl OutletInterface for Outlet {
    fn turn_switch(&mut self, state: &bool) {
        //Повернуть выключатель (во Вкл. или Выкл.)
        if *state {
            self.state = OutletState::IsOn;
            println!("Outlet is switched on");
        } else {
            self.state = OutletState::IsOff;
            println!("Outlet is switched off");
        }
    }

    fn get_power(&self) {
        //Получить потребляемую мощность
        let value = self.power;
        println!("Outler power is {value} vatt");
    }
}

///Интерфейс для термометра
impl ThermometerInterface for Thermometer {
    fn get_temparature(&self) {
        //Получить текущую температуру
        let current_temperature = self.temperature;
        println!("Current temperature: {current_temperature:?}");
    }
}

///Наш умный дом
struct Smarthouse {
    outlet: Outlet,           //Розетка
    thermometer: Thermometer, //Термометр
}

///Интерфейс для умного дома
trait SmarthouseInterface {
    fn new() -> Self; //Конструктор для инициализации со значениями по умолчанию
    fn temperature(&self); //Получить текущую температуру
    fn turn_switch(&mut self, turn: &bool); //Повернуть выключатель
    fn outlet_power(&self); //Получить потребляемую мощность
}

///Интерфейс для умного дома
///
impl SmarthouseInterface for Smarthouse {
    ///Конструктор
    fn new() -> Self {
        //Конструктор для инициализации со значениями по умолчанию
        Self {
            outlet: Outlet {
                power: 3.6,
                state: OutletState::IsOff,
            },
            thermometer: Thermometer { temperature: 23 },
        }
    }

    ///Получить текущую температуру
    fn temperature(&self) {
        self.thermometer.get_temparature();
    }

    ///Повернуть выключатель
    fn turn_switch(&mut self, turn: &bool) {
        self.outlet.turn_switch(turn);
    }

    ///Получить потребляемую мощность
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
