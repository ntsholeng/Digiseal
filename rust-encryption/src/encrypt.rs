//This code encrypts the document using AES-256
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
use rand::Rng;

pub fn encrypt_document(data: &[u8]) -> Vec<u8> {
    let key = Key::from_slice(&rand::thread_rng().gen::<[u8; 32]>());
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&rand::thread_rng().gen::<[u8; 12]>());

    cipher.encrypt(nonce, data).expect("Encryption failed")
}
