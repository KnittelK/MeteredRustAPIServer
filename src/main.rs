
/*
The metered API server minimal feature set includes:
    Add/remove API keys
    Track the number of times each API key has made a request
    Associate API keys with user identity (email address, for example)
*/

use actix_web::{App, HttpServer, web};
use crate::server::routes::{hello, index, register_user};
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use std::env;

mod server;
mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(register_user);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(|| async {"Actix REST API"}))
            .configure(routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await

}
