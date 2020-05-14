use crate::error::GndError;
use dotenv::dotenv;
use std::{env, error};

#[derive(Default, Debug, Clone)]
pub struct Conf {
    pub work_dir: String,
    pub pg_host: String,
    pub pg_user: String,
    pub pg_pass: String,
    pub pg_db: String,
}

impl Conf {
    pub fn new() -> Result<Self, Box<dyn error::Error>> {
        dotenv().ok();
        let conf = Conf {
            work_dir: env::var("WORK_DIR").map_err(|e| GndError::EnvVarError {
                env_var: "WORK_DIR".to_owned(),
                err: e.to_string(),
            })?,
            pg_host: env::var("PG_HOST").map_err(|e| GndError::EnvVarError {
                env_var: "PG_HOST".to_owned(),
                err: e.to_string(),
            })?,
            pg_user: env::var("PG_USER").map_err(|e| GndError::EnvVarError {
                env_var: "PG_USER".to_owned(),
                err: e.to_string(),
            })?,
            pg_pass: env::var("PG_PASS").map_err(|e| GndError::EnvVarError {
                env_var: "PG_PASS".to_owned(),
                err: e.to_string(),
            })?,
            pg_db: env::var("PG_DB").map_err(|e| GndError::EnvVarError {
                env_var: "PG_DB".to_owned(),
                err: e.to_string(),
            })?,
        };
        Ok(conf)
    }
}
