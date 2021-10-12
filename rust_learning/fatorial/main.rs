fn main() {
	let input: i32 = 5;
	println!("{}",fatorial(input));
}

fn fatorial(i: i32) -> i32 {
	return if i == 0 {1} else {i*fatorial(i-1)};
}