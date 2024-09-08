use std::io;

fn to_title_case(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn main() {
    println!("What is your name?");
    let mut buf = String::new();
    let _ = io::stdin().read_line(&mut buf);
    let name: &String = &String::from(buf.trim());
    let lowercase_name = name.to_string().to_lowercase();
    if lowercase_name == "saba" {
        println!("Hello {}, we meet again!", to_title_case(name));
    } else if lowercase_name == "blupus" {
        println!("Hello duffer! You're a good cat!");
    } else {
        println!("Hello {}, nice to meet you!", to_title_case(name));
    }
}
