
use std::io;
// check
fn main() {
    println!("Hi Mani Welcome to Learn Rust in Intellij , It is really perfect ");
    let mut guess = String::new();
    let mut _second_guess = String::new();
    let mut guess_again = String::new();
    io::stdin().read_line(&mut guess_again).expect("Failed to read Line");
    println!("You guessed:{}", guess_again);




    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let  name="Mani";
    println!("The value of name is: {}", name);
    println!("{}", name);


    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


}
//
// fn main() {
//     println!("Guess the number!");
//
//     println!("Please input your guess.");
//
//     let mut guess = String::new();
//
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//
//     println!("You guessed: {}", guess);
// }