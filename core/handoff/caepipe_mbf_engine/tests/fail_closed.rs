//! Where the port does not reproduce the Python, it returns no result rather than a different one.

use open_pipe_stress_caepipe_mbf::{self as mbf, ImportError, ImportOptions, SidecarInput, Value};

const MBF: &str = "INVENTED FAIL-CLOSED CHECK\r\nVERSION\r\n15.10\r\nOPTIONS\r\nSI\r\nOPTEMP\r\n20\r\nMATERIAL\r\n\
M1,0,0.3,1,1,CS,invented steel\r\nM1,20,200000,1.2e-05\r\nPIPE\r\nP1,168.3,7.11,0,0,0,0,0,0\r\nLOADS\r\nL1,20,0,0\r\n\
LAYOUT\r\nF10,KA,MM1,PP1,CL=L1\r\nT20,X3000\r\n";

fn invented() -> Value {
    let text = br#"{"*": {"origin": "project_invented", "source_name": "invented fail-closed material"}}"#;
    mbf::parse_sidecar_json(text).expect("declaration")
}

#[test]
fn a_lone_surrogate_in_sidecar_json_is_refused_as_unsupported() {
    let mut options = ImportOptions::new("invented-test");
    options.material_provenance = Some(invented());
    options.sidecar = Some(SidecarInput::Json(br#"{"node_map": {"n": "\ud800"}}"#.to_vec()));
    match mbf::import(MBF.as_bytes(), &options) {
        Err(ImportError::Unsupported(reason)) => assert!(reason.contains("lone surrogate")),
        other => panic!("expected an unsupported-input refusal, got {other:?}"),
    }
}

#[test]
fn a_bound_current_sidecar_without_a_regenerator_is_checked_by_the_crates_own_writer() {
    // A format-3 sidecar bound to these bytes passes the self-digest and the parser, then reaches the regeneration
    // identity: with no regenerator given, the crate's own SI writer regenerates the sidecar (slice 2), exactly as
    // when the caller passes it.
    let data = MBF.as_bytes();
    let sha: String = {
        use sha2::{Digest, Sha256};
        Sha256::digest(data).iter().map(|byte| format!("{byte:02x}")).collect()
    };
    let body = format!(
        r#"{{"node_map": {{"node:10": "10", "node:20": "20"}}, "element_map": {{"pipe:10-20": {{"from": "10", "to": "20",
        "reversed": false, "y_reference": {{"x": 0.0, "y": 1.0, "z": 0.0}}, "y_reference_authored": true}}}},
        "support_map": {{"support:10": {{"node": "10", "restrained": ["UX", "UY", "UZ", "RX", "RY", "RZ"],
        "family": "anchor", "family_authored": true}}}}, "load_case_map": {{}}, "material_map": {{"M1":
        {{"swbpipe_material": "material:M1", "origin": "project_invented"}}}}, "load_map": {{}}, "pressure_map":
        {{"contract": "none", "shear_model": "euler_bernoulli", "regions": []}}, "mbf_sha256": "{sha}", "sidecar_format": 3}}"#
    );
    let mut sidecar = mbf::parse_sidecar_json(body.as_bytes()).expect("sidecar");
    let digest = mbf::sidecar_self_digest(&sidecar).expect("digest");
    sidecar.as_dict_mut().expect("dict").insert("sidecar_sha256".into(), Value::Str(digest));
    let mut options = ImportOptions::new("invented-test");
    options.sidecar = Some(SidecarInput::Parsed(sidecar));
    let default = mbf::import(data, &options).expect("a result, not an error");
    options.regenerator = Some(&mbf::SiWriter);
    let explicit = mbf::import(data, &options).expect("a result");
    assert_eq!(default.report_bytes().expect("report"), explicit.report_bytes().expect("report"));
    assert_eq!(default.model_bytes().expect("model"), explicit.model_bytes().expect("model"));
}

#[test]
fn every_float_formatted_reads_back_or_the_formatting_fails() {
    for value in [0.1, 1e16, 1e-5, 999999999999975.2, 279836846559720.12, 5e-324, f64::MAX, -0.0, 2.0f64.powi(-1022)] {
        let text = mbf::python_float_repr(value).expect("finite floats format");
        assert_eq!(text.parse::<f64>().expect("reads back").to_bits(), value.to_bits(), "{text}");
    }
}

#[test]
fn a_blank_source_label_raises_as_the_python_does() {
    let options = ImportOptions::new(" \u{1c}");
    match mbf::import(MBF.as_bytes(), &options) {
        Err(ImportError::Python(error)) => {
            assert_eq!((error.kind, error.message.as_str()), ("ValueError", "source_label must be a non-empty string"))
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn an_english_length_far_below_binary64_is_read_exactly_then_refused_as_the_python_refuses_it() {
    // 450 decimal places: Fraction reads it exactly, and the node position it gives underflows binary64.
    let tiny = format!("0.{}1", "0".repeat(450));
    let text = MBF.replace("OPTIONS\r\nSI\r\n", "OPTIONS\r\nHGRA\r\n").replace("T20,X3000", &format!("T20,X{tiny}"));
    let mut options = ImportOptions::new("invented-test");
    options.material_provenance = Some(invented());
    let result = mbf::import(text.as_bytes(), &options).expect("a result, not an error");
    assert_eq!(result.status, "refused");
    let codes: Vec<&str> = result.diagnostics.iter().filter_map(|item| item.get("code").and_then(Value::as_str)).collect();
    assert!(codes.contains(&"MBF-IMPORT-VALUE-UNREPRESENTABLE"), "{codes:?}");
}

#[test]
fn an_exponent_at_the_i64_limits_is_refused_like_any_huge_exponent_without_panicking() {
    // Review r1: `OPTEMP` with exponent -9223372036854775808 overflowed `power.abs()`. Each extreme must take the same
    // path as an ordinary exponent beyond 10^5.
    let mut options = ImportOptions::new("invented-test");
    options.material_provenance = Some(invented());
    let outcome = |optemp: &str| match mbf::import(MBF.replace("OPTEMP\r\n20\r\n", &format!("OPTEMP\r\n{optemp}\r\n")).as_bytes(), &options) {
        // The status and the diagnostic codes; the report also binds the input's own hash and quotes the field.
        Ok(result) => String::from_utf8(result.report_bytes().expect("report bytes"))
            .expect("report text")
            .lines()
            .filter(|line| line.trim_start().starts_with("\"code\"") || line.trim_start().starts_with("\"status\""))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(error) => format!("{error:?}"),
    };
    let huge = outcome("1e-200000");
    assert!(huge.contains("MBF-IMPORT-VALUE-UNREPRESENTABLE"), "{huge}");
    for extreme in ["1e-9223372036854775808", "1e9223372036854775807", "1e-9223372036854775809", "1e9223372036854775808"] {
        assert_eq!(outcome(extreme), huge, "exponent {extreme}");
    }
}

/// A JSON value `levels` deep: arrays, or objects keyed "a".
fn nested(levels: usize, objects: bool) -> String {
    if objects {
        format!("{}1{}", "{\"a\":".repeat(levels), "}".repeat(levels))
    } else {
        format!("{}1{}", "[".repeat(levels), "]".repeat(levels))
    }
}

/// Sidecars that put a value `levels` deep where the importer reads, quotes or compares it.
fn deep_sidecars(levels: usize) -> Vec<String> {
    let mut out = Vec::new();
    for objects in [false, true] {
        let value = nested(levels, objects);
        out.push(value.clone());
        out.push(format!("{{\"node_map\": {{\"n\": {value}}}}}"));
        out.push(format!("{{\"sidecar_format\": {value}}}"));
        out.push(format!("{{\"sidecar_format\": 3, \"mbf_sha256\": {value}, \"node_map\": {{}}}}"));
        out.push(format!("{{\"unknown\": {value}, \"node_map\": {{\"10\": \"node:10\"}}}}"));
    }
    out
}

#[test]
fn sidecar_json_at_the_nesting_limit_imports_or_refuses_and_beyond_it_returns_no_result() {
    // Review r1 robustness: at the former limit of 512 an array 402 deep overflowed this debug test thread's 2 MiB
    // stack and aborted the process. Nesting is bounded before recursion can exhaust a stack, and every consumer of
    // the parsed value (scanner, conversion, the importer's reads and quotes) stays within the bound.
    let limit = mbf::MAX_SIDECAR_NESTING;
    // the outermost container of each placement is at most two levels above the nested value
    for sidecar in deep_sidecars(limit - 2) {
        let mut options = ImportOptions::new("invented-test");
        options.material_provenance = Some(invented());
        options.sidecar = Some(SidecarInput::Json(sidecar.clone().into_bytes()));
        match mbf::import(MBF.as_bytes(), &options) {
            Ok(_) | Err(ImportError::Python(_)) => {}
            Err(other) => panic!("{}: {other:?}", &sidecar[..40.min(sidecar.len())]),
        }
    }
    for sidecar in deep_sidecars(limit + 1) {
        let mut options = ImportOptions::new("invented-test");
        options.material_provenance = Some(invented());
        options.sidecar = Some(SidecarInput::Json(sidecar.clone().into_bytes()));
        match mbf::import(MBF.as_bytes(), &options) {
            Err(ImportError::Unsupported(reason)) => assert!(reason.contains("nests deeper"), "{reason}"),
            other => panic!("{}: {other:?}", &sidecar[..40.min(sidecar.len())]),
        }
    }
}

fn writer_model() -> Value {
    // An invented one-pipe cantilever, enough to reach the load-case reader.
    let text = br#"{"schema_version": "0.8.0", "project": {"units": {"length": "mm", "force": "N", "temperature": "degC"}},
        "nodes": [{"id": "a", "position": {"x": 0, "y": 0, "z": 0}}, {"id": "b", "position": {"x": 1000, "y": 0, "z": 0}}],
        "materials": [{"id": "steel", "elastic_modulus": {"value": 200000, "unit": "MPa"}, "poisson_ratio": {"value": 0.3, "unit": "1"}}],
        "pipe_segments": [{"id": "p", "from": "a", "to": "b", "material": "steel", "section": {"outside_diameter":
            {"value": 100, "unit": "mm"}, "wall_thickness": {"value": 5, "unit": "mm"}}}],
        "supports": [{"id": "s", "node": "a", "family": "anchor", "restraints": ["UX", "UY", "UZ", "RX", "RY", "RZ"]}],
        "load_cases": [{"id": "c", "primitive_loads": [{"id": "f", "category": "concentrated_force", "target":
            {"type": "node", "node": "b"}, "direction": "global_y", "magnitude": {"value": -10, "unit": "N"}, "dimension": "force"}]}]}"#;
    mbf::parse_sidecar_json(text).expect("model")
}

#[test]
fn the_writer_writes_the_invented_cantilever() {
    let result = mbf::render_si(&writer_model(), &mbf::WriterSettings::new("INVENTED")).expect("a result");
    assert_eq!(result.status, "written", "{:?}", result.diagnostics);
}

#[test]
fn a_nan_load_category_is_refused_as_unsupported_by_the_writer() {
    // Python's set of categories holds a NaN by object identity, which decides whether the load is refused; the port
    // cannot know the identity, so it returns no result rather than guessing.
    let mut model = writer_model();
    let load = model.as_dict_mut().unwrap().get_mut(&"load_cases".to_string()).unwrap().as_list().unwrap()[0].clone();
    let mut load = load;
    let primitive = load.as_dict_mut().unwrap().get_mut(&"primitive_loads".to_string()).unwrap();
    let Value::List(items) = primitive else { panic!("list") };
    items[0].as_dict_mut().unwrap().insert("category".into(), Value::Float(f64::NAN));
    model.as_dict_mut().unwrap().insert("load_cases".into(), Value::List(vec![load]));
    match mbf::render_si(&model, &mbf::WriterSettings::new("INVENTED")) {
        Err(ImportError::Unsupported(reason)) => assert!(reason.contains("NaN load category"), "{reason}"),
        other => panic!("expected an unsupported-input refusal, got {other:?}"),
    }
}

#[test]
fn a_model_holding_the_parsers_sentinel_is_refused_as_unsupported_by_the_writer() {
    let mut model = writer_model();
    model.as_dict_mut().unwrap().insert("lines".into(), Value::List(vec![Value::Bad]));
    assert!(matches!(mbf::render_si(&model, &mbf::WriterSettings::new("INVENTED")), Err(ImportError::Unsupported(_))));
    assert!(matches!(
        mbf::regenerate_caepipe_sidecar(&model, &"0".repeat(64), &[], mbf::Reference::None),
        Err(ImportError::Unsupported(_))
    ));
    let mut settings = mbf::WriterSettings::new("INVENTED");
    settings.title = Value::List(vec![Value::Bad]);
    assert!(matches!(mbf::render_si(&writer_model(), &settings), Err(ImportError::Unsupported(_))));
}

/// `levels` lists nested in one another (the innermost holds null), built without recursion.
fn chain(levels: usize) -> Value {
    let mut value = Value::Null;
    for _ in 0..levels {
        value = Value::List(vec![value]);
    }
    value
}

/// Leaks a deep value: dropping it would recurse once per level (review r1 of #32, P1).
fn leak(value: Value) {
    std::mem::forget(value);
}

#[test]
fn a_hostile_depth_is_refused_before_any_walk_and_the_limit_itself_is_taken() {
    let limit = mbf::value::MAX_VALUE_NESTING;
    // At the limit the writer runs (the Python omits the unknown member); one level more is refused unwalked.
    let mut model = writer_model();
    model.as_dict_mut().unwrap().insert("unused".into(), chain(limit - 1));
    assert!(mbf::render_si(&model, &mbf::WriterSettings::new("INVENTED")).is_ok());
    for levels in [limit, 50_000] {
        let mut model = writer_model();
        model.as_dict_mut().unwrap().insert("unused".into(), chain(levels));
        match mbf::render_si(&model, &mbf::WriterSettings::new("INVENTED")) {
            Err(ImportError::Unsupported(reason)) => assert!(reason.contains("deeper than"), "{reason}"),
            other => panic!("expected an unsupported-input refusal, got {other:?}"),
        }
        assert!(matches!(
            mbf::regenerate_caepipe_sidecar(&model, &"0".repeat(64), &[], mbf::Reference::None),
            Err(ImportError::Unsupported(_))
        ));
        let mut settings = mbf::WriterSettings::new("INVENTED");
        settings.title = chain(levels + 1);
        assert!(matches!(mbf::render_si(&writer_model(), &settings), Err(ImportError::Unsupported(_))));
        leak(std::mem::replace(&mut settings.title, Value::Null));
        let mut options = ImportOptions::new("invented-test");
        options.material_provenance = Some(chain(levels + 1));
        assert!(matches!(mbf::import(MBF.as_bytes(), &options), Err(ImportError::Unsupported(_))));
        leak(options.material_provenance.take().unwrap());
        options.sidecar = Some(SidecarInput::Parsed(chain(levels + 1)));
        assert!(matches!(mbf::import(MBF.as_bytes(), &options), Err(ImportError::Unsupported(_))));
        if let Some(SidecarInput::Parsed(value)) = options.sidecar.take() {
            leak(value);
        }
        let deep = chain(levels + 1);
        assert!(mbf::sidecar_self_digest(&deep).is_err());
        leak(deep);
        leak(model);
    }
}
