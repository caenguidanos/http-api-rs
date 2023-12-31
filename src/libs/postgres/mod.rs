pub mod errcodes;

pub type ConnectionPool = sqlx::PgPool;

pub struct ConnectionManager;

impl ConnectionManager {
    pub async fn new_pool(
        url: impl Into<String>,
        max_connections: Option<u32>,
    ) -> Result<ConnectionPool, sqlx::error::Error> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(max_connections.unwrap_or(12))
            .acquire_timeout(std::time::Duration::from_secs(3))
            .connect(url.into().as_str())
            .await?;

        tracing::debug!("Initialized postgres connection");

        Ok(pool)
    }
}

#[cfg(test)]
pub mod fixture {
    use rand::Rng;

    use super::*;

    #[derive(Clone)]
    pub struct PostgresDatabaseFixture {
        pub pool: ConnectionPool,
        pub configuration: PostgresDatabaseFixtureConfiguration,
    }

    #[derive(Clone, Debug)]
    pub struct PostgresDatabaseFixtureConfiguration {
        pub user: String,
        pub pass: String,
        pub host: String,
        pub port: String,
        pub name: String,
    }

    impl PostgresDatabaseFixtureConfiguration {
        pub fn build_uri(&self) -> String {
            format!(
                "postgresql://{}:{}@{}:{}/{}?sslmode=disable",
                self.user, self.pass, self.host, self.port, self.name
            )
        }
    }

    impl Default for PostgresDatabaseFixtureConfiguration {
        fn default() -> Self {
            Self {
                user: std::env::var("DATABASE_USER").unwrap_or(String::from("root")),
                pass: std::env::var("DATABASE_PASS").unwrap_or(String::from("root")),
                host: std::env::var("DATABASE_HOST").unwrap_or(String::from("localhost")),
                port: std::env::var("DATABASE_PORT").unwrap_or(String::from("5432")),
                name: std::env::var("DATABASE_NAME").unwrap_or(String::from("postgres")),
            }
        }
    }

    impl PostgresDatabaseFixture {
        pub async fn new() -> Self {
            let generated_database_name = Self::generate_name();

            let template_database = std::env::var("DATABASE_TEMPLATE").unwrap();

            let pool = ConnectionManager::new_pool(PostgresDatabaseFixtureConfiguration::default().build_uri(), None)
                .await
                .expect("error creating postgres fixture pool");

            sqlx::query(&format!(
                "CREATE DATABASE {generated_database_name} TEMPLATE '{template_database}'"
            ))
            .execute(&pool)
            .await
            .expect("error trying to terminate template database connections");

            let configuration = PostgresDatabaseFixtureConfiguration {
                name: generated_database_name.clone(),
                ..Default::default()
            };

            let pool = sqlx::PgPool::connect(configuration.build_uri().as_str())
                .await
                .expect("error creating postgres fixture pool");

            Self { pool, configuration }
        }

        #[allow(dead_code)]
        async fn terminate_template_pid(template_database: &str, pool: &ConnectionPool) {
            sqlx::query(&format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{template_database}'"
            ))
            .execute(pool)
            .await
            .expect("error trying to terminate template database connections");
        }

        fn generate_name() -> String {
            const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

            let mut rng = rand::thread_rng();

            std::iter::repeat_with(|| {
                let rng_max = rng.gen_range(0..CHARSET.len());

                CHARSET[rng_max] as char
            })
            .take(7)
            .collect::<String>()
        }

        #[allow(dead_code)]
        pub async fn dispose(&self) {
            let configuration = PostgresDatabaseFixtureConfiguration::default();

            let pool = ConnectionManager::new_pool(&configuration.build_uri(), None)
                .await
                .expect("error creating postgres fixture pool");

            sqlx::query(&format!(
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
                self.configuration.name
            ))
            .execute(&pool)
            .await
            .expect("error trying to terminate test database connections");

            sqlx::query(&format!("DROP DATABASE IF EXISTS {}", self.configuration.name))
                .execute(&pool)
                .await
                .expect("error trying to drop test database");
        }
    }
}
