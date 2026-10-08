"""I90 SR-RS repair 2: the mutant schema. Each mutant is a guarded edit in a scratch copy of the head
(WT/scratch/i90_b1_sr_rs/repair_02/mut), active only when the environment variable I90_MUT names it, so
one build serves the control (I90_MUT unset) and every mutant. Each `old` must occur exactly once."""
import json, sys
RE = sys.argv[1]
RS = f"{RE}/src/retained_precision.rs"
s = open(RS, encoding="utf-8").read()
EDITS = [
    # Item 1, (f) at G3.
    ("F1", "the G3 source-index conjunct dropped", '''                && u(&s["index"]) == si as u64
                && owner["kind"] == "case"''', '''                && (mx("F1") || u(&s["index"]) == si as u64)
                && owner["kind"] == "case"'''),
    ("F2", "the G3 source-owner conjunct dropped", '''.is_some_and(|c| c["basis_ref"]["ref_id"] == owner["case_id"]),''', '''.is_some_and(|c| mx("F2") || c["basis_ref"]["ref_id"] == owner["case_id"]),'''),
    ("F3", "the G3 basis-index conjunct dropped", '''            u(&mb["index"]) == mi as u64
                && list(&mb["case_indices"])''', '''            (mx("F3") || u(&mb["index"]) == mi as u64)
                && list(&mb["case_indices"])'''),
    ("F4", "the G3 case_indices uniqueness dropped", '''.all(|ci| u(ci) < cs.len() as u64 && seen.insert(u(ci))),''', '''.all(|ci| u(ci) < cs.len() as u64 && (mx("F4") || seen.insert(u(ci)))),'''),
    ("F5", "the G3 case_indices range dropped", '''.all(|ci| u(ci) < cs.len() as u64 && (mx("F4") || seen.insert(u(ci)))),''', '''.all(|ci| (mx("F5") || u(ci) < cs.len() as u64) && (mx("F4") || seen.insert(u(ci)))),'''),
    ("F6", "the G5 ordinary basis reference dropped", '''        fail(
            at(
                &b["material_bases"],
                &o["material_basis_ref"],''', '''        fail(
            mx("F6") || at(
                &b["material_bases"],
                &o["material_basis_ref"],'''),
    # Item 2, (g).
    ("G1", "combinations back to list().is_empty()", '''.all(|k| model.get(*k).is_none_or(|v| v.as_array().is_some_and(Vec::is_empty)))''', '''.all(|k| if (mx("G1") && *k == "combinations") || (mx("G2") && *k == "components") { list(&model[*k]).is_empty() } else { model.get(*k).is_none_or(|v| v.as_array().is_some_and(Vec::is_empty)) })'''),
    ("G2", "components back to list().is_empty()", None, None),
    # Item 3, C2's cause table.
    ("C1", "the C2 cause table removed", '''        if c["status"] == "unavailable"
            && c["reason"]["cause"]["kind"] != "prepared_product_failure"''', '''        if !mx("C1") && c["status"] == "unavailable"
            && c["reason"]["cause"]["kind"] != "prepared_product_failure"'''),
    ("C2", "the source_error branch always passes", '''                "source_error" => {
                    phase == "preparation"''', '''                "source_error" => {
                    mx("C2") || phase == "preparation"'''),
    ("C3", "the precondition keying coarsened to TS's set of four codes", '''                        && match text(&cause["precondition"]) {''', '''                        && if mx("C3") { matches!(code, "source_unavailable" | "resource_admission_not_available" | "upstream_no_wrap_not_established" | "caller_not_qualified") } else { match text(&cause["precondition"]) {'''),
    ("C4", "the unavailable_precondition branch always passes", '''                "unavailable_precondition" => {
                    matches!(phase, "routing" | "preparation")''', '''                "unavailable_precondition" => {
                    mx("C4") || matches!(phase, "routing" | "preparation")'''),
    ("C5", "the receipt_failure branch always passes", '''                "receipt_failure" => {
                    phase == "receipt"''', '''                "receipt_failure" => {
                    mx("C5") || phase == "receipt"'''),
    ("C6", "the facade_failure branch always passes", '''                "facade_failure" => {
                    phase == "facade"''', '''                "facade_failure" => {
                    mx("C6") || phase == "facade"'''),
    ("C7", "the kernel-reason branch always passes", '''                _ => {
                    phase == "kernel"''', '''                _ => {
                    mx("C7") || phase == "kernel"'''),
    # Item 4, the transport metadata check.
    ("T1", "the G7 metadata check not called", '''    preview_physics_transport_metadata(&projected).map_err(base_error)?;''', '''    if !mx("T1") {
        preview_physics_transport_metadata(&projected).map_err(base_error)?;
    }'''),
    ("T2", "the formulation limitations check dropped", '''            && f["limitations"] == table["supported_profile_limitations"],''', '''            && (mx("T2") || f["limitations"] == table["supported_profile_limitations"]),'''),
    ("T3", "the extrema constants dropped", '''                    && x["approximation"] == "piecewise_quadratic_straight_section_statics"''', '''                    && (mx("T3") || x["approximation"] == "piecewise_quadratic_straight_section_statics")'''),
    ("T4", "the gate reason rule dropped", '''                Some(true) => GATES.contains(&text(&gate["reason"])),''', '''                Some(true) => mx("T4") || GATES.contains(&text(&gate["reason"])),'''),
    ("T5", "the cross-case attribution rule dropped", '''        attribution_sets.len() <= 1,''', '''        mx("T5") || attribution_sets.len() <= 1,'''),
]
for mid, what, old, new in EDITS:
    if old is None:
        continue
    assert s.count(old) == 1, (mid, s.count(old))
    s = s.replace(old, new)
# C3 closes the else-block it opened, after the precondition match's last arm.
old = '''                            _ => false,
                        }
                }'''
assert s.count(old) == 1
s = s.replace(old, '''                            _ => false,
                        } }
                }''')
s += '''
/// I90 mutant schema (scratch only): true when I90_MUT names this mutant.
fn mx(id: &str) -> bool {
    std::env::var("I90_MUT").is_ok_and(|v| v == id)
}
'''
open(RS, "w", encoding="utf-8").write(s)
json.dump([{"id": m, "what": w} for m, w, _, _ in EDITS], open(sys.argv[2], "w"), indent=1)
print(f"{len(EDITS)} mutants written")
