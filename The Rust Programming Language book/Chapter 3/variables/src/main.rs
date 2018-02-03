fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    //x = 6;
    //^^^ here goes the error, because x is immutable
    println!("THe value of x is {}\n", x);

    let mut x = 5;
    println!("The value of mutable x is {}", x);
    x = 6;
    println!("The value of mutable x is {}\n", x);

    //But we can always shadow existing variable
    let x = 5;
    println!("The value of immutable x is {}", x);
    let x = 6;
    println!("The value of immutable, but shadowed x is {}\n", x);

}
