use std::sync::Arc;

use anyhow::Result;

use crate::{
    domain::repositories::brawlers::BrawlerRepository,
    infrastructure::{
        argon2,
        jwt::{authentication_model::LoginModel, jwt_model::Passport},
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
        let username = login_model.username.clone();

        //find this user in database
        let user = self.brawler_repository.find_by_username(username).await?;
        let hashed_password = user.password;

        if !argon2::verify(login_model.password, hashed_password)? {
            return Err(anyhow::anyhow!("Invalid Password !!"));
        }

        let passport = Passport::new(user.id, user.display_name, user.avatar_url)?;
        Ok(passport)
    }
}
