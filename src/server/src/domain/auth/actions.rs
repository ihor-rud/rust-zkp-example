use anyhow::anyhow;
use rand::Rng;

use super::{AuthStorage, VerificationData, Verifier};

pub struct Challenge {
    pub id: String,
    pub challenge: u64,
}

pub async fn register(
    storage: &dyn AuthStorage,
    user_name: String,
    y1: u64,
    y2: u64,
) -> anyhow::Result<()> {
    storage.save_user(user_name, y1, y2).await
}

pub async fn create_authentication_challenge(
    storage: &dyn AuthStorage,
    user_name: String,
    r1: u64,
    r2: u64,
) -> anyhow::Result<Challenge> {
    if !storage.user_exists(&user_name).await? {
        return Err(anyhow!("user not found"));
    }

    let challenge = rand::thread_rng().gen();
    let id = storage.save_challenge(user_name, challenge, r1, r2).await?;
    Ok(Challenge { id, challenge })
}

pub async fn verify_authentication(
    storage: &dyn AuthStorage,
    verifier: &dyn Verifier,
    auth_id: String,
    s: u64,
) -> anyhow::Result<String> {
    let context = storage.fetch_context(&auth_id).await?;

    if !verifier.verify(VerificationData {
        challenge: context.challenge,
        y1: context.y1,
        y2: context.y2,
        r1: context.r1,
        r2: context.r2,
        s,
    }) {
        return Err(anyhow!("invalid proof"));
    }

    storage.allocate_session(auth_id).await
}
