use std::time::{SystemTime, Duration};
use std::{ops::Deref, sync::Arc};
use color_eyre::Result;
use actix_web::{web::Data, FromRequest};
use diesel::{RunQueryDsl, QueryDsl};
use futures::future::{ready, Ready};
use tracing::info;
use crate::models::user::{User, NewUser};
use crate::schema::users;
use crate::{
    config::{crypto::CryptoService, PostgresPool},
    errors::AppError,
    models::user::NewUserForm,
};
use diesel::prelude::*;
pub struct UserRepository {
    pool: Arc<PostgresPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<PostgresPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, new_user: NewUserForm, hashing: &CryptoService) -> Result<User> {
        let password_hash = hashing.hash_password(new_user.password).await.expect("msg");
        let token = hashing.generate_token(new_user.email.clone());
        let dob = SystemTime::UNIX_EPOCH + Duration::from_secs(new_user.dob.parse::<u64>().unwrap());
      
        let new_user = NewUser {
            password: password_hash,
            token,
            email: new_user.email,
            dob
        };
      
        let user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut *self.pool.clone().get()
        .expect("Error connection DB"))
        ?;
       
        info!("User: {:?}", user);
        Ok(user)
    }

    pub async fn find_by_email(&self, email: String) -> Option<User>  {
        // let result = users::table.filter(users::email)
        // let a = self.pool.clone().get();
        let results = users::table
        .filter(users::email.eq(email))
        
        .first::<User>(&mut *self.pool.clone().get()
        .expect("Error connection DB"));

        match results {
            Ok(user) => Some(user),
            Err(_) => None
        }
        // results.map(|user| Some(user))
        // .map_err(|_| None)

        // if results.is_empty() {
        //     Ok(Some(results))
        // } else {

        // }
        
        // .load::<User>(self.pool.clone().get())
        // .expect("Error loading users");

        // let maybe_user = sqlx::query_as::<_, User>("select * from users where id = $1")
        //     .bind(id)
        //     .fetch_optional(&*self.pool)
        //     .await?;

        // Ok(maybe_user)
    }
}



impl FromRequest for UserRepository {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let pool_result = Data::<PostgresPool>::from_request(req, payload).into_inner();

        match pool_result {
            Ok(pool) => ready(Ok(UserRepository::new(pool.deref().clone()))),
            _ => ready(Err(AppError::NOT_AUTHORIZED.default())),
        }
    }
}
