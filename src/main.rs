use std::io;

fn main() {
    let mut pass = Vec::new();
    header();
}

fn header() {
    println!("------------------");
    println!("Password Generator");
    println!("------------------");
}

fn prompt(charType: &str, answer: &str)-> bool {
    println!("Include " + "{}" + "? [Y/N]", charType)
    let mut answer = String::new(); 
    io::stdin().read_line(&mut answer);
    let answer: String = answer.trim().parse().unwrap();
    match answer {
        "Y" | "y" | "YES" | "Yes" | "yes" => return true,
        "N" | "n" | "NO" | "No" | "no" => return false,
        _ => println!("Please enter valid input. [Y/N]")
    }
}
