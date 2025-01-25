use super::{handler, middleware};
use actix_web::middleware::from_fn;
use actix_web::web;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .wrap(from_fn(middleware::auth_middleware::check_auth_middleware))
            .service(handler::user_handler::user),
    );
}
