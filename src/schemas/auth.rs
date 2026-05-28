use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct RegisterSchema {
    pub name: String,
    pub email: String,
    pub password: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}
