fn main() {
    println!("Learning Rust and getting Rusty!");
    array_rust();
}

//array
fn array_rust() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let arr2 = [3; 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let house_names = [
        "Targaryen",
        "Lannister",
        "Stark",
        "Tyrell",
        "Baratheon",
        "Martell",
        "Tully",
        "Bolton",
        "Greyjoy",
        "Arryn",
        "Frey",
        "Mormont",
        "Tarley",
        "Dayne",
        "Umber",
        "Valeryon",
        "Manderly",
        "Clegane",
        "Glover",
        "Karstark",
    ];

    //accessing array elements
    println!("{}", months[0]);
    println!("{}", months[1]);
    println!("{}", arr2[2]);
    println!("{}", arr[3]);

    println!("{}", house_names[0]);

    let message = "The temperature today is:";
    let x = [message; 100];
    println!("{} {}", x[0], x[1]);
}

// fn tuple_list() {
//     let tuple = (1, 7.9, true, 's', "Solana", 25);

//     //destructuring tuple
//     let (one, two, three, four, five, six) = tuple;
//     println!("{one}, {two}, {three}, {four}, {five}, {six}");

//     //access elements directly using period (dot) notation
//     println!("The first element of the tuple is: {}", tuple.0);
//     println!("The second element of the tuple is: {}", tuple.1);
//     println!("The third element of the tuple is: {}", tuple.2);
//     println!("The fourth element of the tuple is: {}", tuple.3);
//     println!("The fifth element of the tuple is: {}", tuple.4);
//     println!("The sixth element of the tuple is: {}", tuple.5);

//     //empty tuple is also called a unit and programs explicitly return a unit type
//     // when they don't return any value
//     let mut empty_tup: (u128, bool, &str, char, f64);
//     empty_tup = (26, true, "Bitcoin", 'B', 7.9);
//     empty_tup.2 = "Ethereum";

//     println!("The empty tuple is: {}", empty_tup.2);
//     let empty_tuple = (1, 6, false);
//     println!("The empty tuple is: {}", empty_tuple.2);

//     //mutable tuple
//     let mut mutable_tuple = (8, true, "ethereum", 'R', 3.6);

//     mutable_tuple.3 = 'N';
//     mutable_tuple.0 += 56;

//     println!("The mutable tuple is: {}", mutable_tuple.3);
//     println!("The mutable tuple is: {}", mutable_tuple.0)
// }

// fn char_type() {
//     let c = 'c';
//     println!("The value of c is: {c}");
// }

// fn number_operation() {
//     let sum = 5 + 12;

//     let subtraction = 98.9 - 4.8;

//     let product = 6 * 7;

//     let qoutient = 17 / 3;

//     let remainder = 76 % 3;

//     println!(
//         "Sum: {sum}, Subtraction: {subtraction}, Product: {product}, Qoutient: {qoutient}, Remainder: {remainder}"
//     );
// }

// fn my_function() {
//     let mut x = 7;
//     println!("The value of x is: {x}");
//     x = 9;
//     println!("The value of x is: {x}");

//     //constants
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}s");
// }

// fn shadow_function() {
//     let x = 9;

//     {
//         let y = 9;
//         println!("The value of y is: {y}");
//     }

//     {
//         let x = 17;
//         println!("The value of x in inner scope is: {x}");
//     }
//     println!("The value of x is: {x}");

//     let mut age = 30;
//     println!("The value of age is: {age}");
//     age = 43;
//     println!("The value of age is: {age}");
//     //mut allow to change the value but not the type
//     //shadowing allow to change the type and value
//     let age = "56";
//     println!("The value of age is: {age}");

//     let age = true;
//     println!("The value of age is: {age}");

//     let age = 31.8;
//     println!("The value of age is: {age}");
// }
