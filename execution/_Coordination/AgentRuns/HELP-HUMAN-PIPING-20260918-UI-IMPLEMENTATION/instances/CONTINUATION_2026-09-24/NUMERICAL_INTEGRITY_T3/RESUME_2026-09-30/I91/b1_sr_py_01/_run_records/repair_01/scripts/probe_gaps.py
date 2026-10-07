"""Probes of pre-existing Python-only admissions next to SR-PY's scope (PY base or head tree)."""
import json, sys
from copy import deepcopy
from pathlib import Path
P = Path(sys.argv[1]); sys.path.insert(0, str(P)); sys.path.insert(0, str(Path(__file__).parent))
from core.analysis_runs import retained_precision as rp
import census_lib as lib
data = json.loads((P / "fixtures/results/retained_precision_cases.json").read_text())
bases = {f["id"]: f for f in data["cases"]}
must = next(e for e in data["must_pass"] if e["id"] == "not_required_second_case_checks_passed")
B = ["retained_precision", "body"]
def verdict(base, edits, inv_edits=()):
    src, inv = lib.apply_entry(bases[base], {"edits": edits, "invocation_edits": list(inv_edits), "rehash": "all"})
    try: r = rp.validate_retained_precision(src, inv); return f"admitted standing={r['standing']}"
    except rp.RetainedPrecisionError as e: return f"{e.gate} {e.code}"
s = lambda path, v: {"path": path, "op": "set", "value": v}
pre = deepcopy(must["edits"])
mb = bases[must["base"]]["source"]["retained_precision"]["body"]["material_bases"]
print("07j not_required statement:", verdict(must["base"], pre))
print("material_bases:", [(m.get("index"), m["case_indices"]) for m in mb])
print("(b) not_required ordinary material_basis_ref = 7:", verdict(must["base"], pre + [s(B + ["ordinary_attempts", 1, "material_basis_ref"], 7)]))
print("(c) material_bases[0].case_indices = [0] (omits the not_required case):", verdict(must["base"], pre + [s(B + ["material_bases", 0, "case_indices"], [0])]))
print("(d1) invocation extra member:", verdict("ordinary_prepared_synthetic", [], [s(["extra"], 1)]))
print("(d2) invocation solver_mode 'foo':", verdict("ordinary_prepared_synthetic", [], [s(["solver_mode"], "foo")]))
print("(f) material_bases[0].index = 5:", verdict(must["base"], pre + [s(B + ["material_bases", 0, "index"], 5)]))
print("(g) model.reference_configurations present:", verdict("ordinary_prepared_synthetic", [], [s(["request", "model", "reference_configurations"], [])]))
