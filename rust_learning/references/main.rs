fn main() {
    let mut x = 10;
    let y = &x;     // referencia imutável
    println!("{} {}",x, y);

    // mudando valores através do ponteiro
    {
        let p = &mut x; // referencia mutável
        *p = 30;
    }

    println!("{}", x);
}