use actix_web::{FromRequest, web::Data};
use actix_web_httpauth::extractors::{bearer::BearerAuth, basic::BasicAuth};
use futures::future::{BoxFuture, ready};
use tracing::{debug, instrument};

use crate::{errors::AppError, db::user::UserRepository, config::crypto::CryptoService};


#[derive(Debug)]
pub struct AuthenticatedUser(pub String);

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        let bearer_result = BearerAuth::from_request(req, payload).into_inner();
        let repository_result = UserRepository::from_request(req, payload).into_inner();
        let crypto_service_result = Data::<CryptoService>::from_request(req, payload).into_inner();
        
        match (bearer_result, repository_result, crypto_service_result) {
            (Ok(bearer), Ok(repository), Ok(crypto_service)) => {
                let future = async move {
                    let email: String = crypto_service
                        .validate_token(bearer.token().to_string())
                        .await
                        .map(|data| data.claims.email)
                        .map_err(|err| {
                            debug!("Cannot verify jwt. {:?}", err);
                            AppError::NOT_AUTHORIZED
                        })?;
                        

                    repository.find_by_email(email.clone()).await.ok_or_else(|| {
                            AppError::NOT_AUTHORIZED
                    })?;

                    Ok(AuthenticatedUser(email))
                };
                Box::pin(future)
            }
            _ => {
                let error = ready(Err(AppError::NOT_AUTHORIZED.into()));
                Box::pin(error)
            }
        }
    }
}

// #[instrument(skip(basic, repository, hashing))]
// pub async fn auth(
//     basic: BasicAuth,
//     repository: UserRepository,
//     hashing: Data<CryptoService>,
// ) -> AppResponse {
//     let username = basic.user_id();
//     let password = basic
//         .password()
//         .ok_or_else(|| {
//             debug!("Invalid request. Missing Basic Auth.");
//             AppError::INVALID_CREDENTIALS
//         })?;

//     let user = repository
//         .find_by_username(username)
//         .await?
//         .ok_or_else(|| {
//             debug!("User doesn't exist.");
//             AppError::INVALID_CREDENTIALS
//         })?;

//     let valid = hashing
//         .verify_password(password, &user.password_hash)
//         .await?;

//     if valid {
//         let token = hashing.generate_jwt(user.id).await?;
//         Ok(HttpResponse::Ok().json(Auth { token }))
//     } else {
//         debug!("Invalid password.");
//         Err(AppError::INVALID_CREDENTIALS.into())
//     }
// }