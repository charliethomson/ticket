use {
    crate::{
        db::{
            types::{User, UserOptions, UserResponse, UserTuple},
            Options,
        },
        routes::users::UserNew,
    },
    mysql::{params, prelude::Queryable},
};

impl User {
    pub fn insert(user: UserNew) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        match conn.exec_drop(
            "insert into users (
                google_id,
                first_name,
                last_name,
                email
            ) values (
                :google_id,
                :first_name,
                :last_name,
                :email
            );",
            params! {
                "google_id" => user.google_id,
                "first_name" => user.first_name.clone(),
                "last_name" => user.last_name.clone(),
                "email" => user.email.clone(),
            },
        ) {
            Ok(_) => Ok(conn
                .query_first::<i64, String>("SELECT max(LAST_INSERT_ID(id)) FROM users".to_owned())?
                .unwrap()),
            // Duplicate user
            Err(mysql::Error::MySqlError(mysql::error::MySqlError { code: 1062, .. })) => {
                Ok(User::find(UserOptions {
                    google_id: Some(user.google_id),
                    ..UserOptions::default()
                })?
                // Unwraps are safe because of the error type, we _will_ find something
                // filtering by the (unique) google_id.
                .unwrap()
                .get(0)
                .unwrap()
                .id)
            }
            Err(e) => Err(e),
        }
    }

    pub fn find(filter: UserOptions) -> mysql::Result<Option<Vec<UserResponse>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let query = format!(
            "select id from users{};",
            if !filter.is_empty() {
                format!(" where {}", filter)
            } else {
                "".to_string()
            }
        );
        let ids: Vec<i64> = conn.query(query)?;

        // TODO (this and also in Workorders)
        let users = ids
            .iter()
            .map(|id| User::by_id(*id))
            .filter(|user| user.is_ok() && user.as_ref().unwrap().is_some())
            .map(|user| user.unwrap().unwrap().into())
            .collect::<Vec<UserResponse>>();
        if !users.is_empty() {
            Ok(Some(users))
        } else {
            Ok(None)
        }
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(user_tuple) = conn.query_first::<UserTuple, String>(format!(
            "select * from users where users.id={}",
            id
        ))? {
            Ok(Some(User::from(user_tuple)))
        } else {
            Ok(None)
        }
    }
}
