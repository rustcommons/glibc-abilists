#![no_main]
use glibc_abilists::{abilist, abilists, release, releases, AbiList};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // Fuzz caller-supplied lookup strings; malformed UTF-8 cannot reach the &str API.
    if let Ok(text) = std::str::from_utf8(data) {
        let (version, library) = text.split_once('\0').unwrap_or((text, "libc"));
        let found = release(version);
        let expected = releases().iter().find(|entry| entry.version == version);
        assert_eq!(
            found.map(|entry| entry.commit),
            expected.map(|entry| entry.commit)
        );
        let found = abilist(version, library);
        let expected = abilists()
            .iter()
            .find(|entry| entry.release == version && entry.library == library);
        assert_eq!(
            found.map(|entry| entry.blob),
            expected.map(|entry| entry.blob)
        );
        if let Some(entry) = found {
            assert!(entry.verify());
        }
    }
    // Exercise embedded table selection and verifier rejection without leaking inputs
    // to satisfy the public type's &'static str fields.
    let index = data.iter().take(8).fold(0usize, |n, byte| {
        n.wrapping_mul(257).wrapping_add(*byte as usize)
    });
    let entry = &abilists()[index % abilists().len()];
    assert!(entry.verify());
    let wrong_hash = AbiList {
        release: entry.release,
        library: entry.library,
        path: entry.path,
        blob: entry.blob,
        sha256: "",
        text: entry.text,
    };
    assert!(!wrong_hash.verify());
});
