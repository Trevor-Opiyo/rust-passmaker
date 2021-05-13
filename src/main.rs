use std::io;
use rand::seq::SliceRandom;

fn main() {
    let mut options = Vec::new();
    let mut lower_case = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut upper_case = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut integers = vec!['0','1','3','4','5','6','7','8','9'];
    let mut special_characters = vec!['!','@','#','$','^','&','*','(',')','_','-','+','=','<','>','?'];
    let password_length;

    header();

    if prompt("lower-case letters") == true {
        options.append(&mut lower_case)
    }
    if prompt("upper-case letters") == true {
        options.append(&mut upper_case)
    }
    if prompt("integers") == true {
        options.append(&mut integers)
    }
    if prompt("special characters") == true {
        options.append(&mut special_characters)
    }

    password_length = get_length();

    let password: Vec<_> = options
        .choose_multiple(&mut rand::thread_rng(), password_length)
        .collect();
    println!("{:?}", password);
}

fn header() {
    println!("\n------------------");
    println!("Password Generator");
    println!("------------------");
}

fn prompt(char_type: &str)-> bool {
    println!("\nInclude {}? [Y/N]\n", char_type);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
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
            return false;
        }
    }
}

fn get_length()-> usize {
    println!("Enter an integer length for the password. ex: \'8\'");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    let answer: usize = answer.trim().parse().unwrap();
    return answer;
}
