/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::State;
use axum::routing::put;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use base64::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use libary::client::Client;

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<HashMap<String, Client>>>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/user", post(new_user))
        .route("/user", get(export_user))
        .route("/user", put(import_user))
        .route("/instance", post(generate_instance))
        .route("/instance", put(import_instance))
        .route("/kex", get(init_kex))
        .route("/kex", put(finish_kex))
        .route("/encrypt", get(encrypt))
        .route("/decrypt", get(decrypt))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn new_user(State(state): State<AppState>, Json(payload): Json<NewUser>) -> StatusCode {
    let mut data = state.data.lock().await;
    data.insert(payload.username.clone(), Client::new_user());
    StatusCode::CREATED
}

#[derive(Deserialize)]
struct NewUser {
    username: String,
}

#[derive(Deserialize)]
struct ExportUser {
    username: String,
    password: String,
}
#[derive(Serialize)]
struct ExportedUser {
    export: String,
}

#[axum::debug_handler]
async fn export_user(
    State(state): State<AppState>,
    Json(payload): Json<ExportUser>,
) -> Result<Json<ExportedUser>, StatusCode> {
    let mut data = state.data.lock().await;
    let client: &mut Client = data
        .get_mut(&payload.username)
        .ok_or(StatusCode::NOT_FOUND)?;
    let export = client
        .export_user(payload.password.as_bytes())
        .map_err(|e| {
            format!("export failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(ExportedUser {
        export: BASE64_STANDARD.encode(export.as_slice()),
    }))
}

#[derive(Deserialize)]
struct ImportUser {
    username: String,
    password: String,
    export: String,
}

async fn import_user(
    State(state): State<AppState>,
    Json(payload): Json<ImportUser>,
) -> Result<StatusCode, StatusCode> {
    let mut data = state.data.lock().await;
    let export = BASE64_STANDARD
        .decode(payload.export.as_bytes())
        .map_err(|e| {
            format!("decode failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client =
        Client::import_user(payload.password.as_bytes(), export.as_slice()).map_err(|e| {
            format!("import failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    data.insert(payload.username.clone(), client);
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct GenerateInstance {
    owner_username: String,
}

#[derive(Serialize)]
struct GeneratedInstance {
    instance: String,
}

async fn generate_instance(
    State(state): State<AppState>,
    Json(payload): Json<GenerateInstance>,
) -> Result<Json<GeneratedInstance>, StatusCode> {
    let mut data = state.data.lock().await;
    let client: &mut Client = data
        .get_mut(&payload.owner_username)
        .ok_or(StatusCode::NOT_FOUND)?;
    let instance = client.generate_instance().map_err(|e| {
        format!("generate failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(GeneratedInstance {
        instance: BASE64_STANDARD.encode(instance.as_slice()),
    }))
}

#[derive(Deserialize)]
struct ImportInstance {
    instance_username: String,
    instance: String,
}

async fn import_instance(
    State(state): State<AppState>,
    Json(payload): Json<ImportInstance>,
) -> Result<StatusCode, StatusCode> {
    let mut data = state.data.lock().await;
    let instance = BASE64_STANDARD
        .decode(payload.instance.as_bytes())
        .map_err(|e| {
            format!("decode failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client = Client::import_instance(instance.as_slice()).map_err(|e| {
        format!("import failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    data.insert(payload.instance_username.clone(), client);
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct InitKex {
    username: String,
    recipient_username: String,
}

#[derive(Serialize)]
struct KexPacket {
    kex: String,
}

async fn init_kex(
    State(state): State<AppState>,
    Json(payload): Json<InitKex>,
) -> Result<Json<KexPacket>, StatusCode> {
    let mut data = state.data.lock().await;
    let client: &mut Client = data
        .get_mut(&payload.username)
        .ok_or(StatusCode::NOT_FOUND)?;
    let kex = client
        .init_dh_kex(&payload.recipient_username)
        .map_err(|e| {
            format!("init failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let kex_packet = client.generate_kex_packet(kex.0, kex.1).map_err(|e| {
        format!("generate failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(KexPacket {
        kex: BASE64_STANDARD.encode(kex_packet.as_slice()),
    }))
}

#[derive(Deserialize)]
struct FinishKex {
    username: String,
    recipient_username: String,
    kex_packet: String,
}

async fn finish_kex(
    State(state): State<AppState>,
    Json(payload): Json<FinishKex>,
) -> Result<StatusCode, StatusCode> {
    let mut data = state.data.lock().await;
    let kex_packet = BASE64_STANDARD
        .decode(payload.kex_packet.as_bytes())
        .map_err(|e| {
            format!("decode failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client: &mut Client = data
        .get_mut(&payload.username)
        .ok_or(StatusCode::NOT_FOUND)?;
    let (pubk, sigpk, vk, sig) = Client::unpack_kex_packet(kex_packet.as_slice()).map_err(|e| {
        format!("unpack failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    client
        .complete_dh_kex(&payload.recipient_username, &pubk, sigpk, vk, sig)
        .map_err(|e| {
            format!("complete failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(StatusCode::CREATED)
}

#[derive(Deserialize)]
struct Encrypt {
    username: String,
    recipient_username: String,
    plaintext: String,
}

#[derive(Serialize)]
struct Ciphertext {
    ciphertext: String,
}

async fn encrypt(
    State(state): State<AppState>,
    Json(payload): Json<Encrypt>,
) -> Result<Json<Ciphertext>, StatusCode> {
    let mut data = state.data.lock().await;
    let client: &mut Client = data
        .get_mut(&payload.username)
        .ok_or(StatusCode::NOT_FOUND)?;
    let ciphertext = client
        .encrypt_message_for_recipient(&payload.recipient_username, payload.plaintext.as_bytes())
        .map_err(|e| {
            format!("encrypt failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(Ciphertext {
        ciphertext: BASE64_STANDARD.encode(ciphertext.as_slice()),
    }))
}

#[derive(Deserialize)]
struct Decrypt {
    username: String,
    sender_username: String,
    ciphertext: String,
}

#[derive(Serialize)]
struct Plaintext {
    plaintext: String,
}

async fn decrypt(
    State(state): State<AppState>,
    Json(payload): Json<Decrypt>,
) -> Result<Json<Plaintext>, StatusCode> {
    let mut data = state.data.lock().await;
    let ciphertext = BASE64_STANDARD
        .decode(payload.ciphertext.as_bytes())
        .map_err(|e| {
            format!("decode failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let client: &mut Client = data
        .get_mut(&payload.username)
        .ok_or(StatusCode::NOT_FOUND)?;
    let plaintext = client
        .decrypt_message_from_sender(&payload.sender_username, ciphertext.as_slice())
        .map_err(|e| {
            format!("decrypt failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    Ok(Json(Plaintext {
        plaintext: String::from_utf8_lossy(plaintext.as_slice()).to_string(),
    }))
}
