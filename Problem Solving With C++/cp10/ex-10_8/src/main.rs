struct Odometer {
    mileage: f64,
    fuel: f64
}

impl Odometer {
    fn new() -> Self {
        Self {
            mileage: 0.,
            fuel: 0.
        }
    }

    fn reset(&mut self) {
        self.mileage = 0.;
    }

    fn increase_mileage(&mut self, mileage: f64) {
        self.mileage += mileage;
    }

    fn set_fuel(&mut self, fuel: f64) {
        self.fuel = fuel;
    }

    fn total_fuel(&self) -> f64 {
        self.mileage * self.fuel
    }
}


fn main() {
    let mut odometer = Odometer::new();
    
    odometer.set_fuel(1.0);
    odometer.increase_mileage(1.0);
    assert_eq!(odometer.total_fuel(), 1.0);

    odometer.increase_mileage(2.0);
    assert_eq!(odometer.total_fuel(), 3.0);

    odometer.set_fuel(2.0);
    assert_eq!(odometer.total_fuel(), 6.0);

    odometer.reset();
    assert_eq!(odometer.total_fuel(), 0.);

    odometer.increase_mileage(4.0);
    assert_eq!(odometer.total_fuel(), 8.0);
}
