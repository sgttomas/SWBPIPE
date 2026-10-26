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

Bound to the two files' sha256 (both are W-15's new modules), the exact allocating statements and the readers' call
sites; a snapshot without them adds nothing (earlier registered trees keep their coefficients), a partly present or
changed owner stops the run."""
import hashlib
import os

PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"
RC = PP + "rigid_component.rs"
CB = RE + "component_body.rs"
PINS = {RC: "0203c6aa752219d4af9df90f4f31c2fc8a4d54960625cda790bab14d8a0d9dc9",
        CB: "ace48d130d710eae3412c1b850962f78673ba03e143044c0fbc5d48f2265b514"}

# (file, statement, count): each owning statement the formulas price or the premise empties.
STATEMENTS = (
    (RC, "model.pipe_segments.iter().map(|pipe| (pipe.id.as_str(), pipe)).collect()", 1),
    (RC, "let pipes = pipe_map(model);", 2),
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
