# Using the ABI lists

Start with the dependency and example in the [README](../README.md#getting-started).
The [0.1.2 API reference](https://docs.rs/glibc-abilists/0.1.2/glibc_abilists/)
documents every public item. The crate needs Rust 1.90 or later; its embedded
dataset describes x86_64's 64-bit Linux ABI regardless of the consuming host.
The collector's Linux/WSL and Git/GPG requirements do not apply to consumers.

## Find and enumerate data

| Task | API | Result |
| --- | --- | --- |
| Look up a release | `release("2.17")` | `Option<&Release>` with tag, commit, signature, and license evidence |
| Look up a library | `abilist("2.17", "libc")` | `Option<&AbiList>` with raw text and file provenance |
| Enumerate releases | `releases()` | Static slice, oldest first |
| Enumerate files | `abilists()` | Static slice, ordered by release and library; filter by `release` as needed |

Use exact release strings such as `2.17`, not `glibc-2.17`, and library names
without `.abilist`. Handle `None` for unknown releases or missing libraries;
for example, `libcrypt` exists in 2.38 but not 2.39. Do not assume every release
contains the same libraries. The current dataset spans 2.17–2.44 and has 13–15
libraries per release. Slices and text borrow static embedded data; lookup does
not fetch or install anything.

## Read raw text

`AbiList::text` contains the upstream file unchanged. Before glibc 2.23, files
place a symbol version on its own line followed by indented symbols. Starting
with 2.23, each line carries the version. This crate does not parse either
format, generate stubs, or select a compatible target for an application.
Use `path` for the upstream path, `blob` for its Git object id, and `sha256`
for the recorded content hash.

## Interpret verification

`AbiList::verify()` recomputes the text's SHA-256 and compares it to the
embedded record. It does not contact upstream or verify a tag signature at
runtime. `Release::signature` records the collector's result: either
`Signature::Verified` (including whether the key expired later), or
`Signature::Exception` with a reason. glibc 2.30 is the documented exception.
An exception does not establish signature presence or a match to the configured
key; its reported key can be empty and its reason is not a per-run attestation.
See [Provenance](PROVENANCE.md) before interpreting these records as evidence.

The lists are upstream declarations; they do not prove runtime behavior or
cover distribution patches. A successful hash check is not an application
compatibility verdict.

## Redistribute files

Preserve the unmodified ABI text and its provenance. Include `LICENSE_TEXT`
when writing or redistributing the ABI lists, as documented in the API, and
retain the notices in [NOTICE.md](../NOTICE.md). The published code and data
are LGPL-2.1-or-later; the collector's MIT license does not replace that license.
If deriving modified data, label it as modified and record the date. This crate
provides no file-export command: consumers decide where to write the text and
license. Constants `UPSTREAM`, `TARGET`, and `LICENSE` identify the dataset.
