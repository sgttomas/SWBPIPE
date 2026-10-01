

#[test]
fn i23_r10_positive_tail_targeted_line2_rounds_nearest() {
    // Isolate the existing R10 vector; no earlier targeted row is evaluated.
    let line = TARGETED.lines().nth(1).expect("I23 R10 corpus line 2");
    let (lhs, expected) = line.split_once(" = ").unwrap();
    let fields: Vec<&str> = lhs.split_whitespace().collect();
    assert_eq!(&fields[..4], &["sum", "4", "128", "3"], "I23 R10 selector");
    let precision = fields[2].parse::<u32>().unwrap();
    let terms = &fields[4..];
    assert_eq!(terms, &["w:+8p0", "w:+8p-128", "w:+8p-133"], "I23 R10 terms");
    assert_eq!(expected, "+80000000000000000000000000000001p0", "I23 R10 oracle identity");
    // Only run_targeted's existing numeric tok(&v)==expected assertion counts;
    // selectors, refusal paths, or an unrelated panic do not establish a kill.
    assert!(run_targeted::<4>(precision, terms, expected), "I23 R10 no refusal");
}
