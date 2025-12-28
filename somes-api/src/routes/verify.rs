use sha3::{Digest, Sha3_256};
use somes_common_lib::SignUpInfo;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub fn create_verification_id(signup_info: &SignUpInfo) -> String {
    let mut bytes = Uuid::new_v4().as_bytes().to_vec();
    bytes.extend_from_slice(signup_info.email.as_bytes());
    bytes.extend_from_slice(signup_info.username.as_bytes());

    bytes.extend_from_slice(
        &SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .to_be_bytes(),
    );

    let mut hasher = Sha3_256::new();
    hasher.update(&bytes);

    base16ct::lower::encode_string(&hasher.finalize())
}
