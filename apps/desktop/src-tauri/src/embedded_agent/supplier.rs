//! Explicit, admitted lookup/copy of original bundled supplier bytes.
//! This confers neither launch authority nor native isolation qualification.
use crate::memory_pool::{Lease, LeasePurpose};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::Path,
    sync::Arc,
};

pub(super) const STARTUP_BYTES: u64 = 4 * 1024 * 1024;
const MANIFEST: &str = include_str!("../../resources/agent-supplier-pins.json");
const MAX_BINARY_BYTES: u64 = 512 * 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Error {
    Admission,
    UnsupportedPlatform,
    InvalidManifest,
    UnavailableResource,
    InvalidResource,
    PinMismatch,
    CopyFailed,
}
#[derive(Deserialize)]
struct Manifest {
    schema: String,
    supplier: String,
    version: String,
    targets: BTreeMap<String, Target>,
}
#[derive(Deserialize)]
struct Target {
    binary: Binary,
}
#[derive(Deserialize)]
struct Binary {
    resource_name: String,
    size: u64,
    sha256: String,
    format: String,
    architecture: String,
}
struct Pin {
    name: String,
    bytes: u64,
    digest: [u8; 32],
    cpu: u32,
}
impl Pin {
    fn select(os: &str, architecture: &str) -> Result<Self, Error> {
        let (target, expected, cpu) = match (os, architecture) {
            ("macos", "aarch64") => ("aarch64-apple-darwin", "arm64", 0x0100_000c),
            ("macos", "x86_64") => ("x86_64-apple-darwin", "x86_64", 0x0100_0007),
            _ => return Err(Error::UnsupportedPlatform),
        };
        // Trusted, bounded compile-time input, parsed only after admission.
        if MANIFEST.len() > 64 * 1024 {
            return Err(Error::InvalidManifest);
        }
        let mut manifest: Manifest =
            serde_json::from_str(MANIFEST).map_err(|_| Error::InvalidManifest)?;
        if manifest.schema != "swbpipe-agent-supplier-pins/1"
            || manifest.supplier != "openai/codex"
            || manifest.version != "0.160.0"
            || manifest.targets.len() != 2
            || !manifest.targets.contains_key("aarch64-apple-darwin")
            || !manifest.targets.contains_key("x86_64-apple-darwin")
        {
            return Err(Error::InvalidManifest);
        }
        let binary = manifest
            .targets
            .remove(target)
            .ok_or(Error::InvalidManifest)?
            .binary;
        if binary.resource_name != format!("codex-{target}")
            || binary.format != "mach-o-64-le"
            || binary.architecture != expected
            || !(32..=MAX_BINARY_BYTES).contains(&binary.size)
            || binary.sha256.len() != 64
        {
            return Err(Error::InvalidManifest);
        }
        let mut digest = [0; 32];
        for (output, pair) in digest
            .iter_mut()
            .zip(binary.sha256.as_bytes().chunks_exact(2))
        {
            let nibble = |byte| match byte {
                b'0'..=b'9' => Ok(byte - b'0'),
                b'a'..=b'f' => Ok(byte - b'a' + 10),
                _ => Err(Error::InvalidManifest),
            };
            *output = nibble(pair[0])? * 16 + nibble(pair[1])?;
        }
        Ok(Self {
            name: binary.resource_name,
            bytes: binary.size,
            digest,
            cpu,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        memory_pool::{test_host::ScriptedHost, MemoryPool},
        solve_jobs::SolveJobRegistry,
    };
    use std::{
        os::unix::fs::{symlink, DirBuilderExt},
        path::PathBuf,
    };

    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let mut nonce = [0; 16];
            getrandom::fill(&mut nonce).unwrap();
            let name: String = nonce.iter().map(|byte| format!("{byte:02x}")).collect();
            let root = Path::new("/private/tmp").join(format!("swbpipe-resource-test-{name}"));
            fs::DirBuilder::new().mode(0o700).create(&root).unwrap();
            fs::create_dir(root.join("agent-supplier")).unwrap();
            Self(root)
        }
        fn source(&self) -> PathBuf {
            self.0.join("agent-supplier/invented-supplier")
        }
        fn put(&self, bytes: &[u8]) {
            fs::write(self.source(), bytes).unwrap();
            fs::set_permissions(self.source(), fs::Permissions::from_mode(0o644)).unwrap();
        }
        fn open(&self) -> Result<BundledSupplier, Error> {
            BundledSupplier::open_pin(&self.0, pin(), backing())
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn bytes() -> Vec<u8> {
        let mut result = vec![0; 32];
        result[..4].copy_from_slice(&[0xcf, 0xfa, 0xed, 0xfe]);
        result[4..8].copy_from_slice(&0x0100_000cu32.to_le_bytes());
        result[12..16].copy_from_slice(&2u32.to_le_bytes());
        result.extend_from_slice(b"invented original binary bytes; never executed");
        result
    }
    fn pin() -> Pin {
        Pin {
            name: "invented-supplier".into(),
            bytes: bytes().len() as u64,
            digest: Sha256::digest(bytes()).into(),
            cpu: 0x0100_000c,
        }
    }
    fn backing() -> Arc<Lease> {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        Arc::new(registry.acquire_interactive_ingress(STARTUP_BYTES).unwrap())
    }
    #[test]
    fn compiled_pins_select_only_the_process_platform_and_architecture() {
        let arm = Pin::select("macos", "aarch64").unwrap();
        let intel = Pin::select("macos", "x86_64").unwrap();
        assert_eq!(arm.name, "codex-aarch64-apple-darwin");
        assert_eq!(arm.bytes, 241_555_024);
        assert_eq!(intel.name, "codex-x86_64-apple-darwin");
        assert_eq!(intel.bytes, 260_633_072);
        assert_ne!(arm.digest, intel.digest);
        assert_ne!(arm.cpu, intel.cpu);
        for (os, arch) in [
            ("windows", "aarch64"),
            ("linux", "x86_64"),
            ("macos", "powerpc"),
        ] {
            assert!(matches!(
                Pin::select(os, arch),
                Err(Error::UnsupportedPlatform)
            ));
        }
    }
    #[test]
    fn admission_precedes_resource_io() {
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let small = Arc::new(registry.acquire_interactive_ingress(1).unwrap());
        assert!(matches!(
            BundledSupplier::open(Path::new("/tmp/invented-unopened-home"), small),
            Err(Error::Admission)
        ));
    }
    #[test]
    fn original_stream_is_copied_exactly_before_executable_permissions() {
        let fixture = Fixture::new();
        fixture.put(&bytes());
        let output = fixture.0.join("runtime-copy");
        fixture.open().unwrap().copy_to(&output).unwrap();
        assert_eq!(fs::read(&output).unwrap(), bytes());
        assert_eq!(
            fs::metadata(output).unwrap().permissions().mode() & 0o777,
            0o500
        );
    }
    #[test]
    fn pathname_replacement_does_not_replace_the_original_open_stream() {
        let fixture = Fixture::new();
        fixture.put(&bytes());
        let source = fixture.open().unwrap();
        fs::rename(fixture.source(), fixture.0.join("original-owned-file")).unwrap();
        fs::write(fixture.source(), b"different path contents").unwrap();
        let output = fixture.0.join("runtime-copy");
        source.copy_to(&output).unwrap();
        assert_eq!(fs::read(output).unwrap(), bytes());
    }
    #[test]
    fn changed_or_growing_source_is_refused_and_partial_copy_is_removed() {
        for grow in [false, true] {
            let fixture = Fixture::new();
            fixture.put(&bytes());
            let source = fixture.open().unwrap();
            let mut changed = bytes();
            if grow {
                changed.push(1);
            } else {
                *changed.last_mut().unwrap() ^= 1;
            }
            fs::write(fixture.source(), changed).unwrap();
            let output = fixture.0.join("runtime-copy");
            assert_eq!(source.copy_to(&output), Err(Error::PinMismatch));
            assert!(!output.exists());
        }
    }
    #[test]
    fn existing_output_and_link_targets_are_never_replaced() {
        let fixture = Fixture::new();
        fixture.put(&bytes());
        let output = fixture.0.join("existing-output");
        fs::write(&output, b"unrelated").unwrap();
        assert_eq!(
            fixture.open().unwrap().copy_to(&output),
            Err(Error::CopyFailed)
        );
        assert_eq!(fs::read(&output).unwrap(), b"unrelated");
        let link = fixture.0.join("output-link");
        symlink(&output, &link).unwrap();
        assert_eq!(
            fixture.open().unwrap().copy_to(&link),
            Err(Error::CopyFailed)
        );
        assert!(link.is_symlink());
        assert_eq!(fs::read(output).unwrap(), b"unrelated");
    }
    #[test]
    fn size_architecture_and_executable_header_are_required_before_copy() {
        for kind in 0..4 {
            let fixture = Fixture::new();
            let mut body = bytes();
            match kind {
                0 => {
                    body.pop();
                }
                1 => body[4] ^= 1,
                2 => body[12] = 6,
                _ => body[0] = 0,
            }
            fixture.put(&body);
            assert!(matches!(fixture.open(), Err(Error::PinMismatch)));
        }
    }
    #[test]
    fn resource_links_directories_and_writable_inputs_are_refused() {
        let fixture = Fixture::new();
        let original = fixture.0.join("outside-original");
        fs::write(&original, bytes()).unwrap();
        symlink(&original, fixture.source()).unwrap();
        assert!(matches!(fixture.open(), Err(Error::UnavailableResource)));
        fs::remove_file(fixture.source()).unwrap();
        fs::create_dir(fixture.source()).unwrap();
        assert!(matches!(fixture.open(), Err(Error::InvalidResource)));
        fs::remove_dir(fixture.source()).unwrap();
        fixture.put(&bytes());
        fs::set_permissions(fixture.source(), fs::Permissions::from_mode(0o666)).unwrap();
        assert!(matches!(fixture.open(), Err(Error::InvalidResource)));
        fs::remove_file(fixture.source()).unwrap();
        fs::remove_dir(fixture.0.join("agent-supplier")).unwrap();
        let elsewhere = fixture.0.join("elsewhere");
        fs::create_dir(&elsewhere).unwrap();
        symlink(&elsewhere, fixture.0.join("agent-supplier")).unwrap();
        assert!(matches!(fixture.open(), Err(Error::InvalidResource)));
    }
    #[test]
    fn admission_backing_lives_through_the_owned_input_stream() {
        let fixture = Fixture::new();
        fixture.put(&bytes());
        let backing = backing();
        let retained = Arc::downgrade(&backing);
        let source = BundledSupplier::open_pin(&fixture.0, pin(), backing).unwrap();
        assert!(retained.upgrade().is_some());
        source.copy_to(&fixture.0.join("copy")).unwrap();
        assert!(retained.upgrade().is_none());
    }
    #[test]
    #[ignore = "manager-owned exact built-bundle resource proof; no supplier execution"]
    fn native_built_bundle_copies_the_selected_original_supplier() {
        assert_eq!(
            std::env::var("SWBPIPE_SUPPLIER_QUALIFY").as_deref(),
            Ok("1")
        );
        let resources = PathBuf::from(
            std::env::var_os("SWBPIPE_SUPPLIER_RESOURCES")
                .expect("selected app resource directory"),
        );
        let fixture = Fixture::new();
        let source = BundledSupplier::open(&resources, backing()).unwrap();
        let output = fixture.0.join("verified-runtime-copy");
        source.copy_to(&output).unwrap();
        let metadata = fs::metadata(output).unwrap();
        assert_eq!(
            metadata.len(),
            Pin::select(std::env::consts::OS, std::env::consts::ARCH)
                .unwrap()
                .bytes
        );
        assert_eq!(metadata.permissions().mode() & 0o777, 0o500);
        println!("SWBPIPE_SUPPLIER: original_bundle_bytes_verified_and_copied");
    }
}

pub(super) struct BundledSupplier {
    input: File,
    pin: Pin,
    _backing: Arc<Lease>,
}
impl BundledSupplier {
    /// `resources` comes from the native app's resource-directory resolver,
    /// never a model/webview/settings path. No home, PATH or cache fallback.
    pub(super) fn open(resources: &Path, backing: Arc<Lease>) -> Result<Self, Error> {
        if backing.bytes() < STARTUP_BYTES || backing.purpose() != LeasePurpose::Ingress {
            return Err(Error::Admission);
        }
        let pin = Pin::select(std::env::consts::OS, std::env::consts::ARCH)?;
        Self::open_pin(resources, pin, backing)
    }
    fn open_pin(resources: &Path, pin: Pin, backing: Arc<Lease>) -> Result<Self, Error> {
        let root = fs::canonicalize(resources).map_err(|_| Error::UnavailableResource)?;
        let directory = root.join("agent-supplier");
        // Reject an intermediate resource alias as well as final links. The
        // native caller owns the resource root; hostile parent mutation by the
        // same uid is outside this boundary's threat model.
        if fs::canonicalize(&directory).map_err(|_| Error::UnavailableResource)? != directory {
            return Err(Error::InvalidResource);
        }
        let mut input = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(directory.join(&pin.name))
            .map_err(|_| Error::UnavailableResource)?;
        let metadata = input.metadata().map_err(|_| Error::InvalidResource)?;
        if !metadata.is_file() || metadata.permissions().mode() & 0o022 != 0 {
            return Err(Error::InvalidResource);
        }
        if metadata.len() != pin.bytes {
            return Err(Error::PinMismatch);
        }
        let mut header = [0; 32];
        input
            .read_exact(&mut header)
            .map_err(|_| Error::PinMismatch)?;
        if header[..4] != [0xcf, 0xfa, 0xed, 0xfe]
            || u32::from_le_bytes(header[4..8].try_into().unwrap()) != pin.cpu
            || u32::from_le_bytes(header[12..16].try_into().unwrap()) != 2
        {
            return Err(Error::PinMismatch);
        }
        input
            .seek(SeekFrom::Start(0))
            .map_err(|_| Error::InvalidResource)?;
        Ok(Self {
            input,
            pin,
            _backing: backing,
        })
    }
    /// Destination is a fresh file in the launcher's owned private scope. Hash
    /// the actual copied stream, not an earlier pathname observation. No exec
    /// permission is granted until exact size/hash and flush succeed.
    pub(super) fn copy_to(mut self, destination: &Path) -> Result<(), Error> {
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(destination)
            .map_err(|_| Error::CopyFailed)?;
        let result = (|| {
            let mut hash = Sha256::new();
            let mut count = 0u64;
            let mut buffer = [0; 64 * 1024];
            loop {
                let n = self
                    .input
                    .read(&mut buffer)
                    .map_err(|_| Error::CopyFailed)?;
                if n == 0 {
                    break;
                }
                count = count.checked_add(n as u64).ok_or(Error::PinMismatch)?;
                if count > self.pin.bytes {
                    return Err(Error::PinMismatch);
                }
                hash.update(&buffer[..n]);
                output
                    .write_all(&buffer[..n])
                    .map_err(|_| Error::CopyFailed)?;
            }
            let digest: [u8; 32] = hash.finalize().into();
            if count != self.pin.bytes || digest != self.pin.digest {
                return Err(Error::PinMismatch);
            }
            output.sync_all().map_err(|_| Error::CopyFailed)?;
            output
                .set_permissions(fs::Permissions::from_mode(0o500))
                .map_err(|_| Error::CopyFailed)
        })();
        drop(output);
        if result.is_err() {
            let _ = fs::remove_file(destination);
        }
        result
    }
}
