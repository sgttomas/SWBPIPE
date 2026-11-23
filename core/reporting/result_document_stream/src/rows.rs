//! One source row's entries in the six row lists, derived as `derive_document`'s row loop
//! derives them, and the comparisons `validate_document`'s row loop makes of them.
//!
//! Every classification comes from `open_pipe_stress_result_export`'s public functions
//! (`signature_for`, `canonical_metadata_for`, `complete_metadata`, `derivative`'s class
//! disclosure, `component_body`). Three decisions that `derivative.rs` makes inline are
//! restated here, each as one function with the signature a shared function would have, so
//! that a later unification is a deletion: [`effective_disposition`] (a), [`disclosure_reason`] (b) and
//! [`target_basis_and_evidence_kind`] (c).

use crate::canonical;
use open_pipe_stress_result_export::component_body::{derivative_agrees, derivative_record, COVERAGE_KEY, RECORD};
use open_pipe_stress_result_export::derivative::{
    checksum, class_disclosure, reference, RETAINED_ABSOLUTE_VERIFIED, RETAINED_NOT_COVERED,
};
use open_pipe_stress_result_export::json_value_eq;
use open_pipe_stress_result_export::retained_precision::AccuracyClass;
use open_pipe_stress_result_export::semantic_contract::{canonical_metadata_for, complete_metadata, signature_for};
use serde_json::{json, Value};

/// Where a row goes: a quantity, a review evidence entry or a disclosure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Disposition {
    Quantity,
    Review,
    Disclosed,
}

impl Disposition {
    pub(crate) const ALL: [Disposition; 3] = [Disposition::Quantity, Disposition::Review, Disposition::Disclosed];

    /// The disposition as a signature and the accounting state it.
    pub(crate) fn code(self) -> &'static str {
        match self {
            Disposition::Quantity => "exported_quantity",
            Disposition::Review => "exported_review",
            Disposition::Disclosed => "disclosed",
        }
    }

    /// A signature's `canonical_disposition`. The resident tables state only these three;
    /// `derive_document` would carry any other string into the accounting entry unread, so
    /// another is refused here rather than given a meaning.
    fn declared(code: &str) -> Result<Self, String> {
        Disposition::ALL
            .into_iter()
            .find(|d| d.code() == code)
            .ok_or_else(|| format!("SOURCE_DISPOSITION_UNKNOWN: {code}"))
    }

    pub(crate) fn slot(self) -> usize {
        self as usize
    }

    fn pointer_prefix(self) -> &'static str {
        match self {
            Disposition::Quantity => "/result_envelope/result_sets/0/values/",
            Disposition::Review => "/result_envelope/review_evidence/",
            Disposition::Disclosed => "/result_envelope/row_disclosures/",
        }
    }

    fn target_type(self) -> &'static str {
        match self {
            Disposition::Quantity => "quantity_result",
            Disposition::Review => "review_evidence",
            Disposition::Disclosed => "row_disclosure",
        }
    }

    pub(crate) fn target_scope(self) -> &'static str {
        match self {
            Disposition::Quantity => "derived_quantity_row",
            Disposition::Review => "derived_review_row",
            Disposition::Disclosed => "derived_disclosure_row",
        }
    }
}

/// A row's effective disposition and what downgraded it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Effective {
    pub(crate) disposition: Disposition,
    pub(crate) review_missing: bool,
    pub(crate) physical_missing: bool,
}

/// (a) The effective disposition of a row: its signature's declared disposition, disclosed
/// instead when a review row's metadata is incomplete, when a quantity of a mandatory family
/// has no canonical metadata, or when its validated class is disclosed.
///
/// Restates `derivative.rs` lines 246 to 258 (`derive_document`) and its copy at lines 569
/// to 589 (`validate_document`'s checks).
fn effective_disposition(
    declared: Disposition,
    family: &Value,
    metadata: Option<&Value>,
    row: &Value,
    class: Option<&(&'static str, String)>,
) -> Effective {
    let mandatory = matches!(family.as_str(), Some("force" | "moment" | "section_property"));
    let review_missing = declared == Disposition::Review && !complete_metadata(row);
    let physical_missing = declared == Disposition::Quantity && mandatory && metadata.is_none();
    let disposition = if review_missing || physical_missing || class.is_some() { Disposition::Disclosed } else { declared };
    Effective { disposition, review_missing, physical_missing }
}

/// (b) The reason code of a disclosed row.
///
/// Restates `derivative.rs` lines 316 to 335 (`derive_document`).
fn disclosure_reason(
    class: Option<&(&'static str, String)>,
    effective: &Effective,
    signature_known: bool,
    category: &Value,
) -> &'static str {
    if let Some((code, _)) = class {
        code
    } else if effective.review_missing {
        "review_metadata_incomplete"
    } else if effective.physical_missing {
        "physical_metadata_incomplete"
    } else if !signature_known {
        "unsupported_source_kind"
    } else if category == "basis_record" {
        "basis_annotation_not_quantity"
    } else if category == "diagnostic_relative_ratio" {
        "diagnostic_relative_ratio_non_governing"
    } else if matches!(category.as_str(), Some("count" | "state" | "flag" | "solver_mode")) {
        "discrete_evidence_not_ratio"
    } else {
        "diagnostic_evidence_not_physical_quantity"
    }
}

/// (c) The basis reference a target takes, and the evidence kind of a review target: the
/// row's own `basis_ref` when it has a non-null one; otherwise a review's `source_basis`
/// reference to its metadata basis and any other row's run reference. The evidence kind is
/// `assembled_load_review` for that category and `user_input_review` for any other.
///
/// Restates `derivative.rs` lines 281 to 291 and line 311 (`derive_document`), and their
/// copies at lines 685 to 695 and 708 to 713 (`validate_document`'s checks).
fn target_basis_and_evidence_kind(
    row: &Value,
    disposition: Disposition,
    category: &Value,
    run_basis: &Value,
) -> (Value, &'static str) {
    let basis = row.get("basis_ref").filter(|v| !v.is_null()).cloned().unwrap_or_else(|| {
        if disposition == Disposition::Review {
            // A review row's metadata is complete (else it is disclosed), so its basis is text.
            reference("source_basis", row["metadata"]["basis"].as_str().unwrap_or_default())
        } else {
            run_basis.clone()
        }
    });
    let evidence_kind = if category == "assembled_load_review" { "assembled_load_review" } else { "user_input_review" };
    (basis, evidence_kind)
}

/// What one row's derivation rests on, read once per document.
pub(crate) struct RowBasis<'a> {
    pub(crate) source: &'a Value,
    pub(crate) table: &'static Value,
    /// The envelope's `run_ref`, `provenance` and `unit_system_ref`, as the base states them.
    pub(crate) run_basis: Value,
    pub(crate) provenance: Value,
    pub(crate) unit_system: Value,
    pub(crate) origin_ref: Value,
    /// The carrier's payload reference and the row scope it gives.
    pub(crate) carrier_ref: Value,
    pub(crate) row_scope: &'static str,
    /// The original producer's payload reference, when the origin has an authentic producer.
    pub(crate) original_ref: Option<Value>,
    /// The envelope id, once the document has one.
    pub(crate) envelope_id: Option<String>,
    /// Whether the source is a successor, whose rows carry validated classes.
    pub(crate) classified: bool,
}

/// One row read against its signature.
pub(crate) struct Classified<'r> {
    pub(crate) id: &'r str,
    pub(crate) value: &'r Value,
    pub(crate) unit: &'r str,
    pub(crate) kind: &'r str,
    signature_known: bool,
    category: Value,
    dimension: Value,
    math: Value,
    family: Value,
    metadata: Option<Value>,
    class: Option<(&'static str, String)>,
    pub(crate) effective: Effective,
    observed: Value,
    object_ref: Value,
}

/// Everything `derive_document`'s loop makes of one row: the four entries, the row's
/// disposition, and the two digests the emissions reuse.
pub(crate) struct DerivedRow {
    pub(crate) disposition: Disposition,
    pub(crate) target: Value,
    pub(crate) annotation: Value,
    pub(crate) accounting: Value,
    pub(crate) witness: Option<Value>,
    /// SHA-256 of the source row's canonical text (both row checksums' value).
    pub(crate) row_digest: [u8; 32],
    /// SHA-256 of the target's canonical text (a witness's `target_row_checksum` value).
    pub(crate) target_digest: [u8; 32],
}

/// A checksum object of `checksum`'s shape whose value is a digest already computed.
fn checksum_of(digest: &[u8; 32], scope: &str, payload_ref: Value) -> Result<Value, String> {
    let mut object = checksum(&Value::Null, scope, payload_ref)?;
    object["value"] = json!(canonical::hex(digest));
    Ok(object)
}

impl<'a> RowBasis<'a> {
    /// The row read against its signature, refusing as `derive_document`'s loop refuses,
    /// in its order, after the id (which its caller reads and binds once).
    pub(crate) fn classify<'r>(&self, row: &'r Value, class: Option<&AccuracyClass>) -> Result<Classified<'r>, String> {
        let id = row["id"].as_str().filter(|s| !s.is_empty()).ok_or("SOURCE_ID_MISSING")?;
        row["value"].as_f64().filter(|x| x.is_finite()).ok_or("SOURCE_VALUE_INVALID")?;
        let value = &row["value"];
        let unit = row["unit"].as_str().filter(|s| !s.is_empty()).ok_or("SOURCE_UNIT_MISSING")?;
        let kind = row["kind"].as_str().ok_or("SOURCE_KIND_MISSING")?;
        let signature = signature_for(self.source, self.table, row)?;
        let s = signature.as_ref();
        let category = s.map(|s| s["category"].clone()).unwrap_or(json!("unknown"));
        let dimension = s.map(|s| s["derivative_target_dimension"].clone()).unwrap_or(Value::Null);
        let math = s.map(|s| s["source_physical_semantic_dimension"].clone()).unwrap_or(Value::Null);
        let declared = match s {
            Some(s) => Disposition::declared(s["canonical_disposition"].as_str().unwrap_or_default())?,
            None => Disposition::Disclosed,
        };
        let metadata = canonical_metadata_for(self.source, self.table, row);
        let family = s.map(|s| s["family"].clone()).unwrap_or(Value::Null);
        let class = class_disclosure(kind, unit, class);
        let effective = effective_disposition(declared, &family, metadata.as_ref(), row, class.as_ref());
        let observed = observed(row);
        let object_ref = reference("preview_entity", row["entity_ref"].as_str().ok_or("ENTITY_REF_MISSING")?);
        Ok(Classified {
            id,
            value,
            unit,
            kind,
            signature_known: s.is_some(),
            category,
            dimension,
            math,
            family,
            metadata,
            class,
            effective,
            observed,
            object_ref,
        })
    }

    fn received_checksum(&self, row_digest: &[u8; 32]) -> Result<Value, String> {
        checksum_of(row_digest, self.row_scope, self.carrier_ref.clone())
    }

    fn original_checksum(&self, row_digest: &[u8; 32]) -> Result<Value, String> {
        match &self.original_ref {
            Some(payload_ref) => checksum_of(row_digest, "raw_source_row", payload_ref.clone()),
            None => Ok(Value::Null),
        }
    }

    fn document_ref(&self) -> Result<Value, String> {
        let id = self.envelope_id.as_deref().ok_or("ENVELOPE_ID_MISSING")?;
        Ok(reference("derivative_document", id))
    }

    /// The row's target: its quantity, review evidence or disclosure entry.
    pub(crate) fn target(&self, c: &Classified, row: &Value, index: usize, row_digest: &[u8; 32]) -> Result<Value, String> {
        let annotation_ref = reference("source_annotation", &format!("source-annotation:{index}"));
        let disposition = c.effective.disposition;
        let (basis, evidence_kind) = target_basis_and_evidence_kind(row, disposition, &c.category, &self.run_basis);
        Ok(match disposition {
            Disposition::Quantity => {
                let mut target = json!({"result_id":c.id,"family":c.family,"object_ref":c.object_ref,"basis_ref":basis,"magnitude":c.value,"unit":c.unit,"dimension":c.dimension,"provenance":self.provenance,"source_kind":c.kind,"source_row_index":index,"source_annotation_ref":annotation_ref});
                if let Some(metadata) = &c.metadata {
                    target["metadata"] = metadata.clone();
                }
                if let Some(record) = derivative_record(row) {
                    target[RECORD] = record;
                }
                target
            }
            Disposition::Review => {
                // A review row's metadata is complete (else it is disclosed), so its location is text.
                let location = reference("source_location", row["metadata"]["location"].as_str().unwrap_or_default());
                json!({"evidence_id":c.id,"source_row_index":index,"source_result_id":c.id,"source_kind":c.kind,"evidence_kind":evidence_kind,"magnitude":c.value,"unit":c.unit,"dimension":c.dimension,"object_ref":c.object_ref,"basis_ref":basis,"location_ref":location,"source_annotation_ref":annotation_ref,"provenance":self.provenance})
            }
            Disposition::Disclosed => {
                let reason = disclosure_reason(c.class.as_ref(), &c.effective, c.signature_known, &c.category);
                let message = match &c.class {
                    Some((_, message)) => message.clone(),
                    None => format!("{}: {reason}; source value/unit and annotation retained; non-governing evidence", c.kind),
                };
                json!({"source_row_index":index,"source_result_id":c.id,"source_kind":c.kind,"source_value":c.value,"source_unit":c.unit,"source_dimension_present":c.observed["present"],"source_dimension":c.observed["value"],"declared_semantic_dimension":c.dimension,"source_physical_semantic_dimension":c.math,"semantic_category":c.category,"reason_code":reason,"object_ref":c.object_ref,"source_field_path":format!("/results/{index}"),"source_annotation_ref":annotation_ref,"received_carrier_row_checksum":self.received_checksum(row_digest)?,"original_producer_row_checksum":self.original_checksum(row_digest)?,"message":message})
            }
        })
    }

    /// The row's source annotation.
    pub(crate) fn annotation(&self, c: &Classified, row: &Value, index: usize) -> Value {
        let mut annotation = json!({"annotation_id":format!("source-annotation:{index}"),"source_row_index":index,"source_result_id":c.id,"metadata":row.get("metadata").cloned().unwrap_or(Value::Null),"basis_ref":row.get("basis_ref").cloned().unwrap_or(Value::Null),"source_result_refs":row.get("source_result_refs").cloned().unwrap_or(json!([])),"observed_carrier_dimension":c.observed,"source_physical_semantic_dimension":c.math,"derivative_target_dimension":c.dimension,"source_origin_ref":self.origin_ref});
        if let Some(record) = derivative_record(row) {
            annotation[RECORD] = record;
        }
        annotation
    }

    /// The row's accounting entry, from its id and kind (read and bound in the row sweep),
    /// its disposition and its place in its target list.
    pub(crate) fn accounting(&self, row: &Value, index: usize, disposition: Disposition, place: usize, row_digest: &[u8; 32]) -> Result<Value, String> {
        let id = row["id"].as_str().ok_or("SOURCE_ID_MISSING")?;
        Ok(json!({"source_row_index":index,"source_result_id":id,"source_kind":row["kind"],"source_field_path":format!("/results/{index}"),"received_carrier_row_checksum":self.received_checksum(row_digest)?,"original_producer_row_checksum":self.original_checksum(row_digest)?,"disposition":disposition.code(),"target_ref":reference(disposition.target_type(), id),"target_field_path":format!("{}{place}", disposition.pointer_prefix())}))
    }

    /// The row's unit preservation witness; a disclosed row has none.
    pub(crate) fn witness(&self, c: &Classified, index: usize, place: usize, row_digest: &[u8; 32], target_digest: &[u8; 32]) -> Result<Option<Value>, String> {
        let disposition = c.effective.disposition;
        if disposition == Disposition::Disclosed {
            return Ok(None);
        }
        let target_checksum = checksum_of(target_digest, disposition.target_scope(), self.document_ref()?)?;
        Ok(Some(json!({"witness_id":format!("value-unit-witness:{index}"),"source_result_ref":reference("received_result",c.id),"source_field_path":format!("/results/{index}"),"source_quantity":{"value":c.value,"unit":c.unit,"observed_dimension":c.observed},"target_result_ref":reference(disposition.target_type(), c.id),"target_field_path":format!("{}{place}", disposition.pointer_prefix()),"target_quantity":{"value":c.value,"unit":c.unit,"dimension":c.dimension},"target_quantity_policy":"preserve_received_value_and_unit_with_independent_semantic_dimension","export_unit_policy":"no_numerical_conversion_preserve_received_unit","conversion_performed":false,"unit_system_ref":self.unit_system,"provenance":self.provenance,"source_row_index":index,"received_carrier_row_checksum":self.received_checksum(row_digest)?,"original_producer_row_checksum":self.original_checksum(row_digest)?,"source_physical_semantic_dimension":c.math,"source_origin_ref":self.origin_ref,"target_row_checksum":target_checksum})))
    }

    /// One row as `derive_document`'s loop derives it, given the row's validated class and
    /// how many targets each list already holds (indexed by [`Disposition::slot`]); the
    /// caller reads the id, refuses a duplicate and counts the target in. Its refusals are
    /// that loop's, in its order.
    pub(crate) fn derive_row(&self, index: usize, row: &Value, class: Option<&AccuracyClass>, placed: &[usize; 3]) -> Result<DerivedRow, String> {
        let c = self.classify(row, class)?;
        let disposition = c.effective.disposition;
        let place = placed[disposition.slot()];
        let row_digest = canonical::sha256(row);
        let annotation = self.annotation(&c, row, index);
        let target = self.target(&c, row, index, &row_digest)?;
        let target_digest = canonical::sha256(&target);
        let accounting = self.accounting(row, index, disposition, place, &row_digest)?;
        let witness = self.witness(&c, index, place, &row_digest, &target_digest)?;
        Ok(DerivedRow { disposition, target, annotation, accounting, witness, row_digest, target_digest })
    }

    /// The comparisons `validate_document`'s row loop makes of row `i`'s accounting entry,
    /// annotation and target, with its refusal codes and in its order (`derivative.rs` lines
    /// 546 to 765). `place` is the target's index in its list, where the entry is written,
    /// and `class` the row's validated class. Every value is recomputed from the source row
    /// through the public functions and (a) and (c), not taken from the derivation. A
    /// non-disclosed row's witness expectation is returned for [`RowBasis::check_witness`], as
    /// `validate_document` keeps it for its witness loop.
    pub(crate) fn check_row(&self, i: usize, row: &Value, class: Option<&AccuracyClass>, a: &Value, ann: &Value, target: &Value, place: usize) -> Result<Option<Expected>, String> {
        let id = row["id"].as_str().ok_or("SOURCE_ID_MISSING")?;
        // The sweep refuses a repeated id before any check, as derive_document does.
        if a["source_row_index"] != i || a["source_result_id"] != row["id"] || a["source_kind"] != row["kind"] || a["source_field_path"] != format!("/results/{i}") {
            return Err("SOURCE_ACCOUNTING_IDENTITY_MISMATCH".into());
        }
        let path = a["target_field_path"].as_str().ok_or("TARGET_POINTER_MISSING")?;
        let signature = signature_for(self.source, self.table, row)?;
        let s = signature.as_ref();
        let md = canonical_metadata_for(self.source, self.table, row);
        let family = s.map(|s| s["family"].clone()).unwrap_or(Value::Null);
        let category = s.map(|s| s["category"].clone()).unwrap_or(json!("unknown"));
        let declared = match s {
            Some(s) => Disposition::declared(s["canonical_disposition"].as_str().unwrap_or_default())?,
            None => Disposition::Disclosed,
        };
        let class = class_disclosure(
            row["kind"].as_str().ok_or("SOURCE_KIND_MISSING")?,
            row["unit"].as_str().ok_or("SOURCE_UNIT_MISSING")?,
            class,
        );
        let disposition = effective_disposition(declared, &family, md.as_ref(), row, class.as_ref()).disposition;
        // The pointer names the entry this target is written as: its list and its place there
        // (validate_document resolves it in the whole document; TARGET_POINTER_UNRESOLVED).
        let prefix = disposition.pointer_prefix();
        if path.starts_with(prefix) && path != format!("{prefix}{place}") {
            return Err("TARGET_POINTER_UNRESOLVED".into());
        }
        let target_id = match disposition {
            Disposition::Quantity => &target["result_id"],
            Disposition::Review => &target["evidence_id"],
            Disposition::Disclosed => &target["source_result_id"],
        };
        if a["disposition"] != disposition.code()
            || !path.starts_with(prefix)
            || path[prefix.len()..].parse::<usize>().is_err()
            || target_id != &row["id"]
            || a["target_ref"] != reference(disposition.target_type(), id)
            || target["source_row_index"] != i
            || target["source_kind"] != row["kind"]
        {
            return Err("TARGET_IDENTITY_OR_DISPOSITION_MISMATCH".into());
        }
        let math = s.map(|s| s["source_physical_semantic_dimension"].clone()).unwrap_or(Value::Null);
        let dimension = s.map(|s| s["derivative_target_dimension"].clone()).unwrap_or(Value::Null);
        let observed = observed(row);
        let ann_id = format!("source-annotation:{i}");
        let ann_ref = reference("source_annotation", &ann_id);
        if ann["annotation_id"] != ann_id
            || ann["source_row_index"] != i
            || ann["source_result_id"] != row["id"]
            || !json_value_eq(&ann["metadata"], row.get("metadata").unwrap_or(&Value::Null))
            || ann["basis_ref"] != row.get("basis_ref").cloned().unwrap_or(Value::Null)
            || ann["source_result_refs"] != row.get("source_result_refs").cloned().unwrap_or(json!([]))
            || ann["observed_carrier_dimension"] != observed
            || ann["source_physical_semantic_dimension"] != math
            || ann["derivative_target_dimension"] != dimension
            || ann["source_origin_ref"] != self.origin_ref
            || target["source_annotation_ref"] != ann_ref
            || !derivative_agrees(ann, row)
        {
            return Err("SOURCE_ANNOTATION_MISMATCH".into());
        }
        if (disposition == Disposition::Quantity && !derivative_agrees(target, row))
            || (disposition != Disposition::Quantity && (target.get(RECORD).is_some() || target.get(COVERAGE_KEY).is_some()))
        {
            return Err("SOURCE_COMPONENT_BODY_MISMATCH".into());
        }
        let row_hash = checksum(row, self.row_scope, self.carrier_ref.clone())?;
        let original = match &self.original_ref {
            Some(payload_ref) => checksum(row, "raw_source_row", payload_ref.clone())?,
            None => Value::Null,
        };
        if a["received_carrier_row_checksum"] != row_hash || a["original_producer_row_checksum"] != original {
            return Err("SOURCE_ROW_HASH_BINDING_MISMATCH".into());
        }
        let (value, unit) = if disposition == Disposition::Disclosed {
            (&target["source_value"], &target["source_unit"])
        } else {
            (&target["magnitude"], &target["unit"])
        };
        if !json_value_eq(value, &row["value"])
            || unit != &row["unit"]
            || target["object_ref"] != reference("preview_entity", row["entity_ref"].as_str().ok_or("ENTITY_REF_MISSING")?)
        {
            return Err("SOURCE_TARGET_VALUE_OR_HASH_MISMATCH".into());
        }
        let (basis, evidence_kind) = target_basis_and_evidence_kind(row, disposition, &category, &self.run_basis);
        if disposition != Disposition::Disclosed && (target["basis_ref"] != basis || target["provenance"] != self.provenance) {
            return Err("TARGET_BASIS_OR_PROVENANCE_MISMATCH".into());
        }
        if disposition == Disposition::Review
            && (target["source_result_id"] != row["id"]
                || target["location_ref"] != reference("source_location", row["metadata"]["location"].as_str().unwrap_or_default())
                || target["evidence_kind"] != evidence_kind)
        {
            return Err("REVIEW_METADATA_MISMATCH".into());
        }
        if disposition == Disposition::Disclosed {
            if target["declared_semantic_dimension"] != dimension
                || target["source_physical_semantic_dimension"] != math
                || target["semantic_category"] != category
                || target["source_dimension_present"] != observed["present"]
                || target["source_dimension"] != observed["value"]
                || target["source_field_path"] != a["source_field_path"]
                || target["received_carrier_row_checksum"] != row_hash
                || target["original_producer_row_checksum"] != original
            {
                return Err("DISCLOSURE_SEMANTICS_MISMATCH".into());
            }
            if self.classified {
                let claimed = target["reason_code"] == RETAINED_ABSOLUTE_VERIFIED || target["reason_code"] == RETAINED_NOT_COVERED;
                let consistent = match &class {
                    Some((code, message)) => target["reason_code"] == *code && target["message"] == *message,
                    None => !claimed,
                };
                if !consistent {
                    return Err("DISCLOSURE_SEMANTICS_MISMATCH".into());
                }
            }
        } else {
            if target["dimension"] != dimension {
                return Err("TARGET_DIMENSION_MISMATCH".into());
            }
            if disposition == Disposition::Quantity && (target["family"] != family || target.get("metadata").cloned() != md) {
                return Err("PHYSICAL_SEMANTICS_MISMATCH".into());
            }
            return Ok(Some(Expected { disposition, row_hash, original, observed, math, dimension }));
        }
        Ok(None)
    }

    /// The comparisons `validate_document` makes of row `i`'s witness (`derivative.rs` lines
    /// 767 to 804), against the expectation [`RowBasis::check_row`] returned: a non-disclosed
    /// row has exactly one witness, equal to what its source row and target give; a disclosed
    /// row has none.
    pub(crate) fn check_witness(&self, i: usize, row: &Value, a: &Value, target: &Value, expected: Option<&Expected>, witness: Option<&Value>) -> Result<(), String> {
        let (expected, w) = match (expected, witness) {
            (None, None) => return Ok(()),
            (Some(expected), Some(w)) => (expected, w),
            _ => return Err("WITNESS_CARDINALITY".into()),
        };
        let Expected { disposition, row_hash, original, observed, math, dimension } = expected;
        if w["witness_id"] != format!("value-unit-witness:{i}")
            || w["source_row_index"] != i
            || w["source_result_ref"] != reference("received_result", row["id"].as_str().unwrap_or_default())
            || w["source_field_path"] != a["source_field_path"]
            || !json_value_eq(&w["source_quantity"], &json!({"value":row["value"],"unit":row["unit"],"observed_dimension":observed}))
            || w["target_result_ref"] != a["target_ref"]
            || w["target_field_path"] != a["target_field_path"]
            || !json_value_eq(&w["target_quantity"], &json!({"value":row["value"],"unit":row["unit"],"dimension":dimension}))
            || w["received_carrier_row_checksum"] != *row_hash
            || w["original_producer_row_checksum"] != *original
            || w["source_physical_semantic_dimension"] != *math
            || w["source_origin_ref"] != self.origin_ref
            || w["unit_system_ref"] != self.unit_system
            || w["provenance"] != self.provenance
            || w["conversion_performed"] != false
            || w["target_quantity_policy"] != "preserve_received_value_and_unit_with_independent_semantic_dimension"
            || w["export_unit_policy"] != "no_numerical_conversion_preserve_received_unit"
            || w["target_row_checksum"] != checksum(target, disposition.target_scope(), self.document_ref()?)?
        {
            return Err("WITNESS_MISMATCH".into());
        }
        Ok(())
    }
}

/// What `validate_document`'s row loop keeps of a non-disclosed row for its witness loop.
pub(crate) struct Expected {
    disposition: Disposition,
    row_hash: Value,
    original: Value,
    observed: Value,
    math: Value,
    dimension: Value,
}

/// The row's carrier dimension as observed: present or not, and its value.
fn observed(row: &Value) -> Value {
    json!({"present":row.get("dimension").is_some(),"value":row.get("dimension").cloned().unwrap_or(Value::Null)})
}
