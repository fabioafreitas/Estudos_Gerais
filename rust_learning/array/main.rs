fn main() {
    let arr1: [i32; 5] = [1,2,3,4,5];
    let arr2: [i32; 5] = [1; 5];

    print_arr(&arr1);
    print_arr(&arr2);
}

fn print_arr(arr: &[i32]) {
    for elem in arr.iter() {
        print!("{}",elem);
    }
    println!();
}