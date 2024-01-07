pub fn run() {
    //primitive and Growable
    let immutable_string = "immutable String";

    let growable_string = String::from("growable string");

    let mut growable_mutable_string = String::from("growable Mutable strin");

    //string length
    let length_growable = growable_string.len();

    //push to growable string, must be mutable
    growable_mutable_string.push('g');

    println!("{:?}\n", (immutable_string, growable_string));
    println!("Length of Growable: {}, {:4}", length_growable, growable_mutable_string);
}