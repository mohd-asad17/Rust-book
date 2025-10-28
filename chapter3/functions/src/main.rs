fn main() {
    println!("Hello, world!");

    let x = five();
    let y = plus_one(5);
    println!("The value of x: {x}");
    println!("The value of y: {y}");
    another_function();
    another_parameter_func(5);
    print_labelled_measurement(5,'h');
}

fn another_function() {
     println!("Hello, world from another functions");
}

fn another_parameter_func(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labelled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    20
}

fn plus_one(x: i32) -> i32 {
    x + 1
}