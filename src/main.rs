use std::io;
use rand::seq::SliceRandom;

fn main() {
    let mut options = Vec::new();
    let mut password_length: usize;
    let mut lower_case = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z'];
    let mut upper_case = vec!['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
    let mut integers = vec!['0','1','2','3','4','5','6','7','8','9'];
    let mut special_characters = vec!['!','@','#','$','^','&','*','(',')','_','-','+','=','<','>','?'];

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

    generate_password(options, password_length)
}

fn header() {
    println!("\n--------------");
    println!("rust-passmaker");
    println!("--------------");
}

fn prompt(char_type: &str)-> bool {
    loop {
        println!("\nInclude {}? [Y/N]\n", char_type);
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

fn get_length()-> usize {
    println!("\nEnter an integer length for the password. [8]\n");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).ok();
    let answer: usize = answer.trim().parse().unwrap();
    return answer;
}

fn generate_password<T>(password_options: Vec<T>, password_length: usize) {
    loop {
        let password: Vec<_> = password_options;
        password.choose_multiple(&mut rand::thread_rng(), password_length).collect();
        print!("\n");
        for index in password {
            print!("{}", index);
        }
        println!("\nWould you like to regenerate the password with the same parameters? [Y/N]?\n");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).ok();
        let answer: String = answer.trim().parse().unwrap();
        match answer.as_str() {
            "Y" | "y" | "YES" | "Yes" | "yes" => {}
            "N" | "n" | "NO" | "No" | "no" => {
                break;
            }
            _=> {
                println!("\nPlease enter valid input. ex: \'Y\' or \'N\'");
            }
        }
    }
}
