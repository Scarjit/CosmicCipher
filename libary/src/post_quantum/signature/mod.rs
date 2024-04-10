/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */


use alloc::vec::Vec;
use anyhow::Error;
use oqs::sig;
use oqs::sig::{Algorithm, PublicKey, SecretKey};
use sha3::Digest;

use crate::shared_interfaces::Signer;

mod mod_test;

// Cyber-Dilithium was standardized as ML-DSA in FIPS 204 (https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.ipd.pdf)
const SIGNATURE_ALGORITHM: Algorithm = Algorithm::MlDsa87Ipd;
pub(crate) struct SigningKey{
    public_key: PublicKey,
    secret_key: Option<SecretKey>,
    signature_algorithm: sig::Sig,
}


pub(crate) fn new() -> Result<SigningKey, Error> {
    let signalg = sig::Sig::new(SIGNATURE_ALGORITHM).map_err(Error::msg)?;
    let keypair = signalg.keypair().map_err(Error::msg)?;
    Ok(SigningKey{
        public_key: keypair.0,
        secret_key: Some(keypair.1),
        signature_algorithm: signalg,
    })
}

pub(crate) fn new_public_key_only(public_key: &[u8]) -> Result<SigningKey, Error> {
    let signalg = sig::Sig::new(SIGNATURE_ALGORITHM).map_err(Error::msg)?;
    // OQS_SIG_alg_dilithium_5 + public_key
    // Read until first null byte to get the algorithm
    let (alg, public_key) = public_key.split_at(SIGNATURE_ALGORITHM.name().len());
    if alg != SIGNATURE_ALGORITHM.name().as_bytes() {
        return Err(Error::msg("Invalid algorithm"));
    }

    let public_key = signalg.public_key_from_bytes(public_key)
        .ok_or_else(|| Error::msg("Invalid public key"))?;

    Ok(SigningKey{
        public_key: public_key.to_owned(),
        secret_key: None,
        signature_algorithm: signalg,
    })
}
impl Signer for SigningKey {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error> {
        // Generate a hash of the message to sign using SHA-3 512
        let hash = sha3::Sha3_512::digest(message);

        // Sign the hash
        let secret_key = self.secret_key.as_ref().ok_or_else(|| Error::msg("No secret key"))?;
        let signature = self.signature_algorithm.sign(&hash, secret_key).map_err(Error::msg)?;
        Ok(signature.into_vec())
    }

    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool,Error> {
        // Generate a hash of the message to sign using SHA-3 512
        let hash = sha3::Sha3_512::digest(message);

        // Validate the signature against the message hash
        let signature = self.signature_algorithm.signature_from_bytes(signature)
            .ok_or_else(|| Error::msg("Invalid signature"))?;


        let validation_result = self.signature_algorithm.verify(&hash, &signature, &self.public_key);

        // Let's check the validation result
        match validation_result {
            Ok(()) => Ok(true),
            Err(e) => {
                // Check the type of Error
                match e {
                    oqs::Error::Error => Ok(false),
                    _ => Err(Error::msg(e))
                }
            }
        }
    }


    fn export_public_key(&self) -> Result<Vec<u8>,Error> {
        // OQS_SIG_alg_dilithium_5 + public_key
        let mut public_key: Vec<u8> = Vec::new();
        public_key.extend_from_slice(SIGNATURE_ALGORITHM.name().as_bytes());
        public_key.extend_from_slice(self.public_key.as_ref());
        Ok(public_key)
    }

}