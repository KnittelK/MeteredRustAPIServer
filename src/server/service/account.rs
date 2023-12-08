use actix_web::web;
use chrono::offset::Local;
use diesel::RunQueryDsl;
use crate::DbPool;
use crate::server::models::account::{Account, NewAccount};
use crate::server::common::errors::DbError;

pub struct AccountService {
    pool: web::Data<DbPool>,
}

impl AccountService {
    pub(crate) fn new(pool: web::Data<DbPool>) -> Self {
        AccountService { pool }
    }

    pub fn add_new_account(&self) -> Result<Account, DbError> {
        let conn = &mut self.pool.get()?;
        use crate::schema::account::dsl::*;

        let curr_date = Local::now().date_naive();
        let new_account = NewAccount { created_at: curr_date, verified: false };

        let res = diesel::insert_into(account)
            .values(&new_account)
            .get_result(conn)?;
        Ok(res)
    }

    // fn _add(&self, conn: &mut PgConnection) -> Result<Account, DbError> {
    //     use crate::schema::account::dsl::*;
    //
    //     let curr_date = Local::now().date_naive();
    //     let new_account = NewAccount{created_at: curr_date, verified: false};
    //
    //     let res = diesel::insert_into(account)
    //         .values(&new_account)
    //         .get_result(conn)?;
    //     res
    // }
}