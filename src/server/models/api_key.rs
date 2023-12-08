use chrono::NaiveDate;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::api_key)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct APIKey {
    pub id: i32,
    pub key: String,
    pub expires: NaiveDate,
    pub created_at: NaiveDate,
    pub user_id: i32,
    pub revoked: bool,
}