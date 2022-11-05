
#[derive(Debug)]
struct Car {
    name: &'static str,
    price: i64
}

trait Automobile {
    fn new(name: &'static str) -> Self;
    fn get_name(&self) -> &'static str;
    fn print(&self) {
        println!("I am {}.", self.get_name());
    }
}

impl Car {
    // TODO: overload Car::new
    // fn new(name: &'static str, price: i64) -> Car {
    //     Car { name: name, price: price }
    // }
}

impl Automobile for Car {
    fn new(name: &'static str) -> Car {
        Car { name: name, price: 0 }
    }
    fn get_name(&self) -> &'static str {
        self.name
    }
}

fn main() {
    println!("Hello, world!");
    // let p = Point { x:2, y:54 };
    let car = Car::new("My car");
    println!("{:?}", car);
}
