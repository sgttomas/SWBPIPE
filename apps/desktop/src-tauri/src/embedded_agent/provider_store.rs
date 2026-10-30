//! Explicit, synchronous provider settings/credential core. The future command
//! owner must run this on admitted blocking custody, never a UI/protocol poll.
//! No module startup, command, helper, credential file, fallback or provider I/O.
use super::relay_policy::{Provider, Secret};
use crate::{memory_pool::Lease, solve_jobs::SolveJobRegistry};
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde::Serialize;
use std::{
    fmt,
    fs::{self, OpenOptions},
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    path::{Path, PathBuf},
};

pub const STATE_BYTES: u64 = 8 * 1024 * 1024;
pub const OPERATION_BYTES: u64 = 16 * 1024 * 1024;
const MAX_ACCOUNTS: i64 = 16;
const MAX_DB_BYTES: u64 = 2 * 1024 * 1024;
const APP_ID: i64 = 0x53574241;
const DB_NAME: &str = "provider-settings.sqlite3";
const CREDENTIALS_SQL:&str="CREATE TABLE credentials(account TEXT PRIMARY KEY,owner TEXT NOT NULL,endpoint TEXT NOT NULL,phase TEXT NOT NULL CHECK(phase IN('pending','active','retired')))";
const PROVIDER_SQL:&str="CREATE TABLE provider(singleton INTEGER PRIMARY KEY CHECK(singleton=1),revision TEXT NOT NULL,endpoint TEXT NOT NULL,model TEXT NOT NULL,account TEXT REFERENCES credentials(account))";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Error {
    Admission,
    InvalidInput,
    Unavailable,
    Corrupt,
    Busy,
    Stale,
    NotConfigured,
    NewCredentialRequired,
    RecoveryRequired,
    Keychain(KeyError),
    ReenterKey { reason: KeyError },
}
impl Error {
    pub fn user_message(self) -> &'static str {
        match self {
            Self::ReenterKey {
                reason: KeyError::InteractionRequired,
            } => "Unlock your Keychain and re-enter your API key.",
            Self::ReenterKey { .. } => {
                "This build cannot use the saved key. Re-enter your API key."
            }
            Self::Keychain(_) => {
                "Keychain access is unavailable. Restore access and re-enter your API key."
            }
            Self::RecoveryRequired => {
                "Previous credential cleanup is pending. Restore Keychain access and retry cleanup."
            }
            Self::Stale => "Provider settings changed. Read them again before saving.",
            Self::Busy => "Another provider settings operation is running.",
            _ => "Provider configuration is unavailable.",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KeyError {
    Missing,
    InteractionRequired,
    Denied,
    Unavailable,
    Duplicate,
    Invalid,
    Unknown,
}

/// The fixed service lives in the native backend. Neither service, account nor
/// ownership marker is accepted from a webview/provider request.
pub trait Keychain {
    fn create(
        &mut self,
        key: &CredentialKey,
        endpoint: &str,
        secret: &Secret,
    ) -> Result<(), KeyError>;
    fn load(&mut self, key: &CredentialKey, endpoint: &str) -> Result<Secret, KeyError>;
    fn delete(&mut self, key: &CredentialKey) -> Result<(), KeyError>;
}
pub struct CredentialKey {
    account: String,
    owner: String,
}
impl CredentialKey {
    pub(super) fn fresh() -> Result<Self, Error> {
        Ok(Self {
            account: nonce()?,
            owner: nonce()?,
        })
    }
    pub(super) fn checked(account: String, owner: String) -> Result<Self, Error> {
        if !identifier(&account) || !identifier(&owner) {
            return Err(Error::Corrupt);
        }
        Ok(Self { account, owner })
    }
    pub(super) fn account(&self) -> &str {
        &self.account
    }
    pub(super) fn owner(&self) -> &str {
        &self.owner
    }
}
impl fmt::Debug for CredentialKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CredentialKey")
    }
}
/// Rust-only input. It cannot be serialized/logged as a generic argument value.
pub enum CredentialChange {
    Keep,
    NoKey,
    Replace(Secret),
}
impl fmt::Debug for CredentialChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CredentialChange")
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialMode {
    None,
    Keychain,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub revision: String,
    pub endpoint: String,
    pub model: String,
    pub credential_mode: CredentialMode,
}
#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub provider: Option<Metadata>,
    pub cleanup_pending: Option<usize>,
    pub cleanup_error: Option<KeyError>,
}
struct Stored {
    metadata: Metadata,
    key: Option<CredentialKey>,
}
/// Host-only launch snapshot. It is never a settings/command response.
pub struct LoadedProvider {
    pub revision: String,
    pub model: String,
    pub provider: Provider,
    pub secret: Option<Secret>,
    _backing: Lease,
}
impl fmt::Debug for LoadedProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LoadedProvider")
    }
}
pub struct Store<K: Keychain> {
    db: Connection,
    directory: PathBuf,
    keys: K,
    registry: SolveJobRegistry,
    _state: Lease,
    #[cfg(test)]
    faults: tests::Faults,
}
impl<K: Keychain> Store<K> {
    /// Directory is supplied by trusted app-local-data resolution and must be
    /// canonical/private. No path originates in provider/model/command bytes.
    pub fn open(directory: &Path, keys: K, registry: SolveJobRegistry) -> Result<Self, Error> {
        let state = registry
            .acquire_interactive_ingress(STATE_BYTES)
            .map_err(|_| Error::Admission)?;
        let meta = fs::symlink_metadata(directory).map_err(|_| Error::Unavailable)?;
        if !directory.is_absolute()
            || !meta.is_dir()
            || meta.permissions().mode() & 0o077 != 0
            || fs::canonicalize(directory).map_err(|_| Error::Unavailable)? != directory
        {
            return Err(Error::Unavailable);
        }
        let _gate = gate(directory)?;
        let path = directory.join(DB_NAME);
        let created = match OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(&path)
        {
            Ok(file) => {
                file.sync_all().map_err(|_| Error::Unavailable)?;
                true
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => false,
            Err(_) => return Err(Error::Unavailable),
        };
        let meta = fs::symlink_metadata(&path).map_err(|_| Error::Unavailable)?;
        if !meta.is_file() || meta.permissions().mode() & 0o077 != 0 || meta.len() > MAX_DB_BYTES {
            return Err(Error::Unavailable);
        }
        let mut db = Connection::open_with_flags(
            &path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(sql)?;
        db.busy_timeout(std::time::Duration::ZERO).map_err(sql)?;
        if !created {
            let app: i64 = db
                .pragma_query_value(None, "application_id", |r| r.get(0))
                .map_err(sql)?;
            let version: i64 = db
                .pragma_query_value(None, "user_version", |r| r.get(0))
                .map_err(sql)?;
            if app != APP_ID || version != 1 {
                return Err(Error::Corrupt);
            }
            validate_schema(&db)?;
        }
        db.execute_batch("PRAGMA trusted_schema=OFF;PRAGMA foreign_keys=ON;PRAGMA journal_mode=DELETE;PRAGMA synchronous=FULL;PRAGMA cache_size=-256;PRAGMA max_page_count=256;").map_err(sql)?;
        if created {
            let tx = db
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(sql)?;
            tx.execute_batch(&format!(
                "{CREDENTIALS_SQL};{PROVIDER_SQL};PRAGMA user_version=1;"
            ))
            .map_err(sql)?;
            tx.pragma_update(None, "application_id", APP_ID)
                .map_err(sql)?;
            tx.commit().map_err(sql)?;
        }
        validate_schema(&db)?;
        // Corrupt contents never become a default provider or credential mode.
        stored(&db)?;
        pending(&db)?;
        Ok(Self {
            db,
            directory: directory.to_owned(),
            keys,
            registry,
            _state: state,
            #[cfg(test)]
            faults: tests::Faults::default(),
        })
    }
    fn admission(&self) -> Result<Lease, Error> {
        self.registry
            .acquire_interactive_ingress(OPERATION_BYTES)
            .map_err(|_| Error::Admission)
    }
    pub fn snapshot(&self) -> Result<Snapshot, Error> {
        let _lease = self.admission()?;
        let _gate = gate(&self.directory)?;
        snapshot(&self.db)
    }
    pub fn save(
        &mut self,
        expected: Option<&str>,
        endpoint: &str,
        model: &str,
        change: CredentialChange,
    ) -> Result<Snapshot, Error> {
        let _lease = self.admission()?;
        let _gate = gate(&self.directory)?;
        let provider = Provider::new(endpoint).map_err(|_| Error::InvalidInput)?;
        let endpoint = provider.endpoint().to_owned();
        if !valid_model(model) || expected.is_some_and(|v| !identifier(v)) {
            return Err(Error::InvalidInput);
        }
        let tx = self
            .db
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(sql)?;
        validate_schema(&tx)?;
        let current = stored(&tx)?;
        matches_revision(current.as_ref(), expected)?;
        if matches!(change, CredentialChange::Keep)
            && current
                .as_ref()
                .is_none_or(|old| old.metadata.endpoint != endpoint)
        {
            return Err(Error::NewCredentialRequired);
        }
        if !pending(&tx)?.is_empty() {
            return Err(Error::RecoveryRequired);
        }
        let mut result = if let CredentialChange::Replace(secret) = change {
            let key = CredentialKey::fresh()?;
            let count: i64 = tx
                .query_row("SELECT count(*) FROM credentials", [], |r| r.get(0))
                .map_err(sql)?;
            if count >= MAX_ACCOUNTS {
                return Err(Error::RecoveryRequired);
            }
            tx.execute(
                "INSERT INTO credentials(account,owner,endpoint,phase) VALUES(?1,?2,?3,'pending')",
                params![key.account, key.owner, endpoint],
            )
            .map_err(sql)?;
            tx.commit().map_err(sql)?; // Durable intent precedes native secret write.
            let tx = self
                .db
                .transaction_with_behavior(TransactionBehavior::Immediate)
                .map_err(sql)?;
            validate_schema(&tx)?;
            let current = stored(&tx)?;
            matches_revision(current.as_ref(), expected)?;
            let present:bool=tx.query_row("SELECT EXISTS(SELECT 1 FROM credentials WHERE account=?1 AND owner=?2 AND phase='pending')",params![key.account,key.owner],|r|r.get(0)).map_err(sql)?;
            if !present {
                return Err(Error::Stale);
            }
            // This write gate also excludes concurrent recovery. Failed/uncertain
            // native operations retain the committed pending intent for recovery.
            self.keys
                .create(&key, &endpoint, &secret)
                .map_err(Error::Keychain)?;
            #[cfg(test)]
            if self.faults.fail_publish {
                tx.pragma_update(None, "query_only", true).unwrap();
            }
            publish(&tx, current.as_ref(), &endpoint, model, Some(&key))?;
            let committed = snapshot(&tx)?;
            tx.commit().map_err(sql)?;
            committed
        } else {
            let key = if matches!(change, CredentialChange::Keep) {
                current.as_ref().and_then(|c| c.key.as_ref())
            } else {
                None
            };
            publish(&tx, current.as_ref(), &endpoint, model, key)?;
            let committed = snapshot(&tx)?;
            tx.commit().map_err(sql)?;
            committed
        };
        #[cfg(test)]
        self.after_commit_fault();
        // Metadata is already committed. Report cleanup trouble with the actual
        // saved snapshot, not as a failure that could invite a duplicate save.
        match cleanup(&mut self.db, &mut self.keys) {
            Ok(()) => result.cleanup_pending = Some(0),
            Err(error) => {
                result.cleanup_pending = None;
                result.cleanup_error = Some(error);
            }
        }
        Ok(result)
    }
    pub fn clear(&mut self, expected: &str) -> Result<Snapshot, Error> {
        let _lease = self.admission()?;
        let _gate = gate(&self.directory)?;
        if !identifier(expected) {
            return Err(Error::InvalidInput);
        }
        let tx = self
            .db
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(sql)?;
        validate_schema(&tx)?;
        let current = stored(&tx)?;
        matches_revision(current.as_ref(), Some(expected))?;
        tx.execute(
            "UPDATE credentials SET phase='retired' WHERE phase='active'",
            [],
        )
        .map_err(sql)?;
        tx.execute("DELETE FROM provider", []).map_err(sql)?;
        let mut result = snapshot(&tx)?;
        tx.commit().map_err(sql)?;
        #[cfg(test)]
        self.after_commit_fault();
        match cleanup(&mut self.db, &mut self.keys) {
            Ok(()) => result.cleanup_pending = Some(0),
            Err(error) => {
                result.cleanup_pending = None;
                result.cleanup_error = Some(error);
            }
        }
        Ok(result)
    }
    #[cfg(test)]
    fn after_commit_fault(&mut self) {
        if self.faults.drop_provider_after_commit {
            self.faults.drop_provider_after_commit = false;
            self.db.execute_batch("DROP TABLE provider").unwrap();
        }
    }
    /// Explicit recovery only. A native error leaves the journal intact, even
    /// for entries deleted earlier in this attempt; exact deletion is idempotent.
    pub fn recover(&mut self) -> Result<Snapshot, Error> {
        let _lease = self.admission()?;
        let _gate = gate(&self.directory)?;
        let tx = self
            .db
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(sql)?;
        validate_schema(&tx)?;
        stored(&tx)?;
        for key in pending(&tx)? {
            match self.keys.delete(&key) {
                Ok(()) | Err(KeyError::Missing) => {}
                Err(e) => return Err(Error::Keychain(e)),
            }
            tx.execute(
                "DELETE FROM credentials WHERE account=?1 AND owner=?2 AND phase!='active'",
                params![key.account, key.owner],
            )
            .map_err(sql)?;
        }
        let committed = snapshot(&tx)?;
        tx.commit().map_err(sql)?;
        Ok(committed)
    }
    pub fn load(&mut self, expected: &str) -> Result<LoadedProvider, Error> {
        let _lease = self.admission()?;
        let _gate = gate(&self.directory)?;
        if !identifier(expected) {
            return Err(Error::InvalidInput);
        }
        let tx = self
            .db
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(sql)?;
        validate_schema(&tx)?;
        let current = stored(&tx)?.ok_or(Error::NotConfigured)?;
        matches_revision(Some(&current), Some(expected))?;
        if !pending(&tx)?.is_empty() {
            return Err(Error::RecoveryRequired);
        }
        let secret = match &current.key {
            Some(key) => Some(
                self.keys
                    .load(key, &current.metadata.endpoint)
                    .map_err(|reason| Error::ReenterKey { reason })?,
            ),
            None => None,
        };
        let provider = Provider::new(&current.metadata.endpoint).map_err(|_| Error::Corrupt)?;
        tx.commit().map_err(sql)?;
        Ok(LoadedProvider {
            revision: current.metadata.revision,
            model: current.metadata.model,
            provider,
            secret,
            _backing: _lease,
        })
    }
}
impl<K: Keychain> fmt::Debug for Store<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ProviderStore")
    }
}
/// Own an OS file lock across both durable journal commit and native calls.
/// Closing this original File releases the lock on error, unwind or process exit.
/// No stored PID, process-global lock or descriptor inheritance is used.
fn gate(directory: &Path) -> Result<std::fs::File, Error> {
    let path = directory.join("provider-settings.lock");
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(&path)
    {
        Ok(file) => file,
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            let meta = fs::symlink_metadata(&path).map_err(|_| Error::Unavailable)?;
            if !meta.is_file() || meta.permissions().mode() & 0o077 != 0 {
                return Err(Error::Unavailable);
            }
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)
                .map_err(|_| Error::Unavailable)?
        }
        Err(_) => return Err(Error::Unavailable),
    };
    file.try_lock().map_err(|error| match error {
        std::fs::TryLockError::WouldBlock => Error::Busy,
        _ => Error::Unavailable,
    })?;
    Ok(file)
}
fn nonce() -> Result<String, Error> {
    let mut bytes = [0u8; 24];
    getrandom::fill(&mut bytes).map_err(|_| Error::Unavailable)?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}
fn identifier(s: &str) -> bool {
    s.len() == 48
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn valid_model(s: &str) -> bool {
    !s.is_empty() && s.len() <= 1024 && !s.chars().any(char::is_control)
}
fn matches_revision(current: Option<&Stored>, expected: Option<&str>) -> Result<(), Error> {
    if current.map(|c| c.metadata.revision.as_str()) == expected {
        Ok(())
    } else {
        Err(Error::Stale)
    }
}
fn sql(error: rusqlite::Error) -> Error {
    match error {
        rusqlite::Error::SqliteFailure(ref e, _)
            if matches!(
                e.code,
                rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
            ) =>
        {
            Error::Busy
        }
        _ => Error::Corrupt,
    }
}
fn validate_schema(db: &Connection) -> Result<(), Error> {
    let app: i64 = db
        .pragma_query_value(None, "application_id", |r| r.get(0))
        .map_err(sql)?;
    let version: i64 = db
        .pragma_query_value(None, "user_version", |r| r.get(0))
        .map_err(sql)?;
    if app != APP_ID || version != 1 {
        return Err(Error::Corrupt);
    }
    let count: i64 = db
        .query_row("SELECT count(*) FROM sqlite_schema", [], |r| r.get(0))
        .map_err(sql)?;
    if count != 3 {
        return Err(Error::Corrupt);
    }
    let temporary: i64 = db
        .query_row("SELECT count(*) FROM sqlite_temp_schema", [], |r| r.get(0))
        .map_err(sql)?;
    if temporary != 0 {
        return Err(Error::Corrupt);
    }
    let expected = [
        (
            "index",
            "sqlite_autoindex_credentials_1",
            "credentials",
            None,
        ),
        ("table", "credentials", "credentials", Some(CREDENTIALS_SQL)),
        ("table", "provider", "provider", Some(PROVIDER_SQL)),
    ];
    let mut statement = db
        .prepare("SELECT type,name,tbl_name,sql FROM sqlite_schema ORDER BY type,name")
        .map_err(sql)?;
    let mut rows = statement.query([]).map_err(sql)?;
    for (kind, name, table, ddl) in expected {
        let row = rows.next().map_err(sql)?.ok_or(Error::Corrupt)?;
        let actual: (String, String, String, Option<String>) = (
            row.get(0).map_err(sql)?,
            row.get(1).map_err(sql)?,
            row.get(2).map_err(sql)?,
            row.get(3).map_err(sql)?,
        );
        if (
            actual.0.as_str(),
            actual.1.as_str(),
            actual.2.as_str(),
            actual.3.as_deref(),
        ) != (kind, name, table, ddl)
        {
            return Err(Error::Corrupt);
        }
    }
    if rows.next().map_err(sql)?.is_some() {
        return Err(Error::Corrupt);
    }
    Ok(())
}
fn stored(db: &Connection) -> Result<Option<Stored>, Error> {
    let count: i64 = db
        .query_row("SELECT count(*) FROM provider", [], |r| r.get(0))
        .map_err(sql)?;
    if !(0..=1).contains(&count) {
        return Err(Error::Corrupt);
    }
    let row = db
        .query_row(
            "SELECT revision,endpoint,model,account FROM provider WHERE singleton=1",
            [],
            |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, Option<String>>(3)?,
                ))
            },
        )
        .optional()
        .map_err(sql)?;
    let Some((revision, endpoint, model, account)) = row else {
        let active: i64 = db
            .query_row(
                "SELECT count(*) FROM credentials WHERE phase='active'",
                [],
                |r| r.get(0),
            )
            .map_err(sql)?;
        return if active == 0 {
            Ok(None)
        } else {
            Err(Error::Corrupt)
        };
    };
    if !identifier(&revision)
        || !valid_model(&model)
        || Provider::new(&endpoint)
            .map(|p| p.endpoint() != endpoint)
            .unwrap_or(true)
    {
        return Err(Error::Corrupt);
    }
    let active: i64 = db
        .query_row(
            "SELECT count(*) FROM credentials WHERE phase='active'",
            [],
            |r| r.get(0),
        )
        .map_err(sql)?;
    if active != i64::from(account.is_some()) {
        return Err(Error::Corrupt);
    }
    let key = if let Some(account) = account {
        let (owner, bound, phase): (String, String, String) = db
            .query_row(
                "SELECT owner,endpoint,phase FROM credentials WHERE account=?1",
                [&account],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(sql)?;
        if phase != "active" || bound != endpoint {
            return Err(Error::Corrupt);
        }
        Some(CredentialKey::checked(account, owner)?)
    } else {
        None
    };
    Ok(Some(Stored {
        metadata: Metadata {
            revision,
            endpoint,
            model,
            credential_mode: if key.is_some() {
                CredentialMode::Keychain
            } else {
                CredentialMode::None
            },
        },
        key,
    }))
}
fn pending(db: &Connection) -> Result<Vec<CredentialKey>, Error> {
    let count: i64 = db
        .query_row("SELECT count(*) FROM credentials", [], |r| r.get(0))
        .map_err(sql)?;
    if !(0..=MAX_ACCOUNTS).contains(&count) {
        return Err(Error::Corrupt);
    }
    let mut statement = db
        .prepare("SELECT account,owner,endpoint,phase FROM credentials WHERE phase!='active'")
        .map_err(sql)?;
    let mut rows = statement.query([]).map_err(sql)?;
    let mut out = Vec::with_capacity(MAX_ACCOUNTS as usize);
    while let Some(row) = rows.next().map_err(sql)? {
        let account: String = row.get(0).map_err(sql)?;
        let owner: String = row.get(1).map_err(sql)?;
        let endpoint: String = row.get(2).map_err(sql)?;
        let phase: String = row.get(3).map_err(sql)?;
        if !matches!(phase.as_str(), "pending" | "retired") || Provider::new(&endpoint).is_err() {
            return Err(Error::Corrupt);
        }
        out.push(CredentialKey::checked(account, owner)?);
    }
    Ok(out)
}
fn snapshot(db: &Connection) -> Result<Snapshot, Error> {
    validate_schema(db)?;
    Ok(Snapshot {
        provider: stored(db)?.map(|s| s.metadata),
        cleanup_pending: Some(pending(db)?.len()),
        cleanup_error: None,
    })
}
fn cleanup<K: Keychain>(db: &mut Connection, keys: &mut K) -> Result<(), KeyError> {
    let tx = db
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| KeyError::Unavailable)?;
    validate_schema(&tx).map_err(|_| KeyError::Invalid)?;
    stored(&tx).map_err(|_| KeyError::Invalid)?;
    for key in pending(&tx).map_err(|_| KeyError::Invalid)? {
        match keys.delete(&key) {
            Ok(()) | Err(KeyError::Missing) => {}
            Err(error) => return Err(error),
        }
        tx.execute(
            "DELETE FROM credentials WHERE account=?1 AND owner=?2 AND phase!='active'",
            params![key.account, key.owner],
        )
        .map_err(|_| KeyError::Unavailable)?;
    }
    tx.commit().map_err(|_| KeyError::Unavailable)
}
fn publish(
    db: &Connection,
    current: Option<&Stored>,
    endpoint: &str,
    model: &str,
    key: Option<&CredentialKey>,
) -> Result<(), Error> {
    if let Some(old) = current.and_then(|c| c.key.as_ref()) {
        if key.is_none_or(|new| new.account != old.account) {
            db.execute(
                "UPDATE credentials SET phase='retired' WHERE account=?1 AND owner=?2",
                params![old.account, old.owner],
            )
            .map_err(sql)?;
        }
    }
    if let Some(key) = key {
        db.execute(
            "UPDATE credentials SET phase='active' WHERE account=?1 AND owner=?2",
            params![key.account, key.owner],
        )
        .map_err(sql)?;
    }
    db.execute("INSERT INTO provider(singleton,revision,endpoint,model,account) VALUES(1,?1,?2,?3,?4) ON CONFLICT(singleton) DO UPDATE SET revision=excluded.revision,endpoint=excluded.endpoint,model=excluded.model,account=excluded.account",params![nonce()?,endpoint,model,key.map(|k|k.account.as_str())]).map_err(sql)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_pool::{test_host::ScriptedHost, MemoryPool};
    use std::{
        collections::HashMap,
        path::PathBuf,
        sync::{Arc, Mutex},
    };
    #[derive(Default)]
    pub(super) struct Faults {
        pub fail_publish: bool,
        pub drop_provider_after_commit: bool,
    }
    #[derive(Default)]
    struct Vault {
        items: HashMap<String, (String, String, String)>,
        loads: usize,
        creates: usize,
        deletes: usize,
        fail_create: Option<KeyError>,
        fail_load: Option<KeyError>,
        fail_delete: Option<KeyError>,
        insert_then_fail: bool,
    }
    #[derive(Clone)]
    struct Keys(Arc<Mutex<Vault>>);
    impl Keychain for Keys {
        fn create(
            &mut self,
            key: &CredentialKey,
            endpoint: &str,
            secret: &Secret,
        ) -> Result<(), KeyError> {
            let mut v = self.0.lock().unwrap();
            v.creates += 1;
            if let Some(e) = v.fail_create {
                return Err(e);
            }
            if v.items.contains_key(&key.account) {
                return Err(KeyError::Duplicate);
            }
            v.items.insert(
                key.account.clone(),
                (
                    key.owner.clone(),
                    endpoint.to_owned(),
                    secret.value().to_owned(),
                ),
            );
            if v.insert_then_fail {
                Err(KeyError::Unknown)
            } else {
                Ok(())
            }
        }
        fn load(&mut self, key: &CredentialKey, endpoint: &str) -> Result<Secret, KeyError> {
            let mut v = self.0.lock().unwrap();
            v.loads += 1;
            if let Some(e) = v.fail_load {
                return Err(e);
            }
            match v.items.get(&key.account) {
                Some((owner, bound, value)) if *owner == key.owner && bound == endpoint => {
                    Secret::new(value.clone()).map_err(|_| KeyError::Invalid)
                }
                _ => Err(KeyError::Missing),
            }
        }
        fn delete(&mut self, key: &CredentialKey) -> Result<(), KeyError> {
            let mut v = self.0.lock().unwrap();
            v.deletes += 1;
            if let Some(e) = v.fail_delete {
                return Err(e);
            }
            if v.items
                .get(&key.account)
                .is_some_and(|(owner, _, _)| *owner == key.owner)
            {
                v.items.remove(&key.account);
                Ok(())
            } else {
                Err(KeyError::Missing)
            }
        }
    }
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let path = PathBuf::from("/private/tmp")
                .join(format!("swbpipe-provider-{}", nonce().unwrap()));
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
    fn setup() -> (Fixture, Store<Keys>, Keys) {
        let f = Fixture::new();
        let k = Keys(Arc::new(Mutex::new(Vault::default())));
        let r = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let store = Store::open(&f.0, k.clone(), r).unwrap();
        (f, store, k)
    }
    fn key() -> CredentialChange {
        CredentialChange::Replace(Secret::new("invented-provider-key-distinctive".into()).unwrap())
    }
    fn revision(snapshot: &Snapshot) -> &str {
        &snapshot.provider.as_ref().unwrap().revision
    }
    #[test]
    fn review_regression_both_mutable_endpoints_cannot_retarget_a_saved_key() {
        let (_f, mut s, _k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        s.db.execute(
            "UPDATE provider SET endpoint='https://other.example/v1'",
            [],
        )
        .unwrap();
        s.db.execute(
            "UPDATE credentials SET endpoint='https://other.example/v1' WHERE phase='active'",
            [],
        )
        .unwrap();
        assert!(
            s.load(revision(&a)).is_err(),
            "mutable metadata retargeted the native secret"
        );
    }
    #[test]
    fn review_regression_schema_trigger_refused_before_any_native_effect() {
        let (_f, mut s, k) = setup();
        s.db.execute_batch("CREATE TRIGGER hostile AFTER INSERT ON provider BEGIN UPDATE credentials SET phase='retired';DELETE FROM provider;END;").unwrap();
        assert_eq!(
            s.save(None, "https://provider.example/v1", "model", key())
                .unwrap_err(),
            Error::Corrupt
        );
        assert_eq!(k.0.lock().unwrap().creates, 0);
    }
    #[test]
    fn review_regression_reopen_rejects_unexpected_schema() {
        let (f, s, k) = setup();
        s.db.execute_batch("CREATE VIEW unexpected AS SELECT 1;")
            .unwrap();
        drop(s);
        let r = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        assert!(matches!(Store::open(&f.0, k, r), Err(Error::Corrupt)));
    }
    #[test]
    fn review_regression_postcommit_save_error_preserves_committed_metadata() {
        let (_f, mut s, _k) = setup();
        s.faults.drop_provider_after_commit = true;
        let result = s
            .save(
                None,
                "https://provider.example/v1",
                "model",
                CredentialChange::NoKey,
            )
            .expect("save committed before read failure");
        assert_eq!(
            result.provider.unwrap().endpoint,
            "https://provider.example/v1"
        );
        assert!(result.cleanup_error.is_some());
        assert!(result.cleanup_pending.is_none());
    }
    #[test]
    fn review_regression_postcommit_clear_error_preserves_committed_removal() {
        let (_f, mut s, _k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        s.faults.drop_provider_after_commit = true;
        let result = s
            .clear(revision(&a))
            .expect("clear committed before read failure");
        assert!(result.provider.is_none());
        assert!(result.cleanup_error.is_some());
        assert!(result.cleanup_pending.is_none());
    }
    #[test]
    fn no_default_and_no_key_is_an_explicit_choice_with_no_vault_access() {
        let (_f, mut s, k) = setup();
        assert!(s.snapshot().unwrap().provider.is_none());
        k.0.lock().unwrap().fail_load = Some(KeyError::Unavailable);
        let a = s
            .save(
                None,
                "http://127.0.0.1:8123/v1",
                "invented-model",
                CredentialChange::NoKey,
            )
            .unwrap();
        let loaded = s.load(revision(&a)).unwrap();
        assert!(loaded.secret.is_none());
        assert_eq!(k.0.lock().unwrap().loads, 0);
        assert_eq!(k.0.lock().unwrap().creates, 0);
    }
    #[test]
    fn persistence_has_only_metadata_and_reopen_binds_the_same_owned_key() {
        let (f, mut s, k) = setup();
        let a = s
            .save(
                None,
                "https://provider.example/v1/",
                "invented-model",
                key(),
            )
            .unwrap();
        assert_eq!(a.cleanup_pending, Some(0));
        assert_eq!(
            a.provider.as_ref().unwrap().endpoint,
            "https://provider.example/v1"
        );
        for entry in fs::read_dir(&f.0).unwrap() {
            let bytes = fs::read(entry.unwrap().path()).unwrap();
            let marker = b"invented-provider-key-distinctive";
            assert!(!bytes.windows(marker.len()).any(|b| b == marker));
        }
        let public = serde_json::to_string(&a).unwrap();
        assert!(!public.contains("invented-provider-key"));
        assert!(!public.contains("account"));
        assert!(!public.contains("owner"));
        let r = revision(&a).to_owned();
        drop(s);
        let registry = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let mut s = Store::open(&f.0, k, registry).unwrap();
        let loaded = s.load(&r).unwrap();
        assert_eq!(
            loaded.secret.unwrap().value(),
            "invented-provider-key-distinctive"
        );
    }
    #[test]
    fn stale_revision_and_bad_inputs_do_not_touch_settings_or_keychain() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        let creates = k.0.lock().unwrap().creates;
        assert_eq!(
            s.save(None, "https://other.example/v1", "model", key())
                .unwrap_err(),
            Error::Stale
        );
        assert_eq!(
            s.save(
                Some(revision(&a)),
                "http://off-host.example/v1",
                "model",
                key()
            )
            .unwrap_err(),
            Error::InvalidInput
        );
        assert_eq!(
            s.save(
                Some(revision(&a)),
                "https://provider.example/v1",
                "bad\nmodel",
                key()
            )
            .unwrap_err(),
            Error::InvalidInput
        );
        assert_eq!(s.snapshot().unwrap(), a);
        assert_eq!(k.0.lock().unwrap().creates, creates);
    }
    #[test]
    fn saved_key_cannot_be_retargeted_to_another_endpoint_by_keep() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "first", key())
            .unwrap();
        assert_eq!(
            s.save(
                Some(revision(&a)),
                "https://other.example/v1",
                "second",
                CredentialChange::Keep
            )
            .unwrap_err(),
            Error::NewCredentialRequired
        );
        let b = s
            .save(
                Some(revision(&a)),
                "https://PROVIDER.example:443/v1/",
                "second",
                CredentialChange::Keep,
            )
            .unwrap();
        assert_eq!(b.provider.as_ref().unwrap().model, "second");
        assert_eq!(k.0.lock().unwrap().creates, 1);
        assert_eq!(k.0.lock().unwrap().loads, 0);
    }
    #[test]
    fn crash_after_native_creation_keeps_pending_intent_and_recovers_without_replay() {
        let (f, mut s, k) = setup();
        k.0.lock().unwrap().insert_then_fail = true;
        assert_eq!(
            s.save(None, "https://provider.example/v1", "model", key())
                .unwrap_err(),
            Error::Keychain(KeyError::Unknown)
        );
        assert_eq!(k.0.lock().unwrap().items.len(), 1);
        assert_eq!(s.snapshot().unwrap().cleanup_pending, Some(1));
        drop(s);
        let r = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let mut s = Store::open(&f.0, k.clone(), r).unwrap();
        assert!(s.snapshot().unwrap().provider.is_none());
        s.recover().unwrap();
        assert!(k.0.lock().unwrap().items.is_empty());
        assert_eq!(k.0.lock().unwrap().creates, 1);
    }
    #[test]
    fn failed_new_key_does_not_replace_current_provider_or_choose_another_store() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        k.0.lock().unwrap().fail_create = Some(KeyError::InteractionRequired);
        assert_eq!(
            s.save(
                Some(revision(&a)),
                "https://other.example/v1",
                "model",
                key()
            )
            .unwrap_err(),
            Error::Keychain(KeyError::InteractionRequired)
        );
        let now = s.snapshot().unwrap();
        assert_eq!(now.provider, a.provider);
        assert_eq!(now.cleanup_pending, Some(1));
        assert_eq!(k.0.lock().unwrap().loads, 0);
    }
    #[test]
    fn acl_change_is_reentry_and_replacement_removes_only_owned_stale_item() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        k.0.lock().unwrap().fail_load = Some(KeyError::Denied);
        assert_eq!(
            s.load(revision(&a)).unwrap_err(),
            Error::ReenterKey {
                reason: KeyError::Denied
            }
        );
        let b = s
            .save(
                Some(revision(&a)),
                "https://provider.example/v1",
                "model",
                CredentialChange::Replace(Secret::new("invented-reentered-key".into()).unwrap()),
            )
            .unwrap();
        assert_eq!(b.cleanup_pending, Some(0));
        assert_eq!(k.0.lock().unwrap().items.len(), 1);
        assert_eq!(k.0.lock().unwrap().deletes, 1);
        k.0.lock().unwrap().fail_load = None;
        assert_eq!(
            s.load(revision(&b)).unwrap().secret.unwrap().value(),
            "invented-reentered-key"
        );
    }
    #[test]
    fn cleanup_failure_preserves_saved_outcome_and_durable_owned_handle() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        k.0.lock().unwrap().fail_delete = Some(KeyError::InteractionRequired);
        let b = s
            .save(
                Some(revision(&a)),
                "https://other.example/v1",
                "model",
                key(),
            )
            .unwrap();
        assert_eq!(b.cleanup_pending, None);
        assert_eq!(s.snapshot().unwrap().cleanup_pending, Some(1));
        assert_eq!(b.cleanup_error, Some(KeyError::InteractionRequired));
        assert_eq!(
            b.provider.as_ref().unwrap().endpoint,
            "https://other.example/v1"
        );
        assert_eq!(s.load(revision(&b)).unwrap_err(), Error::RecoveryRequired);
        k.0.lock().unwrap().fail_delete = None;
        s.recover().unwrap();
        assert_eq!(k.0.lock().unwrap().items.len(), 1);
        assert!(s.load(revision(&b)).is_ok());
    }
    #[test]
    fn every_credential_load_failure_is_a_clear_reentry_state_without_fallback() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        for reason in [
            KeyError::Missing,
            KeyError::InteractionRequired,
            KeyError::Denied,
            KeyError::Unavailable,
            KeyError::Invalid,
            KeyError::Unknown,
        ] {
            k.0.lock().unwrap().fail_load = Some(reason);
            let error = s.load(revision(&a)).unwrap_err();
            assert_eq!(error, Error::ReenterKey { reason });
            assert!(error
                .user_message()
                .to_ascii_lowercase()
                .contains("re-enter"));
        }
        assert_eq!(k.0.lock().unwrap().items.len(), 1);
        assert_eq!(k.0.lock().unwrap().creates, 1);
    }
    #[test]
    fn clear_removes_only_the_current_owned_item_and_never_reuses_its_revision() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        let cleared = s.clear(revision(&a)).unwrap();
        assert!(cleared.provider.is_none());
        assert_eq!(cleared.cleanup_pending, Some(0));
        assert!(k.0.lock().unwrap().items.is_empty());
        assert_eq!(s.load(revision(&a)).unwrap_err(), Error::NotConfigured);
    }
    #[test]
    fn private_store_refuses_symlink_targets_without_changing_them() {
        let f = Fixture::new();
        let outside = Fixture::new();
        let canary = outside.0.join("canary");
        fs::write(&canary, b"invented unrelated data").unwrap();
        std::os::unix::fs::symlink(&canary, f.0.join(DB_NAME)).unwrap();
        let r = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let k = Keys(Arc::new(Mutex::new(Vault::default())));
        assert!(matches!(Store::open(&f.0, k, r), Err(Error::Unavailable)));
        assert_eq!(fs::read(canary).unwrap(), b"invented unrelated data");
    }
    #[test]
    fn owner_marker_prevents_deleting_a_colliding_unowned_item() {
        let (_f, mut s, k) = setup();
        let credential = CredentialKey::fresh().unwrap();
        s.db.execute("INSERT INTO credentials(account,owner,endpoint,phase) VALUES(?1,?2,'https://provider.example/v1','pending')",params![credential.account,credential.owner]).unwrap();
        k.0.lock().unwrap().items.insert(
            credential.account.clone(),
            (
                nonce().unwrap(),
                "https://unrelated.example/v1".into(),
                "unrelated-fixture-value".into(),
            ),
        );
        s.recover().unwrap();
        assert_eq!(k.0.lock().unwrap().items.len(), 1);
    }
    #[test]
    fn a_metadata_commit_failure_keeps_old_profile_and_durable_cleanup_intent() {
        let (_f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        s.faults.fail_publish = true;
        assert_eq!(
            s.save(
                Some(revision(&a)),
                "https://other.example/v1",
                "model",
                key()
            )
            .unwrap_err(),
            Error::Corrupt
        );
        assert_eq!(s.snapshot().unwrap().provider, a.provider);
        assert_eq!(s.snapshot().unwrap().cleanup_pending, Some(1));
        assert_eq!(k.0.lock().unwrap().items.len(), 2);
        s.db.pragma_update(None, "query_only", false).unwrap();
        s.faults.fail_publish = false;
        s.recover().unwrap();
        assert_eq!(k.0.lock().unwrap().items.len(), 1);
    }
    #[test]
    fn cross_instance_file_gate_and_revision_cas_are_both_enforced() {
        let (f, mut a, k) = setup();
        let r = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        let mut b = Store::open(&f.0, k.clone(), r).unwrap();
        let lock = gate(&f.0).unwrap();
        assert_eq!(
            b.save(None, "https://provider.example/v1", "model", key())
                .unwrap_err(),
            Error::Busy
        );
        drop(lock);
        let saved = a
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        assert_eq!(
            b.save(None, "https://other.example/v1", "model", key())
                .unwrap_err(),
            Error::Stale
        );
        assert_eq!(a.snapshot().unwrap(), saved);
    }
    #[test]
    fn corrupted_binding_and_unknown_schema_refuse_before_reading_any_credential() {
        let (f, mut s, k) = setup();
        let a = s
            .save(None, "https://provider.example/v1", "model", key())
            .unwrap();
        s.db.execute(
            "UPDATE credentials SET endpoint='https://other.example/v1' WHERE phase='active'",
            [],
        )
        .unwrap();
        assert_eq!(s.load(revision(&a)).unwrap_err(), Error::Corrupt);
        assert_eq!(k.0.lock().unwrap().loads, 0);
        s.db.pragma_update(None, "user_version", 99).unwrap();
        drop(s);
        let r = SolveJobRegistry::new(MemoryPool::new(ScriptedHost::normal()));
        assert!(matches!(Store::open(&f.0, k, r), Err(Error::Corrupt)));
    }
    #[test]
    fn admission_refuses_before_any_persistent_or_keychain_effect() {
        let f = Fixture::new();
        let k = Keys(Arc::new(Mutex::new(Vault::default())));
        let host = ScriptedHost::normal();
        let r = SolveJobRegistry::new(MemoryPool::new(host.clone()));
        host.set(|s| s.headroom_bytes = Some(0));
        assert!(matches!(
            Store::open(&f.0, k.clone(), r),
            Err(Error::Admission)
        ));
        assert_eq!(fs::read_dir(&f.0).unwrap().count(), 0);
        assert_eq!(k.0.lock().unwrap().creates, 0);
    }
}
