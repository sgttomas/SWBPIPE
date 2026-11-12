//! The Python runner's classification, ported line for line: the two supervisors apply the same
//! rules. This supervisor also has a reading the Python's has not on a killed run, the driver's
//! lifetime-maximum footprint read at its exit (`rss.exit_lifetime_max_phys_footprint_bytes`;
//! the Python's `/usr/bin/time` is killed with the driver's group), and counts it as the stricter
//! observation. So one boundary case can be classed differently: an emergency RSS-cap kill whose
//! sampled footprint was within the budget but whose exit reading is above it is
//! `exceeds_base_machine_budget` here and `killed_by_rss_cap` in the Python. Each record names
//! the readings its classification used (`classification_readings`). Each function names its
//! source in `core/solver/performance_harness/runner/release_runner.py` (`rr`) or `k6_runner.py`
//! (`k6`). The records are `serde_json` values with the Python records' keys, so that each line
//! here can be read against the Python line it ports.

use serde_json::{json, Value};

/// `rr.SOLVED`.
pub const SOLVED: &str = "MECHANICS_SOLVED";
/// `rr.CHECKS_REPORTED`: a `--model-checks` run whose checks returned a report.
pub const CHECKS_REPORTED: &str = "MODEL_CHECKS_REPORTED";
/// `rr.PROGRESS_PREFIX`: the driver's stderr progress lines.
pub const PROGRESS_PREFIX: &str = "release-timing-stage ";
/// `rr.CLASSES`.
pub const CLASSES: [&str; 10] = [
    "ok",
    "refused",
    "timed_out",
    "exceeds_base_machine_budget",
    "killed_by_rss_cap",
    "entry_error",
    "error",
    "skipped_window_budget",
    "model_missing",
    "model_mismatch",
];
/// `k6.HEAP_CAP_MARKER`.
pub const HEAP_CAP_MARKER: &str = "k6_observe: heap cap refused ";

/// A JSON integer (Python `isinstance(v, int) and not isinstance(v, bool)`).
pub fn int(value: Option<&Value>) -> Option<i128> {
    match value? {
        Value::Number(n) => n.as_i64().map(i128::from).or_else(|| n.as_u64().map(i128::from)),
        _ => None,
    }
}

/// Python's `'%s' % value` for the values a record holds: `None`, an integer, a string.
pub fn py_str(value: Option<&Value>) -> String {
    match value {
        None | Some(Value::Null) => "None".to_string(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Bool(b)) => if *b { "True" } else { "False" }.to_string(),
        Some(other) => other.to_string(),
    }
}

fn get<'a>(value: Option<&'a Value>, key: &str) -> Option<&'a Value> {
    value.and_then(|v| v.get(key)).filter(|v| !v.is_null())
}

/// A dict member that is a non-empty object, or none (Python `x.get(k) or {}`).
fn object<'a>(value: Option<&'a Value>, key: &str) -> Option<&'a Value> {
    get(value, key).filter(|v| v.as_object().is_some_and(|o| !o.is_empty()))
}

/// `rr.peak_footprint`: the largest phys_footprint reading seen. One key is added to the Python
/// list: `rss.exit_lifetime_max_phys_footprint_bytes`, the child's lifetime maximum read at its
/// exit before it is reaped, which stands where `/usr/bin/time -l`'s peak memory footprint
/// (`rss.time_peak_footprint_bytes`, null here) stands in the Python.
pub fn peak_footprint(driver: Option<&Value>, launch_record: Option<&Value>, progress_memory: Option<&Value>) -> Option<i128> {
    let sampler = object(launch_record, "sampler");
    let rss = object(launch_record, "rss");
    let mut values = vec![
        int(get(sampler, "peak_phys_footprint_bytes")),
        int(get(sampler, "peak_lifetime_max_phys_footprint_bytes")),
        int(get(rss, "time_peak_footprint_bytes")),
        int(get(rss, "exit_lifetime_max_phys_footprint_bytes")),
    ];
    if driver.is_some_and(truthy) {
        values.push(int(get(driver, "worker_peak_footprint_bytes_equivalent")));
    }
    if progress_memory.is_some_and(truthy) {
        values.push(int(get(progress_memory, "phys_footprint_bytes")));
        values.push(int(get(progress_memory, "lifetime_max_phys_footprint_bytes")));
    }
    values.into_iter().flatten().max()
}

/// `rr.footprint_readings`: the readings [`peak_footprint`] found, each named `source:key`
/// (`sampler`, `rss`, `driver`, `progress`), in its order.
pub fn footprint_readings(driver: Option<&Value>, launch_record: Option<&Value>, progress_memory: Option<&Value>) -> Vec<String> {
    let sampler = object(launch_record, "sampler");
    let rss = object(launch_record, "rss");
    let mut found = Vec::new();
    let mut check = |source: &str, value: Option<&Value>, key: &str| {
        if int(get(value, key)).is_some() {
            found.push(format!("{source}:{key}"));
        }
    };
    check("sampler", sampler, "peak_phys_footprint_bytes");
    check("sampler", sampler, "peak_lifetime_max_phys_footprint_bytes");
    check("rss", rss, "time_peak_footprint_bytes");
    check("rss", rss, "exit_lifetime_max_phys_footprint_bytes");
    if driver.is_some_and(truthy) {
        check("driver", driver, "worker_peak_footprint_bytes_equivalent");
    }
    if progress_memory.is_some_and(truthy) {
        check("progress", progress_memory, "phys_footprint_bytes");
        check("progress", progress_memory, "lifetime_max_phys_footprint_bytes");
    }
    found
}

/// `rr.classification_readings`: the footprint readings [`classify`] compared with the budget to
/// reach its classification, in its branch order (none where no footprint decided it: a timeout,
/// no driver line, an entry error or a failed exit).
pub fn classification_readings(
    launch_record: &Value,
    driver: Option<&Value>,
    progress_memory: Option<&Value>,
) -> Vec<String> {
    let launch = Some(launch_record);
    let k6_class = get(launch, "classification").and_then(Value::as_str);
    if k6_class == Some("killed_by_sampler") {
        let kill = int(get(object(launch, "sampler"), "kill_footprint_bytes"));
        return kill.map(|_| vec!["sampler:kill_footprint_bytes".to_string()]).unwrap_or_default();
    }
    if matches!(k6_class, Some("killed_by_rss_watchdog" | "rlimit_abort" | "heap_cap_abort")) {
        return footprint_readings(driver, launch, progress_memory);
    }
    if k6_class == Some("timed_out") {
        return Vec::new();
    }
    let Some(driver) = driver else { return Vec::new() };
    if get(Some(driver), "status").and_then(Value::as_str) == Some("entry_error") || int(get(launch, "exit_code")) != Some(0) {
        return Vec::new();
    }
    footprint_readings(Some(driver), launch, progress_memory)
}

/// The record's `classification_readings`: `used`, the readings the classification compared
/// with the budget ([`classification_readings`]); `at_exit`, this supervisor's reading of the
/// driver at its exit (`rss:exit_lifetime_max_phys_footprint_bytes`), or null with
/// `at_exit_absent` saying why.
pub fn readings_record(launch_record: &Value, driver: Option<&Value>, progress_memory: Option<&Value>) -> Value {
    let rss = object(Some(launch_record), "rss");
    let at_exit = int(get(rss, "exit_lifetime_max_phys_footprint_bytes")).map(|_| "rss:exit_lifetime_max_phys_footprint_bytes");
    json!({
        "used": classification_readings(launch_record, driver, progress_memory),
        "at_exit": at_exit,
        "at_exit_absent": at_exit.is_none().then_some("proc_pid_rusage gave no reading of the exited driver before it was reaped"),
    })
}

/// Python truthiness of a record value.
pub fn truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(b) => *b,
        Value::Number(n) => n.as_f64().is_some_and(|f| f != 0.0),
        Value::String(s) => !s.is_empty(),
        Value::Array(a) => !a.is_empty(),
        Value::Object(o) => !o.is_empty(),
    }
}

/// `rr.classify`: (classification, detail) of one launched run.
pub fn classify(
    launch_record: &Value,
    driver: Option<&Value>,
    rss_cap_bytes: i128,
    budget_bytes: i128,
    progress_memory: Option<&Value>,
) -> (String, Option<String>) {
    let launch = Some(launch_record);
    let k6_class = get(launch, "classification").and_then(Value::as_str);
    if k6_class == Some("killed_by_sampler") {
        // The footprint watchdog: the product's own measure above the budget (worker_footprint_exceeded).
        let footprint = get(object(launch, "sampler"), "kill_footprint_bytes");
        return (
            "exceeds_base_machine_budget".into(),
            Some(format!("footprint_{}_above_budget_{}_killed", py_str(footprint), budget_bytes)),
        );
    }
    if matches!(k6_class, Some("killed_by_rss_watchdog" | "rlimit_abort" | "heap_cap_abort")) {
        // The emergency RSS cap; RSS runs above phys_footprint, so only a footprint reading above the
        // budget makes the kill `exceeds_base_machine_budget`.
        let footprint = peak_footprint(driver, launch, progress_memory);
        if let Some(f) = footprint.filter(|f| *f > budget_bytes) {
            return (
                "exceeds_base_machine_budget".into(),
                Some(format!("killed_above_rss_cap_{rss_cap_bytes}_footprint_{f}")),
            );
        }
        let reading = footprint.map_or("unread".to_string(), |f| format!("{f}_within_budget"));
        return ("killed_by_rss_cap".into(), Some(format!("killed_above_rss_cap_{rss_cap_bytes}_footprint_{reading}")));
    }
    if k6_class == Some("timed_out") {
        return ("timed_out".into(), Some(format!("after_{}_s", py_str(get(launch, "timeout_s")))));
    }
    let Some(driver) = driver else {
        return ("error".into(), Some(format!("no_driver_line_exit_{}", py_str(get(launch, "exit_code")))));
    };
    let status = get(Some(driver), "status");
    if status.and_then(Value::as_str) == Some("entry_error") {
        let error = object(Some(driver), "entry_error");
        let message: String = py_str(get(error, "message")).chars().take(300).collect();
        return ("entry_error".into(), Some(format!("{}: {}", py_str(get(error, "stage")), message)));
    }
    if int(get(launch, "exit_code")) != Some(0) {
        return ("error".into(), Some(format!("exit_code_{}", py_str(get(launch, "exit_code")))));
    }
    if let Some(f) = peak_footprint(Some(driver), launch, progress_memory).filter(|f| *f > budget_bytes) {
        return ("exceeds_base_machine_budget".into(), Some(format!("footprint_{f}_above_budget_at_result")));
    }
    if matches!(status.and_then(Value::as_str), Some(SOLVED | CHECKS_REPORTED)) {
        return ("ok".into(), None);
    }
    let codes: Vec<String> = get(Some(driver), "blocking_codes")
        .and_then(Value::as_array)
        .map(|codes| codes.iter().map(|c| py_str(Some(c))).collect())
        .unwrap_or_default();
    if !codes.is_empty() {
        return ("refused".into(), Some(codes.join(",")));
    }
    ("error".into(), Some(format!("status_{}", py_str(status))))
}

/// `rr.as_expected`: whether the outcome is the plan's expectation (an observation, not a pass/fail).
pub fn as_expected(expect: &str, classification: &str, detail: Option<&str>) -> Option<bool> {
    if expect == "solved" {
        return Some(classification == "ok");
    }
    if let Some(code) = expect.strip_prefix("refused:") {
        return Some(classification == "refused" && detail.unwrap_or("").split(',').any(|c| c == code));
    }
    if CLASSES.contains(&expect) {
        // Any plain classification name (refused, exceeds_base_machine_budget, timed_out, ...): exact.
        return Some(classification == expect);
    }
    None
}

/// `rr.progress_lines`: the driver's `release-timing-stage {json}` progress lines, in order.
pub fn progress_lines(stderr_text: &str) -> Vec<Value> {
    stderr_text
        .lines()
        .filter_map(|line| line.strip_prefix(PROGRESS_PREFIX))
        .filter_map(|json| serde_json::from_str::<Value>(json).ok())
        .collect()
}

/// `rr.last_progress` on the text of the run's stderr record (`None` when there is no record):
/// (last_stage, last_stage_memory, stages_finished).
pub fn last_progress(stderr_text: Option<&str>) -> (Value, Value, usize) {
    let Some(text) = stderr_text else {
        return (Value::Null, Value::Null, 0);
    };
    let lines = progress_lines(text);
    let Some(last) = lines.last() else {
        return (Value::Null, Value::Null, 0);
    };
    let pick = |keys: &[&str]| {
        Value::Object(keys.iter().map(|k| (k.to_string(), last.get(*k).cloned().unwrap_or(Value::Null))).collect())
    };
    (
        pick(&["repeat", "stage", "seconds"]),
        pick(&["ru_maxrss_bytes", "phys_footprint_bytes", "lifetime_max_phys_footprint_bytes"]),
        lines.len(),
    )
}

/// `k6.jsonl`: the lines that start with `{` and parse.
pub fn jsonl(text: &str) -> Vec<Value> {
    text.lines()
        .map(str::trim)
        .filter(|line| line.starts_with('{'))
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
        .collect()
}

/// `k6.classify` with `k6_protocol=False` on macOS (the Linux `rlimit_abort` branch cannot occur):
/// the launch's own classification, before a footprint-watchdog kill overrides it
/// (`k6.launch`: `if sampler_killed: classification = 'killed_by_sampler'`).
pub fn launch_class(
    killed_by_rss_watchdog: bool,
    timed_out: bool,
    exit_code: i32,
    objects: &[Value],
    stderr_text: &str,
    external_signal: Option<i32>,
) -> (String, Option<String>) {
    if killed_by_rss_watchdog {
        return ("killed_by_rss_watchdog".into(), None);
    }
    if timed_out {
        return ("timed_out".into(), None);
    }
    let summary = objects.iter().any(|o| o.get("kind").and_then(Value::as_str) == Some("summary"));
    if stderr_text.contains(HEAP_CAP_MARKER) && !summary {
        return ("heap_cap_abort".into(), None);
    }
    if exit_code == 0 {
        return ("ok".into(), None);
    }
    if let Some(signal) = external_signal {
        return ("error".into(), Some(format!("external_signal_{signal}")));
    }
    ("error".into(), Some(format!("exit_code_{exit_code}")))
}

/// `k6.sanitize`: no machine path in a record (`/(?:Users|home|private|tmp|var|opt|Volumes)/[^\s:'"]*`
/// becomes `<path>`).
pub fn sanitize(text: &str) -> String {
    const ROOTS: [&str; 7] = ["Users", "home", "private", "tmp", "var", "opt", "Volumes"];
    let bytes = text.as_bytes();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'/' {
            let rest = &text[i + 1..];
            if let Some(root) = ROOTS.iter().find(|r| rest.starts_with(**r) && rest.as_bytes().get(r.len()) == Some(&b'/')) {
                let mut j = i + 1 + root.len() + 1;
                while j < bytes.len() && !(bytes[j].is_ascii_whitespace() || bytes[j] == b':' || bytes[j] == b'\'' || bytes[j] == b'"') {
                    // `\s` in a Python str pattern also matches Unicode spaces; a path in a
                    // record is ASCII, so only ASCII whitespace ends one here.
                    j += 1;
                }
                // Keep char boundaries: advance to the next boundary if a multibyte char was entered.
                while j < bytes.len() && !text.is_char_boundary(j) {
                    j += 1;
                }
                out.push_str("<path>");
                i = j;
                continue;
            }
        }
        let ch = text[i..].chars().next().expect("char at boundary");
        out.push(ch);
        i += ch.len_utf8();
    }
    out
}
