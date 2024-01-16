use std::mem;

pub fn run() {
    //same data type elements

    let prime_nums: [i32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

    println!("{:?}", prime_nums);

    let mut prime_nums_mut: [i32; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

    //change, cant add values
    prime_nums_mut[2] = 29;

    //size is calculated by passing a reference of mutable primnums array
    println!("Mutable Array has a length of {} and uses {} bytes of memory", prime_nums_mut.len(), mem::size_of_val(&prime_nums_mut));

    //slice
    let slice: &[i32] = &prime_nums_mut[3..4];

    println!("Slices {:?}", slice);
}