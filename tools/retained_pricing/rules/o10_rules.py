"""O-10 rules (T4's heads), executed at the end of sq2_decisions_common.py (shared by routes L and E). Every key is
resolved by name at the snapshot (`fnkey`), so the rules carry to any head without line numbers; a rule whose function
does not exist at the snapshot is not added (it has nothing to bound there). The argument for each rule is in
DEL-04-08 Design/retained-pricing-rules.md under the same label."""


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
# B7b-B (D4 (a)): the precommit reader's two route bases, read from retained_precision's ROUTE literals. B7a's source
# has {physics-1, preview-physics-1}; B7b's has {pressure-1, preview-physics-1} (route E rebased on pressure-1).
# B7B_EXACT_BASE selects the B7b route-E rules (sq2_decisions_E.py, below); any other pair stops the run.
_rp_bases = sorted(_re.findall(r'base_id:\s*"([^"]+)"', open(_os.path.join(_SNAP, RE_ + "retained_precision.rs"), encoding="utf-8").read()))
_BASES_B7A = ["openpipestress.result_semantics/0.3.0/physics-1", "openpipestress.result_semantics/0.3.0/preview-physics-1"]
_BASES_B7B = ["openpipestress.result_semantics/0.3.0/pressure-1", "openpipestress.result_semantics/0.3.0/preview-physics-1"]
# T4-S5 (U4B-B): the epoch renames the two bases to pressure-2 and preview-physics-2 (S1's identity change, model
# 0.6.0); the routes, their projections and dispatch are B7b's, so the B7b route-E rules apply unchanged.
_BASES_U4B = ["openpipestress.result_semantics/0.3.0/pressure-2", "openpipestress.result_semantics/0.3.0/preview-physics-2"]
if _rp_bases not in (_BASES_B7A, _BASES_B7B, _BASES_U4B):
    raise SystemExit(f"B7b-B: retained_precision's two route bases changed; re-review R1/J1 and route E: {_rp_bases}")
B7B_EXACT_BASE = _rp_bases in (_BASES_B7B, _BASES_U4B)
# R1 (Agent 0's ruling: accepted). The pressure-1 arm of the precommit reader's dispatch.
_c, _e = fnkey("for_source", RE_ + "semantic_contract.rs"), fnkey("validate_pressure_evidence", RE_ + "physics_evidence.rs")
if _c and _e:
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(_c, _e, O10 + "R1 (ruled accepted): the precommit reader validates project(successor), whose "
        "producer.semantic_contract_id is the route's base id (preview-physics-1 on route L, physics-1 on route E; RS project()), "
        "never pressure-1, so for_source never takes the PRESSURE_ID arm; the same argument as the registered G4 (T17) siblings"
        + ("; B7b-R1: since B7b D4 (a) route E's base is pressure-1, so this zero holds on route L only and "
           "sq2_decisions_E.py removes it" if B7B_EXACT_BASE else ""))]


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
if fnkey("member_descriptors", PP_ + "source_recovery.rs"):
    # RF-ID: two formerly inline member-ID owners moved into this helper.
    # Both are still full <=128-byte input clones with their full calls.
    _rf_file = PP_ + "source_recovery.rs"
    _rf_lines = open(_os.path.join(_SNAP, _rf_file), encoding="utf-8").read().splitlines()
    _rf_id_sites = [i + 1 for i, line in enumerate(_rf_lines) if "member: member.to_owned()," in line]
    if len(_rf_id_sites) != 2:
        raise SystemExit("RF-ID: member_descriptors clone inventory changed")
    O10_ID_AUDIT += [(Resolved(f"{_rf_file}:{line}"),
                      {"member": {"bytes": 128, "source": O10 + "RF-ID: admitted pipe element_id"}})
                     for line in _rf_id_sites]
    # RF-EDGE: support_units passes the bound &CongruenceMember variable
    # `member` to exact_entry_pairs. The by-name value scan mistakes that
    # argument for certified_publication::member; there is no such call.
    O10_EDGE_ZERO += [(fnkey("support_units", _rf_file),
                      fnkey("member", PP_ + "certified_publication.rs"),
                      O10 + "RF-EDGE: pattern-bound member is &CongruenceMember, passed to exact_entry_pairs; not the certified-publication function")]
# B5K1: U5a extracts the mechanics body without deleting its priced sites.
_CASE_FN = "solve_prepared_load_case" if fnkey("solve_prepared_load_case", PP_ + "lib.rs") else "solve_load_case_observed"
_PRESSURE_FN = "assemble_pressure_case" if fnkey("assemble_pressure_case", PP_ + "pressure_runtime.rs") else "build_pressure_case_with_members"
B5_FUNCTION_REKEY = []
if _PRESSURE_FN == "assemble_pressure_case":
    B5_FUNCTION_REKEY = [(fnkey("build_pressure_case_with_members", PP_ + "pressure_runtime.rs"), fnkey(_PRESSURE_FN, PP_ + "pressure_runtime.rs"),
        "B5K1: retained route-L zero edges follow the extracted pressure body; route-E's removal still counts the historical wrapper edges before the migration")]
CE = RE_ + "connector_evidence.rs"
D14 = ("D1.4 (retained_memory.rs D1.4: `if !m.components.is_empty() { return refuse(C::Invocation, F::Components) }`): an admitted "
       "model has no components")
NOV3 = ("D1.3 (namespace_branch) admits only schema 0.1.0/0.2.0 with no pressure contract, or 0.3.0 with "
        "2.0.0/exact_straight_pressure_v2; an ObjectiveConnector is made only by joint::build_connector (FK ObjectiveConnector "
        "fields are private; ObjectiveConnector::new is called only there), whose one call (build_model_for_members) runs only "
        "when exact_contract(model) == PressureV3 (3.0.0/exact_pressure_v3), and iterates model.components")
if B7B_EXACT_BASE:
    # B7b-P1: route E is 3.0.0/exact_pressure_v3 since D4 (a), so premise P1 ("no admitted model is PressureV3") no
    # longer holds; L1-L3 and J rest on P2 (D1.4: no components) alone.
    NOV3 = ("B7b-P1: P1 no longer holds (route E is 3.0.0/exact_pressure_v3, D4 (a)); an ObjectiveConnector is made only by "
            "joint::build_connector (FK ObjectiveConnector fields are private; ObjectiveConnector::new is called only there), "
            "whose one call (build_model_for_members) iterates model.components, empty under P2 (" + D14 + ")")
# O10_LOOPS_LAST: (exact loop header, bound, why, the only functions it may match: checked after the run by o10_check.py).
O10_LOOPS_LAST = []
if fnkey("member_descriptors", PP_ + "source_recovery.rs"):
    O10_LOOPS_LAST += [(
        "for (index, term) in contributions.iter().enumerate().filter(|(_, t)| t.row == dof)",
        "cases*(144*m+s)", O10 + "RF-LU: support unit precheck visits only the complete frame/spring contribution list; per-case scope retained",
        ["source_recovery.rs:support_units"])]
if _PRESSURE_FN == "assemble_pressure_case":
    O10_LOOPS_LAST += [
        ("for ((dof, pbits, area, _), group) in groups", "0", O10 + "B5L1: both bore and joint factor groups are produced only inside D1.5's absent/empty pressure-region loop; retains the former factor-group zero on the new key shape", ["pressure_runtime.rs:finish_source_groups"]),
        ("for edge in edges", "0", O10 + "B5L2: connector pressure edges exist only in pressure regions with built connectors; D1.4/5 have neither and project(successor) retains empty pressure evidence; keeps outer scans and every reader owner", ["connector_evidence.rs:validate_pressure"]),
    ]
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
    # B7b-E1/E2/E4/E5: since D4 (a) route E's G7 reaches validate_pressure_evidence -> validate_physics_evidence_in(..,
    # PressureV3), so `admitted` is true; the zero now rests on D1.4 through the producer instead of `admitted`.
    _E1_B7B = ("B7b-E1 (route E, connector_evidence::validate): every reached caller passes PressureV3 "
        "(validate_pressure_evidence -> validate_physics_evidence_in(.., false, PressureV3)), so `admitted` is true and "
        "`require_connector(admitted || records.is_empty(), ..)` passes; `records` is project(successor)'s "
        "contract_evidence.connector, which the producer sets to joint::connector_evidence(model): one record per "
        "objective-connector component (filter_map over model.components). " + D14 + ", so the list is empty")
    O10_LOOPS_LAST += [
        ("for r in records", "0",
         O10 + (_E1_B7B if B7B_EXACT_BASE else
         "E1 (route E, connector_evidence::validate): `records` non-empty returns Err at `require_connector(admitted || "
         "records.is_empty(), \"UNSUPPORTED\")?` before this loop unless `admitted`; admitted = (contract == PressureV3), and the "
         "reached callers pass StraightV2 (validate_physics_evidence -> validate_physics_evidence_in(.., StraightV2)); the "
         "PressureV3 callers are validate_pressure_evidence (physics_evidence.rs) and its load-state arm "
         "load_reference::validate_pressure_load_reference_evidence (load_reference.rs:387, called only from "
         "validate_pressure_evidence), both behind the edge R1 zeroes"), ["connector_evidence.rs:validate"]),
        ("for component in spans.keys()", "0",
         O10 + "E2 (route E): `spans` is filled only inside the records loop (E1), so it is empty", ["connector_evidence.rs:validate"]),
        ("for row in rows.values()", "Pall",
         O10 + "E3 (route E): one pass over the validated envelope's result rows (physics_evidence `rows`, indexed by id), "
         "<= C·P_final = Pall", ["connector_evidence.rs:validate"]),
    ]
    if fnkey("row_basis", CE):
        O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("validate", CE), fnkey("row_basis", CE),
        O10 + ("B7b-E4 (route E): row_basis is called only for a connector-kind row (is_connector_kind) after the "
        "kind check; the producer publishes connector rows only for built connectors (lib.rs append_connector_results "
        "iterates built.connectors, L1: empty under D1), and " + D14 + ", so project(successor) has no connector row"
        if B7B_EXACT_BASE else
        "E4 (route E): row_basis is called only for a connector-kind row after `require_connector(admitted, \"UNSUPPORTED\")?`, "
        "which returns Err when !admitted (E1: every reached caller passes StraightV2), ending validate"))]
    _E5_FROM = "validate_pressure_evidence" if B7B_EXACT_BASE else "validate_physics_evidence"
    O10_SITE_FROM = O10_SITE_FROM + [(site("require_connector", CE, 'Err(format!("SOURCE_PHYSICS_CONNECTOR_{code}"))'),
        {"from": [fnkey(_E5_FROM, RE_ + "physics_evidence.rs")], "per": "1",
         "why": O10 + "E5 (route E, as physics_evidence.rs require()): require_connector allocates only when it returns Err; all 16 "
         "calls in connector_evidence.rs are `require_connector(..)?` and connector_evidence::validate's caller "
         "(validate_physics_evidence_in) propagates with `?`: <= 1 per " + _E5_FROM + " call"
         + ("; B7b-E5: route E's reached entry is validate_pressure_evidence (D4 (a)), whose physics arm returns "
            "validate_physics_evidence_in's Result unchanged" if B7B_EXACT_BASE else "")})]
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
# priced (no zero); only its identifier and static arguments are classed. Arguments in DEL-04-08 Design/retained-pricing-rules.md, section C.
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
    O10_LOOPS_LAST += [("for junction in &junctions", "m", O10 + "U2E2: checked traversal's junction list has one entry per adjacent member pair, <= traversal length <= m", ["pressure_runtime.rs:" + _PRESSURE_FN])]
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
    if fnkey("section_basis_row", _L) and fnkey("published_nodal_displacements", _L):
        # U4B-K (T4-U4b, §2.3): the arc branch's withheld maximum is gone; a realized arc's certified maximum, when
        # unavailable, warns with the case-qualified id beside the straight id, each ending in the basis's static
        # hyphen segment. The classes are U6-K's.
        O10_ID_AUDIT += [
            (site(_CASE_FN, _L, 'diagnostics.push(diag(&format!("diagnostic:exact-stress:{}:{}:extrema:{}"', full=True),
             {"stable_suffix(&load_case.id)": IN, "stable_suffix(&pipe.element_id)": IN, "basis.hyphen()": ST}),
            (site(_CASE_FN, _L, 'let id = format!("diagnostic:exact-stress:{}:extrema:{}"', full=True),
             {"stable_suffix(&pipe.element_id)": IN, "basis.hyphen()": ST}),
        ]
    elif fnkey("section_basis_row", _L):
        # U6-K (D-5): the arc branch's case-qualified id is now formatted once per basis, beside the straight id; both
        # end in the basis's static hyphen segment (`SectionBasis::hyphen`, <= 15 B). The authored ids keep IN128.
        O10_ID_AUDIT += [
            (site(_CASE_FN, _L, 'Some(_) => format!("diagnostic:exact-stress:{}:{}:extrema:{}"', full=True),
             {"stable_suffix(&load_case.id)": IN, "stable_suffix(&pipe.element_id)": IN, "basis.hyphen()": ST}),
            (site(_CASE_FN, _L, 'None => format!("diagnostic:exact-stress:{}:extrema:{}"', full=True),
             {"stable_suffix(&pipe.element_id)": IN, "basis.hyphen()": ST}),
        ]
    else:
        O10_ID_AUDIT += [
            (site(_CASE_FN, _L, 'last.id = format!("diagnostic:exact-stress:', full=True), {"stable_suffix(&load_case.id)": IN, "stable_suffix(&pipe.element_id)": IN}),
        ]
    if fnkey("arc_recovery_finding", PP_ + "formation_guard.rs"):
        O10_ID_AUDIT += [(site(_CASE_FN, _L, "project: model.project.id.clone()", full=True), {"model.project.id": IN})]
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
    if fnkey("member_prefix", PP_ + "lib.rs"):
        # U8-MP (T4-U8, PP): certified_publication's member and straight_stress_spans call lib.rs member_prefix, the
        # one `format!("member:{}:{member}:", member.len())`, also used by frame_member_identities and the pressure
        # closure identity. Every caller passes a pipe's admitted element id (IN128); its length is NUM.
        O10_ID_AUDIT += [(site("member_prefix", PP_ + "lib.rs", 'format!("member:{}:{member}:", member.len())', full=True),
                          {"member": IN, "member.len()": NUM_})]
    else:
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
    O10_ID_AUDIT += [(site(_CASE_FN, PP_ + "lib.rs", "member: pipe.element_id.clone()", full=True), {"pipe.element_id": IN})]
# Nonlinear K-D5 (DEL-04-01 kd5-nonlinear-active-set.md section 1.5): the
# unavailable-family label moved, unchanged, from `check` into its callee `check_at`.
O10_ID_AUDIT += [(site("check_at" if fnkey("check_at", FK_ + "structural/formation_check.rs") else "check",
                       FK_ + "structural/formation_check.rs", "family.clone()", full=True), {"family": ST})]


if fnkey("member_group_fires", _FG):
    O10_LOOPS_LAST += [
        ("for p in &source.loads", "0", O10 + "KD5L3: D1/W1 excludes load-state; source_factor returns None unless is_load_state; nodal uses push input and no load primitive", ["structural/formation_check.rs:load_closure"]),
        ("for (i, (term, record)) in terms.iter().zip(records).enumerate()", "l", O10 + "KD5L4: exactly one trusted closure record per nodal ledger term, <= l", ["structural/formation_check.rs:load_closure"]),
        ("for (j, expected) in manifest.iter().enumerate()", "0", O10 + "KD5L5: every D1 nodal record is input (formed=false); no manifest is installed; loop only after formed guard", ["structural/formation_check.rs:load_closure"]),
        ('for (location, rows) in locations.into_iter().chain( [" ", " ", " ", " ", " "] .iter() .zip(effective.sections.iter()) .map(|(l, r)| (*l, r.as_slice())), )', "7", O10 + "KD5L6: two end rows plus five fixed CertifiedMemberRecovery sections", ["certified_publication.rs:member_record"]),
        ("iter:station_resultants.iter().enumerate().map", "3", O10 + "KD5L7: quarter_1, midspan and quarter_3 station rows, exactly three", ["lib.rs:" + _CASE_FN]),
        ("iter:diagnostics.iter().position", "D", O10 + "KD5L8: one scan bounded by the case's diagnostic Vec, <= the full diagnostic fixpoint D", ["lib.rs:" + _CASE_FN]),
        ("for rows in &member.sections", "5", O10 + "KD5L9: CertifiedMemberRecovery sections is [..;5]", ["lib.rs:" + _CASE_FN]),
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
        (site(_CASE_FN, PP_ + "lib.rs", "record.certified_support_norms.push", full=True), {"support.id": IN}),
    ]
    _PR = PP_ + "pressure_runtime.rs"
    if fnkey("member_prefix", PP_ + "lib.rs") and fnkey("closure_source", _PR):
        # U8-MP (PP): closure_source's label is now per geometry: a straight operand's begins with its member prefix
        # (lib.rs member_prefix of term.2, a pipe's admitted id: <= "member:" + 3 digits + ":" + 128 + ":" = 140 B)
        # and the arc label keeps KD5A3's spelling. Both are inside the region path (KD5L10).
        O10_ID_AUDIT += [
            (site("closure_source", _PR, 'PressureGeometry::Straight { .. } => format!("{}{}:{}", super::member_prefix(&term.2), term.1, term.3)', full=True),
             {"super::member_prefix(&term.2)": {"bytes": 140, "source": "O10: U8-MP member prefix of an admitted pipe id"}, "term.1": IN, "term.3": ST}),
            (site("closure_source", _PR, '_ => format!("{}:{}:{}", term.1, term.2, term.3),', full=True), {"term.1": IN, "term.2": IN, "term.3": ST}),
        ]
    else:
        O10_ID_AUDIT += [
            # KD5A3 (batch 4 re-key): the pressure closure label moved, unchanged,
            # from finish_source_groups' map closure into its callee closure_source.
            (site("closure_source" if fnkey("closure_source", _PR) else "finish_source_groups",
                  _PR, ('        format!("{}:{}:{}", term.1, term.2, term.3)' if _PRESSURE_FN == "assemble_pressure_case" else 'format!("{}:{}:{}", term.1, term.2, term.3)'), full=True), {"term.1": IN, "term.2": IN, "term.3": ST}),
        ]
    if _PRESSURE_FN == "assemble_pressure_case":
        O10_ID_AUDIT += [(site("closure_source", PP_ + "pressure_runtime.rs", 'return (format!("{}:{}:{}", term.1, term.2, term.3)', full=True), {"term.1": IN, "term.2": IN, "term.3": ST})]
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
    _detail_from = "check"
    if fnkey("check_at", _FC):
        # Nonlinear K-D5 re-review (DEL-04-01 kd5-nonlinear-active-set.md
        # section 1.5): `check` is now a pure delegation to `check_at`, which
        # holds the former `check` body plus the `judged` flag and the
        # per-attempt epsilon_E cache. `check_at` calls `evaluate` once and
        # `detail` at most once on its Err arm, so the KD5M1 cap moves to
        # `check_at` invocations. `evaluate` gained only the judged filter and
        # the cache (no new allocation, error or detail call); `load_closure`
        # gained the two friction formulas in the existing nodal binding arm
        # (inventory still 15). `detail` is byte-identical.
        # T4-U8 (FK): `evaluate` adds one `straight_frames_bound(source)?;` after `load_closure(..)?`. Its only
        # allocation is one returned Failure::Closure (U8-FB), which `detail` formats in its existing Closure arm;
        # the per-invocation caps below are unchanged (retained-pricing-rules.md, "T4-U6/U8: actual-source D3").
        _invocation_pins = {
            "detail": "ae006b02fed244b2e77686378bdb17b87c442a1d4873add6f4b2129d3be1bc0f",
            "check": "d22bba23d1f09025f3ec267d9ea30b8aaa748e6294b5b34d0e4ea7f1f2616ef7",
            "check_at": "29c96900380beb699a6040a67ec13d7b665944cf47fb02d26b1b64d8de4251e4",
            "evaluate": "a422166f88cfa91bb6d5596111fc42f7d5df1757ad76890e40b30d4765d678d1",
            "load_closure": "0b662c23c565d94cf0964b6efcbb575afa41958d21a00f4acf8e453527543867",
        }
        # T4-S5 (U5B-TR): load_closure adds the TieRodReference binding arm (tie rods exist only on connectors, none on
        # D1 by P3). Its one refusal is a Failure::Closure "{source}: tie-rod source binding" in the existing Closure
        # path, inside the 8352 B Closure class; the per-invocation caps are unchanged.
        _detail_from = "check_at"
    _fc_lines = open(_os.path.join(_SNAP, _FC), encoding="utf-8").read().splitlines(keepends=True)
    # U5B-TR: the TieRodReference arm's one returning Closure format (P3: never reached on D1; still charged).
    _TIE_ROD_CLOSURE = 1 if any('"{}: tie-rod source binding",' in _l for _l in _fc_lines) else 0
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
        ("detail", _detail_from, "2", 7, "KD5M1: at most one inner Second detail and one outer detail per check/evaluate invocation; first-error returns prevent another inner detail; charge each exclusive allocation alternative separately"),
        ("load_closure", "load_closure", "1", (17 if _PRESSURE_FN == "assemble_pressure_case" else 15) + _TIE_ROD_CLOSURE, "KD5M2: every error allocation returns Err or propagates with ?, so at most one allocation across all closure errors per invocation; charge each exclusive alternative separately; B5 adds two returning joint-binding literals"),
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
        "lib.rs": _labels_model["primitive_lib_minmax_callers"](_SNAP),
        "pressure_exact.rs": ["fused_mul_add"], "pressure_runtime.rs": ["finish_source_groups"],
        "retained_memory.rs": ["string", "material", "walk", "combination_census", "longest_string", "complete_observations"],
        "retained_product.rs": ["g5a"], "source_recovery.rs": ["solve"] + (
            ["prepare_sources"] if fnkey("member_descriptors", PP_ + "source_recovery.rs") else []),
        "source_receipt/composite.rs": ["norm"],
        "source_receipt/endpoint_maximum.rs": ["absolute", "endpoint_maximum", "directed"],
        "source_receipt/rows.rs": ["derived_for"],
        "case_state/thermal.rs": ["integral", "dilation_difference", "check_path"],
        "certified_publication.rs": ["fallback_denominator_relative_error"],
    }
    # W8-CG (W-8 topology warnings, retained-pricing-rules.md §8.16, §8.18): near_duplicate_nodes' min/max receivers
    # are an edge's usize node indices (`a.min(b)`, `a.max(b)`; the bounding box's f64 `low`/`high` folds are gone with
    # the absolute tolerance); scaled_norm's fold takes `m.max(x.abs())` over f64. Only where the topology checks exist.
    if fnkey("near_duplicate_nodes", PP_ + "topology.rs"):
        _primitive_minmax_callers["topology.rs"] = ["near_duplicate_nodes"] + (
            ["scaled_norm"] if fnkey("scaled_norm", PP_ + "topology.rs") else [])
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
        (site("recovery_finding", _FG, '"{}.{}.{}: member comparison unavailable', full=True, row="format!("), {"record.member": IN}),
    ]
    if not fnkey("member_prefix", PP_ + "lib.rs"):   # U8-MP: on PP source the prefix is lib.rs member_prefix's
        O10_ID_AUDIT += [(site("straight_stress_spans", _CP, 'format!("member:', full=True), {"pipe.element_id": IN, "pipe.element_id.len()": NUM_})]
    O10_ARGS_FIRST += [{"re": "^located$", "max": 128, "why": "KD5A7: endpoint/station location rewrites preserve caller's fixed≤128-B token"}]

# KD5A8: the withheld certified-summary warning's three authored-id copies
# (id suffix, message member id, affected ref) are the pipe's own id, IN128.
# Its {error} operand and the diagnostic's multiplicity are not narrowed.
if fnkey("withhold_certified_summaries", PP_ + "lib.rs"):
    _L = PP_ + "lib.rs"
    _kd5a8_id = site(_CASE_FN, _L, '"diagnostic:stress:{}:summary-unavailable"', full=True)
    _kd5a8_msg = site(_CASE_FN, _L, "the open-formula stress summary of {} is unavailable", full=True)
    _kd5a8_file, _kd5a8_line = _kd5a8_msg.rsplit(":", 1)
    if open(_os.path.join(_SNAP, _L), encoding="utf-8").read().split("\n")[int(_kd5a8_line)].strip() != "vec![pipe.element_id.clone()],":
        raise SystemExit("KD5A8: withheld summary affected-ref site changed; re-review")
    O10_ID_AUDIT += [
        (_kd5a8_id, {"stable_suffix(&pipe.element_id)": IN}),
        (_kd5a8_msg, {"pipe.element_id": IN}),
        (Resolved(f"{_kd5a8_file}:{int(_kd5a8_line) + 1}"), {"pipe.element_id": IN}),
    ]

# B5A1/C5: authored connector identity and the fixed inline recovery literal.
if _PRESSURE_FN == "assemble_pressure_case":
    O10_ID_AUDIT += [(site("pressure_finding", PP_ + "joint.rs", 'format!("load case {case}, region {region}, connector {connector}: {reason}")', full=True), {"connector": IN})]
    import runpy as _b5_audit_runpy
    _b5_audit = _b5_audit_runpy.run_path(_os.path.join(_os.path.dirname(__file__), "../base/batch5_model.py"))
    _b5_audit["descriptor_recovery_bytes"](_SNAP)
    # C5 puts the same 73 tangent-label bytes directly in the basis format!
    # template: its literal portion is now 631 B (previously 558 B + the
    # 73-B borrowed recovery operand). Existing literal TEXT accounting
    # charges those bytes once; there is no recovery_treatment operand.

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
    # U5B-JN (T4-U5b): the tie rod's length, bound to the crate's norm3 as the callers above.
    if fnkey("direction_and_length", FK_ + "tie_rod.rs"):
        _JN1[(FK_ + "tie_rod.rs", "direction_and_length")] = {"open_pipe_stress_correct_norm::norm3(d[0], d[1], d[2])": "norm3"}
    _jn1_found = []
    # W1R-FG (§8.11): the census probe is not compiled in the registered build; its norm3 call is not a caller.
    _jn1_off = __import__("runpy").run_path(_os.path.join(_os.path.dirname(__file__), "..", "base", "loopscan.py"))["off_build_files"](_SNAP)
    for _d, _, _fs in _os.walk(_os.path.join(_SNAP, FK_)):
        for _f in sorted(_fs):
            if _f.endswith(".rs") and "test" not in _f and "/tests" not in _d \
                    and _os.path.relpath(_os.path.join(_d, _f), _SNAP) not in _jn1_off:
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
    # B7b-J1: B7b's route E base is pressure-1 (D4 (a)); either pair (B7B_EXACT_BASE above) excludes pressure-source-1.
    # U4B-B: the epoch's -2 pair is B7b's routes renamed; it excludes pressure-source-1 as well.
    if sorted(_re.findall(r'base_id:\s*"([^"]+)"', _rp)) != (_rp_bases if B7B_EXACT_BASE else _BASES_B7A) \
            or 'projected["producer"]["semantic_contract_id"] = json!(route.base_id);' not in _rp:
        raise SystemExit("J1: retained_precision's G7 projection no longer pins the two base identities; re-review")
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("for_source", _SC), fnkey("validate_in", RE_ + "physics_source.rs"),
        O10 + "J1: the precommit reader validates project(successor), whose producer.semantic_contract_id is the route's "
        "base id (preview-physics-1 on route L, " + ("pressure-1 (B7b D4 (a))" if B7B_EXACT_BASE else "physics-1")
        + " on route E; retained_precision project()), never pressure-source-1, "
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

# U5B-JV (T4-U5b): `Err(Some(problem)) => problems.push(problem)` in joint.rs `hardware` passes the match binding
# `problem` (a String), not a function; the by-name function-as-value scan picks pressure_runtime's private `fn problem`,
# which joint.rs does not call. Only that one edge is zeroed; hardware's other edges stay (and hardware itself is on
# the connector path, which D1's empty components never reach: P3).
_JR = PP_ + "joint.rs"
if fnkey("hardware", _JR) and fnkey("problem", PP_ + "pressure_runtime.rs"):
    site("hardware", _JR, "Err(Some(problem)) => problems.push(problem),")
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("hardware", _JR), fnkey("problem", PP_ + "pressure_runtime.rs"),
        O10 + "U5B-JV: `problem` in `problems.push(problem)` is hardware's match binding `Err(Some(problem))`, a String; "
        "pressure_runtime::problem is a private fn of another module that joint.rs never calls")]

# RA-EDGE (R-A D3, retained-pricing-rules.md §8.13): `publication_predicate(&h, row, scale, meter)` in adaptive.rs
# `certify_rows` passes the loop's pattern binding `((row, meta), v)`, a published row, not a function; the by-name
# function-as-value scan picks R-A's row-trace recorder `fn row` in exact_boundary/exact_operation_count.rs, a module
# exact_boundary.rs declares only under `#[cfg(any(test, feature = "exact-operation-count"))]` (blanked as TEST_ATTR;
# adaptive.rs never names it). Only that one edge is zeroed; certify_rows' other edges stay.
_EOC = FK_ + "structural/exact_boundary/exact_operation_count.rs"
if fnkey("certify_rows", FK_ + "structural/retained/adaptive.rs") and fnkey("row", _EOC):
    site("certify_rows", FK_ + "structural/retained/adaptive.rs", "publication_predicate(&h, row, scale, meter)")
    _eb = open(_os.path.join(_SNAP, FK_ + "structural/exact_boundary.rs"), encoding="utf-8").read()
    if _eb.count("mod exact_operation_count;") != 1 or \
            "#[cfg(any(test, feature = \"exact-operation-count\"))]\npub mod exact_operation_count;" not in _eb:
        raise SystemExit("RA-EDGE: exact_operation_count is no longer a test/exact-operation-count module; re-review")
    if "exact_operation_count" in open(_os.path.join(_SNAP, FK_ + "structural/retained/adaptive.rs"), encoding="utf-8").read():
        raise SystemExit("RA-EDGE: adaptive.rs names exact_operation_count; re-review")
    O10_EDGE_ZERO = O10_EDGE_ZERO + [(fnkey("certify_rows", FK_ + "structural/retained/adaptive.rs"), fnkey("row", _EOC),
        O10 + "RA-EDGE: `row` in `publication_predicate(&h, row, scale, meter)` is certify_rows' for-pattern binding "
        "`((row, meta), v)`; exact_operation_count::row is R-A's row-trace recorder in a test/exact-operation-count "
        "module that adaptive.rs never names")]

# J3 (test code): source_recovery/j7_rf_census.rs is the J7 RF census, a child module that source_recovery.rs declares only
# under #[cfg(test)]; the template inventory's name filter does not see it as test code. Its format sites whose arguments
# have no class (Debug of diagnostic-code vectors, findings and failures; census labels and the expected-file writer) are
# sized 0 as O-10 J's multiplicity-0 sites: no site_zero, so the chain still computes each site's multiplicity and
# o10_check.py fails the run if any is ever positive or no longer present.
_J7C = PP_ + "source_recovery/j7_rf_census.rs"
if _os.path.exists(_os.path.join(_SNAP, _J7C)):
    # O4 S5 (retained-pricing-rules.md §8.9): O4's test modules reach the census through `pub(crate) mod`; the
    # declaration is still the one #[cfg(test)] item, so either visibility keeps the module test-only.
    _j7_decl = open(_os.path.join(_SNAP, PP_ + "source_recovery.rs"), encoding="utf-8").read()
    if not any("#[cfg(test)]\n#[path = \"source_recovery/j7_rf_census.rs\"]\n" + vis + "mod j7_rf_census;" in _j7_decl
               for vis in ("", "pub(crate) ")) or _j7_decl.count("mod j7_rf_census;") != 1:
        raise SystemExit("J3: j7_rf_census is no longer a #[cfg(test)] module; re-review")
    _J3W = (O10 + "J3 (test code, multiplicity 0): source_recovery.rs declares j7_rf_census only under #[cfg(test)]; "
            "size 0 with no site_zero, so o10_check.py fails the run if the site's multiplicity is ever positive")
    _J3_SPECS = (
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
        ("j7_rf_census_expected_write", 'sha(INPUTS.as_bytes()), REFERENCES_SHA256, lines.join(",\\n")', "format!("))
    # Two of these sites also hold an identifier placeholder (`node`, `path`), so the chain still evaluates their format;
    # their other arguments get honest classes (unused at multiplicity 0).
    _j7c = open(_os.path.join(_SNAP, _J7C), encoding="utf-8").read()
    if 'const REFERENCES_SHA256: &str = "7b176dbbf2296be02d8bca19c698ee56d0d5753751150a9175e0d3ad4cf89cc9";' not in _j7c:
        raise SystemExit("J3: the census's pinned references.json changed; re-measure its longest reference key")
    _J3_AUDITS = (
        ("method_row_holds", 'format!("spring:{node}:{ordinal}")',
         {"ordinal": {"bytes": 13, "source": O10 + "J3: a '.'-part of a reference key r[0] of references.json (sha 7b176dbb..., "
                      "pinned and asserted by the census); its longest key is 13 B"}}),
        ("leaf_diff", 'format!("{path}/{k}")',
         {"k": {"bytes": 1024, "source": O10 + "J3: a JSON object key of a produced public envelope (schema field names and "
                "entity-keyed maps of D1-capped ids) or of the test's literals: RES"}}),
    )
    if fnkey("u4b_every_selected_census_case_still_selects_with_its_new_charge", _J7C):
        # U4B-J3 (T4-U4b §4.3, test code): the census's new capability test formats two report lines; J3's
        # multiplicity-0 sizes apply. Its `largest.max(..)` receivers are u64 (`let mut largest = 0u64`), so the
        # by-name call cannot reach certified_publication's private FallbackError::max.
        _u4b_j3 = "u4b_every_selected_census_case_still_selects_with_its_new_charge"
        _J3_SPECS += ((_u4b_j3, 'let mut line = format!("U4B_J7_SELECTED {id} {} class={class} direct_charged={direct} (pinned {})"', None),
                      (_u4b_j3, 'line += &format!(" publication_case_charged={charged}");', None))
        CG_RULES += [{"caller": _fn_n(_u4b_j3, _J7C, 1, 1), "name": "max", "targets": [],
            "added": "U4B-J3", "evidence": "test code: both receivers are the u64 `largest`; the intrinsic u64::max, never the private FallbackError"}]
    if fnkey("member_descriptors", PP_ + "source_recovery.rs"):
        # RF-J3: the census's class/member assertions construct only these
        # two new label templates. Retain J3's measured multiplicity-zero
        # guards, with honest finite classes for their literal-loop inputs.
        _rf_cancel = 'format!("RF-CANCEL-{load}-G1e{exponent}-{pattern}")'
        _rf_companion = 'format!("RF-CANCEL-{load}-G1e8-GnG-{companion}")'
        _J3_SPECS += (("j7_rf_census_classification_and_pin", _rf_cancel, None),
                      ("j7_rf_census_classification_and_pin", _rf_companion, None))
        _J3_AUDITS += (("j7_rf_census_classification_and_pin", _rf_cancel,
                        {"load": {"bytes": 1, "source": "STATIC: F or M"},
                         "pattern": {"bytes": 3, "source": "STATIC: GnG, GGn or nGG"}}),
                       ("j7_rf_census_classification_and_pin", _rf_companion,
                        {"load": {"bytes": 1, "source": "STATIC: F or M"},
                         "companion": {"bytes": 7, "source": "STATIC: ORTHO or INPLANE"}}))
    # B7b-J3: B7b's test-only census rewrite (2d0e2210c1) deletes leaf_diff; site() returns None for a function the
    # snapshot no longer defines, and such a rule bounds nothing. Only J3's own entries are filtered, and the dropped
    # set must be exactly the functions the snapshot no longer defines, of which only leaf_diff may be missing.
    _j3_dropped = sorted({_fn for _fn, _t, _row in _J3_SPECS if site(_fn, _J7C, _t, row=_row) is None}
                         | {_fn for _fn, _t, _v in _J3_AUDITS if site(_fn, _J7C, _t, full=True) is None})
    if any(fnkey(_fn, _J7C) for _fn in _j3_dropped) or not set(_j3_dropped) <= {"leaf_diff"}:
        raise SystemExit(f"B7b-J3: J3's dropped census sites changed; re-review: {_j3_dropped}")
    O10_SITE_SIZE = list(globals().get("O10_SITE_SIZE", [])) + [(_k, 0, _J3W) for _k in (
        site(_fn, _J7C, _t, row=_row) for _fn, _t, _row in _J3_SPECS) if _k is not None]
    O10_ID_AUDIT = O10_ID_AUDIT + [(_k, _v) for _k, _v in (
        (site(_fn, _J7C, _t, full=True), _v) for _fn, _t, _v in _J3_AUDITS) if _k is not None]

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
    # T4-U6/U8 (epoch addbdc030e): a file the crate includes only through `#[cfg(test)] #[path = ".."] mod ..;` is test
    # code, absent from every product build, so its calls are not product callers (arc_marker_tests.rs, the realized-arc
    # marker's reader tests, calls with composite = true). A `#[path]` module without `#[cfg(test)]`, or a plain `mod`
    # named like such a file's stem, stops the run.
    _j5_srcs = {}
    for _d, _, _fs in _os.walk(_os.path.join(_SNAP, RE_)):
        for _f in sorted(_fs):
            if _f.endswith(".rs"):
                _j5_srcs[_os.path.normpath(_os.path.join(_d, _f))] = open(_os.path.join(_d, _f), encoding="utf-8").read()
    _j5_test_only = set()
    for _path, _src in _j5_srcs.items():
        for _attrs, _p in _re.findall(r'((?:#\[[^\]]*\]\s*)*)#\[path\s*=\s*"([^"]+)"\]\s*mod\s+\w+\s*;', _src):
            if "#[cfg(test)]" not in _attrs:
                raise SystemExit(f"J5: {_p} is included as a module outside #[cfg(test)]; re-review")
            _j5_test_only.add(_os.path.normpath(_os.path.join(_os.path.dirname(_path), _p)))
    for _src in _j5_srcs.values():
        for _t in _j5_test_only:
            for _m in _re.finditer(r"\bmod\s+" + _re.escape(_os.path.basename(_t)[:-3]) + r"\s*;", _src):
                if not _re.search(r'#\[path\s*=\s*"[^"]+"\]\s*$', _src[:_m.start()]):
                    raise SystemExit(f"J5: {_t} is also a plain module; re-review")
    for _path, _src in sorted(_j5_srcs.items()):
        if _path in _j5_test_only:
            continue
        _j5_calls += [(_os.path.basename(_path), _m)
                      for _m in _re.findall(r"validate_physics_evidence_in\(([^,]*),\s*(true|false)\b", _src)]
    # B7b-J5: v2's deletion removes validate_physics_evidence's call; validate_pressure_evidence's composite = false
    # call remains (route E's reached caller since D4 (a)), so the prepass edge stays untaken on both routes.
    if sorted((_f, _m[1]) for _f, _m in _j5_calls) != ([("load_reference.rs", "false"), ("physics_evidence.rs", "false"),
                                                        ("physics_source.rs", "true")] if B7B_EXACT_BASE else
                                                       [("load_reference.rs", "false"), ("physics_evidence.rs", "false"),
                                                        ("physics_evidence.rs", "false"), ("physics_source.rs", "true")]):
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

# NLKD5 (DEL-04-01 kd5-nonlinear-active-set.md section 3.3, section 4): the
# nonlinear K-D5 confirmation. The Passed evidence line finds the case's one
# integrity diagnostic by id in the case's diagnostic Vec (one scan, <= D, as
# KD5L8). The confirmation's gate-refusal reason prints the gate's
# Option<f64> reciprocal condition estimate with Debug: "None", or "Some(" +
# an f64 Debug (<= 24 B, composite_text F64D) + ")", <= 30 B.
_NI_CONFIRMATION = "core/solver/nonlinear_integration/src/confirmation.rs"
if fnkey("confirm_with_applied", _NI_CONFIRMATION):
    O10_LOOPS_LAST += [
        ("iter:diagnostics.iter_mut().find", "D", O10 + "NLKD5L1: the evidence line's one scan of the case's diagnostic Vec, <= the full diagnostic fixpoint D", ["lib.rs:" + _CASE_FN]),
    ]
    O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [
        {"re": r"^gate\.report\.reciprocal_condition_estimate$", "max": 30, "why": O10 + "NLKD5A1: StructuralReport.reciprocal_condition_estimate is an f64; its Debug is <= 24 B, within the 30 B cap"},
    ]

# B7b-CA (model 0.5.0's exact half): the explicit corrosion allowance of a pipe section. normalize_model_units converts a
# present allowance like mill_tolerance (its unit-conversion diagnostic ID and refs), and derive_pipe_section refuses an
# invalid or nonzero allowance by name (two diagnostics, each followed by `return None`). Every identifier is the pipe's
# admitted ID (<= 128 B, D1.9 TEXT_BYTES); stable_suffix replaces ':' with '-' and keeps the byte length (as the
# mill-tolerance siblings' carried IN128 audits). The multiplicity of each site is the chain's own (per pipe and call).
if fnkey("derive_pipe_section", PP_ + "lib.rs") and fnkey("normalize_model_units", PP_ + "lib.rs"):
    _LIB = PP_ + "lib.rs"
    _lib_src = open(_os.path.join(_SNAP, _LIB), encoding="utf-8").read()
    if "corrosion_allowance" in _lib_src:
        O10_ID_AUDIT = O10_ID_AUDIT + [
            (site("normalize_model_units", _LIB, '"diagnostic:unit-conversion:pipe:{}:corrosion-allowance", stable_suffix(&pipe.id)', full=True),
             {"stable_suffix(&pipe.id)": IN}),
            (site("normalize_model_units", _LIB, 'vec![pipe.id.clone(), "corrosion_allowance".to_string()],', full=True),
             {"pipe.id": IN}),
        ]
        _dps = open(_os.path.join(_SNAP, _LIB), encoding="utf-8").read().split("\n")
        _dps_start = int(fnkey("derive_pipe_section", _LIB).split(":")[1])
        _dps_end = next(i + 1 for i in range(_dps_start, len(_dps)) if _re.match(r"(pub(\([a-z]+\))?\s+)?fn\s", _dps[i]))
        _ca_fmt = [i + 1 for i in range(_dps_start - 1, _dps_end - 1)
                   if '&format!("diagnostic:section:{}:corrosion-allowance", stable_suffix(pipe_id)),' in _dps[i]]
        _ca_refs = [i + 1 for i in range(_dps_start - 1, _dps_end - 1)
                    if 'vec![pipe_id.to_string(), "corrosion_allowance".to_string()],' in _dps[i]]
        # U6-DPS (T4-U6, D-5): D-5 admits a nonzero allowance, so derive_pipe_section's nonzero refusal
        # (EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED) is gone and one corrosion-allowance diagnostic (the
        # invalid-value refusal) remains, with the same ID, refs and IN128 classes. Only D-5 source may drop it.
        _ca_refused = any("EXACT_PRESSURE_CORROSION_ALLOWANCE_UNSUPPORTED" in _l for _l in _dps[_dps_start - 1:_dps_end - 1])
        if not _ca_refused and not fnkey("section_basis_row", _LIB):
            raise SystemExit("U6-DPS: derive_pipe_section lost its nonzero-allowance refusal without D-5's section bases; re-review")
        _ca_count = 2 if _ca_refused else 1
        if len(_ca_fmt) != _ca_count or len(_ca_refs) != _ca_count:
            raise SystemExit(f"B7b-CA: derive_pipe_section's corrosion-allowance diagnostics changed; re-review: {_ca_fmt} {_ca_refs}")
        O10_ID_AUDIT = O10_ID_AUDIT + [(Resolved(f"{_LIB}:{_n}"), {"stable_suffix(pipe_id)": IN}) for _n in _ca_fmt] \
            + [(Resolved(f"{_LIB}:{_n}"), {"pipe_id": IN}) for _n in _ca_refs]
        # B7b-DPS: B7b's derive_pipe_section (the function its corrosion-allowance refusals changed) pushes at most one
        # diagnostic per call. Checked premise, exactly: (a) every `diag(` in its body is a `diagnostics.push(diag(..));`
        # statement; (b) each such statement is immediately followed by `return None;`; (c) after the signature, the
        # `diagnostics` parameter appears only in those statements (no other push and no callee receives it). Pushes by
        # other functions (its callers' own sites) are their own sites, priced separately. Its four sites are therefore one
        # exclusive group (text_budget B7b-DX): D and the retained diagnostic bytes pay one diagnostic per call, at the
        # largest member's bytes; each site's own TEXT request, ID format and refs stay priced at every site.
        _dps_body = _dps[_dps_start - 1:_dps_end - 1]
        if sum(l.count("diag(") for l in _dps_body) != sum(l.count("diagnostics.push(diag(") for l in _dps_body) \
                or any("diagnostics.push(" in l and "diagnostics.push(diag(" not in l for l in _dps_body):
            raise SystemExit("B7b-DPS: derive_pipe_section has a diagnostic producer other than a pushed diag(..); re-review")
        _dps_sig = next(_i for _i, _l in enumerate(_dps_body) if _l.rstrip().endswith("{"))
        if any(len(_re.findall(r"\bdiagnostics\b", _l)) != _l.count("diagnostics.push(diag(") for _l in _dps_body[_dps_sig + 1:]):
            raise SystemExit("B7b-DPS: derive_pipe_section uses `diagnostics` other than in a pushed diag(..); re-review")
        _dps_sites = []
        for _i, _l in enumerate(_dps_body):
            if "diagnostics.push(diag(" not in _l:
                continue
            _j, _depth = _i, 0
            while True:
                _depth += _dps_body[_j].count("(") - _dps_body[_j].count(")")
                if _depth <= 0:
                    break
                _j += 1
            _next = next(x.strip() for x in _dps_body[_j + 1:] if x.strip())
            if not _dps_body[_j].rstrip().endswith(");") or _next != "return None;":
                raise SystemExit(f"B7b-DPS: a derive_pipe_section diagnostic is not followed by `return None;`: line {_dps_start + _i}")
            _dps_sites.append(_dps_start + _i)
        # U6-DPS: the same checked premise (a)-(c) holds on D-5 source, whose body keeps three of B7b's four sites
        # (section dimensions, mill tolerance, corrosion allowance invalid); the group is still one diagnostic per call.
        if len(_dps_sites) != (4 if _ca_refused else 3):
            raise SystemExit(f"B7b-DPS: derive_pipe_section's diagnostic inventory changed; re-review: {_dps_sites}")
        # Snapshot keys (Resolved): the decision-key carry leaves them as written.
        O10_DIAG_EXCLUSIVE = list(globals().get("O10_DIAG_EXCLUSIVE", [])) + [(
            "B7b-DPS", Resolved(f"lib.rs:{_dps_start}:derive_pipe_section"),
            [Resolved(f"lib.rs:{_n}") for _n in _dps_sites],
            ("B7b-DPS: derive_pipe_section's four diagnostics (section dimensions, mill tolerance, corrosion allowance "
             "invalid, corrosion allowance nonzero) are each" if _ca_refused else
             "U6-DPS (B7b-DPS on D-5 source): derive_pipe_section's three diagnostics (section dimensions, mill "
             "tolerance, corrosion allowance invalid) are each")
            + " a `diagnostics.push(diag(..));` immediately followed by "
            "`return None;`, and `diagnostics` is used nowhere else in the body: at most one per call")]

# B7b-ZA (ccb19ba0bf): retained_precision's namespace predicates (exact_namespace, preview_namespace) call
# section_allowances, whose one loop visits each of the captured model's pipe_segments once (`list(..).iter().all`,
# short-circuiting): <= m per call. On the exact route each visit converts the section's corrosion_allowance once
# (unit_value: its Ok/Err paths are priced at their own sites per call); on the preview route it reads a key only.
_RP = RE_ + "retained_precision.rs"
if fnkey("section_allowances", _RP):
    _rp_src = open(_os.path.join(_SNAP, _RP), encoding="utf-8").read()
    _sa = _rp_src.split("fn section_allowances(", 1)[1].split("\n}\n", 1)[0]
    # O4 S5 (retained-pricing-rules.md §8.9): the exact arm converts the allowance, the wall and (when present) the
    # mill tolerance, three unit_value call sites in the same one visit per pipe; the loop is unchanged.
    if _sa.count("list(&model[\"pipe_segments\"]).iter().all(") != 1 or _sa.count("unit_value(") not in (1, 3) or "for " in _sa:
        raise SystemExit("B7b-ZA: section_allowances changed; re-review its loop and conversions")
    O10_LOOPS_LAST += [('iter:list(&model[" "]).iter().all', "m",
        O10 + "B7b-ZA: section_allowances visits each captured pipe segment once (D1.9: pipes <= m), one unit_value "
        "per visit on the exact route (O4: three, the allowance, wall and mill tolerance, at their own call sites)",
        ["retained_precision.rs:section_allowances"])]

# B7b-JR (b4447b7212): joint::pressure_finding may add a fourth ref (the listed replaced span: an admitted pipe ID,
# <= 128 B) through `refs.extend(extra.map(str::to_string))`; the refs Vec then holds <= 4 Strings in capacity <= 6,
# beyond the generic `refs` class (4*(24+128)). Its multiplicity is 0 on the D1 graph: classify_connectors calls it in
# the loop over each case's pressure_regions' member_connector_ids (regions absent on L and explicitly empty on E, D1.5)
# and through classify_pressure_participation, reached only inside its loop over model.components (P2, D1.4). That zero
# is guarded rather than assumed: o10_check.py fails the run if the diag site's own multiplicity is ever positive.
if fnkey("pressure_finding", PP_ + "joint.rs") and fnkey("classify_pressure_participation", PP_ + "joint.rs"):
    _jr = open(_os.path.join(_SNAP, PP_ + "joint.rs"), encoding="utf-8").read()
    if "refs.extend(extra.map(str::to_string));" in _jr:
        O10_MULT0_SITES = list(globals().get("O10_MULT0_SITES", [])) + [
            (site("pressure_finding", PP_ + "joint.rs", "diagnostics.push(diag("),
             O10 + "B7b-JR: pressure_finding (<= 4 refs, capacity <= 6) has multiplicity 0 by D1.5 and P2 (D1.4)")]


# ---- T4-U6/U8 (DEL-05-03 Design section-bases-and-timoshenko.md §9; retained-pricing-rules.md, "T4-U6/U8: actual-source
# D3"). Each rule is added only where its source exists.
def fn_lines(name, f):
    """(lines, start, end) of the one `fn name` in file f: 1-based start, exclusive end (the next fn at the same or
    lower indent), as site() reads it."""
    k = fnkey(name, f)
    if not k:
        raise SystemExit(f"T4-U6: fn {name} is not unique in {f}")
    lines = open(_os.path.join(_SNAP, f), encoding="utf-8").read().split("\n")
    start = int(k.split(":")[1]); ind = len(lines[start - 1]) - len(lines[start - 1].lstrip())
    end = next((i + 1 for i in range(start, len(lines)) if _re.match(r"\s*(pub(\([a-z]+\))?\s+)?(const\s+)?fn\s", lines[i])
                and len(lines[i]) - len(lines[i].lstrip()) <= ind), len(lines) + 1)
    return lines, start, end


def site_is(name, f, text, nth=1, count=1):
    """The nth (1-based) of exactly `count` lines of fn `name` in file f whose stripped text is exactly `text`."""
    lines, start, end = fn_lines(name, f)
    hits = [i + 1 for i in range(start - 1, end - 1) if lines[i].strip() == text]
    if len(hits) != count:
        raise SystemExit(f"T4-U6 site_is(): {text!r} in {f}:{name}: {hits} (expected {count})")
    return Resolved(f"{f}:{hits[nth - 1]}")


def site_after(name, f, anchor, text, within=12):
    """The first line holding `text` within `within` lines after the one line of fn `name` holding `anchor`."""
    lines, start, end = fn_lines(name, f)
    a = [i + 1 for i in range(start - 1, end - 1) if anchor in lines[i]]
    if len(a) != 1:
        raise SystemExit(f"T4-U6 site_after(): anchor {anchor!r} in {f}:{name}: {a}")
    hits = [i + 1 for i in range(a[0], min(a[0] + within, end - 1)) if text in lines[i]]
    if not hits:
        raise SystemExit(f"T4-U6 site_after(): {text!r} after {anchor!r} in {f}:{name}")
    return Resolved(f"{f}:{hits[0]}")


_LIB = PP_ + "lib.rs"
_D5 = bool(fnkey("section_basis_row", _LIB) and fnkey("push_basis_twins", _LIB))
# U6-K: the eight basis audit keys whose lines D-5 replaced inside the case solve (lib.rs 5387, 5403, 5412, 5497,
# 5498, 5503, 5529, 5536 at 2243be380b: the two stress-finding diagnostic loops, now one loop over the stress sites,
# once per basis; the retained-source and exact maximum rows, now one per basis through section_basis_row) leave
# base/text_args.g4.json. On earlier source they are re-added here by name with their basis classes (the registered
# trees reproduce); on D-5 source the replacement sites take the same classes. A row id that section_basis_row has
# extended by ':' and the basis token (<= 15 B) is TPLSITE 322 + 16 = 338.
_STRESS_DIAG = {"location.replace('_',\"-\")": {"bytes": 600, "source": "STATIC"}, "stable_suffix(&pipe.element_id)": IN}
_MAX_ID = {"load_case.id": IN, "load_case.id.len()": NUM_, "pipe.element_id": IN, "pipe.element_id.len()": NUM_}
_RID = {"bytes": 322, "source": "TPLSITE"}
_RID_D5 = {"bytes": 338, "source": "TPLSITE"}
if fnkey(_CASE_FN, _LIB) and not _D5:
    O10_ID_AUDIT += [
        (site_after(_CASE_FN, _LIB, 'for (location, stress) in [("end_i", &end_i_stress), ("end_j", &end_j_stress)] {', "&format!("), _STRESS_DIAG),
        (site_after(_CASE_FN, _LIB, "for (location, stress) in &station_stresses {", "&format!("), _STRESS_DIAG),
        (site_after(_CASE_FN, _LIB, "for (location, stress) in &station_stresses {", "vec![pipe.element_id.clone()],"), {"pipe.element_id": IN}),
        (site(_CASE_FN, _LIB, 'let result_id = format!("result:elastic-maximum:{}:{}:{}:{}", load_case.id.len(), load_case.id, pipe.element_id.len(), pipe.element_id);', full=True), _MAX_ID),
        (site(_CASE_FN, _LIB, 'results.push(ResultItem { id: result_id.clone(), kind: "pipe_elastic_normal_stress_maximum_v2".into()', full=True), {"pipe.element_id": IN, "result_id": _RID}),
        (site(_CASE_FN, _LIB, 'max_stress = Some(LocatedQuantity { value, unit: "Pa".into(), location_ref: pipe.element_id.clone(), result_ref: result_id.clone() });', full=True), {"pipe.element_id": IN, "result_id": _RID}),
        (site_is(_CASE_FN, _LIB, "let result_id = format!("), _MAX_ID),
        (site(_CASE_FN, _LIB, 'results.push(ResultItem { id:result_id.clone(),kind:"pipe_elastic_normal_stress_maximum_v2".to_string()', full=True), {"pipe.element_id": IN, "result_id": _RID}),
    ]
U6_D5 = _D5
O10_LOOPS_FIRST, U6_BOTH_E, U6_BOTH_L, U6_LOOPS_E, U6_LOOPS_L = [], [], [], [], []


def _text_of(f):
    return open(_os.path.join(_SNAP, f), encoding="utf-8").read()


def _production(f):
    return _text_of(f).split("#[cfg(test)]", 1)[0]


def _fn_text(name, f):
    lines, start, end = fn_lines(name, f)
    return "\n".join(lines[start - 1:end - 1])


def _literals(text):
    """The raw bytes of each string literal in `text` (a raw literal's length bounds its decoded one)."""
    return [len(l.encode()) for l in _re.findall(r'"((?:[^"\\]|\\.)*)"', text)]


def _const_len(f, decl):
    """The declared length N of the one `decl: [T; N]` in file f."""
    src = _text_of(f)
    m = _re.findall(_re.escape(decl) + r":\s*\[[^;\]]+;\s*(\d+)\]", src)
    if len(m) != 1:
        raise SystemExit(f"U6-L: {decl} is not one array constant in {f}: {m}")
    return int(m[0])


def _calls_with_literal_last_arg(src, callee):
    """The last argument of every call `callee(..)` in src (not its definition); each must be one string literal."""
    out = []
    for m in _re.finditer(r"(?<![\w.:])" + _re.escape(callee) + r"\(", src):
        if src[max(0, m.start() - 3):m.start()] == "fn ":
            continue
        i, depth, args, cur = m.end(), 1, [], ""
        while depth:
            ch = src[i]
            if ch == '"':
                j = i + 1
                while src[j] != '"':
                    j += 2 if src[j] == "\\" else 1
                cur += src[i:j + 1]; i = j + 1; continue
            depth += ch in "([{"; depth -= ch in ")]}"
            if ch == "," and depth == 1:
                args.append(cur); cur = ""
            elif depth:
                cur += ch
            i += 1
        args = [a.strip() for a in args + [cur] if a.strip()]
        lits = _literals(args[-1])
        if not _re.fullmatch(r'"(?:[^"\\]|\\.)*"', args[-1]) or len(lits) != 1:
            raise SystemExit(f"U6-A: a {callee}() call's last argument is not one string literal: {args[-1][:80]!r}")
        out.append(lits[0])
    return out


if _D5:
    # ---- U6-L (T4-U6, D-5): the loops D-5 adds or re-shapes. Each bound is read from the code it names; the
    # constants' declared lengths are checked here, and o10_check.py confines each header to the functions listed.
    _GEOM = PP_ + "pressure_exact/source_geometry.rs"
    _SBR = RE_ + "section_basis.rs"
    _PEV = RE_ + "physics_evidence.rs"
    _RPX = RE_ + "retained_precision.rs"
    _RPP = PP_ + "retained_product.rs"
    _COMP = PP_ + "source_receipt/composite.rs"
    _ROWS = PP_ + "source_receipt/rows.rs"
    _CPUB = PP_ + "certified_publication.rs"
    _FC = FK_ + "structural/formation_check.rs"
    _CR = FK_ + "structural/certified_recovery.rs"
    if (_const_len(_GEOM, "pub(crate) const BOTH"), _const_len(_SBR, "pub const BASES"), _const_len(_SBR, "pub const ANNULUS_KEYS"),
            _const_len(_RPP, "const PREPARED_SECTION_BASES")) != (2, 2, 6, 2):
        raise SystemExit("U6-L: a section-basis constant's length changed; re-review")
    # U6-L1: stress_sites is a vec! of the two ends plus one push per station of station_stresses (<= 3: KD5L7's
    # bound on `for (index, (location, stress)) in station_stresses.iter().enumerate()`); `bases` is a vec! of one
    # entry (no reduced stress: the preview route) or two (nominal, reduced).
    _cs, _cs0, _cs1 = fn_lines(_CASE_FN, _LIB)
    _cs_body = [l.strip() for l in _cs[_cs0 - 1:_cs1 - 1]]
    _i = _cs_body.index("let mut stress_sites = vec![")
    if not (_cs_body[_i + 1].startswith('("end_i", 0usize, true, &end_i_stress,') and _cs_body[_i + 2].startswith('("end_j", 4, true, &end_j_stress,')
            and _cs_body[_i + 3] == "];" and _cs_body[_i + 4] == "for (index, (location, stress)) in station_stresses.iter().enumerate() {"
            and _cs_body[_i + 5].startswith("stress_sites.push((*location, index + 1, false, stress,") and _cs_body[_i + 6] == "}"
            and sum("stress_sites.push(" in l for l in _cs_body) == 1):
        raise SystemExit("U6-L1: the case solve's stress_sites changed; re-review")
    _i = _cs_body.index("let bases = match reduced {")
    if not (_cs_body[_i + 1] == "None => vec![(stress, None)],"
            and _cs_body[_i + 2] == "Some(reduced) => vec![(stress, Some(pressure_exact::SectionBasis::Nominal)), (reduced, Some(pressure_exact::SectionBasis::Reduced))],"):
        raise SystemExit("U6-L1: the stress-finding bases changed; re-review")
    # U6-L2: push_basis_twins zips the rows one `push` call made on each basis (checked equal in length). Its two
    # callers' closures append one stress site's rows through exactly one helper, each <= 4 rows: certified
    # append_stress_mpa pushes once per entry of its 4-tuple array; append_endpoint_stress_results and
    # append_station_stress_results push at most once per entry of their 4-tuple local_components.
    _pbt = _fn_text("push_basis_twins", _LIB)
    if not (_pbt.count("push(&mut nominal, SectionBasis::Nominal)?;") == 1 and _pbt.count("push(&mut reduced, SectionBasis::Reduced)?;") == 1
            and "let twins = nominal.len() == reduced.len()" in _pbt):
        raise SystemExit("U6-L2: push_basis_twins changed; re-review")
    _pbt_calls = sum(_text_of(PP_ + _f).count("push_basis_twins(") for _f in ("lib.rs", "source_receipt/rows.rs"))
    if _pbt_calls != 2 or _text_of(_ROWS).count("push_basis_twins(") != 1 or _text_of(_LIB).count("fn push_basis_twins<E>(") != 1:
        raise SystemExit("U6-L2: push_basis_twins's callers changed; re-review")
    for _h, _f, _push in (("append_stress_mpa", _CPUB, "results.push(ResultItem {"),
                          ("append_endpoint_stress_results", _LIB, "append_endpoint_stress_result("),
                          ("append_station_stress_results", _LIB, "append_station_stress_result(")):
        _ht = _fn_text(_h, _f)
        if len(_re.findall(r'"element_local_[a-z_]+_stress(?:_[yz])?"', _ht)) != 4 or _ht.count(_push) != 1:
            raise SystemExit(f"U6-L2: {_h} no longer appends <= 4 rows; re-review")
    # U6-L3: case_observables' headlines: the displacement headline plus one stress headline per entry of
    # stress_bases, a slice literal of one (preview) or two (exact) entries.
    _co = _fn_text("case_observables", _RPP)
    if not ("&[Some(pressure_exact::SectionBasis::Nominal), Some(pressure_exact::SectionBasis::Reduced)]" in _co
            and "&[None]" in _co and "let headlines = std::iter::once((view.headline(false,None)" in _co
            and ".chain(stress_bases.iter().map(" in _co):
        raise SystemExit("U6-L3: case_observables' headlines changed; re-review")
    # U6-L4: section_basis::coverage returns [(Vec<&str>, bool); 2]; physics_evidence zips that array.
    if "pub fn coverage(v: &Value) -> Result<[(Vec<&str>, bool); 2], String>" not in _text_of(_SBR):
        raise SystemExit("U6-L4: section_basis::coverage's array changed; re-review")
    # U6-L5: the rows of one case or combination entry, reduced twins included (retained_precision rows_all_for:
    # list(results) filtered by the entry's basis_ref), are one block of the route's P rows, as rows_for's were
    # (`for row in rows_for(source, c)` and `.. e` bound P); D-5's twins are inside P (row premise, U6-RP).
    if "list(&source[\"results\"])\n        .iter()\n        .filter(|r| r[\"basis_ref\"] == case[\"basis_ref\"])\n        .collect()" not in _fn_text("rows_all_for", _RPX) \
            or not any(_f in _fn_text("rows_for", _RPX) for _f in (
                "rows_all_for(source, case).into_iter().filter(|r| !reduced_twin(r)).collect()",
                # O4 S5 (§8.9, O4-RD): a distinct-basis member's reduced rows stay in the universe; still a filter of
                # rows_all_for's one block, so <= the entry's published rows.
                "rows_all_for(source, case)\n        .into_iter()\n        .filter(|r| !reduced_twin(r) || distinct.contains(text(&r[\"entity_ref\"])))\n        .collect()")) \
            or "all_rows: rows_all_for(source, c)," not in _fn_text("numeric_owner", _RPX) \
            or "let all = rows_all_for(source, c.case);" not in _fn_text("g5b_exact_evidence", _RPX):
        raise SystemExit("U6-L5: a case's row lists changed; re-review")
    # U6-L6: one B7a formed term carries one supplied primitive (attach_last); certified_loads::attach with more
    # than one source runs only in push_exact_pressure_operands, over exact.assembled_operands (empty on D1:
    # route E's LOOP_ZERO_RESTATE `exact\.assembled_operands`; no exact case on route L). The formed force's
    # terms and the formation source's loads are <= q = 12*l (B7a, b7a_formation_model.lengths).
    _lib_lines = _text_of(_LIB).split("\n")
    _attach_owners = []
    for _n, _l in enumerate(_lib_lines):
        if "certified_loads::attach(" in _l:
            _o = next(j for j in range(_n, -1, -1) if _re.match(r"\s*(pub(\([a-z]+\))?\s+)?fn\s", _lib_lines[j]))
            _attach_owners.append("test" if _lib_lines[_o - 1].strip() == "#[test]" else _re.search(r"fn\s+(\w+)", _lib_lines[_o]).group(1))
    if _text_of(PP_ + "certified_loads.rs").count("attach(ledger, ledger.term_count() - 1, vec![source]);") != 1 \
            or sorted(set(_attach_owners) - {"test"}) != ["push_exact_pressure_operands"] or _attach_owners.count("push_exact_pressure_operands") != 1 \
            or "certified_loads::attach(ledger, ledger.term_count() - 1, expected_sources);" not in _fn_text("push_exact_pressure_operands", _LIB):
        raise SystemExit("U6-L6: the formed-term primitive attach changed; re-review")
    O10_LOOPS_LAST += [
        ("for &(location, _, _, stress, reduced) in &stress_sites", "5", O10 + "U6-L1: the two ends plus <= 3 stations", ["lib.rs:" + _CASE_FN]),
        ("for &(location, index, endpoint, stress, reduced) in &stress_sites", "5", O10 + "U6-L1: the two ends plus <= 3 stations", ["lib.rs:" + _CASE_FN]),
        ("for (n, r) in nominal.into_iter().zip(reduced)", "4", O10 + "U6-L2: one stress site's rows on one basis, <= 4", ["lib.rs:push_basis_twins"]),
        ("for (headline, kind, basis) in headlines", "3", O10 + "U6-L3: the displacement headline and <= 2 stress headlines", ["retained_product.rs:case_observables"]),
        ("for basis in PREPARED_SECTION_BASES", "2", O10 + "U6-L: PREPARED_SECTION_BASES is [&str; 2]",
         ["retained_product.rs:prepared_sections", "retained_product.rs:apply_exact_section_overlay"]),
        ("for (i,(r,bound)) in rows.iter().zip(bound_row_indices(route,rows)).enumerate()", "P",
         O10 + "U6-L5: prepared_alias's rows are the frozen case's own rows (its scope's block, <= P), as before D-5", ["retained_product.rs:prepared_alias"]),
        ("for (slot, key) in leaves.iter_mut().zip(ANNULUS_KEYS)", "6", O10 + "U6-L: ANNULUS_KEYS is [&str; 6]", ["section_basis.rs:annulus"]),
        ("for ((entry, basis), slot) in entries.iter().zip(BASES).zip(out.iter_mut())", "2", O10 + "U6-L: BASES is [&str; 2]", ["section_basis.rs:coverage"]),
        ("iter:BASES .into_iter().find", "2", O10 + "U6-L: BASES is [&str; 2]", ["section_basis.rs:id_basis"]),
        ("for b in BASES", "2", O10 + "U6-L: BASES is [&str; 2]", ["section_basis.rs:twin_id"]),
        ('iter:summary.get(" ").is_some_and(Value::is_null) && BASES.iter().all', "2", O10 + "U6-L: BASES is [&str; 2]", ["section_basis.rs:headlines"]),
        ("for (basis, complete) in BASES.into_iter().zip(complete)", "2", O10 + "U6-L: BASES is [&str; 2]", ["section_basis.rs:headlines"]),
        ("for basis in crate::section_basis::BASES", "2", O10 + "U6-L: BASES is [&str; 2]", ["physics_source.rs:maximum_link"]),
        ("for (slot, ((unavailable, complete), done)) in coverage .iter() .zip(coverage_complete.iter_mut()) .enumerate()", "2",
         O10 + "U6-L4: section_basis::coverage returns a [_; 2]", ["physics_evidence.rs:validate_physics_evidence_in"]),
        ("for ((case, member, _, basis), values) in &lame", "Pall",
         O10 + "U6-L: lame holds at most one entry per Lame row of the validated results, <= Pall", ["physics_evidence.rs:validate_physics_evidence_in"]),
        ("for e in combos", "z", O10 + "U6-L: g3's body.combinations, one entry per model combination. D1.4 (B2-A): z <= 2", ["retained_precision.rs:g3"]),
        ("iter:rows_all_for(source, case).into_iter().filter", "P", O10 + "U6-L5: one entry's rows, <= P", ["retained_precision.rs:rows_for"]),
        ("for row in &c.all_rows", "P", O10 + "U6-L5: one selected case's rows, twins included, <= P", ["retained_precision.rs:g5c"]),
        ("for (i, row) in all.iter().enumerate()", "P", O10 + "U6-L5: one selected case's rows, twins included, <= P", ["retained_precision.rs:g5b_exact_evidence"]),
        ("for row in rows_all_for(source, c)", "P", O10 + "U6-L5: one case's rows, twins included, <= P", ["retained_precision.rs:gates_after_g0"]),
        ("for row in rows_all_for(source, e)", "P", O10 + "U6-L5: one combination's rows, twins included, <= P",
         ["retained_precision.rs:r_comb_1", "retained_precision.rs:gates_after_g0"]),
        ('iter:list(&model[" "]).iter().any', "m", O10 + "U6-L: exact_mill_tolerances visits each captured pipe segment once (D1.9: pipes <= m)",
         ["retained_precision.rs:exact_mill_tolerances"]),
        ("iter:source .loads .iter().find", "12*l", O10 + "U6-L6: the formation source's load primitives, <= q = 12*l (B7a)",
         ["structural/formation_check.rs:unbound_straight_primitive"]),
        ("for (index, term) in force .terms() .iter() .enumerate() .filter(|(_, term)| term.dof == dof)", "12*l",
         O10 + "U6-L6: the formed force's terms, <= q = 12*l (B7a)", ["structural/certified_recovery.rs:certify_support_reaction"]),
        ("for primitive in source.loads.iter().filter(|p| p.term_index == index)", "1",
         O10 + "U6-L6: the one supplied primitive of a formed term (B7a body1); the scan's predicate allocates nothing",
         ["structural/certified_recovery.rs:certify_support_reaction"]),
    ]
    # U6-LF (placed first): three D-5 headers name pressure_exact::SectionBasis::BOTH, so D1.5's region zero rule
    # (`pressure|region|aggregates`, which matches `pressure`) would capture them; they are not region loops. BOTH is
    # [SectionBasis; 2]. Route E iterates both bases. On route L each of the case solve's three BOTH loops sits
    # (checked) in an `is_exact(model)` arm (the two maxima) or in `if let Some(state) = exact_pressure ..
    # .pipe_states.get(..)` (the wall action), and route L's model has no exact contract (D1.3 branch L): is_exact
    # is false and build_pressure_case_with_members returns no exact case.
    def _openers(lines, i):
        """The block openers enclosing line i, innermost first (by indentation)."""
        ind, out, j = len(lines[i]) - len(lines[i].lstrip()), [], i - 1
        while j >= 0:
            l = lines[j]
            if l.strip() and len(l) - len(l.lstrip()) < ind:
                ind = len(l) - len(l.lstrip())
                if l.strip() == "{":   # a header broken over lines: its first line is the one above at this indent
                    j = next(k for k in range(j - 1, -1, -1) if lines[k].strip() and len(lines[k]) - len(lines[k].lstrip()) == ind)
                    l = lines[j]
                out.append(l.strip())
                if ind == 0:
                    break
            j -= 1
        return out
    _both = [i for i, l in enumerate(_cs_body) if l == "for basis in pressure_exact::SectionBasis::BOTH {"]
    for _ln in _both:
        if not any(_re.match(r"\} else if pressure_runtime::is_exact\(model\)( && selected_source\.is_some\(\))? \{$", o)
                   or o == "if let Some(state) = exact_pressure" for o in _openers(_cs, _cs0 - 1 + _ln)):
            raise SystemExit(f"U6-LF: a case-solve BOTH loop is not exact-only; re-review: line {_cs0 + _ln}")
    if len(_both) != 3:
        raise SystemExit("U6-LF: the case solve's BOTH loops changed; re-review")
    O10_LOOPS_FIRST += [
        ("for (record, basis) in coverage.iter().zip(pressure_exact::SectionBasis::BOTH)", "2",
         O10 + "U6-LF: zipped with BOTH ([SectionBasis; 2])", ["retained_product.rs:exact_case_evidence"]),
        ("for (pipe, basis) in model .pipe_segments .iter() .flat_map(|pipe| pressure_exact::SectionBasis::BOTH.map(|basis| (pipe, basis)))", "2*m",
         O10 + "U6-LF: each pipe segment (<= m) on both bases", ["source_receipt/composite.rs:validate_physics_rows"]),
    ]
    # U6-L1 (route-specific): the stress-finding `bases` has two entries only when the member has a reduced
    # stress, i.e. a reduced section: exact_basis_section of built.exact_sections, which build_model_for_members fills
    # only under `if pressure_runtime::is_exact(model)` (checked). Route L has no exact contract: one entry.
    _bm = _fn_text("build_model_for_members", _LIB)
    if _bm.count("exact_sections.insert(") != 2 or "        if pressure_runtime::is_exact(model) {" not in _bm \
            or "let reduced_section = exact_bases.map(|bases| exact_basis_section(bases, pressure_exact::SectionBasis::Reduced));" not in _cs_body \
            or "let exact_bases = built.exact_sections.get(&pipe.element_id).copied();" not in _cs_body \
            or "let reduced_stresses = match &reduced_section {" not in _cs_body:
        raise SystemExit("U6-L1: the reduced section's exact-only source changed; re-review")
    U6_LOOPS_E = [("for (stress, basis) in bases", "2", O10 + "U6-L1 (route E): one entry per section basis", ["lib.rs:" + _CASE_FN])]
    U6_LOOPS_L = [("for (stress, basis) in bases", "1", O10 + "U6-L1 (route L): no reduced section without an exact contract", ["lib.rs:" + _CASE_FN])]
    U6_BOTH_E = [("for basis in pressure_exact::SectionBasis::BOTH", "2", O10 + "U6-LF (route E): BOTH is [SectionBasis; 2]",
                  ["lib.rs:" + _CASE_FN, "source_receipt/composite.rs:composite_exact"])]
    U6_BOTH_L = [("for basis in pressure_exact::SectionBasis::BOTH", "0",
                  O10 + "U6-LF (route L): both case-solve loops are in `is_exact(model)` arms (checked); route L has no exact contract",
                  ["lib.rs:" + _CASE_FN])]

    # ---- U6-A (T4-U6/U8): the identifier-bearing and unclassified arguments of the D-5/D-6 sites, by site. Static
    # segments are bounded by their longest literal, read at the snapshot.
    def _const_lit(f, name):
        m = _re.findall(r"pub const " + name + r": &str = \"((?:[^\"\\]|\\.)*)\";", _text_of(f))
        if len(m) != 1:
            raise SystemExit(f"U6-A: {name} is not one &str const in {f}")
        return len(m[0].encode())
    def _st(n, why):
        return {"bytes": n, "source": O10 + "U6-A: " + why}
    _SEG = max(_literals(_fn_text("hyphen", _GEOM)) + _literals(_fn_text("token", _GEOM)))
    _SUFFIX = max(_literals(_fn_text("basis_suffix", _GEOM)))
    _B = max(_const_lit(_SBR, "NOMINAL"), _const_lit(_SBR, "REDUCED"))
    _H = max(_literals(_fn_text("hyphenated", _SBR)))
    _SEP = _const_lit(_SBR, "TOKEN_SEPARATOR")
    if "fn other(basis: &str) -> &'static str {\n    if basis == NOMINAL {\n        REDUCED\n    } else {\n        NOMINAL\n    }" not in _text_of(_SBR):
        raise SystemExit("U6-A: section_basis::other changed; re-review")
    _SEGA = _st(_SEG, "SectionBasis::hyphen/token, the longest literal")
    # The case solve's stress-finding id (lib.rs `let id = format!(`): 18 literal bytes, the IN128 pipe suffix, the
    # historic STATIC 600 location class, two separators and finding.code's Debug (<= 64, its argument rule).
    _DIAG_ID = 18 + 128 + 600 + 2 + 64
    _OC = __import__("runpy").run_path(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "..", "base", "b7a_owner_choices.py"))
    _EXPECTED = max(_OC["BOUNDS"].values())
    _ea = _text_of(PP_ + "exact_admission.rs")
    def _j4b(start, count):
        i = _ea.index(start); j = _ea.index("\n    };", i)
        lits = _literals(_ea[i:j])
        if len(lits) != count or _ea.count(start) != 1:
            raise SystemExit(f"U6-J4: {start!r} no longer selects exactly {count} literals; re-review")
        return max(lits)
    _sh = _production(FK_ + "shear.rs")
    _sh_finite = [m for m in _re.findall(r"(?<![\w.])finite\(\"((?:[^\"\\]|\\.)*)\",", _sh)]
    _sh_range = _re.findall(r"NumericalRange \{\s*name: \"((?:[^\"\\]|\\.)*)\",?\s*\}", _sh)
    if len(_re.findall(r"(?<![\w.])finite\(", _sh)) != len(_sh_finite) + 1 or len(_re.findall(r"NumericalRange \{", _sh)) != len(_sh_range) + 2:
        raise SystemExit("U6-A: shear.rs's &'static str names are not all literals; re-review")
    _SHN = max(len(n.encode()) for n in _sh_finite + _sh_range)
    _spl = _production("core/solver/straight_pipe/src/lib.rs")
    _sp_names = _re.findall(r"UnsupportedLoadForBeamTheory \{\s*name: \"((?:[^\"\\]|\\.)*)\",?\s*\}", _spl)
    if len(_re.findall(r"UnsupportedLoadForBeamTheory \{", _spl)) != len(_sp_names) + 2 or not _sp_names:
        raise SystemExit("U6-A: straight_pipe's UnsupportedLoadForBeamTheory names are not all literals; re-review")
    _F64 = _st(327, "an f64 in Display (the chain's f64_display class, 327)")
    _BMF = __import__("runpy").run_path(_os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "..", "base", "b7a_domain_model.py"))["MEMBER_SOURCE"]
    _CODES = _calls_with_literal_last_arg(_production(_SBR), "require")
    # the second maximum branch's multi-line headline: its result_ref is the fourth line after the opener
    _HLN = 2 if fnkey("published_nodal_displacements", _LIB) else 1
    _hl = int(site_is(_CASE_FN, _LIB, "*headline = Some(LocatedQuantity {").rsplit(":", 1)[1])
    if _cs[_hl + 3].strip() != "result_ref: result_id.clone(),":
        raise SystemExit("U6-A: the per-basis headline's result_ref moved; re-review")
    _HL_REF = Resolved(f"{_LIB}:{_hl + 4}")
    O10_ID_AUDIT += [
        # the case solve (D-5): the stress-finding warning once per basis, and the two maxima once per basis
        (site_after(_CASE_FN, _LIB, "for (stress, basis) in bases {", "let id = format!(", within=3), _STRESS_DIAG),
        (site(_CASE_FN, _LIB, '&basis.map_or_else(|| id.clone(), |basis| format!("{id}:{}", basis.hyphen())),', full=True),
         {"id": _st(_DIAG_ID, "the stress-finding id formatted just above (its four audited parts and 18 literal bytes)"), "basis.hyphen()": _SEGA}),
        (site(_CASE_FN, _LIB, 'let row = section_basis_row(ResultItem { id: format!("result:elastic-maximum:{}:{}:{}:{}", load_case.id.len(), load_case.id, pipe.element_id.len(), pipe.element_id)', full=True), _MAX_ID),
        # U4B-K (T4-U4b): a realized arc's certified maximum sets the per-basis headline with the same statement; its
        # result id is u4b_rows::arc_maximum_row's, the same elastic-maximum template with its basis segment (_RID_D5).
        *[(site_is(_CASE_FN, _LIB, '*headline = Some(LocatedQuantity { value, unit: "Pa".into(), location_ref: pipe.element_id.clone(), result_ref: result_id.clone() });',
                   nth=_k, count=_HLN), {"pipe.element_id": IN, "result_id": _RID_D5}) for _k in range(1, _HLN + 1)],
        (site(_CASE_FN, _LIB, 'diagnostics.push(diag(&format!("diagnostic:source-recovery:{}:{}:maximum:{}",load_case.id,pipe.element_id,basis.hyphen())', full=True),
         {"load_case.id": IN, "pipe.element_id": IN, "basis.hyphen()": _SEGA}),
        (site(_CASE_FN, _LIB, "let row = section_basis_row(ResultItem { id:format!(", full=True), _MAX_ID),
        (site(_CASE_FN, _LIB, '),kind:"pipe_elastic_normal_stress_maximum_v2".to_string(),value,unit:"Pa".to_string(),entity_ref:pipe.element_id.clone(),', full=True),
         {"pipe.element_id": IN}),
        (_HL_REF, {"result_id": _RID_D5}),
        (site_is(_CASE_FN, _LIB, "let result_id = row.id.clone();", nth=1, count=2), {"row.id": _RID_D5}),
        (site_is(_CASE_FN, _LIB, "let result_id = row.id.clone();", nth=2, count=2), {"row.id": _RID_D5}),
        # U4B-K: a realized arc's unavailable certified maximum names its member once more per basis (_HLN - 1 extra).
        *[(site_is(_CASE_FN, _LIB, "pressure_exact::SectionBasis::" + _b + " => unavailable_" + _l + "_maximum_members.push(pipe.element_id.clone()),",
                   nth=_k, count=_HLN + 1), {"pipe.element_id": IN})
          for _b, _l in (("Nominal", "stress"), ("Reduced", "reduced")) for _k in range(1, _HLN + 2)],
        (site(_CASE_FN, _LIB, 'diagnostics.push(diag(&id,"EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE","warning",', full=True),
         {"load_case.id": IN, "pipe.element_id": IN}),
        # RS g5c: a reduced twin's class takes the twin's own row id (a published result id, RES 1024, as the
        # nominal rows' carried audit in the same function)
        # (O4 S5, §8.9: the twin test also excludes a distinct-basis member's reduced rows; the same site follows)
        (site_after("g5c", _RPX, "let class = if reduced_twin(row) && !distinct.contains(text(&row[\"entity_ref\"])) {"
                    if "let class = if reduced_twin(row) && !distinct" in open(_os.path.join(_SNAP, _RPX), encoding="utf-8").read()
                    else "let class = if reduced_twin(row) {", 'result_id: text(&row["id"]).into(),', within=4),
         {'text(&row[""])': RS_}),
        # section_basis_row: the id's segment and the recovery token's suffix
        (site("section_basis_row", _LIB, "row.id.push_str(if hyphen_id { basis.hyphen() } else { basis.token() });", full=True),
         {"ifhyphen_id{basis.hyphen()}else{basis.token()}": _SEGA}),
        (site("section_basis_row", _LIB, "metadata.basis.push_str(basis.basis_suffix());", full=True),
         {"basis.basis_suffix()": _st(_SUFFIX, "SectionBasis::basis_suffix, the longest literal")}),
        # retained_product validate_final_metadata: the expected base recipe (B7a owner choices' max) and the segment
        # U4B-W1: a route-E derived (NotCovered) row's id takes the same hyphen segment as a stress row.
        (site("validate_final_metadata", _RPP, '(Some(basis), k::ProductRecipe::Stress { .. } | k::ProductRecipe::NotCovered { .. }) => format!("{expected}:{}", basis.hyphen()),'
              if fnkey("prepared_not_covered", _RPP) else '(Some(basis), k::ProductRecipe::Stress { .. }) => format!("{expected}:{}", basis.hyphen()),', full=True),
         {"expected": _st(_EXPECTED, "the expected base recipe, max of B7a owner choices' BOUNDS"), "basis.hyphen()": _SEGA}),
        (site("validate_final_metadata", _RPP, '(Some(basis), _) => format!("{expected}:{}", basis.token()),', full=True),
         {"expected": _st(_EXPECTED, "the expected base recipe, max of B7a owner choices' BOUNDS"), "basis.token()": _SEGA}),
        # RE section_basis: the basis names, the twin id, and the private require's codes (every call passes a literal)
        (site("require", _SBR, 'Err(format!("SOURCE_PHYSICS_{code}"))', full=True),
         {"code": _st(max(_CODES), "the longest literal code passed by any require() call in section_basis.rs (private)")}),
        (site("id_basis", _SBR, ".find(|b| id.ends_with(&format!(\":{b}\")) || id.ends_with(&format!(\":{}\", hyphenated(b))))", full=True),
         {"b": _st(_B, "a BASES literal"), "hyphenated(b)": _st(_H, "hyphenated's longest literal")}),
        (site("twin_id", _SBR, '(format!(":{b}"), format!(":{}", other(b))),', full=True),
         {"b": _st(_B, "a BASES literal"), "other(b)": _st(_B, "other() returns NOMINAL or REDUCED")}),
        (site("twin_id", _SBR, '(format!(":{}", hyphenated(b)), format!(":{}", hyphenated(other(b)))),', full=True),
         {"hyphenated(b)": _st(_H, "hyphenated's longest literal"), "hyphenated(other(b))": _st(_H, "hyphenated's longest literal")}),
        (site("twin_id", _SBR, 'return Some(format!("{stem}{to}"));', full=True),
         {"stem": _st(1024, "a prefix of the row id passed (RES, 1024)"), "to": _st(1 + max(_B, _H), "':' and one basis name")}),
        (site("reduced_twin", _RPX, "b.ends_with(&format!(\"{}{}\", crate::section_basis::TOKEN_SEPARATOR, crate::section_basis::REDUCED))", full=True),
         {"crate::section_basis::TOKEN_SEPARATOR": _st(_SEP, "the TOKEN_SEPARATOR literal"), "crate::section_basis::REDUCED": _st(_B, "a BASES literal")}),
        # D-6 (U8): the Timoshenko shear factor's diagnostic, errors and section map; the reaction's frame binding
        (site("build_model_for_members", _LIB, "exact_sections.insert(pipe.id.clone(), bases.with_timoshenko(kappa, shear_area));", full=True), {"pipe.id": IN}),
        (site("build_model_for_members", _LIB, 'diagnostics.push(diag(&format!("diagnostic:exact-section:{}:shear", stable_suffix(&pipe.id)),', full=True),
         {"stable_suffix(&pipe.id)": IN}),
        (site("build_model_for_members", _LIB, 'format!("a timoshenko_annulus_energy_v1 member requires {missing}; no default shear factor is used"), vec![pipe.id.clone()]));', full=True),
         {"pipe.id": IN}),
        (site("fmt", FK_ + "shear.rs", 'Self::NonFiniteInput { name, value } => write!(f, "{name} must be finite, got {value}"),', full=True),
         {"name": _st(_SHN, "a &'static str literal of shear.rs (finite() / NumericalRange)")}),
        (site("fmt", FK_ + "shear.rs", '"inner radius must satisfy 0 <= ri < ro, got ri = {inner_radius}, ro = {outer_radius}"', full=True, row="write!("),
         {"inner_radius": _F64, "outer_radius": _F64}),
        (site("fmt", FK_ + "shear.rs", "Self::NumericalRange { name } => write!(", full=True),
         {"name": _st(_SHN, "a &'static str literal of shear.rs (finite() / NumericalRange)")}),
        (site("fmt", "core/solver/straight_pipe/src/lib.rs", 'write!(f, "unsupported load for the element\'s beam theory: {name}")', full=True),
         {"name": _st(max(len(n.encode()) for n in _sp_names), "the UnsupportedLoadForBeamTheory name literal")}),
        (site("straight_frames_bound", _FC, "p.identity.source", full=True, row="format!("),
         {"p.identity.source": _st(_BMF, "a member_source identity (B7a MEMBER_SOURCE, 268); D1's formed primitives are attach_last's")}),
    ]
    # U6-J4: J4 at pressure_v3_formulation_basis_with_source (T4-U6 renamed it; `selected` is now a bool): identity
    # and source each choose one of two whole static literals.
    if fnkey("pressure_v3_formulation_basis_with_source", PP_ + "exact_admission.rs") and not fnkey("pressure_v3_formulation_basis_in", PP_ + "exact_admission.rs"):
        _EAF = PP_ + "exact_admission.rs"
        O10_ID_AUDIT += [
            (site("pressure_v3_formulation_basis_with_source", _EAF, "identity.to_string(),", full=True),
             {"identity": {"bytes": _j4b("let identity = if selected {", 2), "source": O10 + "U6-J4: the longer of the two static identity literals"}}),
            (site("pressure_v3_formulation_basis_with_source", _EAF, "source.to_string(),", full=True),
             {"source": {"bytes": _j4b("let source = if selected {", 2), "source": O10 + "U6-J4: the longer of the two static source literals"}}),
        ]

# U6-RB/W1-L5: the guarded row index has one insertion and one reader. The
# reduced cohort binds the production `combining` expression, both guards and
# its cfg(test)-only byte-comparison shadow. Route E has no preview; route L
# also omits the index and callee staging when z=0. Any missing/old guard in the
# reduced source fails before the points instead of silently losing U6-RB.
U6_RB_PATCHES, W1_L5_PATCHES, W1_L5_SITE_FROM = [], [], []
_AGG = fnkey("run_linear_static_preview_observed_inner", _LIB)
if _AGG:
    _W1_MODEL = __import__("runpy").run_path(_os.path.join(_os.path.dirname(_os.path.dirname(
        _os.path.abspath(__file__))), "base", "w1_reductions_model.py"))
    _W1_REDUCED = _W1_MODEL["inspect"](_SNAP)
    _agg = _fn_text("run_linear_static_preview_observed_inner", _LIB)
    _RB_GUARDED = ("            if preview.is_some() {\n                rows_by_base_id\n                    .entry(base_id)\n"
                   "                    .or_default()\n                    .insert(load_case_id.clone(), result.clone());\n            }\n")
    _RB_CALL = ("        Some(rendered) => preview_physics::append_combination_results(&model, rendered, &rows_by_base_id, "
                "&mut results, &mut diagnostics),\n")
    if _W1_REDUCED:
        _W1_MODEL["guard"](_agg)
    elif _RB_GUARDED not in _agg or _RB_CALL not in _agg:
        raise SystemExit("U6-RB: the RF guarded rows_by_base_id/call changed; re-review")
    if (_agg.count("rows_by_base_id") != 3
            or "    let preview = (!pressure_runtime::is_exact(&model)).then(|| {\n" not in _agg
            or _agg.count("let preview =") != 1 or _agg.count("append_combination_results(") != 1
            or _agg.count("    let mut results = Vec::new();\n") != 1 or _agg.count("let mut results") != 1):
        raise SystemExit("U6-RB: preview/index uses or push-built results changed; re-review")
    _AGG_ROW = ('    (Xc("ResultItem", PA) + tx("row") * PA) * 2 + X({"Node(String,BTreeMap)": PA}) '
                '+ X({"Node(String,ResultItem)": PA})\n')
    _NO_INDEX = '    V("ResultItem", PA) + tx("row") * PA'
    U6_RB_PATCHES = [("ordinary_caps.py", _AGG_ROW,
        _NO_INDEX + '   # U6-RB/W1-L5: route E has no preview/index; push-capacity results (U6-RB-V)\n',
        "U6-RB: route E has no preview, so no row index clone or map; keep push-capacity results")]
    if _W1_REDUCED:
        # The full preview_physics.rs pin includes append_combination_results:
        # base_ids and case_rows are its only pre-loop construction; results,
        # diagnostics and gates are changed only in its combinations loop.
        _L5_ANCHOR = '    + HashReq("(String,String)", PA) + B(2 * PA * ID))\n'
        _L5_FAMILY = (
            '# W1-L5: the combination callee owns sorted base_ids and filtered/cloned case_rows.\n'
            '_l5_index = (Xc("String", PA) + B(PA * textcap(1024)) + Sort("String", PA) '
            '+ V("ResultItem", PA) + tx("row") * PA)\n'
            'if _mc.get("z", 0):\n'
            '    add("Combination index clones (W1-L5 base_ids, case_rows)", "AggregationSuffix", _l5_index)\n')
        W1_L5_PATCHES = [
            ("ordinary_caps.py", _AGG_ROW,
             '    ((Xc("ResultItem", PA) + tx("row") * PA) * 2 + X({"Node(String,BTreeMap)": PA}) '
             '+ X({"Node(String,ResultItem)": PA}) if _mc.get("z", 0) else V("ResultItem", PA) + tx("row") * PA)'
             '   # W1-L5: no combination index at z=0; keep push-capacity results\n',
             "W1-L5: route L's no-combination path omits the row-index clone and map"),
            ("ordinary_caps.py", _L5_ANCHOR, _L5_ANCHOR + _L5_FAMILY,
             "W1-L5: positive callee index-clone family, reached only with combinations")]
        W1_L5_SITE_FROM = [(site("run_linear_static_preview_observed_inner", _LIB,
                ".insert(load_case_id.clone(), result.clone());"),
            {"from": [_AGG], "per": "min(z, 1) * cases * P",
             "why": O10 + "W1-L5: the guarded load_case_id clone forms once per row with combinations, never at z=0"})]

# U6-BE (RD3/RD4, A2; the final source 588de796a4): section_basis.rs `bit_equal`, the readers' bit-for-bit equality of
# twins and links, recurses into arrays and objects. Its only text is `number`'s Err String (one static literal, 29 B),
# which it creates only for a leaf that is a Value::Number on both sides; a serde_json Number's as_f64 is always Some
# and finite, so the Err never forms, and any formed Err is dropped inside the one `matches!` (at most two live at once,
# whatever the depth). bit_equal allocates nothing else, so its text is counted once per external call, as guard_json's.
# Guard: the function's exact text.
_SBR_BE = RE_ + "section_basis.rs"
if fnkey("bit_equal", _SBR_BE):
    _BE_TEXT = ("pub fn bit_equal(a: &Value, b: &Value) -> bool {\n    match (a, b) {\n"
                "        (Value::Number(_), Value::Number(_)) => {\n"
                "            matches!((number(a), number(b)), (Ok(x), Ok(y)) if x.to_bits() == y.to_bits())\n        }\n"
                "        (Value::Array(x), Value::Array(y)) => x.len() == y.len() && x.iter().zip(y).all(|(p, q)| bit_equal(p, q)),\n"
                "        (Value::Object(x), Value::Object(y)) => {\n"
                "            x.len() == y.len() && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| bit_equal(v, w)))\n        }\n"
                "        _ => a == b,\n    }\n}\n")
    _be = _fn_text("bit_equal", _SBR_BE)
    if not (_be.startswith(_BE_TEXT) and not _be[len(_BE_TEXT):].strip()) \
            or "    v.as_f64()\n        .filter(|n| n.is_finite())\n        .ok_or_else(|| \"SOURCE_PHYSICS_NUMBER_INVALID\".into())\n" \
            not in _fn_text("number", _SBR_BE):
        raise SystemExit("U6-BE: section_basis.rs bit_equal or number changed; re-review its recursion")
    O10_RECURSION_REVIEWED = list(globals().get("O10_RECURSION_REVIEWED", [])) + [(fnkey("bit_equal", _SBR_BE),
        "U6-BE: bit_equal's only text is number's static Err (29 B), formed only for a Number leaf, whose as_f64 is always "
        "Some and finite, and dropped inside the match; nothing else allocates: its text counts once per external call")]

# ====================================================================================================================
# T4-S5: shared epoch 4 (model 0.6.0, U4b, U5b and the W1 not-covered passthrough), actual-source D3. Each label's
# argument is in DEL-04-08 Design retained-pricing-rules.md §8.8. Every rule is keyed by name and guarded by its function.
_U4R, _U4E = PP_ + "u4b_rows.rs", RE_ + "u4b_evidence.rs"
_TR, _CN_FK, _JT = FK_ + "tie_rod.rs", FK_ + "connector.rs", PP_ + "joint.rs"
if fnkey("append_station_quantities", _U4R) and fnkey("member_stations", _U4E):
    def _u4_const(f, name):
        m = _re.findall(r"const " + name + r": &str = \"((?:[^\"\\]|\\.)*)\";", _text_of(f))
        if len(m) != 1:
            raise SystemExit(f"U4B-ID: {name} is not one &str const in {f}")
        return len(m[0].encode())
    for _f in (_U4R, _U4E):
        if not _re.search(r"const STATIONS: \[&str; 5\] = \[\"end_i\", \"end_j\", \"quarter_1\", \"midspan\", \"quarter_3\"\];", _text_of(_f)):
            raise SystemExit(f"U4B-L: {_f} STATIONS is no longer the five reviewed stations")
    if "pub(crate) const BOTH: [SectionBasis; 2]" not in _text_of(PP_ + "pressure_exact/source_geometry.rs"):
        raise SystemExit("U4B-L: SectionBasis::BOTH is no longer two bases")
    _U4L = O10 + "U4B-L: "
    # U4B-NQ (T4-S6, Codex correction; §8.8): not_qualified_for's filter admits only the U4b derived kinds. They are
    # pushed only by u4b_rows (append_station_quantities and the failed-manifest placeholders, bodies pinned by
    # u4b_u5b_owners_model), at most sigma+, sigma- and tau_v,max at the five STATIONS on the two bases of each member
    # (30 per member; realized arcs are absent, P2), so both filtered loops run <= 30m times, not P_final.
    _nq_base = _os.path.join(_os.path.dirname(_os.path.abspath(__file__)), "..", "base")
    __import__("sys").path.append(_nq_base)        # the owners model imports g3lib from base/
    _nq_model = __import__("runpy").run_path(_os.path.join(_nq_base, "u4b_u5b_owners_model.py"))
    __import__("sys").path.remove(_nq_base)
    _nq_state = _nq_model["inspect"](_SNAP)
    if _nq_state["not_qualified"] and _nq_state["station_quantities"] and _nq_model["NOT_QUALIFIED_PER_MEMBER"] == 30:
        _pp_src = _os.path.join(_SNAP, PP_)
        _pp_files = [_os.path.relpath(_os.path.join(_d, _n), _SNAP) for _d, _, _ns in _os.walk(_pp_src) for _n in _ns
                     if _n.endswith(".rs") and not _n.endswith("_tests.rs") and "tests" not in _os.path.relpath(_d, _pp_src).split(_os.sep)]
        _literal = sum(_text_of(_f).count('"element_local_normal_stress_extreme"') + _text_of(_f).count('"element_local_transverse_shear_stress_maximum"')
                       for _f in _pp_files)
        _args = {_f: _text_of(_f).count("EXTREME_KIND, ") + _text_of(_f).count("SHEAR_MAX_KIND, ") for _f in _pp_files}
        if _literal != 2 or {_f: _c for _f, _c in _args.items() if _c} != {_U4R: 4}:
            raise SystemExit(f"U4B-NQ: the derived kinds' producers changed ({_literal}, {_args}); re-review the 30m bound")
        _NQ_BOUND, _NQ_WHY = "30*m", "<= 30 per member: sigma+/sigma-/tau_v,max at five stations on two bases"
    else:
        _NQ_BOUND, _NQ_WHY = "P", "<= P_final"
    _P2 = " (P2: D1 refuses every model with components, so no realized arc, component association or connector exists)"
    O10_LOOPS_LAST += [
        ("for location in STATIONS", "5", _U4L + "STATIONS is the fixed five-station array (checked above)",
         ["u4b_rows.rs:append_station_quantities", "u4b_rows.rs:preview_straight_derived_placeholders",
          "u4b_evidence.rs:member_stations", "u4b_evidence.rs:directional_model_checks"]),
        ("for (slot, b) in m.bases.iter().enumerate()", "2", _U4L + "member_bases: SectionBasis::BOTH (2) or the one preview section", ["u4b_rows.rs:append_station_quantities"]),
        ("for &(basis, e, _, _) in &extremes", "2", _U4L + "extremes holds at most one entry per basis (<= 2)", ["u4b_rows.rs:append_station_quantities"]),
        ("for &(basis, _, y, z) in &extremes", "2", _U4L + "extremes holds at most one entry per basis (<= 2)", ["u4b_rows.rs:append_station_quantities"]),
        ("for (b, factor) in m.bases.iter().zip(&factors)", "2", _U4L + "one factor per basis (<= 2)", ["u4b_rows.rs:append_station_quantities"]),
        ("for (b, value) in m.bases.iter().zip(values)", "2", _U4L + "one hoop value per basis (<= 2)", ["u4b_rows.rs:append_station_quantities"]),
        ("for (b, (_, witnesses)) in bases.iter().zip(&station.normal_witnesses)", "2", _U4L + "one witness list per basis (<= 2); realized arcs only", ["lib.rs:" + _CASE_FN]),
        ("for (b, (_, witnesses)) in bases.iter().zip(&station.directional_witnesses)", "2", _U4L + "one witness list per basis (<= 2); directional arcs only", ["lib.rs:" + _CASE_FN]),
        ("for basis in &m.bases", "2", _U4L + "MemberFacts.bases: section_basis::BASES (2) or [\"\"]", ["u4b_evidence.rs:member_stations"]),
        ("for (got, want) in hoop.iter().zip(expected)", "3", _U4L + "three hoop rows (intrados, crown, extrados)", ["u4b_evidence.rs:member_stations"]),
        ("for &basis in bases", "2", _U4L + "the route's literal basis list (<= 2)", ["u4b_evidence.rs:directional_model_checks"]),
        ("for &pipe in arcs.keys()", "0", _U4L + "directional_arcs keys come only from model.components" + _P2, ["u4b_evidence.rs:directional_model_checks"]),
        ('for component in model[" "].as_array().map(Vec::as_slice).unwrap_or(&[])', "0", _U4L + "the captured model's components" + _P2, ["u4b_evidence.rs:directional_arcs"]),
        ('for record in case[" "].as_array().map(Vec::as_slice).unwrap_or(&[])', "0",
         _U4L + "a case's intensified_measures records exist only for component associations and directional arcs" + _P2, ["u4b_evidence.rs:directional_model_checks"]),
        ("for (pipe, basis, record, row) in maxima", "0", _U4L + "a directional maximum is pushed only after its pipe is a realized arc" + _P2, ["u4b_evidence.rs:directional_records"]),
        ("for cover in stations.values()", "0", _U4L + "a directional station entry is pushed only after its pipe is a realized arc" + _P2, ["u4b_evidence.rs:directional_records"]),
        ('for row in rows.iter().filter(|r| r[" "] == SHEAR_MAX && case_of(r) == Some(case_id) && r[" "] == pipe_id)', "Pall",
         _U4L + "one pass over the projected envelope's rows (<= C·P_final = Pall)", ["u4b_evidence.rs:preview_model_checks"]),
        ("for &(case, pipe, basis, record, row) in arc_extrema", "2*cases*m", _U4L + "arc extrema: at most one per case, member and basis", ["physics_evidence.rs:u4b_cases"]),
        ("for &(_, pipe, record, row) in arc_extrema.iter().filter(|e| e.0 == case_id)", "cases*m", _U4L + "preview arc extrema: at most one per case and member", ["preview_physics_evidence.rs:validate_preview_physics_evidence"]),
        ('for row in results.iter().filter(|r| r[" "][" "] == case_id)', "Pall", _U4L + "one pass over the envelope's rows", ["physics_evidence.rs:u4b_cases"]),
        ("for (position, row) in published.iter().enumerate()", "Pall", _U4L + "one pass over the envelope's rows", ["physics_evidence.rs:validate_physics_evidence_in"]),
        ("for r in view.case_rows()", "P", O10 + "U4B-W1: one case's rows (<= P_final)", ["retained_product.rs:prepared_not_covered"]),
        ("for d in not_qualified_for(rows)?", _NQ_BOUND, O10 + "U4B-NQ: one derived treatment per derived row of the selected case (" + _NQ_WHY + ")", ["source_receipt/rows.rs:bind_for"]),
        ("for row in rows.iter().filter(|r| r.kind == u4b_rows::EXTREME_KIND || r.kind == u4b_rows::SHEAR_MAX_KIND)", _NQ_BOUND,
         O10 + "U4B-NQ: the selected case's sigma+/sigma-/tau_v,max rows (" + _NQ_WHY + ")", ["source_receipt/rows.rs:not_qualified_for"]),
        ("iter:input_result_ids: inputs.iter().map", "3", O10 + "U4B-NQ: at most three formula inputs per derived row", ["source_receipt/rows.rs:not_qualified_for"]),
    ]
if fnkey("tie_rods", _CN_FK) or _os.path.exists(_os.path.join(_SNAP, _TR)):
    # U5B-L: a tie rod exists only in a connector's hardware (ObjectiveConnector::with_tie_rods, called only by
    # joint::build_connector), so P3's empty connector collections leave every rod loop empty under D1.
    _U5L = O10 + "U5B-L: tie rods belong to built connectors, empty under D1 (P3: ObjectiveConnector::with_tie_rods is called only by joint::build_connector)"
    O10_LOOPS_LAST += [
        ("for (connector_index, (connector, record)) in built.connectors.iter().zip(&built.connector_records).enumerate()", "0",
         O10 + "U5B-L: L1: built.connectors is empty under D1", ["lib.rs:add_tie_rod_reference_loads"]),
        ("for (rod_index, (rod, meta)) in connector.tie_rods().iter().zip(&record.tie_rods).enumerate()", "0", _U5L, ["lib.rs:add_tie_rod_reference_loads"]),
        ("for (index, meta) in record.tie_rods.iter().enumerate()", "0", _U5L, ["lib.rs:append_connector_results"]),
        ("for (rod, meta) in connector.tie_rods().iter().zip(&record.tie_rods)", "0", _U5L, ["lib.rs:append_connector_results"]),
        ('for (kind, component, row) in pe.map(|pe| (" ", " ", pe)).into_iter().chain([ (" ", " ", s), ])', "2",
         O10 + "U5B-L: at most the pe_x row and the s_x row", ["lib.rs:append_connector_results"]),
        ("iter:rods .iter().map", "0", _U5L, ["connector.rs:force_scaled"]),
        ("for rod in entries", "0", _U5L + "; a tie_rods array is read only from a connector record (E1: records are empty on route E)", ["connector_evidence.rs:tie_rods"]),
        ('for rod in record[" "].as_array().into_iter().flatten().filter_map(|r| r[" "].as_str())', "0", _U5L + "; inside E2's loop over spans (empty)", ["connector_evidence.rs:validate"]),
        ('iter:record[" "].as_array().into_iter().flatten() .filter_map(|r| r[" "].as_str()).find', "0", _U5L + "; only for a tie-rod row of a connector record (E1: none)", ["connector_evidence.rs:validate"]),
        ("for rod in &rods", "0", _U5L + "; inside validate_pressure's loop over connector records (E1: none)", ["connector_evidence.rs:validate_pressure"]),
        ("iter:rows.values().find", "Pall", O10 + "U5B-L: one pass over the indexed rows", ["connector_evidence.rs:validate_pressure"]),
    ]
    # U5B-REC: ObjectiveConnector::force_scaled maps each rod through TieRod::force_scaled, not itself.
    if fnkey("force_scaled", _CN_FK) and fnkey("force_scaled", _TR):
        site("force_scaled", _CN_FK, ".map(|rod| rod.force_scaled(scale))")
        CG_RULES += [{"caller": _fn_n("force_scaled", _CN_FK, 1, 1), "name": "force_scaled", "targets": [_fn_n("force_scaled", _TR, 1, 1)],
            "added": "U5B-REC", "evidence": "`rod.force_scaled(scale)` receives a &TieRod (connector.tie_rods); the connector's own method is not called recursively"}]
if fnkey("append_station_quantities", _U4R):
    _GEOM_U4 = PP_ + "pressure_exact/source_geometry.rs"
    _SEG_U4 = max(_literals(_fn_text("hyphen", _GEOM_U4)))
    _LOC = max(_literals(_fn_text("id_location", _U4R)) + [len(s) for s in ("end_i", "end_j", "quarter_1", "midspan", "quarter_3")])
    _SHEAR = max(_u4_const(_U4R, "SHEAR_SIGN_EXACT"), _u4_const(_U4R, "SHEAR_SIGN_PREVIEW"), _u4_const(_U4E, "SHEAR_SIGN_EXACT"), _u4_const(_U4E, "SHEAR_SIGN_PREVIEW"))
    _ARCSFX = max(_u4_const(_U4R, "SHEAR_ARC_SUFFIX"), _u4_const(_U4E, "SHEAR_ARC_SUFFIX"))
    _CODE = max(len(c) for c in _re.findall(r'"([A-Z][A-Z0-9_]+)"', _production(_U4E)))
    def _u4(n, why):
        return {"bytes": n, "source": O10 + "U4B-ID: " + why}
    _LOCC, _HY = _u4(_LOC, "u4b_rows::id_location of a station (longest literal)"), _u4(_SEG_U4, "SectionBasis::hyphen, the longest literal")
    _MAPOR = 'b.basis.map_or(String::new(),|b|format!(":{}",b.hyphen()))'
    _STREF = {"stable_suffix(case_id)": IN, "stable_suffix(pipe_id)": IN}
    O10_ID_AUDIT += [
        (site("poisson_out_of_domain", _U4R, 'format!("diagnostic:transverse-shear:{}:{}:poisson"', full=True), _STREF),
        (site("poisson_out_of_domain", _U4R, 'no transverse-shear maximum is published for it in {}", pipe_id', full=True), {"pipe_id": IN, "case_id": IN}),
        (site("poisson_out_of_domain", _U4R, "vec![case_id.to_string(), pipe_id.to_string()]", full=True), {"case_id": IN, "pipe_id": IN}),
        (site("preview_straight_derived_placeholders", _U4R, 'format!("result:stress:{suffix}:{loc}:{tail}")', full=True), {"loc": _LOCC}),
        (site("preview_straight_derived_placeholders", _U4R, 'format!("result:stress:{suffix}:{loc}:transverse-shear-max")', full=True), {"loc": _LOCC}),
        (site("append_station_quantities", _U4R, 'row(format!("result:stress:{suffix}:{loc}:{tail}")', full=True), {"loc": _LOCC}),
        (site("append_station_quantities", _U4R, 'row(format!("result:stress:{suffix}:{loc}:transverse-shear-max")', full=True), {"loc": _LOCC}),
        (site("append_station_quantities", _U4R, 'row(format!("result:intensified-directional:{suffix}:{loc}")', full=True), {"loc": _LOCC}),
        (site("append_station_quantities", _U4R, 'if m.exact { SHEAR_SIGN_EXACT } else { SHEAR_SIGN_PREVIEW },', full=True, row="format!("),
         {"ifm.exact{SHEAR_SIGN_EXACT}else{SHEAR_SIGN_PREVIEW}": _u4(_SHEAR, "the longer route τ sign constant"),
          'ifm.arc{SHEAR_ARC_SUFFIX}else{""}': _u4(_ARCSFX, "SHEAR_ARC_SUFFIX")}),
        (site("append_station_quantities", _U4R, 'format!("diagnostic:toroidal-hoop:{}:{}:domain"', full=True), {"stable_suffix(m.case_id)": IN, "suffix": IN}),
        (site("append_station_quantities", _U4R, "its toroidal hoop and radial rows are not published in {}\", m.pipe_id", full=True), {"m.pipe_id": IN, "m.case_id": IN}),
        (site("append_station_quantities", _U4R, 'format!("diagnostic:toroidal-hoop:{}:{}:evaluation{}"', full=True),
         {"stable_suffix(m.case_id)": IN, "suffix": IN, _MAPOR: _u4(1 + _SEG_U4, "':' and SectionBasis::hyphen"), "b.hyphen()": _HY}),
        (site("append_station_quantities", _U4R, 'format!("{TOROIDAL_EVALUATION_TEXT}: arc {} in {}: {error:?}", m.pipe_id, m.case_id)', full=True), {"m.pipe_id": IN, "m.case_id": IN}),
        (site(_CASE_FN, PP_ + "lib.rs", 'format!("diagnostic:intensified-directional:{}:{}:maximum{}"', full=True),
         {_MAPOR: _u4(1 + _SEG_U4, "':' and SectionBasis::hyphen"), "b.hyphen()": _HY}),
        (site_after(_CASE_FN, PP_ + "lib.rs", "records.push(corrected_displacement_tests::DisplacementPublication {", "case_id: load_case.id.clone(),", within=2),
         {"load_case.id": IN}),   # U5B-PD: the #[cfg(test)] publication probe (compiled out; charged at IN128)
        (site("restate_nonlinear_support_displacements", PP_ + "lib.rs", 'format!("result:nonlinear-support:{}:{}-displacement"', full=True),
         {"stable_suffix(&support.support_id)": IN, "support.dof.as_str()": ST}),
        (site("tie_rod_reference_source", PP_ + "lib.rs", 'format!("{}:{component}:{}:{rod}", component.len(), rod.len())', full=True),
         {"rod": IN, "component": IN, "component.len()": NUM_, "rod.len()": NUM_}),
        (site("validate_final_metadata", PP_ + "retained_product.rs", 'format!("result:stress:{suffix}:{}:{tail}", u4b_rows::id_location(loc))', full=True),
         {"suffix": IN, "u4b_rows::id_location(loc)": _LOCC}),
        (site("fail", _U4E, 'Route::Exact => format!("SOURCE_PHYSICS_{code}"),', full=True), {"code": _u4(_CODE, "the longest code literal in u4b_evidence.rs")}),
        (site("fail", _U4E, 'Route::Preview => format!("SOURCE_PREVIEW_PHYSICS_{code}"),', full=True), {"code": _u4(_CODE, "the longest code literal in u4b_evidence.rs")}),
        (site("shear_sign", _U4E, "format!(", full=True),
         {"ifself==Route::Exact{SHEAR_SIGN_EXACT}else{SHEAR_SIGN_PREVIEW}": _u4(_SHEAR, "the longer route τ sign constant"),
          'ifarc{SHEAR_ARC_SUFFIX}else{""}': _u4(_ARCSFX, "SHEAR_ARC_SUFFIX")}),
    ]
    # U4B-W1/U4B-RD: two selected signs copied by to_string at one site each.
    LEX_SITE_SIZE += [
        (Resolved("preview_physics.rs:" + site("maximum_row", PP_ + "preview_physics.rs", "else { MAXIMUM_SIGN }.to_string(),").rsplit(":", 1)[1]),
         {"class": max(_u4_const(_U4R, "ARC_MAXIMUM_SIGN"), _u4_const(PP_ + "preview_physics.rs", "MAXIMUM_SIGN")), "why": O10 + "U4B-ID: the longer of the two maximum sign constants"}),
        (Resolved("u4b_evidence.rs:" + site("row_semantics", _U4E, "else { SIGMA_PLUS_SIGN }.to_string(), true, false),").rsplit(":", 1)[1]),
         {"class": max(_u4_const(_U4E, "SIGMA_PLUS_SIGN"), _u4_const(_U4E, "SIGMA_MINUS_SIGN")), "why": O10 + "U4B-ID: the longer of the two σ± sign constants"}),
    ]
    # U4B-RD: Route::fail allocates only when it returns Err; every require/fail in u4b_evidence.rs and every outside call
    # into its checks propagates with `?` (or map_err then `?`) to these readers' entries: <= 1 per entry call.
    _U4_ROOTS = [k for k in (fnkey("validate_physics_evidence_in", RE_ + "physics_evidence.rs"),
                             fnkey("validate_preview_physics_evidence", RE_ + "preview_physics_evidence.rs"),
                             fnkey("gates_after_g0", RE_ + "retained_precision.rs"), fnkey("validate", RE_ + "reader_verdict.rs")) if k]
    O10_SITE_FROM += [(site("fail", _U4E, _t), {"from": _U4_ROOTS, "per": "1",
        "why": O10 + "U4B-RD: Route::fail forms its code only on Err, and every failure returns through `?` to the reader entry"})
        for _t in ('Route::Exact => format!("SOURCE_PHYSICS_{code}"),', 'Route::Preview => format!("SOURCE_PREVIEW_PHYSICS_{code}"),')]
if _os.path.exists(_os.path.join(_SNAP, _TR)):
    # U5B-ID: TieRodError's `name` is always one static literal (or finite3's literal argument).
    _trp = _production(_TR)
    _TRN = [len(x) for x in _re.findall(r'name:\s*"([^"]+)"', _trp) + _re.findall(r'finite3\("([^"]+)"', _trp)]
    if len(_re.findall(r"TieRodError::(?:NonFinite|Range)\s*\{\s*name\s*\}", _trp)) != 1 or not _TRN:
        raise SystemExit("U5B-ID: a TieRodError name is no longer a static literal; re-review")
    _TRC = {"bytes": max(_TRN), "source": O10 + "U5B-ID: the longest static tie-rod field name"}
    O10_ID_AUDIT += [(site("fmt", _TR, 'Self::NonFinite { name } => write!(f, "tie rod input {name} is not finite"),', full=True), {"name": _TRC}),
                     (site("fmt", _TR, 'Self::Range { name } => write!(f, "tie rod {name} outside the binary64 range"),', full=True), {"name": _TRC})]
    O10_ID_AUDIT += [(site("connector_record", PP_ + "certified_publication.rs", 'format!("tie_rod:{rod}")', full=True), {"rod": IN})]
if fnkey("hardware", _JT):
    # U5B-JT: joint hardware findings; unreached on D1 (decoded only from model.components: P2). A joined list is bounded
    # by D1's raw census: at most 16,384 values, one 256-byte problem each, plus all 131,072 authored string bytes.
    _JC = {"bytes": 16_384 * 256 + 131_072, "source": O10 + "U5B-JT: raw-census bound of a joined hardware problem list"}
    for _fn, _t, _args in [
            ("hardware", '"objective connector {component}: {}; only {ROD_LAW} is', {'laws.join(";")': _JC}),
            ("hardware", '"objective connector {component}: invalid tie-rod hardware: {}; it is never assembled', {'problems.join(";")': _JC}),
            ("tie_rod", 'laws.push(format!("tie rod {name} law {law} is not supported"));', {"law": IN}),
            ("tie_rod", 'problems.push(format!("{key}.{axis} {problem}"));', {"problem": _JC}),
            ("tie_rod", 'problems.push(format!("stiffness {problem}"));', {"problem": _JC}),
            ("tie_rod", 'problems.push(format!("reference_extension {problem}"));', {"problem": _JC}),
            ("tie_rod", 'return Err(Some(format!("tie rod {name}: {}", problems.join(", "))));', {'problems.join(",")': _JC})]:
        O10_ID_AUDIT += [(site(_fn, _JT, _t, full=True, row="format!(" if not _t.startswith(("laws.", "problems.", "return")) else None), _args)]
    O10_ID_AUDIT += [
        (site("tie_rod_formation_findings", _JT, 'problems.join("; ")))).collect()', full=True, row="format!("), {'problems.join(";")': _JC}),
        (site("model_findings", _JT, '"objective connector {}: {problem}; the connector is not assembled"', full=True, row="format!("), {"problem": _JC}),
    ]
_J7C_U4 = PP_ + "source_recovery/j7_rf_census.rs"
if fnkey("u4b_every_selected_census_case_still_selects_with_its_new_charge", _J7C_U4):
    # U4B-J3 (test code, multiplicity 0 by J3): honest classes for the census line's other arguments.
    O10_ID_AUDIT += [(site("u4b_every_selected_census_case_still_selects_with_its_new_charge", _J7C_U4, 'let mut line = format!("U4B_J7_SELECTED', full=True),
        {"class": {"bytes": 64, "source": O10 + "U4B-J3: a census first-cause class label"}, "direct": NUM_,
         'pinned["first_cause"]["work"]["charged"]': {"bytes": 24, "source": O10 + "U4B-J3: a pinned u64 charge in JSON Display"}})]
if fnkey("append_station_quantities", _U4R):
    _u4p = _production(_U4R)
    _KINDS = max(len(v.encode()) for v in _re.findall(r'_KIND: &str = "([^"]+)";', _u4p))
    _BASES = max(len(v.encode()) for v in _re.findall(r'_BASIS: &str = "([^"]+)";', _u4p))
    _SIGNS = max([len(v.encode()) for v in _re.findall(r'_SIGN: &str = "([^"]+)";', _u4p)] + [_SHEAR + _ARCSFX])
    _LOWER = max(len(v) for v in _re.findall(r'"([a-z][a-z0-9_]*)"', _u4p))
    _ARCREASON = max(_literals(_fn_text("reason", FK_ + "structural/arc_maximum.rs")))
    _RF = site("row", _U4R, "kind: kind.to_string(),", full=True).rsplit(":", 1)[0]
    O10_ID_AUDIT += [
        (site("row", _U4R, "kind: kind.to_string(),", full=True), {"kind": _u4(_KINDS, "the longest u4b_rows kind constant")}),
        (site("row", _U4R, "unit: unit.to_string(),", full=True), {"unit": _u4(3, "\"MPa\" or \"Pa\"")}),
        (site("row", _U4R, "entity_ref: pipe.to_string(),", full=True), {"pipe": IN}),
        (site("row", _U4R, "component: component.to_string(),", full=True), {"component": _u4(_LOWER, "the longest lowercase literal in u4b_rows.rs")}),
        (site("row", _U4R, "location: location.to_string(),", full=True), {"location": _u4(_LOWER, "the longest lowercase literal in u4b_rows.rs")}),
        (site("row", _U4R, "basis: basis.to_string(),", full=True), {"basis": _u4(_BASES, "the longest u4b_rows basis constant")}),
        (site("row", _U4R, "sign_convention: sign.to_string(),", full=True), {"sign": _u4(_SIGNS, "the longest sign constant, or the τ sign with its arc suffix")}),
        (site("final_row_id", _U4R, "id.to_string()", full=True), {"id": RS_}),
        (site("pressure_row_id", _U4R, 'format!("result:pressure-exact:{}:{}:{}:{}:{}:{}"', full=True),
         {"case_id": IN, "pipe_id": IN, "case_id.len()": NUM_, "pipe_id.len()": NUM_,
          "location": _u4(_LOWER, "a station literal"), "component": _u4(_LOWER, "a toroidal component literal")}),
        (site("arc_maximum_row", _U4R, 'format!("result:elastic-maximum:{}:{}:{}:{}"', full=True),
         {"case_id": IN, "pipe_id": IN, "case_id.len()": NUM_, "pipe_id.len()": NUM_}),
        (site("directional_maximum_row", _U4R, 'format!("result:intensified-directional-maximum:{}:{}:{}:{}"', full=True),
         {"case_id": IN, "pipe_id": IN, "case_id.len()": NUM_, "pipe_id.len()": NUM_}),
        (site("append_station_quantities", _U4R, 'vec![m.case_id.to_string(), m.pipe_id.to_string()],', full=True, row=None) if _fn_text("append_station_quantities", _U4R).count('vec![m.case_id.to_string(), m.pipe_id.to_string()],') == 1 else None, {"m.case_id": IN, "m.pipe_id": IN}),
    ]
    O10_ID_AUDIT = [e for e in O10_ID_AUDIT if e[0] is not None]
    # the two diagnostic ref vectors of append_station_quantities (domain and evaluation arms)
    _asq_lines, _asq_start, _asq_end = fn_lines("append_station_quantities", _U4R)
    _asq_refs = [i + 1 for i in range(_asq_start - 1, _asq_end - 1) if "vec![m.case_id.to_string(), m.pipe_id.to_string()]," in _asq_lines[i]]
    if len(_asq_refs) != 2:
        raise SystemExit(f"U4B-ID: append_station_quantities' diagnostic ref vectors changed: {_asq_refs}")
    O10_ID_AUDIT += [(Resolved(f"{_U4R}:{_l}"), {"m.case_id": IN, "m.pipe_id": IN}) for _l in _asq_refs]
    O10_ID_AUDIT += [(site(_fn, _U4R, _t, full=True), {"suffix": IN}) for _fn, _t in (
        ("preview_straight_derived_placeholders", 'format!("result:stress:{suffix}:{loc}:{tail}")'),
        ("preview_straight_derived_placeholders", 'format!("result:stress:{suffix}:{loc}:transverse-shear-max")'),
        ("append_station_quantities", 'row(format!("result:stress:{suffix}:{loc}:{tail}")'),
        ("append_station_quantities", 'row(format!("result:stress:{suffix}:{loc}:transverse-shear-max")'),
        ("append_station_quantities", 'row(format!("result:intensified-directional:{suffix}:{loc}")'))]
    _LIBF = PP_ + "lib.rs"
    O10_ID_AUDIT += [
        (site(_CASE_FN, _LIBF, "let result_id = item.id.clone();", full=True), {"item.id": _RID_D5}),
        (site(_CASE_FN, _LIBF, '"EXACT_STRESS_GOVERNING_MAXIMUM_UNAVAILABLE","warning",format!("signed physical rows remain available; governing circular-normal-stress maximum is unavailable: {reason}"),vec![load_case.id.clone(),pipe.element_id.clone()]));', full=True),
         {"load_case.id": IN, "pipe.element_id": IN}),
        (site(_CASE_FN, _LIBF, 'format!("diagnostic:intensified-directional:{}:{}:maximum{}"', full=True),
         {"stable_suffix(&load_case.id)": IN, "stable_suffix(&pipe.element_id)": IN}),
        (site_after(_CASE_FN, _LIBF, '"DIRECTIONAL_INTENSIFIED_MAXIMUM_UNAVAILABLE", "warning",', "vec![load_case.id.clone(), pipe.element_id.clone()],", within=3),
         {"load_case.id": IN, "pipe.element_id": IN}),
    ]
    LEX_SITE_SIZE += [(Resolved("u4b_rows.rs:" + site("arc_maximum", _U4R, ".map_err(|failure| failure.reason().to_string())").rsplit(":", 1)[1]),
        {"class": _ARCREASON, "why": O10 + "U4B-ID: ArcMaximumFailure::reason returns one of its static literals"})]
    O10_LOOPS_LAST += [("for cell in candidates", "131072", O10 + "U4B-L: the candidate cells of one arc maximum are a subset of its cells, "
        "<= ARC_MAXIMUM_SUBDIVISION_LIMIT (131,072); realized arcs only (P2: none on D1)", ["structural/arc_maximum.rs:locate"])]
    if "pub const ARC_MAXIMUM_SUBDIVISION_LIMIT: usize = 131_072;" not in _text_of(FK_ + "structural/arc_maximum.rs"):
        raise SystemExit("U4B-L: ARC_MAXIMUM_SUBDIVISION_LIMIT changed; re-review")
if fnkey("not_qualified_for", PP_ + "source_receipt/rows.rs"):
    _RW = PP_ + "source_receipt/rows.rs"
    _KLIT = max(len(v) for v in _re.findall(r'find\("([a-z_]+)"', _fn_text("not_qualified_inputs", _RW)))
    O10_ID_AUDIT += [
        (site("bind_for", _RW, "ledger.insert(d.result_id.clone(), d);", full=True), {"d.result_id": RS_}),
        (site("not_qualified_inputs", _RW, 'Err(bad(format!("derived_not_qualified input: {kind}")))', full=True), {"kind": _u4(_KLIT, "the longest input kind literal")}),
        (site("not_qualified_for", _RW, 'format!("derived_not_qualified value: {}", row.id)', full=True), {"row.id": RS_}),
        (site("not_qualified_for", _RW, "result_id: row.id.clone(),", full=True), {"row.id": RS_}),
        (site("not_qualified_for", _RW, "input_result_ids: inputs.iter().map(|r| r.id.clone()).collect(),", full=True), {"r.id": RS_}),
    ]

# ---- O4 S5 (actual-source D3; retained-pricing-rules.md §8.9). O4 admits timoshenko_annulus_energy_v1 documents and
# nonzero mill tolerance/corrosion allowance on route E (W1-E) and on the pressure-source route. Its new loops and text
# arguments are bounded here by name; its heap owners are priced positively by base/o4_owners_model.py.
if fnkey("reduced_section_set", PP_ + "retained_product.rs"):
    _RPO = PP_ + "retained_product.rs"
    # O4-L1: reduced_facts is set once per invocation from a local vector reserved at distinct_bases.len() and pushed
    # only inside prepare_case's member loop under a capacity check (`reduced fact capacity`); distinct_bases takes at
    # most one push per built pipe (capture's pipe loop). So every reduced_facts / distinct_bases / reduced-preparation
    # list holds <= m entries.
    for _needle in ("if reduced_facts.len()==reduced_facts.capacity() {return Err(\"reduced fact capacity\".into());}",
                    "if out.reduced_preparations.len()==out.reduced_preparations.capacity() {return Err(\"reduced preparation capacity\".into());}",
                    "let mut reduced_facts:Vec<ReducedFacts>=if distinct>0 {self.adapter.reserve(distinct)?} else {Vec::new()};",
                    "if distinct>0 {out.reduced_preparations=self.adapter.reserve(distinct)?;}"):
        if _text_of(_RPO).count(_needle) != 1:
            raise SystemExit("O4-L1: the reduced-fact capacity guards changed; re-review")
    O10_LOOPS_LAST += [
        ("for rf in &self.reduced_facts", "m", O10 + "O4-L1: one entry per distinct-basis member, <= m",
         ["retained_product.rs:reduced_section_set"]),
        ("for (r,bound) in view.case_rows().iter().zip(bound_row_indices(route,view.distinct,view.case_rows()))", "P",
         O10 + "O4-L2: the case's own rows (its scope's block, <= P), as prepared_alias's (U6-L5)",
         ["retained_product.rs:reduced_section_set", "retained_product.rs:prepared_not_covered"]),
        ("for id in &self.distinct_bases", "m", O10 + "O4-L1: one id per distinct-basis member, <= m",
         ["retained_product.rs:distinct_copy"]),
        ("for _ in 0..self.reduced_facts.len()", "m", O10 + "O4-L1: one reduced operand table per distinct-basis member, <= m",
         ["retained_product.rs:prepared_not_covered"]),
        ("for (i,(r,bound)) in rows.iter().zip(bound_row_indices(route,&self.distinct_bases,rows)).enumerate()", "P",
         O10 + "U6-L5 (O4: the twin predicate also reads distinct_bases): prepared_alias's rows are the frozen case's own rows, <= P",
         ["retained_product.rs:prepared_alias"]),
    ]
    # O4-A1: TimoshenkoLeafError::reason returns RETAINED_TIMOSHENKO_REFUSAL or one of its two static literals (both
    # read at the snapshot); the warning of a withheld selected maximum pushes the composite error by clone, the same
    # text `error` carried before (error_display).
    _FKS = FK_ + "structural/retained/source.rs"
    _reason = _fn_text("reason", _FKS) if fnkey("reason", _FKS) else ""
    _reason_lits = _re.findall(r'=> "([^"\\]+)"', _reason)
    _refusal = _re.findall(r'pub const RETAINED_TIMOSHENKO_REFUSAL: &str =\s*"([^"\\]+)";', _text_of(FK_ + "structural/retained/assemble.rs"))
    if len(_reason_lits) != 2 or len(_refusal) != 1 or "super::assemble::RETAINED_TIMOSHENKO_REFUSAL" not in _reason:
        raise SystemExit("O4-A1: TimoshenkoLeafError::reason changed; re-review")
    O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [
        {"re": r"^e\.reason\(\)$", "max": max(len(x.encode()) for x in _reason_lits + _refusal),
         "why": O10 + "O4-A1: TimoshenkoLeafError::reason is RETAINED_TIMOSHENKO_REFUSAL or one of two static literals (read at the snapshot)"},
        {"re": r"^error\.clone\(\)$", "max": "error_display",
         "why": O10 + "O4-A1: solve_prepared_load_case's withheld-maximum warning clones the composite error text it moved before (error_display)"},
    ]
if fnkey("attach_reduced_preparations", PP_ + "retained_wire.rs"):
    O10_LOOPS_LAST += [("for r in reduced", "m", O10 + "O4-L1: the attempt's reduced preparations, <= m (one per distinct-basis member)",
                        ["retained_wire.rs:attach_reduced_preparations"])]
if fnkey("distinct_members_of", RE_ + "retained_precision.rs"):
    O10_LOOPS_LAST += [
        ("for (member, shear_area) in leaves", "m", O10 + "O4-L3: native_bytes' Timoshenko leaves filter one CaseSource's section_terms, one per member (<= m)",
         ["retained_precision.rs:native_bytes"]),
        ("iter:rows_all_for(source, case) .into_iter().filter", "P", O10 + "U6-L5 (O4: the filter also keeps a distinct member's reduced rows): one entry's rows, <= P",
         ["retained_precision.rs:rows_for"]),
    ]
# O4-CG1 (call graph): prepare_active_case's one `prepared` call is the path-qualified FK
# `k::TimoshenkoLeaf::prepared(m.id,kappa,a)` (source.rs: shear::shear_area of two binary64 values, no allocation);
# the by-name scan also offered every other `fn prepared` (e.g. membrane_publication_range's test fixture, which
# rebuilds and validates a model). Only the named definition is the target.
if fnkey("prepare_active_case", PP_ + "retained_product.rs") and "k::TimoshenkoLeaf::prepared(" in _fn_text("prepare_active_case", PP_ + "retained_product.rs"):
    if _fn_text("prepare_active_case", PP_ + "retained_product.rs").count("prepared(") != 1:
        raise SystemExit("O4-CG1: prepare_active_case's prepared calls changed; re-review")
    CG_RULES += [{"caller": _fn_n("prepare_active_case", PP_ + "retained_product.rs", 1, 1), "name": "prepared",
        "targets": [_fn_n("prepared", FK_ + "structural/retained/source.rs", 1, 1)],
        "added": "O4-CG1", "evidence": "the path-qualified k::TimoshenkoLeaf::prepared is FK source.rs's one fn prepared"}]
# O4-A2 (TEXT): the source route's stress_operands names a distinct member's reduced operand paths by one closure,
# `format!("/frames/{i}/reduced_recovery_section/{key}")`, called with four static key literals (read at the snapshot).
if fnkey("stress_operands", PP_ + "source_receipt/source.rs"):
    _SO = _fn_text("stress_operands", PP_ + "source_receipt/source.rs")
    _SO_KEYS = _re.findall(r'path\("([a-z_]+)"\)', _SO)
    if len(_SO_KEYS) != 4:
        raise SystemExit("O4-A2: stress_operands' reduced path keys changed; re-review")
    O10_ID_AUDIT += [(site("stress_operands", PP_ + "source_receipt/source.rs",
                           'let path = |key: &str| format!("/frames/{i}/reduced_recovery_section/{key}");', full=True),
                      {"key": {"bytes": max(len(k) for k in _SO_KEYS), "source": O10 + "O4-A2: one of four static path keys"}})]

# ---- The W1 coverage repair (DEL-04-08 Design w1-coverage-repair.md §2.2 and §6; retained-pricing-rules.md §8.11). Each
# rule applies only where the repair's code exists, so earlier revisions price as before.
# W1R-L1: selection writes `body_scales` from the certified proof's final scales, one [f64;4] per body. They are the
# vector row_scales_reduced reserves and resizes to the source's body_count (<= n, a body is a set of nodes), coupled and
# floored in check_intervals; the receipt's body list keeps the native publication's shape (checked in selection
# against 4 * len). The loop it replaces, over the native BTreeMap of the same bodies, had the same bound (n, loop_bounds'
# one-entry-per-body rule).
_W1R_RW = PP_ + "retained_wire.rs"
if fnkey("selection", _W1R_RW) and "let scales = certificate.final_scales();" in _fn_text("selection", _W1R_RW):
    if _fn_text("selection", _W1R_RW).count("scales.iter().enumerate().map(") != 1:
        raise SystemExit("W1R-L1: selection's body-scale loop changed; re-review")
    O10_LOOPS_LAST += [('iter:" ":scales.iter().enumerate().map', "n",
                        O10 + "W1R-L1: the certified proof's final scales, one entry per body (body_count <= n)",
                        ["retained_wire.rs:selection"])]

# ---- The pre-0.4 failure unification (DEL-04-08 Design pre04-failure-unification.md §§2-4; retained-pricing-rules.md
# §8.14). Each rule applies only where the unification's code exists, so earlier revisions price as before.
_PU_LIB = PP_ + "lib.rs"
if fnkey("republication", _PU_LIB) and fnkey("fail_closed_form", _PU_LIB):
    _PU_CASE = _fn_text("solve_prepared_load_case", _PU_LIB)
    _PU_OBS = _fn_text("run_linear_static_preview_observed_inner", _PU_LIB)
    _PU_CAP = _fn_text("run_linear_static_preview_captured", _PU_LIB)
    _PU_REP = _fn_text("republication", _PU_LIB)
    _PU_RUN = _fn_text("permitted_run", _PU_LIB)
    # The cause texts, read at the snapshot. The S2 cause is `support {id} norm: {error.0}` (one admitted support id,
    # IN128, and one error Display, error_display); it becomes the ReceiptError the case's S3 sites read as error.0. The
    # recorded cause is one of S1 `captured source context: {}`, S3 `case {}: {}` (a load case id and that error.0) and
    # S4 `invocation receipt: {}`; the withheld text appends the cause after a fixed literal.
    _PU_TEMPLATES = {
        "s2": ('format!("support {} norm: {}", support.id, error.0)', _PU_CASE, len("support  norm: ")),
        "s1": ('format!("captured source context: {}", error.0)', _PU_OBS, len("captured source context: ")),
        "s3": ('"case {}: {}",', _PU_CASE, len("case : ")),
        "s4": ('"invocation receipt: {}",', _PU_OBS, len("invocation receipt: ")),
        "withheld": ('format!("; the invocation publishes every case on its ordinary route because its selected join could not finalize: {cause}")',
                     _PU_CASE, len("; the invocation publishes every case on its ordinary route because its selected join could not finalize: ")),
    }
    for _k, (_t, _body, _lit) in _PU_TEMPLATES.items():
        if _body.count(_t) != 1:
            raise SystemExit(f"PU-TX: the {_k} cause template changed; re-review")
    _PU_ERR = 8192   # error_display (TEXT.md section 3)
    _PU_S2 = _PU_TEMPLATES["s2"][2] + 128 + _PU_ERR                                  # 8,335
    _PU_CAUSE = max(_PU_TEMPLATES["s1"][2] + _PU_ERR, _PU_TEMPLATES["s3"][2] + 128 + _PU_S2,
                    _PU_TEMPLATES["s4"][2] + _PU_ERR)                                 # 8,470 (S3 carrying the S2 cause)
    _PU_WITHHELD = _PU_TEMPLATES["withheld"][2] + _PU_CAUSE                          # 8,576
    # PU-ID: the two new format sites' identifier arguments, as their siblings: the S2 cause's support id (IN128) and
    # the missing-row text's entity (IN128), kind and location (static), as on the "not unique" line beside it.
    O10_ID_AUDIT += [
        (site("solve_prepared_load_case", _PU_LIB, 'format!("support {} norm: {}", support.id, error.0)', full=True),
         {"support.id": IN}),
        (site("source_row_bindings", _PU_LIB, 'format!("source functional row binding is missing: {entity}/{kind}/{location}")', full=True),
         {"entity": IN, "kind": ST, "location": ST}),
    ]
    # PU-S2: the case's S3 sites read error.0, which may now be the S2 cause, so error.0 there is bounded by the S2
    # site's own size instead of one error Display: the cause record and the finalization diagnostic.
    _PU_S2_CLASS = {"bytes": _PU_S2, "source": O10 + "PU-S2: the S2 cause (support id + error Display + literal) or an error Display"}
    O10_ID_AUDIT += [
        (site("solve_prepared_load_case", _PU_LIB, "source_budget.record_load_state_join_failure(format!(", full=True), {"error.0": _PU_S2_CLASS}),
        (site("solve_prepared_load_case", _PU_LIB, 'format!("{}; actual_finalization_work={:?}", error.0, error.1), vec![load_case.id.clone()]', full=True),
         {"error.0": _PU_S2_CLASS}),
    ]
    # PU-C: the recorded cause (one of S1, S3 and S4) and the withheld text that appends it, now reachable on route E.
    # The withheld literal is fixed (0.4's text, byte for byte); the cause is at most the largest record site.
    _PU_CAUSE_CLASS = {"bytes": _PU_CAUSE, "source": O10 + "PU-C: the recorded cause, the largest of the S1, S3 and S4 record sites"}
    O10_ID_AUDIT += [
        (site("solve_prepared_load_case", _PU_LIB, _PU_TEMPLATES["withheld"][0], full=True), {"cause": _PU_CAUSE_CLASS}),
        (site("solve_prepared_load_case", _PU_LIB, '{failure:?}{withheld}")', full=True),
         {"withheld": {"bytes": _PU_WITHHELD, "source": O10 + "PU-C: the withheld literal and the recorded cause"}}),
    ]
    # PU-CL: the three clones of the recorded cause (the captured route's and the Direct entry's check, and the
    # republication's ledger copy) are each one cause.
    O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [
        {"re": r"^(source_budget|budget|self)\.join_failure$", "max": _PU_CAUSE,
         "why": O10 + "PU-CL: a clone of the recorded cause (Option<String>), at most the largest record site"}]
    # PU-R1 (guard; G4's one-ordinary-run cap): at most one ordinary run is live at a time. The republication's run
    # starts only after the first run returned and its envelope was reduced to the fail-closed form and dropped, and on
    # the Direct entry after the observer (with its permit) dropped; its request comes from the capture. The cause texts
    # that persist are priced at their own sites; the republication is the X3 phase (b7a_phase_model, g4_caps).
    _pu_guards = (
        (_PU_CAP.count("run_linear_static_preview_captured_once(") == 1 and _PU_CAP.count("republication(") == 1
         and _PU_CAP.index("fail_closed_form(envelope)") < _PU_CAP.index("capture.requested()")
         < _PU_CAP.index("drop(fail_closed);") < _PU_CAP.index("republication(")),
        _PU_REP.count("run_linear_static_preview_captured_once(") == 1 and "withholding_load_state_join(cause)" in _PU_REP,
        (_PU_RUN.count("run_linear_static_preview_observed(") == 1 and _PU_RUN.count("republication(") == 1
         and _PU_RUN.index("run_linear_static_preview_observed(") < _PU_RUN.index("fail_closed_form(ordinary)")
         < _PU_RUN.index("drop(observer);") < _PU_RUN.index("capture.requested()") < _PU_RUN.index("drop(fail_closed);")
         < _PU_RUN.index("republication(") < _PU_RUN.index("Err(W1Fallback::Coexistence)")),
    )
    if not all(_pu_guards):
        raise SystemExit(f"PU-R1: the republication's run order changed; re-review G4's one-run cap: {_pu_guards}")

# ---- The W-8 topology warnings (DEL-04-06 Design model-topology-checks.md; retained-pricing-rules.md §8.16). Each rule
# applies only where W-8's code exists, so earlier revisions price as before. `append_topology_diagnostics` runs once
# per ordinary run (run_linear_static_preview_observed_inner, after the build), on both routes; its heap owners outside
# the TEXT inventory are priced positively by base/w8_topology_model.py (W8-TW, W8-ND).
_W8 = PP_ + "topology.rs"
_W8V = PP_ + "validation.rs"
if fnkey("append_topology_diagnostics", _W8):
    _W8_FG = open(_os.path.join(_SNAP, PP_ + "formation_guard.rs"), encoding="utf-8").read()
    if _W8_FG.count("pub(crate) const NAMED: usize = 6;") != 1:
        raise SystemExit("W8-NM: formation_guard's NAMED changed; re-review the W-8 bounds")
    _W8_NAMED = 6
    _W8_TOP = _fn_text("append_topology_diagnostics", _W8)
    _W8_DIS = _fn_text("disconnected_parts", _W8)
    _W8_NEAR = _fn_text("near_duplicate_nodes", _W8)
    # W8-TX (guard): the templates the bounds below read, each once, with their literal bytes.
    _W8_T = {
        "node": ('format!("node#{index}")', _W8_TOP, len("node#")),
        "part": ('"part {} (from {}; {} node{}; {})",', _W8_DIS, len("part  (from ;  node; )")),
        "more_d": ('format!("; and {} more", parts.len() - NAMED)', _W8_DIS, len("; and  more")),
        "more_n": ('format!("; and {} more", count - NAMED)', _W8_NEAR, len("; and  more")),
        "pair": ('.map(|&(a, b)| format!("{} and {}", node_id(a), node_id(b)))', _W8_NEAR, len(" and ")),
        "apart": ('format!("{} and {} ({distance:e} m apart)", node_id(a), node_id(b))', _W8_NEAR, len(" and  ( m apart)")),
    }
    for _k, (_t, _body, _lit) in _W8_T.items():
        if _body.count(_t) != 1:
            raise SystemExit(f"W8-TX: the {_k} template changed; re-review the W-8 text bounds")
    if (_W8_TOP.count("n.id.clone()") != 1 or _W8_DIS.count('.join("; ")') != 1 or _W8_NEAR.count('.join("; ")') != 2
            or '"supported"' not in _W8_DIS or '"no support"' not in _W8_DIS):
        raise SystemExit("W8-TX: the topology texts changed; re-review the W-8 text bounds")
    _W8_INT = 20                                                     # a usize's Display (TEXT.md `int`)
    _W8_ID = max(128, _W8_T["node"][2] + _W8_INT)                     # node_id: an admitted id (l <= 128) or node#{index}
    _W8_PART = _W8_T["part"][2] + _W8_INT + _W8_ID + _W8_INT + len("s") + len("no support")       # 201
    _W8_MORE = _W8_T["more_d"][2] + _W8_INT                                                   # 31
    _W8_JOIN = lambda each: _W8_NAMED * each + (_W8_NAMED - 1) * len("; ")
    _W8_DESCRIBED = _W8_JOIN(_W8_PART)                                                         # 1,216
    _W8_COINCIDENT = _W8_JOIN(_W8_T["pair"][2] + 2 * _W8_ID)                                   # 1,576
    _W8_NEARNAMED = _W8_JOIN(_W8_T["apart"][2] + 2 * _W8_ID + 24)                              # 1,786 (f64_exp 24)
    # W8-L1: the named parts and pairs are `.take(NAMED)` of the part map and the sorted pair lists.
    O10_LOOPS_LAST += [
        ("iter:parts .iter() .take(NAMED) .enumerate().map", str(_W8_NAMED),
         O10 + "W8-L1: the described parts, `.take(NAMED)` (NAMED = 6)", ["topology.rs:disconnected_parts"]),
        ("iter:coincident .iter() .take(NAMED).map", str(_W8_NAMED),
         O10 + "W8-L1: the named coincident pairs, `.take(NAMED)` (NAMED = 6)", ["topology.rs:near_duplicate_nodes"]),
        ("iter:near .iter() .take(NAMED).map", str(_W8_NAMED),
         O10 + "W8-L1: the named near pairs, `.take(NAMED)` (NAMED = 6)", ["topology.rs:near_duplicate_nodes"]),
    # W8-L2: near_duplicate_nodes' two passes over the built nodes' coordinates (the cell grid's fill and the
    # 27-neighbour scan): coordinates is `built.nodes`' positions, one per node (<= n, D1.9). Only the by-name false
    # edges (Vec::push, HashMap::get/entry) reach text from these loops; the real body allocates no text.
        ("for (node, point) in coordinates.iter().enumerate()", "n",
         O10 + "W8-L2: one visit per built node (<= n)", ["topology.rs:near_duplicate_nodes"]),
    ]
    # W8-NI (SITE_FROM): node_id is a closure passed as `&dyn Fn`, so the chain counts its two alternative text sites
    # (the id clone and `node#{index}`) once per call of their defining function. Per call of
    # append_topology_diagnostics it runs at most 60 times: disconnected_parts, NAMED for the description and NAMED for
    # the refs (12); near_duplicate_nodes, two per named pair for each of the two messages and each of the two refs
    # lists (4 x 2 x NAMED = 48). `more` in near_duplicate_nodes is also a closure, called once per message (2).
    _W8_NODE_CALLS = 2 * _W8_NAMED + 4 * 2 * _W8_NAMED
    if (_W8_DIS.count("node_id(") != 2 or _W8_NEAR.count("node_id(") != 6 or _W8_NEAR.count("refs(&mut ") != 2
            or _W8_NEAR.count("more(") != 2 or _W8_NEAR.count("for id in [node_id(a), node_id(b)]") != 1
            or _W8_TOP.count("node_id") != 3):
        raise SystemExit("W8-NI: node_id's or more's call sites changed; re-review their counts")
    _W8_FROM_TOP = [fnkey("append_topology_diagnostics", _W8)]
    O10_SITE_FROM += [
        (site("append_topology_diagnostics", _W8, ".map(|n| n.id.clone())"),
         {"from": _W8_FROM_TOP, "per": str(_W8_NODE_CALLS), "why": O10 + "W8-NI: node_id's id clone, <= 60 calls per run"}),
        (site("append_topology_diagnostics", _W8, 'format!("node#{index}")'),
         {"from": _W8_FROM_TOP, "per": str(_W8_NODE_CALLS), "why": O10 + "W8-NI: node_id's fallback text, <= 60 calls per run"}),
        (site("near_duplicate_nodes", _W8, 'format!("; and {} more", count - NAMED)'),
         {"from": [fnkey("near_duplicate_nodes", _W8)], "per": "2", "why": O10 + "W8-NI: the `more` closure, once per message"}),
    ]
    # W8-ID (ID_AUDIT): the composed texts' placeholders, at their sites. `described`, `named` and `more` are Strings
    # here, not the integers the generic `^more$|^named$` argument rule classes them as (lib.rs's counts), so each is
    # bounded at its site: a join of <= NAMED pieces with "; " separators, or the `more` text.
    _W8_C = lambda b, why: {"bytes": b, "source": O10 + "W8-ID: " + why}
    _W8_MORE_C = _W8_C(_W8_MORE, "`; and {} more` with one usize")
    O10_ID_AUDIT += [
        (site("append_topology_diagnostics", _W8, ".map(|n| n.id.clone())", full=True), {"n.id": IN}),
        (site("disconnected_parts", _W8, '"part {} (from {}; {} node{}; {})",', full=True, row="format!("),
         {"ordinal+1": _W8_C(_W8_INT, "a part's ordinal (usize, <= NAMED)"),
          "node_id(first)": _W8_C(_W8_ID, "node_id: an admitted node id (<= 128) or node#{index}"),
          "nodes": _W8_C(_W8_INT, "the part's node count (usize)"),
          'ifnodes==1{""}else{"s"}': _W8_C(len("s"), "a static plural suffix"),
          'ifgrounded.contains(&body){"supported"}else{"nosupport"}': _W8_C(len("no support"), "a static support state")}),
        (site("near_duplicate_nodes", _W8, '.map(|&(a, b)| format!("{} and {}", node_id(a), node_id(b)))', full=True),
         {"node_id(a)": _W8_C(_W8_ID, "node_id"), "node_id(b)": _W8_C(_W8_ID, "node_id")}),
        (site("near_duplicate_nodes", _W8, 'format!("{} and {} ({distance:e} m apart)", node_id(a), node_id(b))', full=True),
         {"node_id(a)": _W8_C(_W8_ID, "node_id"), "node_id(b)": _W8_C(_W8_ID, "node_id")}),
        (site("disconnected_parts", _W8, '"the model\'s elements form {} disconnected parts;', full=True, row="format!("),
         {"described": _W8_C(_W8_DESCRIBED, "<= NAMED part descriptions joined by '; '"), "more": _W8_MORE_C}),
        (site("near_duplicate_nodes", _W8, '"{} pair{} of distinct nodes share a position', full=True, row="format!("),
         {"named": _W8_C(_W8_COINCIDENT, "<= NAMED coincident pairs joined by '; '"), "more(coincident_count)": _W8_MORE_C}),
        (site("near_duplicate_nodes", _W8, '"{} pair{} of distinct nodes lie no farther apart than', full=True, row="format!("),
         {"named": _W8_C(_W8_NEARNAMED, "<= NAMED near pairs joined by '; '"), "more(near_count)": _W8_MORE_C}),
    ]
    # W8-RF (REFS_FIRST): the three diagnostics' affected refs. The disconnected parts' refs collect `.take(NAMED)` of
    # the part map's keys (an exact-size iterator: capacity <= NAMED); `refs` pushes at most two ids per named pair,
    # skipping repeats: <= 2 NAMED = 12 Strings in a push-grown Vec (capacity 16).
    O10_REFS_FIRST = list(globals().get("O10_REFS_FIRST", [])) + [
        {"re": r"^parts\.keys\(\)\.take\(NAMED\)\.map\(\|&first\| node_id\(first\)\)\.collect\(\)$",
         "bytes": f"{_W8_NAMED}*(24+{_W8_ID})", "why": O10 + "W8-RF: <= NAMED part ids"},
        {"re": r"^refs\(&mut (coincident\.iter\(\)\.copied\(\)|near\.iter\(\)\.map\(\|&\(a, b, _\)\| \(a, b\)\))\)$",
         "bytes": f"16*24+{2 * _W8_NAMED}*{_W8_ID}", "why": O10 + "W8-RF: <= 2 NAMED node ids, capacity 16"},
    ]
    # W8-JN (LEX_SITE_SIZE): the three `.join("; ")` sites, each at its own bound (the description and the two pair
    # lists), in place of the inherited `()` receiver rule (formation_guard's fired items, 50,176 B).
    _w8_lines, _w8_start, _w8_end = fn_lines("near_duplicate_nodes", _W8)
    _w8_joins = [i + 1 for i in range(_w8_start - 1, _w8_end - 1) if '.join("; ")' in _w8_lines[i]]
    if len(_w8_joins) != 2 or not (_W8_NEAR.index("coincident\n            .iter()\n            .take(NAMED)")
                                   < _W8_NEAR.index("near\n            .iter()\n            .take(NAMED)")):
        raise SystemExit("W8-JN: near_duplicate_nodes' joins changed; re-review their bounds")
    LEX_SITE_SIZE = list(globals().get("LEX_SITE_SIZE", [])) + [
        (site("disconnected_parts", _W8, '.join("; ")'),
         {"class": _W8_DESCRIBED, "why": O10 + "W8-JN: <= NAMED part descriptions joined by '; '"}),
        (Resolved(f"topology.rs:{_w8_joins[0]}"),
         {"class": _W8_COINCIDENT, "why": O10 + "W8-JN: <= NAMED coincident pairs joined by '; '"}),
        (Resolved(f"topology.rs:{_w8_joins[1]}"),
         {"class": _W8_NEARNAMED, "why": O10 + "W8-JN: <= NAMED near pairs joined by '; '"}),
    ]
# W8-BT: validate_components' branch-topology warning sits in its loop over model.components, which D1 refuses
# (P2, D1.4: retained_memory.rs `F::Components`), so it has multiplicity 0 on both routes, guarded rather than assumed.
# Its one unclassified placeholder, the node's member count, is a usize.
if fnkey("branch_topology_invalid", _W8V):
    O10_MULT0_SITES = list(globals().get("O10_MULT0_SITES", [])) + [
        (site("validate_components", _W8V, '"BRANCH_TOPOLOGY_INVALID",', row="diag("),
         O10 + "W8-BT: the branch-topology warning has multiplicity 0 by P2 (D1.4: no components)"),
        (site("validate_components", _W8V, '"diagnostic:component:{}:branch-topology",', row="format!("),
         O10 + "W8-BT: its id has multiplicity 0 by P2 (D1.4: no components)"),
    ]
    # §8.18: the warning's message is a multi-line format!, so its TEXT row is the `format!(` line.
    O10_ID_AUDIT += [(site("validate_components", _W8V, "joins {degree}", full=True, row="format!("),
                      {"degree": {"bytes": 20, "source": O10 + "W8-BT: a usize member count"}})]
# W8-BF (retained-pricing-rules.md §8.18): the branch-topology warning's statement of the branch stress factors still
# applied is added after publication by `state_applied_branch_factors(components, preview_cases, diagnostics)`, called
# once, from run_linear_static_preview_observed_inner's solved preview envelope with `&model.components`. Everything it
# allocates lies inside its loop over those components, which D1 refuses (P2, D1.4: `F::Components`), so the loop is
# bound 0 on both routes (U6-LF: placed first, by its own exact header) and its sites have multiplicity 0,
# guarded by o10_check. Its texts are still bounded at their sites: a side is `{side} SIF to {pipe}` with a static
# side ("header" | "branch", 6 B) and a published record's pipe id (an admitted pipe id, IN128); at most two sides (one
# per role: a role's records name the one pipe its reference names; deduplicated) joined by ", "; the suffix adds its
# literal around them.
if fnkey("state_applied_branch_factors", _W8V):
    _W8_BF = _fn_text("state_applied_branch_factors", _W8V)
    _W8_RUN = _fn_text("run_linear_static_preview_observed_inner", PP_ + "lib.rs")
    _w8_bf_loop = "for component in model_components {"
    _w8_bf_side_t = 'let named = format!("{side} SIF to {}", record["pipe_id"].as_str().unwrap_or_default());'
    _w8_bf_join_t = 'let sides = applied.join(", ");'
    _w8_bf_all_t = 'warning.message.push_str(&format!("; its branch stress factors are still applied ({sides})"));'
    _w8_bf_call = ("validation::state_applied_branch_factors(&model.components, &rendered.preview_cases, "
                   "&mut envelope.diagnostics);")
    _w8_bf = (
        _W8_BF.count(_w8_bf_loop) == 1, _W8_BF.count("for ") == 3,
        _W8_BF.count('for (role, side) in [("branch_header", "header"), ("branch_branch", "branch")]') == 1,
        _W8_BF.count(_w8_bf_side_t) == 1, _W8_BF.count(_w8_bf_join_t) == 1, _W8_BF.count(_w8_bf_all_t) == 1,
        _W8_BF.count("format!(") == 2, _W8_BF.count(".join(") == 1, _W8_BF.count("Vec::new()") == 1,
        _W8_BF.count("applied.push(named);") == 1, _W8_BF.count(".push(") == 1, _W8_BF.count(".push_str(") == 1,
        all(_W8_BF.index(_w8_bf_loop) < _W8_BF.index(t) for t in ("Vec::new()", "format!(", ".push", ".join(")),
        _W8_RUN.count(_w8_bf_call) == 1, _text_of(PP_ + "lib.rs").count("state_applied_branch_factors(") == 1,
        _text_of(_W8V).count("state_applied_branch_factors(") == 1,
    )
    if not all(_w8_bf):
        raise SystemExit(f"W8-BF: state_applied_branch_factors changed; re-review its zero and bounds: {_w8_bf}")
    _W8_BF_SIDE = len(" SIF to ") + len("header") + 128                          # 142
    _W8_BF_JOIN = 2 * _W8_BF_SIDE + len(", ")                                    # 286
    _W8_BF_ALL = len("; its branch stress factors are still applied ()") + _W8_BF_JOIN   # 334
    _W8_BF_WHY = O10 + "W8-BF: multiplicity 0 by P2 (D1.4: no components), as W8-BT"
    O10_LOOPS_FIRST += [(_w8_bf_loop[:-2], "0", _W8_BF_WHY + " (its one caller passes &model.components)",
                         ["validation.rs:state_applied_branch_factors"])]
    O10_MULT0_SITES = list(globals().get("O10_MULT0_SITES", [])) + [
        (site("state_applied_branch_factors", _W8V, _w8_bf_side_t), _W8_BF_WHY + " (a side's text)"),
        (site("state_applied_branch_factors", _W8V, _w8_bf_all_t), _W8_BF_WHY + " (the statement)"),
    ]
    O10_ID_AUDIT += [
        (site("state_applied_branch_factors", _W8V, _w8_bf_side_t, full=True),
         {"side": {"bytes": len("header"), "source": O10 + "W8-BF: a static side, header | branch"},
          'record["pipe_id"].as_str().unwrap_or_default()': IN}),
        (site("state_applied_branch_factors", _W8V, _w8_bf_all_t, full=True),
         {"sides": {"bytes": _W8_BF_JOIN, "source": O10 + "W8-BF: <= 2 sides joined by ', '"}}),
    ]
    LEX_SITE_SIZE = list(globals().get("LEX_SITE_SIZE", [])) + [
        (site("state_applied_branch_factors", _W8V, _w8_bf_join_t),
         {"class": _W8_BF_JOIN, "why": O10 + "W8-BF: <= 2 sides joined by ', ' (multiplicity 0)"}),
        (site("state_applied_branch_factors", _W8V, _w8_bf_all_t),
         {"class": _W8_BF_ALL, "why": O10 + "W8-BF: the statement, its literal around <= 2 sides (multiplicity 0)"})]

# NP-MS (retained-pricing-rules.md §8.18): validate_model_settings refuses a malformed
# `model_settings.node_proximity_tolerance` with one blocking diagnostic carrying the accessor's code and message. D1
# admits the record (no D1.3-D1.8 clause names it; its unit is raw text within RawTextBytes, l <= 128), and a finite
# value that is not positive, a unit outside the catalog's lengths or a conversion that over- or underflows
# deserializes and reaches validation, so the site keeps the chain's own multiplicity (once per validate_model_inputs
# call). The accessor `node_proximity_tolerance_m` returns at most one of its five refusal texts per call; the chain
# charges each at every call (two validation calls and the topology call at E), an over-bound kept. Their bounds:
# the path constant at its literal bytes (NP-PT, read here), the authored unit IN128, the value in Display (327) or
# Debug (24) by the inherited f64 classes. The diagnostic's message `error.message` is one of those texts, at most
# 39 + 58 + 327 = 424 B, priced by its inherited class (8,192 B); its refs are the one path String (the default
# vec! element, 24 + 128 B; the to_string site at the literal's bytes).
_NP_MS = PP_ + "model_settings.rs"
if fnkey("validate_model_settings", _W8V) and fnkey("tolerance_not_finite_message", _NP_MS):
    _NP_TEXT = _text_of(_NP_MS)
    _NP_PATH = _re.findall(r'^pub const NODE_PROXIMITY_TOLERANCE_PATH: &str = "([^"\\]*)";$', _NP_TEXT, _re.M)
    _NP_VMS = _fn_text("validate_model_settings", _W8V)
    _NP_T = {
        "tolerance_not_finite_message": 'format!("{NODE_PROXIMITY_TOLERANCE_PATH}.value is {value}; a finite length greater than zero is required.")',
        "tolerance_not_positive_message": 'format!("{NODE_PROXIMITY_TOLERANCE_PATH}.value is {value}; the tolerance must be greater than zero.")',
        "tolerance_unit_not_length_message": 'format!("{NODE_PROXIMITY_TOLERANCE_PATH}.unit `{unit}` is not a length unit of the units catalog.")',
        "tolerance_zero_in_metres_message": 'format!("{NODE_PROXIMITY_TOLERANCE_PATH} is {value:?} {unit}, which is zero in metres; the tolerance must be greater than zero.")',
        "tolerance_not_finite_in_metres_message": 'format!("{NODE_PROXIMITY_TOLERANCE_PATH} is {value:?} {unit}, which is not finite in metres; a finite length greater than zero is required.")',
    }
    _np = (
        len(_NP_PATH) == 1,
        all(_fn_text(fn, _NP_MS).count(t) == 1 for fn, t in _NP_T.items()),
        _re.sub(r"//[^\n]*", "", _NP_TEXT.split("#[cfg(test)]")[0]).count("format!(") == len(_NP_T),
        _NP_VMS.count("diag(") == 1, _NP_VMS.count("error.message,") == 1, _NP_VMS.count("error.code,") == 1,
        _NP_VMS.count('"blocking",') == 1, _NP_VMS.count('"diagnostic:model-settings:node-proximity-tolerance",') == 1,
        _NP_VMS.count("vec![crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH.to_string()],") == 1,
        _fn_text("validate_model_inputs", _W8V).count("validate_model_settings(model, diagnostics);") == 1,
        _text_of(_W8V).count("validate_model_settings(") == 2,
    )
    if not all(_np):
        raise SystemExit(f"NP-MS: the node-proximity tolerance's refusal texts changed; re-review their bounds: {_np}")
    _NP_PB = len(_NP_PATH[0].encode())                                                    # 39 (NP-PT)
    _NP_PT = {"NODE_PROXIMITY_TOLERANCE_PATH": {"bytes": _NP_PB, "source": O10 + f"NP-PT: the path constant, a static literal of {_NP_PB} B"}}
    for _fn, _t in _NP_T.items():
        _audit = dict(_NP_PT)
        if "{unit}" in _t:
            _audit["unit"] = IN
        O10_ID_AUDIT += [(site(_fn, _NP_MS, _t, full=True), _audit)]
    O10_ID_AUDIT += [(site("validate_model_settings", _W8V, ".to_string()],", full=True),
                      {"crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH":
                       {"bytes": _NP_PB, "source": O10 + f"NP-PT: the path constant, a static literal of {_NP_PB} B"}})]

# ---- The silent-zero guard (DEL-04-07 Design pressure-contract.md, "Load cases that apply nothing";
# retained-pricing-rules.md §8.15). Each rule applies only where the guard's code exists, so earlier revisions price as
# before.
_SZ_V = PP_ + "validation.rs"
_SZ_R = PP_ + "case_state/resolve.rs"
if fnkey("validate_required_collections", _SZ_V) and '"LOAD_CASE_APPLIES_NO_ACTION",' in _fn_text("validate_required_collections", _SZ_V):
    _SZ_BODY = _fn_text("validate_required_collections", _SZ_V)
    # SZ-TX (guard): the guard's one diagnostic site, read at the snapshot: one loop over the authored load cases that
    # apply nothing, one LOAD_CASE_APPLIES_NO_ACTION diag whose id is `diagnostic:load-case:{}:applies-no-action` over
    # stable_suffix(&case.id), whose message is one static literal (priced by the inventory at its own bytes) and whose
    # refs are the one case id.
    _SZ_LIT = _re.findall(r'"(the load case applies no action:(?:[^"\\]|\\.)*)"', _SZ_BODY)
    _sz_guards = (
        len(_SZ_LIT) == 1,
        _SZ_BODY.count("for case in model.load_cases.iter().filter(|case| case_applies_nothing(case)) {") == 1,
        _SZ_BODY.count('"LOAD_CASE_APPLIES_NO_ACTION",') == 1,
        _SZ_BODY.count('&format!("diagnostic:load-case:{}:applies-no-action", stable_suffix(&case.id)),') == 1,
        _SZ_BODY.count("vec![case.id.clone()],") == 1,
        _SZ_BODY.count("diag(") == 4,
    )
    if not all(_sz_guards):
        raise SystemExit(f"SZ-TX: the silent-zero guard's diagnostic changed; re-review its bounds: {_sz_guards}")
    # SZ-ID (ID_AUDIT): the id format's stable_suffix(&case.id) (an authored case id with ':' replaced, same length,
    # IN128) and the refs' case.id clone (IN128), as the sibling per-case sites in validation.rs are priced. The site is
    # inside the loop over load cases, so it keeps its full multiplicity: once per authored case per call.
    O10_ID_AUDIT += [
        (site("validate_required_collections", _SZ_V,
              'format!("diagnostic:load-case:{}:applies-no-action", stable_suffix(&case.id))', full=True),
         {"stable_suffix(&case.id)": IN}),
        (site("validate_required_collections", _SZ_V, "vec![case.id.clone()],", full=True), {"case.id": IN}),
    ]
# SZ-CP (EDGE_ZERO): model_loads_every_case decides a connector's preload by `connector_preloads`, passed as a function
# value to `model.components.iter().any(..)`, its one call; it decodes the connector (joint::connector_spec) and its
# stiffness. P2 (D1.4) leaves no components, so the edge is never taken on either route and its callees keep only
# their own callers' multiplicity. Guarded by the source: the one call, over model.components, and no other caller.
if fnkey("connector_preloads", _SZ_V) and fnkey("model_loads_every_case", _SZ_V):
    _SZ_MLEC = _fn_text("model_loads_every_case", _SZ_V)
    if (_SZ_MLEC.count("connector_preloads") != 1 or "model.components.iter().any(connector_preloads)" not in _SZ_MLEC
            or _text_of(_SZ_V).count("connector_preloads") != 2):
        raise SystemExit("SZ-CP: connector_preloads' call changed; re-review its zero edge")
    O10_EDGE_ZERO += [(fnkey("model_loads_every_case", _SZ_V), fnkey("connector_preloads", _SZ_V),
                       O10 + "SZ-CP: connector_preloads runs once per model component, none under " + D14)]
# SZ-LS: resolve_case's two new findings (LOAD_STATE_CASE_APPLIES_NO_ACTION through `block`, LOAD_STATE_SOURCE_EXCLUDED)
# are load-state resolution, reached only under case_state::is_load_state (lib.rs's load_state branch and the
# captured load-state replay). D1.3 refuses every authored load/reference-state field on every branch (retained_memory.rs
# family_clauses: ReferenceConfigurations, MaterialExpansionLaw, RequestExpansionLaws; per case AnalysisState), so
# is_load_state is false on both routes (B7b-LS; route L also has no exact contract) and the base chain's case_state/
# file zero holds. Their text sites have multiplicity 0, guarded rather than assumed.
if fnkey("resolve_case", _SZ_R) and '"LOAD_STATE_SOURCE_EXCLUDED",' in _fn_text("resolve_case", _SZ_R):
    _SZ_LS = O10 + "SZ-LS: load-state resolution has multiplicity 0 by D1.3 (no authored load/reference-state field; B7b-LS)"
    O10_MULT0_SITES = list(globals().get("O10_MULT0_SITES", [])) + [
        (site("resolve_case", _SZ_R, '"LOAD_STATE_SOURCE_EXCLUDED",', row="diag("), _SZ_LS),
        (site("resolve_case", _SZ_R, '"the load case applies no action: none of its {} stored', row="format!("), _SZ_LS),
        (site("resolve_case", _SZ_R, "let mut text = ids.iter().take(crate::formation_guard::NAMED)"), _SZ_LS),
    ]
    # refuse_all_reference_states (called once in lib.rs's load-state branch, after refuse_resolved_span_strain) reads
    # the resolved cases, which exist only for a load-state document; its LOAD_INPUT_MISSING site is guarded the same way.
    if fnkey("refuse_all_reference_states", _SZ_R):
        O10_MULT0_SITES += [(site("refuse_all_reference_states", _SZ_R, '"LOAD_INPUT_MISSING",', row="diag("), _SZ_LS)]

# S1B-MV (W-13 S1b, model document 0.7.0; retained-pricing-rules.md §8.17): the load/reference-state version-mismatch
# diagnostic (`case_state/resolve.rs`) and the joint authoring refusal (`joint.rs authoring_findings`) name the current
# model version through the shared constant. The argument `crate::MODEL_SCHEMA_VERSION` is a `&'static str` whose bytes
# are model_format.rs's one plain literal, read here at the snapshot; any other form stops the run.
_S1B_MV = _re.search(r'^pub const MODEL_SCHEMA_VERSION: &str = "([0-9.]+)";$',
                     open(_os.path.join(_SNAP, PP_ + "model_format.rs"), encoding="utf-8").read(), _re.M)
if _S1B_MV is None:
    raise SystemExit("S1B-MV: MODEL_SCHEMA_VERSION is not one plain version literal; re-review the version arguments")
O10_ARGS_FIRST = list(globals().get("O10_ARGS_FIRST", [])) + [
    {"re": r"^crate::MODEL_SCHEMA_VERSION$", "max": len(_S1B_MV.group(1).encode()),
     "why": O10 + "S1B-MV: the current model version constant, a static literal of " + str(len(_S1B_MV.group(1).encode())) + " B"}]
