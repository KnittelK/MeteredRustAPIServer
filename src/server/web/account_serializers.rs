use serde::{Deserialize};

#[derive(Deserialize)]
pub struct NewAccountRegistration {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String
}