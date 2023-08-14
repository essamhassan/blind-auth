use std::ops::Sub;

use clap::{Parser, Subcommand};
use common_lib::public_params::PublicParams;
use log::info;
use num_traits::Num;
use tonic::transport::Channel;
use num_bigint::BigInt;

use common_lib::blind_auth_api::blind_auth_client::BlindAuthClient;
use common_lib::blind_auth_api::{
    AuthAnswerRequest, AuthAnswerResponse, AuthChallengeRequest,
    RegisterRequest,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    server: String,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Register {
        #[arg(short, long)]
        client_id: String,
    },

    Login {
        /// Sets the user name
        #[arg(short, long)]
        client_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cli = Cli::parse();

    info!("Connect to server");
    let channel = Channel::from_shared(cli.server)?.connect().await?;
    let mut client: BlindAuthClient<Channel> = BlindAuthClient::new(channel);

    match cli.command {
        Commands::Register { client_id } => {
            let (y1, y2) = common_lib::gen_params(&common_lib::prover::read_secret());
            let req = RegisterRequest {
                user: client_id,
                y1: y1,
                y2: y2,
            };
            info!("Sending RegisterRequest: {:?}", req);
            let response = client.register(req).await?;
            info!("Received RegisterResponse: {:?}", response.get_ref());
        }
        Commands::Login { client_id } => {

            let k = common_lib::generate_randomness(&BigInt::from(2), &PublicParams::q().sub(2));
            info!("Generate K = {}", k);
            let (r1, r2) = common_lib::gen_params(&k);

            let req = AuthChallengeRequest {
                user: client_id.clone(),
                r1: r1,
                r2: r2,
            };
            info!("Sending AuthChallengeRequest: {:?}", req);
            let response = client.create_authentication_challenge(req).await?;
            info!("Received AuthChallengeResponse: {:?}", response.get_ref());

            let x = common_lib::prover::read_secret();
            let c = BigInt::from_str_radix(response.get_ref().c.as_str(), 16)?;
            let s = common_lib::prover::compute_auth_secret(c, k, x);



            let req = AuthAnswerRequest {
                auth_id: response.into_inner().auth_id,
                s: s.to_str_radix(16),
            };
            info!("Sending AuthAnswerRequest: {:?}", req);

            let response: tonic::Response<AuthAnswerResponse> =
                client.verify_authentication(req).await?;
            info!("Received AuthAnswerResponse: {:?}", response.get_ref());
        }
    }
    Ok(())
}
