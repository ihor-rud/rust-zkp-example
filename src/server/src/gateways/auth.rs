use std::collections::hash_map::Entry;
use std::collections::HashMap;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use rand::distributions::{Alphanumeric, DistString};
use tokio::sync::Mutex;

use crate::domain::auth::{AuthStorage, ZkpContext};

#[derive(Debug, Clone)]
struct UserContext {
    y1: u64,
    y2: u64,
}

#[derive(Debug, Clone)]
struct Challenge {
    user_name: String,
    challenge: u64,
    r1: u64,
    r2: u64,
}

#[derive(Default)]
pub struct MemoryAuthStorage {
    users: Mutex<HashMap<String, UserContext>>,
    challenges: Mutex<HashMap<String, Challenge>>,
}

#[async_trait]
impl AuthStorage for MemoryAuthStorage {
    async fn save_user(&self, user_name: String, y1: u64, y2: u64) -> anyhow::Result<()> {
        match self.users.lock().await.entry(user_name) {
            Entry::Occupied(_) => return Err(anyhow!("user already present")),
            Entry::Vacant(entry) => entry.insert(UserContext { y1, y2 }),
        };

        Ok(())
    }

    async fn user_exists(&self, user_name: &str) -> anyhow::Result<bool> {
        Ok(self.users.lock().await.get(user_name).is_some())
    }

    async fn save_challenge(
        &self,
        user_name: String,
        challenge: u64,
        r1: u64,
        r2: u64,
    ) -> anyhow::Result<String> {
        let id = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

        self.challenges.lock().await.insert(
            id.clone(),
            Challenge {
                user_name,
                challenge,
                r1,
                r2,
            },
        );

        Ok(id)
    }

    async fn fetch_context(&self, id: &str) -> anyhow::Result<ZkpContext> {
        let challenge = self
            .challenges
            .lock()
            .await
            .get(id)
            .cloned()
            .context("unknown context id")?;

        let user_context = self
            .users
            .lock()
            .await
            .get(&challenge.user_name)
            .cloned()
            .unwrap();

        Ok(ZkpContext {
            challenge: challenge.challenge,
            y1: user_context.y1,
            y2: user_context.y2,
            r1: challenge.r1,
            r2: challenge.r2,
        })
    }

    async fn allocate_session(&self, id: String) -> anyhow::Result<String> {
        self.challenges.lock().await.remove(&id);
        Ok(format!("session/{}", id))
    }
}
