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
        let mut name_parts = info.name.as_ref().unwrap().split_whitespace();
        crate::routes::UserNew {
            google_id: info
                .id
                .as_ref()
                .unwrap()
                .parse::<i128>()
                .unwrap_or_else(|_| panic!("Failed to parse {} as i64", info.id.as_ref().unwrap())),
            first_name: name_parts.next().unwrap().into(),
            last_name: name_parts.next().unwrap().into(),

            email: info.email.unwrap(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct UserPhoneNumber {
    value: String,
}
