fn main() {
    let input = 10;
    for number in 0..(input+1) {
        let mut aux = "".to_string();
        let mut print_text = false;
        if number % 3 == 0{
            aux.push_str("fizz");
			print_text = true;
        }
        if number % 5 == 0{
            aux.push_str("buzz");
			print_text = true;
        }

        if print_text {
            println!("{}",aux);
        } else {
            println!("{}",number);
        }
    }
}