fn main() {
    let mut x: i32 = 10;
    println!("{}", &x);
    foo(&mut x);
    println!("{}", x);
}

fn foo(val: &mut i32) {
    *val += 1;
}