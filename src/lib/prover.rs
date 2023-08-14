use num_bigint::BigInt;
use num_traits::Zero;

use crate::public_params::PublicParams;

// This should read from file, decrypt with user supplied pwd.
pub fn read_secret() -> BigInt {
    BigInt::from(3)
}

// s = k - c * x (mod q)
pub fn compute_auth_secret(c: BigInt, k: BigInt, x: BigInt) -> BigInt {
    let mut auth_secret = (k - (c * x)) % PublicParams::q();
    if auth_secret < BigInt::zero() {
        auth_secret = auth_secret + PublicParams::q();
    }
    auth_secret
}