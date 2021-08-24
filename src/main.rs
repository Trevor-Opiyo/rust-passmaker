use rand::seq::SliceRandom;
use std::io;

fn main() {
    let mut options: Vec<char> = Vec::new();

    header();

    if prompt("lower-case letters") == true {
        options.extend_from_slice(&[
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ])
    }
    if prompt("upper-case letters") == true {
        options.extend_from_slice(&[
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ])
    }
    if prompt("integers") == true {
        options.extend_from_slice(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'])
    }
    if prompt("special characters") == true {
        options.extend_from_slice(&['!', '@', '#', '$', '%', '^', '&', '*', '-', '_', '+'])
    }

    let password_length: usize = get_length();
    generate_password(options, password_length)
}

fn header() {
    println!("\n--------------");
    println!("rust-passmaker");
    println!("--------------");
}

fn prompt(char_type: &str) -> bool {
    loop {
        println!("\nInclude the posssibility of {} in the password? [Y/N]\n", char_type);
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).ok();
        let answer: String = answer.trim().parse().unwrap();
        match answer.as_str() {
            "Y" | "y" | "YES" | "Yes" | "yes" => {
                return true;
            }
            "N" | "n" | "NO" | "No" | "no" => {
                return false;
            }
            _ => {
                println!("\nPlease enter valid input. ex: \'Y\' or \'N\'");
            }
        }
    }
}

fn get_length() -> usize {
    println!("\nEnter an integer length for the password. [8]\n");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).ok();
    let answer: usize = answer.trim().parse().unwrap();
    return answer;
}

fn generate_password(password_options: Vec<char>, password_length: usize) {
    loop {
        let password: Vec<_> = password_options
            .choose_multiple(&mut rand::thread_rng(), password_length)
            .collect();
        print!("\n");
        for index in password {
            print!("{}", index);
        }
        print!("\n");
        println!("\nWould you like to regenerate the password with the same parameters? [Y/N]?\n");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).ok();
        let answer: String = answer.trim().parse().unwrap();
        match answer.as_str() {
            "Y" | "y" | "YES" | "Yes" | "yes" => {}
            "N" | "n" | "NO" | "No" | "no" => {
                break;
            }
            _ => {
                println!("\nPlease enter valid input. ex: \'Y\' or \'N\'");
            }
        }
    }
}
