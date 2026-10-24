//! The handoff directory package on disk (DEL-15-01 Design
//! `canonical-model-export.md` §3, W-6b): its two members, its atomic
//! publication and its verifier.
//!
//! A package is a directory `<name>.swbhandoff/` holding exactly
//! - `model/model.json`: the W-6a canonical model document bytes;
//! - `handoff_package.json`: the `schemas/handoff_package.schema.json` instance
//!   as compact RFC 8785 (JCS) text.
//!
//! Finder's `.DS_Store` metadata files are the only other entries tolerated in
//! an existing package; they are not members.
//!
//! Publication ([`publish_package`]):
//! 1. both members are written into a staging directory
//!    `<name>.swbhandoff.staging-<random>` beside the target, each file and
//!    both directories fsynced, and the staging directory is verified
//!    ([`verify_handoff_package`]);
//! 2. one rename publishes it, refusing to replace anything (macOS
//!    `renamex_np(RENAME_EXCL)`), the single commit point;
//! 3. an existing package is replaced by `renamex_np(staging, target,
//!    RENAME_SWAP)`, which exchanges the two directories in one step; the
//!    swapped-out old package is then removed member by member (never a
//!    recursive delete). A volume that cannot swap refuses
//!    `HANDOFF-REPLACE-UNSUPPORTED`; an existing destination that is not a
//!    handoff package is never replaced;
//! 4. a leftover `<name>.swbhandoff.staging-*` from an earlier crash is
//!    ignored and reported for the user to delete.
//!
//! Every failure before the commit removes this run's staging directory and
//! leaves the previous package as it was.
//!
//! The verifier recomputes the SHA-256 of `model/model.json` and the export
//! hash (the JCS of the instance's `handoff_package_manifest` with its
//! `checksums` member removed), and requires the instance to be exactly the
//! contract envelope around that manifest with exactly the checksum inventory
//! those two hashes give, so a change to any byte of either member fails. Its
//! Python twin is `tests/verifiers/handoff_package.py`.

use crate::model_document_save::ModelDocumentSaveDiagnostic as Diagnostic;
use open_pipe_stress_operation_applier::{canonical_json, sha256_hex};
use serde::Serialize;
use serde_json::{json, Map, Value};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub const PACKAGE_EXTENSION: &str = "swbhandoff";
pub const MODEL_DIRECTORY: &str = "model";
pub const MODEL_FILE: &str = "model.json";
pub const MODEL_MEMBER: &str = "model/model.json";
pub const MANIFEST_MEMBER: &str = "handoff_package.json";
/// Finder metadata, tolerated in an existing package and removed with it.
const FINDER_METADATA: &str = ".DS_Store";
const STAGING_INFIX: &str = ".staging-";

/// The handoff package schema version this producer writes (`schema_version`
/// and `package_identity.package_schema_version`).
pub const HANDOFF_SCHEMA_VERSION: &str = "0.1.0";

pub const DESTINATION_INVALID: &str = "HANDOFF-DESTINATION-INVALID";
pub const DESTINATION_NOT_PACKAGE: &str = "HANDOFF-DESTINATION-NOT-PACKAGE";
pub const DESTINATION_APPEARED: &str = "HANDOFF-DESTINATION-APPEARED";
pub const REPLACE_UNSUPPORTED: &str = "HANDOFF-REPLACE-UNSUPPORTED";
pub const PUBLISH_FAILED: &str = "HANDOFF-PUBLISH-FAILED";
pub const STAGING_LEFTOVER: &str = "HANDOFF-STAGING-LEFTOVER";
pub const OLD_PACKAGE_LEFT: &str = "HANDOFF-OLD-PACKAGE-LEFT";
pub const VERIFY_FAILED: &str = "HANDOFF-VERIFY-FAILED";

fn diagnostic(code: &str, path: &str, message: impl Into<String>) -> Diagnostic {
    Diagnostic { code: code.to_string(), path: path.to_string(), message: message.into() }
}


// ---------------------------------------------------------------------------
// The contract envelope and the checksum inventory (shared with the verifier).
// ---------------------------------------------------------------------------

/// The schema instance around a manifest: the root constants and the
/// `handoff_contract_status` constants (`physical_package_container` stays
/// `TBD`: the directory package is not a container ruling, design §0).
pub fn handoff_instance(manifest: Value) -> Value {
    json!({
        "schema_version": HANDOFF_SCHEMA_VERSION,
        "deliverable_id": "DEL-15-01",
        "package_id": "PKG-15",
        "scope_item": "SOW-074",
        "objectives": ["OBJ-017"],
        "handoff_contract_status": {
            "record_contract": "schema_first_canonical_handoff_package_records",
            "manifest_contract": "schema_first_handoff_manifest_metadata",
            "model_schema_binding": "schemas/model.schema.yaml",
            "units_schema_binding": "schemas/units.schema.yaml",
            "model_state_binding": "schemas/model_state.schema.json",
            "analysis_run_binding": "schemas/analysis_run.schema.json",
            "result_export_binding": "schemas/results.schema.yaml",
            "local_fea_handoff_binding": "schemas/local_fea_handoff.schema.yaml",
            "audit_hash_binding": "hash_records_follow_audit_manifest_predecessor_semantics",
            "target_mapping_taxonomy": "reserved_for_DEL-15-02",
            "physical_package_container": "TBD",
            "external_prover_status": "not_declared_by_handoff_package",
            "professional_boundary": "downstream_modeling_and_review_support_only"
        },
        "handoff_package_manifest": manifest
    })
}

/// The export hash: SHA-256 hex of the JCS of the manifest object with its
/// `checksums` member removed.
pub fn export_hash_hex(manifest: &Map<String, Value>) -> String {
    let mut payload = manifest.clone();
    payload.remove("checksums");
    sha256_hex(&canonical_json(&Value::Object(payload)))
}

/// The member inventory: (1) `model/model.json` by the SHA-256 of its bytes,
/// which are JCS text; (2) the export hash of the manifest.
pub fn checksum_inventory(model_ref: &Value, handoff_package_id: &str, model_hex: &str, export_hex: &str) -> Value {
    let mut member_ref = model_ref.clone();
    member_ref["label"] = json!(MODEL_MEMBER);
    json!([
        {
            "algorithm": "sha256",
            "canonicalization": "JCS",
            "payload_ref": member_ref,
            "payload_scope": "model_payload",
            "value": model_hex
        },
        {
            "algorithm": "sha256",
            "canonicalization": "JCS",
            "payload_ref": { "object_type": "HandoffPackage", "ref": handoff_package_id, "label": MANIFEST_MEMBER },
            "payload_scope": "handoff_package_manifest",
            "value": export_hex
        }
    ])
}

/// Completes a manifest built without `checksums`: inserts the inventory for
/// the model bytes' hash and returns the instance's compact JCS text and the
/// export hash.
pub fn seal_manifest(mut manifest: Map<String, Value>, model_hex: &str) -> (String, String) {
    manifest.remove("checksums");
    let export_hex = export_hash_hex(&manifest);
    let model_ref = manifest.get("model_hash").and_then(|hash| hash.get("payload_ref")).cloned().unwrap_or(Value::Null);
    let package_id = manifest
        .get("package_identity")
        .and_then(|identity| identity.get("handoff_package_id"))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    manifest.insert("checksums".to_string(), checksum_inventory(&model_ref, &package_id, model_hex, &export_hex));
    (canonical_json(&handoff_instance(Value::Object(manifest))), export_hex)
}

// ---------------------------------------------------------------------------
// Layout.
// ---------------------------------------------------------------------------

fn entries(directory: &Path) -> io::Result<Vec<(String, fs::FileType)>> {
    let mut found = Vec::new();
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        // `DirEntry::file_type` does not follow symlinks.
        let kind = entry.file_type()?;
        found.push((entry.file_name().to_string_lossy().into_owned(), kind));
    }
    found.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(found)
}

/// True when `directory` (itself not a symlink) holds exactly the two members,
/// as regular files in a real `model/` directory, besides Finder metadata files.
pub fn is_package_layout(directory: &Path) -> io::Result<bool> {
    let metadata = fs::symlink_metadata(directory)?;
    if !metadata.is_dir() {
        return Ok(false);
    }
    let top = entries(directory)?;
    let top: Vec<_> = top.into_iter().filter(|(name, kind)| !(name == FINDER_METADATA && kind.is_file())).collect();
    let layout = top.len() == 2
        && top.iter().any(|(name, kind)| name == MANIFEST_MEMBER && kind.is_file())
        && top.iter().any(|(name, kind)| name == MODEL_DIRECTORY && kind.is_dir());
    if !layout {
        return Ok(false);
    }
    let model: Vec<_> = entries(&directory.join(MODEL_DIRECTORY))?
        .into_iter()
        .filter(|(name, kind)| !(name == FINDER_METADATA && kind.is_file()))
        .collect();
    Ok(model.len() == 1 && model[0].0 == MODEL_FILE && model[0].1.is_file())
}

fn remove_file_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

fn remove_dir_if_present(path: &Path) -> io::Result<()> {
    match fs::remove_dir(path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        other => other,
    }
}

/// Removes a package directory (or a partly written staging directory) by its
/// known members only. Anything else in it makes a directory removal fail, so
/// nothing the package did not hold is ever deleted.
pub fn remove_package(directory: &Path) -> io::Result<()> {
    let model = directory.join(MODEL_DIRECTORY);
    remove_file_if_present(&model.join(MODEL_FILE))?;
    remove_file_if_present(&model.join(FINDER_METADATA))?;
    remove_dir_if_present(&model)?;
    remove_file_if_present(&directory.join(MANIFEST_MEMBER))?;
    remove_file_if_present(&directory.join(FINDER_METADATA))?;
    remove_dir_if_present(directory)
}

// ---------------------------------------------------------------------------
// Publication.
// ---------------------------------------------------------------------------

/// The steps publication delegates, so tests can inject failures.
pub struct PublishHooks<'a> {
    /// After `model/model.json` is written and before the manifest.
    pub after_model_member: &'a dyn Fn() -> io::Result<()>,
    /// After both members are written and verified, before the commit rename.
    pub before_commit: &'a dyn Fn() -> io::Result<()>,
    /// Publishes into a free name, refusing to replace anything.
    pub rename_new: &'a dyn Fn(&Path, &Path) -> io::Result<()>,
    /// Exchanges the staging directory and the existing package.
    pub swap: &'a dyn Fn(&Path, &Path) -> io::Result<()>,
    /// The staging directory's random suffix.
    pub token: &'a dyn Fn() -> io::Result<String>,
}

fn no_hook() -> io::Result<()> {
    Ok(())
}

pub const PRODUCTION_HOOKS: PublishHooks<'static> = PublishHooks {
    after_model_member: &no_hook,
    before_commit: &no_hook,
    rename_new: &platform_rename_new,
    swap: &platform_swap,
    token: &random_token,
};

fn random_token() -> io::Result<String> {
    let mut bytes = [0u8; 8];
    getrandom::fill(&mut bytes).map_err(|error| io::Error::other(error.to_string()))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// The volume cannot perform the requested rename (`ENOTSUP`, `EINVAL`).
fn unsupported(error: &io::Error) -> bool {
    #[cfg(unix)]
    if matches!(error.raw_os_error(), Some(libc::ENOTSUP) | Some(libc::EINVAL)) {
        return true;
    }
    error.kind() == io::ErrorKind::Unsupported
}

#[cfg(target_os = "macos")]
fn renamex(from: &Path, to: &Path, flags: libc::c_uint) -> io::Result<()> {
    use std::ffi::CString;
    use std::os::unix::ffi::OsStrExt;
    let from = CString::new(from.as_os_str().as_bytes())?;
    let to = CString::new(to.as_os_str().as_bytes())?;
    // SAFETY: both arguments are NUL-terminated paths that outlive the call.
    if unsafe { libc::renamex_np(from.as_ptr(), to.as_ptr(), flags) } == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

/// macOS `renamex_np(RENAME_SWAP)`; elsewhere unsupported.
pub fn platform_swap(staging: &Path, target: &Path) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    return renamex(staging, target, libc::RENAME_SWAP);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (staging, target);
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}

/// macOS `renamex_np(RENAME_EXCL)`. Where the volume cannot honour it (or off
/// macOS) the name is checked and renamed (best effort, as the file writer's
/// same-parent rename: not protected against a concurrent writer).
pub fn platform_rename_new(staging: &Path, target: &Path) -> io::Result<()> {
    #[cfg(target_os = "macos")]
    match renamex(staging, target, libc::RENAME_EXCL) {
        Err(error) if unsupported(&error) => {}
        result => return result,
    }
    if fs::symlink_metadata(target).is_ok() {
        return Err(io::Error::from(io::ErrorKind::AlreadyExists));
    }
    fs::rename(staging, target)
}

fn write_member(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    file.flush()?;
    file.sync_all()
}

fn sync_directory(path: &Path) -> io::Result<()> {
    File::open(path)?.sync_all()
}

/// A published package.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Published {
    pub package_name: String,
    pub replaced_existing: bool,
    /// `durable`, or `saved_durability_uncertain` when the parent directory's
    /// sync after the commit did not complete.
    pub durability: String,
    /// Non-blocking findings: leftover staging directories, an old package
    /// that could not be removed after the swap.
    pub warnings: Vec<Diagnostic>,
}

fn file_name(path: &Path) -> String {
    path.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default()
}

/// The target's checks: a `<stem>.swbhandoff` name in an existing non-symlink
/// directory. Returns the parent and whether a package is to be replaced; an
/// existing destination that is not a handoff package is refused.
fn admit_target(target: &Path) -> Result<(PathBuf, bool), Diagnostic> {
    let name = file_name(target);
    let stem_ok = name
        .strip_suffix(&format!(".{PACKAGE_EXTENSION}"))
        .is_some_and(|stem| !stem.is_empty() && !stem.starts_with('.'));
    if !stem_ok || name.contains(STAGING_INFIX) {
        return Err(diagnostic(DESTINATION_INVALID, "", format!("The package name must be <name>.{PACKAGE_EXTENSION}; `{name}` is not.")));
    }
    let parent = target
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| diagnostic(DESTINATION_INVALID, "", "The package needs an existing parent folder."))?;
    match fs::symlink_metadata(parent) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
        _ => return Err(diagnostic(DESTINATION_INVALID, "", "The package's folder must be an existing folder, not a link.")),
    }
    let replacing = match fs::symlink_metadata(target) {
        Ok(_) => match is_package_layout(target) {
            Ok(true) => true,
            _ => {
                return Err(diagnostic(
                    DESTINATION_NOT_PACKAGE,
                    "",
                    format!("`{name}` exists and is not a handoff package (exactly {MANIFEST_MEMBER} and {MODEL_MEMBER}); it is never replaced. Choose another name."),
                ))
            }
        },
        Err(error) if error.kind() == io::ErrorKind::NotFound => false,
        Err(error) => return Err(diagnostic(DESTINATION_INVALID, "", format!("`{name}` could not be inspected ({error})."))),
    };
    Ok((parent.to_path_buf(), replacing))
}

/// Leftover staging directories of this package name, sorted.
fn leftover_staging(parent: &Path, name: &str) -> Vec<String> {
    let prefix = format!("{name}{STAGING_INFIX}");
    let mut found: Vec<String> = fs::read_dir(parent)
        .map(|listing| {
            listing
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .filter(|entry| entry.starts_with(&prefix))
                .collect()
        })
        .unwrap_or_default();
    found.sort();
    found
}

fn failed(stage: &str, error: impl std::fmt::Display, staging: &Path) -> Diagnostic {
    let cleanup = match remove_package(staging) {
        Ok(()) => String::new(),
        Err(cleanup) => format!(" The staging folder {} could not be removed ({cleanup}); delete it.", file_name(staging)),
    };
    diagnostic(
        PUBLISH_FAILED,
        "",
        format!("The handoff package was not published ({stage}: {error}); the previous package, if any, is unchanged.{cleanup}"),
    )
}

/// Publishes the two members at `target` (see the module docs).
pub fn publish_package(target: &Path, model: &[u8], manifest: &[u8], hooks: &PublishHooks<'_>) -> Result<Published, Diagnostic> {
    let (parent, replacing) = admit_target(target)?;
    let name = file_name(target);
    let mut warnings: Vec<Diagnostic> = leftover_staging(&parent, &name)
        .into_iter()
        .map(|leftover| {
            diagnostic(
                STAGING_LEFTOVER,
                "",
                format!("{leftover} is left from an earlier save that did not finish. It was ignored; delete it when convenient."),
            )
        })
        .collect();
    let token = (hooks.token)().map_err(|error| diagnostic(PUBLISH_FAILED, "", format!("No staging name could be drawn ({error}).")))?;
    let staging = parent.join(format!("{name}{STAGING_INFIX}{token}"));
    fs::create_dir(&staging)
        .map_err(|error| diagnostic(PUBLISH_FAILED, "", format!("The staging folder could not be created ({error}); nothing was written.")))?;
    let write = || -> io::Result<()> {
        let model_directory = staging.join(MODEL_DIRECTORY);
        fs::create_dir(&model_directory)?;
        write_member(&model_directory.join(MODEL_FILE), model)?;
        (hooks.after_model_member)()?;
        write_member(&staging.join(MANIFEST_MEMBER), manifest)?;
        sync_directory(&model_directory)?;
        sync_directory(&staging)
    };
    if let Err(error) = write() {
        return Err(failed("write", error, &staging));
    }
    if let Err(error) = verify_handoff_package(&staging) {
        return Err(failed("verify", error, &staging));
    }
    if let Err(error) = (hooks.before_commit)() {
        return Err(failed("commit", error, &staging));
    }
    if replacing {
        if let Err(error) = (hooks.swap)(&staging, target) {
            if unsupported(&error) {
                let cleanup = remove_package(&staging).err().map(|cleanup| format!(" The staging folder {} could not be removed ({cleanup}); delete it.", file_name(&staging))).unwrap_or_default();
                return Err(diagnostic(
                    REPLACE_UNSUPPORTED,
                    "",
                    format!("This volume cannot replace `{name}` in one step, so it was left unchanged; save under a new name.{cleanup}"),
                ));
            }
            return Err(failed("swap", error, &staging));
        }
        // The staging name now holds the old package.
        match is_package_layout(&staging).map(|layout| layout && remove_package(&staging).is_ok()) {
            Ok(true) => {}
            _ => warnings.push(diagnostic(
                OLD_PACKAGE_LEFT,
                "",
                format!("The replaced package could not be removed and is left as {}; delete it when convenient.", file_name(&staging)),
            )),
        }
    } else if let Err(error) = (hooks.rename_new)(&staging, target) {
        if error.kind() == io::ErrorKind::AlreadyExists {
            let _ = remove_package(&staging);
            return Err(diagnostic(DESTINATION_APPEARED, "", format!("`{name}` appeared while the package was written; nothing was replaced. Save again.")));
        }
        return Err(failed("rename", error, &staging));
    }
    let durability = if crate::atomic_report_package_save::sync_directory_durable(&parent) { "durable" } else { "saved_durability_uncertain" };
    Ok(Published { package_name: name, replaced_existing: replacing, durability: durability.to_string(), warnings })
}

// ---------------------------------------------------------------------------
// The verifier.
// ---------------------------------------------------------------------------

/// A verified package's two hashes, `sha256:<hex>`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct HandoffVerification {
    pub model_hash: String,
    pub export_hash: String,
}

fn own_jcs_text(bytes: &[u8], member: &str) -> Result<Value, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| format!("{member} is not UTF-8"))?;
    let value: Value = serde_json::from_str(text).map_err(|error| format!("{member} is not JSON ({error})"))?;
    if canonical_json(&value) != text {
        return Err(format!("{member} is not its own RFC 8785 (JCS) text"));
    }
    Ok(value)
}

/// `verify_handoff_package` (design §3): the package's layout, both members as
/// their own JCS text, the model member's SHA-256 as the manifest's
/// `model_hash`, and the instance as exactly the contract envelope around its
/// manifest with the checksum inventory the recomputed hashes give.
pub fn verify_handoff_package(directory: &Path) -> Result<HandoffVerification, String> {
    let fail = |detail: String| format!("{VERIFY_FAILED}: {detail}");
    match is_package_layout(directory) {
        Ok(true) => {}
        Ok(false) => return Err(fail(format!("the folder does not hold exactly {MANIFEST_MEMBER} and {MODEL_MEMBER}"))),
        Err(error) => return Err(fail(format!("the folder could not be read ({error})"))),
    }
    let model_bytes = fs::read(directory.join(MODEL_DIRECTORY).join(MODEL_FILE)).map_err(|error| fail(format!("{MODEL_MEMBER}: {error}")))?;
    own_jcs_text(&model_bytes, MODEL_MEMBER).map_err(fail)?;
    let model_hex = sha256_hex(std::str::from_utf8(&model_bytes).expect("checked UTF-8"));
    let manifest_bytes = fs::read(directory.join(MANIFEST_MEMBER)).map_err(|error| fail(format!("{MANIFEST_MEMBER}: {error}")))?;
    let instance = own_jcs_text(&manifest_bytes, MANIFEST_MEMBER).map_err(fail)?;
    let Some(manifest) = instance.get("handoff_package_manifest").and_then(Value::as_object) else {
        return Err(fail(format!("{MANIFEST_MEMBER} has no handoff_package_manifest object")));
    };
    if manifest.get("model_hash").and_then(|hash| hash.get("value")).and_then(Value::as_str) != Some(model_hex.as_str()) {
        return Err(fail(format!("model_hash is not the SHA-256 of {MODEL_MEMBER} ({model_hex})")));
    }
    let mut unsealed = manifest.clone();
    unsealed.remove("checksums");
    let (expected, export_hex) = seal_manifest(unsealed, &model_hex);
    if expected.as_bytes() != manifest_bytes.as_slice() {
        return Err(fail(format!(
            "{MANIFEST_MEMBER} is not the contract envelope with the checksum inventory of {MODEL_MEMBER} (sha256 {model_hex}) and the manifest's export hash (sha256 {export_hex})"
        )));
    }
    Ok(HandoffVerification { model_hash: format!("sha256:{model_hex}"), export_hash: format!("sha256:{export_hex}") })
}
