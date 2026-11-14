//! The readability check's closed record per integrity code
//! (`reader_readability::standing_quality`) is the one the priced reader holds
//! (`result_export::nonlinear_support_evidence::standing_quality`), entry for
//! entry. The reader's table is read from its source text, because it is
//! private to that crate and the crate is kept unchanged.
use open_pipe_stress_report_standing::reader_readability::standing_quality;

const READER: &str = include_str!("../../result_export/src/nonlinear_support_evidence.rs");

fn constant(name: &str) -> String {
    let lead = format!("const {name}: &str = \"");
    let start = READER.find(&lead).unwrap_or_else(|| panic!("constant {name} not found")) + lead.len();
    READER[start..].split('"').next().unwrap().to_string()
}

#[test]
fn the_record_per_integrity_code_is_the_priced_readers() {
    let body_start = READER.find("fn standing_quality(code: &str)").expect("the reader's standing_quality");
    let body = &READER[body_start..];
    let body = &body[..body.find("_ => return None").expect("its default arm")];
    let mut entries = Vec::new();
    for line in body.lines().map(str::trim).filter(|l| l.contains("=> [")) {
        let (key, record) = line.split_once(" => ").unwrap();
        let code = if let Some(quoted) = key.strip_prefix('"') { quoted.trim_end_matches('"').to_string() } else { constant(key) };
        let fields: Vec<String> = record.trim_end_matches(',').trim_matches(|c| c == '[' || c == ']').split(", ").map(|f| f.trim_matches('"').to_string()).collect();
        entries.push((code, fields));
    }
    assert_eq!(entries.len(), 8, "the reader publishes eight integrity codes");
    for (code, fields) in &entries {
        let mine = standing_quality(code).unwrap_or_else(|| panic!("{code}: not in the mirror"));
        assert_eq!(mine.to_vec(), *fields, "{code}");
    }
    assert_eq!(standing_quality("NUMERICAL_INTEGRITY_CHECKS_PASSED_ENOUGH"), None);
}
