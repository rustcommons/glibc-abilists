#![no_main]
// Compile the actual library implementation; do not copy or expose a new API.
#[path = "../../src/sha256.rs"]
mod sha256;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Sanitizers/debug assertions exercise arbitrary bytes and block/padding sizes.
    // Known-vector correctness is checked by the library's existing unit tests.
    let digest = sha256::sha256(data);
    let encoded = sha256::hex(&digest);
    assert_eq!(encoded.len(), 64);
    for (pair, byte) in encoded.as_bytes().chunks_exact(2).zip(digest) {
        let hex = std::str::from_utf8(pair).unwrap();
        assert_eq!(u8::from_str_radix(hex, 16).unwrap(), byte);
    }
});
