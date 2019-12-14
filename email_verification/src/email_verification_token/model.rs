use crate::api_error::ApiError;
use crate::db;
use crate::schema::email_verification_token;
use chrono::{NaiveDateTime, Utc, Duration};
use diesel::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct EmailVerificationTokenMessage {
    pub id: Option<String>,
    pub email: String,
}

#[derive(Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "email_verification_token"]
pub struct EmailVerificationToken {
    pub id: Vec<u8>,
    pub email: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl EmailVerificationToken {
    pub fn find(id: &Vec<u8>) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let token = email_verification_token::table
            .filter(email_verification_token::id.eq(id))
            .first(&conn)?;

        Ok(token)
    }

    pub fn create(body: EmailVerificationTokenMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let id = rand::thread_rng().gen::<[u8; 32]>().to_vec();
        let email = body.email;
        let created_at = Utc::now().naive_utc();
        let expires_at = created_at + Duration::hours(12);
        let token = EmailVerificationToken { id, email, expires_at, created_at };

        let token = diesel::insert_into(email_verification_token::table)
            .values(&token)
            .on_conflict(email_verification_token::email)
            .do_update()
            .set((
                email_verification_token::id.eq(&token.id),
                email_verification_token::created_at.eq(&token.created_at),
                email_verification_token::expires_at.eq(&token.expires_at),
            ))
            .get_result(&conn)?;

        Ok(token)
    }

    pub fn delete(id: &Vec<u8>) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                email_verification_token::table
                    .filter(email_verification_token::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}