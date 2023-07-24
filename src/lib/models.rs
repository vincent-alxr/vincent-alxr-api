use diesel::{prelude::*, FromSqlRow, AsExpression, sql_types::Text, deserialize::{FromSql, self}, pg::{self, Pg, PgValue}, serialize::{self, IsNull, ToSql, Output}, DieselNumericOps};
use rocket::{FromForm, FromFormField};
use serde::Serialize;
use crate::schema::sql_types;
use std::io::Write;



#[derive(Queryable, Selectable, Serialize, QueryableByName, PartialEq, Debug)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

use crate::schema::sql_types::Level;

#[derive(FromSqlRow, Debug, PartialEq, AsExpression, Eq, Serialize, FromFormField)]
#[diesel(sql_type = Level)]
pub enum LevelEnum {
    Beginner,
    Intermediate,
    Expert,
}


impl ToSql<Level, Pg> for LevelEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            LevelEnum::Beginner => out.write_all(b"beginner")?,
            LevelEnum::Intermediate => out.write_all(b"intermediate")?,
            LevelEnum::Expert => out.write_all(b"expert")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Level, Pg> for LevelEnum {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"beginner" => Ok(LevelEnum::Beginner),
            b"intermediate" => Ok(LevelEnum::Intermediate),
            b"expert" => Ok(LevelEnum::Expert),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

#[derive(Queryable, QueryableByName, Selectable, Insertable, Identifiable, PartialEq, Debug, Serialize)]
#[diesel(table_name = crate::schema::users_has_skills)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserHasSkill {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub level: LevelEnum,
}

#[derive(Serialize)]
pub struct UserWithSkills {
    pub user: User,
    pub skills: Vec<UserHasSkill>
}

