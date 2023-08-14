use std::ops::Sub;

use common_lib::blind_auth_api::blind_auth_server::{BlindAuth, BlindAuthServer};
use common_lib::blind_auth_api::{
    AuthAnswerRequest, AuthAnswerResponse, AuthChallengeRequest, AuthChallengeResponse,
    RegisterRequest, RegisterResponse,
};
use common_lib::public_params::PublicParams;
use common_lib::store::models::{Challenge, User};
use common_lib::store::store::DataStore;
use log::{info, warn};
use num_bigint::BigInt;
use num_traits::Num;
use tonic::{transport::Server, Request, Response, Status};

struct AuthServer {
    store: DataStore,
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

        if let Some(user) = self.store.get_user(request.get_ref().user.to_owned()) {
            let c = common_lib::generate_randomness(&BigInt::from(2), &PublicParams::q().sub(2));
            self.store.insert_challenge(Challenge {
                c: c.to_owned(),
                user_id: request.get_ref().user.to_owned(),
                r1: BigInt::from_str_radix(request.get_ref().r1.as_str(), 16).unwrap(),
                r2: BigInt::from_str_radix(request.get_ref().r2.as_str(), 16).unwrap(),
                id: request.get_ref().user.to_owned(),
            });

            Ok(Response::new(AuthChallengeResponse {
                auth_id: user.id,
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
            .get_challenge(request.get_ref().auth_id.to_owned())
            .unwrap();
        let user = self.store.get_user(challenge.user_id.to_owned()).unwrap();
        let auth_s = BigInt::from_str_radix(request.get_ref().s.as_str(), 16).unwrap();

        let success = common_lib::verifier::verify_authentication(&user, &challenge, auth_s);

        if !success {
            return Err(Status::permission_denied("auth challenge failed"));
        }
        Ok(Response::new(AuthAnswerResponse {
            session_id: user.id,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let addr = "0.0.0.0:50051".parse()?;
    let store = DataStore::new();
    let blind_auth = AuthServer { store };

    println!("Server listening on {}", addr);
    Server::builder()
        .add_service(BlindAuthServer::new(blind_auth))
        .serve(addr)
        .await?;

    Ok(())
}
