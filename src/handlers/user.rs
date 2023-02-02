use std::collections::HashMap;

use super::{AppResponse, auth::AuthenticatedUser};
use crate::{
    config::crypto::CryptoService,
    db::user::UserRepository,
    errors::AppError,
    models::user::{NewUserForm, User},
};
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use color_eyre::Result;
use diesel::result::Error;
use tracing::{info, instrument};
use validator::Validate;

pub async fn create_user(
    user: Form<NewUserForm>,
    repository: UserRepository,
    crypto_service: Data<CryptoService>,
) -> AppResponse {
    info!("USer: {:?}", user);
    match user.validate() {
        Ok(_) => (),
        Err(e) => {
            let mut map_error = HashMap::new();
            for x in e.field_errors() {
                map_error.insert(
                    x.0.to_string(),
                    x.1[0].message.clone().unwrap().as_ref().to_string(),
                );
            }
            let error = AppError::INVALID_INPUT.message(map_error);
            return Err(error);
        }
    };

    let result: Result<User> = repository.create(user.0, &crypto_service).await;
    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(error) => {
            let pg_error = error
                .root_cause()
                .downcast_ref::<Error>()
                .ok_or_else(|| {
                    AppError::INTERNAL_ERROR
                })
                ?;

            let mut error_hash = HashMap::new();
            let error_app = match pg_error {
                Error::DatabaseError(kind, _) => {
                    if let diesel::result::DatabaseErrorKind::UniqueViolation = kind {
                        error_hash.insert("email".to_string(), "Email already use!".to_string());
                        AppError::INVALID_INPUT.message(error_hash)
                    } else {
                        error_hash.insert("".to_string(), "Something Database error".to_string());
                        AppError::INVALID_INPUT.message(error_hash)
                    }
                },
                _ => {
                    error_hash.insert("".to_string(), "".to_string());
                    AppError::INTERNAL_ERROR.message(error_hash)
                },
            };

            Err(error_app)
        }
    }
}

pub async fn update_profile(
    user: AuthenticatedUser,
    repository: UserRepository,
) -> AppResponse {
    

    Ok(HttpResponse::Ok().json("dsdsdsd"))
}