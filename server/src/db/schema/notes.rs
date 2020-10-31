use crate::db::{
    types::{Note, NoteTuple, NotesOptions},
    Options,
};
use mysql::{params, prelude::Queryable};

impl Note {
    pub fn insert(&self, workorder_id: i64) -> mysql::Result<i64> {
        let mut conn = crate::db::get_connection()?;
        conn.exec_drop(
            "insert into notes
        (   
            wo_key,
            contents,
            user,
            posted,
            next_update
        ) values (
            :wo_key,
            :contents,
            :user,
            :posted,
            :next_update
        );",
            params! {
                "wo_key" => workorder_id,
                "contents" => self.contents.clone(),
                "user" => self.user,
                "posted" => self.created,
                "next_update" => self.next_update
            },
        )?;

        Ok(conn
            .query_first::<i64, String>(
                "SELECT max(LAST_INSERT_ID(note_id)) FROM notes".to_owned(),
            )?
            .unwrap())
    }

    pub fn find(filter: NotesOptions) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let filter = filter.into_filter();
        let query = format!(
            "select note_id from notes{};",
            if !filter.is_empty() {
                format!(" where {}", filter)
            } else {
                "".to_string()
            }
        );
        let ids: Vec<i64> = conn.query(query)?;

        // TODO (this and also in Workorders and Users)
        let notes = ids
            .iter()
            .map(|id| Note::by_id(*id))
            .filter(|note| note.is_ok() && note.as_ref().unwrap().is_some())
            .map(|note| note.unwrap().unwrap())
            .collect::<Vec<Note>>();
        if !notes.is_empty() {
            Ok(Some(notes))
        } else {
            Ok(None)
        }
    }

    pub fn by_id(id: i64) -> mysql::Result<Option<Self>> {
        let mut conn = crate::db::get_connection()?;
        if let Some(note_tuple) = conn.query_first::<NoteTuple, String>(format!(
            "select contents, user, posted, next_update from notes where note_id={}",
            id
        ))? {
            Ok(Some(Note::from(note_tuple)))
        } else {
            Ok(None)
        }
    }

    pub fn all_for_wo(wo_id: i64) -> mysql::Result<Option<Vec<Self>>> {
        let mut conn = crate::db::get_connection()?;
        let ids: Vec<i64> =
            conn.query::<i64, String>(format!("select note_id from notes where wo_key={}", wo_id))?;

        Ok(Some(
            ids.iter()
                .map(|id| Note::by_id(*id))
                .filter(|note| note.is_ok() && note.as_ref().unwrap().is_some())
                .map(|note| note.unwrap().unwrap())
                .collect(),
        ))
    }
}
