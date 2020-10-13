mod login;
pub use login::*;

#[derive(serde::Deserialize, Debug)]
pub struct AuthRequest {
    code: String,
    state: String,
    scope: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct UserInfo {
    email: Option<String>,
    hd: Option<String>,
    id: Option<String>,
    locale: Option<String>,
    name: Option<String>,
    picture: Option<String>,
    verified_email: Option<bool>,
}
impl From<UserInfo> for crate::routes::UserNew {
    fn from(info: UserInfo) -> crate::routes::UserNew {
        crate::routes::UserNew {
            google_id: info
                .id
                .as_ref()
                .unwrap()
                .parse::<i128>()
                .expect(&format!("Failed to parse {} as i64", info.id.unwrap())),
            name: info.name.unwrap(),
            phone_number: String::new(),
            email: info.email.unwrap(),
        }
    }
}
