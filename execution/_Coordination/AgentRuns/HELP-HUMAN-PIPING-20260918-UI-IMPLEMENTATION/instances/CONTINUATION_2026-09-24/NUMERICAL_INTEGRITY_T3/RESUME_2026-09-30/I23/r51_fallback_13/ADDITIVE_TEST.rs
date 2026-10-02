

#[test]
fn i23_r51_loadonly_y345_p128_retains_bounded_fallback() {
    // Contingent isolation of the existing named R7-M12 witness only.
    // SCALE remains state3; only this case's typed ResidualGate can qualify.
    let checked = e_unit(|name| name == "LOADONLY-y345", &[128]);
    assert_eq!(checked, 1, "I23 R51 exactly one LOADONLY-y345/p128 case");
}
