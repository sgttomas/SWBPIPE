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
        # U2A2 (batch 4, review M-2): the function now holds two blocking
        # diagnostics with the same affected-ref line, the unknown-location
        # refusal and the unrepresentable-section diagnostic. Each is keyed
        # by its own unique preceding line and checked; both stay priced.
        _bp_lines = open(_os.path.join(_SNAP, _BP), encoding="utf-8").read().split("\n")
        for _before in ['format!("arc section location {location:?} is not one of', "RECOVERY_UNREPRESENTABLE,"]:
            _prev = site("append_arc_pressure_results", _BP, _before, full=True)
            _prev_file, _prev_line = _prev.rsplit(":", 1)
            if _bp_lines[int(_prev_line)].strip() != "vec![case.id.clone(), pipe_id.to_string()],":
                raise SystemExit("U2A2: arc pressure affected-ref site changed; re-review")
            O10_ID_AUDIT += [(Resolved(f"{_prev_file}:{int(_prev_line) + 1}"), {"case.id": IN, "pipe_id": IN})]
        # U2L4 (batch 4, review I-1/M-2): the per-section recovery loop runs
        # over the two ends plus the caller's station rows, which every
        # solve_load_case_observed branch forms as at most three (quarter_1,
        # midspan, quarter_3; KD5L7, curved_bend_station_resultants -> [_; 3]).
        O10_LOOPS_LAST += [("for (location, elastic) in sections", "5", O10 + "U2L4: sections = [end_i, end_j] + stations, stations <= 3", ["bend_pressure.rs:append_arc_pressure_results"])]
        # U2A3: the unknown-location refusal's {location:?} is a &'static str
        # of `sections`: the literals end_i/end_j or StationResultants.location
        # (&'static str; quarter_1/midspan/quarter_3). STATIC; the site stays priced.
        O10_ID_AUDIT += [(site("append_arc_pressure_results", _BP, 'format!("arc section location {location:?} is not one of', full=True),
                          {"location|?": ST})]
        for _text, _values in [
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

# O13T2 (T4 batch 4): O-13's arc predicate is removed (the unified K-D5
# predicate decides), so the block above is inert, but `s11g_tests/arc_recovery.rs`
# remains, rewritten. It is still a module of the cfg(test) `s11g_tests`; the
# chain reaches some of its helpers only through conservative name candidates.
# The same honest test-only classes as O13T; never a guessed zero size.
_AR = PP_ + "s11g_tests/arc_recovery.rs"
if not fnkey("arc_recovery_finding", PP_ + "formation_guard.rs") and _os.path.exists(_os.path.join(_SNAP, _AR)):
    for _fn, _text, _values in [
        # Literal kinds only: "axial", "shear", "torsion", "bending" (g2 iterates the same four).
        ("group", "kind: kind.into()", {"kind": ST}),
        # Test project ids: the fixed invented:o13:* ids, the longest 42 B with phi's Display; <= IN128.
        ("p2_realistic_arcs_are_not_demoted", 'let tag = format!("{} {entry:?} {mode:?}"', {"project_of(request)": IN}),
        ("p5_an_already_sensitive_case_is_left_untouched", 'let tag = format!("P5 {} {entry:?} {mode:?}"', {"project_of(request)": IN}),
    ]:
        if fnkey(_fn, _AR):
            O10_ID_AUDIT += [(site(_fn, _AR, _text, full=True), _values)]
    _SRC_BYTES = sum(_os.path.getsize(_os.path.join(_SNAP, PP_ + f)) for f in ("lib.rs", "formation_guard.rs"))
    O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [
        {"re": r"^mode$", "max": 17, "why": O10 + "O13T2: PreviewSolverMode Debug is SparseInteractive (17) or DenseScrutiny (13)"},
        {"re": r"^phi$", "max": 327, "why": O10 + "O13T2: f64 Display maximum is 327"},
        {"re": r"^line$", "max": _SRC_BYTES, "why": O10 + "O13T2: product_text copies a line of its include_str source (lib.rs, formation_guard.rs); one line <= combined source bytes, measured at snapshot"},
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
        ("group", "location: location.into()", {"location": ST}),
        ("group", "kind: kind.into()", {"kind": ST}),
        ("member_record", "member: member.into()", {"member": IN}),
        ("support_record", "member: support.into()", {"support": IN}),
        ("unavailable_record", "member: entity.into()", {"entity": IN}),
        ("connector_record", "member: entity.into()", {"entity": IN}),
    ]:
        if fnkey(_fn, _CP):
            O10_ID_AUDIT += [(site(_fn, _CP, _text, full=True), _values)]


# KD5C1: the public condition-estimator wrapper retains its trait-method
# edge. Move the basis rule by source name instead of pruning the renamed
# still-live method (estimate_rcond -> estimate_condition).
def _fn_n(name, f, ordinal, count):
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().splitlines()
    hits = [i + 1 for i, line in enumerate(lines) if _re.match(r"\s*(pub(\([a-z]+\))?\s+)?(const\s+)?fn\s+" + _re.escape(name) + r"\b", line)]
    if len(hits) != count:
        raise SystemExit(f"KD5C1: {f} fn {name}: expected {count} definitions, found {len(hits)}")
    return Resolved(f"{f}:{hits[ordinal - 1]}:{name}")
_ST = FK_ + "structural.rs"
if fnkey("estimate_condition", _ST):
    _caller, _target, _name = _fn_n("estimate_rcond", _ST, 1, 1), _fn_n("estimate_condition", _ST, 1, 1), "estimate_condition"
else:
    _caller, _target, _name = _fn_n("estimate_rcond", _ST, 1, 2), _fn_n("estimate_rcond", _ST, 2, 2), "estimate_rcond"
CG_RULES = list(globals().get("CG_RULES", [])) + [{"caller": _caller, "name": _name, "targets": [_target],
    "added": "KD5C1", "evidence": "the public estimate_rcond delegates to ConditionMatrix's method; exact named definitions, no call bound removed"}]

# KD5C2: four source-changed audit keys move out of the basis line table.
# Re-add the original site/argument class on historical code and the actual
# replacement site on unified code. The retired record copy is superseded
# by certified_publication's record constructors (KD5A1).
_FG = PP_ + "formation_guard.rs"
if fnkey("member_group_fires", _FG):
    for _text in ['"{}.{}.{}: q={:e}', '"{}.{}.{}: the member bound is unavailable']:
        O10_ID_AUDIT += [(site("recovery_finding", _FG, _text, full=True, row="format!("), {"record.member": IN, "group.location": ST, "group.kind": ST})]
else:
    for _text in ['"{}.{end}: q={q:e}', '"{}.{end}: the formation bound is unavailable']:
        O10_ID_AUDIT += [(site("recovery_finding", _FG, _text, full=True, row="format!("), {"record.member": IN})]
if not fnkey("member_record", _CP):
    O10_ID_AUDIT += [(site("solve_load_case_observed", PP_ + "lib.rs", "member: pipe.element_id.clone()", full=True), {"pipe.element_id": IN})]
O10_ID_AUDIT += [(site("check", FK_ + "structural/formation_check.rs", "family.clone()", full=True), {"family": ST})]


if fnkey("member_group_fires", _FG):
    O10_LOOPS_LAST += [
        ("for p in &source.loads", "0", O10 + "KD5L3: D1/W1 excludes load-state; source_factor returns None unless is_load_state; nodal uses push input and no load primitive", ["structural/formation_check.rs:load_closure"]),
        ("for (i, (term, record)) in terms.iter().zip(records).enumerate()", "l", O10 + "KD5L4: exactly one trusted closure record per nodal ledger term, <= l", ["structural/formation_check.rs:load_closure"]),
        ("for (j, expected) in manifest.iter().enumerate()", "0", O10 + "KD5L5: every D1 nodal record is input (formed=false); no manifest is installed; loop only after formed guard", ["structural/formation_check.rs:load_closure"]),
        ('for (location, rows) in locations.into_iter().chain( [" ", " ", " ", " ", " "] .iter() .zip(effective.sections.iter()) .map(|(l, r)| (*l, r.as_slice())), )', "7", O10 + "KD5L6: two end rows plus five fixed CertifiedMemberRecovery sections", ["certified_publication.rs:member_record"]),
        ("iter:station_resultants.iter().enumerate().map", "3", O10 + "KD5L7: quarter_1, midspan and quarter_3 station rows, exactly three", ["lib.rs:solve_load_case_observed"]),
        ("iter:diagnostics.iter().position", "D", O10 + "KD5L8: one scan bounded by the case's diagnostic Vec, <= the full diagnostic fixpoint D", ["lib.rs:solve_load_case_observed"]),
        ("for rows in &member.sections", "5", O10 + "KD5L9: CertifiedMemberRecovery sections is [..;5]", ["lib.rs:solve_load_case_observed"]),
        ("iter:exact.closure_sources[group_index].iter().enumerate().map", "0", O10 + "KD5L10: D1.5 requires absent/empty pressure regions, so no assembled pressure group or closure_source summand", ["lib.rs:push_exact_pressure_operands"]),
        ("for (i, term) in ledger.terms().iter().enumerate()", "l", O10 + "KD5L11: one input term per admitted nodal primitive, <= l", ["structural/formation_check.rs:evaluate"]),
        ("for p in source.loads.iter().filter(|p| p.term_index == i)", "0", O10 + "KD5L12: supplied load primitive list is empty by KD5L3", ["structural/formation_check.rs:evaluate"]),
    ]
    O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [{"re": r"^route$", "max": 128, "why": O10 + "KD5A3: unavailable-route sentence selects 'on this route' or 'on the nonlinear-support route', static <=128"}]
    for _text in ['format!("diagnostic:stress-free-within-numerical-precision:', 'format!("Load case {case_id}: every member', 'vec![case_id.to_string()]']:
        O10_ID_AUDIT += [(site("stress_free_notice", _FG, _text, full=True), {"case_id": IN})]
    _CL = PP_ + "certified_loads.rs"
    O10_ID_AUDIT += [(site("summand", _CL, "family: family.into()", full=True), {"family": ST}),
        (site("summand", _CL, "source: source.into()", full=True), {"source": CP}),
        (site("solve_load_case_observed", PP_ + "lib.rs", "record.certified_support_norms.push", full=True), {"support.id": IN}),
        # KD5A3 (batch 4 re-key): the pressure closure label moved, unchanged,
        # from finish_source_groups' map closure into its callee closure_source.
        (site("closure_source" if fnkey("closure_source", PP_ + "pressure_runtime.rs") else "finish_source_groups",
              PP_ + "pressure_runtime.rs", 'format!("{}:{}:{}", term.1, term.2, term.3)', full=True), {"term.1": IN, "term.2": IN, "term.3": ST}),
    ]
    _LL = FK_ + "load_ledger.rs"
    CG_RULES += [{"caller": Resolved(_LL + ":" + fnkey("force_scaled", _LL).split(":",1)[1]), "name": "force_scaled",
        "targets": [Resolved(FK_ + "structural/formation_check.rs:" + fnkey("force_scaled", FK_ + "structural/formation_check.rs").split(":",1)[1])],
        "added": "KD5C3", "evidence": "AssembledForce::force_scaled maps its Vec<LoadPrimitive> with p.force_scaled; this is LoadPrimitive's method, never recursive AssembledForce::force_scaled"}]
else:
    for _text, _values in [("map.insert(pipe.element_id.clone()", {"pipe.element_id": IN}), ("map.insert(node.id.clone()", {"node.id": IN}), ("map.insert(support.id.clone()", {"support.id": IN})]:
        O10_ID_AUDIT += [(site("formation_entity_bodies", PP_ + "lib.rs", _text, full=True), _values)]

if fnkey("member_group_fires", _FG):
    for _text in ['format!("{}: absent manifest", term.source)', '"{}: missing summands"', '"{}: missing, duplicate or extra summand"']:
        O10_ID_AUDIT += [(site("load_closure", FK_ + "structural/formation_check.rs", _text, full=True, row="format!("), {"term.source": IN})]

if fnkey("member_group_fires", _FG):
    O10_LOOPS_LAST += [("iter:group.terms.iter().map", "0", O10 + "KD5L13: finish_source_groups receives pressure factor groups from region traversal; D1.5 has no pressure region, hence no group term", ["pressure_runtime.rs:finish_source_groups"])]


# KD5T2: actual exhaustive fixed-label/text bounds, source-pinned inventory.
if fnkey("member_group_fires", _FG):
    import runpy as _pricing_runpy
    _labels_model = _pricing_runpy.run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "kd5_text_model.py"))
    _fixed = _labels_model["inspect"](_SNAP)
    _label = {"bytes": _fixed["label_bytes"], "source": "KD5T2: exhaustive source-pinned constructor label inventory"}
    for _producer in ("group", "magnitude_group"):
        O10_ID_AUDIT += [(site(_producer, _CP, "location: location.into()", full=True), {"location": _label}),
            (site(_producer, _CP, "kind: kind.into()", full=True), {"kind": _label})]
    for _text in ['"{}.{}.{}: q={:e}', '"{}.{}.{}: the member bound is unavailable', '"{}.{}.{}: member comparison unavailable']:
        O10_ID_AUDIT += [(site("recovery_finding", _FG, _text, full=True, row="format!("), {"group.location": _label, "group.kind": _label})]
    O10_ID_AUDIT += [(site("recovery_finding", _FG, '"{}.{}.{}: member comparison unavailable', full=True, row="format!("),
        {"reason": {"bytes": _fixed["comparison_reason_bytes"], "source": "KD5T2: pinned member_group_decision fixed reason inventory"}})]
    O10_ARGS_FIRST = [{"re": r"^route$", "max": _fixed["route_bytes"], "why": "KD5T2: exhaustive source-pinned static route strings"}] + O10_ARGS_FIRST


# KD5A5: these `s` bindings are text, not the inherited f64_display s.
if fnkey("member_group_fires", _FG):
    _FC = FK_ + "structural/formation_check.rs"
    for _text, _bytes, _why in [
        ('Self::Shape(s) => format!("formation source: {s}")', 128, "private Failure::Shape holds a fixed source/shape literal <=128 B"),
        ('Self::Closure(s) => format!("load closure: {s}")', 8352, "max(D1 load id128+ordinary suffix40,load id128+reformation prefix14+error Debug8192) <=8352"),
        ('Self::Second(s) => format!("second solve: {s}")', 8213, "inner subtract/correction failure detail <= existing error8192 +longest inner prefix21; no nested Second from those helpers"),
    ]:
        O10_ID_AUDIT += [(site("detail", _FC, _text, full=True), {"s": {"bytes": _bytes, "source": "KD5A5: " + _why}})]

# KD5A6: Failure::Unavailable's &str converts into an owned String.
if fnkey("member_group_fires", _FG):
    O10_ID_AUDIT += [(site("detail", FK_ + "structural/formation_check.rs", "Self::Unavailable(s) => (*s).into()", full=True),
                     {"(*s)": {"bytes": 128, "source": "KD5A6: private Failure::Unavailable uses contraction/Neumann/non-finite Lambda sigma literals, all <=128 B; pinned evaluate body"}})]

# KD5M1/M2: positive per-invocation caps. Each alternative is still charged.
if fnkey("member_group_fires", _FG):
    import hashlib as _pricing_hashlib
    _FC = FK_ + "structural/formation_check.rs"
    _invocation_pins = {
        "detail": "ae006b02fed244b2e77686378bdb17b87c442a1d4873add6f4b2129d3be1bc0f",
        "check": "d957e111325987e36b41a68b522de7b2dd1cd671493b2709f1e7ee3a4b1acd4d",
        "evaluate": "4233fd8c95e578130216b2125cb2aa1d506fac5e114d556ec6c3a1ca01b141d4",
        "load_closure": "57481954c41488f9490f91070b4094e33fee8e12dad1acf61a05e686b9eb7275",
    }
    _fc_lines = open(_os.path.join(_SNAP, _FC), encoding="utf-8").read().splitlines(keepends=True)
    _invocation_bodies = {}
    for _name, _digest in _invocation_pins.items():
        _start = int(fnkey(_name, _FC).split(":")[1]) - 1
        _indent = len(_fc_lines[_start]) - len(_fc_lines[_start].lstrip())
        _end = next(i for i in range(_start + 1, len(_fc_lines)) if _fc_lines[i].rstrip() == " " * _indent + "}")
        _body = "".join(_fc_lines[_start:_end + 1]).rstrip("\n")
        if _pricing_hashlib.sha256(_body.encode()).hexdigest() != _digest:
            raise SystemExit(f"KD5M1/M2: source pin changed: {_FC}:{_name}; re-review per-invocation control flow")
        _invocation_bodies[_name] = (_start, _end)
    for _name, _from, _per, _expected, _why in [
        ("detail", "check", "2", 7, "KD5M1: at most one inner Second detail and one outer detail per check/evaluate invocation; first-error returns prevent another inner detail; charge each exclusive allocation alternative separately"),
        ("load_closure", "load_closure", "1", 15, "KD5M2: every error allocation returns Err or propagates with ?, so at most one allocation across all closure errors per invocation; charge each exclusive alternative separately"),
    ]:
        _start, _end = _invocation_bodies[_name]
        _sites = [Resolved(_FC.split("/src/")[-1] + f":{i + 1}") for i in range(_start, _end + 1)
                  if "format!(" in _fc_lines[i] or ".into()" in _fc_lines[i]]
        if len(_sites) != _expected:
            raise SystemExit(f"KD5M1/M2: {_name} allocation inventory changed: {_sites}")
        O10_SITE_FROM += [(_site, {"from": [fnkey(_from, _FC)], "per": _per, "why": O10 + _why + "; preserves the full reached ordinary/W2/case/replay source-function multiplier"}) for _site in _sites]

# KD5T3: positive cap only at the closed private StressFinding subject copy.
if fnkey("member_group_fires", _FG):
    _stress_subject = _labels_model["inspect_stress_subjects"](_SNAP)
    LEX_SITE_SIZE += [(Resolved("stress_recovery/src/lib.rs:" + str(_stress_subject["line"])),
        {"class": 128, "why": "KD5T3: production source-pinned private StressFinding::new/helper call tree has17 fixed ASCII subjects, max22UTF8 B; retain existing audit128 and full multiplicity; no type-wide subject or message narrowing"})]

# KD5C4: FallbackError is private/non-escaping; these receivers are the
# named callers' primitive numbers, not that new private method owner.
if fnkey("fallback_circular_maximum", _CP):
    _primitive_minmax_callers = {
        # Batch 4 renamed moment_scales to action_scales (O-13's coupled
        # ActionScales); its min/max receivers are still the f64 scales.
        "formation_guard.rs": ["new", "decide_row", "row_scales", "member_scales",
            "action_scales" if fnkey("action_scales", PP_ + "formation_guard.rs") else "moment_scales", "stress_free_notice"],
        "lib.rs": ["observation_lane_profile", "build_curved_bend_macro_elements", "straight_summary_extrema"],
        "pressure_exact.rs": ["fused_mul_add"], "pressure_runtime.rs": ["finish_source_groups"],
        "retained_memory.rs": ["string", "material", "walk", "combination_census", "longest_string", "complete_observations"],
        "retained_product.rs": ["g5a"], "source_recovery.rs": ["solve"],
        "source_receipt/composite.rs": ["norm"],
        "source_receipt/endpoint_maximum.rs": ["absolute", "endpoint_maximum", "directed"],
        "source_receipt/rows.rs": ["derived_for"],
        "case_state/thermal.rs": ["integral", "dilation_difference", "check_path"],
        "certified_publication.rs": ["fallback_denominator_relative_error"],
    }
    for _file, _names in _primitive_minmax_callers.items():
        for _name in _names:
            _file = PP_ + _file if not _file.startswith(PP_) else _file
            _key = (_fn_n("string", _file, 2, 2) if _file == PP_ + "retained_memory.rs" and _name == "string"
                    else fnkey(_name, _file))
            if not _key:
                raise SystemExit(f"KD5C4: primitive caller is not unique: {_file}:{_name}")
            for _method in ("min", "max"):
                _keep = []
                if _method == "max" and (_file, _name) in [(PP_ + "retained_memory.rs", "longest_string"), (PP_ + "source_recovery.rs", "solve")]:
                    # Preserve the old heuristic's non-TEXT max targets;
                    # this correction removes only the new private candidate.
                    _keep = [_fn_n("max", PP_ + "retained_memory.rs", 1, 2),
                             _fn_n("max", PP_ + "retained_memory.rs", 2, 2),
                             _fn_n("max", FK_ + "structural/retained/product_certificate.rs", 1, 1)]
                CG_RULES += [{"caller": Resolved(_file + ":" + _key.split(":", 1)[1]), "name": _method, "targets": _keep,
                    "added": "KD5C4", "evidence": "named receiver is usize/f64/i32 intrinsic; private FallbackError has no outward producer or escape, so it cannot be this receiver; real CP fallback_section_errors min/max preserved"}]

# KD5C5: subtract only witnessed private FallbackError candidates from
# approved outside callers, retaining every other conservative candidate.
if fnkey("fallback_circular_maximum", _CP):
    for _rule in _labels_model["inspect_fallback_boundary"](_SNAP):
        CG_RULES += [{"caller": Resolved(_rule["caller"]), "name": _rule["name"],
            "exclude_targets": [Resolved(_target) for _target in _rule["exclude_targets"]],
            "added": "KD5C5", "evidence": "current private FallbackError has no trait/public-return/function-pointer/macro escape; pinned outside receivers are Scaled/Bytes/ExactAccumulator/CertifiedRow/CertifiedValue; subtract only this qualified candidate and preserve all other and inside-CP candidates"}]

# KD5C6: returning allocation alternatives only; no visit or graph cap.
import runpy as _c6_runpy
_c6_model = _c6_runpy.run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "kd5_first_error_model.py"))
_c6_state = _c6_model["inspect"](_SNAP)
if _c6_state is not None:
    FIRST_ERROR_GUARD = "KD5C6"
    O10_SITE_FROM += [(Resolved(_key), dict(_entry, **{"from": [Resolved(_root) for _root in _entry["from"]]}))
                     for _key, _entry in _c6_model["site_rules"](_c6_state)]

# KD5L14-L18: actual fixed arrays, conservative span census, and returning
# power-transport error. No function or run multiplier is globally capped.
if fnkey("fallback_circular_maximum", _CP):
    O10_LOOPS_LAST += [
        ("for span in spans", "2*l+1", O10 + "KD5L14: current D1 has no uniform primitive, hence one interval; retain conservative positive2l+1 span allowance from KD5O10", ["certified_publication.rs:fallback_circular_maximum"]),
        ("iter:spans .into_iter().map", "2*l+1", O10 + "KD5L14: FK returned certificate spans have the same current-D1 one-interval producer; positive2l+1 allowance", ["certified_publication.rs:straight_stress_spans"]),
        ("for component in powers", "3", O10 + "KD5L15: three stress component coefficient arrays", ["certified_publication.rs:fallback_circular_maximum"]),
        ("for (station, rows) in member.sections.iter().enumerate()", "5", O10 + "KD5L15: CertifiedMemberRecovery sections is exactly5", ["certified_publication.rs:maximum_cover"]),
        ("for (target, &value) in axial.iter_mut().zip(&span.axial)", "3", O10 + "KD5L15: degree2 axial controls have3 coefficients", ["certified_publication.rs:fallback_circular_maximum"]),
        ("for (target, source) in axial.iter_mut().zip(span.axial)", "3", O10 + "KD5L15: degree2 axial controls have3 coefficients", ["certified_publication.rs:straight_stress_spans"]),
        ("for value in &mut intensity", "3", O10 + "KD5L15: local axial/y/z intensity array is3", ["certified_publication.rs:fallback_circular_maximum"]),
        ("for value in sample", "3", O10 + "KD5L15: axial/y/z stress sample array is3", ["certified_publication.rs:fallback_circular_maximum"]),
        ("for (target, &value) in sections.iter_mut().zip(&ends[..6])", "6", O10 + "KD5L16: force/moment section array has6 components", ["certified_publication.rs:fallback_section_errors"]),
        ("for (index, row) in sections.iter_mut().enumerate()", "6", O10 + "KD5L16: force/moment section array has6 components", ["certified_publication.rs:fallback_section_errors"]),
        ("while remaining != 0", "4194304", O10 + "KD5L17: i32 shift moves toward0 by512 until its final smaller step; ceil(2^31/512) bounds all operands without a pressure-domain inference", ["certified_publication.rs:fallback_denominator_relative_error"]),
        ("while value != 0.0 && exponent != 0", "4194304", O10 + "KD5L17: i32 exponent moves toward0 by512 until final smaller step; positive universal type bound", ["certified_publication.rs:fallback_power_up"]),
    ]
    O10_SITE_FROM += [(site("fallback_power_up", _CP, 'return Err("fallback scaled constituent range".into())'),
        {"from": [fnkey("fallback_power_up", _CP)], "per": "1", "why": O10 + "KD5L18: error allocation returns immediately from the loop, at most1 per actual power-transport invocation; keep full case/replay multiplier"})]

# KD5A7: new endpoint/station MPa copies retain authored IDs and existing
# fixed-token caps; no blanket identifier or message narrowing.
if fnkey("append_stress_mpa", _CP):
    O10_ID_AUDIT += [
        (site("append_stress_mpa", _CP, 'format!("result:stress:', full=True), {"suffix": IN, "located": ST}),
        (site("append_stress_mpa", _CP, "kind: kind.into()", full=True), {"kind": ST}),
        (site("append_stress_mpa", _CP, "entity_ref: pipe_id.into()", full=True), {"pipe_id": IN}),
        (site("append_stress_mpa", _CP, "component: component.into()", full=True), {"component": ST}),
        (site("append_stress_mpa", _CP, "location: location.into()", full=True), {"location": ST}),
        (site("append_stress_mpa", _CP, "basis: basis.into()", full=True), {"basis": ST}),
        (site("append_stress_mpa", _CP, "let sign_convention=", full=True), {"label": ST,
            'iftail=="torsional-shear"{"torsionalshear"}elseiftail=="axial-normal"{"normal"}else{"bendingnormal"}': ST,
            "sign": {"bytes": _fixed["sign_bytes"], "source": "KD5A7: measured fixed sign constants consumed by pinned case caller"}}),
        (site("straight_stress_spans", _CP, 'format!("member:', full=True), {"pipe.element_id": IN, "pipe.element_id.len()": NUM_}),
        (site("recovery_finding", _FG, '"{}.{}.{}: member comparison unavailable', full=True, row="format!("), {"record.member": IN}),
    ]
    O10_ARGS_FIRST += [{"re": "^located$", "max": 128, "why": "KD5A7: endpoint/station location rewrites preserve caller's fixed≤128-B token"}]

# KD5A8: the withheld certified-summary warning's three authored-id copies
# (id suffix, message member id, affected ref) are the pipe's own id, IN128.
# Its {error} operand and the diagnostic's multiplicity are not narrowed.
if fnkey("withhold_certified_summaries", PP_ + "lib.rs"):
    _L = PP_ + "lib.rs"
    _kd5a8_id = site("solve_load_case_observed", _L, '"diagnostic:stress:{}:summary-unavailable"', full=True)
    _kd5a8_msg = site("solve_load_case_observed", _L, "the open-formula stress summary of {} is unavailable", full=True)
    _kd5a8_file, _kd5a8_line = _kd5a8_msg.rsplit(":", 1)
    if open(_os.path.join(_SNAP, _L), encoding="utf-8").read().split("\n")[int(_kd5a8_line)].strip() != "vec![pipe.element_id.clone()],":
        raise SystemExit("KD5A8: withheld summary affected-ref site changed; re-review")
    O10_ID_AUDIT += [
        (_kd5a8_id, {"stable_suffix(&pipe.element_id)": IN}),
        (_kd5a8_msg, {"pipe.element_id": IN}),
        (Resolved(f"{_kd5a8_file}:{int(_kd5a8_line) + 1}"), {"pipe.element_id": IN}),
    ]

# KD5A9 (T4 batch 4): certified_loads::member_source's ledger source label.
# Batch 4's pressure closure binding (arc pressure free strain, region id)
# gives the site a positive multiplicity. `member` is a pipe or connector
# component's authored id (IN128). `source` is an authored load or region id,
# the static "reference", or on the 0.4.0 route source_recovery::eigen_source_id
# ("load_state_eigenstrain:{len}:{pipe_id}", <= 23+20+1+128 B): the composite
# class (600) covers every caller. The site keeps its full multiplicity.
if fnkey("member_source", PP_ + "certified_loads.rs") and fnkey("closure_source", PP_ + "pressure_runtime.rs"):
    O10_ID_AUDIT += [(site("member_source", PP_ + "certified_loads.rs", 'format!("member:{}:{member}:{source}", member.len())', full=True),
                      {"member": IN, "source": CP})]

# C7A1: move only the five replaced owned-key audit lines to source-bound
# name keys. Borrowed input metadata is resolved without an allocation row.
import runpy as _c7_audit_runpy
_c7_audit_model = _c7_audit_runpy.run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "prescribed_key_model.py"))
_c7_audit_borrowed = _c7_audit_model["borrowed"](_SNAP)
_c7_audit_file = RE_ + "retained_precision.rs"
_c7_audit_sites = [
    ("numeric_owner", "prescribed.insert((text(&n[\"id\"]).into(), text(&d[\"dof\"][\"component\"]).into()));",
     "prescribed.insert((text(&n[\"id\"]), text(&d[\"dof\"][\"component\"])));",
     {"text(&d[\"\"][\"\"])": {"bytes": 128, "source": "IN128"}, "text(&n[\"\"])": {"bytes": 128, "source": "IN128"}}),
    ("numeric_owner", "prescribed.contains(&(text(&r[\"entity_ref\"]).into(), c.into()))",
     "prescribed.contains(&(text(&r[\"entity_ref\"]), c))",
     {"text(&r[\"\"])": {"bytes": 1024, "source": "RES"}}),
    ("g5c", "text(&d[\"node_id\"]).to_string(),", "text(&d[\"node_id\"]),",
     {"text(&d[\"\"])": {"bytes": 128, "source": "IN128"}}),
    ("g5c", "text(&d[\"component\"]).to_string(),", "text(&d[\"component\"]),",
     {"text(&d[\"\"])": {"bytes": 128, "source": "STATIC"}}),
    ("g5c", ".contains(&(text(&row[\"entity_ref\"]).into(), d.into()))",
     ".contains(&(text(&row[\"entity_ref\"]), d))",
     {"text(&row[\"\"])": {"bytes": 1024, "source": "RES"}}),
]
C7_INPUT_METADATA = [(site(_name, _c7_audit_file, _borrowed if _c7_audit_borrowed else _owned, full=True), _audit)
                     for _name, _owned, _borrowed, _audit in _c7_audit_sites]
if not _c7_audit_borrowed:
    O10_ID_AUDIT += C7_INPUT_METADATA


# ---- The pressure-source join (b7-plan; Design retained-pricing-rules.md §J). Each rule applies only where the
# join's code exists, so earlier revisions price as before.

# JN1: frame_kernel's correct_norm module moved, byte-identical, into the open_pipe_stress_correct_norm crate (price.py
# scans it and adds each dependent crate's edge). A crate-qualified `open_pipe_stress_correct_norm::normN(` keeps only
# its last qualifier, so by name it would also reach frame_kernel's same-crate private `final_case.rs:norm2` and
# `formation_check.rs:norm3`. Each frame_kernel caller is bound to the crate's function it names, as
# `crate::correct_norm::normN(` resolved before the move; every other candidate keeps its by-name resolution.
_CN = "core/numerics/correct_norm/src/lib.rs"
if _os.path.exists(_os.path.join(_SNAP, _CN)):
    _JN1 = {
        (FK_ + "rigid_body.rs", "assess_rigid_body"): {
            "open_pipe_stress_correct_norm::norm3(r[0], r[1], r[2])": "norm3",
            "open_pipe_stress_correct_norm::norm2(zeta, 1.0)": "norm2"},
        (FK_ + "structural/certified_recovery.rs", "norm_up"): {
            "open_pipe_stress_correct_norm::norm2(norm, v)": "norm2"},
        (FK_ + "structural/certified_recovery.rs", "certified_group"): {
            "open_pipe_stress_correct_norm::norm2(a.published, b.published)": "norm2",
            "open_pipe_stress_correct_norm::norm3(a.published, b.published, c.published)": "norm3",
            "open_pipe_stress_correct_norm::norm2(midpoint_norm, v)": "norm2"},
        (FK_ + "structural/retained/product_certificate/final_case.rs", "support_hypot"): {
            "open_pipe_stress_correct_norm::norm3(v[0],v[1],v[2])": "norm3"},
    }
    _jn1_found = []
    for _d, _, _fs in _os.walk(_os.path.join(_SNAP, FK_)):
        for _f in sorted(_fs):
            if _f.endswith(".rs") and "test" not in _f and "/tests" not in _d:
                _p = _os.path.join(_d, _f)
                _jn1_found += [(_os.path.relpath(_p, _SNAP), _l.strip()) for _l in open(_p, encoding="utf-8") if "open_pipe_stress_correct_norm::" in _l]
    if len(_jn1_found) != sum(len(_v) for _v in _JN1.values()):
        raise SystemExit(f"JN1: frame_kernel's crate-qualified correct_norm call inventory changed; re-review: {_jn1_found}")
    for (_file, _caller), _calls in _JN1.items():
        for _text in _calls:
            site(_caller, _file, _text)  # exactly once inside the caller, else stop
        for _name in sorted(set(_calls.values())):
            CG_RULES += [{"caller": _fn_n(_caller, _file, 1, 1), "name": _name, "targets": [_fn_n(_name, _CN, 1, 1)],
                "added": "JN1", "evidence": "the crate-qualified call names open_pipe_stress_correct_norm's pub fn (frame_kernel's former "
                "correct_norm module, moved unchanged); the same-crate private final_case.rs:norm2/formation_check.rs:norm3 are not that path"}]

# J1: the join's PRESSURE_SOURCE_ID arm of for_source (physics_source::validate_in under SourceContract::PressureV3).
# for_source's only D1 caller is retained_precision::validate's G7 on project(source), which sets
# producer.semantic_contract_id to route.base_id, one of two literals (preview-physics-1 on route L, physics-1 on
# route E; retained_precision.rs ROUTE base_id). So the arm is never taken: R1's and G4 (T17)'s argument for the
# sibling arms.
_SC = RE_ + "semantic_contract.rs"
if fnkey("for_source", _SC) and fnkey("validate_in", RE_ + "physics_source.rs"):
    site("for_source", _SC, "Some(PRESSURE_SOURCE_ID) =>")
    _rp = open(_os.path.join(_SNAP, RE_ + "retained_precision.rs"), encoding="utf-8").read()
    if sorted(_re.findall(r'base_id:\s*"([^"]+)"', _rp)) != ["openpipestress.result_semantics/0.3.0/physics-1",
                                                           "openpipestress.result_semantics/0.3.0/preview-physics-1"] \
            or 'projected["producer"]["semantic_contract_id"] = json!(route.base_id);' not in _rp:
        raise SystemExit("J1: retained_precision's G7 projection no longer pins the two base identities; re-review")
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("for_source", _SC), fnkey("validate_in", RE_ + "physics_source.rs"),
        O10 + "J1: the precommit reader validates project(successor), whose producer.semantic_contract_id is the route's "
        "base id (preview-physics-1 on route L, physics-1 on route E; retained_precision project()), never pressure-source-1, "
        "so for_source never takes the PRESSURE_SOURCE_ID arm; R1's argument")]

# J2: `attempted.push(request, id)` in lib.rs CaseSet::attempted passes the loop's pattern binding `(&request, &id)`, not
# a function; the by-name function-as-value scan picks the join's `fn request()` in source_recovery/j7_t4_consequences.rs,
# a private fn of a `#[cfg(test)]` child module of source_recovery, which lib.rs cannot name. Only that one edge is
# zeroed (the value scan's edges bypass callgraph exclusions); attempted's other edges stay.
_J7T = PP_ + "source_recovery/j7_t4_consequences.rs"
if _os.path.exists(_os.path.join(_SNAP, _J7T)) and fnkey("attempted", PP_ + "lib.rs"):
    site("attempted", PP_ + "lib.rs", "attempted.push(request, id)")
    if "#[cfg(test)]\n#[path = \"source_recovery/j7_t4_consequences.rs\"]\nmod j7_t4_consequences;" not in \
            open(_os.path.join(_SNAP, PP_ + "source_recovery.rs"), encoding="utf-8").read():
        raise SystemExit("J2: j7_t4_consequences is no longer a #[cfg(test)] module; re-review")
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("attempted", PP_ + "lib.rs"), fnkey("request", _J7T),
        O10 + "J2: `request` in `attempted.push(request, id)` is attempted's for-pattern binding `(&request, &id)`; "
        "j7_t4_consequences::request is a private fn of a #[cfg(test)] module, not nameable from lib.rs")]

# J3 (test code): source_recovery/j7_rf_census.rs is the J7 RF census, a child module that source_recovery.rs declares only
# under #[cfg(test)]; the template inventory's name filter does not see it as test code. Its format sites whose arguments
# have no class (Debug of diagnostic-code vectors, findings and failures; census labels and the expected-file writer) are
# sized 0 as O-10 J's multiplicity-0 sites: no site_zero, so the chain still computes each site's multiplicity and
# o10_check.py fails the run if any is ever positive or no longer present.
_J7C = PP_ + "source_recovery/j7_rf_census.rs"
if _os.path.exists(_os.path.join(_SNAP, _J7C)):
    if "#[cfg(test)]\n#[path = \"source_recovery/j7_rf_census.rs\"]\nmod j7_rf_census;" not in \
            open(_os.path.join(_SNAP, PP_ + "source_recovery.rs"), encoding="utf-8").read():
        raise SystemExit("J3: j7_rf_census is no longer a #[cfg(test)] module; re-review")
    _J3W = (O10 + "J3 (test code, multiplicity 0): source_recovery.rs declares j7_rf_census only under #[cfg(test)]; "
            "size 0 with no site_zero, so o10_check.py fails the run if the site's multiplicity is ever positive")
    O10_SITE_SIZE = list(globals().get("O10_SITE_SIZE", [])) + [(site(_fn, _J7C, _t, row=_row), 0, _J3W) for _fn, _t, _row in (
        ("actual", 'format!("normalization/admission: {:?}", blocking(&diagnostics))', None),
        ("actual", '.ok_or_else(|| format!("model build: {:?}", blocking(&diagnostics)))?', None),
        ("actual", 'return Err(format!("model build: {:?}", blocking(&diagnostics)));', None),
        ("actual", 'format!("actual boundary: {:?}", boundary.findings)', None),
        ("actual", 'format!("actual loads: {:?}", blocking(&diagnostics))', None),
        ("method_row_holds", 'format!("spring:{node}:{ordinal}")', None),
        ("leaf_diff", 'format!("{path}/{k}")', None),
        ("method_value", 'format!("{:?}", f.helper_stage)', None),
        ("method_value", 'format!("{:?}", f.error)', None),
        ("public_value", 'format!("{}: {}", d["code"].as_str().unwrap(), d["message"].as_str().unwrap())', None),
        ("census_entry", 'did not produce a selected response: {f:?}")', None),
        ("j7_rf_census_expected_write", 'sha(INPUTS.as_bytes()), REFERENCES_SHA256, lines.join(",\\n")', "format!("))]
    # Two of these sites also hold an identifier placeholder (`node`, `path`), so the chain still evaluates their format;
    # their other arguments get honest classes (unused at multiplicity 0).
    _j7c = open(_os.path.join(_SNAP, _J7C), encoding="utf-8").read()
    if 'const REFERENCES_SHA256: &str = "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9";' not in _j7c:
        raise SystemExit("J3: the census's pinned references.json changed; re-measure its longest reference key")
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("method_row_holds", _J7C, 'format!("spring:{node}:{ordinal}")', full=True),
         {"ordinal": {"bytes": 13, "source": O10 + "J3: a '.'-part of a reference key r[0] of references.json (sha 7b176dbb..., "
                      "pinned and asserted by the census); its longest key is 13 B"}}),
        (site("leaf_diff", _J7C, 'format!("{path}/{k}")', full=True),
         {"k": {"bytes": 1024, "source": O10 + "J3: a JSON object key of a produced public envelope (schema field names and "
                "entity-keyed maps of D1-capped ids) or of the test's literals: RES"}}),
    ]

# J4: the v3 formulation basis's first and selection limitations are `identity.to_string()` and `source.to_string()`,
# each a whole &'static str chosen from literals in pressure_v3_formulation_basis_in (2 identity texts; 3 source texts by
# `selected`). Each is bounded by its longest literal, read at the snapshot (the raw literal's length bounds the decoded one).
_EA = PP_ + "exact_admission.rs"
if fnkey("pressure_v3_formulation_basis_in", _EA):
    _ea = open(_os.path.join(_SNAP, _EA), encoding="utf-8").read()
    def _j4_literals(start, count):
        i = _ea.index(start); j = _ea.index("\n    };", i)
        lits = _re.findall(r'"((?:[^"\\]|\\.)*)"', _ea[i:j])
        if len(lits) != count or _ea.count(start) != 1:
            raise SystemExit(f"J4: {start!r} no longer selects exactly {count} literals; re-review")
        return max(len(l.encode()) for l in lits)
    _J4I, _J4S = _j4_literals("let identity = if selected == Some(true) {", 2), _j4_literals("let source = match selected {", 3)
    O10_ID_AUDIT = O10_ID_AUDIT + [
        (site("pressure_v3_formulation_basis_in", _EA, "identity.to_string(),", full=True),
         {"identity": {"bytes": _J4I, "source": O10 + "J4: the longer of the two static identity literals"}}),
        (site("pressure_v3_formulation_basis_in", _EA, "source.to_string(),", full=True),
         {"source": {"bytes": _J4S, "source": O10 + "J4: the longest of the three static source-selection literals"}}),
    ]

# J5: validate_physics_evidence_in calls load_reference::prepass only under `if resolved`, where `resolved = composite &&
# contract == PressureContract::PressureV3 && ...`. The one call passing composite = true is physics_source::validate_in
# (physics_source.rs), which is unreached on both routes (its only reached caller's edge, for_source -> validate_in, is J1's
# zero; G4 (T17) for its PhysicsV2 sibling). Every other call passes composite = false (physics_evidence.rs
# validate_physics_evidence and validate_pressure_evidence, load_reference.rs validate), so the edge is never taken.
_PE = RE_ + "physics_evidence.rs"
if fnkey("validate_physics_evidence_in", _PE) and fnkey("prepass", RE_ + "load_reference.rs"):
    _pe_txt = open(_os.path.join(_SNAP, _PE), encoding="utf-8").read()
    if not _re.search(r"let resolved = composite && contract == PressureContract::PressureV3\s*\n\s*&& evidence\.get\(\"load_reference_states\"\)\.is_some\(\);\s*\n\s*if resolved \{\s*\n(\s*//[^\n]*\n)*\s*crate::load_reference::prepass\(", _pe_txt) \
            or _pe_txt.count("crate::load_reference::prepass(") != 1:
        raise SystemExit("J5: validate_physics_evidence_in's prepass call is no longer only under `resolved`; re-review")
    _j5_calls = []
    for _d, _, _fs in _os.walk(_os.path.join(_SNAP, RE_)):
        for _f in sorted(_fs):
            if _f.endswith(".rs"):
                _src = open(_os.path.join(_d, _f), encoding="utf-8").read()
                _j5_calls += [(_f, _m) for _m in _re.findall(r"validate_physics_evidence_in\(([^,]*),\s*(true|false)\b", _src)]
    if sorted((_f, _m[1]) for _f, _m in _j5_calls) != [("load_reference.rs", "false"), ("physics_evidence.rs", "false"),
                                                       ("physics_evidence.rs", "false"), ("physics_source.rs", "true")]:
        raise SystemExit(f"J5: validate_physics_evidence_in's call inventory changed; re-review: {_j5_calls}")
    site("validate_in", RE_ + "physics_source.rs", "crate::physics_evidence::validate_physics_evidence_in(source, true, contract.pressure())?;")
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("validate_physics_evidence_in", _PE), fnkey("prepass", RE_ + "load_reference.rs"),
        O10 + "J5: prepass is called only when resolved (composite && PressureV3 && load_reference_states); the only composite "
        "caller is physics_source::validate_in, unreached on both routes (J1; G4 (T17)); every reached caller passes composite = false")]

# J6: the join moved shape_in's body, unchanged but for an explicit `resources` list consulted for a non-local `$ref`, into
# shape_in_resources; shape_in now only delegates with no resources. The registered review of shape_in's bounded self-
# recursion (loop_bounds recursion_reviewed: multiplicity 0 on D1, the source-block shape check is not on the D1 path; text
# only on its error path) applies to the function that now recurses. Its callers on the graph are shape_in (whose D1 reach
# was only through prepass, J5) and physics_source's receipt and transport-metadata checks (unreached).
_SBK = RE_ + "source_blocks.rs"
if fnkey("shape_in_resources", _SBK):
    site("shape_in", _SBK, "shape_in_resources(v, s, root, &[])")
    site("shape_in_resources", _SBK, ".is_some_and(|target| shape_in_resources(v, target, target_root, resources));")
    O10_RECURSION_REVIEWED = [(fnkey("shape_in_resources", _SBK), "J6: shape_in's reviewed recursion moved here with its body; "
        "multiplicity 0 on D1 (the source-block shape check is not on the D1 path; J5); text only on its error path")]
