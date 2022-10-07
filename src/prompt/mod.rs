extern crate rpassword;
    
use rpassword::read_password;
use std::io::{Write, self};
    
pub fn take_user_input() {

    print!("Enter authorizer url: ");
    io::stdout().flush().unwrap();

    let mut authorizer_url = String::new();
    io::stdin().read_line(&mut authorizer_url).unwrap();

    let url = authorizer_url.trim();

    print!("Enter your admin secret: ");
    std::io::stdout().flush().unwrap();
    let admin_secret = read_password().unwrap();

    let secret = admin_secret.trim();

    println!("url ==>> {}", url);

    println!("secret ==>> {}", secret);
}