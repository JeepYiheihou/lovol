mod lamp;

fn main() {
    let _local_lamp : lamp::Lamp = lamp::Lamp{ is_on: false, is_dont_disturb: false };
    println!("Hello, lovol!");
}
