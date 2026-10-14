//! The GNU C Library's own ABI list files for x86_64 Linux, glibc 2.17
//! through 2.44, collected from glibc's signed release tags.
//!
//! Each [`AbiList`] is one unmodified `*.abilist` file from glibc's source
//! tree: the symbols, symbol versions, kinds, and object sizes a glibc
//! release exports from one library. Each [`Release`] records where its files
//! came from: the tag object, commit, signature, and license evidence.
//!
//! The data is glibc's and is licensed under the GNU Lesser General Public
//! License, version 2.1 or later (glibc's `COPYING.LIB`); so is this crate.
//! See `NOTICE.md`.
//!
//! The text is glibc's own format, which changed in 2.23: earlier files list
//! a version on its own line followed by indented symbols, later files put
//! the version on every line. This crate does not parse either.
//!
//! ```
//! let libc = glibc_abilists::abilist("2.44", "libc").unwrap();
//! assert!(libc.text.lines().any(|l| l == "GLIBC_2.2.5 memcpy F"));
//! assert!(libc.verify());
//! assert_eq!(glibc_abilists::release("2.44").unwrap().tag, "glibc-2.44");
//! ```
#![warn(missing_docs)]

// Written by the collector, which `--check`s it byte for byte.
#[rustfmt::skip]
mod generated;
mod sha256;

// The README's example runs as a doc test too.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// The repository the data was collected from.
pub const UPSTREAM: &str = "https://sourceware.org/git/glibc.git";
/// The glibc target the data describes (the x86_64 64-bit ABI).
pub const TARGET: &str = "x86_64-linux-gnu";
/// SPDX license expression of the data and of this crate.
pub const LICENSE: &str = "LGPL-2.1-or-later";
/// The license text (the GNU LGPL version 2.1, as distributed with glibc).
/// Include it whenever the ABI lists are redistributed, for example when
/// writing them to disk.
pub const LICENSE_TEXT: &str = include_str!("../LICENSES/LGPL-2.1-or-later.txt");

/// One glibc release tag and the evidence for its files.
#[derive(Debug)]
pub struct Release {
    /// `2.17`, `2.18`, ...
    pub version: &'static str,
    /// `glibc-2.17`, ...
    pub tag: &'static str,
    /// The annotated tag's object id.
    pub tag_object: &'static str,
    /// The tagged commit.
    pub commit: &'static str,
    /// The commit's committer date (ISO 8601).
    pub commit_date: &'static str,
    /// The tagger, `Name <email>`.
    pub tagger: &'static str,
    /// The tagger date (ISO 8601).
    pub tag_date: &'static str,
    /// The result of checking the tag's OpenPGP signature.
    pub signature: Signature,
    /// The license file at this tag, after following links (glibc's README
    /// names `COPYING.LIB`).
    pub license_path: &'static str,
    /// The license file's git blob id at the tag.
    pub license_blob: &'static str,
    /// SHA-256 of the license file's contents.
    pub license_sha256: &'static str,
}

/// The result of checking a release tag's OpenPGP signature.
#[derive(Debug, PartialEq, Eq)]
pub enum Signature {
    /// Verified with a key from `keys/` in the source repository.
    Verified {
        /// Fingerprint of the key, or subkey, that made the signature.
        signing_key: &'static str,
        /// Fingerprint of that key's primary key.
        primary_key: &'static str,
        /// Where the public key was obtained.
        key_source: &'static str,
        /// The key has expired since; the signature was made before that.
        key_expired_later: bool,
    },
    /// Signed, but the key is not available; recorded with the reason.
    Exception {
        /// Fingerprint of the key that made the signature, as the signature
        /// names it.
        signing_key: &'static str,
        /// Why the key is not available and what is checked instead, from
        /// `keys/exceptions.tsv` in the source repository.
        reason: &'static str,
    },
}

/// One unmodified `*.abilist` file.
#[derive(Debug)]
pub struct AbiList {
    /// The release version, e.g. `2.17`.
    pub release: &'static str,
    /// The file name without `.abilist`, e.g. `libc`, `ld`, `libm`.
    pub library: &'static str,
    /// The file's path in glibc's tree at the release tag.
    pub path: &'static str,
    /// The file's git blob id at the tag.
    pub blob: &'static str,
    /// SHA-256 of the contents.
    pub sha256: &'static str,
    /// The contents, exactly as in glibc.
    pub text: &'static str,
}

impl AbiList {
    /// Recomputes the SHA-256 of [`AbiList::text`] and compares it with the
    /// recorded value.
    pub fn verify(&self) -> bool {
        sha256::hex(&sha256::sha256(self.text.as_bytes())) == self.sha256
    }
}

/// Every release, oldest first.
pub fn releases() -> &'static [Release] {
    generated::RELEASES
}

/// The release for `version` (`2.17`). Returns `None` for a version outside
/// 2.17 through 2.44, or one not written as `2.N`.
pub fn release(version: &str) -> Option<&'static Release> {
    generated::RELEASES.iter().find(|r| r.version == version)
}

/// Every ABI list, ordered by release and library.
pub fn abilists() -> &'static [AbiList] {
    generated::ABILISTS
}

/// The ABI list of `library` (`libc`, `ld`, `libm`, ...) in `version`.
/// Returns `None` when the release is not collected, or when glibc has no ABI
/// list for that library in that release (for example `libcrypt` after 2.38).
pub fn abilist(version: &str, library: &str) -> Option<&'static AbiList> {
    generated::ABILISTS
        .iter()
        .find(|a| a.release == version && a.library == library)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_embedded_file_matches_its_recorded_hash() {
        assert!(!abilists().is_empty());
        for a in abilists() {
            assert!(a.verify(), "{} {} ({})", a.release, a.library, a.path);
        }
    }

    #[test]
    fn covers_every_release_from_2_17_to_2_44() {
        let versions: Vec<_> = releases().iter().map(|r| r.version).collect();
        let want: Vec<String> = (17..=44).map(|m| format!("2.{m}")).collect();
        assert_eq!(versions, want);
        for r in releases() {
            assert_eq!(r.tag, format!("glibc-{}", r.version));
            for lib in ["libc", "ld", "libm", "libpthread", "libdl", "librt"] {
                assert!(abilist(r.version, lib).is_some(), "{} {lib}", r.version);
            }
        }
    }

    #[test]
    fn lookups_return_none_outside_the_data() {
        assert!(release("2.16").is_none() && release("2.45").is_none());
        assert!(release("glibc-2.17").is_none());
        assert!(abilist("2.38", "libcrypt").is_some());
        assert!(abilist("2.39", "libcrypt").is_none());
        assert!(abilist("2.17", "libnone").is_none());
    }

    #[test]
    fn license_text_is_the_lgpl_2_1() {
        assert!(LICENSE_TEXT.contains("GNU LESSER GENERAL PUBLIC LICENSE"));
        assert!(LICENSE_TEXT.contains("Version 2.1, February 1999"));
    }

    #[test]
    fn provenance_fields_are_complete() {
        let hex = |s: &str, n: usize| s.len() == n && s.bytes().all(|b| b.is_ascii_hexdigit());
        for r in releases() {
            assert!(hex(r.tag_object, 40) && hex(r.commit, 40), "{}", r.tag);
            assert!(
                hex(r.license_blob, 40) && hex(r.license_sha256, 64),
                "{}",
                r.tag
            );
            match r.signature {
                Signature::Verified { primary_key, .. } => assert!(hex(primary_key, 40)),
                Signature::Exception { reason, .. } => assert!(!reason.is_empty()),
            }
        }
        for a in abilists() {
            assert!(hex(a.blob, 40) && hex(a.sha256, 64), "{}", a.path);
            assert!(a.path.ends_with(&format!("/{}.abilist", a.library)));
        }
    }
}
