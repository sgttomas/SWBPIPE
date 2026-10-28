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
fn a_bound_current_sidecar_without_the_writer_returns_no_result() {
    // A format-3 sidecar bound to these bytes passes the self-digest and the parser, then reaches the regeneration
    // identity, which needs the SI writer.
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
    assert_eq!(mbf::import(data, &options).err(), Some(ImportError::RegeneratorRequired));
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
