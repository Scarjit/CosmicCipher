/*
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 * Copyright (c) 2024 Ferdinand Linnenberg
 *
 * This file is part of CosmicCipher Project, which is dual-licensed under the Apache License 2.0
 * and the MIT License. You may choose either license to govern your use of this file.
 * See the LICENSE-APACHE.md and LICENSE-MIT.md files in the project root for more information.
 */

import * as wasm from "wasm";

wasm.init();

const bob = "bob";
const alice = "alice";

// Bench user creation
let start = performance.now();
wasm.new_user(bob);
let end = performance.now();
console.log("Creation", end - start, "ms");

// Bench user export
start = performance.now();
const exported = wasm.export_user(bob,"test");
end = performance.now();
console.log("Export", end - start, "ms");

// Bench user import
start = performance.now();
wasm.import_user(bob,"test",exported);
end = performance.now();
console.log("Import", end - start, "ms");

// Bench generate instance
start = performance.now();
const instance = wasm.generate_instance(bob);
end = performance.now();
console.log("Instance", end - start, "ms");

// Bench import instance
start = performance.now();
wasm.import_instance(alice,instance);
end = performance.now();
console.log("Import instance", end - start, "ms");

// Init DH KEX on bob
start = performance.now();
let kex_packet_bob = wasm.init_dh_kex(bob,alice);
end = performance.now();
console.log("Init DH KEX (alice)", end - start, "ms");

// Init DH KEX on bob
start = performance.now();
let kex_packet_alice = wasm.init_dh_kex(alice,bob);
end = performance.now();
console.log("Init DH KEX (bob)", end - start, "ms");


// Process DH KEX on alice
start = performance.now();
wasm.finalize_dh_kex(alice,bob,kex_packet_bob);
end = performance.now();
console.log("Finalize DH KEX (alice)", end - start, "ms");

// Process DH KEX on bob
start = performance.now();
wasm.finalize_dh_kex(bob,alice,kex_packet_alice);
end = performance.now();
console.log("Finalize DH KEX (bob)", end - start, "ms");


// Encrypt some text on bob for alice
const text = "Hello Alice!";
start = performance.now();
const encrypted = wasm.encrypt(bob,alice,text);
end = performance.now();
console.log("Encrypt", end - start, "ms");
console.log("Encrypted", encrypted);


// Decrypt some text on alice from bob
start = performance.now();
const decrypted = wasm.decrypt(alice,bob,encrypted);
end = performance.now();
console.log("Decrypt", end - start, "ms");
console.log("Decrypted", decrypted);
