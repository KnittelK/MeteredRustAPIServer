
use actix_web::{web, post, HttpResponse, Error};
use crate::DbPool;
use crate::server::service::account::AccountService;


#[post("/register")]
async fn register_user(pool: web::Data<DbPool>) -> Result<HttpResponse, Error>{
    let account = AccountService::new(pool);
    let result = web::block(move || {
        account.add_new_account()
    })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}
