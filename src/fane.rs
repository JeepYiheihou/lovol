use std::io::{Error, ErrorKind};

pub struct Fane {
    pub central_on: bool,
}

impl Fane {
    pub fn receive(&mut self, _signal: &str) -> Result<(), Error>  {
        // Parse signal.
        match _signal {
            "turn_on" => self.turn_on(),
            "turn_off" => self.turn_off(),
            "read_status" => self.read_status(),
            _ => Err(Error::new(ErrorKind::Other, "Unsupported incoming signal to Fane"))
        }
    }

    pub fn broadcast(&mut self, _signal: &str) -> Result<(), Error>  {
        // Call client to update all Lohse's.
        Ok(())
    }

    // Turns on the central bit. Triggered by one of the Lohse's.
    pub fn turn_on(&mut self) -> Result<(), Error> {
        if self.central_on {
            println!("Turning on central state when it's already on.");
        }
        self.central_on = true;
        // Broadcast to all Lohse's.
        Ok(())
    }

    // Turns off the central bit. Triggered by one of the Lohse's.
    pub fn turn_off(&mut self) -> Result<(), Error> {
        if !self.central_on {
            println!("Turning off central state when it's already off.");
        }
        self.central_on = false;
        Ok(())
    }

    pub fn read_status(&mut self) -> Result<(), Error> {
        return self.broadcast(if self.central_on {"turn_on"} else {"turn_off"});
    }
}