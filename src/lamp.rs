pub struct Lamp {
    pub is_on: bool,
    pub is_dont_disturb: bool
}

impl Lamp {
    pub fn new(is_on: bool, is_dont_disturb: bool)->Lamp {
        return Lamp{ is_on: is_on, is_dont_disturb: is_dont_disturb };
    }

    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    pub fn turn_switch(&mut self) {
        self.is_on = !self.is_on;
    }
}
