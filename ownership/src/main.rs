// use std::io;

fn main() {
    println!("Ownership in Rust!");
    println!(
        "Ownership is a core Rust concept designed to ensure memory safety without relying on a garbage collector or extensive runtime checks. It’s a set of rules that governs how Rust manages memory, preventing issues like crashes, data corruption, or security vulnerabilities caused by undefined behavior.think of ownership as Rust’s way of enforcing strict rules at compile-time to avoid bugs, similar to how TypeScript prevents type errors before your JavaScript runs."
    );
    println!("Safety in rust is absence of undefined behavior");

    // make this program unsafe by moving the function before the definition of y
    let y: bool = true;
    read(y);
    read(y);
}

fn read(y: bool) {
    if y {
        println!("y is true");
    } else {
        println!("y iis false");
    }
}
