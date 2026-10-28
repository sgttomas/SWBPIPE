//! Byte parity of the Rust importer with `core/handoff/caepipe_mbf/importer.py` (DEL-17-04 Design
//! `desktop-mbf-export-import.md` §2, §6 slice 3).
//!
//! `fixtures/caepipe_mbf/invented/parity/import_vectors.json` is written by `tools/fixtures/caepipe_mbf_import_vectors.py`
//! from the Python over its own test suites and the committed invented MBF corpus, and the Python replays it in
//! `tests/test_caepipe_mbf_import_parity.py`. Here every recorded call is replayed through the Rust port: the model,
//! the import report and the id map must be the Python's bytes (their digests, and the full text for corpus items),
//! or the Python's exception. The writer's regeneration of each bound sidecar is the recorded one (the writer port is
//! slice 2), keyed by the model's canonical bytes, so a model that differs by one byte finds no regeneration.

use open_pipe_stress_caepipe_mbf::{
    self as mbf, ports, Dict, ImportError, ImportOptions, ImportResult, Reference, Regenerated, SidecarInput,
    SidecarRegenerator, Value,
};
use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const NONFINITE: &str = "\u{0}float";

fn project() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn document() -> &'static Value {
    static DOCUMENT: OnceLock<Value> = OnceLock::new();
    DOCUMENT.get_or_init(|| {
        let bytes = std::fs::read(project().join("fixtures/caepipe_mbf/invented/parity/import_vectors.json")).expect("vectors");
        mbf::parse_sidecar_json(&bytes).expect("the vectors are strict JSON")
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

fn b64(text: &str) -> Vec<u8> {
    let table = |c: u8| -> u32 {
        match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a' + 26) as u32,
            b'0'..=b'9' => (c - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => panic!("base64 {c}"),
        }
    };
    let clean: Vec<u8> = text.bytes().filter(|byte| *byte != b'=').collect();
    let mut out = Vec::new();
    for chunk in clean.chunks(4) {
        let mut acc = 0u32;
        for (index, byte) in chunk.iter().enumerate() {
            acc |= table(*byte) << (18 - 6 * index);
        }
        let bytes = [(acc >> 16) as u8, (acc >> 8) as u8, acc as u8];
        out.extend_from_slice(&bytes[..chunk.len() - 1]);
    }
    out
}

fn f64_bits(text: &str) -> f64 {
    let bytes: Vec<u8> = (0..8).map(|index| u8::from_str_radix(&text[2 * index..2 * index + 2], 16).expect("hex")).collect();
    f64::from_le_bytes(bytes.try_into().expect("8 bytes"))
}

fn f64_hex(value: f64) -> String {
    hex(&value.to_le_bytes())
}

struct Inputs<'d> {
    doc: &'d Value,
    blobs: HashMap<String, Vec<u8>>,
    values: HashMap<String, Value>,
}

impl<'d> Inputs<'d> {
    fn new(doc: &'d Value) -> Self {
        Inputs { doc, blobs: HashMap::new(), values: HashMap::new() }
    }

    fn blob(&mut self, key: &str) -> Vec<u8> {
        if let Some(found) = self.blobs.get(key) {
            return found.clone();
        }
        let entry = self.doc.get("blobs").and_then(|blobs| blobs.get(key)).expect("blob").clone();
        let bytes = if let Some(Value::Str(text)) = entry.get("b64") {
            b64(text)
        } else if let Some(Value::Str(text)) = entry.get("text") {
            text.as_bytes().to_vec()
        } else {
            let base: Vec<char> = String::from_utf8(self.blob(entry.get_or_null("base").as_str().expect("base"))).expect("utf-8").chars().collect();
            let prefix = mbf::value::small_int(entry.get_or_null("prefix")).expect("prefix") as usize;
            let suffix = mbf::value::small_int(entry.get_or_null("suffix")).expect("suffix") as usize;
            let middle = entry.get_or_null("middle").as_str().expect("middle");
            let mut text: String = base[..prefix].iter().collect();
            text.push_str(middle);
            text.extend(base[base.len() - suffix..].iter());
            text.into_bytes()
        };
        self.blobs.insert(key.to_string(), bytes.clone());
        bytes
    }

    fn value(&mut self, key: &Value) -> Option<Value> {
        let key = key.as_str()?;
        Some(restore(&self.value_plain(key)))
    }

    fn value_plain(&mut self, key: &str) -> Value {
        if let Some(found) = self.values.get(key) {
            return found.clone();
        }
        let entry = self.doc.get("values").and_then(|values| values.get(key)).expect("value").clone();
        let value = match entry.get("value") {
            Some(value) => value.clone(),
            None => {
                let mut value = self.value_plain(entry.get_or_null("base").as_str().expect("base"));
                for op in entry.get_or_null("ops").as_list().expect("ops") {
                    apply(&mut value, op.as_list().expect("op"));
                }
                value
            }
        };
        self.values.insert(key.to_string(), value.clone());
        value
    }
}

fn apply(value: &mut Value, op: &[Value]) {
    let kind = op[0].as_str().expect("kind");
    let path = op[1].as_list().expect("path");
    if path.is_empty() {
        *value = op[2].clone();
        return;
    }
    let mut parent = value;
    for step in &path[..path.len() - 1] {
        parent = step_into(parent, step);
    }
    let last = &path[path.len() - 1];
    match (parent, last) {
        (Value::Dict(dict), Value::Str(key)) => {
            if kind == "del" {
                dict.remove(key);
            } else {
                dict.insert(key.clone(), op[2].clone());
            }
        }
        (Value::List(items), Value::Int(_)) => {
            let index = mbf::value::small_int(last).expect("index") as usize;
            items[index] = op[2].clone();
        }
        _ => panic!("an op path that does not fit its value"),
    }
}

fn step_into<'v>(value: &'v mut Value, step: &Value) -> &'v mut Value {
    match (value, step) {
        (Value::Dict(dict), Value::Str(key)) => dict.get_mut(key).expect("key"),
        (Value::List(items), Value::Int(_)) => &mut items[mbf::value::small_int(step).expect("index") as usize],
        _ => panic!("an op path that does not fit its value"),
    }
}

fn restore(value: &Value) -> Value {
    match value {
        Value::Dict(dict) => {
            if dict.len() == 1 {
                if let Some(Value::Str(kind)) = dict.get(&NONFINITE.to_string()) {
                    return Value::Float(match kind.as_str() {
                        "nan" => f64::NAN,
                        "inf" => f64::INFINITY,
                        _ => f64::NEG_INFINITY,
                    });
                }
            }
            let mut out = Dict::new();
            for (key, item) in dict.iter() {
                out.insert(key.clone(), restore(item));
            }
            Value::Dict(out)
        }
        Value::List(items) => Value::List(items.iter().map(restore).collect()),
        other => other.clone(),
    }
}

/// The writer's recorded regenerations (`regenerate_caepipe_sidecar`), by the call's identity.
struct Recorded<'d> {
    table: &'d Value,
}

impl<'d> SidecarRegenerator for Recorded<'d> {
    fn regenerate(&self, model: &Value, mbf_sha256: &str, derived: &[String], reference: Reference) -> Result<Regenerated, ImportError> {
        let model_bytes = mbf::canonical_bytes(model).map_err(ImportError::Python)?;
        let derived = ports::dumps_sorted(&Value::List(derived.iter().map(|item| Value::str(item.clone())).collect())).expect("dumps");
        let shown = match reference {
            Reference::None => "{}".to_string(),
            Reference::Celsius(value) => format!("{{\"reference_temperature_c\": \"{}\"}}", f64_hex(value)),
            Reference::Fahrenheit(value) => format!("{{\"reference_temperature_f\": \"{}\"}}", f64_hex(value)),
        };
        let key = sha(format!("{}\n{mbf_sha256}\n{derived}\n{shown}", sha(&model_bytes)).as_bytes());
        let Some(outcome) = self.table.get(&key) else {
            return Err(ImportError::Unsupported(format!("no recorded regeneration for {key}: the model differs from the Python's")));
        };
        Ok(match outcome.get("sidecar") {
            Some(sidecar) => Regenerated::Sidecar(sidecar.clone()),
            None => Regenerated::Refused(
                outcome.get_or_null("refused").as_list().expect("codes").iter().map(|code| code.as_str().expect("code").to_string()).collect(),
            ),
        })
    }
}

fn digest(result: &ImportResult) -> (String, Option<String>, String, String) {
    let model = result.model_bytes().expect("model bytes").map(|bytes| String::from_utf8(bytes).expect("ascii"));
    let report = String::from_utf8(result.report_bytes().expect("report bytes")).expect("ascii");
    let id_map = String::from_utf8(result.id_map_bytes().expect("id map bytes")).expect("ascii");
    let parts = [
        result.status.clone(),
        model.as_ref().map(|text| sha(text.as_bytes())).unwrap_or_else(|| "-".into()),
        sha(report.as_bytes()),
        sha(id_map.as_bytes()),
    ];
    (sha(parts.join("|").as_bytes())[..32].to_string(), model, report, id_map)
}

struct Replay {
    failures: Vec<String>,
    codes: BTreeSet<String>,
    labels: BTreeSet<String>,
    bound_labels: BTreeSet<String>,
    count: usize,
}

fn replay_all() -> &'static Replay {
    static REPLAY: OnceLock<Replay> = OnceLock::new();
    REPLAY.get_or_init(|| {
        let doc = document();
        let mut inputs = Inputs::new(doc);
        let regenerator = Recorded { table: doc.get("regenerations").expect("regenerations") };
        let sources = doc.get_or_null("sources").as_list().expect("sources");
        let mut replay = Replay { failures: Vec::new(), codes: BTreeSet::new(), labels: BTreeSet::new(), bound_labels: BTreeSet::new(), count: 0 };
        let out_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("caepipe_mbf_parity");
        for (index, row) in doc.get_or_null("imports").as_list().expect("imports").iter().enumerate() {
            let row = row.as_list().expect("row");
            let source = sources[mbf::value::small_int(&row[0]).expect("src") as usize].as_str().expect("source");
            let data = inputs.blob(row[1].as_str().expect("data"));
            let sidecar = match &row[2] {
                Value::Null => None,
                Value::List(parts) => {
                    let key = parts[1].as_str().expect("key");
                    Some(if parts[0].as_str() == Some("json") {
                        SidecarInput::Json(inputs.blob(key))
                    } else {
                        SidecarInput::Parsed(inputs.value(&parts[1]).expect("sidecar"))
                    })
                }
                _ => panic!("sidecar"),
            };
            let label = inputs.value(&row[7]).expect("label");
            let mut options = ImportOptions::new(label.as_str().expect("label str"));
            let bound_candidate = sidecar.is_some();
            options.sidecar = sidecar;
            options.material_provenance = inputs.value(&row[3]);
            options.pressure_closure = inputs.value(&row[4]);
            options.shear_model = inputs.value(&row[5]);
            options.rebind_by_node_number = mbf::value::small_int(&row[6]) == Some(1);
            options.pd1_disabled = mbf::value::small_int(&row[8]) == Some(1);
            options.regenerator = Some(&regenerator);
            let expect = &row[9];
            replay.count += 1;
            let outcome = mbf::import(&data, &options);
            let problem = match (outcome, expect.get("raises")) {
                (Err(ImportError::Python(error)), Some(Value::List(raised))) => {
                    let want = (raised[0].as_str().unwrap_or_default(), raised[1].as_str().unwrap_or_default());
                    if (error.kind, error.message.as_str()) == want {
                        None
                    } else {
                        Some(format!("raises {}: {}, the Python {}: {}", error.kind, error.message, want.0, want.1))
                    }
                }
                (Err(error), _) => Some(format!("returns no result: {error}")),
                (Ok(result), Some(raised)) => Some(format!("returns {}, the Python raises {raised:?}", result.status)),
                (Ok(result), None) => {
                    for item in &result.diagnostics {
                        if let Some(code) = item.get("code").and_then(Value::as_str) {
                            replay.codes.insert(code.to_string());
                        }
                    }
                    replay.labels.insert(options.source_label.clone());
                    if bound_candidate && result.source.get("binding").and_then(Value::as_str) == Some("bound") {
                        replay.bound_labels.insert(options.source_label.clone());
                    }
                    let (got, model, report, id_map) = digest(&result);
                    let mut problem = None;
                    if let Some(Value::Str(text)) = expect.get("report") {
                        let want_model = expect.get("model").and_then(Value::as_str).map(String::from);
                        if *text != report || want_model != model || expect.get_or_null("id_map").as_str() != Some(id_map.as_str()) {
                            problem = Some("full outputs differ".to_string());
                        }
                    }
                    if got != expect.get_or_null("digest").as_str().unwrap_or_default() {
                        problem = Some(format!("outputs differ ({})", result.status));
                    }
                    if problem.is_some() {
                        let _ = std::fs::create_dir_all(&out_dir);
                        let _ = std::fs::write(out_dir.join(format!("{index}.report.json")), &report);
                        let _ = std::fs::write(out_dir.join(format!("{index}.id_map.json")), &id_map);
                        if let Some(model) = &model {
                            let _ = std::fs::write(out_dir.join(format!("{index}.model.json")), model);
                        }
                    }
                    problem
                }
            };
            if let Some(problem) = problem {
                replay.failures.push(format!("vector {index} ({source}): {problem}"));
            }
        }
        replay
    })
}

#[test]
fn every_recorded_import_gives_the_pythons_bytes() {
    let replay = replay_all();
    assert!(replay.count > 9000, "the vectors hold {} imports", replay.count);
    let shown: Vec<&String> = replay.failures.iter().take(25).collect();
    assert!(
        replay.failures.is_empty(),
        "{} of {} vectors differ (Rust outputs in target/tmp/caepipe_mbf_parity; the Python's: \
         python3 tools/fixtures/caepipe_mbf_import_vectors.py show <id>):\n{}",
        replay.failures.len(),
        replay.count,
        shown.iter().map(|item| item.as_str()).collect::<Vec<_>>().join("\n")
    );
}

#[test]
fn every_named_importer_code_is_exercised() {
    let source = std::fs::read_to_string(project().join("core/handoff/caepipe_mbf/importer.py")).expect("importer.py");
    let mut codes: BTreeSet<String> = BTreeSet::new();
    let bytes = source.as_bytes();
    let mut at = 0;
    while let Some(found) = source[at..].find("\"MBF-IMPORT-") {
        let start = at + found + 1;
        let end = start + source[start..].find('"').expect("closing quote");
        let code = &source[start..end];
        if code.bytes().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-') && !code.ends_with('-') {
            codes.insert(code.to_string());
        }
        at = end;
        let _ = bytes;
    }
    // The documented refused-comment table names its codes as dict keys too, so they are in the source as well.
    let replay = replay_all();
    let missing: Vec<&String> = codes.iter().filter(|code| !replay.codes.contains(*code)).collect();
    assert!(codes.len() > 60, "found {} codes in importer.py", codes.len());
    assert!(missing.is_empty(), "importer codes no vector exercises: {missing:?}");
}

#[test]
fn the_round_trip_reversible_and_byte_reproducible_sets_import_bound() {
    let replay = replay_all();
    let sets = document().get("sets").expect("sets");
    for name in ["reversible", "byte_reproducible"] {
        let members = sets.get_or_null(name).as_list().expect("set");
        assert!(!members.is_empty());
        for member in members {
            let label = member.as_str().expect("name");
            assert!(replay.bound_labels.contains(label), "{name} model {label} has no bound import among the vectors");
        }
    }
}

#[test]
fn documented_orientations_match() {
    let doc = document();
    let mut inputs = Inputs::new(doc);
    let mut failures = Vec::new();
    for entry in doc.get_or_null("orientations").as_list().expect("orientations") {
        let data = inputs.blob(entry.get_or_null("data").as_str().expect("data"));
        let got = mbf::documented_orientations(&data);
        match (got, entry.get("derived"), entry.get("raises")) {
            (Ok(None), Some(Value::Null), _) => {}
            (Ok(Some(derived)), Some(Value::List(want)), _) => {
                let same = derived.len() == want.len()
                    && derived.iter().zip(want).all(|(((a, b), vector), item)| {
                        let item = item.as_list().expect("item");
                        let wanted = item[2].as_list().expect("vector");
                        item[0].as_str() == Some(a.as_str())
                            && item[1].as_str() == Some(b.as_str())
                            && (0..3).all(|axis| wanted[axis].as_f64().map(f64::to_bits) == Some(vector[axis].to_bits()))
                    });
                if !same {
                    failures.push(entry.get_or_null("data").as_str().unwrap_or_default().to_string());
                }
            }
            (Err(ImportError::Python(error)), _, Some(Value::List(raised))) if raised[0].as_str() == Some(error.kind) => {}
            _ => failures.push(entry.get_or_null("data").as_str().unwrap_or_default().to_string()),
        }
    }
    assert!(failures.is_empty(), "documented orientations differ for {failures:?}");
}

fn numeric(name: &str) -> &'static Vec<Value> {
    document().get("numeric").and_then(|numeric| numeric.get(name)).and_then(Value::as_list).expect("numeric set")
}

#[test]
fn float_repr_matches_python_at_every_boundary() {
    let mut failures = Vec::new();
    for item in numeric("repr") {
        let item = item.as_list().expect("item");
        let value = f64_bits(item[0].as_str().expect("bits"));
        let want = item[1].as_str().expect("repr");
        match mbf::python_float_repr(value) {
            Ok(got) if got == want => {}
            got => failures.push(format!("{value:e}: {got:?} vs {want}")),
        }
    }
    assert!(numeric("repr").len() > 5000);
    assert!(failures.is_empty(), "{} repr mismatches: {:?}", failures.len(), &failures[..failures.len().min(10)]);
}

#[test]
fn format_g6_matches_python() {
    let failures: Vec<String> = numeric("format_g6")
        .iter()
        .filter_map(|item| {
            let item = item.as_list().expect("item");
            let value = f64_bits(item[0].as_str().expect("bits"));
            let got = mbf::python_format_g(value, 6);
            (got != item[1].as_str().expect("text")).then(|| format!("{value:e}: {got} vs {}", item[1].as_str().unwrap()))
        })
        .collect();
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn decimal_parsing_rounds_halfway_cases_to_even_as_python_does() {
    let mut failures = Vec::new();
    for item in numeric("parse") {
        let item = item.as_list().expect("item");
        let text = item[0].as_str().expect("text");
        let exact = f64_bits(item[1].as_str().expect("fraction"));
        let parsed = f64_bits(item[2].as_str().expect("float"));
        if ports::fraction_to_f64(text).map(f64::to_bits) != Some(exact.to_bits()) {
            failures.push(format!("float(Fraction({text}))"));
        }
        if text.parse::<f64>().map(f64::to_bits).ok() != Some(parsed.to_bits()) {
            failures.push(format!("float({text})"));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

fn args(item: &Value) -> Vec<f64> {
    item.as_list().expect("args").iter().map(|arg| f64_bits(arg.as_str().expect("bits"))).collect()
}

#[test]
fn python_hypot_sum_and_the_exact_norm_are_ported_bit_for_bit() {
    let mut failures = Vec::new();
    for (name, function) in [
        ("hypot", (|values: &[f64]| ports::hypot(values)) as fn(&[f64]) -> f64),
        ("sum", |values: &[f64]| ports::sum(values)),
        ("correct_norm3", |values: &[f64]| ports::correct_norm3(values[0], values[1], values[2])),
    ] {
        for item in numeric(name) {
            let item = item.as_list().expect("item");
            let inputs = args(&item[0]);
            let want = f64_bits(item[1].as_str().expect("bits"));
            let got = function(&inputs);
            if got.to_bits() != want.to_bits() && !(got.is_nan() && want.is_nan()) {
                failures.push(format!("{name}{inputs:?}: {got:e} vs {want:e}"));
            }
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn the_ninety_digit_arctangent_rounds_as_the_python() {
    let mut failures = Vec::new();
    for item in numeric("twice_atan2") {
        let item = item.as_list().expect("item");
        let (y, x) = (f64_bits(item[0].as_str().expect("y")), f64_bits(item[1].as_str().expect("x")));
        let want = item[2].as_str().map(f64_bits);
        let got = ports::twice_atan2_nonnegative(y, x);
        if got.map(f64::to_bits) != want.map(f64::to_bits) {
            failures.push(format!("({y:e}, {x:e}): {got:?} vs {want:?}"));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

/// Recorded C-library results that the platform which wrote the vectors (macOS) does not round correctly, with the
/// correctly rounded value, each confirmed independently (a 120-digit series and mpmath at 400 bits). The import that
/// made the call does not carry that last bit into its outputs: its vector passes with the correctly rounded value.
const NOT_CORRECTLY_ROUNDED_WHERE_RECORDED: &[(&str, &str)] = &[("tan:182d4452fb21f93f", "747696fbffff9f41")];

#[test]
fn the_c_library_calls_the_imports_made_are_correctly_rounded_as_recorded() {
    // CPython takes acos, asin, atan2, sin and tan from the platform C library, whose rounding differs between
    // platforms; the port evaluates them correctly rounded, the same native and in wasm. Every call the recorded
    // imports made must equal the Python's result there, or be listed above with its correctly rounded value.
    let mut failures = Vec::new();
    let mut calls = 0;
    for (key, want) in document().get("libm").and_then(Value::as_dict).expect("libm").iter() {
        let (name, rest) = key.split_once(':').expect("key");
        let inputs: Vec<f64> = rest.split(',').map(f64_bits).collect();
        let got = match name {
            "acos" => ports::acos(inputs[0]).ok(),
            "asin" => ports::asin(inputs[0]).ok(),
            "sin" => ports::sin(inputs[0]).ok(),
            "tan" => ports::tan(inputs[0]).ok(),
            "atan2" => Some(ports::atan2(inputs[0], inputs[1])),
            "hypot" => Some(ports::hypot(&inputs)),
            _ => None,
        };
        calls += usize::from(name != "hypot");
        let recorded = want.as_str().expect("bits");
        let expected = match NOT_CORRECTLY_ROUNDED_WHERE_RECORDED.iter().find(|(call, _)| call == key) {
            Some((_, correct)) => {
                assert_ne!(*correct, recorded, "{key} is listed but recorded correctly rounded");
                *correct
            }
            None => recorded,
        };
        let want = f64_bits(expected);
        if got.map(f64::to_bits) != Some(want.to_bits()) {
            failures.push(format!("{name}{inputs:?}: {got:?} vs {want:e}"));
        }
    }
    assert!(calls >= 30, "{calls} C-library calls recorded");
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn the_correctly_rounded_functions_hold_at_their_edges() {
    use std::f64::consts::{FRAC_PI_2, PI};
    assert_eq!(ports::acos(1.0).unwrap().to_bits(), 0.0f64.to_bits());
    assert_eq!(ports::acos(-1.0).unwrap(), PI);
    assert_eq!(ports::acos(0.0).unwrap(), FRAC_PI_2);
    assert!(ports::acos(1.5).is_err() && ports::asin(-1.5).is_err());
    assert_eq!(ports::asin(1.0).unwrap(), FRAC_PI_2);
    assert_eq!(ports::asin(-0.0).unwrap().to_bits(), (-0.0f64).to_bits());
    assert_eq!(ports::sin(-0.0).unwrap().to_bits(), (-0.0f64).to_bits());
    assert_eq!(ports::sin(5e-324).unwrap(), 5e-324);
    assert_eq!(ports::tan(1e-300).unwrap(), 1e-300);
    assert!(ports::tan(f64::INFINITY).is_err() && ports::sin(f64::NEG_INFINITY).is_err());
    assert_eq!(ports::atan2(1.0, 0.0), FRAC_PI_2);
    assert_eq!(ports::atan2(-1.0, -1.0), -3.0 * PI / 4.0);
    assert_eq!(ports::atan2(0.0, -1.0), PI);
    assert_eq!(ports::atan2(-0.0, 1.0).to_bits(), (-0.0f64).to_bits());
    // values mpmath (400 bits) rounds as the port does and macOS's library does not
    assert_eq!(ports::tan(0.6193291121562982).unwrap(), 0.7128966747523655);
    assert_eq!(ports::acos(-0.32021055557652045).unwrap(), 1.8967480640184062);
    assert_eq!(ports::asin(0.24201240367807075).unwrap(), 0.24443937522322975);
}

#[test]
fn strict_sidecar_json_errors_are_the_pythons() {
    let mut failures = Vec::new();
    for item in numeric("json_errors") {
        let item = item.as_list().expect("item");
        let text = item[0].as_str().expect("text");
        let bytes = match text.strip_prefix("b64:") {
            Some(encoded) => b64(encoded),
            None => text.as_bytes().to_vec(),
        };
        let want = &item[1];
        let got = mbf::parse_sidecar_json(&bytes);
        let ok = match (&got, want.get("invalid"), want.get("value"), want.get("unrepresentable")) {
            (Err(mbf::SidecarJsonError::Invalid(findings)), Some(Value::List(expected)), _, _) => {
                findings.len() == expected.len()
                    && findings.iter().zip(expected).all(|((path, reason), item)| {
                        let item = item.as_list().expect("finding");
                        item[0].as_str() == Some(path.as_str()) && item[1].as_str() == Some(reason.as_str())
                    })
            }
            (Ok(value), _, Some(Value::Str(dumped)), _) => {
                let back = restore(&mbf::parse_sidecar_json(dumped.as_bytes()).expect("dumped"));
                format!("{value:?}") == format!("{back:?}")
            }
            (Err(mbf::SidecarJsonError::LoneSurrogate), _, _, Some(_)) => true,
            _ => false,
        };
        if !ok {
            failures.push(format!("{text:?}: {got:?} vs {want:?}"));
        }
    }
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn the_model_schema_version_is_the_schema_files() {
    let schema = std::fs::read(project().join("schemas/model.schema.yaml")).expect("schema");
    let schema = mbf::parse_sidecar_json(&schema).expect("the schema file is JSON");
    let versions = schema.get_or_null("properties").get_or_null("schema_version").get_or_null("enum").as_list().expect("enum");
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].as_str(), Some(mbf::MODEL_SCHEMA_VERSION));
}
