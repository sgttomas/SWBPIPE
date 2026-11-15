//! Only native provider-credential boundary. SDK-public Security APIs; no helper,
//! data-protection entitlement, extra access group,
//! synchronization, keychain-path input or store fallback. Native probes opt in.
//!
//! Two backends, chosen at compile time and by nothing else. Without the
//! `native-smoke` feature (every product build) `live` is the backend, with
//! the Security item calls below. With it (the scripted-run build, DEL-07-11)
//! `live` is not compiled and `smoke` takes its place: every
//! application-owned Keychain item operation (load, create, delete) is refused
//! with the named `KeyError::SmokeConfigurationDenied` before any Security
//! framework call, and no code in that build can reach `SecItemCopyMatching`,
//! `SecItemAdd`, `SecItemDelete` or the default-keychain lookup. No
//! environment variable lifts the refusal; `SWBPIPE_NATIVE_SMOKE_PROVIDER_KEY`
//! selects an in-memory key read from stdin and takes no Keychain route
//! (`native_smoke::take_provider_secret`, used by `live_host`'s Start only for
//! a provider saved with no key). `disallow_ui` stays in both builds: the
//! relay's TLS set-up calls it (`http_relay::tls_config`), and it sets a
//! process policy; it is not an item operation or a keychain lookup.
//! `smoke_boundary` checks this structure from source.
use super::provider_store::KeyError;
use security_framework_sys::keychain::SecKeychainSetUserInteractionAllowed;

#[cfg(not(feature = "native-smoke"))]
pub use live::NativeKeychain;
#[cfg(feature = "native-smoke")]
pub use smoke::NativeKeychain;

const NATIVE_BYTES: u64 = 1024 * 1024;
/// SDK client-library policy, process-local in pinned Apple Globals::mUI.
/// Never restored: this SWBPIPE process must never enable Keychain prompting.
pub(super) fn disallow_ui() -> Result<(), KeyError> {
    status_result(unsafe { SecKeychainSetUserInteractionAllowed(0) })
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

/// The Security-backed backend. Not compiled into the `native-smoke` build.
#[cfg(not(feature = "native-smoke"))]
mod live {
    use super::{classify, disallow_ui, status_result, NATIVE_BYTES};
    use crate::embedded_agent::{
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
        keychain_item::{SecItemAdd, SecItemCopyMatching, SecItemDelete},
    };
    use std::{fmt, ptr};

    const SERVICE: &str = "com.chirality.swbpipe.provider-api-key";
    // These SDK-public constants are absent in security-framework-sys 2.17.0.
    // They use the SDK-declared ABI; minimum-version contracts are recorded in Design.
    extern "C" {
        static kSecUseAuthenticationUIFail: CFStringRef;
        static kSecAttrGeneric: CFStringRef;
        static kSecMatchLimitOne: CFStringRef;
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
        use crate::embedded_agent::provider_store::{CredentialChange, Error as StoreError, Store};
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
                    32768,
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
                    "embedded_agent::keychain_native::live::qualification::resigned_fixture_child",
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
                            "embedded_agent::keychain_native::live::qualification::recover_owned_fixture",
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
                    32768,
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
}

/// The scripted-run backend (`native-smoke`, DEL-07-11): no Security import,
/// no item call and no keychain lookup. Every item operation is the named
/// smoke-configuration denial, and `Store::save` asks `Keychain::item_refusal`
/// before it journals a key, so a refused key leaves no pending cleanup.
#[cfg(feature = "native-smoke")]
mod smoke {
    use super::NATIVE_BYTES;
    use crate::embedded_agent::{
        provider_store::{CredentialKey, KeyError, Keychain},
        relay_policy::Secret,
    };
    use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
    use std::fmt;

    pub struct NativeKeychain {
        _backing: Lease,
    }
    impl NativeKeychain {
        /// The same admission as the live backend, so a settings action
        /// reaches the store as it does in a product build.
        pub fn new(registry: &SolveJobRegistry) -> Result<Self, KeyError> {
            let backing = registry
                .acquire_interactive_ingress(NATIVE_BYTES)
                .map_err(|_| KeyError::Unavailable)?;
            Ok(Self { _backing: backing })
        }
    }
    impl Keychain for NativeKeychain {
        fn create(
            &mut self,
            _key: &CredentialKey,
            _endpoint: &str,
            _secret: &Secret,
        ) -> Result<(), KeyError> {
            Err(KeyError::SmokeConfigurationDenied)
        }
        fn load(&mut self, _key: &CredentialKey, _endpoint: &str) -> Result<Secret, KeyError> {
            Err(KeyError::SmokeConfigurationDenied)
        }
        fn delete(&mut self, _key: &CredentialKey) -> Result<(), KeyError> {
            Err(KeyError::SmokeConfigurationDenied)
        }
        fn item_refusal(&self) -> Option<KeyError> {
            Some(KeyError::SmokeConfigurationDenied)
        }
    }
    impl fmt::Debug for NativeKeychain {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("NativeKeychain")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::embedded_agent::provider_store::{
            CredentialChange, Error as StoreError, Store,
        };
        use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
        use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf};

        const DENIED: KeyError = KeyError::SmokeConfigurationDenied;
        fn registry() -> SolveJobRegistry {
            SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()))
        }
        fn nonce() -> String {
            let mut bytes = [0u8; 24];
            getrandom::fill(&mut bytes).unwrap();
            bytes.iter().map(|b| format!("{b:02x}")).collect()
        }
        /// A private provider directory of the store's required form, removed on drop.
        struct Fixture(PathBuf);
        impl Fixture {
            fn new() -> Self {
                let path =
                    PathBuf::from("/private/tmp").join(format!("swbpipe-smoke-keys-{}", nonce()));
                fs::create_dir(&path).unwrap();
                fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
                Self(path)
            }
        }
        impl Drop for Fixture {
            fn drop(&mut self) {
                let _ = fs::remove_dir_all(&self.0);
            }
        }

        /// The backend this build compiles is the denial: each item operation
        /// returns the named kind, which no Keychain status maps to and which
        /// serializes under its own name. The Security calls are not reached
        /// because this build contains none (`smoke_boundary`).
        #[test]
        fn every_item_operation_is_the_named_smoke_denial() {
            let registry = registry();
            let mut keys = NativeKeychain::new(&registry).unwrap();
            let key = CredentialKey::checked(nonce(), nonce()).unwrap();
            let secret = Secret::new("invented-provider-value".into()).unwrap();
            assert_eq!(
                keys.create(&key, "https://provider.example/v1", &secret),
                Err(DENIED)
            );
            assert!(matches!(
                keys.load(&key, "https://provider.example/v1"),
                Err(DENIED)
            ));
            assert_eq!(keys.delete(&key), Err(DENIED));
            assert_eq!(keys.item_refusal(), Some(DENIED));
            assert_eq!(
                serde_json::to_value(DENIED).unwrap(),
                serde_json::json!("smoke_configuration_denied")
            );
            for code in [-25300, -25308, -25293, -25291, -25294, -25299, -50, -34018, 1, -1] {
                assert_ne!(super::super::classify(code), DENIED, "status {code}");
            }
        }

        /// Through the provider store, as the host's settings actions and
        /// Start reach it: a key is refused with the named denial before any
        /// pending intent is journaled; a provider with no key saves, loads
        /// (the stdin key's route) and clears; a saved credential seeded into
        /// the store, which this build cannot create, is refused at Start and
        /// at cleanup with the same denial, never as a locked, missing or
        /// denied key.
        #[test]
        fn the_store_on_the_smoke_backend_refuses_keys_by_name_and_runs_without_one() {
            let fixture = Fixture::new();
            let registry = registry();
            let mut store = Store::open(
                &fixture.0,
                NativeKeychain::new(&registry).unwrap(),
                registry.clone(),
            )
            .unwrap();
            let endpoint = "https://provider.example/v1";
            let replace = || {
                CredentialChange::Replace(Secret::new("invented-provider-value".into()).unwrap())
            };
            let refused = store
                .save(None, endpoint, "fixture-model", 32768, replace())
                .unwrap_err();
            assert_eq!(refused, StoreError::Keychain(DENIED));
            assert_eq!(
                serde_json::to_value(refused).unwrap(),
                serde_json::json!({"keychain": "smoke_configuration_denied"})
            );
            let message = refused.user_message();
            assert!(message.contains("test build"), "{message}");
            for other in [
                StoreError::Keychain(KeyError::Unavailable),
                StoreError::Keychain(KeyError::Denied),
                StoreError::ReenterKey { reason: KeyError::Missing },
                StoreError::ReenterKey { reason: KeyError::Denied },
                StoreError::ReenterKey { reason: KeyError::InteractionRequired },
                StoreError::RecoveryRequired,
            ] {
                assert_ne!(other.user_message(), message);
            }
            let empty = store.snapshot().unwrap();
            assert!(empty.provider.is_none());
            assert_eq!(empty.cleanup_pending, Some(0), "a refused key journals nothing");

            let saved = store
                .save(None, endpoint, "fixture-model", 32768, CredentialChange::NoKey)
                .unwrap();
            assert_eq!(saved.cleanup_pending, Some(0));
            assert!(saved.cleanup_error.is_none());
            let revision = saved.provider.unwrap().revision;
            assert!(store.load(&revision).unwrap().secret.is_none());
            assert_eq!(store.recover().unwrap().cleanup_pending, Some(0));

            // A store this build did not write: an active and a retired
            // credential row, as a product build would leave them.
            let db = rusqlite::Connection::open(fixture.0.join("provider-settings.sqlite3")).unwrap();
            let (account, retired) = (nonce(), nonce());
            db.execute(
                "INSERT INTO credentials(account,owner,endpoint,phase) VALUES(?1,?2,?3,'active')",
                rusqlite::params![account, nonce(), endpoint],
            )
            .unwrap();
            db.execute(
                "UPDATE provider SET account=?1",
                rusqlite::params![account],
            )
            .unwrap();
            db.execute(
                "INSERT INTO credentials(account,owner,endpoint,phase) VALUES(?1,?2,?3,'retired')",
                rusqlite::params![retired, nonce(), endpoint],
            )
            .unwrap();
            drop(db);
            assert_eq!(
                store.load(&revision).unwrap_err(),
                StoreError::Keychain(DENIED)
            );
            assert_eq!(store.recover().unwrap_err(), StoreError::Keychain(DENIED));
            let cleared = store.clear(&revision).unwrap();
            assert!(cleared.provider.is_none());
            assert_eq!(cleared.cleanup_error, Some(DENIED));
        }
    }
}

/// Source check of the backend split (threat model: an honest edit that moves
/// a Keychain item call, the default-keychain lookup or a Security crate path
/// out of the gated live backend, drops or changes that gate, or adds such a
/// call elsewhere in this crate; not deliberate obfuscation, against which the
/// user-release reach scan's pinned foreign blocks and refused linkage
/// attributes and dynamic lookup stand). Runs in every `cargo test` of this
/// crate, with or without `native-smoke`.
#[cfg(test)]
mod smoke_boundary {
    use proc_macro2::{TokenStream, TokenTree};
    use std::path::{Path, PathBuf};

    /// The item operations and the default-keychain lookup, by the names this
    /// crate could reach them through, and the Security crates themselves.
    const LIVE_ONLY: [&str; 10] = [
        "SecItemCopyMatching",
        "SecItemAdd",
        "SecItemDelete",
        "SecItemUpdate",
        "SecKeychain",
        "SecKeychainCopyDefault",
        "kSecUseKeychain",
        "kSecMatchSearchList",
        "security_framework",
        "extern",
    ];
    /// The one Security name outside the live backend: the relay's no-UI policy.
    const POLICY_IMPORT: &str =
        "use security_framework_sys::keychain::SecKeychainSetUserInteractionAllowed;";

    fn idents(tokens: TokenStream, out: &mut Vec<String>) {
        for token in tokens {
            match token {
                TokenTree::Ident(ident) => out.push(ident.to_string()),
                TokenTree::Group(group) => idents(group.stream(), out),
                _ => {}
            }
        }
    }
    fn named(tokens: TokenStream, names: &[&str]) -> Vec<String> {
        let mut found = Vec::new();
        idents(tokens, &mut found);
        found.retain(|ident| names.contains(&ident.as_str()));
        found
    }
    fn cfg_is(attrs: &[&syn::Attribute], expected: &str) -> bool {
        attrs.len() == 1
            && attrs[0].path().is_ident("cfg")
            && matches!(&attrs[0].meta, syn::Meta::List(list) if list.tokens.to_string() == expected)
    }
    fn non_doc(attrs: &[syn::Attribute]) -> Vec<&syn::Attribute> {
        attrs.iter().filter(|a| !a.path().is_ident("doc")).collect()
    }
    fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                sources(&path, out);
            } else if path.extension().is_some_and(|ext| ext == "rs") {
                out.push(path);
            }
        }
    }

    const NOT_SMOKE: &str = "not (feature = \"native-smoke\")";
    const SMOKE: &str = "feature = \"native-smoke\"";
    /// `pub use <backend>::NativeKeychain;`, by backend name.
    fn reexport(item: &syn::ItemUse) -> Option<String> {
        let syn::UseTree::Path(path) = &item.tree else { return None };
        let syn::UseTree::Name(name) = &*path.tree else { return None };
        (matches!(item.vis, syn::Visibility::Public(_)) && name.ident == "NativeKeychain")
            .then(|| path.ident.to_string())
    }

    /// Findings in this module's own source text; empty when the split holds.
    pub(super) fn findings(source: &str) -> Vec<String> {
        let file = syn::parse_file(source).expect("keychain_native.rs parses");
        let mut problems = Vec::new();
        let mut live_lines = Vec::new();
        let (mut smoke, mut live_use, mut smoke_use) = (0, 0, 0);
        for item in &file.items {
            match item {
                syn::Item::Mod(m) if m.ident == "live" => {
                    if !cfg_is(&non_doc(&m.attrs), NOT_SMOKE) {
                        problems.push("mod live is not gated exactly #[cfg(not(feature = \"native-smoke\"))]".into());
                    }
                    match &m.content {
                        Some((brace, _)) => live_lines
                            .push(m.mod_token.span.start().line..=brace.span.close().end().line),
                        None => problems.push("mod live is not inline".into()),
                    }
                }
                syn::Item::Mod(m) if m.ident == "smoke" => {
                    smoke += 1;
                    if !cfg_is(&non_doc(&m.attrs), SMOKE) {
                        problems.push("mod smoke is not gated exactly #[cfg(feature = \"native-smoke\")]".into());
                    }
                }
                syn::Item::Use(u) => match reexport(u).as_deref() {
                    Some("live") => {
                        live_use += 1;
                        if !cfg_is(&non_doc(&u.attrs), NOT_SMOKE) {
                            problems.push("the live re-export is not gated to builds without native-smoke".into());
                        }
                    }
                    Some("smoke") => {
                        smoke_use += 1;
                        if !cfg_is(&non_doc(&u.attrs), SMOKE) {
                            problems.push("the smoke re-export is not gated to the native-smoke build".into());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        if (live_lines.len(), smoke, live_use, smoke_use) != (1, 1, 1, 1) {
            problems.push(format!(
                "expected one live and one smoke backend and one re-export of each, found {}, {smoke}, {live_use}, {smoke_use}",
                live_lines.len()
            ));
        }
        // Everything outside the live backend, but for the one policy import.
        let mut policy = 0;
        let rest: Vec<&str> = source
            .lines()
            .enumerate()
            .filter(|(index, _)| !live_lines.iter().any(|lines| lines.contains(&(index + 1))))
            .map(|(_, line)| line)
            .filter(|line| {
                let kept = line.trim() != POLICY_IMPORT;
                policy += usize::from(!kept);
                kept
            })
            .collect();
        if policy != 1 {
            problems.push(format!("expected the one no-UI policy import, found {policy}"));
        }
        let tokens: TokenStream = rest.join("\n").parse().expect("the remainder tokenizes");
        let mut names = LIVE_ONLY.to_vec();
        names.push("security_framework_sys");
        for name in named(tokens, &names) {
            problems.push(format!("{name} outside the gated live backend"));
        }
        problems
    }

    #[test]
    fn the_smoke_build_holds_no_keychain_item_call_or_default_lookup() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let own = root.join("embedded_agent/keychain_native.rs");
        let problems = findings(&std::fs::read_to_string(&own).unwrap());
        assert!(problems.is_empty(), "keychain_native.rs:\n{}", problems.join("\n"));
        let mut files = Vec::new();
        sources(&root, &mut files);
        assert!(files.len() > 20, "the check reads the crate's sources");
        let mut elsewhere = Vec::new();
        for file in files.iter().filter(|f| **f != own) {
            let text = std::fs::read_to_string(file).unwrap();
            let tokens: TokenStream = text.parse().unwrap_or_else(|e| panic!("{}: {e:?}", file.display()));
            let mut names: Vec<&str> = LIVE_ONLY.to_vec();
            names.retain(|n| *n != "extern");
            names.extend(["security_framework_sys", "SecKeychainSetUserInteractionAllowed"]);
            for name in named(tokens, &names) {
                elsewhere.push(format!("{}: {name}", file.strip_prefix(&root).unwrap().display()));
            }
        }
        assert!(elsewhere.is_empty(), "Security names outside keychain_native.rs:\n{}", elsewhere.join("\n"));
    }

    #[test]
    fn the_check_refuses_an_ungated_live_backend_and_a_stray_item_call() {
        let source = std::fs::read_to_string(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("src/embedded_agent/keychain_native.rs"),
        )
        .unwrap();
        let gate = "#[cfg(not(feature = \"native-smoke\"))]\nmod live {";
        assert!(source.contains(gate));
        for mutated in [
            source.replace(gate, "mod live {"),
            source.replace(gate, "#[cfg(not(test))]\nmod live {"),
            source.replace(
                "#[cfg(not(feature = \"native-smoke\"))]\npub use live::NativeKeychain;",
                "pub use live::NativeKeychain;",
            ),
            source.replace(
                "pub(super) fn disallow_ui()",
                "fn stray() { let _ = SecItemDelete; }\npub(super) fn disallow_ui()",
            ),
            source.replace(
                "    pub struct NativeKeychain {\n        _backing: Lease,\n    }\n    impl NativeKeychain {\n        /// The same",
                "    pub struct NativeKeychain {\n        _backing: Lease,\n    }\n    fn lookup() { let _ = security_framework::os::macos::keychain::SecKeychain::default(); }\n    impl NativeKeychain {\n        /// The same",
            ),
        ] {
            assert_ne!(mutated, source, "the mutation applies");
            assert!(!findings(&mutated).is_empty(), "a mutation went unrefused");
        }
    }
}
