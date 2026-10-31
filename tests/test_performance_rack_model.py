#!/usr/bin/env python3
"""The RT-U4 reference racks (DEL-04-05, release target criterion 9): core/solver/performance_harness/rack/rack_model.py.

Generation checks (stdlib only): determinism, exact node counts, the bay layout's closed-form counts, invented
identities and provenance, the model document version, the pending-family ledger and the recorded hashes. Solve checks:
each profile at a small size through the product's headless runner (OPENPIPESTRESS_RUNNER_BIN, which the session
builds, or a preset; skipped when absent under OPENPIPESTRESS_NO_CARGO_BUILD=1).
"""
from __future__ import annotations

from collections import Counter
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import subprocess
import sys

import pytest

ROOT = Path(__file__).resolve().parents[1]
GENERATOR = ROOT / "core" / "solver" / "performance_harness" / "rack" / "rack_model.py"
HASHES = ROOT / "core" / "solver" / "performance_harness" / "observations" / "rack" / "models_sha256.txt"
MODEL_FORMAT = ROOT / "core" / "product_physics" / "src" / "model_format.rs"
NONLINEAR_INTEGRATION = ROOT / "core" / "solver" / "nonlinear_integration" / "src" / "lib.rs"
SIZES = (250, 500, 1000, 1500, 2000)
# The model document version whose shapes rack_model.py was written against. When the current version moves, the
# test below fails on purpose: the generator needs a re-check, not only a new version token.
RACK_SHAPES_VERSION = "0.9.0"


def load_generator():
    spec = importlib.util.spec_from_file_location("rack_model", GENERATOR)
    module = importlib.util.module_from_spec(spec)
    sys.modules["rack_model"] = module
    spec.loader.exec_module(module)
    return module


rm = load_generator()


def current_model_version() -> str:
    return re.search(r'pub const MODEL_SCHEMA_VERSION: &str = "([^"]+)";',
                     MODEL_FORMAT.read_text(encoding="utf-8")).group(1)


def walk(value, path=""):
    """(path, key, value) of every dict member, depth first."""
    if isinstance(value, dict):
        for key, item in value.items():
            yield path, key, item
            yield from walk(item, f"{path}.{key}")
    elif isinstance(value, list):
        for index, item in enumerate(value):
            yield from walk(item, f"{path}[{index}]")


# -- version ------------------------------------------------------------------------------------------------------
def test_the_document_carries_the_current_model_version_read_from_model_format():
    document, stats = rm.build("linear-ex", 250)
    assert document["schema_version"] == current_model_version() == rm.MODEL_VERSION == stats["model_version"]
    source = GENERATOR.read_text(encoding="utf-8")
    assert f'"{current_model_version()}"' not in source, "rack_model.py must not hard-code the model version"


def test_the_rack_shapes_were_taken_at_the_current_model_version():
    current = current_model_version()
    assert current == RACK_SHAPES_VERSION, (
        f"model document version moved from {RACK_SHAPES_VERSION} to {current}: the RT-U4 rack generator "
        "(core/solver/performance_harness/rack/rack_model.py) still writes the shapes of "
        f"{RACK_SHAPES_VERSION}. The version ripple (0.9.0 S1) must re-check it: operating condition sets in place "
        "of per-case temperature, pressure and weight; HYD's test_condition; srss/absolute_sum and combination "
        "operands; EXP's difference_basis; its LEDGER rows; then regenerate models_sha256.txt (--hashes), re-run "
        "this file's solve tests and update RACK_SHAPES_VERSION")


# -- determinism, sizes and layout ----------------------------------------------------------------------------------
@pytest.mark.parametrize("profile", sorted(rm.PROFILES))
def test_generation_is_deterministic(profile):
    first = rm.canonical(rm.build(profile, 500)[0])
    second = rm.canonical(rm.build(profile, 500)[0])
    assert first == second
    assert first == json.dumps(json.loads(first), sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()


def test_the_cli_writes_the_canonical_document_and_a_one_line_stats_object(tmp_path):
    out = tmp_path / "rack.json"
    completed = subprocess.run([sys.executable, str(GENERATOR), "--profile", "nonlinear-pf", "--nodes", "250",
                                "-o", str(out)], capture_output=True, text=True, check=False)
    assert completed.returncode == 0, completed.stderr
    document, stats = rm.build("nonlinear-pf", 250)
    assert out.read_bytes() == rm.canonical(document)
    assert completed.stderr.count("\n") == 1 and json.loads(completed.stderr) == stats
    to_stdout = subprocess.run([sys.executable, str(GENERATOR), "--profile", "nonlinear-pf", "--nodes", "250"],
                               capture_output=True, check=False)
    assert to_stdout.stdout == out.read_bytes()


@pytest.mark.parametrize("profile", sorted(rm.PROFILES))
def test_below_a_profiles_minimum_generation_fails_with_a_clear_message(profile):
    minimum = rm.PROFILES[profile]["min_nodes"]
    rm.build(profile, minimum)
    with pytest.raises(ValueError, match=f"profile {profile} needs at least {minimum} nodes"):
        rm.build(profile, minimum - 1)
    completed = subprocess.run([sys.executable, str(GENERATOR), "--profile", profile, "--nodes", str(minimum - 1)],
                               capture_output=True, text=True, check=False)
    assert completed.returncode == 2 and f"needs at least {minimum} nodes" in completed.stderr
    assert completed.stdout == ""


def expected_counts(profile: str, nodes: int) -> dict:
    """Closed forms of the bay layout (rack_model.py's module docstring)."""
    bays, tail = rm.bays_and_tail(nodes)
    rests = 4 * bays + (tail - 1) // 4  # s4, s12, s16 and the loop top per bay; every fourth tail node
    linear = {"anchor": 2 + bays, "line_stop": bays, "guide": rests // 3,
              "vertical_support": rests - rests // 3 + bays + 2 * bays}
    if profile == "nonlinear-pf":
        lift_off, gap = (rests + 2) // 3, rests // 6
        friction = sum(1 for i in range(1, rests + 1) if i % 12 in (2, 5, 8))
        supports = {"anchor": 2 + bays, "line_stop": bays, "guide": rests // 3 - gap, "gap": gap,
                    "lift_off": lift_off, "friction": friction,
                    "vertical_support": (rests + 1) // 3 + gap + bays + 2 * bays}
    else:
        supports = linear
    weight_loads = nodes - 1 + 3 * bays  # every member but the 3 rigid bodies per bay; a force and a moment each
    cases = 7 if profile == "linear-ex" else 1
    return {
        "nodes": nodes, "members": nodes - 1, "bends": 4 * bays,
        "components": {"bend": 4 * bays, "flange": 2 * bays, "valve": bays},
        "supports": supports, "cases": cases, "combinations": 5 if profile == "linear-ex" else 0,
        "regions_per_case": 2 * bays + 1 if rm.PROFILES[profile]["route"] == "EX" else 0,
        "primitive_loads": cases * weight_loads + (nodes - 1),  # thermal on every member of OPE
    }


@pytest.mark.parametrize("profile", sorted(rm.PROFILES))
@pytest.mark.parametrize("nodes", SIZES)
def test_exact_node_counts_and_closed_form_structure(profile, nodes):
    document, stats = rm.build(profile, nodes)
    expected = expected_counts(profile, nodes)
    assert len(document["nodes"]) == nodes == stats["nodes"]
    assert len(document["pipe_segments"]) == expected["members"] == stats["members"]
    assert stats["bends"] == expected["bends"]
    assert stats["components"] == expected["components"]
    assert Counter(s["nonlinear"]["behavior"] if s["family"] == "nonlinear" else s["family"]
                   for s in document["supports"]) == expected["supports"] == stats["supports"]
    assert len(document["load_cases"]) == expected["cases"] == len(stats["cases"])
    assert len(document["combinations"]) == expected["combinations"]
    assert stats["regions_per_case"] == expected["regions_per_case"]
    assert stats["primitive_loads"] == expected["primitive_loads"]
    # A tree: every node is reached and no member repeats a node pair.
    ends = {node["id"] for node in document["nodes"]}
    assert {p["from"] for p in document["pipe_segments"]} | {p["to"] for p in document["pipe_segments"]} == ends
    assert len({frozenset((p["from"], p["to"])) for p in document["pipe_segments"]}) == nodes - 1


def test_every_ex_region_is_a_chain_from_terminal_to_terminal_and_the_regions_cover_every_member():
    document, _ = rm.build("linear-ex", 250)
    pipes = {p["id"]: p for p in document["pipe_segments"]}
    for case in document["load_cases"]:
        covered = []
        for region in case["pressure_regions"]:
            start, end = (t["node_ref"] for t in region["terminals"])
            assert {t["closure_transfer"] for t in region["terminals"]} == {"transfers_to_wall"}
            at = start
            for pid in region["member_pipe_ids"]:
                assert pipes[pid]["from"] == at, (case["id"], region["id"], pid)
                at = pipes[pid]["to"]
            assert at == end
            covered += region["member_pipe_ids"]
        assert sorted(covered) == sorted(pipes)


# -- invented identities and provenance -----------------------------------------------------------------------------
@pytest.mark.parametrize("profile", sorted(rm.PROFILES))
def test_identities_are_rk_and_never_the_reference_systems(profile):
    document, _ = rm.build(profile, 250)
    data = rm.canonical(document)
    assert b"RS-" not in data
    ids = [item for _, key, item in walk(document) if key == "id"]
    assert ids and all(":RK-" in value for value in ids), [value for value in ids if ":RK-" not in value][:5]
    assert len(set(ids)) == len(ids)


@pytest.mark.parametrize("profile", sorted(rm.PROFILES))
def test_every_provenance_and_source_says_invented(profile):
    document, _ = rm.build(profile, 250)
    provenances = [item for _, key, item in walk(document) if key == "provenance"]
    assert provenances and set(provenances) == {"invented_rt_u4_perf_rack_not_library_or_project_data"}
    sources = [item for _, key, item in walk(document) if key.endswith("source_reference")]
    assert sources and all("invented" in value for value in sources)
    assert "reference system" not in rm.canonical(document).decode().lower()


def test_realized_bends_carry_a_user_flexibility_factor_and_its_source():
    document, _ = rm.build("linear-ex", 250)
    bends = [c for c in document["components"] if c["kind"] == "bend"]
    assert bends
    for bend in bends:
        assert bend["mechanics_interface"]["solver_consumption"] == "curved_bend_macro_element"
        assert bend["modifiers"]["flexibility_factor_user_value"]["value"] > 0
        assert "invented" in bend["modifiers"]["source_reference"]


# -- ledger ---------------------------------------------------------------------------------------------------------
def test_the_ledger_is_well_formed_and_printed_by_the_cli():
    keys = {"family", "target_form", "interim_form", "refusal_code", "waits_for", "profiles"}
    for entry in rm.LEDGER:
        assert keys <= set(entry) and set(entry["profiles"]) <= set(rm.PROFILES), entry["family"]
        assert all(isinstance(entry[key], str) and entry[key].strip() for key in keys - {"profiles"})
    completed = subprocess.run([sys.executable, str(GENERATOR), "--ledger"], capture_output=True, text=True,
                               check=True)
    assert json.loads(completed.stdout)["entries"] == json.loads(json.dumps(rm.LEDGER))
    listed = json.loads(subprocess.run([sys.executable, str(GENERATOR), "--list-profiles"], capture_output=True,
                                       text=True, check=True).stdout)
    assert sorted(listed) == sorted(rm.PROFILES)
    for profile in rm.PROFILES:
        assert rm.build(profile, 2000)[1]["pending_families_omitted"] == [e["family"] for e in rm.ledger_for(profile, 2000)]


def test_the_nonlinear_size_entry_matches_the_products_dense_loop_guard():
    """The ledger's size bound is where the product's dense-loop memory estimate (its constants read from
    nonlinear_integration) first exceeds its limit for nonlinear-pf."""
    source = NONLINEAR_INTEGRATION.read_text(encoding="utf-8")
    constant = lambda name: re.search(rf"pub const {name}: u128 = ([^;]+);", source).group(1)  # noqa: E731
    per_global, per_free = int(constant("DENSE_LOOP_BYTES_PER_GLOBAL_ENTRY")), int(constant("DENSE_LOOP_BYTES_PER_FREE_ENTRY"))
    assert constant("DENSE_LOOP_MEMORY_CEILING_BYTES") == "12 * (1 << 30) * 9 / 10"
    ceiling = 12 * (1 << 30) * 9 // 10

    def refused(nodes):
        document, _ = rm.build("nonlinear-pf", nodes)
        restrained = {(s["node"], dof) for s in document["supports"] if s["family"] != "nonlinear"
                      for dof in s["restraints"]}
        total = 6 * nodes
        return per_global * total ** 2 + per_free * (total - len(restrained)) ** 2 > ceiling

    bound = rm.NONLINEAR_DENSE_GUARD_NODES
    assert refused(bound) and not refused(bound - 1)
    assert refused(2000) and not refused(1500)


# -- recorded hashes ------------------------------------------------------------------------------------------------
def test_the_recorded_hashes_are_the_generators():
    recorded = HASHES.read_text(encoding="utf-8")
    assert recorded == rm.hashes_text(), (
        "observations/rack/models_sha256.txt differs from the generator: regenerate it with "
        "`python3 core/solver/performance_harness/rack/rack_model.py --hashes` after a deliberate change")
    rows = [line.split() for line in recorded.splitlines() if not line.startswith("#")]
    assert {(profile, int(nodes)) for _, profile, nodes in rows} == {(p, n) for p in rm.PROFILES for n in SIZES}
    for digest, profile, nodes in rows:
        assert hashlib.sha256(rm.canonical(rm.build(profile, int(nodes))[0])).hexdigest() == digest


# -- the product solve ----------------------------------------------------------------------------------------------
# Small sizes: linear-ex at its one-bay minimum, because the headless runner's solve output (the payload plus its
# redaction decisions and findings) grows with every result row; the nonlinear profiles carry one case.
SOLVE_SIZES = {"linear-ex": 46, "nonlinear-ex": 128, "nonlinear-pf": 128}
EXPECTED_WARNINGS = {
    # The realized bends carry the complete directional SIF pair, so the product raises no
    # BEND_USER_MODIFIER_INPUT_MISSING for them.
    "linear-ex": {"HIGH_DISPLACEMENT_REVIEW", "RULE_CHECK_INPUTS_MISSING"},
    "nonlinear-ex": {"HIGH_DISPLACEMENT_REVIEW", "RULE_CHECK_INPUTS_MISSING"},
    "nonlinear-pf": {"HIGH_DISPLACEMENT_REVIEW", "RULE_CHECK_INPUTS_MISSING",
                     "NUMERICAL_INTEGRITY_RECOVERY_BASIS_UNQUALIFIED"},
}
PAYLOAD_END = '  "decisions": [\n'


def runner_binary() -> str:
    runner = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not runner or not Path(runner).is_file():
        pytest.skip("the headless runner (OPENPIPESTRESS_RUNNER_BIN) is not available in this session")
    return runner


def solve(document: dict) -> dict:
    """The runner's mechanics envelope for `document`. The payload precedes the (much larger) redaction decisions
    in the pretty-printed output, so reading stops there."""
    from tools.caepipe_correlation.run_pilot import runner_request

    body = {"request": runner_request("rt-u4-rack", "core/solver/performance_harness/rack/rack_model.py"),
            "solve": {"preview_model": {"model": document, "materials": []}}}
    process = subprocess.Popen([runner_binary(), "solve", "--explicit-local-private-intent", "--input", "-"],
                               stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
    try:
        process.stdin.write(json.dumps(body))
        process.stdin.close()
        lines, truncated = [], False
        for line in process.stdout:
            if line == PAYLOAD_END:
                truncated = True
                break
            lines.append(line)
        text = "".join(lines)
        output = json.loads(text.rstrip().rstrip(",") + "}" if truncated else text)
    finally:
        process.kill()
        process.wait()
    envelope = output["payload"]["mechanics_envelope"]
    assert envelope is not None, output["payload"]["diagnostics"]
    return envelope


@pytest.mark.parametrize("profile", sorted(rm.PROFILES))
def test_the_product_solves_each_profile_at_a_small_size(profile):
    document, _ = rm.build(profile, SOLVE_SIZES[profile])
    envelope = solve(document)
    diagnostics = envelope["diagnostics"]
    blocking = [(d["code"], d["message"]) for d in diagnostics if d["severity"] == "blocking"]
    assert envelope["status"]["mechanics"] == "MECHANICS_SOLVED" and not blocking, blocking[:5]
    assert {d["code"] for d in diagnostics if d["severity"] == "warning"} == EXPECTED_WARNINGS[profile]
    infos = {d["code"] for d in diagnostics if d["severity"] == "info"}
    if profile == "nonlinear-pf":
        assert "NONLINEAR_SUPPORT_LOOP_CONVERGED" in infos
    else:
        assert not any(code.startswith("NONLINEAR_") for code in infos)


def test_the_2000_node_nonlinear_twin_is_refused_by_the_dense_loop_guard():
    """The ledger's size entry, observed: refused before any case, so the run is short."""
    document, stats = rm.build("nonlinear-pf", 2000)
    assert "nonlinear operating solve above about 1,760 nodes" in stats["pending_families_omitted"]
    envelope = solve(document)
    assert [d["code"] for d in envelope["diagnostics"] if d["severity"] == "blocking"] == [
        "NONLINEAR_DENSE_MEMORY_LIMIT_EXCEEDED"]
