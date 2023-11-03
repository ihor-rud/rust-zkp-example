use async_trait::async_trait;

pub struct ZkpContext {
    pub challenge: u64,
    pub y1: u64,
    pub y2: u64,
    pub r1: u64,
    pub r2: u64,
}

// Abstraction over persistent storage that allows to store Auth data
#[async_trait]
pub trait AuthStorage: Send + Sync {
    async fn save_user(&self, user_name: String, y1: u64, y2: u64) -> anyhow::Result<()>;
    async fn user_exists(&self, user_name: &str) -> anyhow::Result<bool>;
    async fn save_challenge(
        &self,
        user_name: String,
        challenge: u64,
        r1: u64,
        r2: u64,
    ) -> anyhow::Result<String>;
    async fn fetch_context(&self, id: &str) -> anyhow::Result<ZkpContext>;
    async fn allocate_session(&self, id: String) -> anyhow::Result<String>;
}
