//! The streamed CSV save (`native_csv_stream`): exact bytes, the single save's evidence, names,
//! collisions and cleanup, the stream's binding to its webview, job and order, its bounds, and
//! the disk's failures.
use crate::native_result_download::{HostIo, OwnedWrite, SaveIo};
use super::*;
use serde_json::{json, Value};
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

const NAME: &str = "swbpipe-results-displacement.csv";
const WEBVIEW: &str = "main";
const JOB: &str = "job-1";
const HEAD: &str = "\u{FEFF}notice,x\r\n\r\nResult id,Value [mm]\r\n";
static NEXT: AtomicU64 = AtomicU64::new(0);

struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!("swbpipe-csv-stream-{}-{}-{}", std::process::id(), SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
    fn entries(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.0).unwrap().map(|e| e.unwrap().file_name().into_string().unwrap()).collect();
        names.sort();
        names
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn evidence() -> (Screening, LocalFirstEvidence) {
    let value = json!({
        "screening": {"route_id": "DOTH-CSV-011", "export_context": "local_private", "explicit_local_private_intent": true, "blocked": false, "materialization_withheld": false, "lossless_required": false, "exact_payload_match": true, "blocking_count": 0},
        "local_first": {"route_id": "DOTH-CSV-011", "export_context": "local_private", "storage_context": "local_private", "action": "include_metadata_only", "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED", "blocked": false, "metadata_only": true, "explicit_local_private_intent": true}
    });
    (serde_json::from_value(value["screening"].clone()).unwrap(), serde_json::from_value(value["local_first"].clone()).unwrap())
}
fn begin_request(text: &str) -> BeginRequest {
    let (screening, local_first) = evidence();
    BeginRequest { file_name: NAME.into(), text: text.into(), screening, local_first }
}
fn chunk(text: &str) -> ChunkRequest {
    let (screening, local_first) = evidence();
    ChunkRequest { text: text.into(), screening, local_first }
}
fn begin(streams: &CsvStreams, root: &Path, text: &str) -> Result<BeginReceipt, SaveError> {
    streams.begin(WEBVIEW, JOB, true, begin_request(text), || Ok::<_, ()>(root.to_path_buf()))
}
fn never<T>() -> Result<T, ()> {
    panic!("the downloads root must not be resolved")
}

/// Mutations of the evidence a chunk states, each of which the single save refuses.
fn mutated(index: usize) -> (Screening, LocalFirstEvidence) {
    let (mut s, mut l) = evidence();
    match index {
        0 => s.route_id = "DOTH-JSON-001".into(),
        1 => s.export_context = "public_report".into(),
        2 => s.explicit_local_private_intent = false,
        3 => s.blocked = true,
        4 => s.materialization_withheld = true,
        5 => s.lossless_required = true,
        6 => s.exact_payload_match = false,
        7 => s.blocking_count = 1,
        8 => l.route_id = "DOTH-JSON-001".into(),
        9 => l.export_context = "public_report".into(),
        10 => l.storage_context = "remote".into(),
        11 => l.action = "block_storage".into(),
        12 => l.reason_code = "SAFE_PUBLIC_METADATA".into(),
        13 => l.blocked = true,
        14 => l.metadata_only = false,
        15 => l.explicit_local_private_intent = false,
        _ => {
            s.route_id = "DOTH-JSON-001".into();
            l.route_id = "DOTH-JSON-001".into();
            s.lossless_required = true;
        }
    }
    (s, l)
}
const MUTATIONS: usize = 17;

#[test]
fn streamed_chunks_write_exactly_the_concatenated_bytes_with_the_single_saves_receipt() {
    let s = Scratch::new();
    let streams = CsvStreams::default();
    let rows = ["n1,-0\r\n", "\"q,\"\"x\"\"\",1e-7\r\n", "'=SUM(A1),é ☃\r\n"];
    let started = begin(&streams, &s.0, HEAD).unwrap();
    assert_eq!((started.file_name.as_str(), started.sequence, started.byte_count), (NAME, 1, HEAD.len() as u64));
    assert_eq!(started.stream_id.len(), 32);
    let mut expected = HEAD.to_string();
    for (i, row) in rows.iter().enumerate() {
        let receipt = streams.append(WEBVIEW, &started.stream_id, true, i as u64 + 1, chunk(row)).unwrap();
        expected.push_str(row);
        assert_eq!((receipt.sequence, receipt.byte_count), (i as u64 + 2, expected.len() as u64));
    }
    let receipt = streams.finish(WEBVIEW, &started.stream_id, true, 4, expected.len() as u64).unwrap();
    assert_eq!(fs::read(s.0.join(NAME)).unwrap(), expected.as_bytes());
    // The single save's receipt, member for member (the webview's `receipt` check reads exactly these).
    assert_eq!(
        serde_json::to_value(&receipt).unwrap(),
        json!({"outcome": "saved", "file_name": NAME, "byte_count": expected.len(), "replaced_existing": false, "durability": "not_guaranteed", "path_containment": "best_effort_non_adversarial"})
    );
    assert_eq!(serde_json::to_value(&started).unwrap()["outcome"], "streaming");
    assert_eq!(streams.open_count(), 0);
    // A second stream of the same name takes the ` (1)` suffix; the first file is kept.
    let second = begin(&streams, &s.0, HEAD).unwrap();
    assert_eq!(second.file_name, "swbpipe-results-displacement (1).csv");
    streams.finish(WEBVIEW, &second.stream_id, true, 1, HEAD.len() as u64).unwrap();
    assert_eq!(fs::read(s.0.join(NAME)).unwrap(), expected.as_bytes());
    assert_eq!(fs::read(s.0.join(&second.file_name)).unwrap(), HEAD.as_bytes());
}

#[test]
fn begin_refuses_evidence_names_and_texts_before_resolving_downloads_or_creating_a_file() {
    let streams = CsvStreams::default();
    for index in 0..MUTATIONS {
        let (screening, local_first) = mutated(index);
        let request = BeginRequest { screening, local_first, ..begin_request(HEAD) };
        let error = streams.begin(WEBVIEW, JOB, true, request, never).unwrap_err();
        assert_eq!((error.stage, error.cleanup), ("request", "not_needed"), "mutation {index}");
    }
    for name in ["swbpipe-results.csv", "../swbpipe-results-x.csv", "/tmp/swbpipe-results-x.csv", "openpipestress-preview-results-run.json", "swbpipe-results-x (1).csv", ""] {
        let request = BeginRequest { file_name: name.into(), ..begin_request(HEAD) };
        assert_eq!(streams.begin(WEBVIEW, JOB, true, request, never).unwrap_err().code, "INVALID_FILE_NAME", "{name}");
    }
    for (text, code) in [("", "STREAM_CHUNK_EMPTY"), ("Result id\r\n", "INVALID_CSV_BOM"), ("\u{FEFF}", "INVALID_CSV_BOM")] {
        assert_eq!(streams.begin(WEBVIEW, JOB, true, begin_request(text), never).unwrap_err().code, code, "{text:?}");
    }
    let large = format!("\u{FEFF}{}", "x".repeat(MAX_CHUNK_BYTES));
    assert_eq!(streams.begin(WEBVIEW, JOB, true, begin_request(&large), never).unwrap_err().code, "STREAM_CHUNK_TOO_LARGE");
    assert_eq!(streams.begin(WEBVIEW, JOB, false, begin_request(HEAD), never).unwrap_err().code, "STREAM_JOB_RELEASED");
    assert_eq!(streams.begin(WEBVIEW, JOB, true, begin_request(HEAD), || Err::<PathBuf, _>(())).unwrap_err().stage, "downloads");
    assert_eq!(streams.open_count(), 0);
}

#[test]
fn every_appended_chunk_needs_its_own_evidence_and_a_refused_chunk_ends_the_stream_and_removes_its_file() {
    for index in 0..MUTATIONS {
        let s = Scratch::new();
        let streams = CsvStreams::default();
        let started = begin(&streams, &s.0, HEAD).unwrap();
        let (screening, local_first) = mutated(index);
        let error = streams.append(WEBVIEW, &started.stream_id, true, 1, ChunkRequest { text: "a,1\r\n".into(), screening, local_first }).unwrap_err();
        assert_eq!(error.stage, "request", "mutation {index}");
        assert_eq!((error.cleanup, error.partial_file_name.as_deref()), ("removed", Some(NAME)), "mutation {index}");
        assert!(s.entries().is_empty(), "mutation {index}");
        assert_eq!(streams.open_count(), 0);
        assert_eq!(streams.append(WEBVIEW, &started.stream_id, true, 1, chunk("a,1\r\n")).unwrap_err().code, "STREAM_UNKNOWN");
    }
}

#[test]
fn order_size_counts_and_the_job_are_checked_and_each_refusal_ends_the_stream_once() {
    let cases: [(&str, Box<dyn Fn(&CsvStreams, &str) -> Result<(), SaveError>>); 8] = [
        ("STREAM_SEQUENCE_INVALID", Box::new(|st, id| st.append(WEBVIEW, id, true, 0, chunk("a\r\n")).map(drop))),
        ("STREAM_SEQUENCE_INVALID", Box::new(|st, id| st.append(WEBVIEW, id, true, 2, chunk("a\r\n")).map(drop))),
        ("STREAM_CHUNK_EMPTY", Box::new(|st, id| st.append(WEBVIEW, id, true, 1, chunk("")).map(drop))),
        ("STREAM_CHUNK_TOO_LARGE", Box::new(|st, id| st.append(WEBVIEW, id, true, 1, chunk(&"x".repeat(MAX_CHUNK_BYTES + 1))).map(drop))),
        ("STREAM_JOB_RELEASED", Box::new(|st, id| st.append(WEBVIEW, id, false, 1, chunk("a\r\n")).map(drop))),
        ("STREAM_BYTE_COUNT_MISMATCH", Box::new(|st, id| st.finish(WEBVIEW, id, true, 1, HEAD.len() as u64 + 1).map(drop))),
        ("STREAM_SEQUENCE_INVALID", Box::new(|st, id| st.finish(WEBVIEW, id, true, 2, HEAD.len() as u64).map(drop))),
        ("STREAM_JOB_RELEASED", Box::new(|st, id| st.finish(WEBVIEW, id, false, 1, HEAD.len() as u64).map(drop))),
    ];
    for (code, refused) in cases {
        let s = Scratch::new();
        let streams = CsvStreams::default();
        let started = begin(&streams, &s.0, HEAD).unwrap();
        let error = refused(&streams, &started.stream_id).unwrap_err();
        assert_eq!((error.code, error.cleanup), (code, "removed"), "{code}");
        assert!(s.entries().is_empty(), "{code}");
        // Ended once: every later request names an unknown stream, and nothing is written again.
        assert_eq!(streams.finish(WEBVIEW, &started.stream_id, true, 1, HEAD.len() as u64).unwrap_err().code, "STREAM_UNKNOWN");
        assert_eq!(streams.abort(WEBVIEW, &started.stream_id).unwrap_err().code, "STREAM_UNKNOWN");
        assert!(s.entries().is_empty());
    }
    // A duplicate append (the same sequence twice) is refused after the first one is written.
    let s = Scratch::new();
    let streams = CsvStreams::default();
    let started = begin(&streams, &s.0, HEAD).unwrap();
    streams.append(WEBVIEW, &started.stream_id, true, 1, chunk("a,1\r\n")).unwrap();
    assert_eq!(streams.append(WEBVIEW, &started.stream_id, true, 1, chunk("a,1\r\n")).unwrap_err().code, "STREAM_SEQUENCE_INVALID");
    assert!(s.entries().is_empty());
}

#[test]
fn another_webview_can_neither_write_finish_nor_end_a_stream() {
    let s = Scratch::new();
    let streams = CsvStreams::default();
    let started = begin(&streams, &s.0, HEAD).unwrap();
    let other = "other";
    assert_eq!(streams.append(other, &started.stream_id, true, 1, chunk("a\r\n")).unwrap_err().code, "STREAM_WEBVIEW_MISMATCH");
    let (screening, local_first) = mutated(2);
    assert_eq!(streams.append(other, &started.stream_id, true, 1, ChunkRequest { text: "a\r\n".into(), screening, local_first }).unwrap_err().cleanup, "not_needed");
    assert_eq!(streams.finish(other, &started.stream_id, true, 1, HEAD.len() as u64).unwrap_err().code, "STREAM_WEBVIEW_MISMATCH");
    assert_eq!(streams.abort(other, &started.stream_id).unwrap_err().code, "STREAM_WEBVIEW_MISMATCH");
    assert_eq!(streams.open_count(), 1);
    streams.append(WEBVIEW, &started.stream_id, true, 1, chunk("a\r\n")).unwrap();
    streams.finish(WEBVIEW, &started.stream_id, true, 2, HEAD.len() as u64 + 3).unwrap();
    assert_eq!(fs::read_to_string(s.0.join(NAME)).unwrap(), format!("{HEAD}a\r\n"));
}

#[test]
fn abort_disposal_and_reset_end_streams_and_remove_their_partial_files() {
    let s = Scratch::new();
    let streams = CsvStreams::default();
    let started = begin(&streams, &s.0, HEAD).unwrap();
    let receipt = streams.abort(WEBVIEW, &started.stream_id).unwrap();
    assert_eq!(serde_json::to_value(&receipt).unwrap(), json!({"outcome": "aborted", "partial_file_name": NAME, "cleanup": "removed"}));
    assert!(s.entries().is_empty());
    // A job's disposal ends its streams only; the session reset ends every stream.
    let a = streams.begin(WEBVIEW, "job-a", true, begin_request(HEAD), || Ok::<_, ()>(s.0.clone())).unwrap();
    let b = streams.begin(WEBVIEW, "job-b", true, begin_request(HEAD), || Ok::<_, ()>(s.0.clone())).unwrap();
    assert_eq!(streams.job_of(&a.stream_id).as_deref(), Some("job-a"));
    assert_eq!(streams.abort_job(Some("job-a")), 1);
    assert_eq!(s.entries(), vec![b.file_name.clone()]);
    assert_eq!(streams.append(WEBVIEW, &a.stream_id, true, 1, chunk("x\r\n")).unwrap_err().code, "STREAM_UNKNOWN");
    let c = streams.begin(WEBVIEW, "job-c", true, begin_request(HEAD), || Ok::<_, ()>(s.0.clone())).unwrap();
    assert_eq!(streams.abort_job(None), 2);
    assert!(s.entries().is_empty());
    assert_eq!(streams.open_count(), 0);
    assert_eq!(streams.finish(WEBVIEW, &c.stream_id, true, 1, HEAD.len() as u64).unwrap_err().code, "STREAM_UNKNOWN");
}

#[test]
fn at_most_four_streams_are_open_and_existing_entries_are_never_replaced() {
    let s = Scratch::new();
    fs::write(s.0.join(NAME), b"old bytes").unwrap();
    fs::create_dir(s.0.join("swbpipe-results-displacement (1).csv")).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(s.0.join("missing"), s.0.join("swbpipe-results-displacement (2).csv")).unwrap();
    let streams = CsvStreams::default();
    let ids: Vec<BeginReceipt> = (0..MAX_STREAMS).map(|_| begin(&streams, &s.0, HEAD).unwrap()).collect();
    assert_eq!(streams.begin(WEBVIEW, JOB, true, begin_request(HEAD), never).unwrap_err().code, "SAVE_BUSY");
    assert_eq!(fs::read(s.0.join(NAME)).unwrap(), b"old bytes");
    assert!(s.0.join("swbpipe-results-displacement (1).csv").is_dir());
    #[cfg(unix)]
    assert!(fs::symlink_metadata(s.0.join("swbpipe-results-displacement (2).csv")).unwrap().file_type().is_symlink());
    for started in &ids {
        assert_ne!(started.file_name, NAME);
        assert_eq!(destination_of(&streams, &started.stream_id).unwrap().parent().unwrap(), s.0.as_path());
        streams.abort(WEBVIEW, &started.stream_id).unwrap();
    }
    assert_eq!(fs::read(s.0.join(NAME)).unwrap(), b"old bytes");
    assert!(begin(&streams, &s.0, HEAD).is_ok());
}

/// A host whose file writes or flushes fail after `ok_writes` writes, or whose cleanup fails.
struct FaultIo {
    mode: &'static str,
    ok_writes: usize,
}
struct FaultFile {
    file: File,
    mode: &'static str,
    left: Arc<AtomicUsize>,
}
impl Write for FaultFile {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.mode == "write" || self.mode == "cleanup" || self.mode == "identity" {
            if self.left.load(Ordering::SeqCst) == 0 {
                return Err(io::Error::other("injected write failure"));
            }
            self.left.fetch_sub(1, Ordering::SeqCst);
        }
        self.file.write(bytes)
    }
    fn flush(&mut self) -> io::Result<()> {
        if self.mode == "flush" {
            return Err(io::Error::other("injected flush failure"));
        }
        self.file.flush()
    }
}
impl OwnedWrite for FaultFile {
    fn identity(&self) -> io::Result<(u64, u64)> {
        if self.mode == "identity" {
            Err(io::Error::other("no identity"))
        } else {
            self.file.identity()
        }
    }
}
impl SaveIo for FaultIo {
    fn open(&self, path: &Path) -> io::Result<Box<dyn OwnedWrite>> {
        let file = OpenOptions::new().write(true).create_new(true).open(path)?;
        Ok(Box::new(FaultFile { file, mode: self.mode, left: Arc::new(AtomicUsize::new(self.ok_writes)) }))
    }
    fn remove(&self, path: &Path) -> io::Result<()> {
        if self.mode == "cleanup" {
            Err(io::ErrorKind::PermissionDenied.into())
        } else {
            HostIo.remove(path)
        }
    }
}

#[test]
fn disk_failures_end_the_stream_with_a_truthful_cleanup() {
    // (mode, writes that succeed, where it fails, code, cleanup)
    for (mode, ok_writes, at_begin, code, status) in [
        ("write", 0, true, "WRITE_FAILED", "removed"),
        ("write", 1, false, "WRITE_FAILED", "removed"),
        ("cleanup", 1, false, "WRITE_FAILED", "failed"),
        ("identity", 1, false, "WRITE_FAILED", "retained"),
        ("flush", usize::MAX, false, "FLUSH_FAILED", "removed"),
    ] {
        let s = Scratch::new();
        let streams = CsvStreams::with_io(Arc::new(FaultIo { mode, ok_writes }));
        let error = if at_begin {
            begin(&streams, &s.0, HEAD).unwrap_err()
        } else {
            let started = begin(&streams, &s.0, HEAD).unwrap();
            if mode == "flush" {
                streams.finish(WEBVIEW, &started.stream_id, true, 1, HEAD.len() as u64).unwrap_err()
            } else {
                streams.append(WEBVIEW, &started.stream_id, true, 1, chunk("a,1\r\n")).unwrap_err()
            }
        };
        let status = if cfg!(unix) { status } else { "retained" };
        assert_eq!((error.code, error.cleanup, error.partial_file_name.as_deref()), (code, status, Some(NAME)), "{mode}");
        assert_eq!(s.0.join(NAME).exists(), status != "removed", "{mode}");
        assert_eq!(streams.open_count(), 0, "{mode}");
    }
}

#[test]
fn requests_decode_only_their_own_members() {
    let (screening, local_first) = evidence();
    let valid = serde_json::to_value(BeginRequest { file_name: NAME.into(), text: HEAD.into(), screening: screening.clone(), local_first: local_first.clone() }).unwrap();
    for (extra, value) in [("directory", json!("/tmp")), ("path", json!("/tmp/x.csv")), ("href", json!("data:text/csv;charset=utf-8,x")), ("job_id", json!("other"))] {
        let mut encoded = valid.clone();
        encoded[extra] = value;
        assert!(serde_json::from_value::<BeginRequest>(encoded).is_err(), "{extra}");
    }
    let mut chunk_value: Value = serde_json::to_value(ChunkRequest { text: "a".into(), screening, local_first }).unwrap();
    chunk_value["file_name"] = json!(NAME);
    assert!(serde_json::from_value::<ChunkRequest>(chunk_value).is_err());
}
