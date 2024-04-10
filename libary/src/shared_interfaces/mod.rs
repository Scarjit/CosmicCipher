use alloc::vec::Vec;
use anyhow::Error;

pub(crate) trait Signer {
    fn sign(&self, message: &[u8]) -> Result<Vec<u8>, Error>;
    fn verify(&self, message: &[u8], signature: &[u8]) -> Result<bool,Error>;
    fn export_public_key(&self) -> Result<Vec<u8>,Error>;
}