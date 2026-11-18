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
MAIN = "3c8fc26a6c188f1613714f56d9807dc32433c46d"   # main before the boundary (§8.63's comparison basis)
MAIN_RS_PIN = "f85562f6b73580d02e1c6736d472b74d98ad9fbbf25f0e024c8b0ad3a54c2c5c"


def decisions(monkeypatch, snapshot=SOURCE, route="L", tool=PRICING):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(tool / f"rules/sq2_decisions_{route}.py"))


def copy_core(tmp_path):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    return tmp_path


def edit(snap, path, old, new):
    target = snap / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1, (path, old)
    target.write_text(text.replace(old, new), encoding="utf-8")


def tool_with_pin(tmp_path, digest, current=RS_PIN):
    """A scratch copy of the tool whose resource.rs pin is `digest` (the README's step-5 restoration), rules unchanged."""
    tool = tmp_path / "tool"
    shutil.copytree(PRICING, tool, ignore=shutil.ignore_patterns("tests", "__pycache__"))
    rules = tool / "rules/o10_rules.py"
    text = rules.read_text(encoding="utf-8")
    assert text.count(current) == 1
    rules.write_text(text.replace(current, digest), encoding="utf-8")
    return tool


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


LONG = '"a structure name much longer than any reviewed constant"'


@pytest.mark.parametrize("path, old, new, message", [
    # MODAL-CS-RK: run_modal_dense_with_budget no longer only delegates, or delegates with limits.
    (OUTCOME, "    run_modal_dense_within(req, params, budget, None)\n",
     "    let _ = format!(\"{}\", 1);\n    run_modal_dense_within(req, params, budget, None)\n",
     "run_modal_dense_with_budget is not run_modal_dense_within's delegation"),
    (OUTCOME, "    run_modal_dense_within(req, params, budget, None)\n", "    run_modal_dense_within(req, params, budget, limits())\n",
     "MODAL-CS-RK|MODAL-CS-RR"),
    # The whole-file pin.
    (RS, "over the caller's limit of {limit}; the whole set", "over the caller's stated limit of {limit}; the whole set",
     "modal/resource.rs changed"),
    # MODAL-CS-ST: a structure that is not a named constant; the reader's two bypasses; a forwarder's body.
    (OUTCOME, "storage.modal_completed_set_rows(total, MODAL_SET_COMPUTED_EIGENVALUES)?;",
     "storage.modal_completed_set_rows(total, req.name())?;", "MODAL-CS-ST: the reviewed inventory changed"),
    (SKEW, "        storage.modal_shape_reserve(&mut v, dimension, SKEW_LIFT_SCATTER)?;",
     f"        let (structure,) = ({LONG},);\n        storage.modal_shape_reserve(&mut v, dimension, structure)?;",
     "MODAL-CS-ST: the reviewed inventory changed"),
    (MQ, "let overflow = ShapeStorageError::CapacityOverflow { structure, elements: len, element_size };",
     f"let overflow = ShapeStorageError::CapacityOverflow {{ structure: {{ {LONG} }}, elements: len, element_size }};",
     "MODAL-CS-ST: the reviewed inventory changed"),
    (MQ, "    let bytes = requested_bytes::<T>(vec.len(), len, structure)?;\n    vec.try_reserve_exact",
     "    let bytes = requested_bytes::<T>(vec.len(), len, structure)?;\n    let _held = vec.len();\n    vec.try_reserve_exact",
     r"MODAL-CS-ST: core/solver/modal_eigen/src/modal_quantities.rs:reserve_exactly#1 changed"),
    # MODAL-CS-M0: an added format in the unpinned dense body.
    (OUTCOME, "    let total = solution.eigenvalues.len();\n",
     "    let total = solution.eigenvalues.len();\n    let _probe = format!(\"{total}\");\n", "the dense body's TEXT rows changed"),
    # MODAL-CS-TM: the modal document tests are no longer a test-only module.
    ("core/product_physics/src/modal.rs", "#[cfg(test)]\nmod tests_document;", "mod tests_document;",
     "MODAL-CS-TM: core/product_physics/src/modal.rs's `mod tests_document`"),
    # MODAL-CS-LI: a retained-side (frame kernel) call of lift_into, and an edit of modal_lift_within's body.
    (RB, "    /// Adds (Q v)_dof exactly to `acc` (for certificates and the exact",
     "    pub fn lift_twice(&self, v: &[f64], out: &mut [f64]) -> Result<(), BasisError> {\n"
     "        self.lift_into(v, out)\n    }\n\n    /// Adds (Q v)_dof exactly to `acc` (for certificates and the exact",
     "MODAL-CS-LI: the reviewed inventory changed"),
    (SKEW, "        v.resize(dimension, 0.0);\n", "        v.resize(dimension, 0.0);\n        let _k = 0;\n",
     "MODAL-CS-LI: core/product_physics/src/modal/skew_pencil.rs:modal_lift_within#1 changed"),
])
def test_a_changed_boundary_source_stops(tmp_path, monkeypatch, path, old, new, message):
    snap = copy_core(tmp_path)
    edit(snap, path, old, new)
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap)


@pytest.mark.parametrize("files, message", [
    # MODAL-CS-ST: an added storage request.
    ({MODAL + "new_source.rs": "fn extra(s: &mut impl ShapeStorage) {\n    let mut v: Vec<u8> = Vec::new();\n"
      "    let _ = s.modal_shape_reserve(&mut v, 1, MODAL_SET_MODES);\n}\n"}, "MODAL-CS-ST: the reviewed inventory changed"),
    # MODAL-CS-RR: the reader's aliased-constructor caller, and a second production call passing Some.
    ({MODAL + "new_source.rs": "use super::resource::ModalCompletedSetLimits as Limits;\nfn limits() -> Limits {\n"
      "    Limits { computed_modes: 1, completed_modes: 1, normalized_entries: 1, published_entries: 1, response_rows: vec![] }\n}\n"},
     "MODAL-CS-RR: the reviewed inventory changed"),
    ({MODAL + "new_source.rs": "fn again(r: &ModalPreviewRequest, p: &ModalIntegrityParameters, l: &L) {\n"
      "    let _ = super::outcome::run_modal_dense_within(r, p, None, Some(l));\n}\n"}, "MODAL-CS-RR: the reviewed inventory changed"),
    # MODAL-CS-LI: an added caller of modal_lift_within; a lift_into mention in the driver's added correct_norm crate.
    ({MODAL + "new_source.rs": "fn lift(p: &SkewPencil, s: &mut impl ShapeStorage) {\n    let _ = p.modal_lift_within(&[], &[], s);\n}\n"},
     "MODAL-CS-LI: the reviewed inventory changed"),
    ({"core/numerics/correct_norm/src/new_source.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n"
      "    let _ = b.lift_into(v, o);\n}\n"}, "MODAL-CS-LI: the reviewed inventory changed"),
    # The reader's two-declaration module: one test declaration and one #[path] declaration of the same file. The
    # census reads the file wherever it is declared, so its mention stops.
    ({"core/solver/frame_kernel/src/shared.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n    let _ = b.lift_into(v, o);\n}\n",
      "core/solver/frame_kernel/src/lib.rs": "\n#[cfg(test)]\nmod shared;\n#[path = \"shared.rs\"]\nmod production_shared;\n"},
     "MODAL-CS-LI: the reviewed inventory changed"),
    # MODAL-CS-TM: a reviewed test file compiled as production through #[path], from its own crate and from another
    # crate by a relative route; a module path set through cfg_attr; a target outside the files read.
    ({MODAL + "new_source.rs": '\n#[path = "tests_resource.rs"]\nmod production_resource;\n'}, "MODAL-CS-TM: .* names a reviewed test file"),
    ({"core/solver/frame_kernel/src/lib.rs": '\n#[path = "../../../product_physics/src/modal/tests_document.rs"]\nmod copied;\n'},
     "MODAL-CS-TM: .* names a reviewed test file"),
    ({MODAL + "new_source.rs": '\n#[cfg_attr(test, path = "other.rs")]\nmod switched;\n'}, "MODAL-CS-TM: .* sets a module path through cfg_attr"),
    ({"core/solver/frame_kernel/src/lib.rs": '\n#[path = "../../../../tools/elsewhere.rs"]\nmod elsewhere;\n'},
     "MODAL-CS-TM: the reviewed inventory changed"),
    # A reviewed outside target (a tests/ helper a #[path] brings in) is read by the censuses: a call added there stops.
    ({"core/solver/frame_kernel/tests/retained_k4/support.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n"
      "    let _ = b.lift_into(v, o);\n}\n"}, "MODAL-CS-LI: the reviewed inventory changed"),
    # ...and a test file named there through a nested #[path] stops as from the walked crates.
    ({"core/solver/frame_kernel/tests/retained_k4/support.rs": '\n#[path = "../../../../product_physics/src/modal/tests_resource.rs"]\n'
      "mod copied;\n"}, "MODAL-CS-TM: .* names a reviewed test file"),
    # The same file under its test declaration alone: no exclusion, so the census still sees its mention.
    ({"core/solver/frame_kernel/src/shared.rs": "fn twice(b: &RestraintBasis, v: &[f64], o: &mut [f64]) {\n    let _ = b.lift_into(v, o);\n}\n",
      "core/solver/frame_kernel/src/lib.rs": "\n#[cfg(test)]\nmod shared;\n"}, "MODAL-CS-LI: the reviewed inventory changed"),
])
def test_a_new_source_stops(tmp_path, monkeypatch, files, message):
    snap = copy_core(tmp_path)
    for path, text in files.items():
        target = snap / path
        target.write_text((target.read_text(encoding="utf-8") if target.exists() else "") + text, encoding="utf-8")
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap)


def test_comments_and_literals_do_not_trip_the_censuses(tmp_path, monkeypatch):
    snap = copy_core(tmp_path)
    (snap / MODAL / "new_source.rs").write_text(
        "// lift_into, modal_lift_within, run_modal_dense_within, ModalResponseRowProduct { .. }, #[path = \"x.rs\"]\n"
        "const NOTE: &str = \"self.lift_into(v, out); ModalCompletedSetLimits { } modal_shape_reserve(&mut v, n, x)\";\n",
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


@pytest.mark.parametrize("old, new, message", [
    ('format!("n_c = {computed}")', 'format!("n_c := {computed}")', "the boundary's format `n_c = {computed}` is gone"),
    ('None => "no completed-set limit was supplied".to_string(),', 'None => "no completed-set limit supplied".to_string(),',
     "modal_completed_set_refusal's limit text changed"),
    ('let normalized_entries = entries(ModalCompletedSetDimension::NormalizedEntries, "n", coordinates)?;',
     'let normalized_entries = entries(ModalCompletedSetDimension::NormalizedEntries, "nn", coordinates)?;',
     "modal_completed_set_sizes' symbols changed"),
])
def test_the_id_audits_own_source_assertions_stop(tmp_path, monkeypatch, old, new, message):
    """Past the whole-file pin (the copy's pin is the mutated file's hash), each MODAL-CS-ID assertion stops."""
    snap = copy_core(tmp_path / "src")
    edit(snap, RS, old, new)
    tool = tool_with_pin(tmp_path, hashlib.sha256((snap / RS).read_bytes()).hexdigest())
    with pytest.raises(SystemExit, match=message):
        decisions(monkeypatch, snap, tool=tool)


@pytest.mark.parametrize("path, old, new, name", [
    (OUTCOME, "run_modal_dense_within", "run_modal_dense_bounded", "outcome.rs:run_modal_dense_within"),
    (RS, "modal_completed_set_judge", "modal_completed_set_admit", "resource.rs:modal_completed_set_judge"),
    (RB, "pub fn lift_into(", "pub fn lift_into_exact(", "restraint_basis.rs:lift_into"),
])
def test_a_rename_that_keeps_the_boundary_stops_at_its_reviewed_absence(tmp_path, monkeypatch, path, old, new, name):
    snap = copy_core(tmp_path)
    target = snap / path
    target.write_text(target.read_text(encoding="utf-8").replace(old, new), encoding="utf-8")
    with pytest.raises(SystemExit, match=r"O10-GATE \(MODAL-CS, §8\.63\): .*" + name + " is absent"):
        decisions(monkeypatch, snap)


def main_source(tmp_path):
    """Main before the boundary (MAIN), archived from this repository's history; skipped where history lacks it."""
    if subprocess.run(["git", "-C", str(SOURCE), "cat-file", "-e", MAIN + "^{commit}"], capture_output=True).returncode:
        pytest.skip(f"{MAIN} is not in this clone's history")
    snap = tmp_path / "main"
    snap.mkdir()
    archive = subprocess.run(["git", "-C", str(SOURCE), "archive", MAIN, "core"], capture_output=True, check=True).stdout
    subprocess.run(["tar", "-x", "-C", str(snap)], input=archive, check=True)
    return snap


def test_the_historical_absence_is_admitted_only_for_its_source_state(tmp_path, monkeypatch):
    snap = main_source(tmp_path)
    tool = tool_with_pin(tmp_path, MAIN_RS_PIN)
    d = decisions(monkeypatch, snap, tool=tool)
    assert [(u["file"], u["name"], u["state"]) for u in d["O10_UNREACHED_FNS"]] == [(RB, "lift_into", "reviewed-absent")]
    # The same source with the function's text present (here only in a comment) is not that state.
    (snap / RB).write_text((snap / RB).read_text(encoding="utf-8") + "// lift_into\n", encoding="utf-8")
    with pytest.raises(SystemExit, match=r"O10-GATE \(MODAL-CS, §8\.63\): .*restraint_basis\.rs:lift_into is absent"):
        decisions(monkeypatch, snap, tool=tool)


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
    (run / "work/text_budget.caps.out.json").write_text(json.dumps({"rows": list(rows), "roots": [PP + r for r in roots]}))
    if graph:
        (run / "edges.json").write_text(json.dumps({"edges": edges}))
        (run / "audit.json").write_text(json.dumps({"rows": list(audit)}))
    return run


# The node form: callgraph_g5 keys a function `file:line:name`. A real call `x.lift_into(..)`, resolved by name or by the
# receiver's known type, is an edge from the caller's node to lift_into's node; the governing run's graph holds exactly
# one such edge, core/product_physics/src/modal/skew_pencil.rs:51:modal_lift_within -> restraint_basis.rs:1265:lift_into.
ROOT, FN = PP + "lib.rs:1:root", RB + ":1265:lift_into"
OTHER = "core/solver/frame_kernel/src/structural.rs:10:solve"
UNRESOLVED = {"caller": OTHER, "line": "core/solver/frame_kernel/src/structural.rs:12", "name": "lift_into",
              "reason": "receiver typed Rigidity: no method of that name on it or a scanned trait", "context": "r.lift_into("}


@pytest.mark.parametrize("edges, graph, state, stops", [
    ({ROOT: [OTHER], OTHER: [], FN: []}, True, "present", None),                                   # unreached: passes
    # A retained caller's edge into it, real or a false same-name edge: either stops.
    ({ROOT: [OTHER], OTHER: [FN], FN: []}, True, "present", "reached from a retained root"),
    # A call naming it that the graph left unresolved (its audit row) stops even though no edge reaches it.
    ({ROOT: [OTHER], OTHER: [], FN: [], "audit": [UNRESOLVED]}, True, "present", "a call naming it produced no edge"),
    ({ROOT: [OTHER], OTHER: []}, True, "present", "missing from the graph"),
    ({ROOT: [OTHER], OTHER: [], FN: [], RB + ":1300:lift_into": []}, True, "present", "ambiguous in the graph"),
    ({ROOT: []}, True, "present", "the function's crate is not in the graph"),
    (None, False, "present", "the point's graph or roots are missing"),                             # a point not evaluated
    ({ROOT: [OTHER], OTHER: []}, True, "reviewed-absent", None),                                    # the historical state
    ({ROOT: [OTHER], OTHER: [], FN: []}, True, "reviewed-absent", "reviewed absent, but the graph has it"),
])
def test_the_unreached_function_check(tmp_path, edges, graph, state, stops):
    rules = o10_check_rules({"O10_UNREACHED_FNS": [{"file": RB, "name": "lift_into", "state": state, "why": "test"}]})
    good = point(tmp_path, "L_30", {ROOT: [OTHER], OTHER: [], **({FN: []} if state == "present" else {})})
    edges = dict(edges or {}) if graph else None
    audit = edges.pop("audit", ()) if graph else ()
    code, out = check(tmp_path, rules, [good, point(tmp_path, "L_21", edges, graph=graph, audit=audit)])
    assert code == (3 if stops else 0), out
    if stops:
        assert "O10 CHECK FAILED" in out and stops in out


def test_the_unreached_function_check_on_the_candidates_rules(tmp_path, monkeypatch):
    """The candidate's own emitted entry finds lift_into by file and name and stops on a retained caller's path."""
    rules = o10_check_rules(decisions(monkeypatch))
    spec = rules["o10_unreached_fns"]
    assert [(s["file"], s["name"], s["state"]) for s in spec] == [(RB, "lift_into", "present")]
    rules = {"o10_unreached_fns": spec}
    mlw = SKEW + ":51:modal_lift_within"
    today = {ROOT: [OTHER], OTHER: [], mlw: [FN], FN: []}           # the governing run's form: modal unreached
    assert check(tmp_path, rules, [point(tmp_path, "E_30", today)])[0] == 0
    # A retained function gaining the real call: its node's edge to lift_into's node, as the graph resolves it.
    code, out = check(tmp_path, rules, [point(tmp_path, "L_30", {**today, OTHER: [FN]})])
    assert code == 3 and "reached from a retained root" in out
    # Reaching modal_lift_within, whose real call reaches lift_into, stops the same way.
    code, out = check(tmp_path, rules, [point(tmp_path, "L_12", {**today, OTHER: [mlw]})])
    assert code == 3 and "reached from a retained root" in out


def rows_for(keys, counts):
    return [{"file": PP + k.rsplit(":", 1)[0], "line": int(k.rsplit(":", 1)[1]), "mult": 0}
            for k in keys for _ in range(counts.get(k, 1))]


@pytest.mark.parametrize("change", ["exact", "shared-line loss", "size-zero positive", "size-zero missing", "boundary positive"])
def test_o10_rejects_a_changed_guarded_row_through_the_emitted_rules(tmp_path, monkeypatch, change):
    rules = o10_check_rules(decisions(monkeypatch))
    keys, counts = rules["o10_mult0_sites"], rules["o10_mult0_rows"]
    assert counts["modal/resource.rs:1402"] == 2
    size0 = next(str(k) for k in keys if str(k) == "modal/tests_resource.rs:907")
    rows = rows_for(keys, counts)
    if change == "shared-line loss":
        rows.remove(next(r for r in rows if r["file"].endswith("modal/resource.rs") and r["line"] == 1402))
    elif change == "size-zero positive":
        next(r for r in rows if r["file"].endswith(size0.split(":")[0]) and r["line"] == 907)["mult"] = 1
    elif change == "size-zero missing":
        rows = [r for r in rows if not (r["file"].endswith(size0.split(":")[0]) and r["line"] == 907)]
    elif change == "boundary positive":
        next(r for r in rows if r["file"].endswith("modal/outcome.rs") and r["line"] == 446)["mult"] = 1
    code, out = check(tmp_path, {k: v for k, v in rules.items() if k != "o10_unreached_fns"}, [point(tmp_path, "L_30", {}, rows=rows)])
    assert code == (0 if change == "exact" else 3), out
