use crate::lamp::Lamp;
use std::io::{Error, ErrorKind};

pub struct Lohse {
    pub _lamp: Lamp,
}

impl Lohse {
    pub fn receive(&mut self, _signal: &str) -> Result<(), Error>  {
        if !self._lamp.is_online {
            return Ok(());
        }
        // Parse signal.
        match _signal {
            "turn_on" => self.turn_on(),
            "turn_off" => self.turn_off(),
            _ => Err(Error::new(ErrorKind::Other, "Unsupported signal"))
        }
    }

    pub fn transmit(&mut self, _signal: &str) -> Result<(), Error>  {
        if !self._lamp.is_online {
            return Ok(());
        }
        // Call Fane to update.
        Ok(())
    }

    // Turns on the lamp. Triggered by Fane's signal.
    pub fn turn_on(&mut self) -> Result<(), Error> {
        self._lamp.is_on = true;
        if self._lamp.is_online {
            // Turn on physical lamp.
        }
        Ok(())
    }

    // Turns off the lamp. Triggered by Fane's signal.
    pub fn turn_off(&mut self) -> Result<(), Error> {
        self._lamp.is_on = false;
        if self._lamp.is_online {
            // Turn off physical lamp.
        }
        Ok(())
    }

    // Toggles the light's on/off status and send it to Fane. Triggered locally.
    pub fn toggle_lamp(&mut self) {
        self._lamp.is_on = !self._lamp.is_on;

        // Update physical lamp status.
        if self._lamp.is_on {
            // Turn on physical lamp.
        } else {
            // Turn off physical lamp.
        }

        match self.transmit("turn_on") {
            Ok(_) => {},
            Err(e) => eprintln!("Error transmitting signal when lamp is toggled: {}", e),
        }
    }

    pub fn toggle_online(&mut self) {
        self._lamp.is_online = !self._lamp.is_online;

        if self._lamp.is_online {
            self.sync_lamp();
        }
    }

    pub fn sync_lamp(&mut self) {
        // Update lamp status according to Fane.
    }
}