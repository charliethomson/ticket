// Requires the the `ValidateForm` is named `body`
#[macro_export]
macro_rules! validate_ok {
    ($body:expr, $if_ok:block) => {
        if let Err(e) = $body.validate() {
            println!("{:?}", e);
            HttpResponse::BadRequest().json(OkMessage {
                ok: false,
                message: Some(
                    e.iter()
                        .fold(vec!["Validation Errors: ".to_owned()], |mut acc, cur| {
                            acc.push(format!("{:?}", cur));
                            acc
                        })
                        .join(", "),
                ),
            })
        } else {
            $if_ok
        }
    };
}
#[macro_export]
macro_rules! check_logged_in {
    ($identity:expr, $if_ok:block) => {
        // if $identity.identity() == None {
        //     HttpResponse::Unauthorized().finish()
        // } else {
        //     let _user_id: i64 = $identity
        //         .identity()
        //         .unwrap()
        //         .parse()
        //         .expect("Failed to parse user id");
        $if_ok
        // }
    };
}

#[macro_export]
macro_rules! ok {
    ($message:expr) => {
        OkMessage {
            ok: true,
            message: Some($message),
        }
    };
    ($message:expr, $msgty:ty) => {
        OkMessage::<$msgty> {
            ok: true,
            message: Some($message),
        }
    };
    () => {
        OkMessage::<()> {
            ok: true,
            message: None,
        }
    };
}

#[macro_export]
macro_rules! not_ok {
    ($message:expr) => {
        OkMessage {
            ok: false,
            message: Some($message),
        }
    };
}
