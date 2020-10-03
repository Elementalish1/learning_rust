pub struct Rectangle {
    length: u32,
    width: u32
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    pub fn perimiter(&self) -> u32 {
        (self.length * 2) + (self.width * 2)
    }

    pub fn area(&self) -> u32 {
        self.length * self.width
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {length: 8, width: 9};
        let smaller = Rectangle {length: 4, width: 5};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {length: 8, width: 9};
        let smaller = Rectangle {length: 4, width: 5};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn perimiter() {
        let rectangle1 = Rectangle {length: 3, width: 2};
        assert_eq!(rectangle1.perimiter(), 10);

        let rectangle2 = Rectangle {length: 5, width: 5};
        assert_eq!(rectangle2.perimiter(), 20);
    }

    #[test]
    fn area() {
        let rectangle1 = Rectangle {length: 3, width: 2};
        assert_eq!(rectangle1.area(), 6);

        let rectangle2 = Rectangle {length: 5, width: 5};
        assert_eq!(rectangle2.area(), 25);
    }
}
