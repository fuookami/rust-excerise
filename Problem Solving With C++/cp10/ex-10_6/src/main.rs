use rand::Rng;

struct Counter {
    num: u64
}

impl Counter {
    const MAX_NUM : u64 = 9999;

    fn new() -> Self {
        Self {
            num: 0
        }
    }

    fn reset(&mut self) {
        self.num = 0;
    }

    fn increase_1(&mut self) {
        self.num += 1;
    }

    fn increase_10(&mut self) {
        self.num += 10;
    }

    fn increase_100(&mut self) {
        self.num += 100;
    }

    fn increase_1000(&mut self) {
        self.num += 1000;
    }

    fn overflow(&self) -> bool {
        self.num > Self::MAX_NUM
    }

    fn print(&self) {
        match self.overflow() {
            true => println!("{:02}.{:02}, overflow", self.num / 100 % 100, self.num % 100),
            false => println!("{:02}.{:02}", self.num / 100 % 100, self.num % 100),
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut counter = Counter::new();
    counter.print();

    while !counter.overflow() {
        match rng.gen_range(0..4) {
            0 => counter.increase_1(),
            1 => counter.increase_10(),
            2 => counter.increase_100(),
            3 => counter.increase_1000(),
            _ => panic!("Impossible")
        }
        counter.print();
    }

    counter.reset();
    counter.print();

    while !counter.overflow() {
        match rng.gen_range(0..4) {
            0 => counter.increase_1(),
            1 => counter.increase_10(),
            2 => counter.increase_100(),
            3 => counter.increase_1000(),
            _ => panic!("Impossible")
        }
        counter.print();
    }
}
