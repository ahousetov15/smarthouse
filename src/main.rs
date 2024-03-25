enum OutletState {
    IsOn,
    IsOff,
}

struct Outlet {
    power: f32,
    state: OutletState,
}

trait OutletInterface {
    fn turn_switch(&mut self, state: &bool);
    fn get_power(&self);
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

fn main() {
    let mut outlet = Outlet {
        power: 3.6,
        state: OutletState::IsOff,
    };
    outlet.get_power();
    outlet.turn_switch(&true);
}
