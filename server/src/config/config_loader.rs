use anyhow::Result;
use tracing::error;

use crate::config::{
    config_model::{Database, DotEnvyConfig, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyConfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SERVER_PORT is valid")
            .parse()?,
        body_limit: std::env::var("SERVER_BODY_LIMIT")
            .expect("SERVER_BODY_LIMIT is valid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is valid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL")
            .expect("DATABASE_URL is valid")
            .parse()?,
    };

    let secret = std::env::var("JWT_USER_SECRET")
        .expect("SECRET is valid")
        .parse()?;

    let config = DotEnvyConfig {
        server,
        database,
        secret,
    };

    Ok(config)
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_form(&stage_str).unwrap_or_default()
}

pub fn get_user_secret() -> Result<String> {
    let dotenvy_env = match load() {
        Ok(env) => env,
        Err(e) => {
            error!("Failed to load ENV: {}", e);
            std::process::exit(1);
        }
    };
    Ok(dotenvy_env.secret)
}
