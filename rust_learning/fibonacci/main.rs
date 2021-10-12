use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].parse::<i32>().unwrap();
    println!("{}", fib(input));
}

fn fib(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    return fib(x-1) + fib(x-2)
}