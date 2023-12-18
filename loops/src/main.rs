fn main() {
    let mut number = 3;
    while number != 0  {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");

    loop_through_collection();
    loop_with_iterator();
    loop_in_range();
}

fn loop_through_collection () {
    let a= [10, 20, 30, 40 , 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}

fn loop_with_iterator() {
    let a = [10, 20, 30, 40 , 50];
    for element in a.iter(){
        println!("The value is: {}", element);
    }
}

fn loop_in_range() {
    // this is reversed
    for number in (0..4).rev() {
        println!("{}", number);
    }
    println!("done");
}