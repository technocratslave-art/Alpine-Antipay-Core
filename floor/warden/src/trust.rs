use crate::capsule::Capsule;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::fs;

const FLOOR_PUBKEY_HEX: &str =
    "0000000000000000000000000000000000000000000000000000000000000000";

pub fn verify_capsule(c: &Capsule) -> bool {
    let sig_path = format!("{}/manifest.toml.sig", c.path);
    let sig_bytes = match fs::read(sig_path) {
        Ok(b) => b,
        Err(_) => return false,
    };

    let manifest = match fs::read_to_string(format!("{}/manifest.toml", c.path)) {
        Ok(m) => m,
        Err(_) => return false,
    };

    let sig = match Signature::from_slice(&sig_bytes) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let key_bytes = hex::decode(FLOOR_PUBKEY_HEX).unwrap_or(vec![0; 32]);
    let vk = VerifyingKey::from_bytes(&key_bytes.try_into().unwrap()).unwrap();

    vk.verify(manifest.as_bytes(), &sig).is_ok()
}
