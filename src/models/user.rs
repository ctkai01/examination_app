use crate::schema::users;
use chrono::{DateTime, NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use tracing::info;
use validator::{Validate, ValidationError};
use validator_derive::Validate;
use std::{time::{Duration, SystemTime}, borrow::Cow, collections::HashMap};

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub token: String,
    pub full_name: Option<String>,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub dob: SystemTime,
    pub created_at: SystemTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUserForm {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, message = "Password must be greater 3 characters"))]
    pub password: String,
    #[validate(custom( function = "validate_dob" ) )]
    pub dob: String,
}

fn validate_dob(dob: &str) -> Result<(), ValidationError> {
    info!("dob: {}", dob);
    let timestamp = match dob.parse::<u64>() {
        Ok(value) => value,
        Err(_) => return Err(ValidationError {
            code: Cow::from("invalid"),
            message: Some(Cow::from("Invalid dob")),
            params: HashMap::new()
        })
    };
    // Convert the Unix timestamp to a SystemTime value
    let system_time = SystemTime::UNIX_EPOCH + Duration::from_secs(timestamp);
    let error = ValidationError {
        code: Cow::from("limit"),
        message: Some(Cow::from("Age must be greater 10")),
        params: HashMap::new()
    };
    let system_time_from_10_years_ago = SystemTime::now() - Duration::from_secs(10 * 365 * 24 * 60 * 60);
    if system_time > system_time_from_10_years_ago {
        return Err(error)
      
    }
    Ok(())
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub token: String,
    pub dob: SystemTime
}
