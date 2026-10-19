//! T4-S6 byte-identity regression: over every JSON document under the
//! project's `fixtures/` and `core/` trees (result fixtures, corpora,
//! successors, publications, requests and crate test fixtures; each JSONL line
//! separately), the streamed canonical text and digest are the former path's,
//! and every embedded retained or source-block publication and receipt hashes
//! alike through both paths. Every receipt hashes to its committed
//! `receipt_sha256`, and every publication is either an original (its digest is
//! its committed `publication_sha256`) or a transported derivative carrying an
//! original's receipt unchanged. `T4_S6_HASH_CORPUS_ROOT` replaces the two roots
//! with one supplied directory (T4-S6 ran it over the whole project). The
//! former path lives only here.
use open_pipe_stress_result_export::source_blocks::{domain_hash, domain_hash_without};
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

/// Every JSON/JSONL file under the corpus roots, skipping build, dependency and hidden trees.
fn corpus_files() -> Vec<PathBuf> {
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
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let roots = match std::env::var_os("T4_S6_HASH_CORPUS_ROOT") {
        Some(root) => vec![PathBuf::from(root)],
        None => vec![manifest.join("../../../fixtures"), manifest.join("../..")],
    };
    let mut out = Vec::new();
    for root in roots {
        walk(&root, &mut out);
    }
    out
}

/// Each JSON document a file holds (a JSONL file holds one per non-empty line); unparsable files are skipped.
fn corpus_documents(path: &Path) -> Vec<Value> {
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
    transported_publications: usize,
    committed_receipt_digests: usize,
}

/// One embedded publication: where it is, its digest through the former path, its
/// committed `publication_sha256` and its receipt.
struct Publication {
    at: String,
    digest: Result<String, String>,
    committed: Option<String>,
    receipt: Value,
}

/// Publications and receipts: any object carrying a retained or source-block receipt.
/// Every receipt must hash, through both paths, to its committed `receipt_sha256`.
fn embedded(value: &Value, at: &str, counts: &mut Counts, publications: &mut Vec<Publication>) {
    match value {
        Value::Array(items) => items.iter().enumerate().for_each(|(i, v)| embedded(v, &format!("{at}/{i}"), counts, publications)),
        Value::Object(map) => {
            for (key, domain, receipt_domain) in [
                ("retained_precision", "retained_precision_publication_mp_v2", "retained_precision_receipt_mp_v2"),
                ("source_block_recovery", "source_blocks_publication_v1", "source_blocks_receipt_v1"),
            ] {
                let Some(receipt) = map.get(key) else { continue };
                let mut public = value.clone();
                public.as_object_mut().unwrap().remove(key);
                let former = former_text(domain, &public).map(|t| sha(&t));
                assert_eq!(domain_hash_without(domain, map, key), former, "{key} publication at {at}");
                counts.publications += 1;
                // A receipt is an object with a `body` and a `receipt_sha256` (a schema's
                // property of the same name is not one).
                if !(receipt["body"].is_object() && receipt.get("receipt_sha256").is_some()) {
                    continue;
                }
                let receipt_former = former_text(receipt_domain, &receipt["body"]).map(|t| sha(&t));
                assert_eq!(domain_hash(receipt_domain, &receipt["body"]), receipt_former, "{key} receipt at {at}");
                counts.receipts += 1;
                assert_eq!(
                    receipt_former.as_ref().ok().map(String::as_str),
                    receipt["receipt_sha256"].as_str(),
                    "{key} receipt at {at}: committed receipt_sha256"
                );
                counts.committed_receipt_digests += 1;
                publications.push(Publication {
                    at: format!("{at} ({key})"),
                    digest: former,
                    committed: receipt["body"]["publication_sha256"].as_str().map(str::to_owned),
                    receipt: receipt.clone(),
                });
            }
            map.iter().for_each(|(k, v)| embedded(v, &format!("{at}/{k}"), counts, publications));
        }
        _ => {}
    }
}

/// The publication anchors. A publication whose digest equals its committed
/// `publication_sha256` is an original publication. Any other is a transported
/// derivative (a document, analysis run, stress-neutral or derivative envelope that
/// carries its source's receipt unchanged): its digest differs, as expected, and it
/// must carry, byte for byte, the receipt of an original publication in the corpus
/// whose digest is the committed one. Nothing else is accepted.
fn check_anchors(publications: &[Publication], counts: &mut Counts) {
    let originals: Vec<&Publication> =
        publications.iter().filter(|p| p.committed.is_some() && p.digest.as_ref().ok() == p.committed.as_ref()).collect();
    for p in publications {
        let committed = p.committed.as_deref().unwrap_or_else(|| panic!("{}: no committed publication_sha256", p.at));
        if p.digest.as_deref().ok() == Some(committed) {
            counts.committed_publication_digests += 1;
            continue;
        }
        let anchor = originals
            .iter()
            .find(|o| o.committed.as_deref() == Some(committed))
            .unwrap_or_else(|| panic!("{}: publication_sha256 {committed} is neither its own digest nor an original publication's", p.at));
        assert_eq!(p.receipt, anchor.receipt, "{}: transported receipt differs from its original at {}", p.at, anchor.at);
        counts.transported_publications += 1;
    }
}

#[test]
fn streamed_canonical_text_and_digests_equal_the_former_path_over_the_committed_corpus() {
    let mut counts = Counts::default();
    let mut publications = Vec::new();
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
            embedded(&document, &path.display().to_string(), &mut counts, &mut publications);
        }
    }
    check_anchors(&publications, &mut counts);
    eprintln!("T4-S6 corpus: {counts:?}");
    assert_eq!(counts.committed_receipt_digests, counts.receipts, "every receipt matches its committed digest");
    assert_eq!(counts.committed_publication_digests + counts.transported_publications, counts.receipts, "{counts:?}");
    // The corpus is the repository's; these floors only guard against walking nothing.
    assert!(counts.files > 400 && counts.publications > 50 && counts.committed_publication_digests > 20, "{counts:?}");
}
