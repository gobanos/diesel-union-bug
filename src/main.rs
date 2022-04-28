mod models;
mod schema;

use crate::models::User;
use crate::schema::users;
use diesel::prelude::*;
use std::error::Error;

#[cfg(feature = "postgres")]
type Conn = PgConnection;

#[cfg(feature = "mysql")]
type Conn = MysqlConnection;

#[cfg(feature = "sqlite")]
type Conn = SqliteConnection;

fn main() -> Result<(), Box<dyn Error>> {
    let db_url = std::env::var("DATABASE_URL")?;
    let conn = &mut Conn::establish(&db_url)?;

    let users = users::table
        .order_by(users::id.asc())
        .limit(1)
        .union(users::table.order_by(users::id.desc()).limit(1))
        .load::<User>(conn)?;

    assert_eq!(
        users,
        vec![
            User {
                id: 1,
                email: "foo@mail.com".to_string()
            },
            User {
                id: 3,
                email: "foobar@mail.com".to_string()
            }
        ]
    );

    Ok(())
}
