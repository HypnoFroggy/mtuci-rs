
fn main() {}

struct Rect {
    // TODO
    Top_left: (f32, f32),
    Width: f32
}

impl Rect {
    fn new(top_left: (f32, f32), width: f32) -> Self {
        Rect {
            Top_left: top_left,
            Width: width
        }
    }

    fn bottom_right(&self) -> (f32, f32) {
        (self.Top_left.0 + self.Width, self.Top_left.1 - self.Width)
    }

    fn area(&self) -> f32 {
        self.Width * self.Width
    }

    fn perimeter(&self) -> f32 {
        self.Width * 4.0
    }
}


// ----> TESTS
#[cfg(test)]
mod tests {
    use crate::Rect;

    #[test]
    fn bottom_right() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!((6., -3.), rect.bottom_right())
    }

    #[test]
    fn area() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(25., rect.area())
    }

    #[test]
    fn perimeter() {
        let rect = Rect::new((1., 2.), 5.);

        assert_eq!(20., rect.perimeter())
    }
}