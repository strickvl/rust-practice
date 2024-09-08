use std::io;

fn main() {
    loop {
        println!("What is the input string? ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        let char_count = input.chars().count();
        if char_count == 0 {
            eprintln!("You have to enter something!")
        } else {
            println!("{input} has {char_count} characters.");
            break;
        }
    }
}
