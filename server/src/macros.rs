// Requires the the `ValidateForm` is named `body`
#[macro_export]
macro_rules! validate_ok {
    ($body:expr, $if_ok:block) => {
        if let Err(e) = $body.validate() {
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
        if $identity.identity() == None {
            HttpResponse::Unauthorized().finish()
        } else {
            $if_ok
        }
    };
}
