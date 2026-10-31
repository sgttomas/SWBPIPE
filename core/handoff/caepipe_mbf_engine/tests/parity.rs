//! Byte parity of the Rust importer with `core/handoff/caepipe_mbf/importer.py` (DEL-17-04 Design
//! `desktop-mbf-export-import.md` §2, §6 slice 3).
//!
//! `fixtures/caepipe_mbf/invented/parity/import_vectors.json` is written by `tools/fixtures/caepipe_mbf_import_vectors.py`
//! from the Python over its own test suites and the committed invented MBF corpus, and the Python replays it in
//! `tests/test_caepipe_mbf_import_parity.py`. Here every recorded call is replayed through the Rust port: the model,
//! the import report and the id map must be the Python's bytes (their digests, and the full text for corpus items),
//! or the Python's exception. Each bound sidecar's regeneration is the crate's own writer (slice 2), and each is
//! checked against the Python writer's recorded regeneration of the same call, keyed by the model's canonical bytes.

use open_pipe_stress_caepipe_mbf::{self as mbf, ports, ImportError, ImportOptions, ImportResult, Reference, Regenerated, SidecarInput, SidecarRegenerator, Value};
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

mod common;
use common::*;

fn document() -> &'static Value {
    static DOCUMENT: OnceLock<Value> = OnceLock::new();
    DOCUMENT.get_or_init(|| {
        parse(&std::fs::read(project().join("fixtures/caepipe_mbf/invented/parity/import_vectors.json")).expect("vectors"))
    })
}

/// The regeneration identity's writer: the crate's own ([`mbf::SiWriter`]), checked against the Python writer's
/// recorded regeneration of the same call (`regenerate_caepipe_sidecar`), found by the call's identity: the model's
/// canonical bytes, the digest, the derived straights and the reference temperature.
struct Checked<'d> {
    table: &'d Value,
    seen: RefCell<BTreeSet<String>>,
    differences: RefCell<Vec<String>>,
}

/// Type-exact equality with dict order kept (the generator's `same`): 1 != 1.0 != True, -0.0 != 0.0.
fn same(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Dict(x), Value::Dict(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|((p, u), (q, v))| p == q && same(u, v))
        }
        (Value::List(x), Value::List(y)) => x.len() == y.len() && x.iter().zip(y).all(|(u, v)| same(u, v)),
        (Value::Float(x), Value::Float(y)) => x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan()),
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::Str(x), Value::Str(y)) => x == y,
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Null, Value::Null) => true,
        _ => false,
    }
}

impl<'d> SidecarRegenerator for Checked<'d> {
    fn regenerate(&self, model: &Value, mbf_sha256: &str, derived: &[String], reference: Reference) -> Result<Regenerated, ImportError> {
        let model_bytes = mbf::canonical_bytes(model).map_err(ImportError::Python)?;
        let listed = ports::dumps_sorted(&Value::List(derived.iter().map(|item| Value::str(item.clone())).collect())).expect("dumps");
        let shown = match reference {
            Reference::None => "{}".to_string(),
            Reference::Celsius(value) => format!("{{\"reference_temperature_c\": \"{}\"}}", f64_hex(value)),
            Reference::Fahrenheit(value) => format!("{{\"reference_temperature_f\": \"{}\"}}", f64_hex(value)),
        };
        let key = sha(format!("{}\n{mbf_sha256}\n{listed}\n{shown}", sha(&model_bytes)).as_bytes());
        let regenerated = mbf::SiWriter.regenerate(model, mbf_sha256, derived, reference)?;
        self.seen.borrow_mut().insert(key.clone());
        let difference = match (self.table.get(&key), &regenerated) {
            (None, _) => Some("no recorded regeneration: the model differs from the Python's".to_string()),
            (Some(outcome), Regenerated::Sidecar(sidecar)) => match outcome.get("sidecar") {
                Some(recorded) if same(recorded, sidecar) => None,
                Some(_) => Some("the sidecar differs".into()),
                None => Some(format!("the Rust writer regenerates a sidecar, the Python refuses ({:?})", outcome.get("refused"))),
            },
            (Some(outcome), Regenerated::Refused(codes)) => {
                let mut codes: Vec<String> = codes.clone();
                codes.sort();
                codes.dedup();
                let recorded: Option<Vec<String>> = outcome
                    .get("refused")
                    .and_then(Value::as_list)
                    .map(|items| items.iter().filter_map(|item| item.as_str().map(String::from)).collect());
                (recorded.as_ref() != Some(&codes)).then(|| format!("the Rust writer refuses ({codes:?}), the Python {recorded:?}"))
            }
        };
        if let Some(difference) = difference {
            self.differences.borrow_mut().push(format!("regeneration {}: {difference}", &key[..12]));
        }
        Ok(regenerated)
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
    regeneration_differences: Vec<String>,
    regenerations_seen: BTreeSet<String>,
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
        let regenerator = Checked {
            table: doc.get("regenerations").expect("regenerations"),
            seen: RefCell::new(BTreeSet::new()),
            differences: RefCell::new(Vec::new()),
        };
        let sources = doc.get_or_null("sources").as_list().expect("sources");
        let mut replay = Replay {
            failures: Vec::new(),
            regeneration_differences: Vec::new(),
            regenerations_seen: BTreeSet::new(),
            codes: BTreeSet::new(),
            labels: BTreeSet::new(),
            bound_labels: BTreeSet::new(),
            count: 0,
        };
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
        replay.regeneration_differences = regenerator.differences.into_inner();
        replay.regenerations_seen = regenerator.seen.into_inner();
        replay
    })
}

#[test]
fn every_recorded_regeneration_is_the_rust_writers() {
    let replay = replay_all();
    let shown: Vec<&String> = replay.regeneration_differences.iter().take(25).collect();
    assert!(replay.regeneration_differences.is_empty(), "{} regenerations differ:\n{shown:#?}", replay.regeneration_differences.len());
    let table = document().get("regenerations").and_then(Value::as_dict).expect("regenerations");
    let unmade: Vec<&String> = table.keys().filter(|key| !replay.regenerations_seen.contains(*key)).collect();
    assert!(table.len() > 100, "the vectors hold {} regenerations", table.len());
    assert!(unmade.is_empty(), "recorded regenerations the Rust replay never makes: {unmade:?}");
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

#[test]
fn the_trig_and_hypot_calls_the_imports_made_are_the_pythons() {
    // Both twins take acos, asin, atan2, sin and tan correctly rounded (the Python's `correctly_rounded`), so every
    // call the recorded imports made gives the Python's bits, natively and in wasm; hypot is CPython's algorithm.
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
            "atan2" => ports::atan2(inputs[0], inputs[1]).ok(),
            "hypot" => Some(ports::hypot(&inputs)),
            _ => None,
        };
        calls += usize::from(name != "hypot");
        let want = f64_bits(want.as_str().expect("bits"));
        if got.map(f64::to_bits) != Some(want.to_bits()) {
            failures.push(format!("{name}{inputs:?}: {got:?} vs {want:e}"));
        }
    }
    assert!(calls >= 30, "{calls} trigonometric calls recorded");
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn the_correctly_rounded_functions_are_the_pythons() {
    // `numeric.correctly_rounded`: the Python module's results at its test's hard, reduction, special and repro
    // inputs and at sampled inputs of every binade, with `math`'s domain errors.
    let mut failures = Vec::new();
    let mut count = 0;
    for item in numeric("correctly_rounded") {
        let item = item.as_list().expect("item");
        let name = item[0].as_str().expect("name");
        let args: Vec<f64> = item[1].as_list().expect("args").iter().map(|arg| f64_bits(arg.as_str().expect("bits"))).collect();
        let got = match name {
            "acos" => ports::acos(args[0]),
            "asin" => ports::asin(args[0]),
            "sin" => ports::sin(args[0]),
            "tan" => ports::tan(args[0]),
            "atan2" => ports::atan2(args[0], args[1]),
            _ => panic!("{name}"),
        };
        let got = match got {
            Ok(value) if value.is_nan() => Value::str("nan"), // a NaN's sign and payload are the platform's
            Ok(value) => Value::Str(f64_hex(value)),
            Err(error) => Value::List(vec![Value::Str(error.kind.into()), Value::Str(error.message)]),
        };
        if !mbf::py_eq(&got, &item[2]) {
            failures.push(format!("{name}{args:?}: {got:?} vs {:?}", item[2]));
        }
        count += 1;
    }
    assert!(count >= 2000, "{count} vectors");
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
    assert_eq!(ports::atan2(1.0, 0.0).unwrap(), FRAC_PI_2);
    assert_eq!(ports::atan2(-1.0, -1.0).unwrap(), -3.0 * PI / 4.0);
    assert_eq!(ports::atan2(0.0, -1.0).unwrap(), PI);
    assert_eq!(ports::atan2(-0.0, 1.0).unwrap().to_bits(), (-0.0f64).to_bits());
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
