use std::io;

fn main() {
    let x: i8=9;
    let y: i8=10;
    //let y=10.0f32;
    //let y=10__i8;
    //let y=10.0 as f32;

    let z=x+y;
    //let z=x as i32/y;
    //let z=x*y;
    //let z=x/y;
    //let z=x%y;
    //let z=x-y;
    println!("{}", z); 


    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("expected to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", int_input+2);


}
