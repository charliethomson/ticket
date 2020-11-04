mod login;
pub use login::*;

use {
    crate::{
        db::types::User,
        not_ok, ok,
        routes::{api::users::UserNew, OkMessage},
    },
    webforms::validate::ValidateForm,
};

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

pub(in crate::routes::auth) async fn create_new_user_internal(body: UserNew) -> OkMessage<String> {
    if let Err(errors) = body.validate() {
        not_ok!(errors
            .iter()
            .fold(vec!["Validation Errors: ".to_owned()], |mut acc, cur| {
                acc.push(format!("{:?}", cur));
                acc
            })
            .join(", "))
    } else {
        match User::insert(body) {
            Ok(id) => ok!(id.to_string()),
            Err(e) => not_ok!(e.to_string()),
        }
    }
}
