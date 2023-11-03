use std::sync::Arc;

use crate::domain::auth::{AuthStorage, Verifier};

mod auth;
pub use auth::{AuthServer, ZkpAuth};

#[derive(Clone)]
pub struct Context {
    pub auth_storage: Arc<dyn AuthStorage>,
    pub verifier: Arc<dyn Verifier>,
}
