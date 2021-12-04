
use std::io;
// check
fn main() {
    println!("Hi Mani Welcome to Learn Rust in Intellij , It is really perfect ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read Line");
    println!("You guessed:{}",guess)
}

