// Verification
use k256::ecdsa::VerifyingKey;

use super::DEFAULT_MESSAGE;

pub fn verify(msg: Option<String>, my_pkh: String, signature: String) -> Result<(), String> {
    // my_pkh y signature son string encodados en hex, debemos
    // usar hex::decode.
    // el tipo String tiene un metodo as_bytes util aqui
    //
    // Si msg es None, usamos nuestro mensaje DEFAULT_MESSAGE
    // https://docs.rs/k256/0.10.4/k256/ecdsa/index.html

    todo!()
}
