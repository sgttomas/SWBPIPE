//! The readability check's closed record per integrity code
//! (`reader_readability::STANDING_QUALITY`) is the priced reader's own table
//! (`result_export::nonlinear_support_evidence::standing_quality`), code for
//! code and field for field, in both directions.
//!
//! The reader's table is private to that crate, and the crate is kept
//! unchanged, so it is read from its source text. `include_str!` binds the
//! path at compile time: a moved or renamed file is a compile error, never an
//! empty read. Inside the file, a renamed function, a missing default arm, an
//! entry this parser does not recognise, or a table with no entries each fail
//! the test.
use open_pipe_stress_report_standing::reader_readability::{standing_quality, STANDING_QUALITY};
use std::collections::BTreeMap;

const READER: &str = include_str!("../../result_export/src/nonlinear_support_evidence.rs");

fn constant(name: &str) -> String {
    let lead = format!("const {name}: &str = \"");
    let start = READER.find(&lead).unwrap_or_else(|| panic!("constant {name} not found in the reader")) + lead.len();
    READER[start..].split('"').next().unwrap().to_string()
}

/// The reader's table: every arm between `fn standing_quality` and its
/// `_ => return None` arm.
fn readers_table() -> BTreeMap<String, Vec<String>> {
    let start = READER.find("fn standing_quality(code: &str) -> Option<[&'static str; 4]> {").expect("the reader's standing_quality");
    let body = &READER[start..];
    let body = &body[..body.find("_ => return None").expect("its default arm")];
    let mut table = BTreeMap::new();
    for line in body.lines().skip(1).map(str::trim).filter(|l| !l.is_empty() && *l != "Some(match code {") {
        let (key, record) = line.split_once(" => [").unwrap_or_else(|| panic!("an arm this test does not read: {line:?}"));
        let code = match key.strip_prefix('"') {
            Some(quoted) => quoted.strip_suffix('"').expect("a quoted code").to_string(),
            None => constant(key),
        };
        let fields: Vec<String> = record.strip_suffix("],").expect("a one-line record").split(", ").map(|f| f.trim_matches('"').to_string()).collect();
        assert_eq!(fields.len(), 4, "{code}: four fields");
        assert!(table.insert(code.clone(), fields).is_none(), "{code}: listed twice");
    }
    assert!(!table.is_empty(), "no entries read from the reader's table");
    table
}

#[test]
fn the_record_per_integrity_code_is_the_priced_readers() {
    let reader = readers_table();
    let mirror: BTreeMap<String, Vec<String>> =
        STANDING_QUALITY.iter().map(|(code, record)| (code.to_string(), record.iter().map(|f| f.to_string()).collect())).collect();
    assert_eq!(mirror.len(), STANDING_QUALITY.len(), "a code is listed twice in the mirror");
    let only_reader: Vec<_> = reader.keys().filter(|c| !mirror.contains_key(*c)).collect();
    let only_mirror: Vec<_> = mirror.keys().filter(|c| !reader.contains_key(*c)).collect();
    assert!(only_reader.is_empty(), "codes the reader publishes and the mirror lacks: {only_reader:?}");
    assert!(only_mirror.is_empty(), "codes the mirror holds and the reader does not publish: {only_mirror:?}");
    assert_eq!(mirror, reader);
    assert_eq!(standing_quality("NUMERICAL_INTEGRITY_CHECKS_PASSED_ENOUGH"), None);
}
