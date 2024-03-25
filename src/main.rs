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

struct Smarthouse {
    outlet: Outlet,
    thermometer: Thermometer,
}

trait SmarthouseInterface {
    fn new() -> Self;
    fn temperature(&self);
    fn turn_switch(&mut self, turn: &bool);
    fn outlet_power(&self);
}

impl SmarthouseInterface for Smarthouse {
    fn new() -> Self {
        Self {
            outlet: Outlet {
                power: 3.6,
                state: OutletState::IsOff,
            },
            thermometer: Thermometer { temperature: 23 },
        }
    }

    fn temperature(&self) {
        self.thermometer.get_temparature();
    }

    fn turn_switch(&mut self, turn: &bool) {
        self.outlet.turn_switch(turn);
    }

    fn outlet_power(&self) {
        self.outlet.get_power();
    }
}

fn main() {
    let mut smarthouse = Smarthouse::new();
    smarthouse.outlet_power();
    smarthouse.turn_switch(&true);
    smarthouse.turn_switch(&false);
    smarthouse.temperature();
}
