use std::io;

fn main() {
    println!("What is the quote?");
    let mut quote = String::new();
    io::stdin()
        .read_line(&mut quote)
        .expect("Failed to read line");
    let quote = quote.trim();

    println!("Who said it?");
    let mut quote_author = String::new();
    io::stdin()
        .read_line(&mut quote_author)
        .expect("Failed to read line");
    let quote_author = quote_author.trim();

    // using string interpolation:
    // println!("{quote_author} says, \"{quote}.\"")

    // using string concatenation
    let result = String::from(quote_author) + " says, \"" + quote + ".\"";
    println!("{}", result);
}
