use async_trait::async_trait;

mod grpc;

pub use grpc::register;

// Abstraction over server communication protocol
#[async_trait]
pub trait Client {}
