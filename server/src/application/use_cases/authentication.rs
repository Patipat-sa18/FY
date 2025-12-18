use std::sync::Arc;

use anyhow::Result;
use chrono::{Duration, Utc};

use crate::{
    config::config_loader::get_user_secret,
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        argon2,
        jwt::{
            authentication_model::LoginModel,
            generate_token,
            jwt_model::{Claims, Passport},
        },
    },
};
pub struct AuthenticationUseCase<T>
where
    T: BrawlerRepository + Send + Sync,
{
    brawler_repository: Arc<T>,
}
impl<T> AuthenticationUseCase<T>
where
    T: BrawlerRepository + Sync + Send,
{
    pub fn new(brawler_repository: Arc<T>) -> Self {
        Self { brawler_repository }
    }

    pub async fn login(&self, login_model: LoginModel) -> Result<Passport> {
        let secret = get_user_secret()?;
        let username = login_model.username.clone();

        //find this user in database
        let user = self.brawler_repository.find_by_username(username).await?;
        let hashed_password = user.password;

        if !argon2::verify(login_model.password, hashed_password)? {
            return Err(anyhow::anyhow!("Invalid Password !!"));
        }

        let claims = Claims {
            sub: user.id.to_string(),
            exp: (Utc::now() + Duration::days(3)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
        };

        let token = generate_token(secret, &claims)?;

        Ok(Passport { token })
    }
}
