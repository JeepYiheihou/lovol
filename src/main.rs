mod lamp;

fn main() {
    let mut _local_lamp : lamp::Lamp = lamp::Lamp::new(false, false);
    _local_lamp.turn_on();
    println!("Hello, lovol!");
}
