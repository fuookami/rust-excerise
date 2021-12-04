trait Figure {
    fn draw(&self);
    fn erase(&self);
}

struct Point {
    x: i64,
    y: i64,
}

impl Figure for Point {
    fn draw(&self) {
        print!("Draw point ({}, {})", self.x, self.y);
    }

    fn erase(&self) {
        print!("Erase point ({}, {})", self.x, self.y);
    }
}

struct Rectangle {
    center: Point,
    length: u64,
    width: u64,
}

impl Figure for Rectangle {
    fn draw(&self) {
        print!(
            "Draw rectangle (({}, {}), {}, {})",
            self.center.x, self.center.y, self.length, self.width
        );
    }

    fn erase(&self) {
        print!(
            "Erase rectangle (({}, {}), {}, {})",
            self.center.x, self.center.y, self.length, self.width
        );
    }
}

struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Figure for Triangle {
    fn draw(&self) {
        print!(
            "Draw triangle (({}, {}), ({}, {}), ({}, {}))",
            self.a.x, self.a.y, self.b.x, self.b.y, self.c.x, self.c.y
        );
    }

    fn erase(&self) {
        print!(
            "Erase triangle (({}, {}), ({}, {}), ({}, {}))",
            self.a.x, self.a.y, self.b.x, self.b.y, self.c.x, self.c.y
        );
    }
}

fn main() {
    println!("Hello, world!");
}
