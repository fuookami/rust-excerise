#[derive(Clone)]
struct Person {
    name: String,
}

trait Vehicle {
    fn manufacturer(&self) -> &str;
    fn engine_cylinder(&self) -> u64;
    fn owner(&self) -> &Person;
}

trait Car: Vehicle {}

struct SportCar {
    _manufacturer: String,
    _engine_cylinder: u64,
    _owner: Person,
}

impl Vehicle for SportCar {
    fn manufacturer(&self) -> &str {
        &self._manufacturer
    }

    fn engine_cylinder(&self) -> u64 {
        self._engine_cylinder
    }

    fn owner(&self) -> &Person {
        &self._owner
    }
}

impl Car for SportCar {}

fn main() {
    println!("Hello, world!");
}
