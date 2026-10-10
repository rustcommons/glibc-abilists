//! Collects glibc's x86_64 ABI lists from its signed release tags and writes
//! the dataset with its provenance.
//!
//! glibc-abilists-collect --glibc REPO [--root DIR] [--check] [--offline]
//!
//! REPO is a clone of glibc (a bare, blobless clone is enough; git fetches the
//! few blobs it needs). DIR is this repository (default: the current
//! directory). The collector:
//!
//! 1. requires every tag in `upstream.toml`'s range in REPO, and the same tag
//!    object at the upstream repository and at each mirror that has the tag
//!    (skipped with --offline);
//! 2. verifies each tag's signature with the keys in `keys/`, unless the tag
//!    is listed in `keys/exceptions.tsv`; a bad signature always fails;
//! 3. requires glibc's README to name the license file and that file to be
//!    the GNU LGPL version 2.1, and records the file's identity;
//! 4. copies the selected ABI lists unmodified and records each file's path,
//!    git blob id, SHA-256, and size.
//!
//! It writes `data/<version>/<path>`, `data/releases.tsv`, `data/files.tsv`,
//! and `src/generated.rs`. With --check it writes nothing and fails if any of
//! them differ from what it would write.

mod sha256;

use sha256::{hex, sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};
use std::{env, fs};

fn main() -> ExitCode {
    match run(&env::args().skip(1).collect::<Vec<_>>()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("glibc-abilists-collect: {e}");
            ExitCode::FAILURE
        }
    }
}

const USAGE: &str = "usage: glibc-abilists-collect --glibc REPO [--root DIR] [--check] [--offline]";

struct Options {
    glibc: PathBuf,
    root: PathBuf,
    check: bool,
    offline: bool,
}

fn parse_options(args: &[String]) -> Result<Options, String> {
    let mut glibc = None;
    let mut root = PathBuf::from(".");
    let (mut check, mut offline) = (false, false);
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--glibc" => glibc = it.next().map(PathBuf::from),
            "--root" => root = it.next().map(PathBuf::from).ok_or(USAGE)?,
            "--check" => check = true,
            "--offline" => offline = true,
            "-h" | "--help" => return Err(USAGE.to_string()),
            other => return Err(format!("unknown argument {other:?}\n{USAGE}")),
        }
    }
    Ok(Options {
        glibc: glibc.ok_or(USAGE)?,
        root,
        check,
        offline,
    })
}

fn run(args: &[String]) -> Result<(), String> {
    let opts = parse_options(args)?;
    let config = Config::parse(&read_text(&opts.root.join("upstream.toml"))?)?;
    if config.target != "x86_64-linux-gnu" {
        return Err(format!(
            "target {} is not supported yet; the first release is x86_64-linux-gnu only",
            config.target
        ));
    }
    let git = Git(opts.glibc.clone());
    let keys = Keys::load(&opts.root.join("keys"))?;
    let tags: Vec<(String, String)> = config
        .versions()?
        .into_iter()
        .map(|v| (format!("glibc-{v}"), v))
        .collect();

    let local = git.tag_objects()?;
    for (tag, _) in &tags {
        if !local.contains_key(tag) {
            return Err(format!("{tag} is missing from {}", opts.glibc.display()));
        }
    }
    if !opts.offline {
        check_remote(&config.repository, &tags, &local, true)?;
        for m in &config.mirrors {
            check_remote(m, &tags, &local, false)?;
        }
    }

    let gnupg = keys.keyring()?;
    let mut releases = Vec::new();
    let mut files = Vec::new();
    for (tag, version) in &tags {
        let release = collect_release(&git, &config, &keys, &gnupg, tag, version)?;
        eprintln!(
            "{tag}: {} lists, signature {}",
            release.files.len(),
            release.signature_status
        );
        files.extend(release.files.iter().cloned());
        releases.push(release);
    }

    let outputs = render(&releases, &files);
    if opts.check {
        compare(&opts.root, &outputs, &tags)
    } else {
        write(&opts.root, &outputs, &tags)
    }
}

/// The subset of TOML used by `upstream.toml`: `key = "string"` and
/// `key = ["string", ...]` (arrays may span lines), with `#` comments.
struct Config {
    repository: String,
    mirrors: Vec<String>,
    first: String,
    last: String,
    target: String,
    directories: Vec<String>,
    license_file: String,
}

impl Config {
    fn parse(text: &str) -> Result<Self, String> {
        let mut values: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut pending: Option<(String, String)> = None;
        for raw in text.lines() {
            let line = raw.split('#').next().unwrap_or("").trim();
            if let Some((key, mut acc)) = pending.take() {
                acc.push_str(line);
                if acc.contains(']') {
                    values.insert(key, strings(&acc)?);
                } else {
                    pending = Some((key, acc));
                }
                continue;
            }
            if line.is_empty() {
                continue;
            }
            let (key, value) = line
                .split_once('=')
                .ok_or_else(|| format!("upstream.toml: cannot parse {raw:?}"))?;
            let (key, value) = (key.trim().to_string(), value.trim().to_string());
            if value.starts_with('[') && !value.contains(']') {
                pending = Some((key, value));
            } else {
                values.insert(key, strings(&value)?);
            }
        }
        if pending.is_some() {
            return Err("upstream.toml: unterminated array".to_string());
        }
        let mut one = |k: &str| -> Result<String, String> {
            match values.remove(k).as_deref() {
                Some([v]) => Ok(v.clone()),
                _ => Err(format!("upstream.toml: {k} must be one string")),
            }
        };
        let config = Config {
            repository: one("repository")?,
            first: one("first")?,
            last: one("last")?,
            target: one("target")?,
            license_file: one("license_file")?,
            mirrors: values.remove("mirrors").unwrap_or_default(),
            directories: values.remove("directories").unwrap_or_default(),
        };
        if let Some(k) = values.keys().next() {
            return Err(format!("upstream.toml: unknown key {k}"));
        }
        if config.directories.is_empty() {
            return Err("upstream.toml: directories is empty".to_string());
        }
        Ok(config)
    }

    /// `2.17`, `2.18`, ... `last`.
    fn versions(&self) -> Result<Vec<String>, String> {
        let parse = |s: &str| -> Result<(u32, u32), String> {
            let (a, b) = s.split_once('.').ok_or(format!("bad version {s:?}"))?;
            Ok((
                a.parse().map_err(|_| format!("bad version {s:?}"))?,
                b.parse().map_err(|_| format!("bad version {s:?}"))?,
            ))
        };
        let (fa, fb) = parse(&self.first)?;
        let (la, lb) = parse(&self.last)?;
        if fa != la || fb > lb {
            return Err("first and last must share a major version, first <= last".to_string());
        }
        Ok((fb..=lb).map(|m| format!("{fa}.{m}")).collect())
    }
}

/// Extracts the double-quoted strings of a value.
fn strings(value: &str) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    let mut rest = value;
    while let Some(start) = rest.find('"') {
        let tail = &rest[start + 1..];
        let end = tail
            .find('"')
            .ok_or_else(|| format!("unterminated string in {value:?}"))?;
        out.push(tail[..end].to_string());
        rest = &tail[end + 1..];
    }
    if out.is_empty() {
        return Err(format!("expected a string in {value:?}"));
    }
    Ok(out)
}

struct Git(PathBuf);

impl Git {
    fn output(&self, args: &[&str]) -> Result<Vec<u8>, String> {
        git_output(Some(&self.0), args, &[])
    }

    fn text(&self, args: &[&str]) -> Result<String, String> {
        String::from_utf8(self.output(args)?).map_err(|e| format!("git {args:?}: {e}"))
    }

    /// Tag name -> tag object id, for annotated `glibc-*` tags.
    fn tag_objects(&self) -> Result<BTreeMap<String, String>, String> {
        Ok(self
            .text(&[
                "for-each-ref",
                "--format=%(refname:strip=2) %(objectname)",
                "refs/tags/glibc-*",
            ])?
            .lines()
            .filter_map(|l| l.split_once(' '))
            .map(|(t, o)| (t.to_string(), o.to_string()))
            .collect())
    }
}

fn git_output(
    dir: Option<&Path>,
    args: &[&str],
    envs: &[(&str, &Path)],
) -> Result<Vec<u8>, String> {
    let mut cmd = Command::new("git");
    if let Some(d) = dir {
        cmd.arg("-C").arg(d);
    }
    cmd.args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let out = cmd.output().map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "git {}: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    Ok(out.stdout)
}

/// Every tag must match the local tag object at `url`; a mirror may lack a
/// tag (mirrors lag), but must not disagree.
fn check_remote(
    url: &str,
    tags: &[(String, String)],
    local: &BTreeMap<String, String>,
    required: bool,
) -> Result<(), String> {
    let text = String::from_utf8(git_output(
        None,
        &["ls-remote", url, "refs/tags/glibc-*"],
        &[],
    )?)
    .map_err(|e| e.to_string())?;
    let remote: BTreeMap<&str, &str> = text
        .lines()
        .filter_map(|l| l.split_once('\t'))
        .filter(|(_, r)| !r.ends_with("^{}"))
        .map(|(o, r)| (r.trim_start_matches("refs/tags/"), o))
        .collect();
    let (mut agreed, mut absent) = (0, 0);
    for (tag, _) in tags {
        match remote.get(tag.as_str()) {
            Some(o) if *o == local[tag] => agreed += 1,
            Some(o) => {
                return Err(format!(
                    "{url}: {tag} is {o}, but the local clone has {}",
                    local[tag]
                ))
            }
            None if required => return Err(format!("{url}: {tag} is missing")),
            None => absent += 1,
        }
    }
    eprintln!("{url}: {agreed} tags agree, {absent} absent");
    Ok(())
}

struct Keys {
    /// Primary fingerprint -> where the key was obtained.
    sources: BTreeMap<String, String>,
    /// Tag -> reason its signature cannot be verified.
    exceptions: BTreeMap<String, String>,
    files: Vec<PathBuf>,
}

impl Keys {
    fn load(dir: &Path) -> Result<Self, String> {
        let table = |name: &str,
                     key_col: usize,
                     val_col: usize|
         -> Result<BTreeMap<String, String>, String> {
            Ok(read_text(&dir.join(name))?
                .lines()
                .skip(1)
                .filter(|l| !l.trim().is_empty())
                .map(|l| {
                    let c: Vec<&str> = l.split('\t').collect();
                    (
                        c[key_col].to_string(),
                        c.get(val_col).unwrap_or(&"").to_string(),
                    )
                })
                .collect())
        };
        let sources = table("keys.tsv", 0, 2)?;
        let exceptions = table("exceptions.tsv", 0, 2)?;
        let mut files = Vec::new();
        for fp in sources.keys() {
            let f = dir.join(format!("{fp}.asc"));
            if !f.is_file() {
                return Err(format!("{} is listed but missing", f.display()));
            }
            files.push(f);
        }
        Ok(Keys {
            sources,
            exceptions,
            files,
        })
    }

    /// A temporary GnuPG home holding only these keys.
    fn keyring(&self) -> Result<TempDir, String> {
        let home = TempDir::new("glibc-abilists-gnupg")?;
        for f in &self.files {
            let out = Command::new("gpg")
                .env("GNUPGHOME", &home.0)
                .args(["--batch", "--quiet", "--import"])
                .arg(f)
                .output()
                .map_err(|e| format!("cannot run gpg: {e}"))?;
            if !out.status.success() {
                return Err(format!(
                    "gpg --import {}: {}",
                    f.display(),
                    String::from_utf8_lossy(&out.stderr).trim()
                ));
            }
        }
        Ok(home)
    }
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(prefix: &str) -> Result<Self, String> {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let p = env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()));
        fs::create_dir(&p).map_err(|e| format!("{}: {e}", p.display()))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = fs::set_permissions(&p, fs::Permissions::from_mode(0o700));
        }
        Ok(TempDir(p))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone)]
struct FileRecord {
    version: String,
    library: String,
    path: String,
    blob: String,
    sha256: String,
    contents: Vec<u8>,
}

struct Release {
    version: String,
    tag: String,
    tag_object: String,
    commit: String,
    commit_date: String,
    tagger: String,
    tag_date: String,
    /// `verified`, `verified-key-expired-later`, or `exception`.
    signature_status: String,
    signing_key: String,
    primary_key: String,
    key_source: String,
    license_path: String,
    license_blob: String,
    license_sha256: String,
    files: Vec<FileRecord>,
}

fn collect_release(
    git: &Git,
    config: &Config,
    keys: &Keys,
    gnupg: &TempDir,
    tag: &str,
    version: &str,
) -> Result<Release, String> {
    let meta = git.text(&[
        "for-each-ref",
        "--format=%(objecttype)\t%(objectname)\t%(*objectname)\t%(taggername) %(taggeremail)\t%(taggerdate:iso-strict)\t%(*committerdate:iso-strict)",
        &format!("refs/tags/{tag}"),
    ])?;
    let m: Vec<&str> = meta.trim_end().split('\t').collect();
    let [kind, tag_object, commit, tagger, tag_date, commit_date] = m[..] else {
        return Err(format!("{tag}: unexpected tag metadata {meta:?}"));
    };
    if kind != "tag" {
        return Err(format!("{tag} is not an annotated tag"));
    }

    // Signature.
    let raw = Command::new("git")
        .arg("-C")
        .arg(&git.0)
        .args(["verify-tag", "--raw", tag])
        .env("GNUPGHOME", &gnupg.0)
        .output()
        .map_err(|e| format!("cannot run git verify-tag: {e}"))?;
    let status = String::from_utf8_lossy(&raw.stderr).to_string();
    let field = |prefix: &str| -> Option<Vec<String>> {
        status
            .lines()
            .find_map(|l| l.strip_prefix(prefix))
            .map(|r| r.split_whitespace().map(str::to_string).collect())
    };
    if let Some(bad) = ["BADSIG", "REVKEYSIG", "EXPSIG"]
        .iter()
        .find(|s| status.contains(&format!("[GNUPG:] {s} ")))
    {
        return Err(format!("{tag}: signature check reports {bad}\n{status}"));
    }
    // A key that expired after it made the signature still proves who made
    // it (glibc 2.17/2.18 were signed in 2012-2013 with a key that expired in
    // 2016). A signature made after expiry is rejected.
    let validsig = field("[GNUPG:] VALIDSIG ");
    let expired_key = status.contains("[GNUPG:] EXPKEYSIG ");
    if expired_key {
        let signed: u64 = validsig
            .as_ref()
            .and_then(|v| v.get(2))
            .and_then(|t| t.parse().ok())
            .ok_or_else(|| format!("{tag}: expired key without a valid signature\n{status}"))?;
        // gpg prints KEYEXPIRED for every key it considers, so read the
        // signing key's and its primary key's own expiry instead.
        let v = validsig.as_ref().expect("checked above");
        let (signing, primary) = (&v[0], v.last().expect("VALIDSIG has fields"));
        let expiries = key_expiries(gnupg, primary)?;
        let expired = [signing, primary]
            .iter()
            .filter_map(|fp| expiries.get(*fp).copied().flatten())
            .min()
            .ok_or_else(|| {
                format!("{tag}: gpg reports an expired key but {signing} has no expiry")
            })?;
        if signed >= expired {
            return Err(format!(
                "{tag}: signed at {signed}, after key {signing} expired at {expired}"
            ));
        }
    }
    let (signature_status, signing_key, primary_key, key_source) = match validsig {
        Some(v) if raw.status.success() || expired_key => {
            let primary = v.last().cloned().unwrap_or_default();
            let source = keys
                .sources
                .get(&primary)
                .cloned()
                .ok_or_else(|| format!("{tag}: key {primary} is not in keys.tsv"))?;
            if keys.exceptions.contains_key(tag) {
                return Err(format!(
                    "{tag} now verifies; remove it from keys/exceptions.tsv"
                ));
            }
            let state = if expired_key {
                "verified-key-expired-later"
            } else {
                "verified"
            };
            (state.to_string(), v[0].clone(), primary, source)
        }
        _ => match keys.exceptions.get(tag) {
            Some(reason) => {
                let key = field("[GNUPG:] ERRSIG ")
                    .and_then(|v| v.last().cloned())
                    .unwrap_or_default();
                ("exception".to_string(), key, String::new(), reason.clone())
            }
            None => {
                return Err(format!(
                    "{tag}: signature not verified and not listed in keys/exceptions.tsv\n{status}"
                ))
            }
        },
    };

    // License evidence.
    let readme = git.text(&["cat-file", "-p", &format!("{tag}:README")])?;
    if !readme.contains(&format!("See the file {} for copying", config.license_file)) {
        return Err(format!(
            "{tag}: README no longer names {} as the copying conditions",
            config.license_file
        ));
    }
    let (license_path, license_blob, license) = resolve_file(git, tag, &config.license_file)?;
    let license_text = String::from_utf8_lossy(&license);
    if !(license_text.contains("GNU LESSER GENERAL PUBLIC LICENSE")
        && license_text.contains("Version 2.1, February 1999"))
    {
        return Err(format!(
            "{tag}: {license_path} is not the GNU LGPL version 2.1"
        ));
    }

    // ABI lists.
    let mut chosen: BTreeMap<String, (String, String)> = BTreeMap::new();
    for dir in &config.directories {
        let listing = git
            .text(&["ls-tree", &format!("{tag}:{dir}")])
            .unwrap_or_default();
        for line in listing.lines() {
            let Some((info, name)) = line.split_once('\t') else {
                continue;
            };
            let info: Vec<&str> = info.split_whitespace().collect();
            let (Some(stem), ["100644", "blob", blob]) = (name.strip_suffix(".abilist"), &info[..])
            else {
                continue;
            };
            chosen
                .entry(stem.to_string())
                .or_insert_with(|| (format!("{dir}/{name}"), blob.to_string()));
        }
    }
    if !chosen.contains_key("libc") {
        return Err(format!(
            "{tag}: no libc.abilist in the configured directories"
        ));
    }
    let mut files = Vec::new();
    for (library, (path, blob)) in chosen {
        let contents = git.output(&["cat-file", "blob", &blob])?;
        files.push(FileRecord {
            version: version.to_string(),
            library,
            path,
            sha256: hex(&sha256(&contents)),
            blob,
            contents,
        });
    }

    Ok(Release {
        version: version.to_string(),
        tag: tag.to_string(),
        tag_object: tag_object.to_string(),
        commit: commit.to_string(),
        commit_date: iso_date(commit_date),
        tagger: tagger.to_string(),
        tag_date: iso_date(tag_date),
        signature_status,
        signing_key,
        primary_key,
        key_source,
        license_path,
        license_blob,
        license_sha256: hex(&sha256(&license)),
        files,
    })
}

/// git's `iso-strict` dates, with UTC written as `+00:00`: newer git
/// versions print `Z`, older ones `+00:00`, and the output must not depend on
/// the git version.
fn iso_date(date: &str) -> String {
    match date.strip_suffix('Z') {
        Some(rest) => format!("{rest}+00:00"),
        None => date.to_string(),
    }
}

/// Fingerprint -> expiry time (seconds since the epoch) of a primary key and
/// its subkeys, from `gpg --with-colons`.
fn key_expiries(gnupg: &TempDir, primary: &str) -> Result<BTreeMap<String, Option<u64>>, String> {
    let out = Command::new("gpg")
        .env("GNUPGHOME", &gnupg.0)
        .args(["--batch", "--with-colons", "--list-keys", primary])
        .output()
        .map_err(|e| format!("cannot run gpg: {e}"))?;
    let text = String::from_utf8_lossy(&out.stdout);
    let mut map = BTreeMap::new();
    let mut expiry: Option<Option<u64>> = None;
    for line in text.lines() {
        let f: Vec<&str> = line.split(':').collect();
        match f.first() {
            Some(&"pub") | Some(&"sub") => {
                expiry = Some(f.get(6).and_then(|e| e.parse().ok()));
            }
            Some(&"fpr") => {
                if let (Some(e), Some(fp)) = (expiry.take(), f.get(9)) {
                    map.insert(fp.to_string(), e);
                }
            }
            _ => {}
        }
    }
    Ok(map)
}

/// Reads `path` at `tag`, following in-tree symbolic links (COPYING.LIB is a
/// link to COPYING.LESSERv2 in later releases). Returns the final path, its
/// blob id, and its contents.
fn resolve_file(git: &Git, tag: &str, path: &str) -> Result<(String, String, Vec<u8>), String> {
    let mut path = path.to_string();
    for _ in 0..8 {
        let entry = git.text(&["ls-tree", tag, "--", &path])?;
        let (info, _) = entry
            .trim_end()
            .split_once('\t')
            .ok_or_else(|| format!("{tag}: {path} is missing"))?;
        let info: Vec<&str> = info.split_whitespace().collect();
        let [mode, "blob", blob] = info[..] else {
            return Err(format!("{tag}: {path} is not a file"));
        };
        let contents = git.output(&["cat-file", "blob", blob])?;
        if mode != "120000" {
            return Ok((path, blob.to_string(), contents));
        }
        let target = String::from_utf8_lossy(&contents).trim().to_string();
        path = match Path::new(&path)
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
        {
            Some(p) => format!("{}/{target}", p.display()),
            None => target,
        };
    }
    Err(format!(
        "{tag}: too many symbolic links resolving the license file"
    ))
}

/// Every output file, keyed by its path relative to the root.
fn render(releases: &[Release], files: &[FileRecord]) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    let mut rel = String::from(
        "version\ttag\ttag_object\tcommit\tcommit_date\ttagger\ttag_date\tsignature\tsigning_key\tprimary_key\tkey_source_or_reason\tlicense_path\tlicense_blob\tlicense_sha256\tlicense_spdx\n",
    );
    for r in releases {
        rel.push_str(
            &[
                &r.version,
                &r.tag,
                &r.tag_object,
                &r.commit,
                &r.commit_date,
                &r.tagger,
                &r.tag_date,
                &r.signature_status,
                &r.signing_key,
                &r.primary_key,
                &r.key_source,
                &r.license_path,
                &r.license_blob,
                &r.license_sha256,
                "LGPL-2.1-or-later",
            ]
            .join("\t"),
        );
        rel.push('\n');
    }
    out.insert("data/releases.tsv".to_string(), rel.into_bytes());

    let mut tsv = String::from("version\tlibrary\tpath\tblob\tsha256\tbytes\tmodified\n");
    for f in files {
        tsv.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\tno\n",
            f.version,
            f.library,
            f.path,
            f.blob,
            f.sha256,
            f.contents.len()
        ));
        out.insert(format!("data/{}/{}", f.version, f.path), f.contents.clone());
    }
    out.insert("data/files.tsv".to_string(), tsv.into_bytes());
    out.insert(
        "src/generated.rs".to_string(),
        generated_rs(releases, files).into_bytes(),
    );
    out
}

fn generated_rs(releases: &[Release], files: &[FileRecord]) -> String {
    let q = |s: &str| format!("{s:?}");
    let mut s = String::from(
        "// @generated by glibc-abilists-collector from glibc's release tags.\n\
         // Do not edit; rerun the collector (see README.md).\n\n\
         use crate::{AbiList, Release, Signature};\n\n\
         pub(crate) static RELEASES: &[Release] = &[\n",
    );
    for r in releases {
        let signature = if r.signature_status.starts_with("verified") {
            format!(
                "Signature::Verified {{ signing_key: {}, primary_key: {}, key_source: {}, key_expired_later: {} }}",
                q(&r.signing_key),
                q(&r.primary_key),
                q(&r.key_source),
                r.signature_status == "verified-key-expired-later"
            )
        } else {
            format!(
                "Signature::Exception {{ signing_key: {}, reason: {} }}",
                q(&r.signing_key),
                q(&r.key_source)
            )
        };
        s.push_str(&format!(
            "    Release {{\n        version: {},\n        tag: {},\n        tag_object: {},\n        commit: {},\n        commit_date: {},\n        tagger: {},\n        tag_date: {},\n        signature: {signature},\n        license_path: {},\n        license_blob: {},\n        license_sha256: {},\n    }},\n",
            q(&r.version), q(&r.tag), q(&r.tag_object), q(&r.commit), q(&r.commit_date),
            q(&r.tagger), q(&r.tag_date), q(&r.license_path), q(&r.license_blob), q(&r.license_sha256),
        ));
    }
    s.push_str("];\n\npub(crate) static ABILISTS: &[AbiList] = &[\n");
    for f in files {
        s.push_str(&format!(
            "    AbiList {{\n        release: {},\n        library: {},\n        path: {},\n        blob: {},\n        sha256: {},\n        text: include_str!(\"../data/{}/{}\"),\n    }},\n",
            q(&f.version), q(&f.library), q(&f.path), q(&f.blob), q(&f.sha256), f.version, f.path,
        ));
    }
    s.push_str("];\n");
    s
}

/// Files under `data/<version>/` that the collector does not produce.
fn stray_files(
    root: &Path,
    outputs: &BTreeMap<String, Vec<u8>>,
    tags: &[(String, String)],
) -> Vec<String> {
    let mut found = BTreeSet::new();
    for (_, v) in tags {
        walk(root, &root.join("data").join(v), &mut found);
    }
    found
        .into_iter()
        .filter(|p| !outputs.contains_key(p))
        .collect()
}

fn walk(root: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(root, &p, out);
        } else if let Ok(rel) = p.strip_prefix(root) {
            out.insert(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn compare(
    root: &Path,
    outputs: &BTreeMap<String, Vec<u8>>,
    tags: &[(String, String)],
) -> Result<(), String> {
    let mut problems: Vec<String> = Vec::new();
    for (rel, want) in outputs {
        match fs::read(root.join(rel)) {
            Ok(have) if &have == want => {}
            Ok(have) => {
                let (have, want) = (
                    String::from_utf8_lossy(&have),
                    String::from_utf8_lossy(want),
                );
                let first = have
                    .lines()
                    .zip(want.lines())
                    .enumerate()
                    .find(|(_, (h, w))| h != w)
                    .map(|(i, (h, w))| {
                        format!("\n    line {}:\n      have: {h}\n      want: {w}", i + 1)
                    })
                    .unwrap_or_default();
                problems.push(format!("differs: {rel}{first}"));
            }
            Err(_) => problems.push(format!("missing: {rel}")),
        }
    }
    problems.extend(
        stray_files(root, outputs, tags)
            .into_iter()
            .map(|p| format!("not produced by the collector: {p}")),
    );
    if problems.is_empty() {
        eprintln!("check: {} files match glibc's tags", outputs.len());
        Ok(())
    } else {
        Err(format!("check failed:\n  {}", problems.join("\n  ")))
    }
}

fn write(
    root: &Path,
    outputs: &BTreeMap<String, Vec<u8>>,
    tags: &[(String, String)],
) -> Result<(), String> {
    for p in stray_files(root, outputs, tags) {
        fs::remove_file(root.join(&p)).map_err(|e| format!("{p}: {e}"))?;
    }
    for (rel, bytes) in outputs {
        let path = root.join(rel);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    eprintln!("wrote {} files", outputs.len());
    Ok(())
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_upstream_toml() {
        let c = Config::parse(include_str!("../../upstream.toml")).unwrap();
        assert_eq!(c.repository, "https://sourceware.org/git/glibc.git");
        assert_eq!(c.directories.len(), 3);
        let v = c.versions().unwrap();
        assert_eq!(v.first().map(String::as_str), Some("2.17"));
        assert_eq!(v.last().map(String::as_str), Some("2.44"));
        assert_eq!(v.len(), 28);
    }

    #[test]
    fn utc_dates_do_not_depend_on_the_git_version() {
        assert_eq!(
            iso_date("2018-02-01T16:17:18Z"),
            "2018-02-01T16:17:18+00:00"
        );
        assert_eq!(
            iso_date("2018-02-01T16:17:18+00:00"),
            "2018-02-01T16:17:18+00:00"
        );
        assert_eq!(
            iso_date("2019-08-01T00:33:39-04:00"),
            "2019-08-01T00:33:39-04:00"
        );
    }

    #[test]
    fn rejects_unknown_keys_and_bad_ranges() {
        let base = "repository = \"r\"\nfirst = \"2.17\"\nlast = \"2.18\"\ntarget = \"t\"\nlicense_file = \"L\"\ndirectories = [\"d\"]\n";
        assert!(Config::parse(base).is_ok());
        assert!(Config::parse(&format!("{base}extra = \"x\"\n")).is_err());
        let backwards = base.replace("2.18", "2.16");
        assert!(Config::parse(&backwards).unwrap().versions().is_err());
    }
}
