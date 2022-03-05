#[derive(PartialEq, Eq)]
struct Book {
    author: String,
    publication_date: String,
    name: String,
}

impl PartialOrd for Book {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Option::Some(self.author.cmp(&other.author))
    }
}

impl Ord for Book {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.author.cmp(&other.author)
    }
}

fn sort(books: &mut Vec<Book>) {
    books.sort()
}

fn main() {
    println!("Hello, world!");
}
