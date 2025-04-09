// use std::io;

fn main() {
    println!("Ownership in Rust!");

    // let mut user_input: String = String::new();
    // io::stdin()
    //     .read_line(&mut user_input)
    //     .expect("failed to readline");

    // let x = user_input.trim();
    // println!("{x}");

    // let a: bool = true;
    // read(a);
    let a = Box::new(18);

    let b = a;
    println!("{b}");

    println!("value plus one {}", plus_one(3));
}

// fn read(y: bool) {
//     if y {
//         println!("Alice is reading");
//     } else {
//         println!("Alice is not reading");
//     }
// }

fn plus_one(x: i64) -> i64 {
    x + 1
}
