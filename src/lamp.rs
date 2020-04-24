pub struct Lamp {
    pub is_on: bool,
    pub is_dont_disturb: bool
}

impl Lamp {
    pub fn new(is_on: bool, is_dont_disturb: bool) -> Lamp {
        return Lamp {
            is_on: is_on,
            is_dont_disturb: is_dont_disturb
        };
    }

    // Turns on the light unless in DND mode. Triggered when receiving remote signal to turn on.
    pub fn turn_on(&mut self) {
        self.is_on = true;
    }

    // Turns off the light. Triggered when receiving remote signal to turn off.
    pub fn turn_off(&mut self) {
        self.is_on = false;
    }

    // Toggles the light's on/off status and send it to the other lamp.
    pub fn toggle(&mut self) {
        if self.is_dont_disturb {
            return;
        }
        self.is_on = !self.is_on;
        // TODO: Call update lamps.
    }
}
