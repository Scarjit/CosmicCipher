/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use base64::prelude::*;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

use libary::client::Client;

static mut CLIENT: Lazy<HashMap<String, Client>> = Lazy::new(HashMap::new);
#[wasm_bindgen]
pub fn new_user(username: &str) {
    unsafe {
        CLIENT.insert(username.to_string(), Client::new_user());
    }
}

#[wasm_bindgen]
pub fn export_user(username: &str, password: &str) -> Result<String, JsError> {
    let export;
    unsafe {
        match CLIENT.get_mut(username) {
            None => {
                return Err(JsError::new(&format!("User {} not found", username)));
            }
            Some(v) => {
                export = v
                    .export_user(password.as_bytes())
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
            }
        }
    }
    return Ok(BASE64_STANDARD.encode(export.as_slice()));
}

#[wasm_bindgen]
pub fn import_user(username: &str, password: &str, export: &str) -> Result<(), JsError> {
    let export = BASE64_STANDARD
        .decode(export.as_bytes())
        .map_err(|e| JsError::new(&format!("{}", e)))?;
    unsafe {
        let u = Client::import_user(password.as_bytes(), export.as_slice())
            .map_err(|e| JsError::new(&format!("{}", e)))?;
        CLIENT.insert(username.to_string(), u);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn generate_instance(owner_username: &str) -> Result<String, JsError> {
    let instance;
    unsafe {
        match CLIENT.get_mut(owner_username) {
            None => {
                return Err(JsError::new(&format!("User {} not found", owner_username)));
            }
            Some(v) => {
                instance = v
                    .generate_instance()
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
            }
        }
    }
    Ok(BASE64_STANDARD.encode(instance.as_slice()))
}

#[wasm_bindgen]
pub fn import_instance(instance_username: &str, instance: &str) -> Result<(), JsError> {
    let instance = BASE64_STANDARD
        .decode(instance.as_bytes())
        .map_err(|e| JsError::new(&format!("{}", e)))?;
    unsafe {
        let instance = Client::import_instance(instance.as_slice())
            .map_err(|e| JsError::new(&format!("{}", e)))?;
        CLIENT.insert(instance_username.to_string(), instance);
    }
    Ok(())
}

#[wasm_bindgen]
pub fn init_dh_kex(username: &str, recipient_username: &str) -> Result<String, JsError> {
    unsafe {
        return match CLIENT.get_mut(username) {
            None => Err(JsError::new(&format!("User {} not found", username))),
            Some(v) => {
                let kexdata = v
                    .init_dh_kex(recipient_username)
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
                let kex_packet = v
                    .generate_kex_packet(kexdata.0, kexdata.1)
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
                Ok(BASE64_STANDARD.encode(kex_packet.as_slice()))
            }
        };
    }
}

#[wasm_bindgen]
pub fn finalize_dh_kex(
    username: &str,
    recipient_username: &str,
    kex_packet: &str,
) -> Result<(), JsError> {
    let kex_packet = BASE64_STANDARD
        .decode(kex_packet.as_bytes())
        .map_err(|e| JsError::new(&format!("{}", e)))?;
    unsafe {
        return match CLIENT.get_mut(username) {
            None => Err(JsError::new(&format!("User {} not found", username))),
            Some(v) => {
                let (pubk, sigpk, vk, sig) = Client::unpack_kex_packet(kex_packet.as_slice())
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
                v.complete_dh_kex(recipient_username, &pubk, sigpk, vk, sig)
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
                Ok(())
            }
        };
    }
}

#[wasm_bindgen]
pub fn encrypt(
    username: &str,
    recipient_username: &str,
    plaintext: &str,
) -> Result<String, JsError> {
    unsafe {
        return match CLIENT.get_mut(username) {
            None => Err(JsError::new(&format!("User {} not found", username))),
            Some(v) => {
                let ciphertext = v
                    .encrypt_message_for_recipient(recipient_username, plaintext.as_bytes())
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
                Ok(BASE64_STANDARD.encode(ciphertext.as_slice()))
            }
        };
    }
}

#[wasm_bindgen]
pub fn decrypt(username: &str, sender_username: &str, ciphertext: &str) -> Result<String, JsError> {
    let ciphertext = BASE64_STANDARD
        .decode(ciphertext.as_bytes())
        .map_err(|e| JsError::new(&format!("{}", e)))?;
    unsafe {
        return match CLIENT.get_mut(username) {
            None => Err(JsError::new(&format!("User {} not found", username))),
            Some(v) => {
                let plaintext = v
                    .decrypt_message_from_sender(sender_username, ciphertext.as_slice())
                    .map_err(|e| JsError::new(&format!("{}", e)))?;
                Ok(String::from_utf8(plaintext).map_err(|e| JsError::new(&format!("{}", e)))?)
            }
        };
    }
}
