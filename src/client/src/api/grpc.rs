use super::Client;

mod zkp_auth {
    tonic::include_proto!("zkp_auth");
}

use num_bigint::BigUint;
use rand::Rng;
use zkp_auth::auth_client::AuthClient;
use zkp_auth::{AuthenticationAnswerRequest, AuthenticationChallengeRequest, RegisterRequest};

pub struct GRPCClient {
    #[allow(unused)]
    session_id: String,
}

impl Client for GRPCClient {}

pub async fn register(ip: String, user: String, secret: u64) -> anyhow::Result<impl Client> {
    let mut auth = AuthClient::connect(ip).await?;

    let p = BigUint::from(23u64);
    let q = BigUint::from(11u64);
    let g = BigUint::from(4u64);
    let h = BigUint::from(9u64);

    let secret = BigUint::from(secret);
    let y1 = g.modpow(&secret, &p);
    let y2 = h.modpow(&secret, &p);

    let request = tonic::Request::new(RegisterRequest {
        user: user.clone(),
        y1: y1.try_into()?,
        y2: y2.try_into()?,
    });
    auth.register(request).await?;

    let k = BigUint::from(rand::thread_rng().gen::<u64>());
    let r1 = g.modpow(&k, &p);
    let r2 = h.modpow(&k, &p);

    let request = tonic::Request::new(AuthenticationChallengeRequest {
        user,
        r1: r1.try_into()?,
        r2: r2.try_into()?,
    });
    let response = auth
        .create_authentication_challenge(request)
        .await?
        .into_inner();
    let challenge = BigUint::from(response.c);

    let s = k - (&challenge * secret % q);

    let request = tonic::Request::new(AuthenticationAnswerRequest {
        auth_id: response.auth_id,
        s: s.try_into()?,
    });
    let response = auth.verify_authentication(request).await?.into_inner();

    println!(
        "Authentication verified: session_id = {}",
        response.session_id
    );

    Ok(GRPCClient {
        session_id: response.session_id,
    })
}
