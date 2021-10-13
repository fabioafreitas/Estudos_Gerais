struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn print(&self) {
        println!("Rectangle {}x{}\n is_square {}\n",
            self.width,
            self.height, 
            self.is_square()
        );
    }

    fn is_square(&self) -> bool {
        return if self.height == self.width {true} else {false};
    }
}


fn main() {
    let rectangle = Rectangle{width: 10, height: 20};
    let square = Rectangle{width: 20, height: 20};
    
    rectangle.print();
    square.print();
}