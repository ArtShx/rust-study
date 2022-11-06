
#[derive(Debug)]
enum Brand {
    Audi,
    BMW,
    Hyundai,
    Ford,
    UNKNOWN
}

#[derive(Debug)]
pub struct Car {
    name: &'static str,
    price: f64,
    brand: Brand
}

trait Automobile {
    fn new(brand: Brand, name: &'static str) -> Self;
    fn get_name(&self) -> &'static str;
    fn print(&self) {
        println!("I am \"{}\".", self.get_name());
    }
}

impl Car {
    fn new_with_price(name: &'static str, price: f64) -> Car {
        Car { brand: Brand::UNKNOWN, name: name, price: price }
    }
}

impl Automobile for Car {
    fn new(brand: Brand, name: &'static str) -> Car {
        Car { brand: brand, name: name, price: 0. }
    }
    fn get_name(&self) -> &'static str {
        self.name
    }
}

fn get_name_of_automobile(automobile: &impl Automobile) -> () {
    let name = automobile.get_name();
    println!("Name of the automobile: {name}");
}

fn main() {
    println!("Hello, world!");
    // let p = Point { x:2, y:54 };
    let car = Car::new(Brand::Ford, "My car");
    // let other_car = Car::new_with_price("My other car", 5655.45);
    let car_name = car.get_name();
    car.print();
    println!("{:?}; name: {car_name}; brand: {:?}", car, car.brand);
    // println!("{:?}", other_car);
    get_name_of_automobile(&car);
}
