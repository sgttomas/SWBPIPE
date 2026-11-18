"""Reference-system generated-load references (`validation/references/reference_system_generated_loads/`).

The generator rebuilds `references.json` and `SHA256SUMS` in memory and compares them byte for byte with the
committed files. Here: that check passes; the manifest verifies and names exactly the package's files; the
generator imports only the standard library, makes no dynamic import or evaluation, and names no product path but
the model file it reads; its output does not depend on the model's load cases, combinations, analysis status or
diagnostics; the governing text and the sibling values it transcribes match their sources as they stand; and the
recorded comparison rule, floors, EQ-6 expectations, component bits and warning outcomes are consistent.
"""
import ast
import hashlib
import json
import math
import shutil
import struct
import subprocess
import sys
from pathlib import Path

import reference_packages as R

PROJECT = Path(__file__).resolve().parents[1]
NAME = "reference_system_generated_loads"
REFERENCE = PROJECT / "validation" / "references" / NAME
GENERATOR = REFERENCE / "generators" / "rsgl_reference.py"
MODEL_REL = "fixtures/reference_system/reference_system.json"
SPECIFICATION = PROJECT / "fixtures" / "reference_system" / "SPECIFICATION.md"
RIGID_SCOPE = PROJECT / "execution" / "PKG-03" / "DEL-03-05" / "ScopeOfWork.md"

PRODUCT_ROOTS = {"core", "apps", "tests", "tools", "fixtures", "schemas"}
ALLOWED_LITERALS = {MODEL_REL}
FORBIDDEN_CALLS = {"__import__", "import_module", "exec", "eval", "spec_from_file_location"}
DROPPED = ("load_cases", "combinations", "analysis_status", "diagnostics")
SEISMIC = ("RS-EQX", "RS-EQY", "RS-EQZ")


def _refs():
    return json.loads((REFERENCE / "references.json").read_text(encoding="utf-8"))


def _tree():
    return ast.parse(GENERATOR.read_text(encoding="utf-8"))


def _literal(name):
    """The value of a module-level assignment in the generator, read without running it."""
    for node in _tree().body:
        if isinstance(node, ast.Assign) and any(getattr(t, "id", None) == name for t in node.targets):
            return ast.literal_eval(node.value)
    raise KeyError(name)


def _flat(text):
    return " ".join(text.split())


def test_generator_check_rebuilds_the_committed_references(tmp_path):
    problems = R.check_problems(NAME, 0, tmp_path / "check_output")
    assert not problems, "\n".join(problems)


def test_sha256sums_verify_and_name_the_package():
    listed = {}
    for line in (REFERENCE / "SHA256SUMS").read_text().splitlines():
        digest, path = line.split("  ", 1)
        listed[path] = digest
    assert set(listed) == {"README.md", "generators/rsgl_reference.py", "references.json"}
    present = {p.relative_to(REFERENCE).as_posix() for p in REFERENCE.rglob("*")
               if p.is_file() and "__pycache__" not in p.parts}
    assert present == set(listed) | {"SHA256SUMS"}, sorted(present)
    for path, digest in listed.items():
        assert hashlib.sha256((REFERENCE / path).read_bytes()).hexdigest() == digest, path


def test_generator_imports_only_the_standard_library():
    modules = set()
    for node in ast.walk(_tree()):
        if isinstance(node, ast.Import):
            modules |= {alias.name.split(".")[0] for alias in node.names}
        elif isinstance(node, ast.ImportFrom):
            assert node.level == 0, "relative import"
            modules.add(node.module.split(".")[0])
    assert modules and modules <= set(sys.stdlib_module_names), sorted(modules - set(sys.stdlib_module_names))


def test_generator_makes_no_dynamic_import_or_evaluation():
    called = set()
    for node in ast.walk(_tree()):
        if isinstance(node, ast.Call):
            func = node.func
            name = func.id if isinstance(func, ast.Name) else func.attr if isinstance(func, ast.Attribute) else None
            if name in FORBIDDEN_CALLS:
                called.add(name)
    assert not called, sorted(called)


def test_generator_names_no_product_path_but_the_model_file():
    def names_product_path(literal):
        if not literal or any(c.isspace() for c in literal):
            return False
        return any(seg in PRODUCT_ROOTS for seg in literal.replace("\\", "/").split("/"))
    found = sorted({node.value for node in ast.walk(_tree())
                    if isinstance(node, ast.Constant) and isinstance(node.value, str)
                    and names_product_path(node.value) and node.value not in ALLOWED_LITERALS})
    assert not found, found


def test_output_does_not_depend_on_the_dropped_model_members(tmp_path):
    """The package's check, run against a copy of the model whose dropped members are replaced by a marker, must
    still reproduce the committed files: nothing derived reads those members."""
    root = tmp_path / "root"
    package = root / "validation" / "references" / NAME
    shutil.copytree(REFERENCE, package)
    model = json.loads((PROJECT / MODEL_REL).read_text(encoding="utf-8"))
    for key in DROPPED:
        assert key in model, key
        model[key] = {"replaced": "by the dropped-member test"}
    target = root / MODEL_REL
    target.parent.mkdir(parents=True)
    target.write_text(json.dumps(model), encoding="utf-8")
    run = subprocess.run([sys.executable, "-I", "-B", "generators/rsgl_reference.py"], cwd=package,
                         capture_output=True, text=True, timeout=R.CHECK_TIMEOUT_S)
    assert run.returncode == 0, run.stdout + run.stderr


def test_transcribed_governing_text_is_current():
    spec = _flat(SPECIFICATION.read_text(encoding="utf-8"))
    for key, quoted in _literal("SPEC").items():
        statement = quoted.split(": ", 1)[1]
        assert _flat(statement) in spec, key
    assert "A component of kind valve, flange, reducer, rigid or specialty at a node." in \
        RIGID_SCOPE.read_text(encoding="utf-8")
    assert _literal("RIGID_KINDS") == ("valve", "flange", "reducer", "rigid", "specialty")


def test_transcribed_sibling_values_match_their_files():
    sib = _literal("SIBLING")

    def load(entry):
        return json.loads((PROJECT / entry["file"]).read_text(encoding="utf-8"))

    r1 = load(sib["r1_ce"])["key_values"]
    assert sib["r1_ce"]["W_total_N"] == r1["W_total_N"]["exact"]
    for key in ("weight_first_moment_N_m", "weight_moment_about_origin_N_m"):
        assert sib["r1_ce"][key] == [v["exact"] for v in r1[key]], key
    assert sib["tranche1"]["W_total_N"] == load(sib["tranche1"])["key_values"]["W_total_N"]["exact"]
    full = load(sib["pending_full"])["key_values"]["full"]
    for case, (force, moment) in ((k, v) for k, v in sib["pending_full"].items() if k != "file"):
        assert force == [v["exact"] for v in full[case]["generated_force_N"]], case
        assert moment == [v["exact"] for v in full[case]["generated_moment_about_origin_N_m"]], case
    ctl = sib["pending_control_before_2026_11_12"]
    control = next(c for c in load(ctl)["seismic_controls"] if c["control"] == ctl["control"])
    for case in SEISMIC:
        force, moment = ctl[case]
        values = control["values"]["full"][case]
        assert force == [v["exact"] for v in values["generated_force_N"]], case
        assert moment == [v["exact"] for v in values["generated_moment_about_origin_N_m"]], case


def test_every_cross_check_agrees():
    refs = _refs()
    assert refs["crosscheck_count"] == len(refs["crosscheck"]) == 24
    assert all(c["agrees"] for c in refs["crosscheck"]), [c["check"] for c in refs["crosscheck"] if not c["agrees"]]


def test_recorded_comparison_rule():
    rule = _refs()["comparison_rule"]
    assert rule["relative"] == 1e-9
    assert rule["nonzero"] == "|observed − expected| ≤ 1e-9·|expected| (relative alone)"
    assert rule["exact_zero"].startswith("|observed| ≤ 1e-9·zero_scale, zero_scale the largest magnitude of its family")
    assert rule["discrete"] == "compared exactly"


def test_floors_and_eq6_follow_from_the_resultants():
    for case, rec in _refs()["cases"].items():
        res = rec["resultant"]
        for fam, key in (("force_N", "force_N"), ("moment_N_m", "moment_about_origin_N_m")):
            values = [v["value"] for v in res[key]]
            floor = rec["floors"][fam]
            assert floor["zero_scale"] == max(abs(v) for v in values) > 0, (case, fam)
            assert floor["floor"] == 1e-9 * floor["zero_scale"], (case, fam)
            assert floor["exact_zero_components"] == [a for a, v in zip("xyz", res[key]) if v["exact"] == "0"]
        eq6 = rec["EQ-6"]
        assert eq6["term"] == "RS-d" + case[3:]
        for got, want in ((eq6["expected_reaction_force_resultant_N"], res["force_N"]),
                          (eq6["expected_reaction_moment_resultant_about_origin_N_m"],
                           res["moment_about_origin_N_m"])):
            assert [g["value"] for g in got] == [-w["value"] if w["value"] else 0.0 for w in want], case


def test_generated_load_inventory():
    cases = _refs()["cases"]
    assert cases["RS-WIN"]["generated_load_counts"] == {"distributed_on_member": 14}
    nonzero_moments = {"RS-EQX": 3, "RS-EQY": 5, "RS-EQZ": 4}
    for case in SEISMIC:
        counts = cases[case]["generated_load_counts"]
        assert counts["distributed_on_member"] == 21 and counts["component_force"] == 4, case
        assert counts["component_moment required nonzero"] == nonzero_moments[case], case
        assert counts["component_moment required nonzero"] + counts["component_moment zero"] == 8, case
        targets = {ld["target"] for ld in cases[case]["generated_loads"] if ld["kind"] == "distributed_on_member"}
        assert not targets & {"pipe:RS-P020", "pipe:RS-P030", "pipe:RS-P040", "pipe:RS-P150", "pipe:RS-P180"}


def test_component_bits_match_values_and_the_required_nonzero_set():
    for case in SEISMIC:
        for ld in _refs()["cases"][case]["generated_loads"]:
            if ld["kind"] not in ("component_force", "component_moment"):
                continue
            b = ld["binary64_by_formation"]
            assert struct.unpack(">d", bytes.fromhex(b["bits"]))[0] == b["value"], ld
            assert math.isfinite(b["value"]) and (b["value"] != 0) == ld["required_nonzero"], ld
            exact = ld["value_N"] if ld["kind"] == "component_force" else ld["value_N_m"]
            if b["value"] != 0:
                assert abs(b["value"] - exact["value"]) <= 1e-15 * abs(exact["value"]), ld


def test_the_pipes_only_twin_discriminates_every_nonzero_component():
    for case in SEISMIC:
        for d in _refs()["cases"][case]["pipes_only_twin"]["discrimination_at_the_comparison_rule"]:
            assert d["discriminates"] == (not d["expected_exact_zero"]), (case, d)
            if not d["expected_exact_zero"]:
                assert d["difference_over_tolerance"] > 1e6, (case, d)


def test_seismic_model_check_outcomes():
    refs = _refs()
    decision = refs["seismic_warning_decision"]
    assert decision["decided_by"] == "SWBPIPE's Agent 0" and decision["date"] == "2026-11-18"
    assert refs["cases"]["RS-WIN"]["seismic_model_check"]["expected_findings"] == "none"
    for case in SEISMIC:
        rec = refs["cases"][case]["seismic_model_check"]
        assert rec["weight_finding_required"] is False
        assert rec["gravity_comparison"]["gravity_finding_required"] is False
        assert rec["gravity_comparison"]["equal_after_normalization"] is True
        assert rec["any_seismic_model_check_finding_required"] is False
        assert rec["added_weight_named"] == []
        assert len(rec["gravity_comparison"]["baseline_loads"]) == 4
        loads = rec["weight_category_loads_of_the_case"]
        assert loads["count"] == len(loads["ids"]) == len(set(loads["ids"])) == 29
        assert all(i.startswith("load:self-weight:%d:load:%s:" % (len("load:" + case), case)) for i in loads["ids"])
        for key in ("code", "message", "findings_per_load_or_per_case", "order_of_named_loads"):
            assert rec[key] is None, key
    assert all(item["frozen"] is None for item in refs["open_items"])
