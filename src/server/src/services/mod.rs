use std::sync::Arc;

use database::Database;
use quicklink::link::service::LinkService;

use crate::config::Config;

pub type SharedServices = Arc<Services>;

#[derive(Clone)]
pub struct Services {
    pub link: Arc<LinkService<database::link::LinkRepository>>,
}

impl Services {
    pub async fn new(config: &Config) -> Self {
        let db_pool = Database::new(&config.database_url)
            .await
            .expect("Failed to create a new database pool");
        let link_repository = database::link::LinkRepository::new(&db_pool);
        let link_service = LinkService::new(link_repository);

        Self {
            link: Arc::new(link_service),
        }
    }

    pub async fn shared(config: &Config) -> SharedServices {
        let service = Self::new(config).await;
        Arc::new(service)
    }
}
