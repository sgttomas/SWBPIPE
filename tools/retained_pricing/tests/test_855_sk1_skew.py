"""S-K1 skewed restraint directions, R-6/R-7 (§8.55).

- SKW-D1: D1.6 refuses `direction` on every support, so no skewed record is admitted: the skew basis, its solve,
  attribution, the along-direction rows and their combination rows, and the readers' along checks are zero edges, each
  guarded on its source statement.
- SKW-PC: support_record's group calls restate the case's two groups; the along group runs only with an along row.
- SKW-BN: `summary_values.into_iter().reduce(f64::max)` is Iterator::reduce, not RestraintBasis::reduce.
- SKW-ID: FrameKernelError's InvalidInput arm is priced at its static reasons' bytes.
- A changed guard stops the run.

These are source warrants, not runtime allocation observations.
"""
from pathlib import Path
import re
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
FK = "core/solver/frame_kernel/src/"
PP = "core/product_physics/src/"
RE = "core/reporting/result_export/src/"


def decisions(monkeypatch, snapshot=SOURCE, route="L"):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def _name(key):
    return str(key).rsplit(":", 1)[-1]


def _file(key):
    return str(key).split("/src/")[-1].split(":")[0]


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_skew_rules_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    zero = {(_file(a), _name(a), _file(b), _name(b)) for a, b, *_ in d["O10_EDGE_ZERO"]}
    for edge in [("lib.rs", "build_model_for_basis", "skew.rs", "build_basis"),
                 ("lib.rs", "solve_prepared_load_case", "skew.rs", "solve"),
                 ("lib.rs", "solve_prepared_load_case", "skew.rs", "attribute"),
                 ("preview_physics.rs", "render", "skew.rs", "along_direction_row"),
                 ("preview_physics.rs", "append_combination_results", "preview_physics.rs", "along_combination_row"),
                 ("preview_combination_gates.rs", "form", "preview_combination_gates.rs", "along_combination_row"),
                 ("preview_physics_evidence.rs", "row_semantics", "along_direction_evidence.rs", "row_semantics"),
                 ("preview_physics_evidence.rs", "validate_uncertified_with", "along_direction_evidence.rs", "check_case"),
                 ("structural/formation_check.rs", "check_at", "structural/formation_check/skew.rs", "evaluate")]:
        assert edge in zero, edge
    calls = {(_name(e[0]), _name(e[1])): e[2] for e in d["EDGE_PER_CALL"] if "SKW-PC" in str(e[3])}
    assert calls == {("support_record", "group"): "2"}
    reduce = [r for r in d["CG_RULES"] if r.get("added") == "SKW-BN"]
    assert len(reduce) == 1 and reduce[0]["name"] == "reduce" and str(reduce[0]["exclude_targets"][0]).endswith(":reduce")


def test_the_invalid_input_arm_is_priced_at_its_static_reasons(monkeypatch):
    d = decisions(monkeypatch)
    reasons = re.findall(r'FrameKernelError::InvalidInput\("([^"]+)"\)', (SOURCE / FK / "structural/sparse.rs").read_text(encoding="utf-8"))
    assert len(reasons) == 2
    line = next(i + 1 for i, text in enumerate((SOURCE / FK / "lib.rs").read_text(encoding="utf-8").split("\n"))
                if 'Self::InvalidInput(detail) => write!(f, "invalid input: {detail}"),' in text)
    audits = {str(site): table for site, table in d["O10_ID_AUDIT"]}
    assert audits[f"{FK}lib.rs:{line}"]["detail"]["bytes"] == max(len(r) for r in reasons)


def test_a_reachable_along_combination_block_stops(tmp_path, monkeypatch):
    """The producer's along block is guarded by is_skewed_record; without it the SKW-D1 guard (or the file's pin) stops."""
    snap = copy_core(tmp_path)
    edit(snap, PP + "preview_physics.rs", "if crate::document_members::is_skewed_record(support)\n                    && targets",
         "if true\n                    && targets")
    with pytest.raises((SystemExit, ValueError), match="SKW|B7a|source changed|pin"):
        decisions(monkeypatch, snap)


def test_an_admitted_direction_stops(tmp_path, monkeypatch):
    """D1.6's `direction` refusal is SKW-D1's premise; removing it stops the run."""
    snap = copy_core(tmp_path)
    edit(snap, PP + "retained_memory.rs", "if support.direction.is_some() {\n            return refuse(C::Supports, F::SupportDirection);",
         "if false {\n            return refuse(C::Supports, F::SupportDirection);")
    with pytest.raises((SystemExit, ValueError), match="SKW|source changed|pin|profile"):
        decisions(monkeypatch, snap)


def test_the_readers_along_gate_stops_when_ungated(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    edit(snap, RE + "preview_physics_evidence.rs", "if crate::along_direction_evidence::applies(rows_list, model) {",
         "if true {")
    with pytest.raises((SystemExit, ValueError), match="SKW|source changed|pin"):
        decisions(monkeypatch, snap)
