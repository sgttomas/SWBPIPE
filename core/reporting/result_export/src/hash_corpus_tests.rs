//! T4-S6 byte-identity regression: over every JSON document the project
//! commits (fixtures, corpora, successors, publications, references, evidence;
//! each JSONL line separately), the streamed canonical text and digest are the
//! former path's, and every embedded retained or source-block publication and
//! receipt hashes alike through both paths. The former path lives only here.
use super::{domain_hash, domain_hash_without};
use open_pipe_stress_canonical_json::{canonical_json_checked_v1_text, checked_v1_domain_canonical, DomainPayload};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// The former `domain_hash`: copy into a wrapper, serialize, checked-parse, canonicalize.
fn former_text(domain: &str, payload: &Value) -> Result<String, String> {
    canonical_json_checked_v1_text(&serde_json::to_string(&json!({"domain": domain, "payload": payload})).map_err(|e| e.to_string())?)
}
fn sha(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

/// Every committed-looking JSON/JSONL file under the project, skipping build and dependency trees.
pub(crate) fn corpus_files() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().path()).collect();
        entries.sort();
        for path in entries {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if path.is_dir() {
                if !(name.starts_with('.') || name == "target" || name == "node_modules" || name == "dist") {
                    walk(&path, out);
                }
            } else if name.ends_with(".json") || name.ends_with(".jsonl") {
                out.push(path);
            }
        }
    }
    let project = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut out = Vec::new();
    walk(&project, &mut out);
    out
}

/// Each JSON document a file holds (a JSONL file holds one per non-empty line); unparsable files are skipped.
pub(crate) fn corpus_documents(path: &Path) -> Vec<Value> {
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(_) => return Vec::new(),
    };
    if path.extension().is_some_and(|e| e == "jsonl") {
        text.lines().filter(|l| !l.trim().is_empty()).filter_map(|l| serde_json::from_str(l).ok()).collect()
    } else {
        serde_json::from_str(&text).ok().into_iter().collect()
    }
}

#[derive(Default, Debug)]
struct Counts {
    files: usize,
    documents: usize,
    accepted: usize,
    refused: usize,
    publications: usize,
    receipts: usize,
    committed_publication_digests: usize,
    committed_receipt_digests: usize,
}

/// Publications and receipts: any object carrying a retained or source-block receipt.
fn embedded(value: &Value, counts: &mut Counts) {
    match value {
        Value::Array(items) => items.iter().for_each(|v| embedded(v, counts)),
        Value::Object(map) => {
            for (key, domain, receipt_domain) in [
                ("retained_precision", "retained_precision_publication_mp_v2", "retained_precision_receipt_mp_v2"),
                ("source_block_recovery", "source_blocks_publication_v1", "source_blocks_receipt_v1"),
            ] {
                let Some(receipt) = map.get(key) else { continue };
                let mut public = value.clone();
                public.as_object_mut().unwrap().remove(key);
                let former = former_text(domain, &public).map(|t| sha(&t));
                assert_eq!(domain_hash_without(domain, map, key), former, "{key} publication");
                counts.publications += 1;
                if former.as_ref().ok().map(String::as_str) == receipt["body"]["publication_sha256"].as_str() {
                    counts.committed_publication_digests += 1;
                }
                if receipt.get("body").is_some() {
                    let former = former_text(receipt_domain, &receipt["body"]).map(|t| sha(&t));
                    assert_eq!(domain_hash(receipt_domain, &receipt["body"]), former, "{key} receipt");
                    counts.receipts += 1;
                    if former.as_ref().ok().map(String::as_str) == receipt["receipt_sha256"].as_str() {
                        counts.committed_receipt_digests += 1;
                    }
                }
            }
            map.values().for_each(|v| embedded(v, counts));
        }
        _ => {}
    }
}

#[test]
fn streamed_canonical_text_and_digests_equal_the_former_path_over_the_committed_corpus() {
    let mut counts = Counts::default();
    for path in corpus_files() {
        let documents = corpus_documents(&path);
        counts.files += usize::from(!documents.is_empty());
        for document in documents {
            counts.documents += 1;
            let domain = "t4_s6_corpus";
            let former = former_text(domain, &document);
            let mut text = String::new();
            let streamed = checked_v1_domain_canonical(domain, DomainPayload::Value(&document), &mut text).map(|()| text);
            assert_eq!(streamed, former, "canonical text of {}", path.display());
            assert_eq!(domain_hash(domain, &document), former.as_ref().map(|t| sha(t)).map_err(Clone::clone), "digest of {}", path.display());
            if former.is_ok() { counts.accepted += 1 } else { counts.refused += 1 }
            embedded(&document, &mut counts);
        }
    }
    eprintln!("T4-S6 corpus: {counts:?}");
    // The corpus is the repository's; these floors only guard against walking nothing.
    assert!(counts.files > 1000 && counts.publications > 50 && counts.committed_publication_digests > 20, "{counts:?}");
}
