fn main() {
    shaddow_example();
    tupple_example();
    array_example();
}

fn shaddow_example() {
    println!("shaddow example");
    let x = 5;
    let x = x + 1;

    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn tupple_example() {
    println!("tupple example");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y , z) = tup;

    println!("The value of y is: {}",y);
    println!("The fallowing demonstrates acessing tuple by index");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    println!("the first element is {}, the second one is {}, the third one is {}", tup.0, tup.2, tup.2);
    println!("you can also do five_hundred: {}, six_point_four {} and one: {}", five_hundred, six_point_four, one );
}

fn array_example() {

    let test_array = [1,2,3,4,5];
    let first = test_array[0];
    let second = test_array[1];

    println!("one way first: {}, second: {}", first, second);
    println!("Another way third: {}, fourth: {}", test_array[3], test_array[4]);

}