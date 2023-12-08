use diesel::prelude::*;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Account {
    pub id: i32,
    pub created_at: NaiveDate,
    pub verified: bool
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::account)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewAccount {
    pub created_at: NaiveDate,
    pub verified: bool
}

