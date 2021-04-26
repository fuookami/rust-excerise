extern crate quick_xml;
extern crate serde;

use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize, PartialEq)]
struct Contact {
    name: String,
    street: String,
    city: String,
    state: String,
    zip: u64,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Address {
    #[serde(rename = "contact", default)]
    contacts: Vec<Contact>,
}

fn read_file(path: &str) -> String {
    let path = Path::new(path);
    let display = path.display();

    let mut fin = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(fin) => fin,
    };

    let mut s = String::new();
    match fin.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {}
    };

    s
}

fn main() {
    let address: Address = match from_str(&read_file("address.xml")) {
        Result::Ok(v) => v,
        Result::Err(e) => {
            println!("{}", e.to_string());
            return;
        }
    };

    println!("Live in Palmdale: ");
    for contact in &address.contacts {
        if contact.city == "Palmdale" {
            println!("{} {}", contact.name, contact.street);
        }
    }
    println!("");

    println!("ZIP between 90210 to 90214: ");
    for contact in &address.contacts {
        if (90210..=90214).contains(&contact.zip) {
            println!("{}", contact.street);
        }
    }
}
