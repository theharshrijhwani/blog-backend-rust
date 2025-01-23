use super::handler;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(handler::home_handler::greet)
            .service(handler::home_handler::test),
    );
}
