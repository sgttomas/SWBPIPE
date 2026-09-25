use open_pipe_stress_canonical_json::{
    binary64::*, binary64_cli::*, canonical_json, canonical_json_checked_v1_text,
    ecma_number_to_string,
};
use serde_json::{json, Value};

#[test]
fn real_spellings_converge_without_integer_variant_precision() {
    for (tokens, expected) in [
        (vec!["1e160", "1.0e160"], "1e+160"),
        (
            vec!["100000000000000000000", "1e20", "1.0e20"],
            "100000000000000000000",
        ),
        (
            vec![
                "9007199254740993",
                "9007199254740993.0",
                "9.007199254740993e15",
            ],
            "9007199254740992",
        ),
        (vec!["18446744073709551615"], "18446744073709552000"),
        (vec!["1", "1.0", "1e0", "1.0000000000000000001"], "1"),
        (vec!["0e9999999999999999999"], "0"),
    ] {
        for token in tokens {
            assert_eq!(
                canonical_json_binary64_v1_text(token).unwrap(),
                expected,
                "{token}"
            );
        }
    }
}

#[test]
fn rfc_finite_rendering_vectors_and_new_negative_zero_policy() {
    let vectors = [
        (0x0000000000000000, "0"),
        (0x8000000000000000, "0"),
        (0x0000000000000001, "5e-324"),
        (0x8000000000000001, "-5e-324"),
        (0x7fefffffffffffff, "1.7976931348623157e+308"),
        (0xffefffffffffffff, "-1.7976931348623157e+308"),
        (0x4340000000000000, "9007199254740992"),
        (0xc340000000000000, "-9007199254740992"),
        (0x4430000000000000, "295147905179352830000"),
        (0x44b52d02c7e14af5, "9.999999999999997e+22"),
        (0x44b52d02c7e14af6, "1e+23"),
        (0x44b52d02c7e14af7, "1.0000000000000001e+23"),
        (0x444b1ae4d6e2ef4e, "999999999999999700000"),
        (0x444b1ae4d6e2ef4f, "999999999999999900000"),
        (0x444b1ae4d6e2ef50, "1e+21"),
        (0x3eb0c6f7a0b5ed8c, "9.999999999999997e-7"),
        (0x3eb0c6f7a0b5ed8d, "0.000001"),
        (0x41b3de4355555553, "333333333.3333332"),
        (0x41b3de4355555554, "333333333.33333325"),
        (0x41b3de4355555555, "333333333.3333333"),
        (0x41b3de4355555556, "333333333.3333334"),
        (0x41b3de4355555557, "333333333.33333343"),
        (0xbecbf647612f3696, "-0.0000033333333333333333"),
        (0x43143ff3c1cb0959, "1424953923781206.2"),
    ];
    for (bits, expected) in vectors {
        let value = f64::from_bits(bits);
        assert_eq!(ecma_number_to_string(value), expected, "{bits:016x}");
        if bits == 0x8000000000000000 {
            assert!(canonical_json_binary64_v1_value(&json!(value)).is_err());
        } else {
            assert_eq!(
                canonical_json_binary64_v1_value(&json!(value)).unwrap(),
                expected
            );
            let parsed = parse_binary64_v1_text(expected).unwrap();
            assert_eq!(parsed.value().as_f64().unwrap().to_bits(), bits);
        }
    }
}

#[test]
fn original_token_checks_reject_underflow_zero_and_overflow() {
    for token in [
        "-0", "-0.0", "-0e3", "-0e-9999", "1e-324", "-1e-324", "1e309", "NaN", "Infinity",
    ] {
        assert!(canonical_json_binary64_v1_text(token).is_err(), "{token}");
    }
    assert_eq!(
        canonical_json_binary64_v1_text("2.4703282292062328e-324").unwrap(),
        "5e-324"
    );
    assert!(canonical_json_binary64_v1_text("2.4703282292062327e-324")
        .unwrap_err()
        .contains("UNDERFLOW"));
}

#[test]
fn original_count_fields_are_checked_before_projection() {
    let doc = parse_binary64_v1_text(r#"{"count":1,"fraction":1.0000000000000000001,"exp":1e0,"a/b":{"~x":7},"items":[2],"large":9007199254740993,"bool":true}"#).unwrap();
    assert_eq!(doc.exact_integer_at("/count", 0, 10).unwrap(), 1);
    assert_eq!(doc.exact_integer_at("/a~1b/~0x", 0, 10).unwrap(), 7);
    assert_eq!(doc.exact_integer_at("/items/0", 0, 10).unwrap(), 2);
    assert!(doc.value()["count"].is_i64());
    for pointer in ["/fraction", "/exp", "/large", "/bool", "/missing"] {
        assert!(doc.exact_integer_at(pointer, 0, 10).is_err(), "{pointer}");
    }
    for token in [
        "-0",
        "1.0",
        "1e0",
        "01",
        "+1",
        "",
        "-",
        "9007199254740992",
        "-9007199254740992",
        " 1",
        "1 ",
    ] {
        assert!(
            exact_integer_token(token, -9_007_199_254_740_991, 9_007_199_254_740_991).is_err(),
            "{token}"
        );
    }
    assert!(exact_integer_token("-1", 0, 10).is_err());
    assert!(exact_integer_token("1", 2, 1).is_err());
    assert!(exact_integer_value(1, i64::MIN, i64::MAX).is_err());
    assert_eq!(
        exact_integer_token("9007199254740991", 0, 9_007_199_254_740_991).unwrap(),
        9_007_199_254_740_991
    );
}

#[test]
fn strings_duplicates_sorting_and_syntax_are_strict() {
    for text in [
        r#"{"a":1,"\u0061":2}"#,
        r#"{"x":{"a":1,"a":2}}"#,
        r#""\ud800""#,
        r#""\udfff""#,
        r#""\uffff""#,
        r#"{"\ufdd0":0}"#,
        r#"["\udbff\udfff"]"#,
        "[1,]",
        "{\"x\":0,}",
        "[01]",
        "+1",
        "1.",
        "1e",
        "00",
        "true false",
        "\u{feff}0",
        "\u{a0}0",
        "{\"x\":\"a\n\"}",
    ] {
        assert!(canonical_json_binary64_v1_text(text).is_err(), "{text}");
    }
    assert_eq!(
        canonical_json_binary64_v1_text(r#"{"ｚ":1,"\ud83d\ude00":2,"a":"e\u0301","b":"é"}"#)
            .unwrap(),
        "{\"a\":\"e\u{301}\",\"b\":\"é\",\"😀\":2,\"ｚ\":1}"
    );
    assert_ne!(
        canonical_json_binary64_v1_text("\"é\"").unwrap(),
        canonical_json_binary64_v1_text("\"e\u{301}\"").unwrap()
    );
}

#[test]
fn snapshot_domain_is_explicit_and_does_not_alter_history() {
    for value in [
        json!(9_007_199_254_740_993_u64),
        json!(u64::MAX),
        json!(i64::MIN),
        json!(-0.0),
        json!("\u{ffff}"),
    ] {
        assert!(canonical_json_binary64_v1_value(&value).is_err());
    }
    let explicitly_real = Value::from(9_007_199_254_740_993_u64 as f64);
    assert_eq!(
        canonical_json_binary64_v1_value(&explicitly_real).unwrap(),
        "9007199254740992"
    );
    assert_eq!(
        canonical_json(&json!(9_007_199_254_740_993_u64)),
        "9007199254740993"
    );
    assert_eq!(canonical_json_checked_v1_text("-0.0").unwrap(), "0");
    assert!(canonical_json_checked_v1_text("1e160").is_err());
    assert_eq!(canonical_json_checked_v1_text("1e-324").unwrap(), "0");
    assert_eq!(
        canonical_json_checked_v1_text("\"\u{ffff}\"").unwrap(),
        "\"\u{ffff}\""
    );
}

#[test]
fn bounded_document_depth_number_and_nodes() {
    let exactly = format!("{}0", " ".repeat(MAX_DOCUMENT_BYTES - 1));
    assert_eq!(canonical_json_binary64_v1_text(&exactly).unwrap(), "0");
    assert!(canonical_json_binary64_v1_text(&(exactly + " "))
        .unwrap_err()
        .contains("BYTE-LIMIT"));
    let nest = |n| format!("{}0{}", "[".repeat(n), "]".repeat(n));
    assert!(canonical_json_binary64_v1_text(&nest(MAX_DEPTH)).is_ok());
    assert!(canonical_json_binary64_v1_text(&nest(MAX_DEPTH + 1))
        .unwrap_err()
        .contains("DEPTH-LIMIT"));
    assert!(
        canonical_json_binary64_v1_text(&format!("0e{}", "0".repeat(MAX_NUMBER_BYTES - 2))).is_ok()
    );
    assert!(
        canonical_json_binary64_v1_text(&format!("0e{}", "0".repeat(MAX_NUMBER_BYTES - 1)))
            .unwrap_err()
            .contains("NUMBER-LIMIT")
    );
    let array = |count| format!("[{}0]", "0,".repeat(count - 1));
    assert!(canonical_json_binary64_v1_text(&array(MAX_VALUE_NODES - 1)).is_ok());
    assert!(canonical_json_binary64_v1_text(&array(MAX_VALUE_NODES))
        .unwrap_err()
        .contains("NODE-LIMIT"));
}

#[test]
fn cli_profile_envelope_and_batch_contract() {
    let request = json!({"protocol_version":"1.0.0","profile":BINARY64_PROFILE_V1,"items":[{"id":"r","json_text":"1e160"}]});
    let result: Value =
        serde_json::from_str(&canonicalize_binary64_v1_request(&request.to_string()).unwrap())
            .unwrap();
    assert_eq!(result["profile"], BINARY64_PROFILE_V1);
    assert_eq!(result["items"][0]["canonical_json"], "1e+160");
    for (field, value) in [
        ("profile", json!("openpipestress_jcs_ijson_v1")),
        ("profile", json!("future")),
        ("protocol_version", json!("2.0.0")),
        ("unknown", json!(1)),
    ] {
        let mut changed = request.clone();
        changed[field] = value;
        assert!(canonicalize_binary64_v1_request(&changed.to_string()).is_err());
    }
    let duplicate = request.to_string().replacen("{", "{\"profile\":\"x\",", 1);
    assert!(canonicalize_binary64_v1_request(&duplicate)
        .unwrap_err()
        .contains("DUPLICATE"));
    let mut duplicate_id = request.clone();
    duplicate_id["items"]
        .as_array_mut()
        .unwrap()
        .push(request["items"][0].clone());
    assert!(canonicalize_binary64_v1_request(&duplicate_id.to_string()).is_err());
    let mut unknown_item = request.clone();
    unknown_item["items"][0]["surprise"] = json!(true);
    assert!(canonicalize_binary64_v1_request(&unknown_item.to_string()).is_err());
    let mut items = request;
    items["items"] = Value::Array(
        (0..MAX_BATCH_ITEMS)
            .map(|i| json!({"id":i.to_string(),"json_text":"0"}))
            .collect(),
    );
    assert!(canonicalize_binary64_v1_request(&items.to_string()).is_ok());
    items["items"]
        .as_array_mut()
        .unwrap()
        .push(json!({"id":"extra","json_text":"0"}));
    assert!(canonicalize_binary64_v1_request(&items.to_string())
        .unwrap_err()
        .contains("BATCH-LIMIT"));
    assert!(
        canonicalize_binary64_v1_request(&" ".repeat(MAX_REQUEST_BYTES + 1))
            .unwrap_err()
            .contains("BYTE-LIMIT")
    );
}

#[test]
fn repeated_long_keys_cannot_amplify_capture_without_bound() {
    let key = "k".repeat(4 * 1024 * 1024);
    let text = format!("{{\"{key}\":[0,0,0,0]}}");
    assert!(text.len() < MAX_DOCUMENT_BYTES);
    assert!(canonical_json_binary64_v1_text(&text)
        .unwrap_err()
        .contains("PATH-LIMIT"));
}

#[test]
fn programmatic_output_expansion_is_bounded_before_rendering() {
    // Raw control bytes expand sixfold in JSON. Admission counts that expansion
    // before constructing the canonical output string.
    let oversized = Value::String("\u{0}".repeat(MAX_DOCUMENT_BYTES / 6 + 1));
    assert!(canonical_json_binary64_v1_value(&oversized)
        .unwrap_err()
        .contains("BYTE-LIMIT"));
    let exact = Value::String("a".repeat(MAX_DOCUMENT_BYTES - 2));
    assert_eq!(
        canonical_json_binary64_v1_value(&exact).unwrap().len(),
        MAX_DOCUMENT_BYTES
    );
    let one_over = Value::String("a".repeat(MAX_DOCUMENT_BYTES - 1));
    assert!(canonical_json_binary64_v1_value(&one_over).is_err());
    for value in [
        json!({"x": [null, true, false, "é😀\u{0}\n\"\\", 1e160, 0.1]}),
        json!([]),
        json!({}),
    ] {
        assert!(canonical_json_binary64_v1_value(&value).is_ok());
    }
}

#[test]
fn text_canonical_output_limit_cannot_be_bypassed_by_document_method() {
    // Short scientific tokens grow when canonical ECMAScript uses decimal
    // notation. The original text fits8MiB but its canonical form does not.
    let text = format!(
        "[\"{}\",{}1e20]",
        "a".repeat(4 * 1024 * 1024),
        "1e20,".repeat(199_999)
    );
    assert!(text.len() < MAX_DOCUMENT_BYTES);
    let parsed = parse_binary64_v1_text(&text).unwrap();
    assert!(parsed.canonical_json().unwrap_err().contains("BYTE-LIMIT"));
    assert!(canonical_json_binary64_v1_text(&text)
        .unwrap_err()
        .contains("BYTE-LIMIT"));
    let exactly = format!("\"{}\"", "a".repeat(MAX_DOCUMENT_BYTES - 2));
    let canonical = canonical_json_binary64_v1_text(&exactly).unwrap();
    assert_eq!(canonical.len(), MAX_DOCUMENT_BYTES);
    assert_eq!(
        canonical_json_binary64_v1_text(&canonical).unwrap(),
        canonical
    );
}

#[test]
fn direct_snapshot_resource_admission_matches_its_canonical_text() {
    let key = "k".repeat(4 * 1024 * 1024);
    let value = json!({ key: [0, 0, 0, 0] });
    assert!(canonical_json_binary64_v1_value(&value)
        .unwrap_err()
        .contains("PATH-LIMIT"));
    let escaped_key = "/~".repeat(2 * 1024 * 1024);
    let escaped = json!({ escaped_key: [0] });
    assert!(canonical_json_binary64_v1_value(&escaped)
        .unwrap_err()
        .contains("PATH-LIMIT"));
    for value in [
        json!({"a/~": [1e160, {"😀": "é"}]}),
        json!([1, 2, 3]),
        json!(null),
    ] {
        let rendered = canonical_json_binary64_v1_value(&value).unwrap();
        assert_eq!(
            canonical_json_binary64_v1_text(&rendered).unwrap(),
            rendered
        );
    }
}

#[test]
fn shared_admission_reenters_for_structural_boundaries_and_mixed_values() {
    let mut depth = json!(0);
    for _ in 0..MAX_DEPTH {
        depth = json!([depth]);
    }
    let text = canonical_json_binary64_v1_value(&depth).unwrap();
    assert_eq!(canonical_json_binary64_v1_text(&text).unwrap(), text);
    assert!(canonical_json_binary64_v1_value(&json!([depth]))
        .unwrap_err()
        .contains("DEPTH-LIMIT"));
    let nodes = Value::Array(vec![json!(0); MAX_VALUE_NODES - 1]);
    let text = canonical_json_binary64_v1_value(&nodes).unwrap();
    assert_eq!(canonical_json_binary64_v1_text(&text).unwrap(), text);
    let too_many = Value::Array(vec![json!(0); MAX_VALUE_NODES]);
    assert!(canonical_json_binary64_v1_value(&too_many)
        .unwrap_err()
        .contains("NODE-LIMIT"));
    // root/key + three key/index paths consume4*k+10 bytes. This sits within
    // two bytes of the shared path limit while remaining well below8MiB JSON.
    let key = "k".repeat((MAX_CAPTURE_PATH_BYTES - 10) / 4);
    let close_path_limit = json!({key: [0, 0, 0]});
    let text = canonical_json_binary64_v1_value(&close_path_limit).unwrap();
    assert_eq!(canonical_json_binary64_v1_text(&text).unwrap(), text);
    for index in 0..32 {
        let value = json!({
            format!("a/~😀{index}"): [1e160, 5e-324, -5e-324, null, true, false,
                {"nfc":"é", "nfd":"e\u{301}", "escapes":"\u{0}\n\"\\"}],
            "safe_counts": [-9_007_199_254_740_991_i64, index, 9_007_199_254_740_991_i64]
        });
        let rendered = canonical_json_binary64_v1_value(&value).unwrap();
        assert_eq!(
            canonical_json_binary64_v1_text(&rendered).unwrap(),
            rendered
        );
    }
}
