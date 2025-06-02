//to package metadata and encrypted data
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize)]
struct SecureContainer {
    metadata: String,
    encrypted_data: Vec<u8>,
}

pub fn package_document(metadata: String, encrypted_data: Vec<u8>) -> Vec<u8> {
    let container = SecureContainer { metadata, encrypted_data };
    bincode::serialize(&container).expect("Serialization failed")
}
