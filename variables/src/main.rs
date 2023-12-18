fn main() {
    let x = 5;
    let x = x + 1;

    let x = x * 2;
    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y , z) = tup;

    println!("The value of y is: {}",y);
    println!("The fallowing demonstrates acessing tuple by index");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("the first element is {}, the second one is {}, the third one is {}", tup.0, tup.2, tup.2);
    println!("you can also do five_hundred: {}, six_point_four {} and one: {}", five_hundred, six_point_four, one )
}