
use std::io;
// check
fn main() {
    println!("Hi Mani Welcome to Learn Rust in Intellij , It is really perfect ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read Line");
    println!("You guessed:{}",guess);


    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let  name="Mani";
    println!("The value of name is: {}", name);
    println!("{}", name);
}

