mod user;
mod auth;
use actix_web::{web, HttpResponse};
use user::create_user;

use crate::errors::AppError;

use user::update_profile;

type AppResult<T> = Result<T, AppError>;
type AppResponse = AppResult<HttpResponse>;

pub fn app_config(config: &mut web::ServiceConfig) {
    let register = web::resource("/register").route(web::post().to(create_user));

    let me = web::resource("/me")
    .route(web::get().to(update_profile));

    config.service(register).service(me);
}