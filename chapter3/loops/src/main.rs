// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;
//         println!("the counter: {counter}");
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is: {result}");
// }

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}