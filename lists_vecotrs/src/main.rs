use std::collections::HashMap;
fn main() {
    let v: Vec<i32> = Vec::new();

    let nigel_vector = vec![1,2,3];

    let mut nuts : Vec<i32>  = Vec::new();

    //you can push muteable elements
    nuts.push(1);
    nuts.push(2);
    nuts.push(3);
    let length = nuts.len();
    println!("{}", length);
    //let first: &i32 = &v[0];
    let second: Option<&i32> = v.get(1);


    //looping over a muteable vector
    for i in &mut nuts {
        println!("{}", i);
    }

    //looping over
    for i in &mut nuts {
        *i += 50;
    }

    // prints content of entire vector
    println!("{:?}", nuts);

    // creates a muteable string
    let mut s = String::new();

    let data = "data to load into string";

    let s = data.to_string();
    //more simple way to do this
    let my_string = String::from("fucking data in scring");

    // lets grow a string by appending a slice to the end
    let mut muteable_string = String::from("mine");
    muteable_string.push_str("bar");
    println!("{}", muteable_string);

    //use contacatination operation

    let s1 = String::from("Fuck");
    let s2 = String::from("you");

    let s3 = s1 + &s2;
    println!("{}", s3)

}

fn test_hash_map(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("red"), 12);


}