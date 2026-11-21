//! The result export document (DEL-08-04) written in canonical order from the held source,
//! without the document ever being one value.
//!
//! `open_pipe_stress_result_export::derivative::derive_document` builds the document as one
//! value and returns it. [`DocumentSource`] produces the same document, byte for byte in its
//! RFC 8785 canonical text, with the same derivative hash and the same refusals, from the
//! same arguments, while holding only the header (the document with its six row lists empty)
//! and a few facts per source row:
//!
//! - [`DocumentSource::new`] prepares the header, reads every row once (its id, disposition
//!   and place, checking each row's entries as `validate_document` checks them) and makes the
//!   first emission: the document without `result_envelope.reproducibility.derivative_hash`,
//!   into a SHA-256, which is the derivative hash as `derive_document` computes it.
//! - The second emission ([`DocumentSource::emission`], [`DocumentSource::write_to`]) writes
//!   the whole document, the hash member in place, to a caller's stage in bounded chunks. It
//!   is also a content source ([`SourceObject`]) that a visitor walks: what it stages is
//!   exactly the canonical text of what it shows. It reports success only after checking that
//!   its bytes, less the hash member, digest to the first emission's hash.
//! - [`DocumentSource::verify`] re-emits and compares with a written file in constant memory.
//!
//! The six row lists are `result_envelope.result_sets[0].values`, `review_evidence`,
//! `row_accounting`, `row_disclosures`, `source_annotations` and
//! `unit_preservation_witnesses`. The hash member sorts before every row list in canonical
//! order, which is why there are two emissions.
//!
//! Every row and metadata meaning comes from `open_pipe_stress_result_export`'s public
//! functions. What `derivative.rs` decides inline is restated in this crate's private
//! functions, each naming the lines it restates (see `rows` and `header`).
//!
//! Where `derive_document` panics on a malformed base (a base, envelope, first result set or
//! reproducibility member that is not an object, or a missing envelope id), [`DocumentSource::new`]
//! refuses instead, at the same point, with the codes `DERIVATIVE_BASE_NOT_OBJECT`,
//! `RESULT_ENVELOPE_NOT_OBJECT`, `RESULT_SET_NOT_OBJECT`, `REPRODUCIBILITY_NOT_OBJECT` and
//! `ENVELOPE_ID_MISSING`.

mod canonical;
mod emit;
mod header;
mod rows;
mod source_traits;

pub use emit::{walk_whole, Emission, Emitted};
pub use source_traits::{Member, Peek, SourceList, SourceObject, WalkStopped, MAX_SOURCE_NESTING};

#[doc(hidden)]
pub use canonical::canonical_text;

use header::{header_decisions, validate_header, DerivedHeader};
use open_pipe_stress_result_export::derivative::{checksum, guard_json, reference};
use open_pipe_stress_result_export::retained_precision::AccuracyClass;
use open_pipe_stress_result_export::semantic_contract::{for_source, retained_row_classes};
use rows::{Disposition, RowBasis};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

/// One of the six row lists.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RowList {
    Values,
    Reviews,
    Accounting,
    Disclosures,
    Annotations,
    Witnesses,
}

impl RowList {
    pub(crate) const ALL: [RowList; 6] =
        [RowList::Values, RowList::Reviews, RowList::Accounting, RowList::Disclosures, RowList::Annotations, RowList::Witnesses];

    /// The member name of the list (`values` inside the one result set).
    pub(crate) fn name(self) -> &'static str {
        match self {
            RowList::Values => "values",
            RowList::Reviews => "review_evidence",
            RowList::Accounting => "row_accounting",
            RowList::Disclosures => "row_disclosures",
            RowList::Annotations => "source_annotations",
            RowList::Witnesses => "unit_preservation_witnesses",
        }
    }

    /// Whether a row of this disposition has an entry in the list.
    pub(crate) fn holds(self, disposition: Disposition) -> bool {
        match self {
            RowList::Values => disposition == Disposition::Quantity,
            RowList::Reviews => disposition == Disposition::Review,
            RowList::Disclosures => disposition == Disposition::Disclosed,
            RowList::Witnesses => disposition != Disposition::Disclosed,
            RowList::Accounting | RowList::Annotations => true,
        }
    }

    /// The list a row's target is written to.
    fn of_target(disposition: Disposition) -> RowList {
        match disposition {
            Disposition::Quantity => RowList::Values,
            Disposition::Review => RowList::Reviews,
            Disposition::Disclosed => RowList::Disclosures,
        }
    }
}

/// A member of the root, the envelope or the one result set.
pub(crate) enum Slot<'a> {
    /// The header's own member.
    Owned(Value),
    /// A member copied from the source, read where it is.
    Borrowed(&'a Value),
    /// `reproducibility`, whose `derivative_hash` member the payload digest leaves out.
    Reproducibility(Value),
    Envelope,
    ResultSets,
    List(RowList),
}

/// What is kept of one source row between emissions.
pub(crate) struct RowFact {
    pub(crate) disposition: Disposition,
    pub(crate) place: u32,
    pub(crate) class: Option<AccuracyClass>,
    pub(crate) row_digest: [u8; 32],
    pub(crate) target_digest: [u8; 32],
}

/// The result export document of one held source, emitted on demand. See the crate docs.
pub struct DocumentSource<'a> {
    rows: &'a [Value],
    basis: RowBasis<'a>,
    facts: Vec<RowFact>,
    /// Per row list, the SHA-256 of its entries' canonical texts, in order, as checked.
    list_digests: [[u8; 32]; 6],
    root: Vec<(String, Slot<'a>)>,
    envelope: Vec<(String, Slot<'a>)>,
    set: Vec<(String, Slot<'a>)>,
    /// The derivative hash (lowercase hexadecimal), once the first emission has made it.
    derivative: String,
}

/// Members in canonical order (RFC 8785 §3.2.3).
fn sorted<'a>(mut members: Vec<(String, Slot<'a>)>) -> Vec<(String, Slot<'a>)> {
    members.sort_by(|(left, _), (right, _)| left.encode_utf16().cmp(right.encode_utf16()));
    members
}

fn owned(map: Map<String, Value>) -> Vec<(String, Slot<'static>)> {
    map.into_iter().map(|(name, value)| (name, Slot::Owned(value))).collect()
}

impl<'a> DocumentSource<'a> {
    /// Prepares the document `derive_document(base, model, source, origin, request)` would
    /// return, refusing exactly where and as it refuses (in its order: its own checks, its
    /// row loop, and then `validate_document`'s), and makes the first emission.
    pub fn new(base: Value, model: &Value, source: &'a Value, origin: Value, request: Option<&Value>) -> Result<Self, String> {
        let (table, version) = for_source(source)?;
        let classes = retained_row_classes(source)?;
        guard_json(model)?;
        guard_json(source)?;
        guard_json(&base)?;
        guard_json(&origin)?;
        if source["status"]["mechanics"] != "MECHANICS_SOLVED" {
            return Err("SOURCE_NOT_SOLVED".into());
        }
        if model["project"]["id"] != source["model_ref"] {
            return Err("SOURCE_MODEL_IDENTITY_MISMATCH".into());
        }
        let rows = source["results"].as_array().ok_or("SOURCE_ROWS_MISSING")?;
        if rows.is_empty() {
            return Err("SOLVED_SOURCE_ROWS_EMPTY".into());
        }
        let mut root = match base {
            Value::Object(map) => map,
            Value::Null => Map::new(),
            _ => return Err("DERIVATIVE_BASE_NOT_OBJECT".into()),
        };
        let mut envelope = match root.remove("result_envelope") {
            Some(Value::Object(map)) => map,
            None | Some(Value::Null) => Map::new(),
            Some(_) => return Err("RESULT_ENVELOPE_NOT_OBJECT".into()),
        };
        let mechanics_set = match envelope.remove("result_sets") {
            Some(Value::Array(sets)) => sets.into_iter().next().unwrap_or(Value::Null),
            _ => Value::Null,
        };
        envelope.insert("schema_version".into(), json!(version));
        let decisions = header_decisions(source, origin["received_carrier_checksum"]["payload_scope"].as_str());
        let mut borrowed: Vec<(String, Slot<'a>)> = Vec::new();
        let mut copy = |envelope: &mut Map<String, Value>, key: &str| {
            envelope.remove(key);
            borrowed.push((key.to_owned(), Slot::Borrowed(&source[key])));
        };
        for key in ["producer", "numerical_quality", "formulation_basis"] {
            copy(&mut envelope, key);
        }
        if decisions.carries_contract_evidence {
            copy(&mut envelope, "contract_evidence");
        }
        if decisions.carries_source_blocks {
            copy(&mut envelope, "source_block_recovery");
        } else if envelope.contains_key("source_block_recovery") {
            return Err("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN".into());
        }
        if decisions.is_successor {
            copy(&mut envelope, "retained_precision");
        }
        let identity = source["producer"]["semantic_contract_id"].as_str().ok_or("SEMANTIC_CONTRACT_ID_MISSING")?;
        envelope.insert("semantic_contract_ref".into(), reference("semantic_contract", identity));
        let model_ref = source["model_ref"].as_str().ok_or("MODEL_REF_MISSING")?;
        envelope.insert("model_ref".into(), reference("model_payload", model_ref));
        let origin_ref = reference("source_origin_binding", origin["origin_id"].as_str().ok_or("ORIGIN_ID_MISSING")?);
        let member = |name: &str| envelope.get(name).cloned().unwrap_or(Value::Null);
        let (run_basis, provenance, unit_system) = (member("run_ref"), member("provenance"), member("unit_system_ref"));
        let carrier_ref = origin["received_carrier_checksum"]["payload_ref"].clone();
        let authentic = origin["authentic_producer_available"].as_bool().ok_or("ORIGIN_AVAILABILITY_MISSING")?;
        let scope = origin["received_carrier_checksum"]["payload_scope"].as_str().ok_or("ORIGIN_SCOPE_MISSING")?;
        let source_digest = canonical::hex(&canonical::sha256(source));
        if origin["received_carrier_checksum"]["value"] != source_digest {
            return Err("SOURCE_CARRIER_HASH_MISMATCH".into());
        }
        if !authentic && !origin["original_producer_checksum"].is_null() {
            return Err("UNAVAILABLE_PRODUCER_HASH".into());
        }
        if let Some(refusal) = decisions.declared_dimensions {
            return Err(refusal.into());
        }
        let basis = RowBasis {
            source,
            table,
            run_basis,
            provenance,
            unit_system,
            origin_ref,
            carrier_ref,
            row_scope: match scope {
                "received_current_dimension_absent_carrier" => "received_current_dimension_absent_row",
                _ => "raw_source_row",
            },
            original_ref: authentic.then(|| origin["original_producer_checksum"]["payload_ref"].clone()),
            envelope_id: envelope.get("envelope_id").and_then(Value::as_str).map(str::to_owned),
            classified: classes.is_some(),
        };

        // The row sweep: derive_document's loop, and validate_document's comparisons of each
        // row's entries, whose first refusals are kept until the header has been checked.
        let mut ids = HashSet::new();
        let mut placed = [0usize; 3];
        let mut facts = Vec::with_capacity(rows.len());
        let mut lists: [Sha256; 6] = Default::default();
        let (mut row_refusal, mut witness_refusal) = (None, None);
        let mut text = Vec::new();
        let mut checked = |list: RowList, entry: &Value| {
            canonical::bytes(entry, &mut text);
            lists[list as usize].update(&text);
        };
        for (index, row) in rows.iter().enumerate() {
            let id = row["id"].as_str().filter(|s| !s.is_empty()).ok_or("SOURCE_ID_MISSING")?;
            if !ids.insert(id) {
                return Err("DUPLICATE_SOURCE_ID".into());
            }
            let class = classes.as_ref().and_then(|c| c.get(id));
            let derived = basis.derive_row(index, row, class, &placed)?;
            let place = placed[derived.disposition.slot()];
            placed[derived.disposition.slot()] += 1;
            if row_refusal.is_none() {
                row_refusal = basis
                    .check_row(index, row, class, &derived.accounting, &derived.annotation, &derived.target, place)
                    .err();
            }
            if witness_refusal.is_none() {
                witness_refusal = basis
                    .check_witness(index, row, &derived.accounting, &derived.target, derived.disposition, derived.witness.as_ref())
                    .err();
            }
            checked(RowList::of_target(derived.disposition), &derived.target);
            checked(RowList::Accounting, &derived.accounting);
            checked(RowList::Annotations, &derived.annotation);
            if let Some(witness) = &derived.witness {
                checked(RowList::Witnesses, witness);
            }
            facts.push(RowFact {
                disposition: derived.disposition,
                place: u32::try_from(place).map_err(|_| "SOURCE_ROWS_TOO_MANY")?,
                class: class.cloned(),
                row_digest: derived.row_digest,
                target_digest: derived.target_digest,
            });
        }
        drop(ids);
        drop(classes);
        let list_digests = lists.map(|hasher| hasher.finalize().into());

        // derive_document's header after its loop.
        let set = match mechanics_set {
            Value::Object(map) => map,
            Value::Null => Map::new(),
            _ => return Err("RESULT_SET_NOT_OBJECT".into()),
        };
        envelope.insert(
            "unit_witness_policy".into(),
            json!("preserve_received_value_and_unit_with_independent_semantic_dimension"),
        );
        let mut reproducibility = match envelope.remove("reproducibility") {
            Some(Value::Object(map)) => map,
            None | Some(Value::Null) => Map::new(),
            Some(_) => return Err("REPRODUCIBILITY_NOT_OBJECT".into()),
        };
        reproducibility.insert("model_hash".into(), checksum(model, "model_payload", reference("model_payload", model_ref))?);
        reproducibility.insert(
            "raw_source_hashes".into(),
            if authentic { json!([origin["original_producer_checksum"].clone()]) } else { json!([]) },
        );
        reproducibility.insert("source_origin_bindings".into(), json!([origin]));
        reproducibility.insert(
            "request_hash".into(),
            match request {
                Some(v) => checksum(v, "request_payload", reference("runner_request", v["request_id"].as_str().unwrap_or("request")))?,
                None => Value::Null,
            },
        );
        reproducibility.insert("derivative_hash_excludes".into(), json!("result_envelope.reproducibility.derivative_hash"));
        reproducibility.remove("derivative_hash");
        root.insert("schema_version".into(), json!(version));
        let envelope_id = basis.envelope_id.clone().ok_or("ENVELOPE_ID_MISSING")?;

        // validate_document's header comparisons, then its row loop's first refusal, then its
        // witness loop's.
        let contract_evidence = if decisions.carries_contract_evidence { Some(&source["contract_evidence"]) } else { envelope.get("contract_evidence") };
        validate_header(
            &DerivedHeader {
                envelope: &envelope,
                contract_evidence,
                origin: &reproducibility["source_origin_bindings"][0],
                source_digest: &source_digest,
            },
            source,
            &decisions,
        )?;
        if let Some(refusal) = row_refusal.or(witness_refusal) {
            return Err(refusal);
        }

        // The row lists replace whatever the base states under their names.
        let mut set = set;
        set.remove(RowList::Values.name());
        let mut set_members = owned(set);
        set_members.push((RowList::Values.name().into(), Slot::List(RowList::Values)));
        for list in RowList::ALL.into_iter().filter(|list| *list != RowList::Values) {
            envelope.remove(list.name());
        }
        let mut envelope_members = owned(envelope);
        envelope_members.extend(borrowed);
        envelope_members.push(("reproducibility".into(), Slot::Reproducibility(Value::Object(reproducibility))));
        envelope_members.push(("result_sets".into(), Slot::ResultSets));
        for list in RowList::ALL.into_iter().filter(|list| *list != RowList::Values) {
            envelope_members.push((list.name().into(), Slot::List(list)));
        }
        let mut root_members = owned(root);
        root_members.push(("result_envelope".into(), Slot::Envelope));
        let mut document = DocumentSource {
            rows,
            basis,
            facts,
            list_digests,
            root: sorted(root_members),
            envelope: sorted(envelope_members),
            set: sorted(set_members),
            derivative: String::new(),
        };

        // The first emission: the document without its hash member, into a SHA-256.
        let first = Emission::new(&document, emit::Stage::Digest);
        let walked = walk_whole(&first);
        let (_, payload) = first.finish_payload(walked)?;
        let mut hash = checksum(&Value::Null, "derivative_document_excludes_own_hash", reference("derivative_document", &envelope_id))?;
        hash["value"] = json!(payload);
        for (name, slot) in document.envelope.iter_mut() {
            if let (true, Slot::Reproducibility(Value::Object(map))) = (name == "reproducibility", slot) {
                map.insert("derivative_hash".into(), hash.clone());
            }
        }
        document.derivative = payload;
        Ok(document)
    }

    /// The derivative hash: the SHA-256 (lowercase hexadecimal) of the document's canonical
    /// text without `result_envelope.reproducibility.derivative_hash`, the value that member
    /// states.
    pub fn derivative_sha256(&self) -> &str {
        &self.derivative
    }

    /// The document with its six row lists empty, as a value (a copy, including the members
    /// copied from the source; for inspection and tests, not for the emission).
    pub fn header(&self) -> Value {
        fn object(members: &[(String, Slot)], nested: &dyn Fn(&Slot) -> Value) -> Value {
            Value::Object(members.iter().map(|(name, slot)| (name.clone(), nested(slot))).collect())
        }
        fn value(slot: &Slot, document: &DocumentSource) -> Value {
            match slot {
                Slot::Owned(v) | Slot::Reproducibility(v) => v.clone(),
                Slot::Borrowed(v) => (*v).clone(),
                Slot::List(_) => json!([]),
                Slot::ResultSets => json!([object(&document.set, &|s| value(s, document))]),
                Slot::Envelope => object(&document.envelope, &|s| value(s, document)),
            }
        }
        object(&self.root, &|slot| value(slot, self))
    }

    /// The second emission, to `stage`: a content source the caller's visitor walks
    /// ([`SourceObject`]); [`Emission::finish`] then reports whether it succeeded.
    pub fn emission<'s>(&'s self, stage: &'s mut dyn std::io::Write) -> Emission<'s, 'a> {
        Emission::new(self, emit::Stage::Write(stage))
    }

    /// The whole document's canonical bytes, written to `stage` in bounded chunks.
    pub fn write_to(&self, stage: &mut dyn std::io::Write) -> Result<Emitted, String> {
        let emission = self.emission(stage);
        let walked = walk_whole(&emission);
        emission.finish_walk(walked)
    }

    /// Re-emits the document and compares it with what `written` reads, in constant memory:
    /// `Ok` only when the two are equal to the last byte.
    pub fn verify(&self, written: &mut dyn std::io::Read) -> Result<Emitted, String> {
        let emission = Emission::new(self, emit::Stage::Compare(written));
        let walked = walk_whole(&emission);
        emission.finish_walk(walked)
    }

    /// The entry of row `index` in `list`, derived for this emission.
    pub(crate) fn entry(&self, list: RowList, index: usize, fact: &RowFact) -> Result<Value, String> {
        let row = &self.rows[index];
        let place = fact.place as usize;
        if list == RowList::Accounting {
            return self.basis.accounting(row, index, fact.disposition, place, &fact.row_digest);
        }
        let c = self.basis.classify(row, fact.class.as_ref())?;
        if c.effective.disposition != fact.disposition {
            return Err("DOCUMENT_STREAM_DISPOSITION_CHANGED".into());
        }
        match list {
            RowList::Annotations => Ok(self.basis.annotation(&c, row, index)),
            RowList::Witnesses => {
                self.basis.witness(&c, index, place, &fact.row_digest, &fact.target_digest)?.ok_or_else(|| "WITNESS_CARDINALITY".into())
            }
            _ => self.basis.target(&c, row, index, &fact.row_digest),
        }
    }
}
