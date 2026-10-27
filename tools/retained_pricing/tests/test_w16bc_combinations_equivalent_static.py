"""W16B-CB, W16B-RD, W16C-ES, W16BC-BN and W16BC-ID (retained-pricing-rules.md §8.27): W-16 slices b and c add the
exact route's load combinations and equivalent-static generation. D1 admits neither on a retained branch, so their
entry edges are zero; the pressure-2 reader's combination test runs and its direct pass is counted once; by-name edges
into the two new modules' private functions are zero; a changed premise or guard stops the run."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
RM = "core/product_physics/src/retained_memory.rs"
LIB = "core/product_physics/src/lib.rs"
CL = "core/product_physics/src/certified_loads.rs"
PE = "core/reporting/result_export/src/physics_evidence.rs"
EC = "core/product_physics/src/exact_combinations.rs"


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def snapshot_with(tmp_path, path, old, new):
    """A copy of the source tree with one replacement in `path` (the product crates' sources only)."""
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def short(key):
    return str(key).split(":")[-1]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_entry_edges_are_zero(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    zero = lambda label: {(short(a), short(b)) for a, b, why in d["O10_EDGE_ZERO"] if label + ":" in why}
    assert zero("W16B-CB") == {("run_linear_static_preview_observed_inner", "append"),
                               ("run_linear_static_preview_observed_inner", "state_operand_standing")}
    assert zero("W16B-RD") == {("validate_pressure_evidence", "direct_view"), ("validate_pressure_evidence", "validate")}
    assert zero("W16C-ES") == {
        ("append_equivalent_static_generated_loads", callee) for callee in (
            "is_exact", "replaced_span_ids", "selects_timoshenko", "curved_bend_realized_pipe_ids",
            "generation_mass_per_length", "refuse_generated_id_collisions", "wind_span_replaced")} | {
        ("intensity", "is_generated_equivalent_static_load"), ("intensity", "is_exact"),
        ("solve_prepared_load_case", "exact_equivalent_static_record"),
        ("physical_source_built", "exact_equivalent_static_record"),
        ("physical_source_built", "build_load_case_primitive_loads"),
        ("validate_profile", "carries_fields"),
        ("validate_physics_evidence_in", "equivalent_static_record"),
        ("validate_transport_metadata_in", "equivalent_static_record")}
    by_name = {(short(b), short(a)) for a, b, why in d["O10_EDGE_ZERO"] if "W16BC-BN" in why}
    assert {callee for callee, _ in by_name} == {"evidence", "with", "stiffness", "text", "add", "signum"}
    per_call = [(short(a), short(b), per) for a, b, per, why in d["EDGE_PER_CALL"] if "W16B-RD" in why]
    assert per_call == [("validate_pressure_evidence", "validate_direct_pressure_evidence", "1")]
    audits = {str(key).split("/src/")[-1]: value for key, value in d["O10_ID_AUDIT"]
              if str(key).split("/src/")[-1].startswith(("exact_combinations.rs:", "exact_combination_evidence.rs:"))}
    # Three IN128 operand case ids joined by ", "; the longest envelope mode literal (min_abs, max_abs).
    assert sorted(v["cases"]["bytes"] for v in audits.values() if "cases" in v) == [388, 388]
    assert [v for v in audits.values() if any(k.startswith("matchmode") for k in v)][0].popitem()[1]["bytes"] == 7


def test_route_e_reads_load_reference_states_from_the_direct_pass(monkeypatch):
    e = {(short(a), short(b)) for a, b, why in decisions(monkeypatch, route="E")["EDGE_ZERO"] if why.startswith("B7b-LR")}
    assert e == {("validate_direct_pressure_evidence", "validate_pressure_load_reference_evidence")}


@pytest.mark.parametrize("path, old, new, label", [
    # D1.4's exact clause no longer refuses combinations.
    (RM, "    if exact && !m.combinations.is_empty() {\n        return refuse(C::Invocation, F::Combinations);",
     "    if false && exact && !m.combinations.is_empty() {\n        return refuse(C::Invocation, F::Combinations);", "W16B-CB"),
    # D1.5 no longer refuses a generating case.
    (RM, "        if case.equivalent_static.is_some() {\n            return refuse(C::Case, F::EquivalentStatic);",
     "        if false {\n            return refuse(C::Case, F::EquivalentStatic);", "W16C-ES"),
    # The reader enters its combination pass unconditionally.
    (PE, "    if crate::exact_combination_evidence::present(source) {\n",
     "    if true || crate::exact_combination_evidence::present(source) {\n", "W16B-RD"),
    # A by-name callee becomes visible outside its module.
    (EC, "    fn evidence(&self) -> Value {", "    pub(crate) fn evidence(&self) -> Value {", "W16BC-BN"),
])
def test_a_changed_premise_or_guard_stops(tmp_path, monkeypatch, path, old, new, label):
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises(SystemExit, match=label):
        decisions(monkeypatch, snapshot=snap)


@pytest.mark.parametrize("path, old, new, stop", [
    # The append runs off the exact route.
    (LIB, "let exact_combinations = (pressure_runtime::is_exact(&model) && !model.combinations.is_empty()).then(|| {",
     "let exact_combinations = (!model.combinations.is_empty()).then(|| {", "KD5C4: B7a E14 full-file source changed"),
    # The certified producer tests the generated namespace before its generation arm.
    (CL, "        if let Some(generation) = &case.equivalent_static {\n",
     "        let _ = is_generated_equivalent_static_load(case, &load.load_id, None);\n"
     "        if let Some(generation) = &case.equivalent_static {\n",
     "W1 reductions source/lifetime pin changed: core/product_physics/src/certified_loads.rs"),
])
def test_a_changed_pinned_guard_stops_at_its_pin_first(tmp_path, monkeypatch, path, old, new, stop):
    """lib.rs and certified_loads.rs carry full-file pins, which stop such a change first; the W16B-CB and W16C-ES guards
    on the same statements stand behind them."""
    snap = snapshot_with(tmp_path, path, old, new)
    with pytest.raises(ValueError, match=stop):
        decisions(monkeypatch, snapshot=snap)
