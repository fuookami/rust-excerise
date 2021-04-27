use std::collections::BTreeMap;

fn count(context: &str) -> BTreeMap<char, usize> {
    let mut counter = BTreeMap::new();
    for ch in context.chars() {
        if ch.is_alphabetic() {
            *counter.entry(ch.to_ascii_lowercase()).or_insert(0) += 1;
        }
    }
    counter
}

fn main() {
    assert_eq!(count("Eleven plus two"), count("Twelve plus one"));
}
