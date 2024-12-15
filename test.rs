use wasm_bindgen::prelude::*;
use sha2::{Sha256, Sha512, Digest};

// Enable the WASM bindings
#[wasm_bindgen]
pub fn hash_algorithm(password: &str, algorithm: &str) -> String {
    let digest = match algorithm {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(password);
            hasher.finalize()
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(password);
            hasher.finalize()
        }
        _ => panic!("Unsupported algorithm"),
    };
    format!("{:x}", digest)
}
