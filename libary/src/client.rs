/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

// Each client has a static private key and a public key
// The public part is expected to be shared with the server
// These keys are used for signing there ECDH ephemeral keys

// For ECDH the client generates an ephemeral key pair and sends the public part the peer
// The client then receives the public part of the peers ephemeral key and computes the shared secret
// The shared secret is then used to derive a symmetric key for encryption

// - curve25519 for ECDH (X25519)
// - ed25519 for signing
// - chacha20-poly1305 for encryption

struct Client {
    secret_key: ed25519_dalek::SecretKey,
}
