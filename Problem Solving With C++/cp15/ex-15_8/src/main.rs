use std::collections::HashMap;

trait ShippingContainer {
    fn id(&self) -> u64;

    fn manifest(&self) -> &str {
        ""
    }
}

struct ManualShippingContainer {
    _id: u64,
    _manifest: String,
}

impl ShippingContainer for ManualShippingContainer {
    fn id(&self) -> u64 {
        self._id
    }

    fn manifest(&self) -> &str {
        &self._manifest
    }
}

impl ManualShippingContainer {
    fn set_manifest(&mut self, manifest: String) {
        self._manifest = manifest
    }
}

struct RFIDShippingContainer {
    _id: u64,
    _mainfest: HashMap<String, (u64, String)>,
    _mainfest_cache: String,
}

impl ShippingContainer for RFIDShippingContainer {
    fn id(&self) -> u64 {
        self._id
    }

    fn manifest(&self) -> &str {
        &self._mainfest_cache
    }
}

impl RFIDShippingContainer {
    fn add(&mut self, amount: u64, unit: String, name: String) {
        let counter = self._mainfest.entry(name).or_insert((0, unit));
        counter.0 += 1;

        let mut new_mainfest = String::new();
        for unit in &self._mainfest {
            new_mainfest += format!("{} {}{},", unit.1 .0, unit.1 .1, unit.0)
        }
        self._mainfest_cache = new_mainfest
    }
}

fn main() {
    println!("Hello, world!");
}
