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
//! Publication ([`publish_package`]) works through directory handles only
//! (review round 2, finding 3): the parent folder is opened once
//! (`O_DIRECTORY | O_NOFOLLOW`), and every later open, create, read, removal
//! and rename is relative to a held handle (`openat`, `mkdirat`, `fstatat`,
//! `unlinkat`, `renameatx_np`), one path component at a time, never following
//! a symlink and never resolving a path again by name.
//! 1. An existing destination is admitted for replacement only when it
//!    verifies as a handoff package ([`verify_handoff_package`]); its folder,
//!    `model/` folder and members are recorded by device and inode.
//! 2. Both members are written into a staging directory
//!    `<name>.swbhandoff.staging-<random>` beside the target (each created
//!    `O_EXCL | O_NOFOLLOW`, fsynced, and recorded by identity), and the staging
//!    directory is verified.
//! 3. A new name is published by `renameatx_np(RENAME_EXCL)`, refusing to
//!    replace anything, the single commit point. An existing package is
//!    replaced by `renameatx_np(RENAME_SWAP)` after checking that the
//!    destination is still the package admitted; what the swap moved out is
//!    checked again, swapped back if it is not, and otherwise removed member by
//!    member (never a recursive delete). A volume that cannot swap refuses
//!    `HANDOFF-REPLACE-UNSUPPORTED`.
//! 4. A leftover `<name>.swbhandoff.staging-*` from an earlier crash is ignored
//!    and reported for the user to delete.
//!
//! Removal (cleanup of the staging directory, the replaced package) unlinks an
//! entry only while it is, by device and inode, the one this save created or
//! verified, and removes a folder only when it is then empty. A substituted
//! symlink, folder or file is left in place and reported; a hard-linked member
//! loses only the package's link.
//!
//! Residual window (stated after review round 2). The identity checks and the
//! name-based calls that follow them (`unlinkat`, `unlinkat(AT_REMOVEDIR)`,
//! `renameatx_np`) are separate system calls inside held directory handles. A
//! process that can write those folders can, between a check and its call:
//! - substitute one directory entry there, which is then unlinked in place of
//!   the checked one (the entry itself only: never a symlink's target, never a
//!   non-empty folder, never a file's other hard links);
//! - replace the destination before the swap: detected after it by identity
//!   and swapped back; if the swap-back fails, the other folder is left under
//!   the staging name, which the refusal names, and nothing of it is removed;
//! - on a volume without `RENAME_EXCL`, create the destination between the
//!   name check and the plain `renameat` (an empty folder there can be
//!   replaced).
//! Rewrites of member bytes in place (same inode) after verification are not
//! detected. The parent path's own components are resolved once, at admission;
//! a parent folder moved later receives the package where it then is.
//!
//! Every failure before the commit removes this run's staging directory (as
//! above) and leaves the previous package as it was.
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
use std::fs::File;
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
pub const DESTINATION_CHANGED: &str = "HANDOFF-DESTINATION-CHANGED";
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
// Directory handles (review round 2, finding 3).
// ---------------------------------------------------------------------------
//
// Every open, create, read, removal and rename below goes through a directory
// handle opened with `O_DIRECTORY | O_NOFOLLOW`, relative to its parent's
// handle: a name is resolved once, as one component, and no symlink is
// followed. Removals use `unlinkat` (a symlink is never followed; only an entry
// whose device and inode are the ones verified or created is removed) and
// `unlinkat(AT_REMOVEDIR)` (only an empty directory). Renames use
// `renameatx_np` relative to the parent's handle.

use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};

/// A file or directory identity: device and inode.
pub type EntryId = (u64, u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    Directory,
    File,
    Other,
}

fn c_name(name: &str) -> io::Result<std::ffi::CString> {
    if name.is_empty() || name.contains('/') || name == "." || name == ".." {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("`{name}` is not one path component")));
    }
    Ok(std::ffi::CString::new(name)?)
}

fn check(result: libc::c_int) -> io::Result<libc::c_int> {
    if result < 0 { Err(io::Error::last_os_error()) } else { Ok(result) }
}

fn stat_kind(stat: &libc::stat) -> Kind {
    match stat.st_mode & libc::S_IFMT {
        libc::S_IFDIR => Kind::Directory,
        libc::S_IFREG => Kind::File,
        _ => Kind::Other,
    }
}

fn stat_id(stat: &libc::stat) -> EntryId {
    (stat.st_dev as u64, stat.st_ino as u64)
}

const DIR_FLAGS: libc::c_int = libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC;

/// A directory handle.
#[derive(Debug)]
pub struct Dir(OwnedFd);

impl Dir {
    /// Opens a directory by path; its last component must not be a symlink.
    pub fn open_path(path: &Path) -> io::Result<Dir> {
        use std::os::unix::ffi::OsStrExt;
        let path = std::ffi::CString::new(path.as_os_str().as_bytes())?;
        // SAFETY: a NUL-terminated path; the descriptor is owned on success.
        let fd = check(unsafe { libc::open(path.as_ptr(), DIR_FLAGS) })?;
        Ok(Dir(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn fd(&self) -> libc::c_int {
        self.0.as_raw_fd()
    }

    /// The child directory `name`, never through a symlink.
    pub fn open_dir(&self, name: &str) -> io::Result<Dir> {
        let name = c_name(name)?;
        // SAFETY: a valid directory descriptor and a NUL-terminated name.
        let fd = check(unsafe { libc::openat(self.fd(), name.as_ptr(), DIR_FLAGS) })?;
        Ok(Dir(unsafe { OwnedFd::from_raw_fd(fd) }))
    }

    fn stat(&self) -> io::Result<libc::stat> {
        // SAFETY: `stat` is plain data that `fstat` fills.
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        check(unsafe { libc::fstat(self.fd(), &mut stat) })?;
        Ok(stat)
    }

    pub fn id(&self) -> io::Result<EntryId> {
        Ok(stat_id(&self.stat()?))
    }

    /// The child entry `name` itself (a symlink is reported, not followed).
    fn stat_child(&self, name: &str) -> io::Result<libc::stat> {
        let name = c_name(name)?;
        // SAFETY: as above; AT_SYMLINK_NOFOLLOW reports the link itself.
        let mut stat: libc::stat = unsafe { std::mem::zeroed() };
        check(unsafe { libc::fstatat(self.fd(), name.as_ptr(), &mut stat, libc::AT_SYMLINK_NOFOLLOW) })?;
        Ok(stat)
    }

    fn child_id(&self, name: &str) -> io::Result<EntryId> {
        Ok(stat_id(&self.stat_child(name)?))
    }

    /// Every entry's name and kind (no symlink followed), sorted by name.
    fn entries(&self) -> io::Result<Vec<(String, Kind)>> {
        // SAFETY: `fdopendir` takes ownership of the duplicated descriptor and
        // `closedir` releases both; `readdir` entries are read before the next call.
        let duplicate = check(unsafe { libc::fcntl(self.fd(), libc::F_DUPFD_CLOEXEC, 0) })?;
        let stream = unsafe { libc::fdopendir(duplicate) };
        if stream.is_null() {
            let error = io::Error::last_os_error();
            unsafe { libc::close(duplicate) };
            return Err(error);
        }
        // The duplicate shares the handle's offset: read from the start each time.
        unsafe { libc::rewinddir(stream) };
        let mut names = Vec::new();
        loop {
            let entry = unsafe { libc::readdir(stream) };
            if entry.is_null() {
                break;
            }
            let name = unsafe { std::ffi::CStr::from_ptr((*entry).d_name.as_ptr()) }.to_string_lossy().into_owned();
            if name != "." && name != ".." {
                names.push(name);
            }
        }
        unsafe { libc::closedir(stream) };
        let mut found = Vec::with_capacity(names.len());
        for name in names {
            let kind = stat_kind(&self.stat_child(&name)?);
            found.push((name, kind));
        }
        found.sort();
        Ok(found)
    }

    fn create_dir(&self, name: &str) -> io::Result<Dir> {
        let c = c_name(name)?;
        // SAFETY: as above.
        check(unsafe { libc::mkdirat(self.fd(), c.as_ptr(), 0o755) })?;
        self.open_dir(name)
    }

    /// Creates `name` (never an existing entry or a link), writes and fsyncs it.
    fn create_file(&self, name: &str, bytes: &[u8]) -> io::Result<EntryId> {
        let c = c_name(name)?;
        let flags = libc::O_WRONLY | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW | libc::O_CLOEXEC;
        // SAFETY: as above; the descriptor is owned by the File.
        let fd = check(unsafe { libc::openat(self.fd(), c.as_ptr(), flags, 0o644 as libc::c_uint) })?;
        let mut file = unsafe { File::from_raw_fd(fd) };
        use std::os::unix::fs::MetadataExt;
        let metadata = file.metadata()?;
        let id = (metadata.dev(), metadata.ino());
        let written = file.write_all(bytes).and_then(|()| file.flush()).and_then(|()| file.sync_all());
        if let Err(error) = written {
            // The partial file is this save's: removed while it is still the one created.
            drop(file);
            let _ = self.unlink_if(name, id);
            return Err(error);
        }
        Ok(id)
    }

    /// Reads the regular file `name` (never through a link), at most `limit` bytes.
    fn read_file(&self, name: &str, limit: u64) -> io::Result<(Vec<u8>, EntryId)> {
        use std::io::Read;
        use std::os::unix::fs::MetadataExt;
        let c = c_name(name)?;
        let flags = libc::O_RDONLY | libc::O_NOFOLLOW | libc::O_CLOEXEC;
        // SAFETY: as above.
        let fd = check(unsafe { libc::openat(self.fd(), c.as_ptr(), flags) })?;
        let file = unsafe { File::from_raw_fd(fd) };
        let metadata = file.metadata()?;
        if !metadata.is_file() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{name} is not a regular file")));
        }
        if metadata.len() > limit {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{name} is larger than {limit} bytes")));
        }
        let mut bytes = Vec::with_capacity(metadata.len() as usize);
        file.take(limit + 1).read_to_end(&mut bytes)?;
        if bytes.len() as u64 > limit {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("{name} is larger than {limit} bytes")));
        }
        Ok((bytes, (metadata.dev(), metadata.ino())))
    }

    /// Removes the non-directory entry `name` if it is still `expected`.
    fn unlink_if(&self, name: &str, expected: EntryId) -> io::Result<()> {
        let stat = self.stat_child(name)?;
        if stat_id(&stat) != expected || stat_kind(&stat) == Kind::Directory {
            return Err(io::Error::other(format!("{name} is not the entry that was verified")));
        }
        let c = c_name(name)?;
        // SAFETY: as above; unlinkat never follows a symlink.
        check(unsafe { libc::unlinkat(self.fd(), c.as_ptr(), 0) })?;
        Ok(())
    }

    /// Removes the empty directory `name` if it is still `expected`.
    fn rmdir_if(&self, name: &str, expected: EntryId) -> io::Result<()> {
        let stat = self.stat_child(name)?;
        if stat_id(&stat) != expected || stat_kind(&stat) != Kind::Directory {
            return Err(io::Error::other(format!("{name} is not the folder that was verified")));
        }
        let c = c_name(name)?;
        // SAFETY: as above; AT_REMOVEDIR removes only an empty directory.
        check(unsafe { libc::unlinkat(self.fd(), c.as_ptr(), libc::AT_REMOVEDIR) })?;
        Ok(())
    }

    /// Removes a regular Finder metadata file `name`, if there is one.
    fn unlink_finder_metadata(&self) -> io::Result<()> {
        match self.stat_child(FINDER_METADATA) {
            Ok(stat) if stat_kind(&stat) == Kind::File => self.unlink_if(FINDER_METADATA, stat_id(&stat)),
            Ok(_) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error),
        }
    }

    fn sync(&self) -> io::Result<()> {
        // SAFETY: a valid descriptor.
        check(unsafe { libc::fsync(self.fd()) }).map(drop)
    }

    #[cfg(target_os = "macos")]
    fn rename_with(&self, from: &str, to: &str, flags: libc::c_uint) -> io::Result<()> {
        let (from, to) = (c_name(from)?, c_name(to)?);
        // SAFETY: valid descriptors and NUL-terminated names, both relative to this directory.
        check(unsafe { libc::renameatx_np(self.fd(), from.as_ptr(), self.fd(), to.as_ptr(), flags) }).map(drop)
    }

    /// `renameatx_np(RENAME_SWAP)`; elsewhere unsupported.
    fn swap(&self, from: &str, to: &str) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        return self.rename_with(from, to, libc::RENAME_SWAP);
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (from, to);
            Err(io::Error::from(io::ErrorKind::Unsupported))
        }
    }

    /// `renameatx_np(RENAME_EXCL)`; where the volume cannot honour it, the name
    /// is checked and renamed (not protected against a concurrent writer).
    fn rename_new(&self, from: &str, to: &str) -> io::Result<()> {
        #[cfg(target_os = "macos")]
        match self.rename_with(from, to, libc::RENAME_EXCL) {
            Err(error) if unsupported(&error) => {}
            result => return result,
        }
        if self.stat_child(to).is_ok() {
            return Err(io::Error::from(io::ErrorKind::AlreadyExists));
        }
        let (from, to) = (c_name(from)?, c_name(to)?);
        // SAFETY: as above.
        check(unsafe { libc::renameat(self.fd(), from.as_ptr(), self.fd(), to.as_ptr()) }).map(drop)
    }
}

// ---------------------------------------------------------------------------
// Layout and identity.
// ---------------------------------------------------------------------------

/// A package directory's identity: the folder, its `model/` folder and both
/// member files, each by device and inode, with the package layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PackageIdentity {
    folder: EntryId,
    model_folder: EntryId,
    model_file: EntryId,
    manifest: EntryId,
}

fn without_finder_metadata(entries: Vec<(String, Kind)>) -> Vec<(String, Kind)> {
    entries.into_iter().filter(|(name, kind)| !(name == FINDER_METADATA && *kind == Kind::File)).collect()
}

/// The identity of the package folder `dir` when it holds exactly the two
/// members, as regular files in a real `model/` folder, besides Finder
/// metadata files; otherwise `None`.
fn layout_identity(dir: &Dir) -> io::Result<Option<(PackageIdentity, Dir)>> {
    let top = without_finder_metadata(dir.entries()?);
    let wanted = vec![(MANIFEST_MEMBER.to_string(), Kind::File), (MODEL_DIRECTORY.to_string(), Kind::Directory)];
    if top != wanted {
        return Ok(None);
    }
    let model = dir.open_dir(MODEL_DIRECTORY)?;
    if without_finder_metadata(model.entries()?) != vec![(MODEL_FILE.to_string(), Kind::File)] {
        return Ok(None);
    }
    let identity = PackageIdentity {
        folder: dir.id()?,
        model_folder: model.id()?,
        model_file: model.child_id(MODEL_FILE)?,
        manifest: dir.child_id(MANIFEST_MEMBER)?,
    };
    // The handle and the name agree on the model folder.
    if dir.child_id(MODEL_DIRECTORY)? != identity.model_folder {
        return Ok(None);
    }
    Ok(Some((identity, model)))
}

/// The package identity of the folder `name` in `parent`, if it has the layout.
fn identity_at(parent: &Dir, name: &str) -> Option<PackageIdentity> {
    let dir = parent.open_dir(name).ok()?;
    if parent.child_id(name).ok()? != dir.id().ok()? {
        return None;
    }
    layout_identity(&dir).ok().flatten().map(|(identity, _)| identity)
}

/// What this save created in its staging folder, by identity.
#[derive(Debug, Default, Clone, Copy)]
struct Created {
    folder: Option<EntryId>,
    model_folder: Option<EntryId>,
    model_file: Option<EntryId>,
    manifest: Option<EntryId>,
}

impl From<PackageIdentity> for Created {
    fn from(identity: PackageIdentity) -> Self {
        Created { folder: Some(identity.folder), model_folder: Some(identity.model_folder), model_file: Some(identity.model_file), manifest: Some(identity.manifest) }
    }
}

/// Removes the folder `name` in `parent` and its known members, only where each
/// entry is still the one `known` names (never a symlink's target, never
/// anything else: a directory that holds anything more is not removed).
fn remove_known(parent: &Dir, name: &str, known: Created) -> io::Result<()> {
    let Some(folder_id) = known.folder else { return Ok(()) };
    let folder = parent.open_dir(name)?;
    if folder.id()? != folder_id || parent.child_id(name)? != folder_id {
        return Err(io::Error::other(format!("{name} is not the folder that was verified")));
    }
    if let Some(model_id) = known.model_folder {
        let model = folder.open_dir(MODEL_DIRECTORY)?;
        if model.id()? != model_id {
            return Err(io::Error::other(format!("{name}/{MODEL_DIRECTORY} is not the folder that was verified")));
        }
        if let Some(file) = known.model_file {
            model.unlink_if(MODEL_FILE, file)?;
        }
        model.unlink_finder_metadata()?;
        folder.rmdir_if(MODEL_DIRECTORY, model_id)?;
    }
    if let Some(manifest) = known.manifest {
        folder.unlink_if(MANIFEST_MEMBER, manifest)?;
    }
    folder.unlink_finder_metadata()?;
    parent.rmdir_if(name, folder_id)
}

// ---------------------------------------------------------------------------
// Publication.
// ---------------------------------------------------------------------------

/// A path-based rename a test substitutes for the directory-handle one.
pub type RenameHook<'a> = &'a dyn Fn(&Path, &Path) -> io::Result<()>;

/// The steps publication delegates, so tests can inject failures.
pub struct PublishHooks<'a> {
    /// After `model/model.json` is written and before the manifest.
    pub after_model_member: &'a dyn Fn() -> io::Result<()>,
    /// After both members are written and verified, before the commit rename.
    pub before_commit: &'a dyn Fn() -> io::Result<()>,
    /// Publishes into a free name, refusing to replace anything (`None`: the
    /// directory handle's `renameatx_np(RENAME_EXCL)`).
    pub rename_new: Option<RenameHook<'a>>,
    /// Exchanges the staging directory and the existing package (`None`: the
    /// directory handle's `renameatx_np(RENAME_SWAP)`).
    pub swap: Option<RenameHook<'a>>,
    /// The staging directory's random suffix.
    pub token: &'a dyn Fn() -> io::Result<String>,
}

fn no_hook() -> io::Result<()> {
    Ok(())
}

pub const PRODUCTION_HOOKS: PublishHooks<'static> = PublishHooks {
    after_model_member: &no_hook,
    before_commit: &no_hook,
    rename_new: None,
    swap: None,
    token: &random_token,
};

fn random_token() -> io::Result<String> {
    let mut bytes = [0u8; 8];
    getrandom::fill(&mut bytes).map_err(|error| io::Error::other(error.to_string()))?;
    Ok(bytes.iter().map(|byte| format!("{byte:02x}")).collect())
}

/// The volume cannot perform the requested rename (`ENOTSUP`, `EINVAL`).
fn unsupported(error: &io::Error) -> bool {
    matches!(error.raw_os_error(), Some(libc::ENOTSUP) | Some(libc::EINVAL)) || error.kind() == io::ErrorKind::Unsupported
}

/// macOS `renamex_np(RENAME_SWAP)` by path: only for tests that substitute the
/// swap and then perform it (production swaps through the parent's handle).
pub fn platform_swap(staging: &Path, target: &Path) -> io::Result<()> {
    let (Some(parent), Some(from), Some(to)) = (staging.parent(), staging.file_name(), target.file_name()) else {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    };
    Dir::open_path(parent)?.swap(&from.to_string_lossy(), &to.to_string_lossy())
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

/// The destination admitted: the parent's handle, the package name, and the
/// identity of the existing package it replaces (none for a new name).
struct Admitted {
    parent: Dir,
    parent_path: PathBuf,
    name: String,
    replacing: Option<PackageIdentity>,
}

/// The target's checks: a `<stem>.swbhandoff` name in an existing non-symlink
/// folder. An existing destination is admitted for replacement only when it
/// verifies as a handoff package ([`verify_handoff_package`], review round 2,
/// note 6); anything else is refused and never touched.
fn admit_target(target: &Path) -> Result<Admitted, Diagnostic> {
    let name = file_name(target);
    let stem_ok = name
        .strip_suffix(&format!(".{PACKAGE_EXTENSION}"))
        .is_some_and(|stem| !stem.is_empty() && !stem.starts_with('.'));
    if !stem_ok || name.contains(STAGING_INFIX) || c_name(&name).is_err() {
        return Err(diagnostic(DESTINATION_INVALID, "", format!("The package name must be <name>.{PACKAGE_EXTENSION}; `{name}` is not.")));
    }
    let parent_path = target
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .ok_or_else(|| diagnostic(DESTINATION_INVALID, "", "The package needs an existing parent folder."))?;
    let parent = Dir::open_path(parent_path).map_err(|_| diagnostic(DESTINATION_INVALID, "", "The package's folder must be an existing folder, not a link."))?;
    let not_package = || {
        diagnostic(
            DESTINATION_NOT_PACKAGE,
            "",
            format!("`{name}` exists and is not a handoff package that verifies; it is never replaced. Choose another name."),
        )
    };
    let replacing = match parent.stat_child(&name) {
        Ok(stat) if stat_kind(&stat) == Kind::Directory => {
            let existing = parent.open_dir(&name).map_err(|_| not_package())?;
            if existing.id().ok() != Some(stat_id(&stat)) || verify_dir(&existing).is_err() {
                return Err(not_package());
            }
            match layout_identity(&existing) {
                Ok(Some((identity, _))) => Some(identity),
                _ => return Err(not_package()),
            }
        }
        Ok(_) => return Err(not_package()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(diagnostic(DESTINATION_INVALID, "", format!("`{name}` could not be inspected ({error})."))),
    };
    Ok(Admitted { parent, parent_path: parent_path.to_path_buf(), name, replacing })
}

/// Leftover staging directories of this package name, sorted.
fn leftover_staging(parent: &Dir, name: &str) -> Vec<String> {
    let prefix = format!("{name}{STAGING_INFIX}");
    parent.entries().map(|entries| entries.into_iter().map(|(entry, _)| entry).filter(|entry| entry.starts_with(&prefix)).collect()).unwrap_or_default()
}

fn cleanup_note(parent: &Dir, staging_name: &str, created: Created) -> String {
    match remove_known(parent, staging_name, created) {
        Ok(()) => String::new(),
        Err(cleanup) => format!(" The staging folder {staging_name} could not be removed ({cleanup}); delete it."),
    }
}

fn failed(stage: &str, error: impl std::fmt::Display, parent: &Dir, staging_name: &str, created: Created) -> Diagnostic {
    let cleanup = cleanup_note(parent, staging_name, created);
    diagnostic(
        PUBLISH_FAILED,
        "",
        format!("The handoff package was not published ({stage}: {error}); the previous package, if any, is unchanged.{cleanup}"),
    )
}

/// Publishes the two members at `target` (see the module docs).
pub fn publish_package(target: &Path, model: &[u8], manifest: &[u8], hooks: &PublishHooks<'_>) -> Result<Published, Diagnostic> {
    let Admitted { parent, parent_path, name, replacing } = admit_target(target)?;
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
    let staging_name = format!("{name}{STAGING_INFIX}{token}");
    if c_name(&staging_name).is_err() {
        return Err(diagnostic(PUBLISH_FAILED, "", "The staging name is not one path component; nothing was written."));
    }
    let staging_path = parent_path.join(&staging_name);
    let mut created = Created::default();
    let staging = parent
        .create_dir(&staging_name)
        .map_err(|error| diagnostic(PUBLISH_FAILED, "", format!("The staging folder could not be created ({error}); nothing was written.")))?;
    match staging.id() {
        Ok(id) => created.folder = Some(id),
        Err(error) => return Err(diagnostic(PUBLISH_FAILED, "", format!("The staging folder could not be inspected ({error}); delete {staging_name}."))),
    }
    let mut write = || -> io::Result<()> {
        let model_directory = staging.create_dir(MODEL_DIRECTORY)?;
        created.model_folder = Some(model_directory.id()?);
        created.model_file = Some(model_directory.create_file(MODEL_FILE, model)?);
        (hooks.after_model_member)()?;
        created.manifest = Some(staging.create_file(MANIFEST_MEMBER, manifest)?);
        model_directory.sync()?;
        staging.sync()
    };
    if let Err(error) = write() {
        return Err(failed("write", error, &parent, &staging_name, created));
    }
    // The staging folder verifies through the parent's handle, as published,
    // and is exactly what this save created.
    let staged = match identity_at(&parent, &staging_name) {
        Some(identity) if Created::from(identity).folder == created.folder
            && Some(identity.model_folder) == created.model_folder
            && Some(identity.model_file) == created.model_file
            && Some(identity.manifest) == created.manifest => identity,
        _ => return Err(failed("verify", "the staging folder is not what this save wrote", &parent, &staging_name, created)),
    };
    if let Err(error) = verify_dir(&staging) {
        return Err(failed("verify", error, &parent, &staging_name, created));
    }
    if let Err(error) = (hooks.before_commit)() {
        return Err(failed("commit", error, &parent, &staging_name, created));
    }
    if let Some(admitted) = replacing {
        let changed = || {
            diagnostic(
                DESTINATION_CHANGED,
                "",
                format!("`{name}` changed while the package was written (it is no longer the handoff package that was there); nothing was replaced. Save again."),
            )
        };
        // Immediately before the swap: the destination is still the package
        // admitted, by identity and layout, and the staging folder still ours.
        if identity_at(&parent, &name) != Some(admitted) {
            let _ = remove_known(&parent, &staging_name, created);
            return Err(changed());
        }
        if identity_at(&parent, &staging_name) != Some(staged) {
            return Err(failed("commit", "the staging folder changed", &parent, &staging_name, created));
        }
        // The exchange (and its undoing) of the staging folder and the destination.
        let exchange = || match hooks.swap {
            Some(swap) => swap(&staging_path, target),
            None => parent.swap(&staging_name, &name),
        };
        let swapped = exchange();
        if let Err(error) = swapped {
            if unsupported(&error) {
                let cleanup = cleanup_note(&parent, &staging_name, created);
                return Err(diagnostic(
                    REPLACE_UNSUPPORTED,
                    "",
                    format!("This volume cannot replace `{name}` in one step, so it was left unchanged; save under a new name.{cleanup}"),
                ));
            }
            return Err(failed("swap", error, &parent, &staging_name, created));
        }
        // What the swap moved out must be the package admitted. If the
        // destination was replaced between the check and the swap, the swap is
        // undone, so the other folder is back under its name, and nothing of it
        // is removed.
        if identity_at(&parent, &staging_name) != Some(admitted) {
            let ours_published = parent.child_id(&name).ok() == created.folder;
            let restored = ours_published && exchange().is_ok();
            if restored {
                let _ = remove_known(&parent, &staging_name, created);
            }
            let mut refusal = changed();
            if !restored {
                refusal.message.push_str(&format!(
                    " The folder that was there is now {staging_name}; the new package is `{name}`. Check both before deleting either."
                ));
            }
            return Err(refusal);
        }
        // The staging name now holds the admitted package: removed member by
        // member, each only while it is the entry admitted.
        if remove_known(&parent, &staging_name, Created::from(admitted)).is_err() {
            warnings.push(diagnostic(
                OLD_PACKAGE_LEFT,
                "",
                format!("The replaced package could not be removed and is left as {staging_name}; delete it when convenient."),
            ));
        }
    } else {
        let renamed = match hooks.rename_new {
            Some(rename) => rename(&staging_path, target),
            None => parent.rename_new(&staging_name, &name),
        };
        if let Err(error) = renamed {
            if error.kind() == io::ErrorKind::AlreadyExists || error.raw_os_error() == Some(libc::EEXIST) {
                let _ = remove_known(&parent, &staging_name, created);
                return Err(diagnostic(DESTINATION_APPEARED, "", format!("`{name}` appeared while the package was written; nothing was replaced. Save again.")));
            }
            return Err(failed("rename", error, &parent, &staging_name, created));
        }
    }
    let durability = if parent.sync().is_ok() { "durable" } else { "saved_durability_uncertain" };
    Ok(Published { package_name: name, replaced_existing: replacing.is_some(), durability: durability.to_string(), warnings })
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

/// The largest manifest read (the model member's limit is the model-document cap).
const MAX_MANIFEST_BYTES: u64 = 64 * 1024 * 1024;

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
/// `model_hash`, the instance as exactly the contract envelope around its
/// manifest with the checksum inventory the recomputed hashes give, and the
/// binding rules ([`crate::handoff_package::manifest_binding_findings`]).
///
/// What it proves, and what it does not: it is an offline check of the
/// package's internal consistency. It does not prove that the records the
/// manifest references outside the package exist or hold what their digests
/// say: the basis state in the local store (`model_state_refs`, its record
/// hash), the stored libraries and rule packs (the reference-metadata
/// checksums). A forger who rewrites those references and their digests
/// together produces a package this verifier accepts; only the store can
/// confirm them, which the save does when it publishes.
pub fn verify_handoff_package(directory: &Path) -> Result<HandoffVerification, String> {
    let dir = Dir::open_path(directory).map_err(|error| format!("{VERIFY_FAILED}: the folder could not be opened as a folder ({error})"))?;
    verify_dir(&dir)
}

fn verify_dir(dir: &Dir) -> Result<HandoffVerification, String> {
    let fail = |detail: String| format!("{VERIFY_FAILED}: {detail}");
    let model_folder = match layout_identity(dir) {
        Ok(Some((_, model))) => model,
        Ok(None) => return Err(fail(format!("the folder does not hold exactly {MANIFEST_MEMBER} and {MODEL_MEMBER}"))),
        Err(error) => return Err(fail(format!("the folder could not be read ({error})"))),
    };
    let (model_bytes, _) = model_folder
        .read_file(MODEL_FILE, crate::model_document_file::MAX_MODEL_DOCUMENT_BYTES)
        .map_err(|error| fail(format!("{MODEL_MEMBER}: {error}")))?;
    let model = own_jcs_text(&model_bytes, MODEL_MEMBER).map_err(fail)?;
    let model_hex = sha256_hex(std::str::from_utf8(&model_bytes).expect("checked UTF-8"));
    let (manifest_bytes, _) = dir.read_file(MANIFEST_MEMBER, MAX_MANIFEST_BYTES).map_err(|error| fail(format!("{MANIFEST_MEMBER}: {error}")))?;
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
    // The manifest binds this model member (review round 1, finding 3).
    let findings = crate::handoff_package::manifest_binding_findings(manifest, &model, &model_hex);
    if !findings.is_empty() {
        return Err(fail(format!("{MANIFEST_MEMBER} does not bind {MODEL_MEMBER}: {}", findings.join("; "))));
    }
    if expected.as_bytes() != manifest_bytes.as_slice() {
        return Err(fail(format!(
            "{MANIFEST_MEMBER} is not the contract envelope with the checksum inventory of {MODEL_MEMBER} (sha256 {model_hex}) and the manifest's export hash (sha256 {export_hex})"
        )));
    }
    Ok(HandoffVerification { model_hash: format!("sha256:{model_hex}"), export_hash: format!("sha256:{export_hex}") })
}
