use super::DEFAULT_MESSAGE;
use k256::ecdsa::{signature::Signer, Signature, SigningKey};

use rand_core::OsRng;

pub fn sign_message(msg: Option<String>) -> Result<Vec<u8>, String> {
    // if msg is None, we should sign our DEFAULT_MESSAGE..
    // generar aqui el publicKey, "verifying-key "

    todo!()
}
