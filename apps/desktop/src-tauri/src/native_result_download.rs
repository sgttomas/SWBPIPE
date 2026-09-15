//! Private exact-byte local result saving. Admission evidence is defense in depth,
//! not caller authentication. Filesystem containment is non-adversarial best effort.
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::{atomic::{AtomicBool, Ordering}, Arc},
};

const PREFIX: &str = "data:application/json;charset=utf-8,";
const RESULT_FILE_PREFIX: &str = "openpipestress-preview-results-";
const STRESS_FILE_PREFIX: &str = "openpipestress-preview-stress-neutral-";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SaveRequest {
    pub href: String,
    pub file_name: String,
    pub screening: Screening,
    pub local_first: LocalFirstEvidence,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Screening {
    pub route_id: String,
    pub export_context: String,
    pub explicit_local_private_intent: bool,
    pub blocked: bool,
    pub materialization_withheld: bool,
    pub lossless_required: bool,
    pub exact_payload_match: bool,
    pub blocking_count: usize,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct LocalFirstEvidence {
    pub route_id: String,
    pub export_context: String,
    pub storage_context: String,
    pub action: String,
    pub reason_code: String,
    pub blocked: bool,
    pub metadata_only: bool,
    pub explicit_local_private_intent: bool,
}
#[derive(Debug, Serialize)]
pub(super) struct SaveReceipt {
    pub outcome: &'static str,
    pub file_name: String,
    pub byte_count: usize,
    pub replaced_existing: bool,
    pub durability: &'static str,
    pub path_containment: &'static str,
}
#[derive(Debug, Serialize)]
pub(super) struct SaveError {
    pub code: &'static str,
    pub stage: &'static str,
    pub message: &'static str,
    pub partial_file_name: Option<String>,
    pub cleanup: &'static str,
}
fn failure(code: &'static str, stage: &'static str) -> SaveError {
    SaveError { code, stage, message: "Local result save did not complete.", partial_file_name: None, cleanup: "not_needed" }
}
impl SaveError {
    pub(super) fn worker() -> Self {
        Self { cleanup: "unknown", ..failure("WORKER_FAILED", "worker") }
    }
}

#[derive(Default)]
pub(super) struct SaveAdmission(Arc<AtomicBool>);
#[derive(Debug)]
pub(super) struct SavePermit(Arc<AtomicBool>);
impl SaveAdmission {
    pub(super) fn admit(&self) -> Result<SavePermit, SaveError> {
        self.0.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map(|_| SavePermit(self.0.clone()))
            .map_err(|_| failure("SAVE_BUSY", "busy"))
    }
}
impl Drop for SavePermit {
    fn drop(&mut self) { self.0.store(false, Ordering::Release); }
}

fn valid_name(name: &str, prefix: &str) -> bool {
    let Some(token) = name.strip_prefix(prefix).and_then(|v| v.strip_suffix(".json")) else { return false; };
    !token.is_empty() && token.split('-').all(|part| !part.is_empty()
        && part.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()))
}
fn validate(request: &SaveRequest) -> Result<(), SaveError> {
    let s = &request.screening;
    let l = &request.local_first;
    if s.export_context != "local_private" || !s.explicit_local_private_intent || s.blocked || s.materialization_withheld
        || !s.lossless_required || !s.exact_payload_match || s.blocking_count != 0
        || l.route_id != s.route_id || l.export_context != s.export_context
        || l.storage_context != "local_private" || l.action != "include_metadata_only"
        || l.reason_code != "PRIVATE_LOCAL_METADATA_ALLOWED" || l.blocked
        || !l.metadata_only || !l.explicit_local_private_intent
    { return Err(failure("REQUEST_EVIDENCE_DENIED", "request")); }
    let prefix = match s.route_id.as_str() {
        "DOTH-JSON-001" => RESULT_FILE_PREFIX,
        "DOTH-FORMAT-003" => STRESS_FILE_PREFIX,
        _ => return Err(failure("REQUEST_EVIDENCE_DENIED", "request")),
    };
    if !valid_name(&request.file_name, prefix) { return Err(failure("INVALID_FILE_NAME", "request")); }
    Ok(())
}
fn decode(href: &str) -> Result<Vec<u8>, SaveError> {
    let body = href.strip_prefix(PREFIX).filter(|body| !body.is_empty())
        .ok_or_else(|| failure("INVALID_DATA_PREFIX", "decode"))?;
    let mut decoded = Vec::with_capacity(body.len());
    let bytes = body.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            let hex = |b: u8| (b as char).to_digit(16).map(|v| v as u8);
            let pair = bytes.get(index + 1).and_then(|a| hex(*a)).zip(bytes.get(index + 2).and_then(|b| hex(*b)))
                .ok_or_else(|| failure("INVALID_PERCENT_ESCAPE", "decode"))?;
            decoded.push(pair.0 * 16 + pair.1); index += 3;
        } else { decoded.push(bytes[index]); index += 1; }
    }
    let text = std::str::from_utf8(&decoded).map_err(|_| failure("INVALID_UTF8", "decode"))?;
    serde_json::from_str::<serde_json::Value>(text).map_err(|_| failure("INVALID_JSON", "decode"))?;
    Ok(decoded)
}

trait OwnedWrite: Write { fn identity(&self) -> io::Result<(u64, u64)>; }
#[cfg(unix)]
fn identity(metadata: fs::Metadata) -> io::Result<(u64, u64)> {
    use std::os::unix::fs::MetadataExt;
    Ok((metadata.dev(), metadata.ino()))
}
#[cfg(not(unix))]
fn identity(_: fs::Metadata) -> io::Result<(u64, u64)> { Err(io::ErrorKind::Unsupported.into()) }
impl OwnedWrite for File {
    fn identity(&self) -> io::Result<(u64, u64)> { identity(self.metadata()?) }
}
trait SaveIo {
    fn directory(&self, path: &Path) -> io::Result<bool> { fs::metadata(path).map(|m| m.is_dir()) }
    fn canonical(&self, path: &Path) -> io::Result<PathBuf> { fs::canonicalize(path) }
    fn open(&self, path: &Path) -> io::Result<Box<dyn OwnedWrite>>;
    fn path_identity(&self, path: &Path) -> io::Result<(u64, u64)> { identity(fs::symlink_metadata(path)?) }
    fn remove(&self, path: &Path) -> io::Result<()> { fs::remove_file(path) }
}
struct HostIo;
impl SaveIo for HostIo {
    fn open(&self, path: &Path) -> io::Result<Box<dyn OwnedWrite>> {
        OpenOptions::new().write(true).create_new(true).open(path).map(|f| Box::new(f) as Box<dyn OwnedWrite>)
    }
}
fn cleanup(error: &mut SaveError, path: &Path, owned: Option<(u64, u64)>, io: &impl SaveIo) {
    error.partial_file_name = path.file_name().and_then(|v| v.to_str()).map(str::to_owned);
    error.cleanup = match owned {
        Some(owned) if io.path_identity(path).ok() == Some(owned) => {
            if io.remove(path).is_ok() { "removed" } else { "failed" }
        }
        _ => "retained",
    };
}
fn save_bytes(root: &Path, basename: &str, bytes: &[u8], start: usize, io: &impl SaveIo) -> Result<SaveReceipt, SaveError> {
    if !root.is_absolute() || !matches!(io.directory(root), Ok(true)) {
        return Err(failure("DOWNLOADS_UNAVAILABLE", "downloads"));
    }
    let canonical_root = io.canonical(root).map_err(|_| failure("DOWNLOADS_CANONICAL_FAILED", "containment"))?;
    let stem = basename.strip_suffix(".json").ok_or_else(|| failure("INVALID_FILE_NAME", "request"))?;
    let mut counter = start;
    loop {
        let file_name = if counter == 0 { basename.to_owned() } else { format!("{stem} ({counter}).json") };
        let destination = root.join(&file_name);
        if destination.parent().and_then(|p| io.canonical(p).ok()).as_ref() != Some(&canonical_root) {
            return Err(failure("PATH_CONTAINMENT_FAILED", "containment"));
        }
        let mut file = match io.open(&destination) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                counter = counter.checked_add(1).ok_or_else(|| failure("COLLISION_OVERFLOW", "open"))?;
                continue;
            }
            Err(_) => return Err(failure("CREATE_NEW_FAILED", "open")),
        };
        let owned = file.identity().ok();
        if file.write_all(bytes).is_err() {
            let mut error = failure("WRITE_FAILED", "write"); cleanup(&mut error, &destination, owned, io); return Err(error);
        }
        if file.flush().is_err() {
            let mut error = failure("FLUSH_FAILED", "flush"); cleanup(&mut error, &destination, owned, io); return Err(error);
        }
        return Ok(SaveReceipt { outcome: "saved", file_name, byte_count: bytes.len(), replaced_existing: false,
            durability: "not_guaranteed", path_containment: "best_effort_non_adversarial" });
    }
}

pub(super) fn save_request<E>(request: SaveRequest, downloads: impl FnOnce() -> Result<PathBuf, E>) -> Result<SaveReceipt, SaveError> {
    validate(&request)?;
    let bytes = decode(&request.href)?;
    let root = downloads().map_err(|_| failure("DOWNLOADS_RESOLVER_FAILED", "downloads"))?;
    save_bytes(&root, &request.file_name, &bytes, 0, &HostIo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::{cell::Cell, sync::atomic::AtomicU64, time::{SystemTime, UNIX_EPOCH}};
    const NAME: &str = "openpipestress-preview-results-run-350.json";
    const STRESS_NAME: &str = "openpipestress-preview-stress-neutral-result-run-350.json";
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Scratch(PathBuf);
    impl Scratch { fn new() -> Self { let p = std::env::temp_dir().join(format!("ops-result-save-{}-{}-{}", std::process::id(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(), NEXT.fetch_add(1, Ordering::Relaxed))); fs::create_dir(&p).unwrap(); Self(p) } }
    impl Drop for Scratch { fn drop(&mut self) { let _ = fs::remove_dir_all(&self.0); } }
    fn percent(text: &str) -> String { text.as_bytes().iter().map(|b| format!("%{b:02X}")).collect() }
    fn request(text: &str) -> SaveRequest {
        request_for(text, "DOTH-JSON-001", NAME)
    }
    fn request_for(text: &str, route: &str, name: &str) -> SaveRequest {
        serde_json::from_value(json!({"href":format!("{PREFIX}{}",percent(text)),"file_name":name,
            "screening":{"route_id":route,"export_context":"local_private","explicit_local_private_intent":true,"blocked":false,"materialization_withheld":false,"lossless_required":true,"exact_payload_match":true,"blocking_count":0},
            "local_first":{"route_id":route,"export_context":"local_private","storage_context":"local_private","action":"include_metadata_only","reason_code":"PRIVATE_LOCAL_METADATA_ALLOWED","blocked":false,"metadata_only":true,"explicit_local_private_intent":true}})).unwrap()
    }
    #[test]
    fn contradictory_route_intent_evidence_and_unknown_fields_never_resolve_or_write() {
        for mutation in 0..17 {
            let mut r = request("{}");
            match mutation {
                0 => r.screening.route_id = "other".into(), 1 => r.screening.export_context = "public_report".into(),
                2 => r.screening.explicit_local_private_intent = false, 3 => r.screening.blocked = true,
                4 => r.screening.materialization_withheld = true, 5 => r.screening.lossless_required = false,
                6 => r.screening.exact_payload_match = false, 7 => r.screening.blocking_count = 1,
                8 => r.local_first.route_id = "other".into(), 9 => r.local_first.export_context = "public_report".into(),
                10 => r.local_first.storage_context = "remote".into(), 11 => r.local_first.action = "block_storage".into(),
                12 => r.local_first.reason_code = "SAFE_PUBLIC_METADATA".into(), 13 => r.local_first.blocked = true,
                14 => r.local_first.metadata_only = false, 15 => r.local_first.explicit_local_private_intent = false,
                _ => r.file_name = "../outside.json".into(),
            }
            assert_eq!(save_request(r, || -> Result<PathBuf, ()> { panic!("must not resolve") }).unwrap_err().stage, "request");
        }
        let valid = serde_json::to_value(request("{}")).unwrap();
        let mut encoded = valid.clone(); encoded["directory"] = json!("/tmp");
        assert!(serde_json::from_value::<SaveRequest>(encoded).is_err());
        let mut encoded = valid["screening"].clone(); encoded["unexpected"] = json!(true);
        assert!(serde_json::from_value::<Screening>(encoded).is_err());
        let mut encoded = valid["local_first"].clone(); encoded["unexpected"] = json!(true);
        assert!(serde_json::from_value::<LocalFirstEvidence>(encoded).is_err());
    }
    #[test]
    fn strict_prefix_percent_utf8_json_and_unsuffixed_names() {
        for href in ["data:application/json,{}", "data:application/json;charset=utf-8,", "data:application/json;charset=UTF-8,{}", "data:application/json;charset=utf-8;base64,e30=", "data:application/json;charset=utf-8,%", "data:application/json;charset=utf-8,%0G", "data:application/json;charset=utf-8,%FF", "data:application/json;charset=utf-8,{oops}"] {
            assert!(decode(href).is_err(), "{href}");
        }
        assert_eq!(decode("data:application/json;charset=utf-8,%22a+b%2525%22").unwrap(), b"\"a+b%25\"");
        for name in ["openpipestress-preview-results-.json", "openpipestress-preview-results-run--350.json", "openpipestress-preview-results-RUN.json", "openpipestress-preview-results-rún.json", "openpipestress-preview-results-run (1).json", "openpipestress-preview-results-run.JSON", "/outside.json", "../outside.json", "other.json"] { assert!(!valid_name(name, RESULT_FILE_PREFIX), "{name}"); }
        assert!(valid_name(STRESS_NAME, STRESS_FILE_PREFIX));
        assert!(!valid_name(STRESS_NAME, RESULT_FILE_PREFIX));
    }
    #[test]
    fn exactly_paired_result_and_stress_profiles_are_admitted() {
        let s = Scratch::new();
        let result = save_request(request_for("{\"family\":\"result\"}", "DOTH-JSON-001", NAME), || Ok::<_, ()>(s.0.clone())).unwrap();
        let stress = save_request(request_for("{\"family\":\"stress\"}", "DOTH-FORMAT-003", STRESS_NAME), || Ok::<_, ()>(s.0.clone())).unwrap();
        assert!(result.file_name.starts_with(RESULT_FILE_PREFIX));
        assert!(stress.file_name.starts_with(STRESS_FILE_PREFIX));
        for (route, name) in [("DOTH-JSON-001", STRESS_NAME), ("DOTH-FORMAT-003", NAME)] {
            assert_eq!(save_request(request_for("{}", route, name), || -> Result<PathBuf, ()> { panic!("must not resolve") }).unwrap_err().stage, "request");
        }
    }
    #[test]
    fn actual_files_preserve_exact_escaped_unicode_numbers_and_multi_megabyte_bytes() {
        let s = Scratch::new();
        let texts = ["{\n \"text\":\"é ☃ + %25 %2525 \\n\\t\\\\\", \"number\":1.2300e+02, \"negative\":-0.0\n}".to_owned(), format!("{{\"large\":\"{}\"}}", "é+%".repeat(600_000))];
        for text in texts { let receipt = save_request(request(&text), || Ok::<_, ()>(s.0.clone())).unwrap(); assert_eq!(fs::read(s.0.join(receipt.file_name)).unwrap(), text.as_bytes()); assert_eq!(receipt.byte_count,text.len()); assert!(!receipt.replaced_existing); assert_eq!(receipt.durability,"not_guaranteed"); }
    }
    #[test]
    fn repeated_save_file_directory_and_symlink_collisions_preserve_existing_entries() {
        let s = Scratch::new(); fs::write(s.0.join(NAME), b"old bytes").unwrap();
        let stem = NAME.strip_suffix(".json").unwrap(); fs::create_dir(s.0.join(format!("{stem} (1).json"))).unwrap();
        #[cfg(unix)] { std::os::unix::fs::symlink(s.0.join("missing"),s.0.join(format!("{stem} (2).json"))).unwrap(); }
        let first = save_request(request("{\"value\":350}"),|| Ok::<_, ()>(s.0.clone())).unwrap();
        let first_bytes = fs::read(s.0.join(&first.file_name)).unwrap(); let second = save_request(request("{\"value\":500}"),|| Ok::<_, ()>(s.0.clone())).unwrap();
        assert_ne!(first.file_name,second.file_name); assert_eq!(fs::read(s.0.join(NAME)).unwrap(),b"old bytes"); assert_eq!(fs::read(s.0.join(first.file_name)).unwrap(),first_bytes);
        assert!(s.0.join(format!("{stem} (1).json")).is_dir());
        #[cfg(unix)] assert!(fs::symlink_metadata(s.0.join(format!("{stem} (2).json"))).unwrap().file_type().is_symlink());
    }
    #[test]
    fn downloads_errors_fail_without_fallback_and_platform_symlink_root_is_allowed() {
        assert_eq!(save_request(request("{}"),|| Err::<PathBuf,_>(())).unwrap_err().stage,"downloads");
        let s = Scratch::new(); fs::write(s.0.join("file"),b"x").unwrap();
        for root in [PathBuf::from("relative"),s.0.join("missing"),s.0.join("file")] { assert!(save_request(request("{}"),||Ok::<_,()>(root)).is_err()); }
        #[cfg(unix)] { let target=s.0.join("actual");fs::create_dir(&target).unwrap();let root=s.0.join("Downloads");std::os::unix::fs::symlink(&target,&root).unwrap();let receipt=save_request(request("{}"),||Ok::<_,()>(root)).unwrap();assert_eq!(fs::read(target.join(receipt.file_name)).unwrap(),b"{}"); }
    }
    struct FaultIo { mode: &'static str, calls: Cell<usize> }
    struct FaultFile { file: File, mode: &'static str, written: bool }
    impl Write for FaultFile {
        fn write(&mut self,b:&[u8])->io::Result<usize>{ if self.mode=="write" { if self.written{return Err(io::ErrorKind::Other.into());}self.written=true;self.file.write(&b[..b.len().min(2)]) } else {self.file.write(b)} }
        fn flush(&mut self)->io::Result<()> {if self.mode=="flush"{Err(io::ErrorKind::Other.into())}else{self.file.flush()}}
    }
    impl OwnedWrite for FaultFile {fn identity(&self)->io::Result<(u64,u64)>{if self.mode=="identity"{Err(io::ErrorKind::Other.into())}else{self.file.identity()}}}
    impl SaveIo for FaultIo {
        fn directory(&self,p:&Path)->io::Result<bool>{if self.mode=="directory"{Err(io::ErrorKind::PermissionDenied.into())}else{HostIo.directory(p)}}
        fn canonical(&self,p:&Path)->io::Result<PathBuf>{if self.mode=="canonical"{Err(io::ErrorKind::PermissionDenied.into())}else if self.mode=="parent"&&self.calls.replace(self.calls.get()+1)>0{Ok(PathBuf::from("/outside"))}else{HostIo.canonical(p)}}
        fn open(&self,p:&Path)->io::Result<Box<dyn OwnedWrite>> {
            if self.mode=="open"{return Err(io::ErrorKind::PermissionDenied.into());}
            if self.mode=="overflow"{return Err(io::ErrorKind::AlreadyExists.into());}
            if self.mode=="race" && self.calls.replace(self.calls.get()+1)==0 {fs::write(p,b"raced bytes")?;}
            let file=OpenOptions::new().write(true).create_new(true).open(p)?;
            Ok(Box::new(FaultFile{file,mode:if self.mode=="cleanup"||self.mode=="identity"{"write"}else{self.mode},written:false}))
        }
        fn path_identity(&self,p:&Path)->io::Result<(u64,u64)>{if self.mode=="identity"{Err(io::ErrorKind::Other.into())}else{HostIo.path_identity(p)}}
        fn remove(&self,p:&Path)->io::Result<()>{if self.mode=="cleanup"{Err(io::ErrorKind::PermissionDenied.into())}else{HostIo.remove(p)}}
    }
    #[test]
    fn actual_create_new_race_retries_only_already_exists_and_checked_overflow_fails() {
        let s=Scratch::new(); let io=FaultIo{mode:"race",calls:Cell::new(0)};
        let receipt=save_bytes(&s.0,NAME,b"{}",0,&io).unwrap(); assert!(receipt.file_name.contains(" (1)")); assert_eq!(fs::read(s.0.join(NAME)).unwrap(),b"raced bytes");
        let io=FaultIo{mode:"overflow",calls:Cell::new(0)};assert_eq!(save_bytes(&s.0,NAME,b"{}",usize::MAX,&io).unwrap_err().code,"COLLISION_OVERFLOW");
    }
    #[test]
    fn partial_write_flush_and_cleanup_failures_preserve_primary_error_and_owned_cleanup_status() {
        for (mode,code,cleanup_status) in [("write","WRITE_FAILED","removed"),("flush","FLUSH_FAILED","removed"),("cleanup","WRITE_FAILED","failed"),("identity","WRITE_FAILED","retained")] {
            let s=Scratch::new();let io=FaultIo{mode,calls:Cell::new(0)};let error=save_bytes(&s.0,NAME,b"{\"value\":350}",0,&io).unwrap_err();let cleanup_status=if cfg!(unix){cleanup_status}else{"retained"};assert_eq!(error.code,code);assert_eq!(error.cleanup,cleanup_status);assert_eq!(error.partial_file_name.as_deref(),Some(NAME));assert_eq!(s.0.join(NAME).exists(),cleanup_status!="removed");
        }
        for mode in ["directory","canonical","parent","open"] {let s=Scratch::new();let io=FaultIo{mode,calls:Cell::new(0)};let error=save_bytes(&s.0,NAME,b"{}",0,&io).unwrap_err();assert_eq!(error.cleanup,"not_needed");assert!(!s.0.join(NAME).exists());}
    }
    #[test]
    fn admission_is_owned_until_actual_worker_guard_drops_even_if_requester_is_gone() {
        let admission=SaveAdmission::default();let permit=admission.admit().unwrap(); assert_eq!(admission.admit().unwrap_err().code,"SAVE_BUSY");
        let (send,receive)=std::sync::mpsc::channel();let (done_send,done_receive)=std::sync::mpsc::channel();
        let worker=std::thread::spawn(move||{let _permit=permit;receive.recv().unwrap();done_send.send(()).unwrap();});
        assert!(admission.admit().is_err());send.send(()).unwrap();done_receive.recv().unwrap();worker.join().unwrap();assert!(admission.admit().is_ok());
        let permit=admission.admit().unwrap();let worker=std::thread::spawn(move||{let _permit=permit;panic!("injected worker panic");});assert!(worker.join().is_err());assert!(admission.admit().is_ok());assert_eq!(SaveError::worker().cleanup,"unknown");
    }
}
