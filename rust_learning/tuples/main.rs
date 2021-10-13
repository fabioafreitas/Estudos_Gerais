fn main() {
    let tup1 = (1, 5, 4);
    println!("{}", tup1.0);
    
    let tup2 = (1, "a", false, (1, 2 ,3));
    println!("{:?}", tup2.3);
    
    let (x,y,z) = tup1;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
}