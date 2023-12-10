use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::prelude::*;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use uuid::Uuid;
use crate::DbPool;
use crate::schema::account::dsl::account;
use crate::server::models::account::{Account, NewAccount};
use crate::server::common::errors::DbError;
use crate::server::models::api_key::{APIKey, NewApiKey};
use crate::server::web::account_serializers::{LoginCredentials, NewAccountRegistration};

pub struct AccountService {
    pool: web::Data<DbPool>,
}

impl AccountService {
    pub fn new(pool: web::Data<DbPool>) -> Self {
        AccountService { pool }
    }

    pub fn add_new_account(&self, new_account: NewAccountRegistration) -> Result<Account, DbError> {
        let conn = &mut self.pool.get()?;
        conn.transaction::<_, DbError, _>(|conn| {
            use crate::schema::account::dsl::*;
            use crate::schema::api_key::dsl::*;

            let salted_hashed_password = self.generate_password_hash(&new_account.password);
            let generated_key = self.generate_api_key();
            let new_account = NewAccount::from(new_account, salted_hashed_password);

            let acc: Account = diesel::insert_into(account)
                .values(&new_account)
                .get_result(conn)?;

            let new_api_key = NewApiKey::from(generated_key, acc.id);
            diesel::insert_into(api_key)
                .values(&new_api_key)
                .execute(conn)?;

            Ok(acc)
        })
    }

    pub fn login_user(&self, provided_credentials: LoginCredentials) -> Result<APIKey, DbError>{
        let fetched_account = self.verify_password_login(
            &provided_credentials.username, &provided_credentials.password
        );
        self.fetch_api_key(fetched_account.unwrap())
    }

    pub fn fetch_api_key(&self, user_account: Account) -> Result<APIKey, DbError> {
        let conn = &mut self.pool.get()?;
        use crate::schema::api_key::dsl::*;

        let mut record = api_key
            .filter(user_id.eq(user_account.id))
            .select(APIKey::as_select())
            .load(conn)?;

        Ok(record.pop().unwrap())
    }

    fn generate_api_key(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn generate_password_hash(&self, user_password: &String) -> String {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);

        let hashed_password = argon2
            .hash_password(user_password.as_ref(), &salt)
            .unwrap()
            .to_string();
        hashed_password
    }

    fn verify_password_login(&self, provided_username: &String, provided_password: &String) -> Result<Account, DbError>{
        let conn = &mut self.pool.get()?;
        use crate::schema::account::dsl::*;

        let mut fetched_account = account
            .filter(username.eq(provided_username))
            .select(Account::as_select())
            .load(conn)?;
        let parsed_hash = PasswordHash::new(&fetched_account[0].password).unwrap();
        Argon2::default().verify_password(provided_password.as_ref(), &parsed_hash).unwrap();
        Ok(fetched_account.pop().unwrap())
    }
}
