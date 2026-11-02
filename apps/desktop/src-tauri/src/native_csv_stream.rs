//! C1 native custody (DEL-04-05 `c1-native-custody-plan.md` §5; DEL-08-04 `csv-export.md` §4):
//! the CSV table route (`DOTH-CSV-011`) written to Downloads in bounded chunks, for a results
//! group too large to assemble in the webview. It is the private local save of
//! `native_result_download`, streamed: the same evidence rules, file names, exclusive creation,
//! collision suffixes, partial-file cleanup and receipts, with no other filesystem authority.
//!
//! - **Begin** (`begin_local_csv_stream {jobId, request: {file_name, text, screening,
//!   local_first}}`): the first chunk with the stream's file name. Its evidence must state the CSV
//!   route's local-private output with explicit intent, exactly as a single save's; its text must
//!   begin with the UTF-8 byte-order mark. The job must be a result this process still holds, and
//!   the stream is bound to it and to the calling webview. The file is created exclusively (never
//!   replacing an entry) and the first chunk written; the reply names the stream and the file.
//! - **Append** (`append_local_csv_stream {streamId, sequence, chunk: {text, screening,
//!   local_first}}`): the next chunk, in order (`sequence` is the number of chunks written), each
//!   with its own screening evidence, non-empty and at most [`MAX_CHUNK_BYTES`].
//! - **Finish** (`finish_local_csv_stream {streamId, sequence, byteCount}`): the chunk count and
//!   byte count the webview sent must be those written; the file is flushed and the receipt is the
//!   single save's (`SaveReceipt`).
//! - **Abort** (`abort_local_csv_stream {streamId}`), and every refusal after begin: the stream
//!   ends and its partial file is removed when it is still the file this stream created (the single
//!   save's `cleanup`), with the cleanup stated in the reply.
//!
//! A stream ends exactly once: finish, abort, a refused chunk (wrong webview, sequence, size,
//! evidence or BOM), a write failure, or the disposal of its job or the session reset (which
//! abort every stream of the job, or all). A refused request names its stream's cleanup. At most
//! [`MAX_STREAMS`] are open; each holds one file handle and counters, never the text. Callers
//! cannot name a path, a directory or another job's bytes: the file is the one begin created.
//! Admission evidence is defense in depth, not caller authentication (as the single save's);
//! filesystem containment is non-adversarial best effort.
use crate::native_result_download::{cleanup, failure, open_new, profile, valid_csv_name, validate_evidence, HostIo, LocalFirstEvidence, OwnedWrite, SaveError, SaveIo, SaveReceipt, Screening, CSV_ROUTE, UTF8_BOM};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

/// The most streams open at once.
pub(crate) const MAX_STREAMS: usize = 4;
/// The largest chunk text, in UTF-8 bytes (the webview sends far smaller ones: one page of rows,
/// split at 256 Ki UTF-16 units).
pub(crate) const MAX_CHUNK_BYTES: usize = 4 << 20;
/// The largest raw argument body of a stream command: a chunk at its limit, JSON-escaped.
pub(crate) const MAX_STREAM_BODY_BYTES: usize = 6 * MAX_CHUNK_BYTES + (64 << 10);

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BeginRequest {
    pub file_name: String,
    pub text: String,
    pub screening: Screening,
    pub local_first: LocalFirstEvidence,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ChunkRequest {
    pub text: String,
    pub screening: Screening,
    pub local_first: LocalFirstEvidence,
}
#[derive(Debug, Serialize)]
pub(crate) struct BeginReceipt {
    pub outcome: &'static str,
    pub stream_id: String,
    pub file_name: String,
    pub sequence: u64,
    pub byte_count: u64,
}
#[derive(Debug, Serialize)]
pub(crate) struct AppendReceipt {
    pub outcome: &'static str,
    pub sequence: u64,
    pub byte_count: u64,
}
#[derive(Debug, Serialize)]
pub(crate) struct AbortReceipt {
    pub outcome: &'static str,
    pub partial_file_name: Option<String>,
    pub cleanup: &'static str,
}

/// An open stream: its binding, its counters and its file (out of the table while a chunk is
/// written, so no lock is held across the write).
struct Stream {
    webview: String,
    job_id: String,
    chunks: u64,
    bytes: u64,
    file: Option<Box<dyn OwnedWrite>>,
    destination: PathBuf,
    file_name: String,
    owned: Option<(u64, u64)>,
    /// A refusal arrived while a chunk was being written: the writer ends the stream.
    poisoned: bool,
}

/// The open streams of this process.
pub(crate) struct CsvStreams {
    streams: Mutex<HashMap<String, Stream>>,
    io: Arc<dyn SaveIo + Send + Sync>,
}

impl Default for CsvStreams {
    fn default() -> Self {
        Self::with_io(Arc::new(HostIo))
    }
}

fn stream_failure(code: &'static str) -> SaveError {
    failure(code, "stream")
}

/// The stream's end: its file closed, then its partial file removed when it is still the file
/// the stream created, as the single save's cleanup states it.
fn ended(mut error: SaveError, mut stream: Stream, io: &(dyn SaveIo + Send + Sync)) -> SaveError {
    drop(stream.file.take());
    cleanup(&mut error, &stream.destination, stream.owned, io);
    error
}

fn chunk_text(text: &str, first: bool) -> Result<(), SaveError> {
    if text.is_empty() {
        return Err(stream_failure("STREAM_CHUNK_EMPTY"));
    }
    if text.len() > MAX_CHUNK_BYTES {
        return Err(stream_failure("STREAM_CHUNK_TOO_LARGE"));
    }
    if first && (!text.as_bytes().starts_with(UTF8_BOM) || text.len() == UTF8_BOM.len()) {
        return Err(failure("INVALID_CSV_BOM", "decode"));
    }
    Ok(())
}

/// A chunk's evidence: the CSV route's, as a single CSV save's must state it.
fn chunk_evidence(screening: &Screening, local_first: &LocalFirstEvidence) -> Result<(), SaveError> {
    let route = validate_evidence(screening, local_first)?;
    if !route.csv || screening.route_id != CSV_ROUTE {
        return Err(failure("REQUEST_EVIDENCE_DENIED", "request"));
    }
    Ok(())
}

fn stream_id() -> Result<String, SaveError> {
    let mut bytes = [0u8; 16];
    getrandom::fill(&mut bytes).map_err(|_| stream_failure("STREAM_ID_UNAVAILABLE"))?;
    Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
}

impl CsvStreams {
    fn with_io(io: Arc<dyn SaveIo + Send + Sync>) -> Self {
        Self { streams: Mutex::new(HashMap::new()), io }
    }

    fn table(&self) -> std::sync::MutexGuard<'_, HashMap<String, Stream>> {
        self.streams.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// The number of open streams.
    #[cfg(test)]
    pub(crate) fn open_count(&self) -> usize {
        self.table().len()
    }

    /// Begins a stream for `webview` on held job `job_id` (`job_held`): validates the request,
    /// resolves the downloads root, creates the file exclusively and writes the first chunk.
    pub(crate) fn begin<E>(
        &self,
        webview: &str,
        job_id: &str,
        job_held: bool,
        request: BeginRequest,
        downloads: impl FnOnce() -> Result<PathBuf, E>,
    ) -> Result<BeginReceipt, SaveError> {
        chunk_evidence(&request.screening, &request.local_first)?;
        if !valid_csv_name(&request.file_name) {
            return Err(failure("INVALID_FILE_NAME", "request"));
        }
        chunk_text(&request.text, true)?;
        if !job_held {
            return Err(stream_failure("STREAM_JOB_RELEASED"));
        }
        if self.table().len() >= MAX_STREAMS {
            return Err(failure("SAVE_BUSY", "busy"));
        }
        let id = stream_id()?;
        let root = downloads().map_err(|_| failure("DOWNLOADS_RESOLVER_FAILED", "downloads"))?;
        let ext = profile(CSV_ROUTE).expect("the CSV route has a profile").ext;
        let opened = open_new(&root, &request.file_name, ext, 0, self.io.as_ref())?;
        let mut stream = Stream {
            webview: webview.to_owned(),
            job_id: job_id.to_owned(),
            chunks: 0,
            bytes: 0,
            file: Some(opened.file),
            destination: opened.destination,
            file_name: opened.file_name,
            owned: opened.owned,
            poisoned: false,
        };
        if stream.file.as_mut().expect("opened").write_all(request.text.as_bytes()).is_err() {
            return Err(ended(failure("WRITE_FAILED", "write"), stream, self.io.as_ref()));
        }
        stream.chunks = 1;
        stream.bytes = request.text.len() as u64;
        let mut table = self.table();
        if table.len() >= MAX_STREAMS {
            drop(table);
            return Err(ended(failure("SAVE_BUSY", "busy"), stream, self.io.as_ref()));
        }
        let receipt = BeginReceipt { outcome: "streaming", stream_id: id.clone(), file_name: stream.file_name.clone(), sequence: 1, byte_count: stream.bytes };
        table.insert(id, stream);
        Ok(receipt)
    }

    /// Removes the stream and ends it with `error`, its partial file cleaned up.
    fn end_with(&self, id: &str, error: SaveError) -> SaveError {
        let mut table = self.table();
        match table.get_mut(id) {
            None => error,
            // Its writer holds the file: it ends the stream when its operation returns.
            Some(stream) if stream.file.is_none() => {
                stream.poisoned = true;
                SaveError { cleanup: "unknown", ..error }
            }
            Some(_) => {
                let stream = table.remove(id).expect("present");
                drop(table);
                ended(error, stream, self.io.as_ref())
            }
        }
    }

    /// Takes the stream's file for one operation after checking its binding; a refusal ends it.
    fn checkout(&self, webview: &str, id: &str, job_held: bool, sequence: u64) -> Result<(Box<dyn OwnedWrite>, u64), SaveError> {
        let mut table = self.table();
        let Some(stream) = table.get_mut(id) else { return Err(stream_failure("STREAM_UNKNOWN")) };
        if stream.webview != webview {
            // Another webview's stream is neither written nor ended by this one.
            return Err(stream_failure("STREAM_WEBVIEW_MISMATCH"));
        }
        let refusal = if stream.file.is_none() {
            Some("STREAM_BUSY")
        } else if !job_held {
            Some("STREAM_JOB_RELEASED")
        } else if sequence != stream.chunks {
            Some("STREAM_SEQUENCE_INVALID")
        } else {
            None
        };
        if let Some(code) = refusal {
            drop(table);
            return Err(self.end_with(id, stream_failure(code)));
        }
        Ok((stream.file.take().expect("checked"), stream.bytes))
    }

    /// Returns the file after an operation; a stream poisoned meanwhile is ended.
    fn checkin(&self, id: &str, file: Box<dyn OwnedWrite>, chunks: u64, bytes: u64) -> Result<(), SaveError> {
        let mut table = self.table();
        let stream = table.get_mut(id).expect("a checked-out stream stays in the table");
        stream.file = Some(file);
        if stream.poisoned {
            drop(table);
            return Err(self.end_with(id, stream_failure("STREAM_BUSY")));
        }
        stream.chunks = chunks;
        stream.bytes = bytes;
        Ok(())
    }

    /// The job a stream is bound to (for the command's held-result check).
    pub(crate) fn job_of(&self, id: &str) -> Option<String> {
        self.table().get(id).map(|stream| stream.job_id.clone())
    }

    /// Appends chunk `sequence` (the count written so far).
    pub(crate) fn append(&self, webview: &str, id: &str, job_held: bool, sequence: u64, chunk: ChunkRequest) -> Result<AppendReceipt, SaveError> {
        let checks = chunk_evidence(&chunk.screening, &chunk.local_first).and_then(|()| chunk_text(&chunk.text, false));
        if let Err(error) = checks {
            // Only the stream's own webview may end it by a refused chunk.
            if self.table().get(id).is_some_and(|stream| stream.webview == webview && stream.file.is_some()) {
                return Err(self.end_with(id, error));
            }
            return Err(error);
        }
        let (mut file, bytes) = self.checkout(webview, id, job_held, sequence)?;
        if file.write_all(chunk.text.as_bytes()).is_err() {
            self.checkin(id, file, sequence, bytes)?;
            return Err(self.end_with(id, failure("WRITE_FAILED", "write")));
        }
        let bytes = bytes + chunk.text.len() as u64;
        self.checkin(id, file, sequence + 1, bytes)?;
        Ok(AppendReceipt { outcome: "streaming", sequence: sequence + 1, byte_count: bytes })
    }

    /// Finishes the stream: the counts the webview sent must be those written.
    pub(crate) fn finish(&self, webview: &str, id: &str, job_held: bool, sequence: u64, byte_count: u64) -> Result<SaveReceipt, SaveError> {
        let (mut file, bytes) = self.checkout(webview, id, job_held, sequence)?;
        if bytes != byte_count {
            self.checkin(id, file, sequence, bytes)?;
            return Err(self.end_with(id, stream_failure("STREAM_BYTE_COUNT_MISMATCH")));
        }
        if file.flush().is_err() {
            self.checkin(id, file, sequence, bytes)?;
            return Err(self.end_with(id, failure("FLUSH_FAILED", "flush")));
        }
        drop(file);
        let stream = self.table().remove(id).expect("a checked-out stream stays in the table");
        if stream.poisoned {
            let mut error = stream_failure("STREAM_BUSY");
            cleanup(&mut error, &stream.destination, stream.owned, self.io.as_ref());
            return Err(error);
        }
        Ok(SaveReceipt {
            outcome: "saved",
            file_name: stream.file_name,
            byte_count: usize::try_from(bytes).unwrap_or(usize::MAX),
            replaced_existing: false,
            durability: "not_guaranteed",
            path_containment: "best_effort_non_adversarial",
        })
    }

    /// Aborts the webview's stream, removing its partial file.
    pub(crate) fn abort(&self, webview: &str, id: &str) -> Result<AbortReceipt, SaveError> {
        {
            let table = self.table();
            let Some(stream) = table.get(id) else { return Err(stream_failure("STREAM_UNKNOWN")) };
            if stream.webview != webview {
                return Err(stream_failure("STREAM_WEBVIEW_MISMATCH"));
            }
        }
        let ended = self.end_with(id, stream_failure("STREAM_ABORTED"));
        Ok(AbortReceipt { outcome: "aborted", partial_file_name: ended.partial_file_name, cleanup: ended.cleanup })
    }

    /// Ends every stream bound to `job_id` (its result was disposed), or every stream (`None`: the
    /// session reset), removing their partial files. Returns how many ended.
    pub(crate) fn abort_job(&self, job_id: Option<&str>) -> usize {
        let ids: Vec<String> = self.table().iter().filter(|(_, stream)| job_id.is_none_or(|job| stream.job_id == job)).map(|(id, _)| id.clone()).collect();
        for id in &ids {
            self.end_with(id, stream_failure("STREAM_JOB_RELEASED"));
        }
        ids.len()
    }
}

/// The destination of an open stream (tests).
#[cfg(test)]
pub(crate) fn destination_of(streams: &CsvStreams, id: &str) -> Option<PathBuf> {
    streams.table().get(id).map(|stream| stream.destination.clone())
}

#[cfg(test)]
#[path = "native_csv_stream_tests.rs"]
mod tests;
