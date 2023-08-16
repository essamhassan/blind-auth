use num_bigint::BigInt;

pub struct PublicParams;

// https://datatracker.ietf.org/doc/html/rfc3526#section-3
const P_PRIME: &str = "ffffffffffffffffc90fdaa22168c234c4c6628b80dc1cd129024e088a67cc74020bbea63b139b22514a08798e3404ddef9519b3cd3a431b302b0a6df25f14374fe1356d6d51c245e485b576625e7ec6f44c42e9a637ed6b0bff5cb6f406b7edee386bfb5a899fa5ae9f24117c4b1fe649286651ece45b3dc2007cb8a163bf0598da48361c55d39a69163fa8fd24cf5f83655d23dca3ad961c62f356208552bb9ed529077096966d670c354e4abc9804f1746c08ca18217c32905e462e36ce3be39e772c180e86039b2783a2ec07a28fb5c55df06f4c52c9de2bcbf6955817183995497cea956ae515d2261898fa051015728e5a8aacaa68ffffffffffffffff";
const Q_SUBPRIME: &str = "7fffffffffffffffe487ed5110b4611a62633145c06e0e68948127044533e63a0105df531d89cd9128a5043cc71a026ef7ca8cd9e69d218d98158536f92f8a1ba7f09ab6b6a8e122f242dabb312f3f637a262174d31bf6b585ffae5b7a035bf6f71c35fdad44cfd2d74f9208be258ff324943328f6722d9ee1003e5c50b1df82cc6d241b0e2ae9cd348b1fd47e9267afc1b2ae91ee51d6cb0e3179ab1042a95dcf6a9483b84b4b36b3861aa7255e4c0278ba3604650c10be19482f23171b671df1cf3b960c074301cd93c1d17603d147dae2aef837a62964ef15e5fb4aac0b8c1ccaa4be754ab5728ae9130c4c7d02880ab9472d455655347fffffffffffffff";

impl PublicParams {
    pub fn g() -> BigInt {
        BigInt::from(4)
    }

    pub fn h() -> BigInt {
        BigInt::from(9)
    }

    pub fn p() -> BigInt {
        BigInt::parse_bytes(P_PRIME.as_bytes(), 16).unwrap()
    }

    pub fn q() -> BigInt {
        BigInt::parse_bytes(Q_SUBPRIME.as_bytes(), 16).unwrap()
    }
}
