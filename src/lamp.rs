pub struct Lamp {
    pub is_on: bool,
    pub is_online: bool
}

impl Lamp {
    pub fn new(is_on: bool, is_online: bool) -> Lamp {
        return Lamp {
            is_on: is_on,
            is_online: is_online
        };
    }
}
