use std::io;

fn main() {
    let arr = [1,2,3,4,5];

    println!("Please enter an array of index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Enter the valid index");

    let element = arr[index];
    println!("The element {} and index {} of an array", element, index);
}