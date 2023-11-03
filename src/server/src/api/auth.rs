use tonic::{Request, Response, Status};
use zkp_auth::auth_server::Auth;
use zkp_auth::{
    AuthenticationAnswerRequest,
    AuthenticationAnswerResponse,
    AuthenticationChallengeRequest,
    AuthenticationChallengeResponse,
    RegisterRequest,
    RegisterResponse,
};

use super::Context;
use crate::domain::auth;

mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

pub use zkp_auth::auth_server::AuthServer;

pub struct ZkpAuth {
    pub context: Context,
}

#[tonic::async_trait]
impl Auth for ZkpAuth {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> tonic::Result<Response<RegisterResponse>> {
        let message = request.into_inner();

        auth::register(
            self.context.auth_storage.as_ref(),
            message.user,
            message.y1,
            message.y2,
        )
        .await
        .map_err(|err| Status::from_error(err.into()))?;

        Ok(Response::new(RegisterResponse {}))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthenticationChallengeRequest>,
    ) -> tonic::Result<Response<AuthenticationChallengeResponse>> {
        let message = request.into_inner();
        let challenge = auth::create_authentication_challenge(
            self.context.auth_storage.as_ref(),
            message.user,
            message.r1,
            message.r2,
        )
        .await
        .map_err(|err| Status::from_error(err.into()))?;

        Ok(Response::new(AuthenticationChallengeResponse {
            auth_id: challenge.id,
            c: challenge.challenge,
        }))
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthenticationAnswerRequest>,
    ) -> tonic::Result<Response<AuthenticationAnswerResponse>> {
        let message = request.into_inner();

        let session_id = auth::verify_authentication(
            self.context.auth_storage.as_ref(),
            self.context.verifier.as_ref(),
            message.auth_id,
            message.s,
        )
        .await
        .map_err(|err| Status::from_error(err.into()))?;

        Ok(Response::new(AuthenticationAnswerResponse { session_id }))
    }
}
