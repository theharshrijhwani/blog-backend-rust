use super::handler;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(handler::auth_handler::login)
            .service(handler::auth_handler::register),
    );
}
