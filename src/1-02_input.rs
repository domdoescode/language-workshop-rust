fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("What is your name? ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    print!("Hello {}!", s);
}
