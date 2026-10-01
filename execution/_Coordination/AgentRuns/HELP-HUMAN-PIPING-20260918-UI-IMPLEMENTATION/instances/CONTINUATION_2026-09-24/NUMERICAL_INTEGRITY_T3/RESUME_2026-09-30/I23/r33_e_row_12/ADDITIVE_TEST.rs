

#[test]
fn i23_r33_n05_p128_e_rows_match_fixed_scale() {
    // Only this existing case/precision reaches the unchanged SCALE checks.
    // The original N05 p128 E-row digest/token assertion is the qualifier;
    // coverage, corrections, g, A-bar, body or setup failures do not count.
    let checked = e_unit(|name| name == "N05", &[128]);
    assert_eq!(checked, 1, "I23 R33 exactly one N05/p128 case");
}
