trait Pet {
    fn description(&self) -> &str;
}

struct PetBase {
    name: String,
    neuter_spayed: bool,
    talks: bool,
}

impl PetBase {
    fn new(name: String, talks: bool) -> Self {
        Self {
            name: name,
            neuter_spayed: false,
            talks: talks,
        }
    }
}

struct Dog {
    pet: PetBase,
    _description: String,
}

impl Pet for Dog {
    fn description(&self) -> &str {
        &self._description
    }
}

impl Dog {
    fn new(name: String, talks: bool) -> Self {
        let description = format!("Dog named {}\nNeuter/Spayed {}", name, false);
        Self {
            pet: PetBase::new(name, talks),
            _description: description,
        }
    }
}

struct Cat {
    pet: PetBase,
    _description: String,
}

impl Pet for Cat {
    fn description(&self) -> &str {
        &self._description
    }
}

impl Cat {
    fn new(name: String, talks: bool) -> Self {
        let description = format!("Dog named {}\nNeuter/Spayed {}", name, false);
        Self {
            pet: PetBase::new(name, talks),
            _description: description,
        }
    }
}

struct Parrot {
    pet: PetBase,
    _description: String,
}

impl Pet for Parrot {
    fn description(&self) -> &str {
        &self._description
    }
}

impl Parrot {
    fn new(name: String, talks: bool) -> Self {
        let description = format!("Dog named {}\nTalks: {}", name, talks);
        Self {
            pet: PetBase::new(name, talks),
            _description: description,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
