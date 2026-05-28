use actix_web::web;

use crate::handlers::auth::{register, login};

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
}
