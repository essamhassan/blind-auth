use crate::public_params::PublicParams;
use log::debug;
use num_bigint::BigInt;

use super::store::models::{Challenge, User};

pub fn verify_authentication(user: &User, challenge: &Challenge, s: BigInt) -> bool {
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
