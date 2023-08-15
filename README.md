[![build](https://github.com/essamhassan/blind-auth/actions/workflows/build.yml/badge.svg)](https://github.com/essamhassan/blind-auth/actions/workflows/build.yml)
[![test](https://github.com/essamhassan/blind-auth/actions/workflows/test.yml/badge.svg)](https://github.com/essamhassan/blind-auth/actions/workflows/test.yml)
# Blind Auth
Chaum-Pederson Passwordless Authenticati

# Code structure

```
.
├── build # docker files
│   ├── prover
│   │   └── Dockerfile
│   └── verifier
│       └── Dockerfile
├── build.rs # protoc generator
├── Cargo.lock
├── Cargo.toml
├── docker-compose.yml # Docker compose setup
├── proto
│   └── blind_auth.proto # Service definition
├── README.md # <- You are here
├── src
│   ├── lib
│   │   ├── common.rs # Common libs between prover and verifier
│   │   ├── prover.rs # Prover libs
│   │   ├── public_params.rs # Public values
│   │   ├── store
│   │   │   ├── models.rs # App models
│   │   │   └── store.rs # In-memory store
│   │   ├── store.rs
│   │   └── verifier.rs # Verifier libs
│   ├── prover.rs # Prover entry point
│   └── verifier.rs # Verifier entry point
└── tests
    └── verifier_tests.rs # verifier tests
```

# Run
## Start containers

```bash
docker-compose up
```
## Run client flow

- Register
```bash
docker exec -it -e RUST_LOG=debug prover ./prover http://verifier:50051 register --client-id="clienttest"
```

- Login
```bash
docker exec -it -e RUST_LOG=debug prover ./prover http://verifier:50051 register --client-id="clienttest"
```

# Pending work
- Looks like there is an opportunity to improve performance of BigInt expontiation.
