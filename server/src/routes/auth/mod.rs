mod login;
pub use login::*;

use crate::db::UserNew;

#[derive(serde::Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct UserInfo {
    email: Option<String>,
    hd: Option<String>,
    id: Option<String>,
    locale: Option<String>,
    name: Option<String>,
    picture: Option<String>,
    verified_email: Option<bool>,
}
impl From<UserInfo> for UserNew {
    fn from(info: UserInfo) -> UserNew {
        let mut name_parts = info.name.as_ref().unwrap().split_whitespace();
        UserNew {
            google_id: Some(
                info.id
                    .as_ref()
                    .unwrap()
                    .parse::<i128>()
                    .unwrap_or_else(|_| {
                        panic!("Failed to parse {} as i64", info.id.as_ref().unwrap())
                    })
                    // TODO: Be
                    .to_be_bytes()
                    .to_vec(),
            ),
            portal_id: None,
            first_name: name_parts.next().unwrap().into(),
            last_name: name_parts.next().unwrap().into(),
            email_address: info.email.unwrap(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UserPhoneNumber {
    value: String,
}
