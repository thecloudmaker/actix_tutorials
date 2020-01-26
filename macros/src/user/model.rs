use crate::api_error::ApiError;
use crate::db;
use crate::db::LoadPaginated;
use crate::{filter, sort_by};
use crate::schema::user;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "user"]
pub struct UserMessage {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "user"]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct Params {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub sort_by: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "created_at[gte]")]
    pub created_at_gte: Option<NaiveDateTime>,
    #[serde(rename = "created_at[lte]")]
    pub created_at_lte: Option<NaiveDateTime>,
    #[serde(rename = "updated_at[gte]")]
    pub updated_at_gte: Option<NaiveDateTime>,
    #[serde(rename = "updated_at[lte]")]
    pub updated_at_lte: Option<NaiveDateTime>,
}

impl User {
    pub fn find_all(params: Params) -> Result<(Vec<Self>, i64), ApiError> {
        let conn = db::connection()?;

        let mut query = user::table.into_boxed();

        query = filter!(query,
            (user::email, @like, params.email),
            (user::created_at, @ge, params.created_at_gte),
            (user::created_at, @le, params.created_at_lte),
            (user::updated_at, @ge, params.updated_at_gte),
            (user::updated_at, @le, params.updated_at_lte)
        );

        query = sort_by!(query, params.sort_by,
            ("id", user::id),
            ("email", user::email),
            ("created_at", user::created_at),
            ("updated_at", user::updated_at)
        );

        let (users, total_pages) = query
            .load_with_pagination(&conn, params.page, params.page_size)?;
        
        Ok((users, total_pages))
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = user::table
            .filter(user::id.eq(id))
            .first(&conn)?;

        Ok(user)
    }

    pub fn create(user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = User::from(user);
        let user = diesel::insert_into(user::table)
            .values(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn update(id: Uuid, user: UserMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let user = diesel::update(user::table)
            .filter(user::id.eq(id))
            .set(user)
            .get_result(&conn)?;

        Ok(user)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                user::table
                    .filter(user::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<UserMessage> for User {
    fn from(user: UserMessage) -> Self {
        User {
            id: Uuid::new_v4(),
            email: user.email,
            password: user.password,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}
