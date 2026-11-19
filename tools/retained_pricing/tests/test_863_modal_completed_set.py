"""DEL-04-09 completed-set boundary (§8.63, MODAL-CS): the pricing rules for PR 390's fallible typed outcome storage.

No retained root reaches modal, so every new site is at multiplicity zero. The rules:
- MODAL-CS-RK re-keys the dense entry's three sites to run_modal_dense_within, which now holds the body, and stops
  unless run_modal_dense_with_budget is exactly its one-line delegation; the sites' TEXT rows are `format_args!(`.
- resource.rs is re-pinned whole; MODAL-CS-ID audits the boundary's unclassified arguments at source bounds.
- MODAL-CS-TM, -ST, -RR and -LI read source by exact reviewed inventories (rules/modal_cs_inventory.json): the test
  modules, with every `#[path]` checked by its target; every storage request and ShapeStorageError construction, with the structure
  forwarders body-pinned; every mention of the dense entry and of the caller limits; every mention of lift_into and
  modal_lift_within, whose body is pinned.
- MODAL-CS-UR asks o10_check.py to find lift_into in each point's graph and stop if a retained root reaches it.
- MODAL-CS-M0 guards every boundary row, the dense body's rows and the audited test rows at multiplicity 0 with their
  exact row counts; J3's pattern sizes the four test Debug sites of a whole ModalOutcome 0.
- Reviewed absences admit a source before the boundary; a rename that keeps the boundary stops.
These are source warrants, not runtime allocation observations.
"""
import hashlib
import json
import re
from pathlib import Path
import runpy
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
sys.path.insert(0, str(PRICING / "lib"))
from rule_ops import o10_check_rules  # noqa: E402  (the emitter sq2_rules.py uses for SQ2_RULES.json)

PP = "core/product_physics/src/"
MODAL = PP + "modal/"
RS, TRS, OUTCOME = MODAL + "resource.rs", MODAL + "tests_resource.rs", MODAL + "outcome.rs"
SKEW, MQ = MODAL + "skew_pencil.rs", "core/solver/modal_eigen/src/modal_quantities.rs"
RB = "core/solver/frame_kernel/src/restraint_basis.rs"
RS_PIN = "f9c7f62a6061a051042299569a65f2d94e00ae9f566f17074243d1d44f0e5622"
MQ_PIN = "b8d4cbdc37b56b9359c2b8a6aa593ea8829fcd94de597f6a7e00dfc7364070f3"
MAIN = "3c8fc26a6c188f1613714f56d9807dc32433c46d"   # main before the boundary (§8.63's comparison basis)
MAIN_RS_PIN = "f85562f6b73580d02e1c6736d472b74d98ad9fbbf25f0e024c8b0ad3a54c2c5c"


def decisions(monkeypatch, snapshot=SOURCE, route="L", tool=PRICING):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(tool / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def sha(snap, path):
    return hashlib.sha256((snap / path).read_bytes()).hexdigest()


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def tool_with(tmp_path, edits):
    """A scratch copy of the tool with exact text edits: {tool-relative file: [(old, new), ...]}, each old text once."""
    tool = tmp_path / "tool"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("tests", "__pycache__"))
    for rel, pairs in edits.items():
        text = (tool / rel).read_text(encoding="utf-8")
        for old, new in pairs:
            assert text.count(old) == 1, (rel, old)
            text = text.replace(old, new)
        (tool / rel).write_text(text, encoding="utf-8")
    return tool


def tool_with_pin(tmp_path, digest, current=RS_PIN):
    """A scratch copy of the tool whose resource.rs pin (or another pin, `current`) is `digest` (the README's step-5
    restoration), rules otherwise unchanged."""
    return tool_with(tmp_path, {"rules/o10_rules.py": [(current, digest)]})


U = 20
EXPR = 14 + 2 * U
BOUND = 62 + U
STRUCT = 35                # `cumulative effective-mass fractions`, the longest production structure constant
LIMIT = 87 + U
EXPECTED_ARGS = [
    (RS, {"expression": EXPR, "limit": U}), (RS, {"expression": EXPR, "bound": BOUND}), (RS, {"u64::MAX": U}),
    (RS, {"computed": U}), (RS, {"completed": U, "width": U, "usize::MAX": U}), (RS, {"element": U}),
    (RS, {"isize::MAX": U}), (RS, {"r": U}), (RS, {"r": U, "sizes.coordinates": U}), (RS, {"r": U, "sizes.global": U}),
    (RS, {"r": U, "product.rows_per_mode": U}), (RS, {"u64::MAX": U}), (RS, {"limit": U}),
    (RS, {"structure": STRUCT, "elements": U, "element_size": U, "isize::MAX": U}),
    (RS, {"bytes": U, "elements": U, "structure": STRUCT, "limit": LIMIT}),
    (TRS, {"actual-1": U}), (TRS, {"u64::MAX": U}), (TRS, {"usize::MAX": U}), (TRS, {"usize::MAX/4": U}),
    (TRS, {"structure": STRUCT}),
]


LONG = '"a structure name much longer than any reviewed constant"'


# ---- the checker, through the emitter's own O-10 inputs (rule_ops.o10_check_rules, as sq2_rules.py writes them)

def check(tmp_path, rules, runs):
    chain = tmp_path / "chain"
    chain.mkdir(exist_ok=True)
    (chain / "SQ2_RULES.json").write_text(json.dumps(rules))
    found = subprocess.run([sys.executable, str(PRICING / "lib/o10_check.py"), str(chain), *map(str, runs)], capture_output=True, text=True)
    return found.returncode, found.stdout + found.stderr


def point(tmp_path, name, edges=None, roots=("lib.rs:1:root",), rows=(), graph=True, audit=()):
    """A point as price.py leaves it: callgraph_g5's edges.json and audit.json, text_budget's rows and roots."""
    run = tmp_path / name
    (run / "work").mkdir(parents=True)
    (run / "looplog.json").write_text("[]")
    budget = {"rows": list(rows)}
    if roots is not None:
        budget["roots"] = [PP + r for r in roots]
    (run / "work/text_budget.caps.out.json").write_text(json.dumps(budget))
    if graph:
        (run / "edges.json").write_text(json.dumps({"edges": edges}))
        (run / "audit.json").write_text(json.dumps({"rows": list(audit)}))
    return run


@pytest.mark.parametrize("route", ["L", "E"])
def test_the_boundary_audits_sizes_and_guards_are_carried(monkeypatch, route):
    d = decisions(monkeypatch, route=route)
    found = [(str(k).rsplit(":", 1)[0], {a: v["bytes"] for a, v in value.items()}) for k, value in d["O10_ID_AUDIT"]
             if "MODAL-CS" in next(iter(value.values()))["source"]]
    assert found == EXPECTED_ARGS
    sizes = [(str(k), b) for k, b, why in d["O10_SITE_SIZE"] if "§8.63" in why]
    assert len(sizes) == 4 and all(k.startswith("modal/tests_resource.rs:") and b == 0 for k, b in sizes)
    guarded = [str(k) for k, why in d["O10_MULT0_SITES"] if "MODAL-CS-M0" in why]
    rows = dict(d["O10_MULT0_ROWS"])
    assert len(guarded) == 35 == len(set(guarded)) and set(rows) == set(guarded)
    assert sum(k.startswith("modal/resource.rs:") for k in guarded) == 25
    assert sum(k.startswith("modal/outcome.rs:") for k in guarded) == 5
    assert sum(rows.values()) == 37 and {k: n for k, n in rows.items() if n != 1} == {"modal/resource.rs:1402": 2, "modal/resource.rs:1422": 2}
    assert {str(k).split("/src/")[-1] for k, v in d["O10_ID_AUDIT"] if "MODAL-CS" in next(iter(v.values()))["source"]} <= set(guarded)
    assert not set(guarded) & {str(k).split("/src/")[-1] for k, _, _ in d["O10_SITE_SIZE"]}
    assert not set(guarded) & {str(k).split("/src/")[-1] for k, _ in d.get("O10_SITE_ZERO", [])}
    assert [(u["file"], u["name"], u["state"]) for u in d["O10_UNREACHED_FNS"]] == [(RB, "lift_into", "present")]


def test_the_dense_entry_sites_are_the_format_args_rows_of_the_body(monkeypatch):
    d = decisions(monkeypatch)
    keys = [str(k) for k, v in d["O10_ID_AUDIT"] if str(k).startswith(OUTCOME + ":")
            and set(v) & {"i+1", "orthogonality.pair.map(|(a,b)|(a+1,b+1))|?"}]
    assert len(keys) == 3
    text = (SOURCE / OUTCOME).read_text(encoding="utf-8").split("\n")
    start = next(i for i, l in enumerate(text) if l.startswith("pub(crate) fn run_modal_dense_within("))
    lines = [int(k.rsplit(":", 1)[1]) for k in keys]
    assert all(n - 1 > start for n in lines) and all("format_args!(" in text[n - 1] for n in lines)
    # MODAL-CS-RK's key-to-bound mapping: each re-keyed audit keeps its own argument and bound.
    bounds = {int(str(k).rsplit(":", 1)[1]): {a: v["bytes"] for a, v in value.items()} for k, value in d["O10_ID_AUDIT"]
              if str(k) in keys}
    assert bounds == {446: {"i+1": 20}, 501: {"orthogonality.pair.map(|(a,b)|(a+1,b+1))|?": 50}, 559: {"i+1": 20}}


SHADOW = ("    storage.modal_shape_reserve(&mut out, modes.len(), CUMULATIVE_FRACTIONS)?;",
          f"    const CUMULATIVE_FRACTIONS: &str = {LONG};\n    storage.modal_shape_reserve(&mut out, modes.len(), CUMULATIVE_FRACTIONS)?;")


ALIAS = ("    storage.modal_shape_reserve(&mut out, modes.len(), CUMULATIVE_FRACTIONS)?;",
         "    {\n        use self::ShapeStorageError::AllocationFailed as Refused;\n        if modes.is_empty() {\n"
         f"            return Err(Refused {{ structure: {LONG}, elements: 0, bytes: 0 }});\n        }}\n    }}\n"
         "    storage.modal_shape_reserve(&mut out, modes.len(), CUMULATIVE_FRACTIONS)?;")


@pytest.mark.parametrize("old, new", [SHADOW, ALIAS], ids=["shadowing-constant", "aliased-variant"])
def test_modal_quantities_is_pinned_whole(tmp_path, monkeypatch, old, new):
    """The second reader's two edits keep every inventoried text and change a production structure's length. The text
    scan does not resolve names, so it passes them; the whole-file pin stops them."""
    snap = copy_core(tmp_path / "src")
    edit(snap, MQ, old, new)
    with pytest.raises(SystemExit, match=r"MODAL-CS-ST: core/solver/modal_eigen/src/modal_quantities\.rs changed"):
        decisions(monkeypatch, snap)


def test_comments_and_literals_do_not_trip_the_censuses(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    (snap / MODAL / "new_source.rs").write_text(
        "// lift_into, modal_lift_within, run_modal_dense_within, ModalResponseRowProduct { .. }, #[path = \"x.rs\"]\n"
        "const NOTE: &str = \"self.lift_into(v, out); ModalCompletedSetLimits { } modal_shape_reserve(&mut v, n, x)\";\n",
        encoding="utf-8")
    decisions(monkeypatch, snap)


def test_an_edit_to_a_test_helper_brought_in_by_path_passes(tmp_path, monkeypatch):
    """A test-file entry's target (a tests/ helper) does not bind its bytes: ordinary test work there does not stop."""
    snap = copy_core(tmp_path)
    target = snap / "core/solver/frame_kernel/tests/retained_k4/support.rs"
    target.write_text(target.read_text(encoding="utf-8") + "\npub(crate) fn helper_added_by_a_test() -> u8 {\n    1\n}\n",
                      encoding="utf-8")
    decisions(monkeypatch, snap)


def test_an_unrelated_path_module_in_the_files_read_passes(tmp_path, monkeypatch):
    """Main e0e3e3c7's form: a #[path] module of the frame kernel naming a file in the files read, nothing of the boundary."""
    snap = copy_core(tmp_path)
    (snap / "core/solver/frame_kernel/src/structural/inverse_note.rs").write_text("pub(crate) fn note() -> u8 {\n    1\n}\n",
                                                                                 encoding="utf-8")
    target = snap / "core/solver/frame_kernel/src/structural/arc_certificate.rs"
    target.write_text(target.read_text(encoding="utf-8") + '\n#[allow(dead_code)]\n#[path = "inverse_note.rs"]\nmod inverse_note;\n',
                      encoding="utf-8")
    decisions(monkeypatch, snap)
