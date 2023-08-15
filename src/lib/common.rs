use crate::public_params::PublicParams;
use log::info;
use num_bigint::BigInt;
use rand::Rng;

pub mod prover;
pub mod public_params;
pub mod store;
pub mod verifier;

// bundles grpc package for proto
pub mod blind_auth_api {
    tonic::include_proto!("blind_auth");
}


// Generates a random BigInt between min and max
pub fn generate_randomness(min: &BigInt, max: &BigInt) -> BigInt {
    let mut rng = rand::thread_rng();
    let range: BigInt = max - min;

    let random_bytes: Vec<u8> = (0..range.bits() / 8 + 1).map(|_| rng.gen()).collect();
    let random_bigint = BigInt::from_bytes_le(num_bigint::Sign::Plus, &random_bytes);

    min + random_bigint % &range
}


// Generates a param pair p1, p2 where p1=(g^exponent)%p, p2=(h^exponent)%p
pub fn gen_params(exponent: &BigInt) -> (String, String) {
    let p1 = PublicParams::g().modpow(exponent, &PublicParams::p());
    let p2 = PublicParams::h().modpow(exponent, &PublicParams::p());

    info!("generating p1 = {}, p2 = {}", p1, p2);

    (p1.to_str_radix(16), p2.to_str_radix(16))
}
