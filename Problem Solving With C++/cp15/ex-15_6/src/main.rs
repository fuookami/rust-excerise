struct Screen {
    screen: Vec<Vec<bool>>,
}

impl Screen {
    fn draw(&mut self, x: usize, y: usize) {
        self.screen[x][y] = true;
    }

    fn erase(&mut self, x: usize, y: usize) {
        self.screen[x][y] = false;
    }
}

trait Figure {
    fn screen(&self) -> &Screen;
    fn draw(&mut self);
    fn erase(&mut self);
}

struct Point<'a> {
    _screen: &'a mut Screen,
    x: usize,
    y: usize,
}

impl<'a> Figure for Point<'a> {
    fn screen(&self) -> &Screen {
        self._screen
    }

    fn draw(&mut self) {
        self._screen.draw(self.x, self.y);
    }

    fn erase(&mut self) {
        print!("Erase point ({}, {})", self.x, self.y);
    }
}

struct Rectangle<'a> {
    _screen: &'a mut Screen,
    center: Point<'a>,
    length: usize,
    width: usize,
}

impl<'a> Figure for Rectangle<'a> {
    fn screen(&self) -> &Screen {
        self._screen
    }

    fn draw(&mut self) {
        for i in (self.center.x - self.length / 2)..=(self.center.x + self.length / 2) {
            self._screen.draw(i, self.center.y - self.width / 2);
            self._screen.draw(i, self.center.y + self.width / 2);
        }
        for i in (self.center.y - self.width / 2)..=(self.center.y + self.width / 2) {
            self._screen.draw(self.center.x - self.length / 2, i);
            self._screen.draw(self.center.x + self.length / 2, i);
        }
    }

    fn erase(&mut self) {
        for i in (self.center.x - self.length / 2)..=(self.center.x + self.length / 2) {
            self._screen.erase(i, self.center.y - self.width / 2);
            self._screen.erase(i, self.center.y + self.width / 2);
        }
        for i in (self.center.y - self.width / 2)..=(self.center.y + self.width / 2) {
            self._screen.erase(self.center.x - self.length / 2, i);
            self._screen.erase(self.center.x + self.length / 2, i);
        }
    }
}

struct Triangle<'a> {
    _screen: &'a mut Screen,
    a: Point<'a>,
    b: Point<'a>,
    c: Point<'a>,
}

impl<'a> Figure for Triangle<'a> {
    fn screen(&self) -> &Screen {
        self._screen
    }

    fn draw(&mut self) {
        for i in self.a.x..=self.b.x {
            self._screen.draw(i, Self::y(i, &self.a, &self.b));
        }
        for i in self.a.x..=self.c.x {
            self._screen.draw(i, Self::y(i, &self.a, &self.c));
        }
        for i in self.b.x..=self.c.x {
            self._screen.draw(i, Self::y(i, &self.b, &self.c));
        }
    }

    fn erase(&mut self) {
        for i in self.a.x..=self.b.x {
            self._screen.erase(i, Self::y(i, &self.a, &self.b));
        }
        for i in self.a.x..=self.c.x {
            self._screen.erase(i, Self::y(i, &self.a, &self.c));
        }
        for i in self.b.x..=self.c.x {
            self._screen.erase(i, Self::y(i, &self.b, &self.c));
        }
    }
}

impl<'a> Triangle<'a> {
    fn y(x: usize, p1: &Point<'a>, p2: &Point<'a>) -> usize {
        let p = (p2.x - p1.x) as f64 / (x - p1.x) as f64;
        ((p2.y - p1.y) as f64 * p).ceil() as usize + p1.y
    }
}

fn main() {
    println!("Hello, world!");
}
