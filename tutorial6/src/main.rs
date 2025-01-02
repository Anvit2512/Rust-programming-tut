fn main() {
    println!("Hello, world!");
    test_one();
    add_numbers(20,30);


    let number={
        let x=3;
        x+1
    };
    println!("{}", number);

    let result=add_number(2,3);
    println!("{}",result);
}

fn test_one(){
    println!("Test has been called...");
}

fn add_numbers(x: i32, y: i32){
    println!("The sum is: {} ",x+y);
}

fn add_number(x:i32, y:i32) -> i32{
    x+y
    //return x+y;
}