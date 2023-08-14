use num_bigint::BigInt;

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub y1: BigInt,
    pub y2: BigInt,
}

#[derive(Clone)]
pub struct Challenge {
    pub c: BigInt,
    pub r1: BigInt,
    pub r2: BigInt,
    pub user_id: String,
    pub id: String,
}

#[derive(Clone)]
pub struct Session {
    pub id: String,
    pub user_id: String
}