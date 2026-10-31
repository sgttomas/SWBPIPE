//! Byte parity of the Rust SI writer with `core/handoff/caepipe_mbf/si_writer.py` (DEL-17-04 Design
//! `desktop-mbf-export-import.md` §2, §6 slice 2).
//!
//! `fixtures/caepipe_mbf/invented/parity/writer_vectors.json` is written by `tools/fixtures/caepipe_mbf_writer_vectors.py`
//! from the Python over the test suites that call the writer, the committed invented corpus and the generator's
//! coverage inputs, and the Python replays it in `tests/test_caepipe_mbf_writer_parity.py`. Here every recorded call
//! is replayed through the Rust port: the status, the MBF text, the `to_dict()` bytes and the sidecar bytes (or a
//! regeneration's sidecar or refusals) must be the Python's (their digest, and the full text for corpus and coverage
//! items), or the Python's exception.

use open_pipe_stress_caepipe_mbf::{self as mbf, ports, ImportError, Reference, Value, WriterSettings};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::OnceLock;

mod common;
use common::*;

fn document() -> &'static Value {
    static DOCUMENT: OnceLock<Value> = OnceLock::new();
    DOCUMENT.get_or_init(|| {
        parse(&std::fs::read(project().join("fixtures/caepipe_mbf/invented/parity/writer_vectors.json")).expect("vectors"))
    })
}

/// The generator's `chain_model(model, count)`.
fn chain_model(model: &Value, count: usize) -> Value {
    let mut out = model.clone();
    let pipe = model.get_or_null("pipe_segments").as_list().expect("pipes")[0].clone();
    let nodes = (0..=count)
        .map(|index| {
            mbf::dict![
                ("id", Value::str(format!("node:chain-{index}"))),
                ("position", mbf::dict![("x", Value::Float(index as f64)), ("y", Value::Float(0.0)), ("z", Value::Float(0.0))]),
            ]
        })
        .collect();
    let pipes = (0..count)
        .map(|index| {
            mbf::dict![
                ("id", Value::str(format!("pipe:chain-{index}"))),
                ("from", Value::str(format!("node:chain-{index}"))),
                ("to", Value::str(format!("node:chain-{}", index + 1))),
                ("material", pipe.get_or_null("material").clone()),
                ("section", pipe.get_or_null("section").clone()),
            ]
        })
        .collect();
    let dict = out.as_dict_mut().expect("model");
    dict.insert("nodes".into(), Value::List(nodes));
    dict.insert("pipe_segments".into(), Value::List(pipes));
    for key in ["supports", "components", "load_cases"] {
        dict.insert(key.into(), Value::List(Vec::new()));
    }
    out
}

fn ascii(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).expect("ascii")
}

/// The generator's `failed(error)`.
fn failed(error: &mbf::PyError) -> String {
    format!("!{}: {}", error.kind, error.message)
}

/// The generator's `outputs(kind, result)` for the Rust result, or the exception.
fn outputs(kind: &str, model: &Value, settings: &Value, codes: &mut BTreeSet<String>) -> Result<BTreeMap<String, String>, ImportError> {
    let mut out = BTreeMap::new();
    if kind == "regenerate" {
        let derived: Vec<String> = settings
            .get_or_null("derived_y_reference")
            .as_list()
            .expect("derived")
            .iter()
            .map(|item| item.as_str().expect("str").to_string())
            .collect();
        let reference = settings.get_or_null("reference");
        let reference = match (reference.get("reference_temperature_c"), reference.get("reference_temperature_f")) {
            (Some(value), _) => Reference::Celsius(value.as_f64().expect("float")),
            (None, Some(value)) => Reference::Fahrenheit(value.as_f64().expect("float")),
            (None, None) => Reference::None,
        };
        let sha = document().get_or_null("regeneration_sha256").as_str().expect("sha");
        let (sidecar, refusals) = mbf::regenerate_caepipe_sidecar(model, sha, &derived, reference)?;
        for item in &refusals {
            codes.insert(item.get_or_null("code").as_str().expect("code").to_string());
        }
        match sidecar {
            Some(sidecar) => out.insert("sidecar".into(), ascii(mbf::canonical_bytes(&sidecar).map_err(ImportError::Python)?)),
            None => out.insert("refused".into(), ascii(mbf::canonical_bytes(&Value::List(refusals)).map_err(ImportError::Python)?)),
        };
        return Ok(out);
    }
    let mut writer = WriterSettings::new("");
    writer.title = settings.get_or_null("title").clone();
    writer.reference_temperature_c = settings.get_or_null("reference_temperature_c").clone();
    writer.max_layout_rows = mbf::value::small_int(settings.get_or_null("max_layout_rows"));
    writer.reference_temperature_f = settings.get_or_null("reference_temperature_f").clone();
    writer.piping_code = settings.get_or_null("piping_code").clone();
    writer.preflight_disabled = settings.get_or_null("preflight").as_bool() == Some(false);
    let chained;
    let model = match mbf::value::small_int(settings.get_or_null("chain")) {
        Some(count) => {
            chained = chain_model(model, count as usize);
            &chained
        }
        None => model,
    };
    let result = mbf::render_si(model, &writer)?;
    for item in &result.diagnostics {
        codes.insert(item.get_or_null("code").as_str().expect("code").to_string());
    }
    out.insert("status".into(), result.status.clone());
    out.insert("text".into(), result.text.clone().unwrap_or_else(|| "-".into()));
    out.insert("to_dict".into(), result.to_dict_bytes().map(ascii).unwrap_or_else(|error| failed(&error)));
    let sidecar = if result.status == "written" {
        let text = result.text.as_ref().expect("text");
        result.sidecar_bytes(&sha(text.as_bytes())).map(ascii).unwrap_or_else(|error| failed(&error))
    } else {
        "-".into()
    };
    out.insert("sidecar".into(), sidecar);
    Ok(out)
}

/// The generator's `digest(texts)`.
fn digest(texts: &BTreeMap<String, String>) -> String {
    let parts: Vec<String> = texts.iter().map(|(key, value)| format!("{key}={}", sha(value.as_bytes()))).collect();
    sha(parts.join("|").as_bytes())[..32].to_string()
}

struct Replay {
    failures: Vec<String>,
    codes: BTreeSet<String>,
    count: usize,
    renders: usize,
    regenerations: usize,
}

fn replay_all() -> &'static Replay {
    static REPLAY: OnceLock<Replay> = OnceLock::new();
    REPLAY.get_or_init(|| {
        let doc = document();
        let mut inputs = Inputs::new(doc);
        let sources = doc.get_or_null("sources").as_list().expect("sources");
        let mut replay = Replay { failures: Vec::new(), codes: BTreeSet::new(), count: 0, renders: 0, regenerations: 0 };
        let out_dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("caepipe_mbf_writer_parity");
        for (index, row) in doc.get_or_null("vectors").as_list().expect("vectors").iter().enumerate() {
            let row = row.as_list().expect("row");
            let source = sources[mbf::value::small_int(&row[0]).expect("src") as usize].as_str().expect("source");
            let kind = row[1].as_str().expect("kind");
            let model = inputs.value(&row[2]).expect("model");
            let settings = inputs.value(&row[3]).expect("settings");
            let expect = &row[4];
            replay.count += 1;
            if kind == "render" {
                replay.renders += 1;
            } else {
                replay.regenerations += 1;
            }
            let problem = match (outputs(kind, &model, &settings, &mut replay.codes), expect.get("raises")) {
                (Err(ImportError::Python(error)), Some(Value::List(raised))) => {
                    let want = (raised[0].as_str().unwrap_or_default(), raised[1].as_str().unwrap_or_default());
                    ((error.kind, error.message.as_str()) != want)
                        .then(|| format!("raises {}: {}, the Python {}: {}", error.kind, error.message, want.0, want.1))
                }
                (Err(error), _) => Some(format!("returns no result: {error}")),
                (Ok(texts), Some(raised)) => Some(format!("returns {:?}, the Python raises {raised:?}", texts.get("status"))),
                (Ok(texts), None) => {
                    let mut problem = None;
                    for (key, value) in &texts {
                        if let Some(Value::Str(want)) = expect.get(key) {
                            if want != value {
                                problem = Some(format!("{key} differs"));
                            }
                        }
                    }
                    if digest(&texts) != expect.get_or_null("digest").as_str().unwrap_or_default() {
                        problem = problem.or_else(|| Some(format!("outputs differ ({kind})")));
                    }
                    if problem.is_some() {
                        let _ = std::fs::create_dir_all(&out_dir);
                        for (key, value) in &texts {
                            let _ = std::fs::write(out_dir.join(format!("{index}.{key}.txt")), value);
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
fn every_recorded_writer_call_gives_the_pythons_bytes() {
    let replay = replay_all();
    assert!(replay.renders > 300 && replay.regenerations > 200, "{} renders, {} regenerations", replay.renders, replay.regenerations);
    let shown: Vec<&String> = replay.failures.iter().take(25).collect();
    assert!(
        replay.failures.is_empty(),
        "{} of {} vectors differ (Rust outputs in target/tmp/caepipe_mbf_writer_parity; the Python's: \
         python3 tools/fixtures/caepipe_mbf_writer_vectors.py show <id>):\n{}",
        replay.failures.len(),
        replay.count,
        shown.iter().map(|item| item.as_str()).collect::<Vec<_>>().join("\n")
    );
}

/// The writer's codes as `writer_codes()` reads them from si_writer.py: its `MBF-SI-*` literals and the retired
/// model-version code.
fn writer_codes() -> BTreeSet<String> {
    let source = std::fs::read_to_string(project().join("core/handoff/caepipe_mbf/si_writer.py")).expect("si_writer.py");
    let mut codes: BTreeSet<String> = BTreeSet::new();
    let mut at = 0;
    while let Some(found) = source[at..].find("\"MBF-SI-") {
        let start = at + found + 1;
        let end = start + source[start..].find('"').expect("closing quote");
        let code = &source[start..end];
        if code.bytes().all(|byte| byte.is_ascii_uppercase() || byte.is_ascii_digit() || byte == b'-') {
            codes.insert(code.to_string());
        }
        at = end + 1;
    }
    codes.insert("MODEL_SCHEMA_VERSION_RETIRED".into());
    codes
}

#[test]
fn every_writer_code_is_emitted_by_the_rust_replay() {
    let codes = writer_codes();
    assert!(codes.len() > 40, "found {} codes in si_writer.py", codes.len());
    let replay = replay_all();
    let missing: Vec<&String> = codes.iter().filter(|code| !replay.codes.contains(*code)).collect();
    assert!(missing.is_empty(), "writer codes no Rust replay emits: {missing:?}");
    let recorded: BTreeSet<String> = document()
        .get_or_null("codes")
        .as_list()
        .expect("codes")
        .iter()
        .map(|code| code.as_str().expect("code").to_string())
        .collect();
    assert_eq!(replay.codes, recorded, "the Rust replay's codes are not the Python's");
}

fn numeric(name: &str) -> &'static Vec<Value> {
    document().get("numeric").and_then(|numeric| numeric.get(name)).and_then(Value::as_list).expect("numeric set")
}

#[test]
fn ten_digit_formatting_matches_python_at_its_rounding_boundaries() {
    let mut failures = Vec::new();
    for item in numeric("num") {
        let item = item.as_list().expect("item");
        let value = f64_bits(item[0].as_str().expect("bits"));
        let want = item[1].as_str().expect("text");
        if ports::writer_num(value) != want {
            failures.push(format!("{value:e}: {} vs {want}", ports::writer_num(value)));
        }
    }
    assert!(numeric("num").len() > 1000);
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn ten_digit_formatting_of_exact_fractions_matches_pythons_fraction_format() {
    let mut failures = Vec::new();
    for item in numeric("num_fraction") {
        let item = item.as_list().expect("item");
        let (numerator, denominator) = (item[0].as_str().expect("n"), item[1].as_str().expect("d"));
        let got = ports::writer_num_fraction(numerator, denominator).expect("a fraction");
        if got != item[2].as_str().expect("text") {
            failures.push(format!("{numerator}/{denominator}: {got} vs {:?}", item[2]));
        }
    }
    assert!(numeric("num_fraction").len() > 300);
    assert!(failures.is_empty(), "{failures:?}");
}

#[test]
fn the_unit_tables_and_normalizations_are_the_pythons_exact_factors() {
    type Factor = (String, String, Option<(String, String)>);
    let recorded: Vec<Factor> = numeric("factors")
        .iter()
        .map(|item| {
            let item = item.as_list().expect("item");
            let factor = item[2].as_list().map(|pair| (pair[0].as_str().expect("n").to_string(), pair[1].as_str().expect("d").to_string()));
            (item[0].as_str().expect("table").to_string(), item[1].as_str().expect("unit").to_string(), factor)
        })
        .collect();
    let ported: Vec<Factor> =
        ports::writer_unit_factors().into_iter().map(|(table, unit, factor)| (table.to_string(), unit.to_string(), factor)).collect();
    assert_eq!(ported, recorded);
}

#[test]
fn degrees_and_radians_match_python() {
    for item in numeric("degrees") {
        let item = item.as_list().expect("item");
        let value = f64_bits(item[0].as_str().expect("bits"));
        assert_eq!(f64_hex(ports::degrees(value)), item[1].as_str().expect("degrees"), "degrees({value:e})");
        assert_eq!(f64_hex(ports::radians(value)), item[2].as_str().expect("radians"), "radians({value:e})");
    }
}

#[test]
fn the_vectors_cover_the_corpus_the_import_vectors_write_back() {
    // Each corpus model the import vectors write back is written here from the same import (the Python test also
    // holds every correlation case to a vector).
    let sources: BTreeSet<&str> =
        document().get_or_null("sources").as_list().expect("sources").iter().map(|item| item.as_str().expect("source")).collect();
    let imports =
        parse(&std::fs::read(project().join("fixtures/caepipe_mbf/invented/parity/import_vectors.json")).expect("vectors"));
    let written_back: Vec<&str> = imports
        .get_or_null("sources")
        .as_list()
        .expect("sources")
        .iter()
        .filter_map(Value::as_str)
        .filter(|source| source.starts_with("corpus:") && source.ends_with("written back"))
        .collect();
    assert!(written_back.len() >= 10, "{} written-back corpus models", written_back.len());
    let missing: Vec<&&str> = written_back.iter().filter(|source| !sources.contains(**source)).collect();
    assert!(missing.is_empty(), "written-back corpus models without a writer vector: {missing:?}");
    assert!(sources.iter().filter(|source| source.starts_with("corpus:cases/")).count() >= 8);
}
