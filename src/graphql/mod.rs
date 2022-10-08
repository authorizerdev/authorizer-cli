use std::borrow::Cow;

use reqwest::Client;
use serde_json::{Value, Map};

use crate::utils::get_valid_emails;

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

fn convert<'a>(s: &'a Cow<'_, str>) -> &'a str {
    s.as_ref()
}

pub async fn send_invitation(url: &str, secret: &str, user_emails: Vec<Value>, is_debug_mode_on: bool) -> Result<()> {
    if is_debug_mode_on {
        println!("-> input user emails: {:?}", user_emails);
    }
    const SEND_INVITAION_MUTATION: &str = "mutation inviteMembers($params: InviteMemberInput!) {\n  _invite_members(params: $params) {\n    message\n    __typename\n  }\n}";
    let client = Client::new();
    let mut map = Map::new();
    let mut params_map = Map::new();
    let mut emails_map = Map::new();
    let valid_emails = get_valid_emails(user_emails);
    if is_debug_mode_on {
        println!("-> valid user emails: {:?}", valid_emails);
    }
    if valid_emails.len() == 0 {
        Err("Uploaded list is empty!")?;
    }
    emails_map.insert("emails".to_string(), Value::Array(valid_emails));
    params_map.insert("params".to_string(), Value::Object(emails_map));
    map.insert("query".to_string(), Value::String(SEND_INVITAION_MUTATION.to_string()));
    map.insert("variables".to_string(), Value::Object(params_map));
    let req = client
        .post(url)
        .header("x-authorizer-admin-secret", secret)
        .json(&map);

    let res = req.send().await?;
    if is_debug_mode_on {
        println!("-> request status: {}", res.status());
    }
    let body = res.bytes().await?;
    let v = body.to_vec();
    let s = String::from_utf8_lossy(&v);
    let str_v: Value = serde_json::from_str(convert(&s))?;
    if is_debug_mode_on {
        println!("-> response: {} ", s);
    }
    if !str_v["data"]["_invite_members"]["message"].is_null() {
        println!("Success: {}", str_v["data"]["_invite_members"]["message"]);
    } else if !str_v["errors"][0]["message"].is_null() {
        println!("Error: {}", str_v["errors"][0]["message"]);
    }
    Ok(())
}