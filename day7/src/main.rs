use std::io::{self, Write};

fn main() -> Result<(), std::io::Error> {
    println!("in>");
    io::stdout().flush().expect("Failed to flush stdout.");
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    match input_text.trim().parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", input_text),
    };
    println!("Bye !");
    Ok(())
}
