"""O-10 rules (T4's heads), executed at the end of sq2_decisions_common.py (shared by routes L and E). Every key is
resolved by name at the snapshot (`fnkey`), so the rules carry to any head without line numbers; a rule whose function
does not exist at the snapshot is not added (it has nothing to bound there). The argument for each rule is in
DEL-05-02 Design/retained-pricing-rules.md under the same label."""


def fnkey(name, f):
    """`basename-relative:line:name` of the one top-level-or-impl `fn name` in file f at the snapshot, else None."""
    p = _os.path.join(_SNAP, f)
    if not _os.path.exists(p):
        return None
    lines = open(p, encoding="utf-8").read().split("\n")
    hits = [i + 1 for i, l in enumerate(lines) if _re.match(r"\s*(pub(\([a-z]+\))?\s+)?(const\s+)?fn\s+" + _re.escape(name) + r"\b", l)]
    if len(hits) != 1:
        return None
    return Resolved(f"{f.split('/src/')[-1]}:{hits[0]}:{name}")


RE_ = "core/reporting/result_export/src/"
O10_EDGE_ZERO, O10_SITE_FROM, O10_SITE_ZERO, O10_ID_AUDIT = [], [], [], []
O10 = "O-10 (T4): "
# R1 (Agent 0's ruling: accepted). The pressure-1 arm of the precommit reader's dispatch.
_c, _e = fnkey("for_source", RE_ + "semantic_contract.rs"), fnkey("validate_pressure_evidence", RE_ + "physics_evidence.rs")
if _c and _e:
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(_c, _e, O10 + "R1 (ruled accepted): the precommit reader validates project(successor), whose "
        "producer.semantic_contract_id is the route's base id (preview-physics-1 on route L, physics-1 on route E; RS project()), "
        "never pressure-1, so for_source never takes the PRESSURE_ID arm; the same argument as the registered G4 (T17) siblings")]


def site(name, f, text, full=False, row=None):
    """The one line holding `text` inside fn `name` of file f (to the next fn at the same or lower indent), or None. With
    `row`, the TEXT row's line: the nearest line at or above it holding `row` (a multi-line macro's first line)."""
    k = fnkey(name, f)
    if not k:
        return None
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    start = int(k.split(":")[1]); ind = len(lines[start - 1]) - len(lines[start - 1].lstrip())
    end = next((i + 1 for i in range(start, len(lines)) if _re.match(r"\s*(pub(\([a-z]+\))?\s+)?fn\s", lines[i]) and len(lines[i]) - len(lines[i].lstrip()) <= ind), len(lines) + 1)
    hits = [i + 1 for i in range(start - 1, end - 1) if text in lines[i]]
    if len(hits) != 1:
        raise SystemExit(f"o10 site(): {text!r} in {f}:{name}: {hits}")
    if row:
        hits = [next(i for i in range(hits[0], start - 1, -1) if row in lines[i - 1])]
    return Resolved((f if full else f.split("/src/")[-1]) + f":{hits[0]}")


PP_ = "core/product_physics/src/"
FK_ = "core/solver/frame_kernel/src/"
CE = RE_ + "connector_evidence.rs"
D14 = ("D1.4 (retained_memory.rs D1.4: `if !m.components.is_empty() { return refuse(C::Invocation, F::Components) }`): an admitted "
       "model has no components")
NOV3 = ("D1.3 (namespace_branch) admits only schema 0.1.0/0.2.0 with no pressure contract, or 0.3.0 with "
        "2.0.0/exact_straight_pressure_v2; an ObjectiveConnector is made only by joint::build_connector (FK ObjectiveConnector "
        "fields are private; ObjectiveConnector::new is called only there), whose one call (build_model_for_members) runs only "
        "when exact_contract(model) == PressureV3 (3.0.0/exact_pressure_v3), and iterates model.components")
# O10_LOOPS_LAST: (exact loop header, bound, why, the only functions it may match: checked after the run by o10_check.py).
O10_LOOPS_LAST = []
if fnkey("append_connector_results", PP_ + "lib.rs"):
    O10_LOOPS_LAST += [
        ("for (connector, record) in built.connectors.iter().zip(&built.connector_records)", "0",
         O10 + "L1: built.connectors (and connector_records, pushed together) is empty under D1. " + NOV3 + "; and " + D14,
         ["lib.rs:append_connector_results", "lib.rs:add_connector_reference_loads"]),
        ("for c in &source.connectors", "0",
         O10 + "L2: FK formation_check's source.connectors are the built model's connectors (PP build_model_for_members; NI "
         "adapter copies of them, or empty), empty under D1. " + NOV3, ["structural/formation_check.rs:body_scales"]),
        ("iter:connectors .iter().map", "0",
         O10 + "L3: NI structural_adapter's connectors are the built model's (PP lib.rs `connectors: built.connectors.clone()`) "
         "or empty, empty under D1. " + NOV3, ["structural_adapter.rs:new"]),
    ]
if fnkey("validate", CE):
    O10_LOOPS_LAST += [
        ("for r in records", "0",
         O10 + "E1 (route E, connector_evidence::validate): `records` non-empty returns Err at `require_connector(admitted || "
         "records.is_empty(), \"UNSUPPORTED\")?` before this loop unless `admitted`; admitted = (contract == PressureV3), and the "
         "reached callers pass StraightV2 (validate_physics_evidence -> validate_physics_evidence_in(.., StraightV2)); the "
         "PressureV3 callers are validate_pressure_evidence (physics_evidence.rs) and its load-state arm "
         "load_reference::validate_pressure_load_reference_evidence (load_reference.rs:387, called only from "
         "validate_pressure_evidence), both behind the edge R1 zeroes", ["connector_evidence.rs:validate"]),
        ("for component in spans.keys()", "0",
         O10 + "E2 (route E): `spans` is filled only inside the records loop (E1), so it is empty", ["connector_evidence.rs:validate"]),
        ("for row in rows.values()", "Pall",
         O10 + "E3 (route E): one pass over the validated envelope's result rows (physics_evidence `rows`, indexed by id), "
         "<= C·P_final = Pall", ["connector_evidence.rs:validate"]),
    ]
    if fnkey("row_basis", CE):
        O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("validate", CE), fnkey("row_basis", CE),
        O10 + "E4 (route E): row_basis is called only for a connector-kind row after `require_connector(admitted, \"UNSUPPORTED\")?`, "
        "which returns Err when !admitted (E1: every reached caller passes StraightV2), ending validate")]
    O10_SITE_FROM = O10_SITE_FROM + [(site("require_connector", CE, 'Err(format!("SOURCE_PHYSICS_CONNECTOR_{code}"))'),
        {"from": [fnkey("validate_physics_evidence", RE_ + "physics_evidence.rs")], "per": "1",
         "why": O10 + "E5 (route E, as physics_evidence.rs require()): require_connector allocates only when it returns Err; all 16 "
         "calls in connector_evidence.rs are `require_connector(..)?` and connector_evidence::validate's caller "
         "(validate_physics_evidence_in) propagates with `?`: <= 1 per validate_physics_evidence call"})]
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("require_connector", CE, 'Err(format!("SOURCE_PHYSICS_CONNECTOR_{code}"))', full=True), {"code": ST}),
    ]
    if fnkey("row_basis", CE):
        O10_ID_AUDIT += [(site("row_basis", CE, "format!(\"objective_connector_v1;replaces_span={span}", full=True), {"span": IN})]
if fnkey("fmt", FK_ + "connector.rs"):
    O10_ID_AUDIT = O10_ID_AUDIT + [(site("fmt", FK_ + "connector.rs", t_, full=True), {"name": ST}) for t_ in (
        'write!(f, "connector input {name} is not finite")',
        'write!(f, "connector triad {name} is not proper orthonormal")',
        'Self::Range { name } => write!(f, "connector {name} outside the binary64 normal range")')]
if fnkey("classify_connectors", PP_ + "joint.rs"):
    _J = PP_ + "joint.rs"
    _JS = [site(fn_, _J, t_, row="format!(") for fn_, t_ in (("refuse_legacy_joints", '"{what}; it is neither solved nor converted'),
                        ("decode_fields", "consumption.unwrap_or_default()"),
                        ("model_findings", 'topology.and_then(|t| text(t, "type")).unwrap_or("(none)")'),
                        ("model_findings", "{problem}; the connector is not assembled"),
                        ("classify_connectors", "extra.unwrap_or_default()"),
                        ("classify_connectors", "is incomplete or invalid: {problems}"))]
    _JW = (O10 + "J (multiplicity 0 on the D1 graph): every path to this site iterates model.components (registered rule "
           "`model.components` -> 0). " + D14 + ". Its size is set to 0 (its composed arguments are never priced); no site_zero, "
           "so the chain still computes the site's true multiplicity and o10_check.py fails the run if it is ever positive")
    O10_SITE_SIZE = [(k, 0, _JW) for k in _JS]
    # The two sites also carry an identifier placeholder (the connector's id), so the chain evaluates their format even
    # with a site size; their composed arguments get audited bounds (unused at multiplicity 0; kept honest regardless).
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("model_findings", _J, "{problem}; the connector is not assembled", full=True, row="format!("),
         {"problem": {"bytes": 512, "source": "O10: one topology finding: a literal <= 60 B with <= 2 model ids (<= 128 B each, D1 text cap) or a static literal"}}),
        (site("classify_connectors", _J, "is incomplete or invalid: {problems}", full=True, row="format!("),
         {"problems": {"bytes": 8222, "source": "O10: decode_fields' problems joined by '; ': <= 16 findings (9 pushes and the attachment/axes/q_ref/scale results), each <= 512 B: 16*512 + 15*2"}}),
    ]

# ---- O-10, T4 batch 2's final head (e9cc7e3f5f; HELP_HUMAN's rulings 3 and 5): classifications. Each new site is kept
# priced (no zero); only its identifier and static arguments are classed. Arguments in DEL-05-02 Design/retained-pricing-rules.md, section C.
if fnkey("append_retained_source_not_joined", PP_ + "lib.rs"):
    _L = PP_ + "lib.rs"
    def _const_len(f, name):
        src = open(_os.path.join(_SNAP, f), encoding="utf-8").read()
        m = _re.search(r"const\s+" + _re.escape(name) + r'\s*:\s*&str\s*=\s*"((?:[^"\\]|\\.)*)"\s*;', src)
        if not m:
            raise SystemExit(f"o10: const {name} not found in {f}")
        return len(m.group(1).encode())
    _NJ = _const_len(PP_ + "exact_admission.rs", "RETAINED_SOURCE_NOT_JOINED_TEXT")
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("append_retained_source_not_joined", _L, '"diagnostic:source-recovery:{}:not-joined", stable_suffix(case_id)', full=True),
         {"stable_suffix(case_id)": IN}),
        (site("append_retained_source_not_joined", _L, "vec![case_id.to_string()]", full=True), {"case_id": IN}),
    ]
if fnkey("append_short_bend_refusal", PP_ + "lib.rs"):
    _L = PP_ + "lib.rs"
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("append_short_bend_refusal", _L, '"diagnostic:curved-bend-short:{}", stable_suffix(case_id)', full=True),
         {"stable_suffix(case_id)": IN}),
        (site("append_short_bend_refusal", _L, '"Load case {case_id}: realized bend {} (pipe {})', full=True, row="format!("),
         {"case_id": IN, "bend.component_id": IN, "bend.pipe_id": IN}),
        (site("append_short_bend_refusal", _L, "vec![case_id.to_string(), bend.component_id.clone(), bend.pipe_id.clone()]", full=True),
         {"case_id": IN, "bend.component_id": IN, "bend.pipe_id": IN}),
    ]
_SB = "core/product_physics/src/s11g_tests/short_bend.rs"
if _os.path.exists(_os.path.join(_SNAP, _SB)):
    # T (test code): `src/s11g_tests/short_bend.rs` is a module of `s11g_tests`, which lib.rs declares only under
    # `#[cfg(test)]`; the chain reaches none of its functions (multiplicity 0). Its label arguments get honest classes.
    _SBW = "O10 T (test code, unreached): "
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("a_pivot_screen_refusal_names_a_bend_short_against_its_neighbours", _SB, 'let label = format!("{entry:?} {mode:?}");', full=True),
         {"mode|?": {"bytes": 17, "source": _SBW + "PreviewSolverMode Debug: SparseInteractive (17) or DenseScrutiny (13)"}}),
        (site("a_passed_or_sensitive_bend_carries_no_short_bend_diagnostic", _SB, 'let label = format!("{phi} {entry:?} {mode:?}");', full=True),
         {"mode|?": {"bytes": 17, "source": _SBW + "PreviewSolverMode Debug: SparseInteractive (17) or DenseScrutiny (13)"},
          "phi": {"bytes": 327, "source": _SBW + "an f64 in Display (the chain's f64_display class, 327)"}}),
    ]
if fnkey("append_retained_source_not_joined", PP_ + "lib.rs"):
    O10_ARGS_FIRST = [{"re": r"^exact_admission::RETAINED_SOURCE_NOT_JOINED_TEXT$", "max": _NJ,
        "why": O10 + "N (diag message): the &'static str const RETAINED_SOURCE_NOT_JOINED_TEXT, its literal length read from exact_admission.rs at the snapshot"}]


# ---- U2/O-13 at 98dba87714 (Design retained-pricing-rules.md §E).
# Header limits are true upper bounds even for a route conservatively reaching
# an arc helper. No new edge or site is declared zero.
if fnkey("add_curved_bend_pressure_equivalent_load", PP_ + "lib.rs"):
    O10_LOOPS_LAST += [
        ("for (pipe_index, state) in &exact.pipe_states", "m", O10 + "U2L1: pipe_states has at most one entry per built pipe, hence <= m", ["lib.rs:add_curved_bend_pressure_equivalent_load"]),
        ("for (local_row, &global_row) in dof_map.iter().enumerate()", "12", O10 + "U2L2: element_dof_map returns [usize; ELEMENT_DOF], ELEMENT_DOF = 12", ["lib.rs:add_curved_bend_thermal_equivalent_load", "lib.rs:add_curved_bend_pressure_equivalent_load"]),
        ("for (local_col, &free_value) in free_expansion.iter().enumerate()", "12", O10 + "U2L3: curved_bend_free_expansion_displacements returns [f64; ELEMENT_DOF], ELEMENT_DOF = 12", ["lib.rs:add_curved_bend_thermal_equivalent_load", "lib.rs:add_curved_bend_pressure_equivalent_load"]),
    ]
    O10_LOOPS_LAST += [("for junction in &junctions", "m", O10 + "U2E2: checked traversal's junction list has one entry per adjacent member pair, <= traversal length <= m", ["pressure_runtime.rs:build_pressure_case_with_members"])]
    O10_LOOPS_LAST += [("for (kind, component, stations) in crate::arc_evidence::ROWS", "4", O10 + "U2E1: arc_evidence::ROWS is a fixed array of four row families", ["physics_evidence.rs:validate_physics_evidence_in"])]
    _LL = FK_ + "load_ledger.rs"
    O10_ID_AUDIT += [(site("push_product", _LL, "source: source.into()", full=True), {"source": IN})]
    _BP = PP_ + "bend_pressure.rs"
    if fnkey("append_arc_pressure_results", _BP):
        for _text, _values in [
            ("vec![case.id.clone(), pipe_id.to_string()]", {"case.id": IN, "pipe_id": IN}),
            ('"result:pressure-exact:{}:{}:{}:{}:{}:{}"', {"case.id.len()": NUM_, "case.id": IN, "pipe_id.len()": NUM_, "pipe_id": IN,
                "location": ST, "component": ST}),
            ("kind: kind.to_string()", {"kind": ST}),
            ("entity_ref: pipe_id.to_string()", {"pipe_id": IN}),
            ("ref_id: case.id.clone()", {"case.id": IN}),
            ("component: component.to_string()", {"component": ST}),
            ("location: location.to_string()", {"location": ST}),
        ]:
            O10_ID_AUDIT += [(site("append_arc_pressure_results", _BP, _text, full=True, row="format!(" if _text.startswith('"result:') else None), _values)]
    _L = PP_ + "lib.rs"
    O10_ID_AUDIT += [
        (site("solve_load_case_observed", _L, 'last.id = format!("diagnostic:exact-stress:', full=True), {"stable_suffix(&load_case.id)": IN, "stable_suffix(&pipe.element_id)": IN}),
    ]
    if fnkey("arc_recovery_finding", PP_ + "formation_guard.rs"):
        O10_ID_AUDIT += [(site("solve_load_case_observed", _L, "project: model.project.id.clone()", full=True), {"model.project.id": IN})]
    if fnkey("arc_recovery_record", _L):
        O10_ID_AUDIT += [(site("arc_recovery_record", _L, "member: pipe.element_id.clone()", full=True), {"pipe.element_id": IN})]
    # U2C: keep this conservative callgraph site's computed multiplicity.
    # The authored string is D1-capped; the static default also fits IN128.
    O10_ID_AUDIT += [(site("component_solver_consumption", _L, ".to_string()", full=True), {
        "terface.as_ref().and_then(|interface|interface.solver_consumption.as_deref()).unwrap_or(default)": IN,
    })]
    O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [{"re": r"^pressure_treatment$", "max": 128,
        "why": O10 + "U2A: append_curved_bend_macro_element_results selects one of two static ASCII literals, each <128 B"}]

if fnkey("arc_recovery_finding", PP_ + "formation_guard.rs"):
    _FG = PP_ + "formation_guard.rs"
    O10_LOOPS_LAST += [("for (slot, group) in ArcRowGroup::ALL.into_iter().enumerate()", "4", O10 + "O13L1: ArcRowGroup::ALL is [Self;4]; includes all four groups", ["formation_guard.rs:arc_recovery_finding", "s11g_tests/arc_recovery.rs:p2_realistic_arcs_are_not_demoted"])]
    O10_ID_AUDIT += [
        (site("arc_recovery_finding", _FG, '"{}: the formation bound is unavailable', full=True, row="format!("), {"record.member": IN}),
        (site("arc_recovery_finding", _FG, '"S11-G recovery guard', full=True, row="format!("), {"case_id": IN}),
        (site("arc_recovery_finding", _FG, '"{}.{name}.{}:', full=True, row="format!("), {"record.member": IN, "group.label()": ST, "name": ST, "unit": ST}),
    ]
    # Six named items; each unavailable item <= id+41+8192. 600 covers
    # each guard's literal, then id, item count and the "and more" suffix.
    _SENTENCE = 600 + 128 + 20 + 6 * (128 + 41 + 8192) + 10 + 31
    if fnkey("short_bend_finding", _FG):
        O10_ID_AUDIT += [(site("short_bend_finding", _FG, '"Short-bend rule:', full=True, row="format!("), {"case_id": IN})]
    if fnkey("merge", _FG):
        O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [{"re": r"^merged\.sentence$", "max": 3 * _SENTENCE + 2,
            "why": O10 + "O13A1: merge receives at most straight, arc and short-bend findings; each <= S, joined with two spaces; prefix <= whole"}]
    _AR = PP_ + "s11g_tests/arc_recovery.rs"
    if _os.path.exists(_os.path.join(_SNAP, _AR)):
        # All test-only; honest classes, never a guessed zero size.
        for _fn, _text, _values in [
            ("record", "member: member.to_string()", {"member": IN}),
            ("g5_merge_keeps_a_single_sentence_and_orders_several", 'format!("{} {} {}"', {"straight.sentence": {"bytes": _SENTENCE, "source": "O13A1"}, "arc.sentence": {"bytes": _SENTENCE, "source": "O13A1"}, "short.sentence": {"bytes": _SENTENCE, "source": "O13A1"}}),
            ("p2_realistic_arcs_are_not_demoted", 'let tag = format!("{} {entry:?} {mode:?}"', {"project_of(request)": IN}),
            ("p5_an_already_sensitive_case_is_left_untouched", 'let tag = format!("P5 {} {entry:?} {mode:?}"', {"project_of(request)": IN}),
        ]:
            if fnkey(_fn, _AR):
                O10_ID_AUDIT += [(site(_fn, _AR, _text, full=True), _values)]
        _SRC_BYTES = sum(_os.path.getsize(_os.path.join(_SNAP, PP_ + f)) for f in ("lib.rs", "formation_guard.rs"))
        O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [
            {"re": r"^mode$", "max": 17, "why": O10 + "O13T: PreviewSolverMode Debug is SparseInteractive (17) or DenseScrutiny (13)"},
            {"re": r"^phi$|^factor$", "max": 327, "why": O10 + "O13T: f64 Display maximum is 327"},
            {"re": r"^line$", "max": _SRC_BYTES, "why": O10 + "O13T: product_text copies a line of its include_str source; one line <= combined source bytes, measured at snapshot"},
        ]


# KD5L1: current unified RecoveryRecord includes supports. Rebind the
# surviving earlier header conservatively; an old snapshot does not change.
if fnkey("member_group_fires", PP_ + "formation_guard.rs"):
    REBIND = list(globals().get("REBIND", [])) + [("record in records", "m+g",
        "KD5L1: RecoveryRecord has one record per admitted straight member and support; arcs/connectors excluded by D1.4/8")]
    O10_LOOPS_LAST += [("for group in &record.groups", "28", O10 + "KD5L2: each member has at most seven locations times four groups; a support has two <=28", ["formation_guard.rs:member_scales", "formation_guard.rs:recovery_finding"])]


# KD5A1: current certified publication's input ids and fixed owned labels.
_CP = PP_ + "certified_publication.rs"
if fnkey("member", _CP):
    O10_ID_AUDIT += [(site("member", _CP, 'let prefix = format!("member:', full=True), {"pipe.element_id.len()": NUM_, "pipe.element_id": IN})]
    for _fn, _text, _values in [
        ("group", "location: location.into()", {"location": ST, "kind": ST}),
        ("member_record", "member: member.into()", {"member": IN}),
        ("support_record", "member: support.into()", {"support": IN}),
        ("unavailable_record", "member: entity.into()", {"entity": IN}),
        ("connector_record", "member: entity.into()", {"entity": IN}),
    ]:
        if fnkey(_fn, _CP):
            O10_ID_AUDIT += [(site(_fn, _CP, _text, full=True), _values)]
