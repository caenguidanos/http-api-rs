#[cfg(test)]
pub mod tests {
    use crate::contexts::ecommerce::common;
    use crate::libs;

    #[derive(Clone)]
    pub struct HttpContextFixture {
        pub token: String,
        pub services: common::infrastructure::DependencyContainer,
    }

    impl HttpContextFixture {
        pub async fn new() -> Self {
            let database = libs::pg::fixture::PostgresDatabaseFixture::new().await;

            Self {
                token: common::infrastructure::extractors::fixture::encode_jwt(&[]),
                services: common::infrastructure::DependencyContainer::new(database.pool),
            }
        }

        #[allow(dead_code)]
        pub fn with_permissions(&mut self, permissions: &[&str]) {
            self.token = common::infrastructure::extractors::fixture::encode_jwt(permissions);
        }
    }
}
