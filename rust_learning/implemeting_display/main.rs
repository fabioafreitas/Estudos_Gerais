use std::fmt;

struct Rectangle {
    width: u32,
    height: u32
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle {}x{}\n", self.width, self.height)
    }
}

fn main() {
    let rectangle = Rectangle{width: 10, height: 20};
    let square = Rectangle{width: 20, height: 20};
    
    println!("{}{}", rectangle, square);
}