//! Independent parser-feature closure probe; JCS and the I-JSON guard stay fixed.
use open_pipe_stress_canonical_json::{canonical_json, canonical_json_checked_v1_text};
use serde_json::{json, Value};
#[test]
fn checked_text_preserves_same_unit_nonzero_binary64_bits() {
    let fixture:Value=serde_json::from_str(include_str!("../../../../fixtures/results/precision_transport_v0_3.json")).unwrap();
    for vector in fixture["vectors"].as_array().unwrap() {
            let bits=u64::from_str_radix(vector["bits_hex"].as_str().unwrap(),16).unwrap();
            let original:f64=vector["decimal"].as_str().unwrap().parse().unwrap();
            assert_eq!(original.to_bits(),bits);
            let input=json!({"value":original});
            let checked=canonical_json_checked_v1_text(&serde_json::to_string(&input).unwrap()).unwrap();
            assert_eq!(checked,canonical_json(&input),"{bits:016x}");
            let parsed:Value=serde_json::from_str(&checked).unwrap();
            assert_eq!(parsed["value"].as_f64().unwrap().to_bits(),original.to_bits());
    }
    assert!(canonical_json_checked_v1_text("{\"value\":9007199254740992}").is_err());
}
