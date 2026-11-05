//! RT-U4 C1 residual U1: a bounded diagnostic measurement of the SQLite/FTS5
//! memory on the native Save Project's write path that the per-command
//! ledger does not price (DEL-04-05 `Work/c1-u1-sqlite-measurement.md`).
//!
//! Evidence only. Nothing here charges, limits or configures anything: no
//! heap limit, no `sqlite3_config`, no change to the App. Both tests are
//! `#[ignore]`d, so no ordinary or CI run executes them.
//!
//! `u1_drive_all` (needs `SWBPIPE_U1_OUT`) runs every case in its own fresh
//! child process of this test binary, one at a time: `sqlite3_status64` is
//! process-global, so each child does only its own case's SQLite work. A
//! child (`u1_measure_case`, case named by `SWBPIPE_U1_CASE`) builds its store
//! and earlier saves with the production opener and write functions, then
//! measures one Save Project write: `stored_read::open_store`,
//! `stored_read::write_transaction` (BEGIN IMMEDIATE), the production
//! `replaced_search_bytes` query, `crate::upsert_project_texts`,
//! `crate::replace_project_search` and the transaction's COMMIT, in
//! `finish_save`'s order. The high-water marks are read and reset at each of
//! those calls' boundaries, and inside them at each top-level statement's
//! start and end (SQLite's `SQLITE_TRACE_STMT` and `SQLITE_TRACE_PROFILE`
//! events for the statement the caller stepped; FTS5's nested statements do
//! not move a boundary).

use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use rusqlite::ffi;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::native_persistence::{record_size, replaced_search_bytes, Cell};
use crate::stored_read;

const MIB: usize = 1 << 20;
const CASE_ENV: &str = "SWBPIPE_U1_CASE";
const OUT_ENV: &str = "SWBPIPE_U1_OUT";
const HEAD_ENV: &str = "SWBPIPE_U1_HEAD";
const ONLY_ENV: &str = "SWBPIPE_U1_ONLY";
const WORKER: &str = "stored_read::sqlite_u1::u1_measure_case";
const RESULT_PREFIX: &str = "U1-RESULT ";
/// A fixed save time: the measurement is deterministic.
const NOW: i64 = 1_700_000_000;

// ---------------------------------------------------------------------------
// Fixtures (invented data only)
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq)]
enum Family {
    /// One short token repeated.
    Repeated,
    /// Short tokens, every one distinct (`t<variant letter><hex counter>`).
    Distinct,
    /// The many-distinct family's shortest-token extreme: every token
    /// distinct and as short as possible (bijective base-36 numerals over
    /// `a-z0-9` in length order, the variant taking one residue class mod 4
    /// so that two variants share no token).
    Short,
    /// Long tokens of non-ASCII and mixed-case letters (Latin with
    /// diacritics, Greek, Cyrillic, CJK) in a 8 / 300 / 5 000 / 40 000
    /// character cycle; the longest exceed FTS5's 32 768-byte token cap.
    Unicode,
    /// JSON resembling a stored record: an array of node objects with
    /// pseudo-random decimal coordinates.
    Json,
}

const FAMILIES: [Family; 5] = [Family::Repeated, Family::Distinct, Family::Short, Family::Unicode, Family::Json];

impl Family {
    fn name(self) -> &'static str {
        match self {
            Family::Repeated => "repeated",
            Family::Distinct => "distinct",
            Family::Short => "short",
            Family::Unicode => "unicode",
            Family::Json => "json",
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Fx {
    family: Family,
    mib: usize,
    variant: u32,
}

const fn fx(family: Family, mib: usize, variant: u32) -> Fx {
    Fx { family, mib, variant }
}

impl Fx {
    fn label(&self) -> String {
        format!("{}-{}MiB-v{}", self.family.name(), self.mib, self.variant)
    }
}

/// xorshift64, seeded per family and variant.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }
}

const UNICODE_ALPHABET: [char; 36] = [
    'a', 'e', 'é', 'ü', 'ñ', 'å', 'ø', 'ß', 'A', 'É', 'Ü', 'Ñ', 'α', 'β', 'γ', 'δ', 'ω', 'Α', 'Β', 'Ω', 'а', 'б', 'в', 'ж', 'я', 'А', 'Ж', 'Я', '中', '文',
    '字', '漢', '語', 'ś', 'ł', 'ž',
];
const UNICODE_TOKEN_CHARS: [usize; 4] = [8, 300, 5_000, 40_000];

/// The fixture's text, exactly `mib` MiB long (padded with ASCII spaces).
fn fixture(fx: Fx) -> String {
    let bytes = fx.mib * MIB;
    let mut text = String::with_capacity(bytes + 64);
    match fx.family {
        Family::Repeated => {
            let token = ["load ", "beam ", "node ", "pipe ", "span "][fx.variant as usize % 5];
            while text.len() + token.len() <= bytes {
                text.push_str(token);
            }
        }
        Family::Distinct => {
            let letter = char::from(b'a' + (fx.variant % 26) as u8);
            let mut counter: u64 = 0;
            loop {
                let before = text.len();
                write!(text, "t{letter}{counter:x} ").expect("a String write");
                if text.len() > bytes {
                    text.truncate(before);
                    break;
                }
                counter += 1;
            }
        }
        Family::Short => {
            const DIGITS: &[u8; 36] = b"abcdefghijklmnopqrstuvwxyz0123456789";
            let mut index = u64::from(fx.variant % 4);
            let mut token = Vec::with_capacity(16);
            loop {
                token.clear();
                let mut n = index + 1;
                while n > 0 {
                    n -= 1;
                    token.push(DIGITS[(n % 36) as usize]);
                    n /= 36;
                }
                token.reverse();
                if text.len() + token.len() + 1 > bytes {
                    break;
                }
                text.push_str(std::str::from_utf8(&token).expect("ASCII"));
                text.push(' ');
                index += 4;
            }
        }
        Family::Unicode => {
            let mut rng = Rng::new(0x5500 + u64::from(fx.variant));
            let mut token = 0usize;
            'text: loop {
                for _ in 0..UNICODE_TOKEN_CHARS[token % UNICODE_TOKEN_CHARS.len()] {
                    let c = UNICODE_ALPHABET[rng.below(UNICODE_ALPHABET.len() as u64) as usize];
                    if text.len() + c.len_utf8() + 1 > bytes {
                        break 'text;
                    }
                    text.push(c);
                }
                text.push(' ');
                token += 1;
            }
        }
        Family::Json => {
            let suffix = "]}";
            write!(text, "{{\"schema_version\":\"analysis_run.u1-fixture\",\"run_id\":\"run-u1-{}\",\"nodes\":[", fx.variant).expect("a String write");
            let mut rng = Rng::new(0xA500 + u64::from(fx.variant));
            let coordinate = |rng: &mut Rng| (rng.below(2_000_000_000_000) as f64) / 1_000_000.0 - 1_000_000.0;
            let mut node: u64 = 0;
            loop {
                let before = text.len();
                if node > 0 {
                    text.push(',');
                }
                let (x, y, z) = (coordinate(&mut rng), coordinate(&mut rng), coordinate(&mut rng));
                write!(
                    text,
                    "{{\"id\":\"N{}-{node}\",\"x\":{x},\"y\":{y},\"z\":{z},\"type\":\"node\",\"restraint\":null,\"label\":\"node {node} of run {}\"}}",
                    fx.variant, fx.variant
                )
                .expect("a String write");
                if text.len() + suffix.len() > bytes {
                    text.truncate(before);
                    break;
                }
                node += 1;
            }
            while text.len() + suffix.len() < bytes {
                text.push(' ');
            }
            text.push_str(suffix);
        }
    }
    while text.len() < bytes {
        text.push(' ');
    }
    assert_eq!(text.len(), bytes, "fixture {} has its exact length", fx.label());
    text
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes).iter().fold(String::new(), |mut hex, byte| {
        write!(hex, "{byte:02x}").expect("a String write");
        hex
    })
}

// ---------------------------------------------------------------------------
// Cases
// ---------------------------------------------------------------------------

struct Case {
    name: String,
    kind: &'static str,
    /// Committed saves before the measured one, in order: (project index,
    /// record fixture). Each is its own production save (its own connection,
    /// write transaction and commit).
    prior: Vec<(usize, Fx)>,
    /// The measured save of project 0 ("A").
    new: Fx,
}

fn cases() -> Vec<Case> {
    use Family::*;
    let mut cases = Vec::new();
    for family in FAMILIES {
        for mib in [1, 4, 16, 64] {
            cases.push(Case { name: format!("fresh/{}/{mib}", family.name()), kind: "fresh", prior: vec![], new: fx(family, mib, 1) });
        }
    }
    let pairs = [(1, 1), (4, 4), (16, 16), (64, 64), (1, 4), (4, 16), (16, 64), (1, 64), (4, 1), (16, 4), (64, 16), (64, 1)];
    for family in FAMILIES {
        for (old, new) in pairs {
            cases.push(Case {
                name: format!("replace/{}/{old}->{new}", family.name()),
                kind: "replace",
                prior: vec![(0, fx(family, old, 0))],
                new: fx(family, new, 1),
            });
        }
    }
    for (old, new) in [(fx(Json, 16, 0), fx(Distinct, 16, 1)), (fx(Distinct, 16, 0), fx(Json, 16, 1)), (fx(Distinct, 64, 0), fx(Repeated, 64, 1)), (fx(Repeated, 64, 0), fx(Distinct, 64, 1))] {
        cases.push(Case { name: format!("cross/{}->{}", old.label(), new.label()), kind: "cross", prior: vec![(0, old)], new });
    }
    let multi: [(&str, Vec<(usize, Fx)>, Fx); 6] = [
        ("M1 B json16 then A fresh json1", vec![(1, fx(Json, 16, 2))], fx(Json, 1, 1)),
        ("M2 B json16 then A json1, A 1->1", vec![(1, fx(Json, 16, 2)), (0, fx(Json, 1, 0))], fx(Json, 1, 1)),
        ("M3 A json1 then B json16, A 1->1", vec![(0, fx(Json, 1, 0)), (1, fx(Json, 16, 2))], fx(Json, 1, 1)),
        ("M4 B distinct64 then A distinct1, A 1->4", vec![(1, fx(Distinct, 64, 2)), (0, fx(Distinct, 1, 0))], fx(Distinct, 4, 1)),
        ("M5 B json16, A json4, C json16, A 4->4", vec![(1, fx(Json, 16, 2)), (0, fx(Json, 4, 0)), (2, fx(Json, 16, 3))], fx(Json, 4, 1)),
        ("M6 A distinct16 then B distinct16, A 16->16", vec![(0, fx(Distinct, 16, 0)), (1, fx(Distinct, 16, 2))], fx(Distinct, 16, 1)),
    ];
    for (name, prior, new) in multi {
        cases.push(Case { name: format!("multi/{name}"), kind: "multi", prior, new });
    }
    let history: [(&str, Family, usize); 2] = [("H1 json4 saved 4 times, then 4->4", Json, 4), ("H2 distinct16 saved 4 times, then 16->16", Distinct, 16)];
    for (name, family, mib) in history {
        let prior = [0, 2, 3, 4].iter().map(|variant| (0, fx(family, mib, *variant))).collect();
        cases.push(Case { name: format!("history/{name}"), kind: "history", prior, new: fx(family, mib, 1) });
    }
    cases
}

// ---------------------------------------------------------------------------
// Projects and the production write calls
// ---------------------------------------------------------------------------

struct Project {
    id: String,
    name: String,
    model: String,
    record: String,
}

impl Project {
    fn new(index: usize, record: String) -> Self {
        let letter = ["A", "B", "C"][index];
        let id = format!("project:u1-{}", letter.to_lowercase());
        let name = format!("U1 project {letter}");
        let model = format!("{{\"project\":{{\"id\":\"{id}\",\"name\":\"{name}\"}},\"schema_version\":\"u1-fixture-model\"}}");
        Project { id, name, model, record }
    }

    /// The row's texts as a native save writes them: the fixture is the
    /// analysis-run record; the result slot is `null` (the result text is the
    /// project row's, priced there, and not part of the search text).
    fn texts(&self) -> crate::ProjectTexts<'_> {
        crate::ProjectTexts {
            model_json: &self.model,
            editor_intents_json: "[]",
            proposal_json: "null",
            selected_review_target_json: "null",
            mechanics_result_json: "null",
            analysis_run_json: &self.record,
            model_hash_json: "null",
            project_envelope_hash_json: "null",
        }
    }
}

/// An earlier save, as a command makes it: the store opened, a write
/// transaction, the production row writes, committed, closed.
fn prior_save(path: &Path, project: &Project) -> Result<(), String> {
    let mut connection = stored_read::open_store(path)?;
    let transaction = stored_read::write_transaction(&mut connection)?;
    let texts = project.texts();
    let search = texts.search_text();
    crate::write_project_texts(&transaction, &project.id, &project.name, &texts, &search, NOW).map_err(|error| error.to_string())?;
    transaction.commit().map_err(|error| error.to_string())
}

// ---------------------------------------------------------------------------
// SQLite's process-global memory status and the statement boundaries
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
struct Mark {
    /// The boundary that closed the interval: a harness call's end, or
    /// `before: <sql>` / `exec: <sql>` for a top-level statement's start / end.
    label: String,
    /// `SQLITE_STATUS_MEMORY_USED` current at the boundary.
    current: i64,
    /// Its high-water over the interval (reset at every boundary).
    peak: i64,
    /// `SQLITE_STATUS_MALLOC_SIZE` high-water over the interval: the largest
    /// single allocation request.
    largest_allocation: i64,
    /// `SQLITE_STATUS_MALLOC_COUNT` current and high-water.
    allocations: i64,
    peak_allocations: i64,
}

static MARKS: Mutex<Vec<Mark>> = Mutex::new(Vec::new());

struct TraceState {
    recording: bool,
    /// The top-level statement running (its pointer), 0 when none.
    outer: usize,
    outer_sql: String,
    nested_statements: u64,
    nested_sql_max: usize,
}

static TRACE: Mutex<TraceState> = Mutex::new(TraceState { recording: false, outer: 0, outer_sql: String::new(), nested_statements: 0, nested_sql_max: 0 });

fn status(op: c_int, reset: bool) -> (i64, i64) {
    let (mut current, mut high) = (0i64, 0i64);
    // SAFETY: two live out-pointers; the status call reads SQLite's global counters only.
    unsafe { ffi::sqlite3_status64(op, &mut current, &mut high, c_int::from(reset)) };
    (current, high)
}

/// Closes the interval since the previous boundary: reads every counter's
/// current and high-water, then resets the high-waters to the currents.
fn mark(label: impl Into<String>) {
    let (current, peak) = status(ffi::SQLITE_STATUS_MEMORY_USED, true);
    let (_, largest_allocation) = status(ffi::SQLITE_STATUS_MALLOC_SIZE, true);
    let (allocations, peak_allocations) = status(ffi::SQLITE_STATUS_MALLOC_COUNT, true);
    MARKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).push(Mark { label: label.into(), current, peak, largest_allocation, allocations, peak_allocations });
}

fn sql_head(sql: &str) -> String {
    sql.split_whitespace().collect::<Vec<_>>().join(" ").chars().take(64).collect()
}

/// `sqlite3_trace_v2` callback. Rust allocations here are not SQLite's, so
/// they do not move the counters read.
unsafe extern "C" fn trace(event: c_uint, _context: *mut c_void, statement: *mut c_void, detail: *mut c_void) -> c_int {
    let mut state = TRACE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    if !state.recording {
        return 0;
    }
    match event as c_int {
        ffi::SQLITE_TRACE_STMT => {
            // SAFETY: for SQLITE_TRACE_STMT the detail is the statement's NUL-terminated SQL text.
            let sql = if detail.is_null() { String::new() } else { unsafe { CStr::from_ptr(detail as *const c_char) }.to_string_lossy().into_owned() };
            if state.outer == 0 {
                state.outer = statement as usize;
                state.outer_sql = sql_head(&sql);
                let label = format!("before: {}", state.outer_sql);
                drop(state);
                mark(label);
            } else if statement as usize != state.outer {
                state.nested_statements += 1;
                state.nested_sql_max = state.nested_sql_max.max(sql.len());
            }
        }
        ffi::SQLITE_TRACE_PROFILE => {
            if state.outer != 0 && statement as usize == state.outer {
                let label = format!("exec: {}", state.outer_sql);
                state.outer = 0;
                drop(state);
                mark(label);
            }
        }
        _ => {}
    }
    0
}

fn set_trace(connection: *mut ffi::sqlite3, on: bool) {
    let mask = if on { (ffi::SQLITE_TRACE_STMT | ffi::SQLITE_TRACE_PROFILE) as c_uint } else { 0 };
    // SAFETY: a live connection handle owned by the caller; the callback only reads SQLite's global status.
    let rc = unsafe { ffi::sqlite3_trace_v2(connection, mask, if on { Some(trace) } else { None }, std::ptr::null_mut()) };
    assert_eq!(rc, ffi::SQLITE_OK, "sqlite3_trace_v2");
    TRACE.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).recording = on;
}

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!("swbpipe-u1-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory");
    dir
}

// ---------------------------------------------------------------------------
// One case, in its own process
// ---------------------------------------------------------------------------

fn fixture_row(role: &str, project: usize, fx: Fx, text: &str) -> Value {
    let project = ["A", "B", "C"][project];
    json!({
        "role": role,
        "project": project,
        "fixture": fx.label(),
        "family": fx.family.name(),
        "mib": fx.mib,
        "variant": fx.variant,
        "bytes": text.len(),
        "sha256": sha256_hex(text.as_bytes()),
    })
}

fn measure(case: &Case) -> Value {
    let dir = scratch();
    let path = dir.join("store.sqlite3");
    // Setup, before the measured window: the store created and migrated by
    // the production opener, then each earlier save.
    drop(stored_read::open_store(&path).expect("the store opens"));
    let mut fixtures = Vec::new();
    let mut prior_a_search = 0u64;
    // Each other project's stored search text: its last save's.
    let mut last: Vec<(usize, u64)> = Vec::new();
    for (index, fx) in &case.prior {
        let record = fixture(*fx);
        fixtures.push(fixture_row("prior", *index, *fx, &record));
        let project = Project::new(*index, record);
        let search_length = project.texts().search_parts().length();
        if *index == 0 {
            prior_a_search = search_length;
        } else {
            last.retain(|(other, _)| other != index);
            last.push((*index, search_length));
        }
        prior_save(&path, &project).expect("an earlier save");
    }
    let other_rows: Vec<u64> = last.iter().map(|(_, length)| *length).collect();
    let record = fixture(case.new);
    fixtures.push(fixture_row("new", 0, case.new, &record));
    let project = Project::new(0, record);
    let texts = project.texts();
    let search = texts.search_text();
    assert_eq!(search.len() as u64, texts.search_parts().length());

    // The measured window.
    MARKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clear();
    mark("baseline");
    let mut connection = stored_read::open_store(&path).expect("the store opens");
    mark("open_store");
    // SAFETY: the handle of `connection`, which outlives every use below.
    let handle = unsafe { connection.handle() };
    set_trace(handle, true);
    let transaction = stored_read::write_transaction(&mut connection).expect("BEGIN IMMEDIATE");
    mark("write_transaction");
    let replaced = replaced_search_bytes(&transaction, &project.id).expect("the replaced search row's bytes");
    mark("replaced_search_bytes");
    crate::upsert_project_texts(&transaction, &project.id, &project.name, &texts, NOW).expect("the project row");
    mark("upsert_project_texts");
    crate::replace_project_search(&transaction, &project.id, &project.name, &search).expect("the search row");
    mark("replace_project_search");
    transaction.commit().expect("COMMIT");
    mark("commit");
    set_trace(handle, false);
    let (mut cache_used, mut cache_high) = (0 as c_int, 0 as c_int);
    // SAFETY: the live handle and two out-pointers; DBSTATUS_CACHE_USED reads the pager's counters.
    unsafe { ffi::sqlite3_db_status(handle, ffi::SQLITE_DBSTATUS_CACHE_USED, &mut cache_used, &mut cache_high, 0) };
    drop(connection);
    mark("close");
    let marks = MARKS.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).clone();
    let (nested_statements, nested_sql_max) = {
        let state = TRACE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        (state.nested_statements, state.nested_sql_max)
    };

    // After the window: the index's shape, read by a separate connection.
    let after = {
        let connection = stored_read::open_store(&path).expect("the store opens");
        let data: (i64, i64) = connection.query_row("SELECT count(*), coalesce(sum(length(block)), 0) FROM local_project_fts_data", [], |row| Ok((row.get(0)?, row.get(1)?))).expect("fts data");
        let segments: i64 = connection.query_row("SELECT count(DISTINCT segid) FROM local_project_fts_idx", [], |row| row.get(0)).expect("fts idx");
        let content: i64 = connection.query_row("SELECT count(*) FROM local_project_fts_content", [], |row| row.get(0)).expect("fts content");
        json!({ "fts_data_rows": data.0, "fts_data_bytes": data.1, "fts_segments": segments, "fts_content_rows": content })
    };
    let _ = std::fs::remove_dir_all(&dir);

    let at = |label: &str| marks.iter().position(|mark| mark.label == label).unwrap_or_else(|| panic!("boundary {label}"));
    let (baseline, open, begin, replaced_at, upsert, replace, commit, close) = (
        at("baseline"),
        at("open_store"),
        at("write_transaction"),
        at("replaced_search_bytes"),
        at("upsert_project_texts"),
        at("replace_project_search"),
        at("commit"),
        at("close"),
    );
    let peak = |from: usize, to: usize| marks[from + 1..=to].iter().map(|mark| mark.peak).max().unwrap_or(i64::MIN);
    // The FTS delete phase runs from the upsert's return to the DELETE's end
    // (it includes the statements FTS5 itself runs while the DELETE is
    // prepared); the insert phase from there to `replace_project_search`'s return.
    let delete_end = upsert + 1 + marks[upsert + 1..=replace].iter().position(|mark| mark.label.starts_with("exec: DELETE FROM local_project_fts")).expect("the DELETE's end");
    let upsert_peak = peak(replaced_at, upsert);
    let delete_peak = peak(upsert, delete_end);
    let insert_peak = peak(delete_end, replace);
    let commit_peak = peak(replace, commit);
    let replace_start = marks[upsert].current;
    let id_name = (project.id.len() + project.name.len()) as u64;
    let search_row = record_size(&[Cell::Null, Cell::Text(project.id.len() as u64), Cell::Text(project.name.len() as u64), Cell::Text(search.len() as u64)]);
    let ledger_search = id_name + search.len() as u64 + search_row + replaced;
    let delta_replace = delete_peak.max(insert_peak) - replace_start;
    let delta_commit = commit_peak - replace_start;
    let excess_replace = delta_replace - ledger_search as i64;
    let memstatus = marks[open].current > 0 && marks[open].allocations > 0;
    json!({
        "case": case.name,
        "kind": case.kind,
        "memstatus_enabled": memstatus,
        "fixtures": fixtures,
        "search_bytes": search.len(),
        "replaced_search_bytes": replaced,
        "prior_a_search_bytes": prior_a_search,
        "other_rows_search_bytes": other_rows,
        "summary": if memstatus { json!({
            "baseline_current": marks[baseline].current,
            "open_peak": marks[open].peak,
            "open_end": marks[open].current,
            "begin_peak": peak(open, begin),
            "replaced_query_peak": peak(begin, replaced_at),
            "upsert_start": marks[replaced_at].current,
            "upsert_peak": upsert_peak,
            "replace_start": replace_start,
            "delete_peak": delete_peak,
            "insert_peak": insert_peak,
            "replace_end": marks[replace].current,
            "commit_peak": commit_peak,
            "commit_end": marks[commit].current,
            "close_end": marks[close].current,
            "window_peak": marks[baseline + 1..=commit].iter().map(|mark| mark.peak).max(),
            "largest_allocation": marks[baseline + 1..=commit].iter().map(|mark| mark.largest_allocation).max(),
            "ledger_search_copies": ledger_search,
            "delta_replace": delta_replace,
            "excess_replace": excess_replace,
            "delta_commit": delta_commit,
            "u1": excess_replace.max(delta_commit),
            "page_cache_used_after_commit": cache_used,
        }) } else { json!("unavailable evidence: SQLITE_STATUS_MEMORY_USED read zero") },
        "trace": { "nested_statements": nested_statements, "nested_sql_max_bytes": nested_sql_max },
        "after": after,
        "marks": marks,
    })
}

fn sqlite_value(value: rusqlite::types::Value) -> Value {
    use rusqlite::types::Value as V;
    match value {
        V::Null => Value::Null,
        V::Integer(integer) => json!(integer),
        V::Real(real) => json!(real),
        V::Text(text) => json!(text),
        V::Blob(blob) => json!(format!("<{} byte blob>", blob.len())),
    }
}

/// The linked SQLite and the production connection's settings.
fn probe() -> Value {
    let dir = scratch();
    let path = dir.join("probe.sqlite3");
    let connection = stored_read::open_store(&path).expect("the store opens");
    let (current, _) = status(ffi::SQLITE_STATUS_MEMORY_USED, false);
    let (allocations, _) = status(ffi::SQLITE_STATUS_MALLOC_COUNT, false);
    // SAFETY: SQLite's static NUL-terminated version strings.
    let (version, source_id) = unsafe { (CStr::from_ptr(ffi::sqlite3_libversion()).to_string_lossy().into_owned(), CStr::from_ptr(ffi::sqlite3_sourceid()).to_string_lossy().into_owned()) };
    // SAFETY: plain queries of the linked library.
    let (version_number, threadsafe) = unsafe { (ffi::sqlite3_libversion_number(), ffi::sqlite3_threadsafe()) };
    let mut pragmas = serde_json::Map::new();
    for name in ["page_size", "cache_size", "cache_spill", "journal_mode", "synchronous", "wal_autocheckpoint", "temp_store", "mmap_size", "locking_mode", "foreign_keys", "user_version"] {
        let value = connection.query_row(&format!("PRAGMA {name}"), [], |row| row.get::<_, rusqlite::types::Value>(0)).map(sqlite_value).unwrap_or_else(|error| json!(format!("error: {error}")));
        pragmas.insert(name.to_string(), value);
    }
    let mut statement = connection.prepare("SELECT k, v FROM local_project_fts_config ORDER BY k").expect("fts config");
    let fts_config: Vec<Value> = statement.query_map([], |row| Ok(json!([sqlite_value(row.get(0)?), sqlite_value(row.get(1)?)]))).expect("fts config rows").collect::<Result<_, _>>().expect("fts config rows");
    drop(statement);
    let compile_options = crate::sqlite_compile_options(&connection).expect("compile options");
    let fts5 = crate::fts5_available(&connection);
    drop(connection);
    let _ = std::fs::remove_dir_all(&dir);
    json!({
        "case": "probe",
        "sqlite_libversion": version,
        "sqlite_libversion_number": version_number,
        "sqlite_sourceid": source_id,
        "sqlite_threadsafe": threadsafe,
        "memory_used_after_open": current,
        "malloc_count_after_open": allocations,
        "memstatus_enabled": current > 0 && allocations > 0,
        "fts5_available": fts5,
        "production_connection_pragmas": pragmas,
        "local_project_fts_config_rows": fts_config,
        "compile_options": compile_options,
    })
}

#[test]
#[ignore = "RT-U4 C1 U1 diagnostic: one case per fresh process, spawned by u1_drive_all"]
fn u1_measure_case() {
    let Ok(name) = std::env::var(CASE_ENV) else {
        return;
    };
    let result = if name == "probe" {
        probe()
    } else {
        let case = cases().into_iter().find(|case| case.name == name).unwrap_or_else(|| panic!("no case {name}"));
        measure(&case)
    };
    println!("{RESULT_PREFIX}{result}");
}

// ---------------------------------------------------------------------------
// The driver and the generated report
// ---------------------------------------------------------------------------

#[test]
#[ignore = "RT-U4 C1 U1 diagnostic measurement driver (needs SWBPIPE_U1_OUT)"]
fn u1_drive_all() {
    let Some(out) = std::env::var_os(OUT_ENV) else {
        eprintln!("u1: set {OUT_ENV} to the generated report's path");
        return;
    };
    let only = std::env::var(ONLY_ENV).ok();
    let exe = std::env::current_exe().expect("the test binary");
    let names: Vec<String> = std::iter::once("probe".to_string())
        .chain(cases().into_iter().map(|case| case.name))
        .filter(|name| name == "probe" || only.as_deref().map_or(true, |filter| name.contains(filter)))
        .collect();
    let mut raw = Vec::new();
    for name in names {
        // One child at a time, each a fresh process doing only this case's SQLite work.
        let output = Command::new(&exe)
            .args(["--ignored", "--exact", WORKER, "--nocapture", "--test-threads=1", "-q"])
            .env(CASE_ENV, &name)
            .env_remove(OUT_ENV)
            .output()
            .expect("the case's process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(output.status.success(), "case {name} failed: {}\n{}", output.status, String::from_utf8_lossy(&output.stderr));
        let line = stdout.lines().find_map(|line| line.strip_prefix(RESULT_PREFIX)).unwrap_or_else(|| panic!("case {name} printed no result"));
        eprintln!("u1: {name}: done");
        raw.push(line.to_string());
    }
    let head = std::env::var(HEAD_ENV).unwrap_or_else(|_| "(not given)".to_string());
    std::fs::write(&out, render(&raw, &head)).expect("the report");
}

fn number(value: &Value) -> i64 {
    value.as_i64().unwrap_or_else(|| value.as_u64().map_or(0, |unsigned| unsigned as i64))
}

fn render(raw: &[String], head: &str) -> String {
    let values: Vec<Value> = raw.iter().map(|line| serde_json::from_str(line).expect("a result line")).collect();
    let mut out = String::new();
    let w = &mut out;
    writeln!(w, "<!-- Generated by `stored_read::sqlite_u1::u1_drive_all` (apps/desktop/src-tauri/src/stored_read_sqlite_u1.rs) at measured head {head}. Do not edit. -->").unwrap();
    writeln!(w).unwrap();
    let probe = &values[0];
    writeln!(w, "#### G1. Linked SQLite and the production connection (probe process)").unwrap();
    writeln!(w).unwrap();
    for key in ["sqlite_libversion", "sqlite_libversion_number", "sqlite_sourceid", "sqlite_threadsafe", "fts5_available", "memstatus_enabled", "memory_used_after_open", "malloc_count_after_open"] {
        writeln!(w, "- `{key}`: `{}`", probe[key]).unwrap();
    }
    writeln!(w, "- production connection pragmas (after `stored_read::open_store`): `{}`", probe["production_connection_pragmas"]).unwrap();
    writeln!(w, "- `local_project_fts_config` rows (FTS5 options set by the store; none means every FTS5 default): `{}`", probe["local_project_fts_config_rows"]).unwrap();
    writeln!(w, "- `PRAGMA compile_options` (via `crate::sqlite_compile_options`):").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "```text").unwrap();
    for option in probe["compile_options"].as_array().into_iter().flatten() {
        writeln!(w, "{}", option.as_str().unwrap_or_default()).unwrap();
    }
    writeln!(w, "```").unwrap();
    writeln!(w).unwrap();

    writeln!(w, "#### G2. Fixtures (generator `fixture()` in the module above; deterministic)").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "| fixture | family | MiB | variant | bytes | SHA-256 |").unwrap();
    writeln!(w, "|---|---|---:|---:|---:|---|").unwrap();
    let mut seen = std::collections::BTreeMap::new();
    for value in &values[1..] {
        for fixture in value["fixtures"].as_array().into_iter().flatten() {
            seen.entry(fixture["fixture"].as_str().unwrap_or_default().to_string()).or_insert_with(|| fixture.clone());
        }
    }
    for fixture in seen.values() {
        writeln!(w, "| {} | {} | {} | {} | {} | `{}` |", fixture["fixture"].as_str().unwrap_or_default(), fixture["family"].as_str().unwrap_or_default(), fixture["mib"], fixture["variant"], fixture["bytes"], fixture["sha256"].as_str().unwrap_or_default()).unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "#### G3. Per case: `SQLITE_STATUS_MEMORY_USED` (bytes)").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "|S| is the new search text, |S'| the replaced search row's slots (`replaced_search_bytes`), T the other projects' stored search texts. Phase high-waters are absolute (process-global current plus the phase's rise); `live end` is the current after COMMIT returned and after the connection closed.").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "| case | \\|S\\| | \\|S'\\| | T | baseline | open end | upsert hw | FTS delete hw | FTS insert hw | live after replace | COMMIT hw | live after COMMIT | live after close |").unwrap();
    writeln!(w, "|---|---:|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|").unwrap();
    for value in &values[1..] {
        let s = &value["summary"];
        if !s.is_object() {
            writeln!(w, "| {} | {} | {} | | {} | | | | | | | | |", value["case"].as_str().unwrap_or_default(), value["search_bytes"], value["replaced_search_bytes"], s).unwrap();
            continue;
        }
        let others = value["other_rows_search_bytes"].as_array().map(|rows| rows.iter().map(|row| row.to_string()).collect::<Vec<_>>().join(" + ")).unwrap_or_default();
        writeln!(
            w,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            value["case"].as_str().unwrap_or_default(),
            value["search_bytes"],
            value["replaced_search_bytes"],
            if others.is_empty() { "0".to_string() } else { others },
            s["baseline_current"],
            s["open_end"],
            s["upsert_peak"],
            s["delete_peak"],
            s["insert_peak"],
            s["replace_end"],
            s["commit_peak"],
            s["commit_end"],
            s["close_end"]
        )
        .unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "#### G4. Per case: rise over the replace's start and the ledger's search charge (bytes)").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "R0 = current when `replace_project_search` began. Δrep = max(FTS delete hw, FTS insert hw) − R0. Ledger = `finish_save`'s \"SQLite's copies of the project's search row\" reservation (id + name + |S| + search-row record + |S'|), held only while `replace_project_search` runs. Excess = Δrep − ledger. Δcommit = COMMIT hw − R0 (the ledger holds no SQLite reservation for COMMIT). U1 = max(excess, Δcommit). Largest = the largest single SQLite allocation request in the window.").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "| case | R0 | Δrep | ledger | excess | live after replace − R0 | Δcommit | U1 | U1/\\|S\\| | largest | page cache after COMMIT | FTS segments after |").unwrap();
    writeln!(w, "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|").unwrap();
    for value in &values[1..] {
        let s = &value["summary"];
        if !s.is_object() {
            continue;
        }
        let search = number(&value["search_bytes"]).max(1) as f64;
        writeln!(
            w,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {:.3} | {} | {} | {} |",
            value["case"].as_str().unwrap_or_default(),
            s["replace_start"],
            s["delta_replace"],
            s["ledger_search_copies"],
            s["excess_replace"],
            number(&s["replace_end"]) - number(&s["replace_start"]),
            s["delta_commit"],
            s["u1"],
            number(&s["u1"]) as f64 / search,
            s["largest_allocation"],
            s["page_cache_used_after_commit"],
            value["after"]["fts_segments"]
        )
        .unwrap();
    }
    writeln!(w).unwrap();

    writeln!(w, "#### G5. Empirical fits over the measured cases only").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "For each base X and constant k, the least c with U1 ≤ c·X + k for every measured case (no margin added; not a bound for any unmeasured text). S' = replaced search row, Tmax = largest other stored search text.").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "| base X | k = 0 | k = 1 MiB | k = 4 MiB | k = 8 MiB |").unwrap();
    writeln!(w, "|---|---:|---:|---:|---:|").unwrap();
    type Base = fn(&Value) -> i64;
    let bases: [(&str, Base); 4] = [
        ("\\|S\\|", |v| number(&v["search_bytes"])),
        ("\\|S\\| + \\|S'\\|", |v| number(&v["search_bytes"]) + number(&v["replaced_search_bytes"])),
        ("max(\\|S\\|, \\|S'\\|)", |v| number(&v["search_bytes"]).max(number(&v["replaced_search_bytes"]))),
        ("\\|S\\| + \\|S'\\| + Tmax", |v| {
            number(&v["search_bytes"]) + number(&v["replaced_search_bytes"]) + v["other_rows_search_bytes"].as_array().into_iter().flatten().map(number).max().unwrap_or(0)
        }),
    ];
    let measured: Vec<&Value> = values[1..].iter().filter(|value| value["summary"].is_object()).collect();
    for (name, base) in bases {
        let mut row = format!("| {name} |");
        for k in [0i64, MIB as i64, 4 * MIB as i64, 8 * MIB as i64] {
            let c = measured.iter().map(|value| (number(&value["summary"]["u1"]) - k) as f64 / base(value).max(1) as f64).fold(f64::MIN, f64::max);
            write!(row, " {c:.3} |").unwrap();
        }
        writeln!(w, "{row}").unwrap();
    }
    writeln!(w).unwrap();
    writeln!(w, "Per family (base \\|S\\| + \\|S'\\| + Tmax, k = 4 MiB): least c over that family's cases (a cross case counts for its new text's family).").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "| family | cases | least c |").unwrap();
    writeln!(w, "|---|---:|---:|").unwrap();
    for family in FAMILIES {
        let members: Vec<&&Value> = measured.iter().filter(|value| value["fixtures"].as_array().and_then(|rows| rows.last()).map_or(false, |row| row["family"] == family.name())).collect();
        let c = members
            .iter()
            .map(|value| {
                let base = number(&value["search_bytes"]) + number(&value["replaced_search_bytes"]) + value["other_rows_search_bytes"].as_array().into_iter().flatten().map(number).max().unwrap_or(0);
                (number(&value["summary"]["u1"]) - 4 * MIB as i64) as f64 / base.max(1) as f64
            })
            .fold(f64::MIN, f64::max);
        if members.is_empty() {
            writeln!(w, "| {} | 0 | — |", family.name()).unwrap();
        } else {
            writeln!(w, "| {} | {} | {c:.3} |", family.name(), members.len()).unwrap();
        }
    }
    writeln!(w).unwrap();

    writeln!(w, "#### G6. Raw child output (verbatim result lines, one per process)").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "Each line's `marks` are every boundary in order: `label` closed the interval since the previous mark; `peak` and `largest_allocation` are that interval's high-waters, reset at the mark.").unwrap();
    writeln!(w).unwrap();
    writeln!(w, "```json").unwrap();
    for line in raw {
        writeln!(w, "{line}").unwrap();
    }
    writeln!(w, "```").unwrap();
    out
}
