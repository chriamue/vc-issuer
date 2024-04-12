fn decrypt_jwe(
    jwe_token: &str,
    rsa_private_key: &RsaKeyPair,
) -> Result<String, biscuit::errors::Error> {
    // Parse the JWE token
    let encrypted: Encrypted<CompactJson, Empty> = jwe_token.parse()?;

    // Decrypt the JWE using the provided RSA private key
    let decrypted_data = encrypted.decrypt(
        &KeyManagementAlgorithm::RsaOaep,
        &ContentEncryptionAlgorithm::A256GCM,
        rsa_private_key,
        &SystemRandom::new(),
    )?;

    // Convert the decrypted payload into a String
    let decrypted_string = String::from_utf8(decrypted_data.payload.to_vec())?;

    Ok(decrypted_string)
}
