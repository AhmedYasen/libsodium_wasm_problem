use sodiumoxide::crypto::sign;
use sodiumoxide::crypto::sign::ed25519::{PublicKey, SecretKey};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Krypt{
    pk: PublicKey,
    sk: SecretKey,
}

#[wasm_bindgen]
impl Krypt {
    pub fn new() -> Self {
        let (pk, sk) = sign::gen_keypair();
        Self {pk, sk}
    }

    pub fn sign_data(&self, data: &str) -> Vec<u8> {
        sign::sign(data.as_bytes(), &self.sk)
    }

    pub fn verify_data(&self, signed_data: &[u8]) -> Vec<u8> {
        sign::verify(&signed_data, &self.pk).unwrap()
    }
}