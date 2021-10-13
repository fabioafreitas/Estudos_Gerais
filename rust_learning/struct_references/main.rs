struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let mut bg = Color{ red: 255, green: 254, blue: 253 };
    println!("{} {} {}",bg.red, bg.green, bg.blue);
    change_color_to_black(&mut bg);
    println!("{} {} {}",bg.red, bg.green, bg.blue);
}

fn change_color_to_black(color: &mut Color) {
    (*color).red = 0;
    (*color).green = 0;
    (*color).blue = 0;
}