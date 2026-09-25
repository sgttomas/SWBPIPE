//! Private producer finalization for the selected bounded source method.
//! Digests bind custody; they do not replace the retained arithmetic proof.
use super::source_recovery::{self, SelectedSourceRecovery};
use super::*;
use open_pipe_stress_canonical_json::canonical_json_checked_v1_text;
use open_pipe_stress_frame_kernel::structural::{
    exact_boundary as exact, StructuralError, StructuralReport,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
mod rows;
mod source;

pub(super) const SEMANTIC_ID: &str = "openpipestress.result_semantics/0.3.0/source-blocks-1";
const CRITERION: f64 = 1e-9;
const MAX_ITEMS: usize = 16_384;
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ReceiptError(pub String, pub Option<exact::WorkReport>);
fn bad(message: impl Into<String>) -> ReceiptError {
    ReceiptError(message.into(), None)
}
fn checked(value: &Value) -> Result<String, ReceiptError> {
    let text = serde_json::to_string(value).map_err(|e| bad(e.to_string()))?;
    canonical_json_checked_v1_text(&text).map_err(bad)
}
fn hash(domain: &'static str, payload: &Value) -> Result<String, ReceiptError> {
    let text = checked(&json!({"domain":domain,"payload":payload}))?;
    Ok(format!("{:x}", Sha256::digest(text.as_bytes())))
}
fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}
fn same(a: f64, b: f64) -> bool {
    a.to_bits() == b.to_bits()
}
/// Fixed arithmetic for the registered scaled norm recipes. The finalizer
/// separately refuses nonfinite inputs and nonzero subnormal outputs.
pub(super) fn scaled_norm(values: [f64; 3]) -> f64 {
    let m = values.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
    if m == 0.0 { return 0.0; }
    let [a, b, c] = values.map(|v| v / m);
    m * ((a * a + b * b) + c * c).sqrt()
}
fn serialized<T: Serialize>(v: &T) -> Result<Value, ReceiptError> {
    serde_json::to_value(v).map_err(|e| bad(e.to_string()))
}

/// Captures the actual raw request before parsing. No parsed/hash constructor.
#[derive(Debug)]
pub(super) struct CapturedInvocation {
    raw: Value,
    mode: PreviewSolverMode,
    digest: String,
    encoded_len: usize,
}
impl CapturedInvocation {
    pub(super) fn parse(
        raw: Value,
        mode: PreviewSolverMode,
    ) -> Result<(LinearStaticPreviewRequest, Self), ReceiptError> {
        let encoded_len = checked(&raw)?.len();
        let digest = hash(
            "source_blocks_invocation_v1",
            &json!({"request":&raw,"solver_mode":mode.as_str()}),
        )?;
        let parsed =
            serde_json::from_value(raw.clone()).map_err(|e| bad(format!("request: {e}")))?;
        Ok((
            parsed,
            Self {
                raw,
                mode,
                digest,
                encoded_len,
            },
        ))
    }
    pub(super) fn mode(&self) -> PreviewSolverMode {
        self.mode
    }
    fn requested(&self) -> Result<LinearStaticPreviewRequest, ReceiptError> {
        serde_json::from_value(self.raw.clone()).map_err(|e| bad(e.to_string()))
    }
    /// Reconstruct only the actual product normalization/material selection. Raw
    /// custody/hashing remains the preparse Value, never these typed objects.
    fn check_input(
        &self,
        input: &source_recovery::Input<'_>,
        selected: &mut SelectedSourceRecovery,
    ) -> Result<Option<String>, ReceiptError> {
        let request = self.requested()?;
        let mut model = request.model;
        let mut materials = if request.materials.is_empty() {
            model.materials.clone()
        } else {
            request.materials
        };
        let mut diagnostics = Vec::new();
        resolve_shared_sections(&mut model, &mut diagnostics);
        normalize_model_units(&mut model, &mut materials, &mut diagnostics);
        if has_blocking(&diagnostics) || model.project.id != input.model.project.id {
            return Err(bad("invocation normalized model mismatch"));
        }
        let case = model
            .load_cases
            .iter()
            .find(|c| c.id == input.load_case.id)
            .ok_or_else(|| bad("invocation case missing"))?;

        let mut material_basis_record = None;
        if modulus_basis_key(case, &mut diagnostics).is_some() {
            let (resolved, record) =
                materials_for_modulus_basis(&model, &materials, case, &mut diagnostics)
                    .ok_or_else(|| bad("invocation material selection"))?;
            materials = resolved;
            material_basis_record = Some(record);
        }
        let built = build_model(&model, &materials, &mut diagnostics)
            .ok_or_else(|| bad("invocation build unavailable"))?;
        if has_blocking(&diagnostics) {
            return Err(bad("invocation actual built source unavailable"));
        }
        let boundary = prepare_boundary(built.nodes.len(), &built.supports);
        let mut stiffness = assemble_global_stiffness_with_user_elements(
            built.nodes.len(),
            &built.frame_elements,
            &built.user_stiffness_elements,
        )
        .map_err(|e| bad(e.to_string()))?;
        add_curved_bend_stiffness_contributions(&mut stiffness, &built.curved_bend_elements);
        for spring in &boundary.springs {
            let dof = spring.node_dof.global_index();
            stiffness[dof][dof] += spring.stiffness.value;
        }
        let loads = build_load_case_primitive_loads(&model, case, &mut diagnostics);
        let application = prepare_loads(built.nodes.len(), built.pipes.len(), &loads);
        let force = application.global_load_vector(built.nodes.len());
        let prescribed: Vec<_> = boundary
            .restrained_dofs
            .iter()
            .map(|&d| {
                (
                    d,
                    boundary
                        .imposed_displacements
                        .iter()
                        .find(|p| p.node_dof.global_index() == d)
                        .map(|p| p.displacement.value)
                        .unwrap_or(0.0),
                )
            })
            .collect();
        let free: Vec<_> = (0..force.len())
            .filter(|d| !boundary.restrained_dofs.contains(d))
            .collect();
        // The selected minimum rejects every non-nodal producer in prepare_sources;
        // no unsupported source is omitted into an apparently empty witness.
        if has_blocking(&diagnostics) {
            return Err(bad("captured load preparation"));
        }
        selected
            .replay_against(source_recovery::Input {
                model: &model,
                built: &built,
                stiffness: &stiffness,
                force: &force,
                free: &free,
                prescribed: &prescribed,
                spring_entries: &boundary.springs,
                load_case: case,
                load_application: &application,
                thermal_loads: &[],
                pressure_thrust_loads: &[],
            })
            .map_err(|e| bad(format!("captured source replay: {e:?}")))?;
        Ok(material_basis_record)
    }
}

fn finalization_size(
    capture: &CapturedInvocation,
    selected: &SelectedSourceRecovery,
    rows: &[ResultItem],
) -> usize {
    let response = selected.retained().response();
    let source = response.source_system();
    let mut n = response
        .source_identity()
        .len()
        .saturating_mul(12)
        .saturating_add(
            source
                .stiffness
                .len()
                .saturating_mul(source.stiffness.len())
                .saturating_mul(32),
        );
    // Raw Value has already passed checked parsing; count encoded custody bytes,
    // then debit the existing attempt before normalization/rebuild/hash copies.
    n = n.saturating_add(capture.encoded_len.saturating_mul(12));
    for d in selected.retained().descriptors() {
        n = n
            .saturating_add(d.key.case_id.len().saturating_mul(12))
            .saturating_add(
                d.offset
                    .iter()
                    .map(|p| p.len().saturating_mul(48))
                    .sum::<usize>(),
            );
        for t in &d.terms {
            n = n.saturating_add(48).saturating_add(
                t.products
                    .iter()
                    .map(|p| p.len().saturating_mul(48))
                    .sum::<usize>(),
            );
        }
    }
    for r in rows {
        n = n
            .saturating_add(r.id.len())
            .saturating_add(r.kind.len())
            .saturating_add(r.entity_ref.len())
            .saturating_add(r.unit.len());
        if let Some(m) = &r.metadata {
            n = n
                .saturating_add(m.component.len())
                .saturating_add(m.coordinate_system.len())
                .saturating_add(m.location.len())
                .saturating_add(m.basis.len())
                .saturating_add(m.sign_convention.len());
        }
    }
    n.saturating_mul(2)
}

#[derive(Debug, Clone)]
pub(super) struct OrdinaryAttempt {
    mode: PreviewSolverMode,
    outcome: &'static str,
    report: Option<String>,
    failure: Option<(&'static str, String)>,
    expected_code: &'static str,
}
impl OrdinaryAttempt {
    pub(super) fn passed(
        mode: PreviewSolverMode,
        report: &StructuralReport,
        diagnostic_ref: String,
    ) -> Self {
        let sensitive =
            report.quality == open_pipe_stress_frame_kernel::structural::SolveQuality::Sensitive;
        Self {
            mode,
            outcome: if sensitive {
                "sensitive"
            } else {
                "checks_passed"
            },
            report: Some(diagnostic_ref),
            failure: None,
            expected_code: if sensitive {
                "NUMERICAL_INTEGRITY_SENSITIVE"
            } else {
                "NUMERICAL_INTEGRITY_CHECKS_PASSED"
            },
        }
    }
    pub(super) fn rejected(
        mode: PreviewSolverMode,
        error: &StructuralError,
        diagnostic_ref: String,
    ) -> Self {
        let (stage, code) = match error {
            StructuralError::Mechanism { .. } => {
                ("geometry", "NUMERICAL_INTEGRITY_PHYSICAL_MECHANISM")
            }
            StructuralError::NegativeEnergy { .. } => {
                ("factorization", "NUMERICAL_INTEGRITY_NEGATIVE_ENERGY")
            }
            StructuralError::Asymmetric { .. } | StructuralError::InvalidInput(_) => {
                ("input", "NUMERICAL_INTEGRITY_FAILED")
            }
            StructuralError::Range(_) => ("range", "NUMERICAL_INTEGRITY_UNRESOLVED"),
            StructuralError::NumericallyUnresolved { reason, .. }
                if reason.contains("assembly") || reason.contains("contribution") =>
            {
                ("assembly", "NUMERICAL_INTEGRITY_ASSEMBLY_UNRESOLVED")
            }
            StructuralError::NumericallyUnresolved { reason, .. }
                if reason.contains("condition") =>
            {
                ("condition", "NUMERICAL_INTEGRITY_UNRESOLVED")
            }
            StructuralError::NumericallyUnresolved { reason, .. }
                if reason.contains("residual") =>
            {
                ("residual", "NUMERICAL_INTEGRITY_UNRESOLVED")
            }
            _ => ("factorization", "NUMERICAL_INTEGRITY_UNRESOLVED"),
        };
        Self {
            mode,
            outcome: "rejected",
            report: None,
            failure: Some((stage, diagnostic_ref)),
            expected_code: code,
        }
    }
    pub(super) fn not_attempted(mode: PreviewSolverMode) -> Self {
        Self {
            mode,
            outcome: "not_attempted",
            report: None,
            failure: None,
            expected_code: "",
        }
    }
    fn wire(&self, index: usize, case: &str, envelope: &Value) -> Result<Value, ReceiptError> {
        let q = envelope["numerical_quality"]["cases"]
            .as_array()
            .and_then(|a| a.get(index))
            .ok_or_else(|| bad("ordinary quality case missing"))?;
        if q["basis_ref"] != json!({"ref_type":"load_case","ref_id":case}) {
            return Err(bad("ordinary quality case binding"));
        }
        let quality = q["solve_quality"].as_str().unwrap_or("");
        if !match self.outcome {
            "not_attempted" => quality == "not_assessed",
            "checks_passed" => quality == "checks_passed",
            "sensitive" => quality == "sensitive",
            "rejected" => matches!(quality, "failed" | "unresolved"),
            _ => false,
        } {
            return Err(bad("ordinary outcome changed"));
        }
        if let Some(reference) = self
            .report
            .as_ref()
            .or_else(|| self.failure.as_ref().map(|p| &p.1))
        {
            let matching: Vec<_> = envelope["diagnostics"]
                .as_array()
                .ok_or_else(|| bad("diagnostics"))?
                .iter()
                .filter(|d| d["id"] == *reference)
                .collect();
            if matching.len() != 1 || matching[0]["code"] != self.expected_code {
                return Err(bad("ordinary diagnostic binding"));
            }
        }
        Ok(
            json!({"requested_mode":self.mode.as_str(),"outcome":self.outcome,"structural_report_diagnostic_ref":self.report,"failure":self.failure.as_ref().map(|(s,d)|json!({"stage":s,"diagnostic_ref":d})),"quality_case_index":index}),
        )
    }
}

#[derive(Debug, Clone)]
pub(super) struct FunctionalRowBinding {
    pub functional_index: usize,
    pub result_id: String,
}
#[derive(Debug, Clone, Serialize)]
struct Projection {
    projection_id: String,
    functional_id: String,
    result_id: String,
    quantity: &'static str,
    value: f64,
    value_bits: String,
    unit: &'static str,
    interval: [f64; 2],
    absolute_error_bound: f64,
    relative_error_bound: f64,
    relative_limit: f64,
    basis: &'static str,
}
#[derive(Debug, Clone, Serialize)]
struct RowTreatment {
    result_id: String,
    treatment: &'static str,
    projection_id: Option<String>,
    recipe_id: Option<&'static str>,
    input_result_ids: Vec<String>,
}
#[derive(Debug)]
pub(super) struct FinalizedSourceBlockCase {
    invocation: String,
    case_id: String,
    ordinary: OrdinaryAttempt,
    source: Option<Value>,
    projections: Vec<Projection>,
    rows: Vec<RowTreatment>,
    supports: Value,
    failure: Option<Value>,
    work: Value,
    actual_rows: Value,
    qualified: bool,
    exact: bool,
    // Own the live-created companion until publication finalization completes.
    _selected: Option<SelectedSourceRecovery>,
}
impl FinalizedSourceBlockCase {
    pub(super) fn charged_work(&self) -> usize {
        self._selected.as_ref().map(|selected| selected.summary().work.charged)
            .unwrap_or_else(|| self.work["charged"].as_u64().unwrap_or(0) as usize)
    }

    pub(super) fn is_qualified(&self) -> bool {
        self.qualified
    }

    pub(super) fn exact(
        capture: &CapturedInvocation,
        input: source_recovery::Input<'_>,
        mut selected: SelectedSourceRecovery,
        ordinary: OrdinaryAttempt,
        rows: &[ResultItem],
        bindings: &[FunctionalRowBinding],
    ) -> Result<Self, ReceiptError> {
        if ordinary.mode != capture.mode {
            return Err(bad("requested mode mismatch"));
        }
        let checked_case = (|| {
            selected
                .charge_finalization(
                    1024usize
                        .saturating_add(rows.len().saturating_mul(32))
                        .saturating_add(selected.retained().descriptors().len().saturating_mul(32)),
                )
                .map_err(|e| bad(format!("finalization reservation: {e:?}")))?;
            let reservation = finalization_size(capture, &selected, rows);
            selected
                .charge_finalization(reservation)
                .map_err(|e| bad(format!("finalization reservation: {e:?}")))?;
            let material_basis_record = capture.check_input(&input, &mut selected)?;
            selected
                .check_binding_against(source_recovery::Input {
                    model: input.model,
                    built: input.built,
                    stiffness: input.stiffness,
                    force: input.force,
                    free: input.free,
                    prescribed: input.prescribed,
                    spring_entries: input.spring_entries,
                    load_case: input.load_case,
                    load_application: input.load_application,
                    thermal_loads: input.thermal_loads,
                    pressure_thrust_loads: input.pressure_thrust_loads,
                })
                .map_err(|e| bad(format!("current source binding: {e:?}")))?;
            let case_id = input.load_case.id.clone();
            let ledger = rows::bind(&input, &selected, rows, bindings)?;
            let source = source::commitment(
                &input,
                &selected,
                bindings,
                &capture.digest,
                material_basis_record.as_deref(),
                &ledger.rows,
                rows,
            )?;
            let qualified = ledger.rows.iter().all(|r| {
                r.treatment != "inspection_only"
                    || rows
                        .iter()
                        .find(|v| v.id == r.result_id)
                        .is_some_and(rows::observation)
            });
            Ok::<_, ReceiptError>((case_id, ledger, source, qualified, serialized(&rows)?))
        })();
        let (case_id, ledger, source, qualified, actual_rows) = checked_case.map_err(|mut e| {
            e.1 = Some(selected.summary().work);
            e
        })?;
        let work = work_wire(selected.summary().work)?;
        Ok(Self {
            invocation: capture.digest.clone(),
            case_id,
            ordinary,
            source: Some(source),
            projections: ledger.projections,
            rows: ledger.rows,
            supports: ledger.supports,
            failure: None,
            work,
            actual_rows,
            qualified,
            exact: true,
            _selected: Some(selected),
        })
    }
    pub(super) fn ordinary(
        capture: &CapturedInvocation,
        case_id: &str,
        ordinary: OrdinaryAttempt,
        rows: &[ResultItem],
    ) -> Result<Self, ReceiptError> {
        if ordinary.mode != capture.mode || ordinary.outcome != "checks_passed" {
            return Err(bad("ordinary selection not p1 checks-passed"));
        }
        validate_case_rows(case_id, rows)?;
        Ok(Self {
            invocation: capture.digest.clone(),
            case_id: case_id.into(),
            ordinary,
            source: None,
            projections: vec![],
            rows: rows
                .iter()
                .map(|r| RowTreatment {
                    result_id: r.id.clone(),
                    treatment: "ordinary_checked",
                    projection_id: None,
                    recipe_id: None,
                    input_result_ids: vec![],
                })
                .collect(),
            supports: json!([]),
            failure: None,
            work: work_wire(exact::WorkReport {
                charged: 0,
                rejected: 0,
                limit: 0,
            })?,
            actual_rows: serialized(&rows)?,
            qualified: true,
            exact: false,
            _selected: None,
        })
    }
    pub(super) fn failed(
        capture: &CapturedInvocation,
        case_id: &str,
        ordinary: OrdinaryAttempt,
        failure: &source_recovery::RecoveryFailure,
        diagnostic_ref: &str,
        rows: &[ResultItem],
    ) -> Result<Self, ReceiptError> {
        validate_case_rows(case_id, rows)?;
        let (stage, code, block) = source::failure_fields(failure);
        Ok(Self {
            invocation: capture.digest.clone(),
            case_id: case_id.into(),
            ordinary,
            source: None,
            projections: vec![],
            rows: rows
                .iter()
                .map(|r| RowTreatment {
                    result_id: r.id.clone(),
                    treatment: "inspection_only",
                    projection_id: None,
                    recipe_id: None,
                    input_result_ids: vec![],
                })
                .collect(),
            supports: json!([]),
            failure: Some(
                json!({"stage":stage,"code":code,"diagnostic_ref":diagnostic_ref,"block_order":block}),
            ),
            work: work_wire(failure.work)?,
            actual_rows: serialized(&rows)?,
            qualified: false,
            exact: false,
            _selected: None,
        })
    }
}
fn validate_case_rows(case: &str, rows: &[ResultItem]) -> Result<(), ReceiptError> {
    let mut ids = BTreeSet::new();
    if rows.len() > MAX_ITEMS {
        return Err(bad("row budget"));
    }
    for r in rows {
        if r.id.is_empty()
            || !ids.insert(&r.id)
            || !r.value.is_finite()
            || r.basis_ref.as_ref()
                != Some(&ResultBasisRef {
                    ref_type: "load_case".into(),
                    ref_id: case.into(),
                })
        {
            return Err(bad("invalid/duplicate/misowned case row"));
        }
    }
    Ok(())
}
fn work_wire(work: exact::WorkReport) -> Result<Value, ReceiptError> {
    const MAX: u64 = 9_007_199_254_740_991;
    if work.charged > work.limit || (work.limit as u64) > MAX {
        return Err(bad("invalid work accounting"));
    }
    Ok(
        json!({"limit":work.limit,"charged":work.charged,"reserved_unobserved_failure":0,"rejected_reservation":if work.rejected == usize::MAX || (work.rejected as u64)>MAX{json!({"kind":"overflow","amount":null})}else{json!({"kind":"finite","amount":work.rejected})}}),
    )
}

#[derive(Debug, Serialize)]
pub(super) struct FinalizedSourceBlockReceipt {
    body: Value,
    receipt_sha256: String,
}
impl FinalizedSourceBlockReceipt {
    pub(super) fn finalize(
        capture: &CapturedInvocation,
        envelope: &MechanicsEnvelope,
        cases: Vec<FinalizedSourceBlockCase>,
        invocation_budget: &mut SourceRecoveryBudget,
    ) -> Result<Self, ReceiptError> {
        if envelope.results.len() > MAX_ITEMS || envelope.diagnostics.len() > MAX_ITEMS {
            return Err(bad("publication inventory budget"));
        }
        let base_reservation = 1024usize
            .saturating_add(envelope.results.len().saturating_mul(32))
            .saturating_add(envelope.diagnostics.len().saturating_mul(32));
        invocation_budget.reserve_publication(base_reservation)
            .map_err(|work| ReceiptError("invocation publication reservation".into(), Some(work)))?;
        let size = envelope.diagnostics.iter().fold(0usize, |n, d| {
            n.saturating_add(d.id.len()).saturating_add(d.code.len()).saturating_add(d.message.len())
                .saturating_add(d.affected_refs.iter().map(String::len).sum::<usize>())
        });
        invocation_budget.reserve_publication(size.saturating_mul(12))
            .map_err(|work| ReceiptError("invocation publication reservation".into(), Some(work)))?;
        let publication = serialized(envelope)?;
        if publication.get("source_block_recovery").is_some()
            || envelope.producer.semantic_contract_id != SEMANTIC_ID
        {
            return Err(bad(
                "publication must have source identity and omit receipt",
            ));
        }
        let request = capture.requested()?;
        if request.model.project.id != envelope.model_ref
            || cases.is_empty()
            || cases.len() != request.model.load_cases.len()
        {
            return Err(bad("invocation case coverage"));
        }
        if serialized(&assessed_numerical_quality(
            &request.model,
            &envelope.diagnostics,
        ))? != serialized(&envelope.numerical_quality)?
        {
            return Err(bad(
                "ordinary numerical summary no longer matches actual diagnostics",
            ));
        }
        let mut ids = BTreeSet::new();
        for r in &envelope.results {
            if !ids.insert(r.id.as_str()) {
                return Err(bad("duplicate result id"));
            }
        }
        for d in &envelope.diagnostics {
            if !ids.insert(d.id.as_str()) {
                return Err(bad("result/diagnostic collision"));
            }
        }
        let mut wire = Vec::new();
        let mut accounted = BTreeSet::new();
        let mut qualified = 0;
        for (index, (case, requested)) in cases.iter().zip(&request.model.load_cases).enumerate() {
            if case.case_id != requested.id
                || case.invocation != capture.digest
                || case.ordinary.mode != capture.mode
            {
                return Err(bad("invocation/case order mismatch"));
            }
            let actual: Vec<_> = envelope
                .results
                .iter()
                .filter(|r| {
                    r.basis_ref
                        .as_ref()
                        .is_some_and(|b| b.ref_type == "load_case" && b.ref_id == case.case_id)
                })
                .cloned()
                .collect();
            if serialized(&actual)? != case.actual_rows {
                return Err(bad("final row bytes changed"));
            }
            for r in &case.rows {
                if !accounted.insert(r.result_id.clone()) {
                    return Err(bad("row accounted twice"));
                }
            }
            let ordinary = case.ordinary.wire(index, &case.case_id, &publication)?;
            let q = &publication["numerical_quality"]["cases"][index];
            if case.qualified
                && !case.exact
                && (q["structural_status"] != "passive_model_basis"
                    || q["model_matrix_fidelity"] != "represented_equations_retained"
                    || !matches!(
                        q["accuracy_evidence"].as_str(),
                        Some("not_claimed" | "reference_verified")
                    ))
            {
                return Err(bad("ordinary p1 qualification"));
            }
            let eligible = case.qualified && request.model.combinations.is_empty();
            if eligible {
                qualified += 1;
            }
            let mut failure = case.failure.clone();
            if !eligible && failure.is_none() {
                let diagnostic = envelope
                    .diagnostics
                    .iter()
                    .find(|d| {
                        d.code == "SOURCE_RECOVERY_DERIVED_UNQUALIFIED"
                            && d.affected_refs.contains(&case.case_id)
                    })
                    .ok_or_else(|| bad("unqualified derived case needs actual diagnostic"))?;
                failure = Some(
                    json!({"stage":"derived_rows","code":"unsupported_derived_quantity","diagnostic_ref":diagnostic.id,"block_order":null}),
                );
            }
            if let Some(f) = &failure {
                if !envelope
                    .diagnostics
                    .iter()
                    .any(|d| Some(d.id.as_str()) == f["diagnostic_ref"].as_str())
                {
                    return Err(bad("source failure diagnostic missing"));
                }
            }
            wire.push(json!({"basis_ref":{"ref_type":"load_case","ref_id":case.case_id},"outcome":if eligible{"qualified"}else if failure.as_ref().is_some_and(|f|matches!(f["code"].as_str(),Some("unsupported_family"|"unsupported_block"|"unsupported_source_closure"|"unsupported_derived_quantity"|"support_attribution_ambiguous"))){"unsupported"}else{"failed"},"requested_mode":capture.mode.as_str(),"selected_method":if eligible{Some(if case.exact{"retained_source_blocks_exact_v1"}else if capture.mode==PreviewSolverMode::DenseScrutiny{"ordinary_dense_structural_v1"}else{"ordinary_sparse_structural_v1"})}else{None},"ordinary_attempt":ordinary,"source":case.source,"projections":case.projections,"rows":case.rows,"supports":case.supports,"failure":failure,"work":case.work}));
        }
        let mut observations = Vec::new();
        for r in &envelope.results {
            if !accounted.contains(&r.id) {
                if rows::observation(r) {
                    observations.push(r.id.clone());
                } else {
                    return Err(bad("unaccounted physical result"));
                }
            }
        }
        rows::validate_summary(envelope)?;
        let case_reserved = cases.iter().try_fold(0usize, |sum, case| {
            let charged = case.work["charged"].as_u64().ok_or_else(|| bad("case work count"))? as usize;
            let reserved = case.work["reserved_unobserved_failure"].as_u64().ok_or_else(|| bad("case unobserved reservation"))? as usize;
            sum.checked_add(charged).and_then(|v| v.checked_add(reserved)).ok_or_else(|| bad("invocation work sum overflow"))
        })?;
        if case_reserved.checked_add(invocation_budget.publication_charged) != Some(invocation_budget.charged)
            || invocation_budget.charged > invocation_budget.invocation_limit
            || invocation_budget.invocation_limit > SOURCE_BLOCKS_INVOCATION_WORK_LIMIT
        { return Err(bad("invocation work ledger mismatch")); }
        let body = json!({"receipt_version":"1.0.0","policy":"SOURCE-BLOCKS-1","status":if qualified==cases.len(){"qualified"}else if qualified>0{"partial"}else{"unavailable"},"invocation":{"algorithm":"sha256","canonicalization":"openpipestress_jcs_ijson_v1","payload_scope":"source_blocks_invocation_v1","value":capture.digest},"publication_sha256":hash("source_blocks_publication_v1",&publication)?,"cases":wire,"envelope_observation_result_ids":observations,"invocation_work":{"limit":invocation_budget.invocation_limit,"charged":invocation_budget.charged,"publication_charged":invocation_budget.publication_charged}});
        let receipt_sha256 = hash("source_blocks_receipt_v1", &body)?;
        Ok(Self {
            body,
            receipt_sha256,
        })
    }
    pub(super) fn into_wire(self) -> Value {
        json!({"body":self.body,"receipt_sha256":self.receipt_sha256})
    }
}

#[cfg(test)]
mod tests;
