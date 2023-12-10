use chrono::{Duration, NaiveDate, Local};
use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::api_key)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct APIKey {
    pub id: i32,
    pub key: String,
    pub expires: NaiveDate,
    pub created_at: NaiveDate,
    pub user_id: i32,
    pub revoked: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::api_key)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewApiKey {
    pub key: String,
    pub expires: NaiveDate,
    pub created_at: NaiveDate,
    pub user_id: i32,
    pub revoked: bool,
}

impl NewApiKey {
    pub fn from(key: String, user_id: i32) -> Self {
        NewApiKey {
            key,
            expires: (Local::now() + Duration::days(365)).date_naive(),
            created_at: Local::now().date_naive(),
            user_id,
            revoked: false
        }
    }
}