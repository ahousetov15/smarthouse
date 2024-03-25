enum OutletState {
    IsOn,
    IsOff,
}

struct Outlet {
    power: f32,
    state: OutletState,
}

struct Thermometer {
    temperature: i32,
}

trait OutletInterface {
    fn turn_switch(&mut self, state: &bool);
    fn get_power(&self);
}

trait ThermometerInterface {
    fn get_temparature(&self);
}

impl OutletInterface for Outlet {
    fn turn_switch(&mut self, state: &bool) {
        if *state == true {
            self.state = OutletState::IsOn;
            println!("Outlet is switched on");
        } else {
            self.state = OutletState::IsOff;
            println!("Outlet is switched off");
        }
    }

    fn get_power(&self) {
        let value = self.power;
        println!("Outler power is {value} vatt");
    }
}

impl ThermometerInterface for Thermometer {
    fn get_temparature(&self) {
        let current_temperature = self.temperature;
        println!("Current temperature: {current_temperature:?}");
    }
}

fn main() {
    let mut outlet = Outlet {
        power: 3.6,
        state: OutletState::IsOff,
    };
    let temp = Thermometer { temperature: 23 };
    outlet.get_power();
    outlet.turn_switch(&true);
    temp.get_temparature();
}
