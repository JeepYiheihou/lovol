mod lamp;

fn main() {
    let mut _local_lamp : lamp::Lamp = lamp::Lamp::new(false, false);
    println!("Hello, lovol! before {} ", _local_lamp.is_on);
    _local_lamp.turn_on();
    println!("Hello, lovol! after {} ", _local_lamp.is_on);
}
