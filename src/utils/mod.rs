use regex::Regex;
use serde_json::Value;

pub fn get_valid_emails(user_emails: Vec<Value>) -> Vec<Value> {
    println!("user emails: {:?} ", user_emails);
    let email_regex = Regex::new(r"[\w._%+-]+@[\w.-]+\.[a-zA-Z]{2,3}").unwrap();
    let mut valid_emails = vec![];
    for email in user_emails {
        println!("email: {}, status: {}", &email.to_string(), email_regex.is_match(&email.to_string()));
        if email_regex.is_match(&email.to_string()) {
            valid_emails.push(email);
        }
    };
    println!("valid emails: {:?} ", valid_emails);
    return valid_emails;
}