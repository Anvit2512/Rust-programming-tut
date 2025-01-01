fn main() {
    let x=4;
    println!("x is: {}", x);
    let mut y=5;
    println!("y is: {}", y);
    y=6;
    println!("y is: {}", y);

    let z=5;
    println!("z is: {}", z);
    let z=6;
    println!("z is: {}", z);

    {
        let x=2;
        println!("x is: {}", x);
    }

    const SECONDS_IN_MINUTE: u32=60;
    println!("{}",SECONDS_IN_MINUTE);
}
