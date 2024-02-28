#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    pub fn area(&self) -> f64 {
        ((self.p1.x - self.p2.x) * (self.p1.y - self.p2.y)).abs()
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * ((self.p1.x - self.p2.x).abs() + (self.p1.y - self.p2.y).abs())
    }

    // require the instance of Rectangle to be mutable.
    pub fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p1.y += y;
        self.p2.x += x;
        self.p2.y += y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        let mut rect = Rectangle {
            p1: Point::new(0.0, 0.0),
            p2: Point::new(10.0, 10.0),
        };

        assert_eq!(rect.area(), 100.0);
        assert_eq!(rect.perimeter(), 40.0);

        rect.translate(1.0, 1.0);
        assert_eq!(rect.area(), 100.0);
    }
}
