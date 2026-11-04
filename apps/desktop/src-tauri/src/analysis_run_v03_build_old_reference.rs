//! RT-U4 test reference: the builder's record passes as they were before the
//! native-build repair, **verbatim** from `analysis_run_v03_build.rs` at
//! e231ea144 (each item below is that file's lines, unedited: `Built` 768–797,
//! `build` 813–957, `validate_built` 971–995, `Part` 1027–1037,
//! `write_result_ref` 1041–1088, `write_record` 1090–1212, `write_list`
//! 1214–1222, `Revision` 1224–1268, `header_record` 1270–1278,
//! `write_canonical_record` 1280–1285, `record_bytes` 1292–1313,
//! `header_bytes` 1315–1325, `write_header` 1327–1361, `preview_parsed`
//! 1450–1479). They keep their names inside this module, which shadows the
//! current items of the same name. The helpers they call are the current
//! module's, none of whose output changed: `interpret`, `canonical_sha256`,
//! `has_outside_profile` and `validate_route` only gained test-only pass
//! counters (`tests::passes`), and `Writer`'s own methods are unchanged
//! (its `RecordOut` implementation is new, and inherent methods take
//! precedence here). Used only to compare the current builder's bytes,
//! checksums, lengths, refusals and passes with these.
#![allow(dead_code, clippy::all)]
use super::*;

/// What the passes established: the record's members other than its rows,
/// borrowed from the source and the inputs, its checksums and its stored
/// length.
pub(crate) struct Built<'a> {
    /// The parsed held result the record was built from (the persistence
    /// writers read it; as the retained reader rewrote it, numbers only).
    pub(crate) source: &'a Value,
    rows: &'a [Value],
    pub(crate) route: Route,
    table: &'static Value,
    pub(crate) run_id: &'a str,
    model_ref: &'a str,
    solver_name: &'a Value,
    solver_version: &'a Value,
    solver_build_ref: &'a Value,
    manifest_ref: &'a Value,
    manifest_sha256: &'a Value,
    settings_ref: String,
    unit_system_ref: String,
    load_basis: Vec<Value>,
    producer_diagnostics: &'a [Value],
    rule_pack_diagnostics: Vec<Value>,
    statuses: Vec<Value>,
    pub(crate) rule_check_status: String,
    /// The copied members, in key order.
    copies: Vec<(&'static str, &'a Value)>,
    pub(crate) received_sha256: String,
    pub(crate) record_sha256: String,
    pub(crate) record_bytes: u64,
}

/// The record after the route's reader passed (`buildAnalysisRecord` from
/// `validateSourceRuleStatus` on, then `validateAnalysisRunV03`'s checks the
/// record can still fail), in the TypeScript's order. `reserve` admits the
/// fingerprints' allocation.
pub(crate) fn build<'a>(
    source: &'a Value,
    route: Route,
    inputs: &BuildInputs<'a>,
    reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>,
) -> Result<Built<'a>, CommandError<String>> {
    let refuse = |text: String| CommandError::Command(text);
    let manifest = inputs.manifest;
    // validateSourceRuleStatus
    let status = get(Some(source), "status", "result").map_err(refuse)?;
    let status = status.filter(|status| !status.is_null()).ok_or_else(|| refuse(shape("result.status")))?;
    let source_rule = get(Some(status), "rule_check", "result.status").map_err(refuse)?;
    if !nullish(source_rule) && !source_rule.and_then(Value::as_str).is_some_and(|rule| RULE_STATUSES.contains(&rule)) {
        return Err(refuse("ANALYSIS_SOURCE_RULE_STATUS_INVALID".into()));
    }
    let load_basis = expected_load_basis(source, inputs.load_basis_refs).map_err(refuse)?;
    let model_basis = get(Some(manifest.manifest), "model_basis", "inputManifest.manifest").map_err(refuse)?;
    let manifest_model = get(model_basis, "model_ref", "inputManifest.manifest.model_basis").map_err(refuse)?;
    let model_ref = source["model_ref"].as_str().expect("checked with the load basis");
    if !strict_eq_str(manifest_model, model_ref) {
        return Err(refuse("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH".into()));
    }
    // The rule-pack reference diagnostics, against the manifest's exclusions.
    let identities: Vec<String> = inputs.rule_pack_exclusions.iter().map(|e| rule_pack_identity(&e.rule_pack_id, &e.rule_pack_version)).collect();
    let rule_pack_diagnostics: Vec<Value> = inputs
        .rule_pack_exclusions
        .iter()
        .zip(&identities)
        .map(|(exclusion, identity)| json!({"source_annotation": rule_pack_ref_diagnostic(&exclusion.code, identity)}))
        .collect();
    let excluded: Vec<String> = manifest_excluded_rule_packs(manifest.manifest).map_err(refuse)?.iter().map(|(id, version)| rule_pack_identity(id, version)).collect();
    if identities != excluded {
        let none_resolved = manifest.manifest.get("active_rule_packs").and_then(Value::as_array).is_some_and(Vec::is_empty);
        return Err(refuse(if identities.is_empty() && !excluded.is_empty() && none_resolved {
            format!("{RULE_PACKS_NOT_RESOLVED}: {RULE_PACKS_NOT_RESOLVED_DETAIL}")
        } else {
            "ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH".into()
        }));
    }
    // The rows (`Promise.all`): an interpretation refusal first, whichever row.
    let rows = rows_of(source).map_err(refuse)?;
    let mut outside = false;
    for row in rows {
        match route.table {
            None => return Err(refuse(SOURCE_CONTRACT_UNSUPPORTED.into())),
            Some(table) => {
                interpret(table, row).map_err(refuse)?;
            }
        }
        outside = outside || has_outside_profile(row);
    }
    if outside {
        return Err(refuse(OUTSIDE_PROFILE.into()));
    }
    let table = route.table.expect("checked with the rows");
    let rule_check_status = match inputs.rule_check_status.filter(|value| !value.is_null()).or(source_rule.filter(|value| !value.is_null())) {
        None => RULE_STATUSES[0].to_string(),
        Some(Value::String(rule)) if RULE_STATUSES.contains(&rule.as_str()) => rule.clone(),
        Some(_) => return Err(refuse("ANALYSIS_RULE_STATUS_INVALID".into())),
    };
    let statuses = statuses(status.get("mechanics"), &rule_check_status);
    // The record literal's reads, in its order.
    let solver_basis = get(Some(manifest.manifest), "solver_basis", "inputManifest.manifest").map_err(refuse)?;
    let read = |key: &str| get(solver_basis, key, "inputManifest.manifest.solver_basis").map_err(refuse);
    let (solver_name, solver_version, solver_build_ref) = (read("solver_name")?, read("solver_version")?, read("solver_build_ref")?);
    let manifest_ref_ref = get(Some(manifest.manifest_ref), "ref", "inputManifest.manifest_ref").map_err(refuse)?;
    let settings_ref = format!("solver-settings:{}:{}", js_string(manifest_ref_ref), js_string(Some(manifest.manifest_sha256)));
    let unit_system_ref = format!("unit-system:{model_ref}:{}", js_string(Some(manifest.manifest_sha256)));
    let producer_diagnostics = match source.get("diagnostics") {
        Some(Value::Array(items)) => items.as_slice(),
        _ => return Err(refuse(shape("result.diagnostics is not an array"))),
    };
    if has_outside_profile(source) {
        return Err(refuse(OUTSIDE_PROFILE.into()));
    }
    let received_sha256 = canonical_sha256(source);
    let copied = |key: &'static str| (key, source.get(key));
    let copies: Vec<(&'static str, Option<&'a Value>)> = match route.name {
        "pressure_source" => vec![copied("contract_evidence"), copied("source_block_recovery")],
        _ if route.retained() => vec![copied("retained_precision")],
        _ => Vec::new(),
    };
    // The record checksum's checked text: `undefined` or a number outside the
    // profile, in the record's member order, then serde_json's depth.
    let run_id = source["run_id"].as_str().expect("checked with the load basis");
    let first_violation = [solver_name, solver_version, solver_build_ref]
        .into_iter()
        .map(checked_violation)
        .chain(rows.iter().flat_map(|row| [checked_violation(row.get("kind")), checked_violation(row.get("unit"))]))
        .chain([checked_violation(Some(manifest.manifest_ref)), checked_violation(Some(manifest.manifest_sha256))])
        .chain(copies.iter().map(|(_, value)| checked_violation(*value)))
        .flatten()
        .next();
    if let Some(violation) = first_violation {
        return Err(refuse(violation.into()));
    }
    let deep = |value: &Value, level: usize| level + container_depth(value) > PARSE_DEPTH_LIMIT;
    let too_deep = [(solver_name, 4), (solver_version, 4), (solver_build_ref, 5)].iter().any(|(value, level)| value.is_some_and(|v| deep(v, *level)))
        || rows.iter().any(|row| ["kind", "unit", "metadata"].iter().any(|key| row.get(key).is_some_and(|value| deep(value, 6))))
        || producer_diagnostics.iter().any(|item| deep(item, 5))
        || statuses.iter().any(|item| deep(item, 4))
        || deep(manifest.manifest_ref, 6)
        || deep(manifest.manifest_sha256, 6)
        || copies.iter().any(|(_, value)| value.is_some_and(|v| deep(v, 3)));
    if too_deep {
        return Err(refuse(RECURSION_LIMIT.into()));
    }
    let mut built = Built {
        source,
        rows,
        table,
        run_id,
        model_ref,
        solver_name: solver_name.expect("checked"),
        solver_version: solver_version.expect("checked"),
        solver_build_ref: solver_build_ref.expect("checked"),
        manifest_ref: manifest.manifest_ref,
        manifest_sha256: manifest.manifest_sha256,
        settings_ref,
        unit_system_ref,
        load_basis,
        producer_diagnostics,
        rule_pack_diagnostics,
        statuses,
        rule_check_status,
        copies: copies.into_iter().map(|(key, value)| (key, value.expect("checked"))).collect(),
        received_sha256,
        record_sha256: String::new(),
        record_bytes: 0,
        route,
    };
    let mut hasher = Sha256::new();
    write_record(&mut Writer::new(&mut hasher, Style::Canonical), &built, Part::Projection, None, &mut |row| canonical_sha256(row));
    built.record_sha256 = hex(&hasher.finalize());
    let mut count = Count::default();
    let placeholder = "0".repeat(64);
    write_record(&mut Writer::new(&mut count, Style::Stored), &built, Part::Stored, None, &mut |_| placeholder.clone());
    built.record_bytes = count.0;
    validate_built(&built, source, inputs, reserve)?;
    Ok(built)
}

/// `validateAnalysisRunV03(record, result, loadBasisRefs)` on the record just
/// built: the checks such a record can fail, in its order (the appended
/// diagnostics' codes and identities, the mechanics status, the producer,
/// duplicate row ids); the rest hold by construction.
fn validate_built(built: &Built<'_>, source: &Value, inputs: &BuildInputs<'_>, reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>) -> Result<(), CommandError<String>> {
    let refuse = |text: &str| CommandError::Command(text.to_string());
    let mut seen = std::collections::HashSet::new();
    for exclusion in inputs.rule_pack_exclusions {
        let identity = rule_pack_identity(&exclusion.rule_pack_id, &exclusion.rule_pack_version);
        if ![RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH].contains(&exclusion.code.as_str()) || !seen.insert(identity) {
            return Err(refuse("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH"));
        }
    }
    if !source["status"]["mechanics"].as_str().is_some_and(|mechanics| ["MECHANICS_SOLVED", "MODEL_INCOMPLETE"].contains(&mechanics)) {
        return Err(refuse("ANALYSIS_SOURCE_MECHANICS_STATUS_INVALID"));
    }
    let producer = source.get("producer").filter(|producer| !producer.is_null()).ok_or_else(|| CommandError::Command(shape("result.producer")))?;
    if !strict_eq(Some(built.solver_name), get_opt(Some(producer), "component_name")) || !strict_eq(Some(built.solver_version), get_opt(Some(producer), "component_version")) {
        return Err(refuse("ANALYSIS_SOURCE_PRODUCER_MISMATCH"));
    }
    if duplicate_ids(built.rows, reserve)? {
        return Err(refuse("ANALYSIS_ROW_ACCOUNTING_MISMATCH"));
    }
    Ok(())
}

/// Which part of the record a writer writes.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Part {
    /// The record checksum's payload (`analysisRecordProjection`): the record
    /// without its own checksum.
    Projection,
    /// The whole record.
    Stored,
    /// The header's record: every member but the row-scale ones.
    Header,
}

/// One result ref, written from the borrowed row (its keys in order).
fn write_result_ref<S: Sink>(w: &mut Writer<'_, S>, built: &Built<'_>, index: usize, row: &Value, row_sha256: &str) {
    let (semantic, findings) = interpret(built.table, row).expect("interpreted by the row pass");
    let id = row["id"].as_str().expect("checked with the load basis");
    let member = |key: &str| semantic.and_then(|entry| entry.get(key)).filter(|value| !value.is_null());
    w.raw("{\"category\":");
    match member("category") {
        Some(value) => w.value(value),
        None => w.string("unknown"),
    }
    w.raw(",\"hash_refs\":[");
    w.checksum(CHECKED_PROFILE_V1, |w| w.object_ref("Result", id), "result_row", |w| w.string(row_sha256));
    w.raw("],\"interpretation\":{\"findings\":[");
    for (at, finding) in findings.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(finding);
    }
    w.raw("],\"status\":");
    match member("canonical_disposition") {
        Some(value) => w.value(value),
        None => w.string("unavailable"),
    }
    w.raw("},\"privacy_classification\":\"source_evidence\",\"provenance\":");
    w.raw(PROVENANCE);
    w.raw(",\"result_family\":");
    w.value(member("family").unwrap_or(&Value::Null));
    w.raw(",\"result_ref\":");
    w.object_ref("Result", id);
    w.raw(",\"semantic_contract\":{\"id\":");
    w.string(&built.route.binding_id);
    w.raw(",\"sha256\":");
    w.string(&built.route.binding_sha256);
    w.raw(",\"signature_id\":");
    w.value(member("signature_id").unwrap_or(&Value::Null));
    w.raw("},\"source_annotation\":{\"kind\":");
    w.value(row.get("kind").expect("checked with the record checksum"));
    w.raw(",\"metadata\":");
    w.value(row.get("metadata").unwrap_or(&Value::Null));
    w.raw(",\"unit\":");
    w.value(row.get("unit").expect("checked with the record checksum"));
    w.raw("},\"source_dimension\":");
    w.value(member("source_physical_semantic_dimension").unwrap_or(&Value::Null));
    w.raw(",\"source_row_index\":");
    w.unsigned(index as u64);
    w.raw("}");
}

/// The record (or its projection or header part), its members in key order,
/// which is the same in both styles (every key the record itself writes is
/// ASCII); copied values follow the writer's style. `row_sha256` gives each
/// row's checksum.
fn write_record<S: Sink>(w: &mut Writer<'_, S>, built: &Built<'_>, part: Part, revision: Option<&Revision<'_>>, row_sha256: &mut dyn FnMut(&Value) -> String) {
    let copy = |w: &mut Writer<'_, S>, key: &str| {
        if part == Part::Header {
            return;
        }
        if let Some((_, value)) = built.copies.iter().find(|(name, _)| *name == key) {
            w.raw(",");
            w.key(key);
            w.value(value);
        }
    };
    w.raw("{\"analysis_run\":{\"analysis_status\":[");
    for (at, status) in built.statuses.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(status);
    }
    w.raw("]");
    copy(w, "contract_evidence");
    match revision {
        Some(revision) => {
            w.raw(",\"created_at\":");
            w.string(revision.created_at);
        }
        None => w.raw(",\"created_at\":null"),
    }
    if part != Part::Header {
        w.raw(",\"diagnostics\":[");
        for (at, item) in built.producer_diagnostics.iter().chain(&built.rule_pack_diagnostics).enumerate() {
            if at > 0 {
                w.raw(",");
            }
            if at < built.producer_diagnostics.len() {
                w.raw("{\"source_annotation\":");
                w.value(item);
                w.raw("}");
            } else {
                w.value(item);
            }
        }
        w.raw("]");
    }
    w.raw(",\"hashes\":[");
    if part != Part::Projection {
        let record_sha256 = revision.map_or(built.record_sha256.as_str(), |revision| revision.record_sha256.as_str());
        w.checksum(CHECKED_PROFILE_V1, |w| w.object_ref("AnalysisRun", built.run_id), "analysis_run_record", |w| w.string(record_sha256));
        w.raw(",");
    }
    let envelope = format!("result-envelope:{}", built.run_id);
    w.checksum(CHECKED_PROFILE_V1, |w| w.object_ref("ResultEnvelope", &envelope), "received_result", |w| w.string(&built.received_sha256));
    w.raw("],\"immutability_policy\":{\"hash_invalidates_external_acceptance\":true,\"mutation_policy\":\"changes_create_new_immutable_record_revision\",\"new_mechanics_run_required_for_record_revision\":false,\"record_revision_identity\":\"analysis_run_record_sha256\",\"run_record_is_read_only\":true},\"library_refs\":[");
    write_list(w, revision.map_or(&[][..], |revision| revision.library_refs));
    w.raw("],\"load_basis_refs\":[");
    for (at, reference) in built.load_basis.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(reference);
    }
    w.raw("],\"model_state_ref\":");
    match revision {
        Some(revision) => w.object_ref("ModelState", revision.state_id),
        None => w.object_ref("ModelState", &format!("state:{}:preview", built.model_ref)),
    }
    w.raw(",\"professional_boundary\":{\"human_review_required\":true,\"software_makes_approval_claim\":false,\"software_makes_authentication_claim\":false,\"software_makes_certification_claim\":false,\"software_makes_compliance_claim\":false,\"software_makes_sealing_claim\":false},\"provenance\":");
    w.raw(PROVENANCE);
    w.raw(",\"reproducibility\":{\"determinism_notes\":[");
    // A recorded revision drops the `created_at_unavailable` note (its `created_at` is set).
    let notes = if revision.is_some() { &DETERMINISM_NOTES[1..] } else { &DETERMINISM_NOTES[..] };
    for (at, note) in notes.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(note);
    }
    w.raw("],\"input_manifest_hashes\":[");
    w.checksum("rfc8785_jcs", |w| w.value(built.manifest_ref), "input_manifest", |w| w.value(built.manifest_sha256));
    w.raw("],\"input_manifest_refs\":[");
    w.value(built.manifest_ref);
    w.raw("],\"semantic_contract\":{\"id\":");
    w.string(&built.route.binding_id);
    w.raw(",\"sha256\":");
    w.string(&built.route.binding_sha256);
    w.raw("},\"unresolved_tbd\":[]}");
    if part != Part::Header {
        w.raw(",\"result_refs\":[");
        for (index, row) in built.rows.iter().enumerate() {
            if index > 0 {
                w.raw(",");
            }
            let row_hash = row_sha256(row);
            write_result_ref(w, built, index, row, &row_hash);
        }
        w.raw("]");
    }
    copy(w, "retained_precision");
    w.raw(",\"rule_pack_refs\":[");
    write_list(w, revision.map_or(&[][..], |revision| revision.rule_pack_refs));
    w.raw("],\"run_id\":");
    w.string(built.run_id);
    w.raw(",\"run_kind\":\"mechanics_solve\",\"run_name\":");
    w.string(&format!("{} analysis record", built.run_id));
    w.raw(",\"settings_ref\":");
    w.object_ref("SolverSettings", &built.settings_ref);
    w.raw(",\"solver_version\":{\"build_ref\":{\"object_type\":\"ExternalReference\",\"ref\":");
    w.value(built.solver_build_ref);
    w.raw("},\"solver_name\":");
    w.value(built.solver_name);
    w.raw(",\"solver_version\":");
    w.value(built.solver_version);
    w.raw("}");
    copy(w, "source_block_recovery");
    w.raw(",\"unit_system_ref\":");
    w.object_ref("UnitSystem", &built.unit_system_ref);
    w.raw("},\"deliverable_id\":\"DEL-14-02\",\"objectives\":[\"OBJ-016\"],\"package_id\":\"PKG-14\",\"run_contract_status\":{\"external_validation_boundary\":\"reference_only_not_determined_by_software\",\"record_contract\":\"strict_analysis_run_v0_3\",\"result_binding\":\"received_mechanics_result\"},\"schema_version\":\"");
    w.raw(ANALYSIS_RUN_V03);
    w.raw("\",\"scope_item\":\"SOW-072\"}");
}

/// Writes `items` as a JSON list's members (comma-separated).
fn write_list<S: Sink>(w: &mut Writer<'_, S>, items: &[Value]) {
    for (at, item) in items.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(item);
    }
}

/// Record Run's recorded revision of the record
/// (`analysis_runs::recorded_revision` over the transient record): its
/// `created_at`, its state, the manifest's rule-pack refs and the solved
/// model's library refs, the `created_at_unavailable` note dropped, and its
/// own `analysis_run_record` checksum (the canonical projection of the
/// revision, [`revision_sha256`]). Every other member is the transient
/// record's.
pub(crate) struct Revision<'r> {
    pub(crate) created_at: &'r str,
    pub(crate) state_id: &'r str,
    pub(crate) rule_pack_refs: &'r [Value],
    pub(crate) library_refs: &'r [Value],
    /// Filled by [`Revision::sealed`].
    pub(crate) record_sha256: String,
}

impl Revision<'_> {
    /// The revision with its own checksum: the record's canonical projection
    /// with the revision applied, streamed into SHA-256 (each row's checksum
    /// streamed again).
    pub(crate) fn sealed(mut self, built: &Built<'_>) -> Self {
        let mut hasher = Sha256::new();
        write_record(&mut Writer::new(&mut hasher, Style::Canonical), built, Part::Projection, Some(&self), &mut |row| canonical_sha256(row));
        self.record_sha256 = hex(&hasher.finalize());
        self
    }

    /// The revision's exact stored length, counted (the rows' checksums by length).
    pub(crate) fn stored_length(&self, built: &Built<'_>) -> u64 {
        let mut count = Count::default();
        let placeholder = "0".repeat(64);
        write_record(&mut Writer::new(&mut count, Style::Stored), built, Part::Stored, Some(self), &mut |_| placeholder.clone());
        count.0
    }

    /// The revision's stored bytes, into a buffer of `length` (its
    /// [`Revision::stored_length`]) admitted through `reserve` first.
    pub(crate) fn stored_bytes<R>(&self, built: &Built<'_>, length: u64, reserve: impl FnOnce(u64) -> Result<R, CommandError<String>>) -> Result<(Vec<u8>, R), CommandError<String>> {
        let guard = reserve(ingress::allocation_envelope(length).unwrap_or(u64::MAX))?;
        let mut out = Vec::with_capacity(length as usize);
        write_record(&mut Writer::new(&mut out, Style::Stored), built, Part::Stored, Some(self), &mut |row| canonical_sha256(row));
        assert!(out.len() as u64 == length, "the recorded revision is its counted length");
        Ok((out, guard))
    }
}

/// The record's members other than its row-scale ones (the header's
/// `record`: no `result_refs`, producer `diagnostics` or copied members), as a
/// value, for the binding checks Record Run applies to a record
/// (`analysis_runs::check_native_header`). Its size does not grow with the rows.
pub(crate) fn header_record(built: &Built<'_>) -> Value {
    let mut out = Vec::new();
    write_record(&mut Writer::new(&mut out, Style::Stored), built, Part::Header, None, &mut |_| unreachable!("the header writes no rows"));
    serde_json::from_slice(&out).expect("the header record is JSON")
}

/// The whole transient record's RFC 8785 canonical text (`canonical_json` of
/// its stored value) streamed into `out`: the project envelope payload's
/// `analysis_run` (each row's checksum streamed again).
pub(crate) fn write_canonical_record<S: Sink>(built: &Built<'_>, out: &mut S) {
    write_record(&mut Writer::new(out, Style::Canonical), built, Part::Stored, None, &mut |row| canonical_sha256(row));
}

/// The record's stored bytes: the length checked against `max_bytes` and
/// admitted through `reserve` before the buffer is allocated (a record over
/// the bound is refused by name, nothing allocated), then written at that
/// exact capacity, each row's checksum computed again.
pub(crate) fn record_bytes<R>(
    built: &Built<'_>,
    max_bytes: u64,
    reserve: impl FnOnce(u64) -> Result<R, CommandError<String>>,
) -> Result<(Vec<u8>, R), CommandError<String>> {
    if built.record_bytes > max_bytes {
        return Err(CommandError::Command(format!(
            "{RECORD_TOO_LARGE}: run `{}`'s analysis-run record is {} bytes, over the limit of {max_bytes} bytes; nothing was produced.",
            crate::stored_read::shown(built.run_id),
            built.record_bytes
        )));
    }
    let guard = reserve(ingress::allocation_envelope(built.record_bytes).unwrap_or(u64::MAX))?;
    let mut out = Vec::with_capacity(built.record_bytes as usize);
    write_record(&mut Writer::new(&mut out, Style::Stored), built, Part::Stored, None, &mut |row| canonical_sha256(row));
    assert!(out.len() as u64 == built.record_bytes, "the stored record is its counted length");
    Ok((out, guard))
}

/// The preview header's bytes (module docs), the response lease admitted at
/// their exact length through `reserve` before they are written.
pub(crate) fn header_bytes<R>(built: &Built<'_>, job_id: &str, reserve: impl FnOnce(u64) -> Result<R, CommandError<String>>) -> Result<(Vec<u8>, R), CommandError<String>> {
    let mut count = Count::default();
    write_header(&mut Writer::new(&mut count, Style::Stored), built, job_id);
    let guard = reserve(ingress::allocation_envelope(count.0).unwrap_or(u64::MAX))?;
    let mut out = Vec::with_capacity(count.0 as usize);
    write_header(&mut Writer::new(&mut out, Style::Stored), built, job_id);
    debug_assert!(out.len() as u64 == count.0);
    Ok((out, guard))
}

fn write_header<S: Sink>(w: &mut Writer<'_, S>, built: &Built<'_>, job_id: &str) {
    w.raw("{\"carried_members\":[");
    for (at, (key, _)) in built.copies.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.string(key);
    }
    w.raw("],\"document_kind\":");
    w.string(DOCUMENT_KIND);
    w.raw(",\"job_id\":");
    w.string(job_id);
    w.raw(",\"producer_diagnostic_count\":");
    w.unsigned(built.producer_diagnostics.len() as u64);
    w.raw(",\"record\":");
    write_record(w, built, Part::Header, None, &mut |_| unreachable!("the header writes no rows"));
    w.raw(",\"record_bytes\":");
    w.unsigned(built.record_bytes);
    w.raw(",\"result_ref_count\":");
    w.unsigned(built.rows.len() as u64);
    w.raw(",\"route\":");
    w.string(built.route.name);
    w.raw(",\"rule_check_status\":");
    w.string(&built.rule_check_status);
    w.raw(",\"rule_pack_ref_diagnostics\":[");
    for (at, item) in built.rule_pack_diagnostics.iter().enumerate() {
        if at > 0 {
            w.raw(",");
        }
        w.value(item);
    }
    w.raw("],\"version\":");
    w.unsigned(VERSION);
    w.raw("}");
}

/// `buildAnalysisRunPreview` after its manifest check, on a parsed source:
/// the solve's status, the dispatch, the model's load basis (written to
/// `load_basis`), the route's reader, then the record.
#[allow(clippy::too_many_arguments)]
pub(crate) fn preview_parsed<'a>(
    source: &'a mut Value,
    evidence: &'a ManifestArg,
    manifest_sha256: &'a Value,
    manifest: &'a Value,
    aggregate: Option<&'a Value>,
    exclusions: &'a [Exclusion],
    load_basis: &'a mut Value,
    reserve: &mut dyn FnMut(u64) -> Result<Option<Lease>, CommandError<String>>,
) -> Result<Built<'a>, CommandError<String>> {
    let refuse = CommandError::Command;
    let status = get(Some(&*source), "status", "result").map_err(refuse)?;
    get(status, "rule_check", "result.status").map_err(refuse)?;
    let route = dispatch(source).map_err(refuse)?;
    *load_basis = model_load_basis_refs(manifest).map_err(refuse)?;
    validate_route(source, &route).map_err(refuse)?;
    let source: &'a Value = source;
    let load_basis: &'a Value = load_basis;
    let inputs = BuildInputs {
        manifest: ManifestEvidence { manifest_ref: &evidence.manifest_ref, manifest_sha256, manifest },
        rule_check_status: applied_rule_check_status(aggregate, get_opt(source.get("status"), "rule_check")),
        load_basis_refs: Some(load_basis),
        rule_pack_exclusions: exclusions,
    };
    build(source, route, &inputs, reserve)
}

