//! The exact-route reports' closed statement table in Rust (W-11, DEL-08-01
//! Design `exact-route-report-sections.md` §2.1 and §3.5).
//!
//! The table is `fixtures/reports/exact_report_statements.json`, which the
//! TypeScript table's test (`apps/desktop/src/features/report/exactReportLimitations.test.ts`)
//! writes from the reviewed TypeScript source. It is embedded here, parsed
//! once and required to hold exactly [`EXACT_STATEMENT_COUNT`] entries with
//! unique lowercase 64-hex digests and non-empty statements; any other
//! content refuses every exact section. There is no other Rust copy: no Rust
//! path copies a producer string or strips a suffix. A producer string is
//! matched by the SHA-256 of its UTF-8 bytes. Standard claim fence applies
//! (claims taxonomy per DEC-081).

use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::OnceLock;

/// The embedded table, as the TypeScript table's test writes it.
pub const EXACT_STATEMENTS_JSON: &str = include_str!("../../../../fixtures/reports/exact_report_statements.json");
/// The number of distinct exact-route producer strings (§1.3).
pub const EXACT_STATEMENT_COUNT: usize = 19;
/// The refusal of a malformed table.
pub const EXACT_STATEMENT_TABLE_INVALID: &str = "REPORT-EXACT-STATEMENT-TABLE-INVALID";

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TableDocument {
    schema_version: String,
    #[allow(dead_code)]
    generator: String,
    entries: Vec<TableEntry>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TableEntry {
    producer_sha256: String,
    statement: String,
}

/// A parsed, well-formed statement table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExactStatementTable {
    by_digest: BTreeMap<String, String>,
}

impl ExactStatementTable {
    /// The statement for the producer string with this SHA-256, if any.
    pub fn statement_for(&self, producer_sha256: &str) -> Option<&str> {
        self.by_digest.get(producer_sha256).map(String::as_str)
    }

    /// Whether `text` is one of the table's statements.
    pub fn is_statement(&self, text: &str) -> bool {
        self.by_digest.values().any(|statement| statement == text)
    }

    /// The entries, `(producer_sha256, statement)`, in digest order.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.by_digest.iter().map(|(digest, statement)| (digest.as_str(), statement.as_str()))
    }

    pub fn len(&self) -> usize {
        self.by_digest.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_digest.is_empty()
    }
}

fn is_lower_hex_64(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// Parse a statement table document; anything but exactly
/// [`EXACT_STATEMENT_COUNT`] well-formed, uniquely keyed entries refuses.
pub fn parse_exact_statement_table(text: &str) -> Result<ExactStatementTable, String> {
    let fail = |detail: &str| Err(format!("{EXACT_STATEMENT_TABLE_INVALID}: {detail}"));
    let document: TableDocument = match serde_json::from_str(text) {
        Ok(document) => document,
        Err(error) => return fail(&format!("the table is not the table document form: {error}")),
    };
    if document.schema_version != "1.0.0" {
        return fail("the table's schema_version is not 1.0.0");
    }
    if document.entries.len() != EXACT_STATEMENT_COUNT {
        return fail(&format!("the table holds {} entries, not {EXACT_STATEMENT_COUNT}", document.entries.len()));
    }
    let mut by_digest = BTreeMap::new();
    for entry in document.entries {
        if !is_lower_hex_64(&entry.producer_sha256) {
            return fail("an entry's producer_sha256 is not 64 lowercase hex characters");
        }
        if entry.statement.trim().is_empty() {
            return fail("an entry's statement is empty");
        }
        if by_digest.insert(entry.producer_sha256, entry.statement).is_some() {
            return fail("two entries name the same producer_sha256");
        }
    }
    Ok(ExactStatementTable { by_digest })
}

/// The embedded table, parsed once; an error when its content is malformed.
pub fn exact_statement_table() -> Result<&'static ExactStatementTable, &'static str> {
    static TABLE: OnceLock<Result<ExactStatementTable, String>> = OnceLock::new();
    TABLE
        .get_or_init(|| parse_exact_statement_table(EXACT_STATEMENTS_JSON))
        .as_ref()
        .map_err(String::as_str)
}

/// The SHA-256 of a string's UTF-8 bytes, as 64 lowercase hex characters.
pub fn producer_sha256(text: &str) -> String {
    let digest = Sha256::digest(text.as_bytes());
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The reviewed statement for the producer string with this SHA-256, or
/// `None` when the table has none or is malformed.
pub fn exact_statement_for(producer_sha256: &str) -> Option<&'static str> {
    exact_statement_table().ok()?.statement_for(producer_sha256)
}

/// Whether `text` is a statement of the embedded table; `false` for every
/// text when the table is malformed.
pub fn is_exact_statement(text: &str) -> bool {
    exact_statement_table().is_ok_and(|table| table.is_statement(text))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    const COVERAGE: &str = include_str!("../../../../fixtures/reports/exact_route_limitations.json");
    const OMITTED_SENTENCE: &str =
        "No code compliance or professional acceptance is produced, and actual numerical admission remains separate.";

    fn coverage_strings() -> Vec<String> {
        let coverage: Value = serde_json::from_str(COVERAGE).unwrap();
        coverage["strings"].as_array().unwrap().iter().map(|text| text.as_str().unwrap().to_string()).collect()
    }

    #[test]
    fn the_embedded_table_parses_with_nineteen_entries() {
        let table = exact_statement_table().expect("the embedded table is well formed");
        assert_eq!(table.len(), EXACT_STATEMENT_COUNT);
        assert_eq!(table.len(), 19);
    }

    #[test]
    fn its_digests_are_the_coverage_fixtures_strings_digests() {
        let table = exact_statement_table().unwrap();
        let strings = coverage_strings();
        assert_eq!(strings.len(), 19);
        let mut fixture: Vec<String> = strings.iter().map(|text| producer_sha256(text)).collect();
        fixture.sort();
        let table_digests: Vec<String> = table.entries().map(|(digest, _)| digest.to_string()).collect();
        assert_eq!(table_digests, fixture);
    }

    #[test]
    fn each_statement_is_its_string_except_the_two_source_paragraphs() {
        let strings = coverage_strings();
        let mut omitted = 0;
        for text in &strings {
            let statement = exact_statement_for(&producer_sha256(text)).unwrap();
            match text.strip_suffix(OMITTED_SENTENCE) {
                Some(head) => {
                    omitted += 1;
                    assert_eq!(statement, head.strip_suffix(' ').unwrap());
                    assert!(text.contains("Other source families are declined by name."));
                }
                None => assert_eq!(statement, text),
            }
            assert!(is_exact_statement(statement));
            assert!(!statement.contains(OMITTED_SENTENCE));
        }
        assert_eq!(omitted, 2, "S9 and S10");
        assert!(!is_exact_statement("An invented statement."));
        assert_eq!(exact_statement_for(&producer_sha256("An invented limitation.")), None);
    }

    #[test]
    fn malformed_table_content_refuses_every_exact_section() {
        let document: Value = serde_json::from_str(EXACT_STATEMENTS_JSON).unwrap();
        let first_digest = document["entries"][0]["producer_sha256"].clone();
        let edits: Vec<Box<dyn Fn(&mut Value)>> = vec![
            Box::new(|d| {
                d["entries"].as_array_mut().unwrap().pop();
            }),
            Box::new(|d| {
                let extra = d["entries"][0].clone();
                d["entries"].as_array_mut().unwrap().push(extra);
            }),
            Box::new(move |d| d["entries"][1]["producer_sha256"] = first_digest.clone()),
            Box::new(|d| d["entries"][0]["producer_sha256"] = Value::from("ABCDEF")),
            Box::new(|d| {
                let upper = d["entries"][0]["producer_sha256"].as_str().unwrap().to_ascii_uppercase();
                d["entries"][0]["producer_sha256"] = Value::from(upper);
            }),
            Box::new(|d| d["entries"][0]["statement"] = Value::from("  ")),
            Box::new(|d| d["schema_version"] = Value::from("2.0.0")),
            Box::new(|d| d["unexpected"] = Value::from(true)),
            Box::new(|d| d["entries"][0]["note"] = Value::from("x")),
        ];
        for (index, edit) in edits.iter().enumerate() {
            let mut changed = document.clone();
            edit(&mut changed);
            let error = parse_exact_statement_table(&changed.to_string()).unwrap_err();
            assert!(error.starts_with(EXACT_STATEMENT_TABLE_INVALID), "edit {index}: {error}");
        }
        for text in ["", "not json", "[]", "{}"] {
            assert!(parse_exact_statement_table(text).is_err());
        }
        assert_eq!(parse_exact_statement_table(EXACT_STATEMENTS_JSON).as_ref().ok(), exact_statement_table().ok());
    }
}
