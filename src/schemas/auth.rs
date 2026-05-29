use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct RegisterSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub user_id: u64,
    pub exp: usize,
}
