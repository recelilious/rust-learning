fn main() {
    let mut x = 1;
    println!("The value of x is {}", x);
    x = 2;
    println!("The value of x is {}", x);

    let y = 1;
    println!("The value of y is {}", y);

    let y = 2;
    println!("The value of y is {}", y);

    {
        let y = 3;
        println!("The value of y is {}", y);
    }

    println!("The value of y is {}", y);
}
