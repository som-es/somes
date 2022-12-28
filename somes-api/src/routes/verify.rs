use std::{time::{SystemTime, UNIX_EPOCH}, hash::{Hash, Hasher}};

use axum::extract::{Query, State};
use somes_common_lib::SignUpInfo;
use uuid::Uuid;

use crate::server::{VerificationMap, VerificationHasher};

pub fn create_verification_id(hasher: VerificationHasher, signup_info: &SignUpInfo) -> u64 {
    let mut bytes = Uuid::new_v4().as_bytes().to_vec();
    bytes.extend_from_slice(signup_info.email.as_bytes());
    bytes.extend_from_slice(signup_info.username.as_bytes());
    
    bytes.extend_from_slice(&SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos().to_be_bytes());
    
    let mut hasher = hasher.write().unwrap();
    bytes.hash(&mut *hasher);
    hasher.finish()
}


pub async fn verify(
    Query(id): Query<u64>,
    State(verification_map): State<VerificationMap>,
) {

}
