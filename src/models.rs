use crate::schema::*;
use diesel::{Identifiable, Insertable, Queryable, QueryableByName};

#[derive(Debug, Clone, Identifiable, Queryable, QueryableByName, Insertable, PartialEq, Eq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub email: String,
}
