extern crate rpassword;
    
use rpassword::read_password;
use std::io::{Write, self};
    
pub fn take_user_input() -> [String; 2] {

    print!("Enter authorizer url: ");
    io::stdout().flush().unwrap();

    let mut authorizer_url = String::new();
    io::stdin().read_line(&mut authorizer_url).unwrap();

    let mut url = authorizer_url.trim().to_owned();

    if url.to_string().chars().last().unwrap() == '/' {
        url.push_str("graphql")
    } else {
        url.push_str("/graphql")
    }

    print!("Enter your admin secret: ");
    std::io::stdout().flush().unwrap();
    
    let admin_secret = read_password().unwrap();
    let secret = admin_secret.trim();

    return [url.to_string(), secret.to_string()];
}