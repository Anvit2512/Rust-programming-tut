fn main() {
    println!("Hello, world!");

    let x: i32=2;
    println!("x is: {}", x);

    let mut tup: (i32,bool,char)=(1, true, 's');
    let tup2: (i8,bool,char)=(1,true,'s');
    println!("{}",tup.1);
    println!("{}",tup2.0);
    tup.0=10;
    println!("{}",tup.0);


    let mut arr:[i32; 5]=[1,2,3,4,5];
    arr[4]=3;
    println!("{}",arr[4]);

    let x:u8 =4;
    let y=x;
    println!("{},{}",x,y);



}
