use std::io::{stdin, stdout, Write};

fn main() {
    let mut input_text = String::new();
    print!("What number do you want to sum? ");
    let _ = stdout().flush();

    stdin()
        .read_line(&mut input_text)
        .expect("Did not enter a correct input");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("The result is {}", sum(i)),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

fn sum(int: u32) -> u32 {
    let mut counter = 0;
    let mut total = 0;

    loop {
        counter += 1;
        total += counter;

        if counter == int {
            break;
        }
    }

    return total;
}
