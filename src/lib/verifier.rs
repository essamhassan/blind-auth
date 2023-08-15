use log::{debug, info};
use num_bigint::BigInt;
use num_traits::Num;
use std::ops::Sub;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::blind_auth_api::blind_auth_server::BlindAuth;
use crate::blind_auth_api::{
    AuthAnswerRequest, AuthAnswerResponse, AuthChallengeRequest, AuthChallengeResponse,
    RegisterRequest, RegisterResponse,
};
use crate::generate_randomness;
use crate::public_params::PublicParams;
use crate::store::models::{Challenge, Session, User};
use crate::store::store;

pub struct AuthServer {
    pub store: store::DataStore,
}

#[tonic::async_trait]
impl BlindAuth for AuthServer {
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        info!("register request: {:?}", request.get_ref());

        self.store.insert_user(User {
            id: request.get_ref().user.to_string(),
            y1: BigInt::from_str_radix(request.get_ref().y1.as_str(), 16).unwrap(),
            y2: BigInt::from_str_radix(request.get_ref().y2.as_str(), 16).unwrap(),
        });

        Ok(Response::new(RegisterResponse { success: true }))
    }

    async fn create_authentication_challenge(
        &self,
        request: Request<AuthChallengeRequest>,
    ) -> Result<Response<AuthChallengeResponse>, Status> {
        info!(
            "create_authentication_challenge req: {:?}",
            request.get_ref()
        );

        if let Some(user) = self.store.get_user(&request.get_ref().user) {
            let c = generate_randomness(&BigInt::from(2), &PublicParams::q().sub(2));
            let challenge = Challenge {
                c: c.clone(),
                user_id: user.id,
                r1: BigInt::from_str_radix(request.get_ref().r1.as_str(), 16).unwrap(),
                r2: BigInt::from_str_radix(request.get_ref().r2.as_str(), 16).unwrap(),
                id: generate_id(),
            };
            self.store.insert_challenge(challenge.clone());

            Ok(Response::new(AuthChallengeResponse {
                auth_id: challenge.id,
                c: c.to_str_radix(16),
            }))
        } else {
            return Err(Status::failed_precondition("user is not registered"));
        }
    }

    async fn verify_authentication(
        &self,
        request: Request<AuthAnswerRequest>,
    ) -> Result<Response<AuthAnswerResponse>, Status> {
        info!("verify_authentication req: {:?}", request.get_ref());
        let challenge = self
            .store
            .get_challenge(&request.get_ref().auth_id)
            .unwrap();
        let user = self.store.get_user(&challenge.user_id).unwrap();
        let auth_s = BigInt::from_str_radix(request.get_ref().s.as_str(), 16).unwrap();

        let success = verify_challenge(&user, &challenge, auth_s);
        let session = Session {
            id: generate_id(),
            user_id: user.id,
        };
        self.store.insert_session(session.clone());
        if !success {
            return Err(Status::permission_denied("auth challenge failed"));
        }
        Ok(Response::new(AuthAnswerResponse {
            session_id: session.id,
        }))
    }
}

// Verifiers challenge response following the predicate:
// r1 == g^s * y1*c (mod p) && r2 == h^s * y2^c (mod p)
fn verify_challenge(user: &User, challenge: &Challenge, s: BigInt) -> bool {
    debug!(
        "y1 {}, g {}, y2 {}, h {}, c {}, s {}",
        user.y1,
        PublicParams::g(),
        user.y2,
        PublicParams::h(),
        challenge.c,
        s
    );

    let rhs = (PublicParams::g().modpow(&s, &PublicParams::p())
        * user.y1.modpow(&challenge.c, &PublicParams::p()))
        % &PublicParams::p();

    let lhs = (PublicParams::h().modpow(&s, &PublicParams::p())
        * user.y2.modpow(&challenge.c, &PublicParams::p()))
        % &PublicParams::p();

    debug!(
        "rhs {}, r1 {}, lhs {}, r2 {}",
        rhs, challenge.r1, lhs, challenge.r2
    );

    return rhs == challenge.r1 && lhs == challenge.r2;
}

pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}
