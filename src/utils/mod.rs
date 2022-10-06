use regex::Regex;
use serde_json::Value;

pub fn get_valid_emails(user_emails: Vec<Value>) -> Vec<Value> {
    let email_regex = Regex::new(r"[\w._%+-]+@[\w.-]+\.[a-zA-Z]{2,3}").unwrap();
    let mut valid_emails = vec![];
    for email in user_emails {
        if email_regex.is_match(&email.to_string()) {
            valid_emails.push(email);
        }
    };
    return valid_emails;
}