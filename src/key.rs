use ed25519_dalek::{SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

pub fn keypair_from_seed(seed: &str) -> (SigningKey, VerifyingKey) {
    let mut hasher = Sha256::new();
    hasher.update(seed);
    let result = hasher.finalize();

    let seed: [u8; 32] = result[..]
        .try_into()
        .expect("Hash output size does not match ed25519 seed size");

    let signing_key = SigningKey::from_bytes(&seed);
    let verify_key = signing_key.verifying_key();

    (signing_key, verify_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::Signer;
    use ed25519_dalek::Verifier;

    #[test]
    fn test_keypair_from_seed() {
        let seed = "seed123!";
        let (signing_key, verifying_key) = keypair_from_seed(seed);

        let message = b"Hello, World!";
        let signature = signing_key.sign(message);
        assert!(verifying_key.verify(message, &signature).is_ok());
    }
}
