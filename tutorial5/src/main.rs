fn main() {
    let cond= 2<3;
    println!("{}",cond);
    // && || !

    let cond2=false || cond;

    println!("{}", cond2);

    let food="cookie";

    if food=="cookie"{
        println!("I like cookies too!");
    }
    else if food=="bread"{
        println!("that sounds healthy!");
    }
    else{
        println!("oh, that's too bad");
    }
}
