pub fn run() {
    //primitive and Growable
    let _immutable_string = "Immutable String";

    let mut mutable_string = String::from("Mutable String");

    //push to mutable 'char'
    mutable_string.push('i');

    //push string
    mutable_string.push_str("s pushed here");

    println!("{}", mutable_string.chars().rev().collect());
}