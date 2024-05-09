# Dependencies

This project wouldn't be possible without the following amazing dependencies:

| Name | Version | Authors | Repository | License | Description |
|----|----|----|----|----|----|
| "addr2line"| "0.21.0"| | "https://github.com/gimli-rs/addr2line"| "Apache-2.0 OR MIT"| "A cross-platform symbolication library written in Rust|  using `gimli`"|
| "adler"| "1.0.2"| "Jonas Schievink <jonasschievink@gmail.com>"| "https://github.com/jonas-schievink/adler.git"| "0BSD OR Apache-2.0 OR MIT"| "A simple clean-room implementation of the Adler-32 checksum"|
| "aead"| "0.5.2"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits"| "Apache-2.0 OR MIT"| "Traits for Authenticated Encryption with Associated Data (AEAD) algorithms|  such as AES-GCM as ChaCha20Poly1305|  which provide a high-level API"|
| "ahash"| "0.8.11"| "Tom Kaitchuck <Tom.Kaitchuck@gmail.com>"| "https://github.com/tkaitchuck/ahash"| "Apache-2.0 OR MIT"| "A non-cryptographic hash function using AES-NI for high performance"|
| "allocator-api2"| "0.2.18"| "Zakarum <zaq.dev@icloud.com>"| "https://github.com/zakarumych/allocator-api2"| "Apache-2.0 OR MIT"| "Mirror of Rust's allocator API"|
| "anyhow"| "1.0.82"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/anyhow"| "Apache-2.0 OR MIT"| "Flexible concrete Error type built on std::error::Error"|
| "argon2"| "0.5.3"| "RustCrypto Developers"| "https://github.com/RustCrypto/password-hashes/tree/master/argon2"| "Apache-2.0 OR MIT"| "Pure Rust implementation of the Argon2 password hashing function with support for the Argon2d|  Argon2i|  and Argon2id algorithmic variants"|
| "async-trait"| "0.1.80"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/async-trait"| "Apache-2.0 OR MIT"| "Type erasure for async trait methods"|
| "axum"| "0.7.5"| | "https://github.com/tokio-rs/axum"| "MIT"| "Web framework that focuses on ergonomics and modularity"|
| "axum-core"| "0.4.3"| | "https://github.com/tokio-rs/axum"| "MIT"| "Core types and traits for axum"|
| "axum-macros"| "0.4.1"| | "https://github.com/tokio-rs/axum"| "MIT"| "Macros for axum"|
| "backtrace"| "0.3.71"| "The Rust Project Developers"| "https://github.com/rust-lang/backtrace-rs"| "Apache-2.0 OR MIT"| "A library to acquire a stack trace (backtrace) at runtime in a Rust program."|
| "base64"| "0.13.1"| "Alice Maz <alice@alicemaz.com>|Marshall Pierce <marshall@mpierce.org>"| "https://github.com/marshallpierce/rust-base64"| "Apache-2.0 OR MIT"| "encodes and decodes base64 as bytes or utf8"|
| "base64"| "0.22.1"| "Marshall Pierce <marshall@mpierce.org>"| "https://github.com/marshallpierce/rust-base64"| "Apache-2.0 OR MIT"| "encodes and decodes base64 as bytes or utf8"|
| "base64ct"| "1.6.0"| "RustCrypto Developers"| "https://github.com/RustCrypto/formats/tree/master/base64ct"| "Apache-2.0 OR MIT"| "Pure Rust implementation of Base64 (RFC 4648) which avoids any usages of data-dependent branches/LUTs and thereby provides portable ""best effort"" constant-time operation and embedded-friendly no_std support"|
| "bitvec"| "1.0.1"| | "https://github.com/bitvecto-rs/bitvec"| "MIT"| "Addresses memory by bits|  for packed collections and bitfields"|
| "blake2"| "0.10.6"| "RustCrypto Developers"| "https://github.com/RustCrypto/hashes"| "Apache-2.0 OR MIT"| "BLAKE2 hash functions"|
| "block-buffer"| "0.10.4"| "RustCrypto Developers"| "https://github.com/RustCrypto/utils"| "Apache-2.0 OR MIT"| "Buffer type for block processing of data"|
| "bson"| "2.10.0"| "Y. T. Chung <zonyitoo@gmail.com>|Kevin Yeh <kevinyeah@utexas.edu>|Saghm Rossi <saghmrossi@gmail.com>|Patrick Freed <patrick.freed@mongodb.com>|Isabel Atkinson <isabel.atkinson@mongodb.com>|Abraham Egnor <abraham.egnor@mongodb.com>"| "https://github.com/mongodb/bson-rust"| "MIT"| "Encoding and decoding support for BSON in Rust"|
| "bumpalo"| "3.16.0"| "Nick Fitzgerald <fitzgen@gmail.com>"| "https://github.com/fitzgen/bumpalo"| "Apache-2.0 OR MIT"| "A fast bump allocation arena for Rust."|
| "bytes"| "1.6.0"| "Carl Lerche <me@carllerche.com>|Sean McArthur <sean@seanmonstar.com>"| "https://github.com/tokio-rs/bytes"| "MIT"| "Types and traits for working with bytes"|
| "cc"| "1.0.97"| "Alex Crichton <alex@alexcrichton.com>"| "https://github.com/rust-lang/cc-rs"| "Apache-2.0 OR MIT"| "A build-time dependency for Cargo build scripts to assist in invoking the native C compiler to compile native C code into a static archive to be linked into Rust code."|
| "cfg-if"| "1.0.0"| "Alex Crichton <alex@alexcrichton.com>"| "https://github.com/alexcrichton/cfg-if"| "Apache-2.0 OR MIT"| "A macro to ergonomically define an item depending on a large number of #[cfg] parameters. Structured like an if-else chain|  the first matching branch is the item that gets emitted."|
| "chacha20"| "0.9.1"| "RustCrypto Developers"| "https://github.com/RustCrypto/stream-ciphers"| "Apache-2.0 OR MIT"| "The ChaCha20 stream cipher (RFC 8439) implemented in pure Rust using traits from the RustCrypto `cipher` crate|  with optional architecture-specific hardware acceleration (AVX2|  SSE2). Additionally provides the ChaCha8|  ChaCha12|  XChaCha20|  XChaCha12 and XChaCha8 stream ciphers|  and also optional rand_core-compatible RNGs based on those ciphers."|
| "chacha20poly1305"| "0.10.1"| "RustCrypto Developers"| "https://github.com/RustCrypto/AEADs/tree/master/chacha20poly1305"| "Apache-2.0 OR MIT"| "Pure Rust implementation of the ChaCha20Poly1305 Authenticated Encryption with Additional Data Cipher (RFC 8439) with optional architecture-specific hardware acceleration. Also contains implementations of the XChaCha20Poly1305 extended nonce variant of ChaCha20Poly1305|  and the reduced-round ChaCha8Poly1305 and ChaCha12Poly1305 lightweight variants."|
| "cipher"| "0.4.4"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits"| "Apache-2.0 OR MIT"| "Traits for describing block ciphers and stream ciphers"|
| "console_error_panic_hook"| "0.1.7"| "Nick Fitzgerald <fitzgen@gmail.com>"| "https://github.com/rustwasm/console_error_panic_hook"| "Apache-2.0 OR MIT"| "A panic hook for `wasm32-unknown-unknown` that logs panics to `console.error`"|
| "const-oid"| "0.9.6"| "RustCrypto Developers"| "https://github.com/RustCrypto/formats/tree/master/const-oid"| "Apache-2.0 OR MIT"| "Const-friendly implementation of the ISO/IEC Object Identifier (OID) standard as defined in ITU X.660|  with support for BER/DER encoding/decoding as well as heapless no_std (i.e. embedded) support"|
| "cpufeatures"| "0.2.12"| "RustCrypto Developers"| "https://github.com/RustCrypto/utils"| "Apache-2.0 OR MIT"| "Lightweight runtime CPU feature detection for aarch64|  loongarch64|  and x86/x86_64 targets|   with no_std support and support for mobile targets including Android and iOS"|
| "crypto-common"| "0.1.6"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits"| "Apache-2.0 OR MIT"| "Common cryptographic traits"|
| "curve25519-dalek"| "4.1.2"| "Isis Lovecruft <isis@patternsinthevoid.net>|Henry de Valence <hdevalence@hdevalence.ca>"| "https://github.com/dalek-cryptography/curve25519-dalek/tree/main/curve25519-dalek"| "BSD-3-Clause"| "A pure-Rust implementation of group operations on ristretto255 and Curve25519"|
| "curve25519-dalek-derive"| "0.1.1"| | "https://github.com/dalek-cryptography/curve25519-dalek"| "Apache-2.0 OR MIT"| "curve25519-dalek Derives"|
| "der"| "0.7.9"| "RustCrypto Developers"| "https://github.com/RustCrypto/formats/tree/master/der"| "Apache-2.0 OR MIT"| "Pure Rust embedded-friendly implementation of the Distinguished Encoding Rules (DER) for Abstract Syntax Notation One (ASN.1) as described in ITU X.690 with full support for heapless no_std targets"|
| "deranged"| "0.3.11"| "Jacob Pratt <jacob@jhpratt.dev>"| "https://github.com/jhpratt/deranged"| "Apache-2.0 OR MIT"| "Ranged integers"|
| "digest"| "0.10.7"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits"| "Apache-2.0 OR MIT"| "Traits for cryptographic hash functions and message authentication codes"|
| "ed25519"| "2.2.3"| "RustCrypto Developers"| "https://github.com/RustCrypto/signatures/tree/master/ed25519"| "Apache-2.0 OR MIT"| "Edwards Digital Signature Algorithm (EdDSA) over Curve25519 (as specified in RFC 8032) support library providing signature type definitions and PKCS#8 private key decoding/encoding support"|
| "ed25519-dalek"| "2.1.1"| "isis lovecruft <isis@patternsinthevoid.net>|Tony Arcieri <bascule@gmail.com>|Michael Rosenberg <michael@mrosenberg.pub>"| "https://github.com/dalek-cryptography/curve25519-dalek/tree/main/ed25519-dalek"| "BSD-3-Clause"| "Fast and efficient ed25519 EdDSA key generations|  signing|  and verification in pure Rust."|
| "equivalent"| "1.0.1"| | "https://github.com/cuviper/equivalent"| "Apache-2.0 OR MIT"| "Traits for key comparison in maps."|
| "fiat-crypto"| "0.2.8"| "Fiat Crypto library authors <jgross@mit.edu>"| "https://github.com/mit-plv/fiat-crypto"| "Apache-2.0 OR BSD-1-Clause OR MIT"| "Fiat-crypto generated Rust"|
| "fnv"| "1.0.7"| "Alex Crichton <alex@alexcrichton.com>"| "https://github.com/servo/rust-fnv"| "Apache-2.0 OR MIT"| "Fowler–Noll–Vo hash function"|
| "form_urlencoded"| "1.2.1"| "The rust-url developers"| "https://github.com/servo/rust-url"| "Apache-2.0 OR MIT"| "Parser and serializer for the application/x-www-form-urlencoded syntax|  as used by HTML forms."|
| "funty"| "2.0.0"| "myrrlyn <self@myrrlyn.dev>"| "https://github.com/myrrlyn/funty"| "MIT"| "Trait generalization over the primitive types"|
| "futures-channel"| "0.3.30"| | "https://github.com/rust-lang/futures-rs"| "Apache-2.0 OR MIT"| "Channels for asynchronous communication using futures-rs."|
| "futures-core"| "0.3.30"| | "https://github.com/rust-lang/futures-rs"| "Apache-2.0 OR MIT"| "The core traits and types in for the `futures` library."|
| "futures-task"| "0.3.30"| | "https://github.com/rust-lang/futures-rs"| "Apache-2.0 OR MIT"| "Tools for working with tasks."|
| "futures-util"| "0.3.30"| | "https://github.com/rust-lang/futures-rs"| "Apache-2.0 OR MIT"| "Common utilities and extension traits for the futures-rs library."|
| "generic-array"| "0.14.7"| "Bartłomiej Kamiński <fizyk20@gmail.com>|Aaron Trent <novacrazy@gmail.com>"| "https://github.com/fizyk20/generic-array.git"| "MIT"| "Generic types implementing functionality of arrays"|
| "getrandom"| "0.2.14"| "The Rand Project Developers"| "https://github.com/rust-random/getrandom"| "Apache-2.0 OR MIT"| "A small cross-platform library for retrieving random data from system source"|
| "gimli"| "0.28.1"| | "https://github.com/gimli-rs/gimli"| "Apache-2.0 OR MIT"| "A library for reading and writing the DWARF debugging format."|
| "hashbrown"| "0.14.5"| "Amanieu d'Antras <amanieu@gmail.com>"| "https://github.com/rust-lang/hashbrown"| "Apache-2.0 OR MIT"| "A Rust port of Google's SwissTable hash map"|
| "heck"| "0.4.1"| "Without Boats <woboats@gmail.com>"| "https://github.com/withoutboats/heck"| "Apache-2.0 OR MIT"| "heck is a case conversion library."|
| "hermit-abi"| "0.3.9"| "Stefan Lankes"| "https://github.com/hermit-os/hermit-rs"| "Apache-2.0 OR MIT"| "Hermit system calls definitions."|
| "hex"| "0.4.3"| "KokaKiwi <kokakiwi@kokakiwi.net>"| "https://github.com/KokaKiwi/rust-hex"| "Apache-2.0 OR MIT"| "Encoding and decoding data into/from hexadecimal representation."|
| "http"| "1.1.0"| "Alex Crichton <alex@alexcrichton.com>|Carl Lerche <me@carllerche.com>|Sean McArthur <sean@seanmonstar.com>"| "https://github.com/hyperium/http"| "Apache-2.0 OR MIT"| "A set of types for representing HTTP requests and responses."|
| "http-body"| "1.0.0"| "Carl Lerche <me@carllerche.com>|Lucio Franco <luciofranco14@gmail.com>|Sean McArthur <sean@seanmonstar.com>"| "https://github.com/hyperium/http-body"| "MIT"| "Trait representing an asynchronous|  streaming|  HTTP request or response body."|
| "http-body-util"| "0.1.1"| "Carl Lerche <me@carllerche.com>|Lucio Franco <luciofranco14@gmail.com>|Sean McArthur <sean@seanmonstar.com>"| "https://github.com/hyperium/http-body"| "MIT"| "Combinators and adapters for HTTP request or response bodies."|
| "httparse"| "1.8.0"| "Sean McArthur <sean@seanmonstar.com>"| "https://github.com/seanmonstar/httparse"| "Apache-2.0 OR MIT"| "A tiny|  safe|  speedy|  zero-copy HTTP/1.x parser."|
| "httpdate"| "1.0.3"| "Pyfisch <pyfisch@posteo.org>"| "https://github.com/pyfisch/httpdate"| "Apache-2.0 OR MIT"| "HTTP date parsing and formatting"|
| "hyper"| "1.3.1"| "Sean McArthur <sean@seanmonstar.com>"| "https://github.com/hyperium/hyper"| "MIT"| "A fast and correct HTTP library."|
| "hyper-util"| "0.1.3"| "Sean McArthur <sean@seanmonstar.com>"| "https://github.com/hyperium/hyper-util"| "MIT"| "hyper utilities"|
| "indexmap"| "2.2.6"| | "https://github.com/indexmap-rs/indexmap"| "Apache-2.0 OR MIT"| "A hash table with consistent order and fast iteration."|
| "inout"| "0.1.3"| "RustCrypto Developers"| "https://github.com/RustCrypto/utils"| "Apache-2.0 OR MIT"| "Custom reference types for code generic over in-place and buffer-to-buffer modes of operation."|
| "itoa"| "1.0.11"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/itoa"| "Apache-2.0 OR MIT"| "Fast integer primitive to string conversion"|
| "js-sys"| "0.3.69"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/js-sys"| "Apache-2.0 OR MIT"| "Bindings for all JS global objects and functions in all JS environments like Node.js and browsers|  built on `#[wasm_bindgen]` using the `wasm-bindgen` crate."|
| "keccak"| "0.1.5"| "RustCrypto Developers"| "https://github.com/RustCrypto/sponges/tree/master/keccak"| "Apache-2.0 OR MIT"| "Pure Rust implementation of the Keccak sponge function including the keccak-f and keccak-p variants"|
| "lazy_static"| "1.4.0"| "Marvin Löbel <loebel.marvin@gmail.com>"| "https://github.com/rust-lang-nursery/lazy-static.rs"| "Apache-2.0 OR MIT"| "A macro for declaring lazily evaluated statics in Rust."|
| "libary"| "0.1.0"| | "https://github.com/Scarjit/CosmicCipher"| "Apache-2.0 OR MIT"| |
| "libc"| "0.2.153"| "The Rust Project Developers"| "https://github.com/rust-lang/libc"| "Apache-2.0 OR MIT"| "Raw FFI bindings to platform libraries like libc."|
| "log"| "0.4.21"| "The Rust Project Developers"| "https://github.com/rust-lang/log"| "Apache-2.0 OR MIT"| "A lightweight logging facade for Rust"|
| "lz4_flex"| "0.11.3"| "Pascal Seitz <pascal.seitz@gmail.com>|Arthur Silva <arthurprs@gmail.com>|ticki <Ticki@users.noreply.github.com>"| "https://github.com/pseitz/lz4_flex"| "MIT"| "Fastest LZ4 implementation in Rust|  no unsafe by default."|
| "matchit"| "0.7.3"| "Ibraheem Ahmed <ibraheem@ibraheem.ca>"| "https://github.com/ibraheemdev/matchit"| "MIT AND BSD-3-Clause"| "A high performance|  zero-copy URL router."|
| "memchr"| "2.7.2"| "Andrew Gallant <jamslam@gmail.com>|bluss"| "https://github.com/BurntSushi/memchr"| "MIT OR Unlicense"| "Provides extremely fast (uses SIMD on x86_64|  aarch64 and wasm32) routines for 1|  2 or 3 byte search and single substring search."|
| "mime"| "0.3.17"| "Sean McArthur <sean@seanmonstar.com>"| "https://github.com/hyperium/mime"| "Apache-2.0 OR MIT"| "Strongly Typed Mimes"|
| "miniz_oxide"| "0.7.2"| "Frommi <daniil.liferenko@gmail.com>|oyvindln <oyvindln@users.noreply.github.com>"| "https://github.com/Frommi/miniz_oxide/tree/master/miniz_oxide"| "Apache-2.0 OR MIT OR Zlib"| "DEFLATE compression and decompression library rewritten in Rust based on miniz"|
| "mio"| "0.8.11"| "Carl Lerche <me@carllerche.com>|Thomas de Zeeuw <thomasdezeeuw@gmail.com>|Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/mio"| "MIT"| "Lightweight non-blocking I/O."|
| "nu-ansi-term"| "0.46.0"| "ogham@bsago.me|Ryan Scheel (Havvy) <ryan.havvy@gmail.com>|Josh Triplett <josh@joshtriplett.org>|The Nushell Project Developers"| "https://github.com/nushell/nu-ansi-term"| "MIT"| "Library for ANSI terminal colors and styles (bold|  underline)"|
| "num-conv"| "0.1.0"| "Jacob Pratt <jacob@jhpratt.dev>"| "https://github.com/jhpratt/num-conv"| "Apache-2.0 OR MIT"| "`num_conv` is a crate to convert between integer types without using `as` casts. This provides better certainty when refactoring|  makes the exact behavior of code more explicit|  and allows using turbofish syntax."|
| "num_cpus"| "1.16.0"| "Sean McArthur <sean@seanmonstar.com>"| "https://github.com/seanmonstar/num_cpus"| "Apache-2.0 OR MIT"| "Get the number of CPUs on a machine."|
| "object"| "0.32.2"| | "https://github.com/gimli-rs/object"| "Apache-2.0 OR MIT"| "A unified interface for reading and writing object file formats."|
| "once_cell"| "1.19.0"| "Aleksey Kladov <aleksey.kladov@gmail.com>"| "https://github.com/matklad/once_cell"| "Apache-2.0 OR MIT"| "Single assignment cells and lazy values."|
| "opaque-debug"| "0.3.1"| "RustCrypto Developers"| "https://github.com/RustCrypto/utils"| "Apache-2.0 OR MIT"| "Macro for opaque Debug trait implementation"|
| "overload"| "0.1.1"| "Daniel Salvadori <danaugrs@gmail.com>"| "https://github.com/danaugrs/overload"| "MIT"| "Provides a macro to simplify operator overloading."|
| "password-hash"| "0.5.0"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits/tree/master/password-hash"| "Apache-2.0 OR MIT"| "Traits which describe the functionality of password hashing algorithms|  as well as a `no_std`-friendly implementation of the PHC string format (a well-defined subset of the Modular Crypt Format a.k.a. MCF)"|
| "percent-encoding"| "2.3.1"| "The rust-url developers"| "https://github.com/servo/rust-url/"| "Apache-2.0 OR MIT"| "Percent encoding and decoding"|
| "pin-project"| "1.1.5"| | "https://github.com/taiki-e/pin-project"| "Apache-2.0 OR MIT"| "A crate for safe and ergonomic pin-projection."|
| "pin-project-internal"| "1.1.5"| | "https://github.com/taiki-e/pin-project"| "Apache-2.0 OR MIT"| "Implementation detail of the `pin-project` crate."|
| "pin-project-lite"| "0.2.14"| | "https://github.com/taiki-e/pin-project-lite"| "Apache-2.0 OR MIT"| "A lightweight version of pin-project written with declarative macros."|
| "pin-utils"| "0.1.0"| "Josef Brandl <mail@josefbrandl.de>"| "https://github.com/rust-lang-nursery/pin-utils"| "Apache-2.0 OR MIT"| "Utilities for pinning"|
| "pkcs8"| "0.10.2"| "RustCrypto Developers"| "https://github.com/RustCrypto/formats/tree/master/pkcs8"| "Apache-2.0 OR MIT"| "Pure Rust implementation of Public-Key Cryptography Standards (PKCS) #8: Private-Key Information Syntax Specification (RFC 5208)|  with additional support for PKCS#8v2 asymmetric key packages (RFC 5958)"|
| "platforms"| "3.4.0"| "Tony Arcieri <bascule@gmail.com>|Sergey ""Shnatsel"" Davidoff <shnatsel@gmail.com>"| "https://github.com/rustsec/rustsec/tree/main/platforms"| "Apache-2.0 OR MIT"| "Rust platform registry with information about valid Rust platforms (target triple|  target_arch|  target_os) sourced from the Rust compiler."|
| "poly1305"| "0.8.0"| "RustCrypto Developers"| "https://github.com/RustCrypto/universal-hashes"| "Apache-2.0 OR MIT"| "The Poly1305 universal hash function and message authentication code"|
| "powerfmt"| "0.2.0"| "Jacob Pratt <jacob@jhpratt.dev>"| "https://github.com/jhpratt/powerfmt"| "Apache-2.0 OR MIT"| "`powerfmt` is a library that provides utilities for formatting values. This crate makes it     significantly easier to support filling to a minimum width with alignment|  avoid heap     allocation|  and avoid repetitive calculations."|
| "ppv-lite86"| "0.2.17"| "The CryptoCorrosion Contributors"| "https://github.com/cryptocorrosion/cryptocorrosion"| "Apache-2.0 OR MIT"| "Implementation of the crypto-simd API for x86"|
| "proc-macro2"| "1.0.79"| "David Tolnay <dtolnay@gmail.com>|Alex Crichton <alex@alexcrichton.com>"| "https://github.com/dtolnay/proc-macro2"| "Apache-2.0 OR MIT"| "A substitute implementation of the compiler's `proc_macro` API to decouple token-based libraries from the procedural macro use case."|
| "quote"| "1.0.36"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/quote"| "Apache-2.0 OR MIT"| "Quasi-quoting macro quote!(...)"|
| "radium"| "0.7.0"| "Nika Layzell <nika@thelayzells.com>|myrrlyn <self@myrrlyn.dev>"| "https://github.com/bitvecto-rs/radium"| "MIT"| "Portable interfaces for maybe-atomic types"|
| "rand"| "0.8.5"| "The Rand Project Developers|The Rust Project Developers"| "https://github.com/rust-random/rand"| "Apache-2.0 OR MIT"| "Random number generators and other randomness functionality."|
| "rand_chacha"| "0.3.1"| "The Rand Project Developers|The Rust Project Developers|The CryptoCorrosion Contributors"| "https://github.com/rust-random/rand"| "Apache-2.0 OR MIT"| "ChaCha random number generator"|
| "rand_core"| "0.6.4"| "The Rand Project Developers|The Rust Project Developers"| "https://github.com/rust-random/rand"| "Apache-2.0 OR MIT"| "Core random number generator traits and tools for implementation."|
| "rest"| "0.1.0"| | | | |
| "rustc-demangle"| "0.1.24"| "Alex Crichton <alex@alexcrichton.com>"| "https://github.com/rust-lang/rustc-demangle"| "Apache-2.0 OR MIT"| "Rust compiler symbol demangling."|
| "rustc_version"| "0.4.0"| "Dirkjan Ochtman <dirkjan@ochtman.nl>|Marvin Löbel <loebel.marvin@gmail.com>"| "https://github.com/Kimundi/rustc-version-rs"| "Apache-2.0 OR MIT"| "A library for querying the version of a installed rustc compiler"|
| "rustversion"| "1.0.16"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/rustversion"| "Apache-2.0 OR MIT"| "Conditional compilation according to rustc compiler version"|
| "ryu"| "1.0.17"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/ryu"| "Apache-2.0 OR BSL-1.0"| "Fast floating point to string conversion"|
| "scoped-tls"| "1.0.1"| "Alex Crichton <alex@alexcrichton.com>"| "https://github.com/alexcrichton/scoped-tls"| "Apache-2.0 OR MIT"| "Library implementation of the standard library's old `scoped_thread_local!` macro for providing scoped access to thread local storage (TLS) so any type can be stored into TLS."|
| "semver"| "1.0.22"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/semver"| "Apache-2.0 OR MIT"| "Parser and evaluator for Cargo's flavor of Semantic Versioning"|
| "serde"| "1.0.201"| "Erick Tryzelaar <erick.tryzelaar@gmail.com>|David Tolnay <dtolnay@gmail.com>"| "https://github.com/serde-rs/serde"| "Apache-2.0 OR MIT"| "A generic serialization/deserialization framework"|
| "serde_bytes"| "0.11.14"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/serde-rs/bytes"| "Apache-2.0 OR MIT"| "Optimized handling of `&[u8]` and `Vec<u8>` for Serde"|
| "serde_derive"| "1.0.201"| "Erick Tryzelaar <erick.tryzelaar@gmail.com>|David Tolnay <dtolnay@gmail.com>"| "https://github.com/serde-rs/serde"| "Apache-2.0 OR MIT"| "Macros 1.1 implementation of #[derive(Serialize|  Deserialize)]"|
| "serde_json"| "1.0.116"| "Erick Tryzelaar <erick.tryzelaar@gmail.com>|David Tolnay <dtolnay@gmail.com>"| "https://github.com/serde-rs/json"| "Apache-2.0 OR MIT"| "A JSON serialization file format"|
| "serde_path_to_error"| "0.1.16"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/path-to-error"| "Apache-2.0 OR MIT"| "Path to the element that failed to deserialize"|
| "serde_urlencoded"| "0.7.1"| "Anthony Ramine <n.oxyde@gmail.com>"| "https://github.com/nox/serde_urlencoded"| "Apache-2.0 OR MIT"| "`x-www-form-urlencoded` meets Serde"|
| "sha2"| "0.10.8"| "RustCrypto Developers"| "https://github.com/RustCrypto/hashes"| "Apache-2.0 OR MIT"| "Pure Rust implementation of the SHA-2 hash function family including SHA-224|  SHA-256|  SHA-384|  and SHA-512."|
| "sha3"| "0.10.8"| "RustCrypto Developers"| "https://github.com/RustCrypto/hashes"| "Apache-2.0 OR MIT"| "Pure Rust implementation of SHA-3|  a family of Keccak-based hash functions including the SHAKE family of eXtendable-Output Functions (XOFs)|  as well as the accelerated variant TurboSHAKE"|
| "sharded-slab"| "0.1.7"| "Eliza Weisman <eliza@buoyant.io>"| "https://github.com/hawkw/sharded-slab"| "MIT"| "A lock-free concurrent slab."|
| "signature"| "2.2.0"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits/tree/master/signature"| "Apache-2.0 OR MIT"| "Traits for cryptographic signature algorithms (e.g. ECDSA|  Ed25519)"|
| "smallvec"| "1.13.2"| "The Servo Project Developers"| "https://github.com/servo/rust-smallvec"| "Apache-2.0 OR MIT"| "'Small vector' optimization: store up to a small number of items on the stack"|
| "socket2"| "0.5.7"| "Alex Crichton <alex@alexcrichton.com>|Thomas de Zeeuw <thomasdezeeuw@gmail.com>"| "https://github.com/rust-lang/socket2"| "Apache-2.0 OR MIT"| "Utilities for handling networking sockets with a maximal amount of configuration possible intended."|
| "spki"| "0.7.3"| "RustCrypto Developers"| "https://github.com/RustCrypto/formats/tree/master/spki"| "Apache-2.0 OR MIT"| "X.509 Subject Public Key Info (RFC5280) describing public keys as well as their associated AlgorithmIdentifiers (i.e. OIDs)"|
| "static_assertions"| "1.1.0"| "Nikolai Vazquez"| "https://github.com/nvzqz/static-assertions-rs"| "Apache-2.0 OR MIT"| "Compile-time assertions to ensure that invariants are met."|
| "subtle"| "2.5.0"| "Isis Lovecruft <isis@patternsinthevoid.net>|Henry de Valence <hdevalence@hdevalence.ca>"| "https://github.com/dalek-cryptography/subtle"| "BSD-3-Clause"| "Pure-Rust traits and utilities for constant-time cryptographic implementations."|
| "syn"| "2.0.58"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/syn"| "Apache-2.0 OR MIT"| "Parser for Rust source code"|
| "sync_wrapper"| "0.1.2"| "Actyx AG <developer@actyx.io>"| "https://github.com/Actyx/sync_wrapper"| "Apache-2.0"| "A tool for enlisting the compiler’s help in proving the absence of concurrency"|
| "sync_wrapper"| "1.0.1"| "Actyx AG <developer@actyx.io>"| "https://github.com/Actyx/sync_wrapper"| "Apache-2.0"| "A tool for enlisting the compiler's help in proving the absence of concurrency"|
| "tap"| "1.0.1"| "Elliott Linder <elliott.darfink@gmail.com>|myrrlyn <self@myrrlyn.dev>"| "https://github.com/myrrlyn/tap"| "MIT"| "Generic extensions for tapping values in Rust"|
| "thread_local"| "1.1.8"| "Amanieu d'Antras <amanieu@gmail.com>"| "https://github.com/Amanieu/thread_local-rs"| "Apache-2.0 OR MIT"| "Per-object thread-local storage"|
| "time"| "0.3.36"| "Jacob Pratt <open-source@jhpratt.dev>|Time contributors"| "https://github.com/time-rs/time"| "Apache-2.0 OR MIT"| "Date and time library. Fully interoperable with the standard library. Mostly compatible with #![no_std]."|
| "time-core"| "0.1.2"| "Jacob Pratt <open-source@jhpratt.dev>|Time contributors"| "https://github.com/time-rs/time"| "Apache-2.0 OR MIT"| "This crate is an implementation detail and should not be relied upon directly."|
| "time-macros"| "0.2.18"| "Jacob Pratt <open-source@jhpratt.dev>|Time contributors"| "https://github.com/time-rs/time"| "Apache-2.0 OR MIT"| "Procedural macros for the time crate.     This crate is an implementation detail and should not be relied upon directly."|
| "tokio"| "1.37.0"| "Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/tokio"| "MIT"| "An event-driven|  non-blocking I/O platform for writing asynchronous I/O backed applications."|
| "tokio-macros"| "2.2.0"| "Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/tokio"| "MIT"| "Tokio's proc macros."|
| "tower"| "0.4.13"| "Tower Maintainers <team@tower-rs.com>"| "https://github.com/tower-rs/tower"| "MIT"| "Tower is a library of modular and reusable components for building robust clients and servers."|
| "tower-layer"| "0.3.2"| "Tower Maintainers <team@tower-rs.com>"| "https://github.com/tower-rs/tower"| "MIT"| "Decorates a `Service` to allow easy composition between `Service`s."|
| "tower-service"| "0.3.2"| "Tower Maintainers <team@tower-rs.com>"| "https://github.com/tower-rs/tower"| "MIT"| "Trait representing an asynchronous|  request / response based|  client or server."|
| "tracing"| "0.1.40"| "Eliza Weisman <eliza@buoyant.io>|Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/tracing"| "MIT"| "Application-level tracing for Rust."|
| "tracing-core"| "0.1.32"| "Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/tracing"| "MIT"| "Core primitives for application-level tracing."|
| "tracing-log"| "0.2.0"| "Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/tracing"| "MIT"| "Provides compatibility between `tracing` and the `log` crate."|
| "tracing-subscriber"| "0.3.18"| "Eliza Weisman <eliza@buoyant.io>|David Barsky <me@davidbarsky.com>|Tokio Contributors <team@tokio.rs>"| "https://github.com/tokio-rs/tracing"| "MIT"| "Utilities for implementing and composing `tracing` subscribers."|
| "twox-hash"| "1.6.3"| "Jake Goulding <jake.goulding@gmail.com>"| "https://github.com/shepmaster/twox-hash"| "MIT"| "A Rust implementation of the XXHash and XXH3 algorithms"|
| "typenum"| "1.17.0"| "Paho Lurie-Gregg <paho@paholg.com>|Andre Bogus <bogusandre@gmail.com>"| "https://github.com/paholg/typenum"| "Apache-2.0 OR MIT"| "Typenum is a Rust library for type-level numbers evaluated at     compile time. It currently supports bits|  unsigned integers|  and signed     integers. It also provides a type-level array of type-level numbers|  but its     implementation is incomplete."|
| "unicode-ident"| "1.0.12"| "David Tolnay <dtolnay@gmail.com>"| "https://github.com/dtolnay/unicode-ident"| "(MIT OR Apache-2.0) AND Unicode-DFS-2016"| "Determine whether characters have the XID_Start or XID_Continue properties according to Unicode Standard Annex #31"|
| "universal-hash"| "0.5.1"| "RustCrypto Developers"| "https://github.com/RustCrypto/traits"| "Apache-2.0 OR MIT"| "Traits which describe the functionality of universal hash functions (UHFs)"|
| "uuid"| "1.8.0"| "Ashley Mannix<ashleymannix@live.com.au>|Christopher Armstrong|Dylan DPC<dylan.dpc@gmail.com>|Hunar Roop Kahlon<hunar.roop@gmail.com>"| "https://github.com/uuid-rs/uuid"| "Apache-2.0 OR MIT"| "A library to generate and parse UUIDs."|
| "valuable"| "0.1.0"| | "https://github.com/tokio-rs/valuable"| "MIT"| "Object-safe value inspection|  used to pass un-typed structured data across trait-object boundaries."|
| "version_check"| "0.9.4"| "Sergio Benitez <sb@sergio.bz>"| "https://github.com/SergioBenitez/version_check"| "Apache-2.0 OR MIT"| "Tiny crate to check the version of the installed/running rustc."|
| "wasi"| "0.11.0+wasi-snapshot-preview1"| "The Cranelift Project Developers"| "https://github.com/bytecodealliance/wasi"| "Apache-2.0 OR Apache-2.0 WITH LLVM-exception OR MIT"| "Experimental WASI API bindings for Rust"|
| "wasm"| "0.1.0"| | "https://github.com/Scarjit/CosmicCipher"| "Apache-2.0 OR MIT"| |
| "wasm-bindgen"| "0.2.92"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen"| "Apache-2.0 OR MIT"| "Easy support for interacting between JS and Rust."|
| "wasm-bindgen-backend"| "0.2.92"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/backend"| "Apache-2.0 OR MIT"| "Backend code generation of the wasm-bindgen tool"|
| "wasm-bindgen-futures"| "0.4.42"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/futures"| "Apache-2.0 OR MIT"| "Bridging the gap between Rust Futures and JavaScript Promises"|
| "wasm-bindgen-macro"| "0.2.92"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/macro"| "Apache-2.0 OR MIT"| "Definition of the `#[wasm_bindgen]` attribute|  an internal dependency"|
| "wasm-bindgen-macro-support"| "0.2.92"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/macro-support"| "Apache-2.0 OR MIT"| "The part of the implementation of the `#[wasm_bindgen]` attribute that is not in the shared backend crate"|
| "wasm-bindgen-shared"| "0.2.92"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/shared"| "Apache-2.0 OR MIT"| "Shared support between wasm-bindgen and wasm-bindgen cli|  an internal dependency."|
| "wasm-bindgen-test"| "0.3.42"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen"| "Apache-2.0 OR MIT"| "Internal testing crate for wasm-bindgen"|
| "wasm-bindgen-test-macro"| "0.3.42"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen"| "Apache-2.0 OR MIT"| "Internal testing macro for wasm-bindgen"|
| "web-sys"| "0.3.69"| "The wasm-bindgen Developers"| "https://github.com/rustwasm/wasm-bindgen/tree/master/crates/web-sys"| "Apache-2.0 OR MIT"| "Bindings for all Web APIs|  a procedurally generated crate from WebIDL"|
| "winapi"| "0.3.9"| "Peter Atashian <retep998@gmail.com>"| "https://github.com/retep998/winapi-rs"| "Apache-2.0 OR MIT"| "Raw FFI bindings for all of Windows API."|
| "winapi-i686-pc-windows-gnu"| "0.4.0"| "Peter Atashian <retep998@gmail.com>"| "https://github.com/retep998/winapi-rs"| "Apache-2.0 OR MIT"| "Import libraries for the i686-pc-windows-gnu target. Please don't use this crate directly|  depend on winapi instead."|
| "winapi-x86_64-pc-windows-gnu"| "0.4.0"| "Peter Atashian <retep998@gmail.com>"| "https://github.com/retep998/winapi-rs"| "Apache-2.0 OR MIT"| "Import libraries for the x86_64-pc-windows-gnu target. Please don't use this crate directly|  depend on winapi instead."|
| "windows-sys"| "0.48.0"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Rust for Windows"|
| "windows-sys"| "0.52.0"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Rust for Windows"|
| "windows-targets"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import libs for Windows"|
| "windows-targets"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import libs for Windows"|
| "windows_aarch64_gnullvm"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_aarch64_gnullvm"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_aarch64_msvc"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_aarch64_msvc"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_i686_gnu"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_i686_gnu"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_i686_gnullvm"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_i686_msvc"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_i686_msvc"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_x86_64_gnu"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_x86_64_gnu"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_x86_64_gnullvm"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_x86_64_gnullvm"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_x86_64_msvc"| "0.48.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "windows_x86_64_msvc"| "0.52.5"| "Microsoft"| "https://github.com/microsoft/windows-rs"| "Apache-2.0 OR MIT"| "Import lib for Windows"|
| "wyz"| "0.5.1"| "myrrlyn <self@myrrlyn.dev>"| "https://github.com/myrrlyn/wyz"| "MIT"| "myrrlyn’s utility collection"|
| "x25519-dalek"| "2.0.1"| "Isis Lovecruft <isis@patternsinthevoid.net>|DebugSteven <debugsteven@gmail.com>|Henry de Valence <hdevalence@hdevalence.ca>"| "https://github.com/dalek-cryptography/curve25519-dalek/tree/main/x25519-dalek"| "BSD-3-Clause"| "X25519 elliptic curve Diffie-Hellman key exchange in pure-Rust|  using curve25519-dalek."|
| "zerocopy"| "0.7.34"| "Joshua Liebow-Feeser <joshlf@google.com>"| "https://github.com/google/zerocopy"| "Apache-2.0 OR BSD-2-Clause OR MIT"| "Utilities for zero-copy parsing and serialization"|
| "zerocopy-derive"| "0.7.34"| "Joshua Liebow-Feeser <joshlf@google.com>"| "https://github.com/google/zerocopy"| "Apache-2.0 OR BSD-2-Clause OR MIT"| "Custom derive for traits from the zerocopy crate"|
| "zeroize"| "1.7.0"| "The RustCrypto Project Developers"| "https://github.com/RustCrypto/utils/tree/master/zeroize"| "Apache-2.0 OR MIT"| "Securely clear secrets from memory with a simple trait built on stable Rust primitives which guarantee memory is zeroed using an operation will not be 'optimized away' by the compiler. Uses a portable pure Rust implementation that works everywhere|  even WASM!"|
| "zeroize_derive"| "1.4.2"| "The RustCrypto Project Developers"| "https://github.com/RustCrypto/utils/tree/master/zeroize/derive"| "Apache-2.0 OR MIT"| "Custom derive support for zeroize"|
