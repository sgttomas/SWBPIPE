//! Only native provider-credential boundary. SDK-public Security APIs; no helper,
//! data-protection entitlement, extra access group,
//! synchronization, keychain-path input or store fallback. Native probes opt in.
use super::{
    provider_store::{CredentialKey, KeyError, Keychain},
    relay_policy::{Provider, Secret, SECRET_BYTES},
};
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use core_foundation::{
    array::CFArray,
    base::{CFType, CFTypeRef, TCFType},
    boolean::CFBoolean,
    data::CFData,
    dictionary::CFDictionary,
    string::{CFString, CFStringRef},
};
use security_framework::os::macos::keychain::SecKeychain;
use security_framework_sys::{
    item::{
        kSecAttrAccount, kSecAttrService, kSecAttrSynchronizable, kSecClass,
        kSecClassGenericPassword, kSecMatchLimit, kSecMatchSearchList, kSecReturnData,
        kSecUseAuthenticationUI, kSecUseKeychain, kSecValueData,
    },
    keychain::SecKeychainSetUserInteractionAllowed,
    keychain_item::{SecItemAdd, SecItemCopyMatching, SecItemDelete},
};
use std::{fmt, ptr};

const SERVICE: &str = "com.chirality.swbpipe.provider-api-key";
const NATIVE_BYTES: u64 = 1024 * 1024;
// These SDK-public constants are absent in security-framework-sys 2.17.0.
// They use the SDK-declared ABI; minimum-version contracts are recorded in Design.
extern "C" {
    static kSecUseAuthenticationUIFail: CFStringRef;
    static kSecAttrGeneric: CFStringRef;
    static kSecMatchLimitOne: CFStringRef;
}
/// SDK client-library policy, process-local in pinned Apple Globals::mUI.
/// Never restored: this SWBPIPE process must never enable Keychain prompting.
pub(super) fn disallow_ui() -> Result<(), KeyError> {
    status_result(unsafe { SecKeychainSetUserInteractionAllowed(0) })
}
pub struct NativeKeychain {
    _backing: Lease,
}
impl NativeKeychain {
    pub fn new(registry: &SolveJobRegistry) -> Result<Self, KeyError> {
        let backing = registry
            .acquire_interactive_ingress(NATIVE_BYTES)
            .map_err(|_| KeyError::Unavailable)?;
        Ok(Self { _backing: backing })
    }
    fn query(
        &self,
        key: &CredentialKey,
        create: bool,
    ) -> Result<Vec<(CFString, CFType)>, KeyError> {
        disallow_ui()?;
        let keychain = SecKeychain::default().map_err(|error| classify(error.code()))?;
        // SAFETY: each imported reference is an immutable SDK CFString constant;
        // owned CF wrappers retain it. All dictionary values are owned CF types.
        unsafe {
            let mut fields = vec![
                pair(
                    kSecClass,
                    CFString::wrap_under_get_rule(kSecClassGenericPassword).into_CFType(),
                ),
                pair(
                    kSecAttrService,
                    CFString::from_static_string(SERVICE).into_CFType(),
                ),
                pair(kSecAttrAccount, CFString::new(key.account()).into_CFType()),
                pair(
                    kSecAttrGeneric,
                    CFData::from_buffer(key.owner().as_bytes()).into_CFType(),
                ),
                pair(
                    kSecAttrSynchronizable,
                    CFBoolean::false_value().into_CFType(),
                ),
                pair(
                    kSecUseAuthenticationUI,
                    CFString::wrap_under_get_rule(kSecUseAuthenticationUIFail).into_CFType(),
                ),
            ];
            if create {
                fields.push(pair(kSecUseKeychain, keychain.as_CFType()));
            } else {
                fields.push(pair(
                    kSecMatchSearchList,
                    CFArray::from_CFTypes(&[keychain]).into_CFType(),
                ));
            }
            Ok(fields)
        }
    }
}
impl NativeKeychain {
    fn load_status(
        &mut self,
        key: &CredentialKey,
        endpoint: &str,
    ) -> Result<Secret, (KeyError, i32)> {
        let mut fields = self.query(key, false).map_err(|e| (e, 0))?;
        unsafe {
            fields.push(pair(kSecReturnData, CFBoolean::true_value().into_CFType()));
            fields.push(pair(
                kSecMatchLimit,
                CFString::wrap_under_get_rule(kSecMatchLimitOne).into_CFType(),
            ));
        }
        let query = CFDictionary::from_CFType_pairs(&fields);
        let mut output: CFTypeRef = ptr::null();
        let status = unsafe { SecItemCopyMatching(query.as_concrete_TypeRef(), &mut output) };
        // Result ownership exists only on success. Error output is unspecified
        // by the API and must not be treated as a CF object.
        status_result(status).map_err(|e| (e, status))?;
        if output.is_null() {
            return Err((KeyError::Invalid, -50));
        }
        let object = unsafe { CFType::wrap_under_create_rule(output) };
        secret_data(object, endpoint).map_err(|e| (e, -50))
    }
}
impl Keychain for NativeKeychain {
    fn create(
        &mut self,
        key: &CredentialKey,
        endpoint: &str,
        secret: &Secret,
    ) -> Result<(), KeyError> {
        let payload = credential_data(endpoint, secret)?;
        let mut fields = self.query(key, true)?;
        // SAFETY: static key, owned data, dictionary and data live through the
        // synchronous call. No result ownership is requested and no upsert exists.
        unsafe {
            fields.push(pair(
                kSecValueData,
                CFData::from_buffer(&payload).into_CFType(),
            ));
        }
        let query = CFDictionary::from_CFType_pairs(&fields);
        let status = unsafe { SecItemAdd(query.as_concrete_TypeRef(), ptr::null_mut()) };
        status_result(status)
    }
    fn load(&mut self, key: &CredentialKey, endpoint: &str) -> Result<Secret, KeyError> {
        self.load_status(key, endpoint).map_err(|(class, _)| class)
    }
    fn delete(&mut self, key: &CredentialKey) -> Result<(), KeyError> {
        let query = CFDictionary::from_CFType_pairs(&self.query(key, false)?);
        // Exact service/account/ownership marker, current default keychain only.
        // No password read and no unscoped service-wide deletion is possible.
        status_result(unsafe { SecItemDelete(query.as_concrete_TypeRef()) })
    }
}
impl fmt::Debug for NativeKeychain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NativeKeychain")
    }
}
const DATA_MAGIC: &[u8] = b"SWBPIPE-PROVIDER-1\0";
const ENDPOINT_BYTES: usize = 2048;
const DATA_BYTES: usize = DATA_MAGIC.len() + 2 + ENDPOINT_BYTES + SECRET_BYTES;
fn canonical(endpoint: &str) -> bool {
    endpoint.len() <= ENDPOINT_BYTES
        && Provider::new(endpoint)
            .map(|p| p.endpoint() == endpoint)
            .unwrap_or(false)
}
fn credential_data(endpoint: &str, secret: &Secret) -> Result<Vec<u8>, KeyError> {
    if !canonical(endpoint) {
        return Err(KeyError::Invalid);
    }
    let mut bytes =
        Vec::with_capacity(DATA_MAGIC.len() + 2 + endpoint.len() + secret.value().len());
    bytes.extend_from_slice(DATA_MAGIC);
    bytes.extend_from_slice(&(endpoint.len() as u16).to_be_bytes());
    bytes.extend_from_slice(endpoint.as_bytes());
    bytes.extend_from_slice(secret.value().as_bytes());
    Ok(bytes)
}
fn secret_data(object: CFType, expected_endpoint: &str) -> Result<Secret, KeyError> {
    if !canonical(expected_endpoint) {
        return Err(KeyError::Invalid);
    }
    let data = object.downcast_into::<CFData>().ok_or(KeyError::Invalid)?;
    if data.len() <= 0 || data.len() > DATA_BYTES as isize {
        return Err(KeyError::Invalid);
    }
    let bytes = data.bytes();
    let rest = bytes.strip_prefix(DATA_MAGIC).ok_or(KeyError::Invalid)?;
    if rest.len() < 2 {
        return Err(KeyError::Invalid);
    }
    let len = u16::from_be_bytes([rest[0], rest[1]]) as usize;
    if len > ENDPOINT_BYTES
        || rest.len() < 2 + len
        || &rest[2..2 + len] != expected_endpoint.as_bytes()
    {
        return Err(KeyError::Invalid);
    }
    let secret = &rest[2 + len..];
    if secret.is_empty() || secret.len() > SECRET_BYTES {
        return Err(KeyError::Invalid);
    }
    let value = std::str::from_utf8(secret)
        .map_err(|_| KeyError::Invalid)?
        .to_owned();
    Secret::new(value).map_err(|_| KeyError::Invalid)
}
unsafe fn pair(key: CFStringRef, value: CFType) -> (CFString, CFType) {
    (unsafe { CFString::wrap_under_get_rule(key) }, value)
}
fn status_result(code: i32) -> Result<(), KeyError> {
    if code == 0 {
        Ok(())
    } else {
        Err(classify(code))
    }
}
fn classify(code: i32) -> KeyError {
    match code {
        -25300 => KeyError::Missing,
        -25308 | -25293 => KeyError::Denied,
        -67050 | -67062 | -67061 | -34018 => KeyError::Invalid,
        -25291 | -25294 => KeyError::Unavailable,
        -25299 => KeyError::Duplicate,
        -50 => KeyError::Invalid,
        _ => KeyError::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn protected_native_payload_binds_the_recipient_before_releasing_the_secret() {
        let secret = Secret::new("invented-provider-value".into()).unwrap();
        let payload = credential_data("https://provider.example/v1", &secret).unwrap();
        assert!(matches!(
            secret_data(
                CFData::from_buffer(&payload).into_CFType(),
                "https://other.example/v1"
            ),
            Err(KeyError::Invalid)
        ));
        assert!(
            matches!(
                secret_data(
                    CFData::from_buffer(secret.value().as_bytes()).into_CFType(),
                    "https://provider.example/v1"
                ),
                Err(KeyError::Invalid)
            ),
            "unbound legacy values never bypass recipient binding"
        );
        assert_eq!(
            secret_data(
                CFData::from_buffer(&payload).into_CFType(),
                "https://provider.example/v1"
            )
            .unwrap()
            .value(),
            secret.value()
        );
    }
    #[test]
    fn native_results_are_typed_bounded_and_never_render_os_messages() {
        assert!(matches!(
            secret_data(
                CFString::from_static_string("not data").into_CFType(),
                "https://provider.example/v1"
            ),
            Err(KeyError::Invalid)
        ));
        for data in [
            vec![],
            vec![b'x'; SECRET_BYTES + 1],
            b"invalid\r\nheader".to_vec(),
        ] {
            assert!(matches!(
                secret_data(
                    CFData::from_buffer(&data).into_CFType(),
                    "https://provider.example/v1"
                ),
                Err(KeyError::Invalid)
            ));
        }
        let secret = secret_data(
            CFData::from_buffer(
                &credential_data(
                    "https://provider.example/v1",
                    &Secret::new("invented-provider-value".into()).unwrap(),
                )
                .unwrap(),
            )
            .into_CFType(),
            "https://provider.example/v1",
        )
        .unwrap();
        assert_eq!(secret.value(), "invented-provider-value");
        assert_eq!(classify(-25308), KeyError::Denied);
        assert_eq!(classify(-25293), KeyError::Denied);
        assert_eq!(classify(-67050), KeyError::Invalid);
        assert_eq!(classify(-34018), KeyError::Invalid);
        assert_eq!(classify(-25299), KeyError::Duplicate);
    }
}

#[cfg(test)]
mod qualification {
    use super::super::provider_store::{CredentialChange, Error as StoreError, Store};
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::{
        fs,
        io::Read,
        os::unix::process::ExitStatusExt,
        path::PathBuf,
        process::{Child, Command, Stdio},
        time::{Duration, Instant},
    };
    fn registry() -> SolveJobRegistry {
        SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
    }
    fn directory() -> PathBuf {
        assert_eq!(
            std::env::var("SWBPIPE_KEYCHAIN_QUALIFY").as_deref(),
            Ok("1")
        );
        let path = PathBuf::from(
            std::env::var_os("SWBPIPE_KEYCHAIN_FIXTURE_DIR").expect("private fixture directory"),
        );
        assert!(path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("swbpipe-keychain-fixture-"));
        path
    }
    fn nonce() -> String {
        let mut bytes = [0u8; 24];
        getrandom::fill(&mut bytes).unwrap();
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
    struct OwnedChild(Option<Child>);
    impl Drop for OwnedChild {
        fn drop(&mut self) {
            if let Some(mut child) = self.0.take() {
                let _ = child.kill();
                let _ = child.wait();
            }
        }
    }
    struct Recovery {
        directory: PathBuf,
        registry: SolveJobRegistry,
        clean: bool,
    }
    impl Recovery {
        fn finish(&mut self) -> bool {
            let Ok(keys) = NativeKeychain::new(&self.registry) else {
                return false;
            };
            let Ok(mut store) = Store::open(&self.directory, keys, self.registry.clone()) else {
                return false;
            };
            let Ok(snapshot) = store.snapshot() else {
                return false;
            };
            if let Some(profile) = snapshot.provider {
                if store.clear(&profile.revision).is_err() {
                    return false;
                }
            }
            if store.recover().is_err() {
                return false;
            }
            let Ok(snapshot) = store.snapshot() else {
                return false;
            };
            self.clean = snapshot.provider.is_none() && snapshot.cleanup_pending == Some(0);
            self.clean
        }
    }
    impl Drop for Recovery {
        fn drop(&mut self) {
            if !self.clean {
                let _ = self.finish();
            }
        }
    }
    #[test]
    #[ignore = "manager-owned native synthetic Keychain/signature probe after credentials review"]
    fn native_resign_reentry_and_owned_cleanup() {
        qualify(false);
    }
    #[test]
    #[ignore = "manager-owned renamed-app denial and durable stale-item recovery"]
    fn native_renamed_reentry_retains_stale_and_uses_fresh_key() {
        qualify(true);
    }
    fn qualify(renamed: bool) {
        let directory = directory();
        let registry = registry();
        let mut recovery = Recovery {
            directory: directory.clone(),
            registry: registry.clone(),
            clean: false,
        };
        let mut store = Store::open(
            &directory,
            NativeKeychain::new(&registry).unwrap(),
            registry.clone(),
        )
        .unwrap();
        let before = store.snapshot().unwrap();
        assert!(
            before.provider.is_none() && before.cleanup_pending == Some(0),
            "fixture was not empty"
        );
        let value = format!("fixture-native-{}", nonce());
        let saved = store
            .save(
                None,
                "https://provider.example/v1",
                "fixture-only",
                CredentialChange::Replace(Secret::new(value.clone()).unwrap()),
            )
            .unwrap();
        let revision = saved.provider.unwrap().revision;
        assert_eq!(
            store.load(&revision).unwrap().secret.unwrap().value(),
            value
        );
        // Never rewrite/re-sign an already-executed Mach-O inode. This is a fresh
        // private copy, deliberately a different code identifier from this owner.
        let original = std::env::current_exe().unwrap();
        // Apple's legacy safe-delete checks the executable basename as well as
        // item ownership. Preserve it to model an update, not a renamed app.
        let executable = if renamed {
            directory.join("resigned-fixture")
        } else {
            directory.join(original.file_name().unwrap())
        };
        assert!(!executable.exists());
        fs::copy(original, &executable).unwrap();
        let output = Command::new("/usr/bin/codesign")
            .args(["--force", "--sign", "-", "--identifier"])
            .arg(format!(
                "com.chirality.swbpipe.keychain-fixture.{}",
                nonce()
            ))
            .arg(&executable)
            .env_clear()
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .output()
            .unwrap();
        assert!(output.status.success(), "fixture signing failed");
        let child = Command::new(&executable)
            .args([
                "--exact",
                "embedded_agent::keychain_native::qualification::resigned_fixture_child",
                "--ignored",
                "--nocapture",
                "--test-threads=1",
            ])
            .env_clear()
            .env("SWBPIPE_KEYCHAIN_QUALIFY", "1")
            .env("SWBPIPE_KEYCHAIN_FIXTURE_DIR", &directory)
            .env("SWBPIPE_KEYCHAIN_RENAMED", if renamed { "1" } else { "0" })
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut child = OwnedChild(Some(child));
        let deadline = Instant::now() + Duration::from_secs(20);
        let status = loop {
            match child.0.as_mut().unwrap().try_wait() {
                Ok(Some(status)) => break status,
                Ok(None) => {
                    assert!(
                        Instant::now() < deadline,
                        "no-UI signature fixture did not return"
                    );
                    std::thread::sleep(Duration::from_millis(10));
                }
                Err(_) => panic!("fixture observation failed"),
            }
        };
        let mut records = String::new();
        child
            .0
            .as_mut()
            .unwrap()
            .stdout
            .take()
            .unwrap()
            .take(8192)
            .read_to_string(&mut records)
            .unwrap();
        child.0.take();
        println!(
            "SWBPIPE_KEYCHAIN: child_exit={:?},signal={:?}",
            status.code(),
            status.signal()
        );
        for line in records.lines() {
            if let Some((_, record)) = line.split_once("SWBPIPE_KEYCHAIN: ") {
                // Qualification emits only fixed stages, typed errors and OSStatus.
                println!("SWBPIPE_KEYCHAIN: {record}");
            }
        }
        assert!(
            status.success(),
            "signature fixture did not satisfy exact no-UI/reentry/cleanup predicate"
        );
        assert!(records
            .lines()
            .any(|line| line == "SWBPIPE_KEYCHAIN: denied,reentry,cleanup_standing_recorded"));
        drop(store);
        if renamed {
            // Two executable basenames created the two synthetic items. Each
            // original identity attempts only the journal's exact owned handles;
            // partial deletion can be completed idempotently by the other.
            let _ = recovery.finish();
            let mut cleaner = OwnedChild(Some(
                Command::new(&executable)
                    .args([
                        "--exact",
                        "embedded_agent::keychain_native::qualification::recover_owned_fixture",
                        "--ignored",
                        "--nocapture",
                        "--test-threads=1",
                    ])
                    .env_clear()
                    .env("SWBPIPE_KEYCHAIN_QUALIFY", "1")
                    .env("SWBPIPE_KEYCHAIN_FIXTURE_DIR", &directory)
                    .stdin(Stdio::null())
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .spawn()
                    .unwrap(),
            ));
            let deadline = Instant::now() + Duration::from_secs(20);
            loop {
                match cleaner.0.as_mut().unwrap().try_wait() {
                    Ok(Some(_)) => break,
                    Ok(None) => {
                        assert!(Instant::now() < deadline, "owned recovery child timed out");
                        std::thread::sleep(Duration::from_millis(10));
                    }
                    Err(_) => panic!("owned recovery observation failed"),
                }
            }
            cleaner.0.take();
        }
        assert!(
            recovery.finish(),
            "fixture cleanup retained; inspect its owned journal"
        );
        println!("SWBPIPE_KEYCHAIN: owner_roundtrip,resign_reentry,owned_cleanup");
    }
    #[test]
    #[ignore = "manager-owned recovery of an interrupted synthetic Keychain fixture"]
    fn recover_owned_fixture() {
        let mut recovery = Recovery {
            directory: directory(),
            registry: registry(),
            clean: false,
        };
        assert!(recovery.finish(), "owned fixture journal retained");
        println!("SWBPIPE_KEYCHAIN: owned_recovery_complete");
    }
    #[test]
    #[ignore = "invoked only by the owned re-signed Keychain fixture"]
    fn resigned_fixture_child() {
        let directory = directory();
        let registry = registry();
        let mut store = Store::open(
            &directory,
            NativeKeychain::new(&registry).unwrap(),
            registry.clone(),
        )
        .unwrap();
        let profile = store.snapshot().unwrap().provider.unwrap();
        let started = Instant::now();
        let result = store.load(&profile.revision);
        assert!(
            started.elapsed() < Duration::from_secs(2),
            "no-prompt load exceeded short bound"
        );
        println!(
            "SWBPIPE_KEYCHAIN: first_load={:?}",
            result.as_ref().map(|_| ()).map_err(|e| *e)
        );
        let mapped_interaction = matches!(
            result,
            Err(StoreError::ReenterKey {
                reason: KeyError::Denied
            })
        );
        assert!(
            matches!(result, Err(StoreError::ReenterKey { .. })),
            "re-signed item unexpectedly readable"
        );
        // Exact OSStatus is inspected only in this fixture, never returned as
        // an arbitrary native diagnostic or a credential-bearing error string.
        let db = rusqlite::Connection::open_with_flags(
            directory.join("provider-settings.sqlite3"),
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .unwrap();
        let (account, owner) = db
            .query_row(
                "SELECT account,owner FROM credentials WHERE phase='active'",
                [],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .unwrap();
        drop(db);
        let key = CredentialKey::checked(account, owner).unwrap();
        let mut native = NativeKeychain::new(&registry).unwrap();
        let result = native.load_status(&key, &profile.endpoint);
        println!(
            "SWBPIPE_KEYCHAIN: exact_status={:?}",
            result.as_ref().map(|_| ()).map_err(|e| *e)
        );
        // Agent 0 disposition: legacy Globals.cpp:68–71 selects nullCred
        // without UI and can return AuthFailed; the modern path returns
        // InteractionNotAllowed. Only these two raw statuses qualify.
        let exact_interaction = matches!(result, Err((KeyError::Denied, -25308 | -25293)));
        assert!(
            mapped_interaction && exact_interaction,
            "required no-prompt denial was not established"
        );
        let fresh_value = format!("fixture-reentered-{}", nonce());
        let saved = store
            .save(
                Some(&profile.revision),
                &profile.endpoint,
                &profile.model,
                CredentialChange::Replace(Secret::new(fresh_value.clone()).unwrap()),
            )
            .unwrap();
        println!(
            "SWBPIPE_KEYCHAIN: reentry_cleanup={:?},error={:?}",
            saved.cleanup_pending, saved.cleanup_error
        );
        let renamed = std::env::var("SWBPIPE_KEYCHAIN_RENAMED").as_deref() == Ok("1");
        if renamed {
            assert!(saved.cleanup_pending.is_none());
            assert_eq!(saved.cleanup_error, Some(KeyError::Unknown));
            let query = CFDictionary::from_CFType_pairs(&native.query(&key, false).unwrap());
            let status = unsafe { SecItemDelete(query.as_concrete_TypeRef()) };
            assert_eq!(
                status, -25244,
                "renamed fixture must retain owner-edit refusal"
            );
            println!("SWBPIPE_KEYCHAIN: stale_delete_status=-25244,owned_handle_retained");
        } else {
            assert_eq!(
                saved.cleanup_pending,
                Some(0),
                "stale owned credential was not removed"
            );
            assert!(saved.cleanup_error.is_none());
        }
        let revision = saved.provider.unwrap().revision;
        let loaded = store.load(&revision).unwrap();
        assert!(
            loaded.secret.as_ref().unwrap().value() == fresh_value,
            "fresh key did not match its saved value"
        );
        drop(loaded);
        println!("SWBPIPE_KEYCHAIN: fresh_committed_key_works");
        let cleared = store.clear(&revision).unwrap();
        assert!(cleared.provider.is_none());
        if renamed {
            assert!(cleared.cleanup_pending.is_none());
        } else {
            assert_eq!(cleared.cleanup_pending, Some(0));
        }
        assert!(
            mapped_interaction && exact_interaction,
            "required interaction-not-allowed case was not established"
        );
        println!("SWBPIPE_KEYCHAIN: denied,reentry,cleanup_standing_recorded");
    }
}
