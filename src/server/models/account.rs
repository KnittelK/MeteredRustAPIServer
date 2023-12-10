use diesel::prelude::*;
use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use crate::server::web::account_serializers::NewAccountRegistration;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub created_at: NaiveDate,
    pub verified: bool,
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccount {
    pub created_at: NaiveDate,
    pub verified: bool,
    pub username: String,
    pub password: String,
    pub email: String,
}


impl NewAccount {
    pub fn from(new_account_registration: NewAccountRegistration, hashed_password: String) -> Self {
        NewAccount {
            created_at: Local::now().date_naive(),
            verified: false,
            password: hashed_password,
            username: new_account_registration.username,
            email: new_account_registration.email,
        }
    }
}