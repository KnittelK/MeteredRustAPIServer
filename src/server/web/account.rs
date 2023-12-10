use actix_web::{web, post, HttpResponse, Error};
use crate::DbPool;
use crate::server::service::account::AccountService;
use crate::server::web::account_serializers::{LoginCredentials, NewAccountRegistration};

#[post("/register")]
pub async fn register_user(pool: web::Data<DbPool>, payload: web::Json<NewAccountRegistration>) -> Result<HttpResponse, Error>{
    let account = AccountService::new(pool);
    let result = web::block(move || {
        account.add_new_account(payload.into_inner())
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}


#[post("/login")]
pub async fn login_user(pool: web::Data<DbPool>, payload: web::Json<LoginCredentials>) -> Result<HttpResponse, Error> {
    let account = AccountService::new(pool);
    let result = web::block(move || {
        account.login_user(payload.into_inner())
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}