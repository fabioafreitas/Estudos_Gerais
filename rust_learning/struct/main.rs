struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Color{ red: 255, green: 254, blue: 253 };

    bg.red = 1;

    println!("{} {} {}",bg.red, bg.green, bg.blue);
}