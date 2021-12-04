#[derive(Clone)]
struct Person {
    name: String,
}

trait Vehicle {
    fn manufacturer(&self) -> &str;
    fn engine_cylinder(&self) -> u64;
    fn owner(&self) -> &Person;
}

#[derive(Clone)]
struct Truck {
    _manufacturer: String,
    _engine_cylinder: u64,
    _owner: Person,
    _capacity: f64,
    _traction: u64,
}

impl Vehicle for Truck {
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

impl Truck {
    fn new(
        manufacturer: String,
        engine_cylinder: u64,
        owner: Person,
        capacity: f64,
        traction: u64,
    ) -> Truck {
        Truck {
            _manufacturer: manufacturer,
            _engine_cylinder: engine_cylinder,
            _owner: owner,
            _capacity: capacity,
            _traction: traction,
        }
    }

    fn capacity(&self) -> f64 {
        self._capacity
    }

    fn traction(&self) -> u64 {
        self._traction
    }
}

fn main() {
    println!("Hello, world!");
}
