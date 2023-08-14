use common_lib::blind_auth_api::blind_auth_server::BlindAuth;
use common_lib::blind_auth_api::{AuthAnswerRequest, AuthChallengeRequest, RegisterRequest};
use common_lib::store::models::{Challenge, User};
use common_lib::store::store::DataStore;
use common_lib::verifier::AuthServer;
use num_bigint::BigInt;
use num_traits::Num;
use tonic::Request;

#[tokio::test]
async fn test_register() {
    // Initialize the AuthServer and DataStore
    let store = DataStore::new();
    let auth_server = AuthServer { store };

    // Create a RegisterRequest for testing
    let register_request = RegisterRequest {
        user: String::from("testuser"),
        y1: String::from("2d"),
        y2: String::from("1f"),
    };

    // Call the register method
    let response = auth_server.register(Request::new(register_request)).await;

    // Assert that the registration was successful
    assert!(response.is_ok());
    let response = response.unwrap();
    assert_eq!(response.get_ref().success, true);

    // Assert that the user is stored in the DataStore
    let stored_user = auth_server.store.get_user(&String::from("testuser"));
    assert!(stored_user.is_some());
}

#[tokio::test]
async fn test_create_authentication_challenge() {
    // Initialize the AuthServer and DataStore
    let store = DataStore::new();
    let auth_server = AuthServer { store };

    // Insert a user into the DataStore
    let user = User {
        id: String::from("dummy"),
        y1: BigInt::from(1),
        y2: BigInt::from(3),
    };
    auth_server.store.insert_user(user.clone());

    // Create an AuthChallengeRequest for testing
    let challenge_request = AuthChallengeRequest {
        user: user.id.clone(),
        r1: BigInt::from(1).to_str_radix(16),
        r2: BigInt::from(1).to_str_radix(16),
    };

    // Call the create_authentication_challenge method
    let response = auth_server
        .create_authentication_challenge(Request::new(challenge_request))
        .await;

    // Assert that the challenge creation was successful
    assert!(response.is_ok());
    let response = response.unwrap();

    // Assert that the challenge is stored in the DataStore
    let stored_challenge = auth_server.store.get_challenge(&response.get_ref().auth_id);
    assert!(stored_challenge.is_some());
}

#[tokio::test]
async fn test_verify_authentication_successful() {
    // Initialize the AuthServer and DataStore
    let store = DataStore::new();
    let auth_server = AuthServer { store };

    // dummy values that should pass verification.
    let r1 = "67419400b47b3283039ae352a461b106487b9ec5657c7765c5520360ccfa365b34564f7f88dbea6378cbc94da0c9ca47a5d5e78f161a7d95db47a5a46b573ea04164d3707a52400e3671e17cea29f966cb1b2ce02ea1552e3b770e97cef5941634c73c73e2ab34ec52b21dce745363f4650b14e20f190ca4708813a77625c8534470035de73d528620440a5323fd27ecc436df28b9765e0c2d82343fa7f018f8f70984d8267444ec75559c16d1750ac08de1517f5336ee99940180ebb9f3bfae84a101d93530b633c68791c70bddb90f99601a2805ec47bcc94b4c8ab55b916e918160050811fc783affccaf7a89d63579a415761ef1e39dbf8a10c4a8d6e99f";
    let r2 = "f84de68e82fd4089a398a4fe1cb1ed3fa71e11210668a2e5d065d1c7c0c4e444294669b5a8f2eb5aa7d0ca3c0e41f195de63c074b447a0ad09b8c616b4801225934d0f14c0d9e44df16248c4b83440c33f9c938488784b741111fae9226f4bedc78aa845981f0a4559200c1c3e959997b52db25a64a42ee496bba80f8e21e40346accbc8697d943ba94cda6374827111210dcce4893e75e79acefc5c6ca5e6c56e7405bddd98e724f9e4c9868b0dc0e40896ddb3397403ebf4865b0c73c523f6c21b68455e5b8f15970b867e0063fcd21e2cbd61b85c20533002631d3e191fd1f23b753eb2f4e05b1266f37bcd37ed97ec0baa66a570811b1bac9fbb194be317";
    let c = "466e3762bbf38f1b505ab055cb322720b0f7b8e7458243706561c712150a128c5bf5620498b6de3868e6546f4bfda41028c9aa84e703e23c344a7ef858ca95af32aee6a824e67dff7839e6d126bd4cc2f95ed53731f8892bbede260d24a829cae94dde0433f59c33939ecf47c7017f679a9b7b8b28c918e869f1bd729bccf57c69c7fcb970525c753ad8daba89ca7330244b80fbeac6caaad9415382aed99e1dba7da3f531a04d98ebdb51c25829116a16a6ad5f7a48f7aeac7cddcd92f94f3a48ae9c3c630028b70257eadcdfb95721d0ad96e1abfd748a7ca6af601d881692d8471ec12e02ebf62827bc8948ca28b1913c4fc5e702450295c64bb3666340c6";
    let s = "73090574eefbfe658d044018d8ff38040514d3c256489f629c28abcea2487225de41c9b910dfdd639362175f60951c0da16692e582320f56671854f57254a2cb382a7f36880dbe739d909cb59d9015fb0756d0f834931697dd5f8efea374653109f72dafe39de86090c30a22f8f5a12e3ee5a6c5e106ebbe6780c32d2993273d654cf051e8a2e2a271497681ea4f13d67484a141c8c24ee95fb579951946996162f9eb924ff68dcda172c0e0d2be757470b3e198cdc1038a0e37ad1fa0e16b2dc9f302c06a3f2aa9eb1a3b899d03ae3d9364dafe0d92288927ef87f6147cfe69afaed9be2fd05d661d2f604d79346377bd4033ce8637d9ffc89bd455651be4ca";

    // Insert a user and challenge into the DataStore
    let user = User {
        id: String::from("dummy"),
        y1: BigInt::from_str_radix("40", 16).unwrap(),
        y2: BigInt::from_str_radix("2d9", 16).unwrap(),
    };
    let challenge = Challenge {
        user_id: String::from("dummy"),
        c: BigInt::from_str_radix(c, 16).unwrap(),
        r2: BigInt::from_str_radix(r2, 16).unwrap(),
        r1: BigInt::from_str_radix(r1, 16).unwrap(),
        id: String::from("challengeid"),
    };
    auth_server.store.insert_user(user.clone());
    auth_server.store.insert_challenge(challenge.clone());

    // Create an AuthAnswerRequest for testing
    let auth_answer_request = AuthAnswerRequest {
        auth_id: challenge.id,
        s: s.to_string(),
    };

    // Call the verify_authentication method
    let response = auth_server
        .verify_authentication(Request::new(auth_answer_request))
        .await;

    // Assert that authentication verification was successful
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_verify_authentication_failed() {
    // Initialize the AuthServer and DataStore
    let store = DataStore::new();
    let auth_server = AuthServer { store };

    // Insert a user and challenge into the DataStore
    let user = User {
        id: String::from("dummy"),
        y1: BigInt::from(1),
        y2: BigInt::from(3),
    };
    auth_server.store.insert_user(user.clone());
    let challenge = Challenge {
        user_id: String::from("dummy"),
        c: BigInt::from(1),
        r2: BigInt::from(3),
        r1: BigInt::from(4),
        id: String::from("challengeid"),
    };
    auth_server.store.insert_challenge(challenge.clone());

    // Create an AuthAnswerRequest for testing
    let auth_answer_request = AuthAnswerRequest {
        auth_id: challenge.id,
        s: BigInt::from(5).to_str_radix(16),
    };

    // Call the verify_authentication method
    let response = auth_server
        .verify_authentication(Request::new(auth_answer_request))
        .await;

    // Assert that authentication verification failed
    assert!(response.is_err());
    assert_eq!(
        response.err().unwrap().code(),
        tonic::Code::PermissionDenied
    );
}
