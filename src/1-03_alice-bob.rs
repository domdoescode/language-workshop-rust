fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("What is your name? ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    if s == "Bob" || s == "Alice" {
        print!("Hello {}!", s);
    } else {
        println!("I only say hello to Bob or Alice.");
    }
}
