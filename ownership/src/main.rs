fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    //demo_scope_broken();
    demo_scope_working();

    let mut string_of_words = String::from("This is a test sentence");
    let word = first_word(&string_of_words);
    println!("the first word, was {}", &word);
    string_of_words.clear();
}

fn demo_scope_working(){
    let mut s = String::from("test string");
    my_print_working(&mut s);
}

//This is just to demonstrate what will not work
//fn demo_scope_broken(){
//    let s = String::from("hello");
//    my_print(s)
//}

fn my_print_broken(s: String) {
    println!("{}", s)
}

fn my_print_working(some_string: &mut String){
    println!("{}", some_string)
}

//fn dangling_refernce(){
//    let reference_to_nothing = dangle();
//}

//fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
//}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); // this converts our string to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
