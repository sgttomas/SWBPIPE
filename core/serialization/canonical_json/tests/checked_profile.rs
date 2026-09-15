use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;

#[test]
fn canonicalizes_checked_boundaries() {
    assert_eq!(canonical_json_checked_v1_text(r#"{"\uFF5A":1,"😀":2,"n":-0.0}"#).unwrap(), r#"{"n":0,"😀":2,"ｚ":1}"#);
    assert_eq!(canonical_json_checked_v1_text("5e-324").unwrap(), "5e-324");
    assert_eq!(canonical_json_checked_v1_text("9007199254740991").unwrap(), "9007199254740991");
}

#[test]
fn rejects_duplicates_and_unsafe_integrals() {
    assert!(canonical_json_checked_v1_text(r#"{"a":1,"a":2}"#).unwrap_err().contains("duplicate"));
    assert!(canonical_json_checked_v1_text("9007199254740992").unwrap_err().contains("UNSAFE"));
    assert!(canonical_json_checked_v1_text("1.7976931348623157e308").unwrap_err().contains("UNSAFE"));
}
