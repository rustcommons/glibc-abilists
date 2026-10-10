# Notice

## The data: GNU C Library

Everything under `data/` except `releases.tsv` and `files.tsv` is copied,
unmodified, from the **GNU C Library (glibc)**:

- Upstream: https://sourceware.org/git/glibc.git, release tags `glibc-2.17`
  through `glibc-2.44`. `data/releases.tsv` gives each tag's object, commit,
  tagger, and signature result; `data/files.tsv` gives each file's path in
  glibc's tree, git blob id, SHA-256, and size.
- Copyright (C) Free Software Foundation, Inc. and the glibc contributors.
- License: the GNU Lesser General Public License, version 2.1 or later
  (`LGPL-2.1-or-later`). glibc's README names `COPYING.LIB` as the library's
  copying conditions; the collector checks that statement and that the file is
  the LGPL 2.1 at every tag, and records the file's identity. The license text
  is in [LICENSES/LGPL-2.1-or-later.txt](LICENSES/LGPL-2.1-or-later.txt), copied
  from glibc 2.44's `COPYING.LESSERv2` (its `COPYING.LIB` link target).
- glibc's `LICENSES` file lists notices for contributions by other copyright
  holders; none of them covers the ABI list files.

The provenance tables (`releases.tsv`, `files.tsv`), the crate's code in
`src/`, and `src/generated.rs` are distributed under the same license as the
data, LGPL-2.1-or-later.

## The collector

`collector/` is MIT licensed ([LICENSES/MIT.txt](LICENSES/MIT.txt)) and is not
part of the published crate. It contains no glibc code; it reads glibc's
repository with `git` and verifies signatures with `gpg`.

## Release-manager keys

`keys/` holds the public OpenPGP keys of glibc's release managers, exported
from the sources listed in `keys/keys.tsv`, to verify release tags. They are
not part of the published crate.

## Credit

The idea of keeping glibc's ABI lists as a reusable, versioned dataset comes
from the **Zig project's** [libc-abi-tools](https://codeberg.org/ziglang/libc-abi-tools),
as of commit `9217cf899bf12bbb24a8a3de4cc2de682de0379d` ("update to Zig
0.17.0", 2026-10-02; [browse](https://codeberg.org/ziglang/libc-abi-tools/src/commit/9217cf899bf12bbb24a8a3de4cc2de682de0379d)), published under the MIT License
(`LICENSE` git blob `9ce01373c89082fd81e8ea0793aab0978140526b`). This project
is an independent implementation: it uses no Zig code or files, and its data
comes directly from glibc. No endorsement by Zig or by the GNU project is
implied.
