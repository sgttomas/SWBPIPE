"""W-15 D3 (DEL-03-05 Design rigid-component-stiffness-scaling.md §2.8; retained-pricing-rules.md §8.21): the positive
owners the rigid-component stiffness scaling (slice a) and the reference-section identification (slice d) add outside
the chain's TEXT inventory. D1 admits no component (P2), so no member is scaled, no row carries `component_body` and no
coverage entry lists bodies; what remains on D1 is the predicate's and the readers' working sets, priced here.

W15-PM (producer, per ordinary run, BuildScratch): rigid_component's `pipe_map` (a HashMap<&str, &PreviewPipe> over the
m pipe segments, collected from an exact-size iterator: one table, no growth) is built by `conflicts` (validation's
append_mapping_conflicts, once after validate_components' loop) and by `applied_scales` (build_model_for_members, once
per build), which holds its own map while `conflicts` builds a second: at most two live at once. Every other owner of
both functions holds nothing on D1: the curved-bend and replaced-span sets, the member order, the per-member component
lists, the findings, the conflicted set and the returned scales are each collected from, or pushed only inside a loop
over, `model.components`, which P2 empties (an empty collection allocates nothing). Charged as two tables of m entries.
W15-RB (readers, per base read on the projected successor, T17 V5): component_body's `source_bodies` (called by the
pressure-2 and preview-physics-2 base readers and by route L's model checks, one call at a time) collects the case
references of the evidence's case lists (flatten: push-grown to pushcap(cases) with its previous backing; twice, one at
a time) and one case's coverage entries (<= 2, exact); the readers' shape checks each clone one stress extremum
(`extremum_without_record`) or one coverage entry (`coverage_without_bodies`) as a statement temporary, one at a time.
An extremum clone is bounded by one pipe_stress_extrema entry of the D-5 census (o4_owners_model MAXIMUM_EVIDENCE) and
the preview extremum (<= 15 entries, keys <= 40 B), componentwise; a coverage entry holds <= 2m member ids (IN128) and
<= 4 keys. On D1 the body list, the listed and record sets, the component set and model_bodies' map hold nothing
(no coverage entry lists bodies and the captured model's components are empty, P2). Both clones are charged, with the
case and entry lists, beside U4B-RD's and W7-RD's sets.

W15C (W-15 slice c, the eccentric offset; retained-pricing-rules.md §8.26): Topology::of builds a third pipe map with
the node positions, the per-node member lists, the mapping table and the realized and replaced sets, but only for
judge_all, which both offset entry points call after their allocation-free first pass; that pass returns when no
component carries `eccentric_offset`, which P2 makes every D1 request (W15C-NO, o10_rules.py). So nothing of it is
live on D1 and W15-PM's two tables are unchanged.

Bound to the two files' sha256 (both are W-15's new modules), the exact allocating statements and the readers' call
sites; a snapshot without them adds nothing (earlier registered trees keep their coefficients), a partly present or
changed owner stops the run."""
import hashlib
import os

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
RC = PP + "rigid_component.rs"
CB = RE + "component_body.rs"
# RT-A3 reducer 2b (§8.41): rigid_component.rs extracts member_mappings (Topology::of's mapping loop, unchanged) and
# judge_reducer/body_ends (judge_all's per-reducer body, now collected), and adds reducer_body_members and
# pressure_step_run_evidence for the pressure step; W15C-NO and P2 still empty every component path on D1.
# §8.33 (RT-A3 increment 1): rigid_component.rs's STRESS_KINDS gains branch_intensified_bending_stress_v1 (18 kinds), a
# static array read only by annotate_component_bodies' `contains` scan of existing rows; no owner, statement or reader
# call changes, and D1 has no component body (P2).
PINS = {RC: "e8095fe56cdd9adc73e35ba234825300eafd6b7b1639668e3d65a2dec378ef90",
        CB: "ace48d130d710eae3412c1b850962f78673ba03e143044c0fbc5d48f2265b514"}

# (file, statement, count): each owning statement the formulas price or the premise empties.
STATEMENTS = (
    (RC, "model.pipe_segments.iter().map(|pipe| (pipe.id.as_str(), pipe)).collect()", 1),
    (RC, "let pipes = pipe_map(model);", 3),
    # RT-A3 reducer 2b (§8.41): the second Topology::of is pressure_step_run_evidence's; member_mappings now builds
    # Topology::of's mapping table (moved, same statement) and reducer_body_members' set. Both new entries are called
    # only inside reducers_in_regions' loop over model.components (P2) behind D1.5's absent/empty regions, so nothing
    # of them is live on D1; the call sites below bind that premise.
    (RC, "let topology = Topology::of(model);", 2),
    (RC, "let mut mapping: HashMap<&str, Mapping> = HashMap::new();", 1),
    (RC, "member_mappings(model)", 2),
    (RC, "let mut oriented = Vec::with_capacity(axes.len());", 1),
    (PP + "pressure_runtime.rs", "for reducer in exact_admission::reducers_in_regions(model, case) {", 1),
    (PP + "pressure_runtime.rs", "super::rigid_component::pressure_step_run_evidence(model, reducer.component)", 1),
    (PP + "exact_admission.rs", ".get_or_insert_with(|| super::rigid_component::reducer_body_members(model))", 1),
    (RC, "let conflicted = conflicts(model).into_iter().map(|(member, _)| member).collect::<HashSet<_>>();", 1),
    (RC, "for component in model.components.iter().filter(|component| is_rigid_family(component)) {", 1),
    (RC, "let mut by_member: BTreeMap<&str, Vec<&str>> = BTreeMap::new();", 1),
    (RC, "let mut order: Vec<&str> = Vec::new();", 1),
    (RC, "let mut found = Vec::new();", 1),
    (PP + "validation.rs", "crate::rigid_component::append_mapping_conflicts(model, diagnostics);", 1),
    (PP + "lib.rs", "let rigid_scales = rigid_component::applied_scales(model);", 1),
    (CB, "let mut copy = value.clone();", 1),
    (CB, "Value::Array(entries) => entries.iter().collect(),", 1),
    (CB, "entry @ Value::Object(_) => vec![entry],", 1),
    (CB, "let listed: HashSet<&str> = listed.unwrap_or_default().into_iter().collect();", 1),
    (RE + "physics_evidence.rs", "crate::component_body::source_bodies(", 1),
    (RE + "physics_evidence.rs", "crate::component_body::extremum_without_record(", 1),
    (RE + "preview_physics_evidence.rs", "crate::component_body::source_bodies(", 1),
    (RE + "preview_physics_evidence.rs", "crate::component_body::extremum_without_record(", 1),
    (RE + "preview_physics_evidence.rs", "crate::component_body::coverage_without_bodies(", 1),
    (RE + "retained_precision.rs", "crate::component_body::extremum_without_record(", 1),
    (RE + "retained_precision.rs", "crate::component_body::coverage_without_bodies(", 1),
    (RE + "section_basis.rs", "crate::component_body::coverage_without_bodies(", 1),
    (RE + "u4b_evidence.rs", "crate::component_body::model_checks(", 2),
)
# RT-A3 reducer 2b (§8.41): (file, calling function, its D1-empty loop header, the one call after it).
PRESSURE_STEP_PREMISES = (
    (PP + "exact_admission.rs", "reducers_in_regions",
     "for component in model.components.iter().filter(|component| component_family(component) == ExactFamily::Reducer) {",
     "super::rigid_component::reducer_body_members(model)"),
    (PP + "pressure_runtime.rs", "refuse_reducers_outside_step",
     "for reducer in exact_admission::reducers_in_regions(model, case) {",
     "super::rigid_component::pressure_step_run_evidence(model, reducer.component)"),
)
# Each is called exactly once in product_physics' sources (test modules included, conservatively).
PRESSURE_STEP_ENTRIES = ("pressure_step_run_evidence", "reducer_body_members")
# The preview extremum (ordinary_caps PREVIEW): <= 15 entries, three static strings <= 120 B, the pipe id (IN128) and
# the result id (<= 1024 B), keys <= 40 B each.
PREVIEW_EXTREMUM = {"arr": 0, "obj": 1, "ent": 15, "strb": 128 + 1024 + 3 * 120, "keyb": 15 * 40}
COVERAGE_KEYS = 4


def _read(snapshot, rel):
    path = os.path.join(snapshot, rel)
    if not os.path.exists(path):
        return None
    with open(path, encoding="utf-8") as stream:
        return stream.read()


def inspect(snapshot):
    """True when W-15's owners are present as reviewed; False on an earlier revision; anything else stops."""
    texts = {rel: _read(snapshot, rel) for rel in {rel for rel, _, _ in STATEMENTS} | set(PINS)}
    if texts[RC] is None and texts[CB] is None:
        if any(needle in (texts[rel] or "") for rel, needle, _ in STATEMENTS if rel not in PINS):
            raise SystemExit("W15 owners: the W-15 calls are present without their modules; re-review")
        return False
    for rel, pin in PINS.items():
        if texts[rel] is None or hashlib.sha256(texts[rel].encode()).hexdigest() != pin:
            raise SystemExit(f"W15 owners: {rel} changed or is missing; re-review the owners it prices")
    found = [(texts[rel] or "").count(needle) for rel, needle, _ in STATEMENTS]
    if found != [count for *_, count in STATEMENTS]:
        raise SystemExit(f"W15 owners: an owning statement or reader call changed; re-review W15-PM/W15-RB: {found}")
    # RT-A3 reducer 2b (§8.41, RTA3R2B-NZ): check listed spellings, counts and header-before-call order, not loop
    # containment. Full-file pins and source review establish that these entries sit inside D1-empty loops in the
    # selected source (reducers_in_regions' component loop and the loop over its result in refuse_reducers_outside_step).
    callers = {}
    for root, _, names in os.walk(os.path.join(snapshot, PP)):
        for name in names:
            if name.endswith(".rs"):
                with open(os.path.join(root, name), encoding="utf-8") as stream:
                    body = stream.read()
                for entry in PRESSURE_STEP_ENTRIES:
                    callers[entry] = callers.get(entry, 0) + body.count(entry + "(") - body.count("fn " + entry + "(")
    if callers != {entry: 1 for entry in PRESSURE_STEP_ENTRIES}:
        raise SystemExit(f"W15 owners: a pressure-step entry gained or lost a caller; re-review its zero multiplicity: {callers}")
    for rel, fn, header, call in PRESSURE_STEP_PREMISES:
        text = texts.get(rel) or _read(snapshot, rel) or ""
        start = text.find(f"fn {fn}")
        ends = [text.find(n, start + 1) for n in ("\nfn ", "\npub(crate) fn ", "\npub fn ")] if start >= 0 else []
        end = min([e for e in ends if e >= 0] or [len(text)])
        at_header, at_call = text.find(header, start, end), text.find(call, start, end)
        if not (0 <= start < at_header < at_call) or text.count(call) != 1:
            raise SystemExit(f"W15 owners: the pressure-step premise changed ({fn}); re-review its zero multiplicity")
    return True


def producer_maps(m):
    """W15-PM: two exact-size HashMap<&str, &PreviewPipe> tables of m entries, live at once in applied_scales."""
    from g3lib import HashReq
    return HashReq("(&str,&T)", m) * 2


def extremum_facts():
    """One cloned extremum: the census entry and the preview extremum, componentwise."""
    import o4_owners_model as o4
    census = o4.maximum_evidence_facts()
    return {k: max(census.get(k, 0), PREVIEW_EXTREMUM.get(k, 0)) for k in ("arr", "obj", "ent", "strb", "keyb")}


def reader_working_set(cases, m, VF):
    """W15-RB: source_bodies' case and entry lists, one cloned extremum and one cloned coverage entry."""
    from g3lib import s, pushcap
    lists = s("&Value") * (pushcap(cases) + pushcap(cases) // 2 + 2)
    e = extremum_facts()
    extremum = VF(arr=e["arr"], obj=e["obj"], ent=e["ent"], strb=e["strb"], keyb=e["keyb"]).tree()
    coverage = VF(arr=2 * m, obj=1, ent=COVERAGE_KEYS, strb=2 * m * 128 + 8, keyb=COVERAGE_KEYS * 40).tree()
    return lists + extremum + coverage
