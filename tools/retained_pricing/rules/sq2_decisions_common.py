"""SQ2 G5, rules common to both routes. B2-A's D1.4 admits on branch L z <= 2 model
combinations of any basis with C_eq = c + z <= 3, each with <= 3 terms (repeats counted) and <= 3 range operands).
The D1.4 combination zero rules of I65/I104 are removed and every loop they bound on the reached graph gets its real
bound. Count variables: z (model combinations, <= 2); the literal 3 is D1.9's COMBINATION_TERMS / RANGE_OPERANDS cap
(retained_memory.rs caps). Each entry: exact loop header (as loopscan prints it) -> (bound, why).
"""
ZERO_REMOVED = [
    (r"combination|operand_id", "D1.4: model.combinations is empty"),
    (r'item\.kind == \" \"\)\.filter_map|append_combination|^iter:sources\.iter\(\)\.map$', "combination records (D1.4: no combinations)"),
    (r"^for\ gate\ in\ gates$", "G4 (T17 precommit reader at b1f80234dc): combination gates exist only for combinations (D1.4)"),
    (r'^for\ gate\ in\ list\(\&evidence\["\ "\]\)$', "I104: combination_gates exist only for combinations (none in D1.4/B1)"),
]

# ---- SQ2 phase 2: the RS reader's function keys are resolved by name at the snapshot (env SQ2_SNAP, the
# projects/chirality-piping dir), so a later RS head needs no hand re-keying; each name must be defined exactly once.
import os as _os, re as _re
class Resolved(str):
    """O-10: a key resolved at the snapshot itself (by name or by text): sq2_rules.py's decision key map leaves it alone."""
_SNAP = _os.environ["SQ2_SNAP"]     # the snapshot's projects/chirality-piping (price.py sets it)
def rs(name, f="core/reporting/result_export/src/retained_precision.rs"):
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    hits = [i + 1 for i, l in enumerate(lines) if _re.match(r"\s*(pub(\([a-z]+\))?\s+)?fn\s+" + _re.escape(name) + r"\b", l)]
    if len(hits) > 1:   # a test-only forwarding wrapper of the same name (indented): the module-level fn
        hits = [h for h in hits if not lines[h - 1][:1].isspace()]
    if len(hits) != 1: raise SystemExit(f"rs(): {name} defined {len(hits)} times in {f}")
    return Resolved(f"{f.split('/src/')[-1]}:{hits[0]}:{name}")
def rs_any(*names, f="core/reporting/result_export/src/retained_precision.rs"):
    """rs() of the first of `names` defined at the snapshot (a function renamed between heads)."""
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read()
    for n in names:
        if _re.search(r"\bfn\s+" + _re.escape(n) + r"\b", lines):
            return rs(n, f)
    raise SystemExit(f"rs_any(): none of {names} defined in {f}")
# PB7 / #1251 (E13): the legacy combination engine is deleted on main after #1251. EDGE_ZERO[0] zeroed its only call
# edge; with the function gone the edge has no callee, so this key may stay unmapped (the rule then names nothing).
# Each entry is (the kind the line map must report, why); rule_ops.carry_decision_keys never allows "replaced".
KEYMAP_ALLOW = {"lib.rs:13102:append_combination_results": ("deleted",
    "E13 (#1251): fn append_combination_results is deleted (lib.rs, pre-B7 cleanup); EDGE_ZERO[0]'s callee no longer exists"),
    # B7b-K1: the function-as-value zero edge's callee is deleted with the v2-era readers; the edge names nothing.
    "load_reference_source.rs:46:code": ("deleted",
    "B7b (A4/A5, 54018fe688): load_reference_source.rs is reduced to the current pre-pass and its private fn code is "
    "deleted; preview_case_evidence's `.map(code)` names load_reference.rs's code (EDGE_ZERO[3]) and this callee no longer exists")}
Z = "D1.4 (B2-A): the model's combinations, z <= 2 (D1.9 row Combinations)"
T = "D1.4 (B2-A): one combination's terms, repeats counted, <= 3 (D1.9 row CombinationTerms)"
O = "D1.4 (B2-A): one combination's operand load cases (mechanics terms, a subtraction's two ids, or a range envelope's ids), <= 3 (D1.9 rows CombinationTerms and RangeOperands)"
NEW = {
    # load_case_algebra (lib.rs) and PP's CombinationExpression
    "for operand_id in ordered_operand_ids": ("3", "load_case_algebra evaluate_range_envelope: the range envelope's sorted operand ids. " + O),
    "iter:operand_ids.iter().map(String::as_str)": ("3", "load_case_algebra evaluate_range_envelope: the operand ids. " + O),
    "for operand_id in operand_ids": ("3", "PP append_combination_modulus_basis_records (lib.rs) and validation.rs validate_range_envelope_combination: one combination's operand ids. " + O),
    "for case in expression.operand_ids()": ("3", "PP lib.rs append_combined_vector_magnitude: CombinationExpression::operand_ids. " + O),
    "for operand_id in expression.operand_ids()": ("3", "PP lib.rs append_combination_results: CombinationExpression::operand_ids. " + O),
    "iter:> combination .minuend_id .iter() .chain(combination.subtrahend_id.iter()).map": ("2", "PP lib.rs append_combination_modulus_basis_records: a subtraction's minuend and subtrahend ids (two Options)"),
    "iter:> combination .operand_ids .iter() .flatten().map": ("3", "PP lib.rs append_combination_modulus_basis_records: a range envelope's operand ids. " + O),
    # the model's combinations
    "for combination in &model.combinations": ("z", "PP (lib.rs append_combination_modulus_basis_records, append_combination_results; preview_physics.rs append_combination_results; retained_product.rs capture_combinations; validation.rs validate_provenance, validate_combinations; pressure_runtime.rs validate_profile): the model's combinations. " + Z),
    "iter:combinations.iter().map": ("z", "PP lib.rs w1_case_ids: the request's combinations. " + Z),
    "iter:combinations.all": ("z", "PP lib.rs w1_combinations_admitted. " + Z),
    "iter:model.combinations.iter().map": ("z", "PP lib.rs w1_model_combinations_admitted; preview_physics.rs render's headline-scope refs. " + Z),
    "iter:model .combinations .iter().map": ("z", "PP validation.rs validate_ids: the combinations' ids. " + Z),
    'iter:" ":input.model.combinations.iter().map': ("z", "PP source_receipt/source.rs commitment: the combinations' ids. " + Z),
    "for combination in &m.combinations": ("z", "PP retained_memory.rs census walk over the typed request's combinations (admitted inputs: D1.9). " + Z),
    "for id in combination.operand_ids.iter().flatten()": ("3", "PP retained_memory.rs census walk: one combination's operand ids. " + O),
    "for term in &combination.terms": ("3", "PP retained_memory.rs census walk, retained_product.rs capture_combinations, validation.rs validate_mechanics_combination: one combination's terms. " + T),
    # preview_physics.rs
    "iter:combination.terms.iter().map": ("3", "PP preview_physics.rs gate_reason: the factor sums over one combination's terms. " + T),
    "iter:combination .terms .iter().filter_map": ("3", "PP preview_physics.rs gate_reason: one combination's terms. " + T),
    "iter:combination .terms .iter() .filter_map(|term| model.load_cases.iter().find(|c| c.id == term.load_case)).map": ("3", "PP preview_physics.rs gate_reason: one combination's terms (the find is a closure over the c load cases). " + T),
    "iter:sources.iter().map": ("3", "PP preview_physics.rs append_combination_results: `sources` holds one row per operand case (operands.iter().filter_map). " + O),
    # retained_product.rs (B2-P)
    "for (gate, combination) in gates.iter().zip(&self.combinations)": ("z", "PP retained_product.rs preview_case_evidence (T-9'): one gate entry per captured combination (lengths checked equal first). " + Z),
    "for (index, combination) in self.capture.combinations.iter().enumerate()": ("z", "PP retained_product.rs combination_capacity, dispositions: the captured combinations. " + Z),
    "for &(case, _) in &combination.terms": ("3", "PP retained_product.rs combination_capacity: one captured combination's terms. " + T),
    "for combination in &self.combinations": ("z", "PP retained_product.rs staged_envelope: one CombinationAttempt per model combination. " + Z),
    "for _ in 0..self.combinations.len()": ("z", "PP retained_product.rs bind_combination_rows: one row run per captured combination. " + Z),
    "for (k,combination) in self.combinations.iter().enumerate()": ("z", "PP retained_product.rs bind_combination_rows: the owner search over the captured combinations. " + Z),
    "for k in 0..self.combinations.len()": ("z", "PP retained_product.rs combine_on (T-10b (i) and (iii)): the CombinationAttempts. " + Z),
    "for combination in &mut self.combinations": ("z", "PP retained_product.rs combine_on (T-10b (ii)): the CombinationAttempts. " + Z),
    "for operand in &mut combination.operands": ("3", "PP retained_product.rs combine_on: one retained combination's operands (one per term). " + T),
    "for operand in &self.combinations[k].operands": ("3", "PP retained_product.rs combination_call: one retained combination's operands. " + T),
    "iter:if !combination.mechanics || !open || !distinct_terms(&combination.terms) || !combination.terms.iter().any": ("3", "PP retained_product.rs combination_capacity: one captured combination's terms. " + T),
    "iter:if !self.combinations.iter().any": ("z", "PP retained_product.rs combine: the CombinationAttempts. " + Z),
    "iter:self.combinations.iter().filter": ("z", "PP retained_product.rs combine_on (ii): the CombinationAttempts. " + Z),
    "iter:|combination| matches!(combination.end, CombinationEnd::Retained) && combination.operands.iter().any": ("3", "PP retained_product.rs combine_on (ii): one combination's operands. " + T),
    "iter:self.combinations.iter() .filter(|combination| matches!(combination.end, CombinationEnd::Retained) && combination.operands.iter().any(|operand| operand.case == owner)).map": ("z", "PP retained_product.rs combine_on (ii): requested_by over the CombinationAttempts. " + Z),
    "iter:self.combinations[k].operands.iter().find_map": ("3", "PP retained_product.rs combine_on (iii): one combination's operands. " + T),
    # retained_wire.rs (B2-P serializer)
    "for combination in combos.iter()": ("z", "PP retained_wire.rs serialize_cases_with: one CombinationAttempt per model combination (checked equal to the model's first). " + Z),
    "for combination in combos.iter_mut()": ("z", "PP retained_wire.rs serialize_cases_with: the CombinationAttempts. " + Z),
    "for (combination, ((raw_combination, refs), part)) in combos.iter().zip(model_combinations.iter().zip(combination_refs).zip(combination_parts))": ("z", "PP retained_wire.rs serialize_cases_with: combinations[]. " + Z),
    "for (operand, requested) in combination.operands.iter().zip(&call.requested_operands)": ("3", "PP retained_wire.rs serialize_combination_attempt: one retained combination's operands. " + T),
    "iter:pc.combinations.len() || combos.iter().enumerate().any": ("z", "PP retained_wire.rs serialize_cases_with: the CombinationAttempts. " + Z),
    "iter:model_combinations.iter().map": ("z", "PP retained_wire.rs serialize_cases_with: the raw request's combinations (equal in number to the captured, checked first). " + Z),
    "iter:if combination_ids.iter().zip(&pc.combinations).any": ("z", "PP retained_wire.rs serialize_cases_with. " + Z),
    "iter:combos.iter().filter(|combination| combination.call.is_some()).map": ("z", "PP retained_wire.rs serialize_cases_with: the combinations with a Call. " + Z),
    # readers on the precommit (project(successor): preview-physics-1 base) and RS's transport metadata
    "for gate in gates": ("z", "RE preview_physics_evidence.rs validate_preview_physics_evidence: combination_gates of the producer's own staged successor, one entry per model combination (B2-P T-9'). " + Z),
    "for combination in &combination_order": ("z", "RE preview_physics_evidence.rs validate_preview_physics_evidence: the model's combinations in order. " + Z),
    'for gate in list(&evidence[" "])': ("z", "RE retained_precision.rs preview_physics_transport_metadata (RS at c3cf970ad3; lane RS is changing this reader: phase 2): combination_gates, one per model combination. " + Z),
}
# headers longer than the inventory's 300-character excerpt: matched by an anchored prefix
PREFIX = {
    "iter:for (request, case) in cases.iter().enumerate() { set.push(request, case[\" \"].as_str()?)?; } if combinations.iter().any": ("z", "PP lib.rs w1_case_ids: `combinations.iter().any` over the request's combinations. " + Z),
    "iter:if gate.is_some_and(|gate| gate[\" \"] == serde_json::Value::Bool(true)) { CombinationEnd::BaseWithheld(": ("3", "PP retained_product.rs dispositions: `combination.terms.iter().any` over one combination's terms. " + T),
}
NEW.update({
    'iter:results .iter() .filter(|item| item.kind == " ").filter_map': ("Pall", "PP lib.rs append_combination_modulus_basis_records: the rows published so far (case and combination rows): at most the envelope's rows. Pall = C_eq * P_final bounds them: a combination's block is at most a case's row universe (RV115: 7n + 50m + 8g rows, with <= 3 modulus records within P_final's + 3)"),
})
EDGE_ZERO = [
    ("lib.rs:2354:run_linear_static_preview_observed", "lib.rs:13102:append_combination_results",
     "the legacy combination path runs only with `preview` None (lib.rs:2693-2695, 2772-2781): is_exact or source_selected. On D1.3's branch L is_exact is false, so a case is source-selected, which source_eligible (lib.rs:1170) forbids with any combination; so whenever it runs, model.combinations is empty and its loop iterates nothing. On branch E, D1.4 refuses combinations"),
]
EDGE_PER_CALL = [
    ("lib.rs:12999:append_combined_vector_magnitude", "lib.rs:13356:combination_diag", "1",
     "both combination_diag pushes are followed by `return` (lib.rs:13042-13045, 13055-13064): at most one per call"),
]
# ---- batch 2 (from the first full run's unmapped headers, fan-outs and growth)
NEW.update({
    "for term in terms": ("3", "load_case_algebra evaluate_linear_combination: one mechanics combination's terms (CombinationExpression::evaluate's filter_map over the authored terms). " + T + ". The header occurs elsewhere (FK structural.rs, ledger.rs, formation_check.rs, RE source_blocks.rs) only off the TEXT stacks of this graph: none of those is evaluated (the loop log's contexts are load_case_algebra's only)"),
    "for status in &operand.statuses": ("2", "load_case_algebra collect_statuses: an AlgebraOperand's statuses; every operand on the graph is built with exactly two (MechanicsSolved, HumanReviewRequired: preview_physics.rs evaluate_values, lib.rs append_combined_vector_magnitude, append_combination_results)"),
    "iter:terms .iter().filter_map": ("3", "PP lib.rs CombinationExpression::evaluate: one mechanics combination's terms. " + T),
    'iter:" ":terms.iter().map': ("3", "PP retained_wire.rs combination_expression: one combination's terms. " + T),
    "iter:values .iter().map": ("3", "PP preview_physics.rs evaluate_values: one (case, value) per operand. " + O),
    "for (operand_index, &(case, factor)) in terms.iter().enumerate()": ("3", "PP retained_product.rs combine_on (T-10b (i)): one retained combination's terms. " + T),
    "for (index,row) in ordinary.results.iter().enumerate().skip(end)": ("Pall", "PP retained_product.rs bind_combination_rows: the envelope's rows after the case blocks (the combination rows): at most the envelope's rows, Pall = C_eq * P_final"),
    "for record in preparations": ("z", "PP retained_wire.rs serialize_cases_with: operand_preparations, one per not_required owner case named by a retained combination: <= c - 1 with c <= 3 - z; within B2's domain at most one (B2-C REVISION_01 (h)); bounded here by z"),
    "for owner in owners": ("z", "PP retained_product.rs combine_on (T-10b (ii)): one operand preparation per owner case (as `for record in preparations`): bounded by z"),
    "iter:c.requested_operands.iter().map": ("3", "PP retained_wire.rs invocation_arrays_mapped: one combination Call's requested operands. " + O),
    "iter:combos.iter().filter": ("z", "PP retained_wire.rs serialize_cases_with: the CombinationAttempts. " + Z),
    "for &(factor, operand) in operands": ("3", "FK origins.rs solve_combination_sources: one Call's operands (PP passes one per term). " + T),
    "for (operand, (_, recorded)) in operands.iter().enumerate()": ("3", "FK origins.rs solve_combination_sources: one Call's operands. " + T),
    "for (n, s) in names.iter().zip(view.stages.iter().skip(1))": ("9", "PP retained_wire.rs serialize_combination_attempt: the zip with the nine-name array literal"),
})
PREFIX.update({
    "iter:if last.is_some_and(|last| charged != e.exact(last.invocation_after, \" \")) || inv.calls().windows(2).any": ("1+z", "PP retained_wire.rs invocation_arrays_mapped: windows over the invocation's Calls: one batch Call and at most one MechanicsCombinationCall per combination"),
})
EDGE_PER_CALL += [
    ("preview_physics.rs:919:append_combination_results", "lib.rs:12999:append_combined_vector_magnitude", "z*n",
     "per combination, only the base rows of kind displacement_magnitude reach it (preview_physics.rs:961-963), one base id per node"),
    ("preview_physics.rs:919:append_combination_results", "preview_physics.rs:892:evaluate_values", "z*(P+8*g)",
     "per combination: one call per base row (<= P_final base ids, one per row of a case's universe) and, per support, six component calls and two magnitude calls (preview_physics.rs:999-1046)"),
]
LEX_SITE_SIZE = [
    ("load_case_algebra/src/lib.rs:182", {"class": "ident", "why": "SQ2 G5: AlgebraFinding::new's subject_id: every caller passes an operand or load-case id (<= 128 B, D1.9 TEXT_BYTES), a literal (\"linear_combination\", \"range_envelope\"), or a format! of two ids (moved; counted at its own site, load_case_algebra lib.rs:408)"}),
    ("load_case_algebra/src/lib.rs:183", {"class": 128, "why": "SQ2 G5: AlgebraFinding::new's message: every caller passes a string literal of at most 80 bytes (load_case_algebra lib.rs:263-575: the longest is 'algebra operands with incomplete model status cannot produce an algebra output'), so <= 128"}),
    ("load_case_algebra/src/lib.rs:120", {"class": "ident", "why": "SQ2 G5: AlgebraOperand::new's operand_id: callers pass case.to_string() / operand_id.to_string() (an id <= 128 B)"}),
    ("load_case_algebra/src/lib.rs:121", {"class": "ident", "why": "SQ2 G5: AlgebraOperand::new's state_id: as operand_id"}),
]
CG_RULES = [
    {"caller": "core/product_physics/src/retained_product.rs:5457:combination_call", "name": "freeze", "targets": ["core/product_physics/src/retained_receipt.rs:57:freeze"],
     "added": "SQ2 G5 (B2-P)", "evidence": "retained_product.rs combination_call `combination.trace.freeze(capture)`: trace is a trace::PreparedTrace (CombinationAttempt.trace), so PreparedTrace::freeze (retained_receipt.rs:57), never PreparedCases::freeze"},
    {"caller": "core/product_physics/src/retained_product.rs:5547:prepare_operand", "name": "freeze", "targets": ["core/product_physics/src/retained_receipt.rs:57:freeze"],
     "added": "SQ2 G5 (B2-P)", "evidence": "retained_product.rs prepare_operand `trace.freeze(self)`: trace is the local trace::PreparedTrace::default()"},
    {"caller": "core/product_physics/src/retained_wire.rs:1926:serialize_cases_with", "name": "native", "targets": ["core/product_physics/src/retained_product.rs:5285:native"],
     "added": "SQ2 G5 (B2-P)", "evidence": "retained_wire.rs serialize_cases_with `combination.native()` (combos: [rp::CombinationAttempt]): CombinationAttempt::native, a getter; never PreparedCases::native"},
    {"caller": "core/product_physics/src/retained_wire.rs:1926:serialize_cases_with", "name": "retained", "targets": ["core/product_physics/src/retained_product.rs:5287:retained"],
     "added": "SQ2 G5 (B2-P)", "evidence": "retained_wire.rs serialize_cases_with `combination.retained()`: CombinationAttempt::retained"},
    {"caller": "core/product_physics/src/lib.rs:3336:w1_transaction", "name": "native", "targets": ["core/product_physics/src/retained_product.rs:4056:native"],
     "added": "SQ2 G5 (B2-P)", "evidence": "lib.rs w1_transaction `prepared.native()`: prepared is PreparedCases, so PreparedCases::native"},
    {"caller": "core/product_physics/src/preview_physics.rs:831:gate_reason", "name": "sum", "targets": [],
     "added": "SQ2 G5 (B2)", "evidence": "preview_physics.rs gate_reason `combination.terms.iter().map(|t| t.factor).sum()`: std Iterator::sum over f64, not retained_wire's Enc::sum"},
]
NEW.update({
    'iter:" ":old.iter().map': ("m", "PP retained_wire.rs operational_value (B2-P's helper of C3's `operational` member): the old operational records, one OperationalSpent per member"),
    'iter:" ":new.iter().map': ("m", "PP retained_wire.rs operational_value: the new operational records, one per member"),
    "iter:entries.iter().map": ("m", "PP retained_wire.rs preparation_members (B2-P's helper): C3's preparation entries, one per member"),
})
FV = "the call graph's function-as-value heuristic (callgraph_g5.py:590-612) reads a pattern binding passed to a tuple variant as a function value (its `bound` scan does not see a binding nested in a variant pattern), so the edge is not a call"
EDGE_ZERO += [
    ("retained_product.rs:5457:combination_call", "source_recovery.rs:1432:solve", "`Some(k::ExecutionOutcome::Selected(solve)) => k::RecordedOperand::Selected(solve)` (retained_product.rs:5465): " + FV),
    ("retained_product.rs:5457:combination_call", "structural/exact_boundary/functionals.rs:200:solve", "as the source_recovery edge: " + FV),
    ("retained_product.rs:2562:preview_case_evidence", "load_reference.rs:223:code", "`serde_json::Value::String(code)) if ...` (retained_product.rs:2594): " + FV),
    ("retained_product.rs:2562:preview_case_evidence", "load_reference_source.rs:46:code", "as the load_reference edge: " + FV),
    ("retained_product.rs:5357:combine_on", "physics_source.rs:50:case", "`case` is a pattern binding (`&(case, factor)`; retained_product.rs:5370-5373) passed as a value: " + FV),
]
SITE_ZERO = [
    ("retained_wire.rs:2062", "B2-P: Option::replace on a sources[] slot (serialize_cases_with, operand preparations), not a String (as I104's :1964 and :2048)"),
    ("retained_wire.rs:2084", "B2-P: Option::replace on a sources[] slot (serialize_cases_with, combination sources), not a String"),
    ("retained_product.rs:299", "B2-P: Cell::replace on `lent` (an Option<usize> slot index; with_case), not a String"),
]
ARGS_FIRST = [
    {"re": r'^gate\.and_then\(\|gate\|\s*gate\[" "\]\.as_str\(\)\)\.unwrap_or_default\(\)$', "max": "static",
     "why": "B2-P dispositions (retained_product.rs:5327): a withheld gate entry's `reason`, one of the three gate codes the producer itself wrote (preview_physics.rs gate_reason; COMBINATION_GATE_CODES), copied with to_owned: a static literal"},
]
def _a(**kw): return {k.replace("__", "."): v for k, v in kw.items()}
IN, ST, RS_, CP, NUM_ = ({"bytes": 128, "source": "IN128"}, {"bytes": 128, "source": "STATIC"}, {"bytes": 1024, "source": "RES"},
                         {"bytes": 600, "source": "COMP"}, {"bytes": 20, "source": "NUM"})
P_ = "core/product_physics/src/"; A_ = "core/loads/load_case_algebra/src/lib.rs:"
ID_AUDIT = [
    (P_ + "lib.rs:13054", {"reference.entity_ref": IN, "combination.id": IN}),
    (P_ + "lib.rs:10246", {"item.entity_ref": IN, "metadata.basis": {"bytes": 2914, "source": "COMP"}}),
    (P_ + "lib.rs:13070", {"reference.id": RS_, "combination.id": IN}),
    (P_ + "validation.rs:1991", {"load_case_ref": IN, "combination.id": IN}),
    (P_ + "validation.rs:2144", {"operand_id": IN, "combination.id": IN}),
    (A_ + "408", {"left.operand_id": IN, "right.operand_id": IN}),
    (P_ + "lib.rs:10280", {"stable_suffix(&combination.id)": IN, "stable_suffix(operand_id)": IN}),
    (P_ + "lib.rs:13018", {"base_id": RS_, "suffix": ST}),
    (P_ + "lib.rs:13366", {"stable_suffix(&combination.id)": IN, "stable_suffix(base_id)": RS_, "stable_suffix(code)": ST}),
    (P_ + "lib.rs:13388", {"stable_suffix(combination_id)": IN, "result_tail(base_id)": CP}),
    (P_ + "preview_physics.rs:943", {"identity(&[&combination.id])": CP}),
    (P_ + "preview_physics.rs:989", {"base_id": RS_, "combination.id": IN}),
    (P_ + "preview_physics.rs:1012", {"support.id": IN, "component": ST, "support.id.len()": NUM_}),
    (P_ + "preview_physics.rs:1058", {"identity(&[&combination.id])": CP}),
    (P_ + "preview_physics.rs:1067", {"identity(&[&combination.id])": CP}),
    (P_ + "retained_product.rs:3019", {"row.entity_ref": IN, "component": ST, "row.entity_ref.len()": NUM_}),
    (P_ + "retained_product.rs:3074", {"stable_suffix(id)": IN, "super::result_tail(&expected)": RS_}),
    (P_ + "retained_wire.rs:2136", {"combination_id": IN}),
    (P_ + "retained_wire.rs:2138", {"combination_id": IN}),
    (P_ + "retained_wire.rs:2328", {"combination_id": IN}),
    (P_ + "validation.rs:1948", {"stable_suffix(&combination.id)": IN}),
    (P_ + "validation.rs:1964", {"stable_suffix(&combination.id)": IN}),
    (P_ + "validation.rs:1983", {"stable_suffix(&combination.id)": IN, "stable_suffix(load_case_ref)": IN}),
    (P_ + "validation.rs:2013", {"stable_suffix(&combination.id)": IN}),
    (P_ + "validation.rs:2026", {"stable_suffix(&combination.id)": IN, "stable_suffix(&term.load_case)": IN}),
    (P_ + "validation.rs:2071", {"stable_suffix(&combination.id)": IN, "stable_suffix(minuend)": IN}),
    (P_ + "validation.rs:2104", {"stable_suffix(&combination.id)": IN}),
    (P_ + "validation.rs:2121", {"stable_suffix(&combination.id)": IN}),
    (P_ + "validation.rs:2136", {"stable_suffix(&combination.id)": IN, "stable_suffix(operand_id)": IN}),
    (A_ + "138", {"operand_id": IN}),
    (A_ + "341", {"operand.operand_id": IN}),
    (A_ + "398", {"left.operand_id": IN}),
    (A_ + "399", {"right.operand_id": IN}),
    (A_ + "466", {"operand.operand_id": IN}),
    (A_ + "120", {"operand_id": IN}),
    (A_ + "121", {"state_id": IN}),
    (A_ + "182", {"subject_id": IN}),
    (A_ + "183", {"message": {"bytes": 128, "source": "STATIC"}}),
    (P_ + "lib.rs:10288", {"operand_id": IN}),
    (P_ + "lib.rs:13085", {"row.id": RS_}),
    (P_ + "lib.rs:13364", {"base_id": RS_}),
    (P_ + "lib.rs:3205", {"case_ids": {"bytes": 0, "source": "NOTID"}}),
    (P_ + "lib.rs:3211", {"case_ids": {"bytes": 0, "source": "NOTID"}}),
    (P_ + "preview_physics.rs:809", {"c.id": IN}),
    (P_ + "preview_physics.rs:883", {"expression.result_metadata_basis()": ST}),
    (P_ + "preview_physics.rs:979", {"source.id": RS_}),
    (P_ + "preview_physics.rs:1022", {"s.id": RS_}),
    (P_ + "preview_physics.rs:1046", {"s.id": RS_}),
    (P_ + "retained_wire.rs:2242", {"source": {"bytes": 128, "source": "STATIC"}}),
    (P_ + "preview_physics.rs:986", {"identity(&[&combination.id,base_id])": {"bytes": 1200, "source": "COMP"}}),
    (P_ + "lib.rs:10291", {"combination.id": IN}),
    (P_ + "lib.rs:10297", {"combination.id": IN}),
    (P_ + "lib.rs:12906", {"combination.operand_ids": {"bytes": 384, "source": "IN128"}}),
    (P_ + "lib.rs:13094", {"combination.id": IN}),
    (P_ + "preview_physics.rs:880", {"combination.id": IN}),
    (P_ + "preview_physics.rs:947", {"combination.id": IN}),
    (P_ + "preview_physics.rs:990", {"combination.id": IN}),
    (P_ + "preview_physics.rs:1062", {"combination.id": IN}),
    (P_ + "preview_physics.rs:1071", {"combination.id": IN}),
    (P_ + "retained_product.rs:5522", {"capture.combinations[index].id": IN}),
] + [(P_ + "validation.rs:%d" % n, {"combination.id": IN}) for n in (1955, 1971, 2020, 2034, 2079, 2112, 2128)]
ID_AUDIT_WHY = ("SQ2 G5 (B2): IN128 = a model id (combination, load case, node, support or operand id; D1.9 TEXT_BYTES <= 128) or its "
    "stable_suffix (':' replaced, same length); STATIC = a literal or fixed vocabulary (components, gate and diagnostic codes, metadata basis labels); "
    "RES = a result row id (<= 1024, as the existing `row.id` audits); COMP = an identity() of one model id, or a result tail (600, as the existing "
    "audits); identity(&[&combination.id, base_id]) = 128 + 1024 + two length prefixes <= 1200; combination.operand_ids = a clone of <= 3 ids; "
    "`case_ids.clone()` clones an iterator (no text); retained_wire.rs:2242 `source` is source_identity's static kind token")
# load_case_algebra (findings exist only on an algebra refusal): per-call counts. The short key lib.rs:179:new is shared
# with stress_recovery's StressFinding::new, so only (caller, callee) edges, which are unique, are bound.
_AF = "load_case_algebra AlgebraFinding::new"
def _algebra_finding_new():
    """AlgebraFinding::new resolved by name at the snapshot. The basis short key lib.rs:179:new also named a stress_recovery
    lib.rs line; once that file's lines moved (the pressure-source join removed its #[path] correct_norm module, -4
    lines) the basis key is ambiguous between the two lib.rs files. load_case_algebra's definition is unchanged."""
    f = "core/loads/load_case_algebra/src/lib.rs"
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    hits = [i + 1 for i, l in enumerate(lines) if _re.match(r"\s*fn new\(code: FindingCode, subject_id: impl Into<String>, message: impl Into<String>\) -> Self \{$", l)]
    if len(hits) != 1:
        raise SystemExit(f"AlgebraFinding::new defined {len(hits)} times in {f}")
    return Resolved(f"lib.rs:{hits[0]}:new")
_AFN = _algebra_finding_new()
EDGE_PER_CALL += [
    ("lib.rs:292:evaluate_linear_combination", _AFN, "4", _AF + ": one empty-expression finding, then per term (<= 3) at most one of non-finite / duplicate / missing (each `continue`s)"),
    ("lib.rs:292:evaluate_linear_combination", "lib.rs:546:collect_statuses", "3", "one call per term (<= 3)"),
    ("lib.rs:292:evaluate_linear_combination", "lib.rs:524:same_dimension_or_record", "3", "one call per term (<= 3)"),
    ("lib.rs:426:evaluate_range_envelope", _AFN, "4", _AF + ": one empty-expression finding, then per operand (<= 3) at most one of duplicate / missing"),
    ("lib.rs:426:evaluate_range_envelope", "lib.rs:546:collect_statuses", "3", "one call per operand (<= 3)"),
    ("lib.rs:426:evaluate_range_envelope", "lib.rs:524:same_dimension_or_record", "3", "one call per operand (<= 3)"),
    ("lib.rs:364:evaluate_result_state_subtraction", _AFN, "4", _AF + ": duplicate (1), missing (<= 2, one per distinct id), dimension mismatch (1)"),
    ("lib.rs:364:evaluate_result_state_subtraction", "lib.rs:546:collect_statuses", "2", "the minuend's and the subtrahend's"),
    ("lib.rs:546:collect_statuses", _AFN, "3", _AF + ": empty statuses (1) or one per status (<= 2, every operand on the graph has two), plus model-incomplete (1)"),
    ("lib.rs:524:same_dimension_or_record", _AFN, "1", _AF + ": its one site"),
]
PREFIX.update({
    'for r in projected[" "].as_array_mut().ok_or_else(||': ("Pall", "RS retained_precision.rs project (B3b changed its error code by route): the projection's `results` rows, at most the envelope's rows (Pall = C_eq * P_final)"),
})
NEW.update({
    # B7a named bases add groups, not additional batch Calls. This exact
    # header precedes the historical combined calls/groups regex.
    "iter:inv.groups().iter().map": ("cases+z", "B7a stage 1: one group per distinct requested basis plus each combination group"),
    "for case in &operands": ("3", "PP preview_physics.rs append_combination_results: the operand case ids of one combination (expression.operand_ids()). " + O),
})
# I104 kept three D1-era "one per invocation" rules at 1 (calls, groups, material bases). B2 adds one
# MechanicsCombinationCall, with its own Group, per retained combination (FK solve_combination_sources; every combination
# successor at hand has calls = groups = 1 + (combinations with a Call)): calls and groups are rebound to 1 + z. Material
# bases stay one (retained_wire.rs material_basis emits one basis; every combination successor has material_bases = 1).
REBIND = [
    ('^for\\ \\(call_id,\\ call\\)\\ in\\ list\\(\\&b\\["\\ "\\]\\)\\.iter\\(\\)\\.enumerate\\(\\)$', "1+z",
     "RS g5_native: body.calls: the batch CaseBatchCall and one MechanicsCombinationCall per retained combination (B2-C §2.4)"),
    ('^for\\ \\(i,\\ g\\)\\ in\\ list\\(\\&b\\["\\ "\\]\\)\\.iter\\(\\)\\.enumerate\\(\\)$', "1+z",
     "RS g5_native: body.groups: one per Call (the batch's and each combination Call's own Group)"),
    ('^iter:inv\\.calls\\(\\)\\.iter\\(\\)\\.map$|^iter:inv\\.groups\\(\\)\\.iter\\(\\)\\.map$', "1+z",
     "PP retained_wire.rs invocation_arrays_mapped: the invocation's Calls and Groups, one batch and one per combination Call"),
]
# ---- phase 2: B2's RS reader (lane RS, merged at 2243be380b). The reader validates the producer's own staged successor at
# precommit (T17), so every receipt array is the producer's: body.sources <= 3 (case sources of prepared attempts, at most
# one registered operand preparation, one CombinationSource per combination with a Run: <= C_eq = 3 at every admitted shape;
# bound by att = 3), body.combinations = z, body.operand_preparations <= 1 within B2's domain (REVISION_01 (h); bound by z),
# one combination's operands/terms <= 3, a combination's row block <= P_final, a Group's imports <= 7 (SLOTS).
RSR = "RS retained_precision.rs (B2 reader) "
NEW.update({
    "for s in sources": ("att+z", RSR + "g1_combinations: body.sources: the CaseSources (prepared attempts and registered operand preparations, <= c) and one CombinationSource per combination with a Run (<= z)"),
    'for s in sources.iter().filter(|s| s[" "][" "] == " ")': ("z", RSR + "g1_combinations: the CombinationSources, one per combination with a Run"),
    'for o in list(&s[" "])': ("3", RSR + "g1_combinations, g8_combinations: one CombinationSource's operands, one per term. " + T),
    "for (j, e) in combos.iter().enumerate()": ("z", RSR + "g3: body.combinations. " + Z),
    "for (i, p) in ops.iter().enumerate()": ("z", RSR + "g3: body.operand_preparations (<= 1 within B2's domain, bounded by z)"),
    "for (k, (j, e)) in with_calls.into_iter().enumerate()": ("z", RSR + "g5_native_combinations: the combinations with a Call. " + Z),
    "for (t, o) in terms.iter().zip(requested)": ("3", RSR + "g5_native_combinations: one combination's terms. " + T),
    "for (((o, r), ci), t) in operands.iter().zip(requested).zip(&term_cases).zip(terms)": ("3", RSR + "g5_native_combinations: one CombinationSource's operands, zipped with its terms (99cb958e85). " + T),
    # the same loop before 99cb958e85 (the registered tree's revision, f78f52b7e2): kept so the tool prices both sides
    "for ((o, r), ci) in operands.iter().zip(requested).zip(&term_cases)": ("3", RSR + "g5_native_combinations: one CombinationSource's operands (before 99cb958e85). " + T),
    "for x in &expected": ("7", RSR + "g5_native_combinations: the expected Group imports, at most one per slot (SLOTS has 7 entries)"),
    'iter:list(&g[" "]) .iter().map': ("7", RSR + "g5_native: a Group's imports, at most one per slot (SLOTS has 7 entries; a batch Group has none)"),
    'for e in list(&b[" "])': ("z", RSR + "combination_causes, r_comb_1: body.combinations. " + Z + ". The header occurs only in these two functions on this graph (rule_matches)"),
    'for p in list(&b[" "])': ("z", RSR + "operand_preparations: body.operand_preparations (<= 1 within B2's domain, bounded by z). The header occurs only there on this graph"),
    'for e in list(&b[" "]) .iter() .filter(|e| e[" "] == " ")': ("z", RSR + "numeric_cases, g8_combinations: the retained_selected combinations. " + Z),
    "for o in operands": ("3", RSR + "combination_sources, g8_combinations: one CombinationSource's operands. " + T),
    'for e in list(&body[" "])': ("z", RSR + "g5a, validate (G6): body.combinations. " + Z + ". The header occurs only in these two functions on this graph"),
    'for s in list(&b[" "]) .iter() .filter(|s| s[" "][" "] != " ")': ("att", RSR + "g8: the CaseSources: prepared attempts and registered operand preparations (a preparation only for a case outside A), <= c = att at a shape"),
    'for a in list(&b[" "]) .iter() .filter(|a| a[" "][" "] != " ") .chain(list(&b[" "]))': ("att", RSR + "g8: the case attempts and the operand preparations: a <= c attempts and a preparation only for a case outside A, so <= c <= 3"),
    'for s in list(&b[" "]) .iter() .filter(|s| s[" "][" "] == " ")': ("z", RSR + "g8_combinations: the CombinationSources. " + Z),
    'for a in list(&b[" "]) .iter() .filter(|a| a[" "][" "] == " ")': ("z", RSR + "g8_combinations: the CombinationAttempts. " + Z),
    "for row in rows_for(source, e)": ("P", RSR + "r_comb_1, validate (G6): one combination's rows (a combination's block <= P_final rows)"),
})
PREFIX.update({
    'iter:authored.len() && list(&b[" "]) .iter() .zip(authored).all': ("z", RSR + "g8 (G8 invocation): body.combinations against the request's. " + Z),
})
REBIND = REBIND + [
    ('^for\\ c\\ in\\ cases$', "cases+z", RSR + "g5b, g5b_exact_evidence, g5c: the numeric owners (numeric_cases): the selected cases and, B2, the retained_selected combinations (<= c + z)"),
]
def _rs_lines(name, f="core/reporting/result_export/src/retained_precision.rs"):
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    top = [i + 1 for i, l in enumerate(lines) if _re.match(r"fn\s+" + _re.escape(name) + r"\b", l)]
    wrap = [i + 1 for i, l in enumerate(lines) if _re.match(r"\s+pub fn\s+" + _re.escape(name) + r"\b", l)]
    if len(top) != 1 or len(wrap) != 1: raise SystemExit(f"_rs_lines: {name} {top} {wrap}")
    return Resolved(f"core/reporting/result_export/src/retained_precision.rs:{top[0]}:{name}"), Resolved(f"core/reporting/result_export/src/retained_precision.rs:{wrap[0]}:{name}")
def _rs_full(name): return Resolved("core/reporting/result_export/src/" + rs(name))
_WR = "RS retained_precision.rs reader_logic (a #[doc(hidden)] test-only forwarding module, no production caller): `super::NAME(..)` is the module-level fn, never the wrapper itself"
CG_RULES = CG_RULES + [r for name, callers in (("combination_reason_table", ["combination_attempt"]), ("combination_causes", ["g5_products"]), ("r_comb_1", ["validate"]))
    for real, wrap in [_rs_lines(name)]
    for r in ([{"caller": wrap, "name": name, "targets": [real], "added": "SQ2 G5 (B2's RS reader)", "evidence": _WR.replace("NAME", name)}]
              + [{"caller": _rs_full(c), "name": name, "targets": [real], "added": "SQ2 G5 (B2's RS reader)",
                  "evidence": f"RS {c} calls the module-level {name}; the same-named reader_logic wrapper is test-only (by name the call would fan out to both)"} for c in callers])]
def rs_line(text, fn, f="core/reporting/result_export/src/retained_precision.rs"):
    """The one line holding `text` inside fn `fn` (from its definition to the next top-level fn), as an audit key."""
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    start = int(rs(fn, f).split(":")[1]); end = next((i + 1 for i in range(start, len(lines)) if _re.match(r"(pub(\([a-z]+\))?\s+)?fn\s", lines[i])), len(lines))
    hits = [i + 1 for i in range(start - 1, end - 1) if text in lines[i]]
    if len(hits) != 1: raise SystemExit(f"rs_line: {text!r} in {fn}: {hits}")
    return Resolved(f"{f}:{hits[0]}")
PREFIX.update({
    'iter:list(&m[" "]).len() && list(&x[" "]).iter().zip(list(&m[" "])).all': ("3", RSR + "expression_matches (G8 invocation): one mechanics combination's terms. " + T),
})
ID_AUDIT = ID_AUDIT + [
    (rs_line('result_id: text(&row["id"]).into(),', "r_comb_1"), {'text(&row[""])': RS_}),
]
# Silent-drop guard D3 (retained-pricing-rules.md §8.19, SD-RK): PP preview_physics.rs append_combination_results names
# every withheld row, so two basis keys of its audited sites no longer carry by line: the source-missing diagnostic's refs
# (basis :990) moved into the `source_missing` closure, where its text is no longer unique in the function (the new
# LOAD_COMBINATION_ROW_WITHHELD diagnostic has the same refs line), and a support component row's source refs (basis
# :1022) are now pushed to the group's `rows` (`rows.push(..)`, was `results.push(..)`). Both are re-keyed by name at the
# snapshot to the same sites with unchanged audits; a source without the closure keeps the basis keys.
def _pp_combination_line(text, after=None, within=8):
    f = P_ + "preview_physics.rs"
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    start = int(rs("append_combination_results", f).split(":")[1])
    end = next((i + 1 for i in range(start, len(lines)) if _re.match(r"(pub(\([a-z]+\))?\s+)?fn\s", lines[i])), len(lines))
    if after is not None:
        anchors = [i + 1 for i in range(start - 1, end - 1) if after in lines[i]]
        if len(anchors) != 1: raise SystemExit(f"SD-RK: {after!r} in append_combination_results: {anchors}")
        start, end = anchors[0], min(end, anchors[0] + within)
    hits = [i + 1 for i in range(start - 1, end - 1) if text in lines[i]]
    if len(hits) != 1: raise SystemExit(f"SD-RK: {text!r} in append_combination_results: {hits}")
    return Resolved(f"{f}:{hits[0]}")
if "let source_missing = |diagnostics: &mut Vec<Diagnostic>|" in open(_os.path.join(_SNAP, P_ + "preview_physics.rs"), encoding="utf-8").read():
    _SD_RK = {
        P_ + "preview_physics.rs:990": _pp_combination_line("vec![combination.id.clone()],", after="combination-source-missing:"),
        P_ + "preview_physics.rs:1022": _pp_combination_line(
            "rows.push(combination_row(combination, &expression, sources[0], qualified_combination_result_id(&combination.id, &base(component)), value, sources.iter().map(|s| s.id.clone()).collect()));"),
    }
    ID_AUDIT = [(_SD_RK.get(k, k) if isinstance(k, str) and type(k).__name__ != "Resolved" else k, v) for k, v in ID_AUDIT]
ARGS_FIRST = ARGS_FIRST + [
    {"re": r'^text\(&x\[" "\]\)$', "max": "static",
     "why": RSR + "g5_native: `text(&x[\"slot\"]).to_owned()`: a Group import's slot name, one of the producer's seven slot tokens (SLOTS): a static literal"},
]
EDGE_ZERO += [(rs("expression_matches"), t_,
    "RS expression_matches `.map(text)` passes RS's own `text` (retained_precision.rs's module fn, a borrow, no allocation) as a value; " + FV)
    for t_ in ("load_reference.rs:237:text", "physics_evidence.rs:45:text", "physics_source.rs:25:text", "preview_physics_evidence.rs:97:text", "source_blocks.rs:41:text")]
CG_RULES = CG_RULES + [{"caller": _rs_full("g5_products"), "name": "reason_table", "targets": [_rs_lines("reason_table")[0]], "added": "SQ2 G5 (B2's RS reader)",
    "evidence": "RS g5_products calls the module-level reason_table; the same-named reader_logic wrapper is test-only (by name the call would fan out to both)"}]
# SQ2: at a shape (c, z), TEXT's `att` is the attempted cases a <= c (the typed forms keep a = C_eq = 3). The loops whose
# domain also holds the combinations' attempts, Runs or sources are rebound to att + z.
REBIND = REBIND + [
    ('^for\\ \\(ai,\\ a\\)\\ in\\ list\\(\\&b\\["\\ "\\]\\)\\.iter\\(\\)\\.enumerate\\(\\)$', "att+z", RSR + "g5_products: body.product_attempts: one per case in A and one CombinationAttempt per combination with a Run"),
    ('^for\\ s\\ in\\ list\\(\\&r\\["\\ "\\]\\["\\ "\\]\\)$', "att+z", RSR + "g1: body.sources: the CaseSources (<= c) and one CombinationSource per combination with a Run"),
    ('^for\\ \\(i,\\ a\\)\\ in\\ list\\(\\&b\\["\\ "\\]\\)\\.iter\\(\\)\\.enumerate\\(\\)$', "att+z", RSR + "g3: body.product_attempts (cases in A and combinations with a Run)"),
]
SITE_FROM_REBIND = [("retained_precision.rs:223", "1+4*64*(att+z)", RSR + "g5_native's tw(): one Run per attempted case and one per combination Call with a Run"),
                    ("retained_precision.rs:225", "1+4*64*(att+z)", RSR + "as :223")]
# O-10 (T4's heads): rules keyed by name at the snapshot; their arguments are in DEL-04-08's
# Design/retained-pricing-rules.md.
exec(open(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "o10_rules.py")).read())
# B7b-VP: model 0.5.0's exact half adds validate_profile's per-pipe corrosion-allowance check (pressure_runtime.rs): on a
# 0.5.0 document each pipe takes at most one arm of `match (&model.pressure_contract, &pipe.section.corrosion_allowance)`
# (EXACT_PRESSURE_CORROSION_ALLOWANCE_REQUIRED on E, CORROSION_ALLOWANCE_WITHOUT_PRESSURE_CONTRACT on L): <= m problem()
# calls per validate_profile call, independent of the per-load arm the base rule bounds by L. Its contract-arm problems
# cannot fire on admitted inputs (L has no contract; E's namespace requires {3.0.0, exact_pressure_v3, euler_bernoulli});
# every admitted document is 0.5.0, so neither version arm fires. The base per-call bound is rebound L -> L+m.
_VP_FILE = "core/product_physics/src/pressure_runtime.rs"
if fnkey("validate_profile", _VP_FILE) and fnkey("problem", _VP_FILE):
    _vp_src = open(_os.path.join(_SNAP, _VP_FILE), encoding="utf-8").read()
    if "match (&model.pressure_contract, &pipe.section.corrosion_allowance) {" in _vp_src:
        if _vp_src.count("for pipe in &model.pipe_segments {") != 1:
            raise SystemExit("B7b-VP: validate_profile's per-pipe corrosion loop changed; re-review")
        EDGE_PER_CALL = list(EDGE_PER_CALL) + [(fnkey("validate_profile", _VP_FILE), fnkey("problem", _VP_FILE), "L+m",
            "B7b-VP: validate_profile's problem() calls: <= L from the base rule's per-load arm (PRESSURE_MODEL_REAUTHOR_REQUIRED "
            "or EXACT_PRESSURE_REQUIRES_REGION, one per primitive load) plus <= m from the per-pipe corrosion-allowance match "
            "(one arm per pipe); the 0.5.0 contract/shear/version arms cannot fire on an admitted document")]
# B7a E13 leftover: the support-vector reaction arm no longer exists.
# The complete current lib.rs pin guards this exact ID preimage removal;
# all surviving displacement and other decision audits continue strict carry.
ID_AUDIT = _labels_model["retire_combination_reaction_audit"](_SNAP, ID_AUDIT)

# B7a: exact new owner headers and site-local positive metadata classes.
# Existing O10 match checks, generic strict mapping and all other rules stay.
_wide_model = __import__("runpy").run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "b7a_domain_model.py"))
_wide_loops, _wide_audits, _wide_lex = _wide_model["text_decisions"](_SNAP, Resolved)
O10_LOOPS_LAST = list(globals().get("O10_LOOPS_LAST", [])) + _wide_loops
ID_AUDIT = list(ID_AUDIT) + _wide_audits
LEX_SITE_SIZE = list(LEX_SITE_SIZE) + _wide_lex

# Closed current formed-load premise repair; existing strict carry/O10 remain.
_formed_model = __import__("runpy").run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "b7a_formation_model.py"))
O10_LOOPS_LAST = _formed_model["rebind_o10"](_SNAP, O10_LOOPS_LAST)
_formed_totals, _formed_audits = _formed_model["decisions"](_SNAP, Resolved)
LOOP_TOTAL = list(globals().get("LOOP_TOTAL", [])) + _formed_totals
ID_AUDIT = list(ID_AUDIT) + _formed_audits
SITE_TOTAL = list(globals().get("SITE_TOTAL", [])) + _formed_model["site_totals"](_SNAP, Resolved)
LEX_SITE_SIZE = list(LEX_SITE_SIZE) + _formed_model["lex_sizes"](_SNAP, Resolved)
SITE_FROM = list(globals().get("SITE_FROM", [])) + _formed_model["first_return_rules"](_SNAP, Resolved)

# Five additional immediate-return allocations, with full owned call counts.
_owned_returns = __import__("runpy").run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "b7a_returning_errors.py"))
SITE_FROM = list(SITE_FROM) + _owned_returns["rules"](_SNAP, Resolved)

# W1: positive member-only source, wire and independent-reader traversals.
_w1_members = __import__("runpy").run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "w1_member_load_model.py"))
_w1_member_loops, _w1_member_audits = _w1_members["text_decisions"](_SNAP, Resolved)
O10_LOOPS_LAST = list(O10_LOOPS_LAST) + _w1_member_loops
ID_AUDIT = list(ID_AUDIT) + _w1_member_audits
LEX_SITE_SIZE = list(LEX_SITE_SIZE) + _w1_members["identifier_lex_sizes"](_SNAP, Resolved)
# The tuple-bound numeric `gap` passed to n(gap) is not a function value.
# Remove only this impossible target; real nonlinear constructors remain.
O10_EDGE_ZERO = list(O10_EDGE_ZERO) + [(fnkey("prepared_maxima", PP_ + "retained_product.rs"),
    fnkey("gap", "core/solver/nonlinear_supports/src/lib.rs"),
    "W1-GAP: source-pinned prepared_maxima binds gap as an f64 tuple member and passes it to its number closure; no nonlinear support constructor is called")]


# Exact current producer spellings, applied after ordinary decision carry.
B7A_SCOPED_TEXT = {"model": _os.path.join(_os.path.dirname(__file__), "..", "base", "b7a_scoped_text_model.py"), "snapshot": _SNAP}

# Exclusive current-owner allocation choices after strict carry.
B7A_OWNER_CHOICES = True
