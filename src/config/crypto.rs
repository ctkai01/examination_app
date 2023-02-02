use std::sync::Arc;

use actix_web::web::block;
use argonautica::{Hasher, Verifier};
use chrono::{Utc, Duration};
use chrono_tz::Asia::Ho_Chi_Minh;
use jsonwebtoken::{DecodingKey, decode, EncodingKey, Algorithm, Validation, Header, encode, errors::Error, TokenData};
use serde::{Serialize, Deserialize};
use futures::compat::Future01CompatExt;
use eyre::eyre;
use color_eyre::Result;
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub email: String
}

#[derive(Debug, Clone)]
pub struct CryptoService {
    pub key_password: Arc<String>,
    pub jwt_secret: Arc<String>
}

#[derive(Serialize)]
pub struct Auth {
    pub access_token: String,
}

impl CryptoService {
    #[instrument(skip(self, password), err)]
    pub async fn hash_password(&self, password: String) -> Result<String> {
        Hasher::default()
            .with_secret_key(&*self.key_password)
            .with_password(password)
            .hash_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Hashing error: {}", err))
    }

    #[instrument(skip(self, password, password_hash))]
    pub async fn verify_password(&self, password: &str, password_hash: &str) -> Result<bool> {
        Verifier::default()
            .with_secret_key(&*self.key_password)
            .with_hash(password_hash)
            .with_password(password)
            .verify_non_blocking()
            .compat()
            .await
            .map_err(|err| eyre!("Verifying error: {}", err))
    }

    pub fn generate_token(&self, email: String) -> String {
        let expired_time = Utc::now().with_timezone(&Ho_Chi_Minh) + Duration::days(1);
        let claims = Claims {
            exp: expired_time.timestamp() as usize,
            email
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .unwrap()
    }

    #[instrument(skip(self, token))]
    pub async fn validate_token(&self, token: String) -> Result<TokenData<Claims>> {
        // let _decode = decode::<Claims>(
        //     token.as_str(),
        //     &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
        //     &Validation::new(Algorithm::HS256),
        // )
        // .map(|decoded_claim| decoded_claim.claims.email)
        // .map_err(|err| err);
        let jwt_key = self.jwt_secret.clone();

        block(move || {
            let decoding_key = DecodingKey::from_secret(jwt_key.as_bytes());
            let validation = Validation::default();
            decode::<Claims>(&token, &decoding_key, &validation)
        })
        .await?
        .map_err(|err| eyre!("Verifying jwt token: {}", err))


        // match _decode {
        //     Ok(decoded_claim) => Ok(decoded_claim.claims.email),
        //     Err(err) => eyre!("Verifying jwt token: {}", err),
        // }
    }
}