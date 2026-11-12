"""DEL-10-05 AC-013 (VER-009): the run evidence of the actual `openpipestress-runner` process,
checked by the independent verifier `tests/verifiers/run_evidence.py`, which imports no product
code, from the request bytes the runner read and the result it emitted.

For each verb and for `size-hangers`, admitted and refused, with local-private intent: every digest
equals the one this verifier computes. Without intent, no digest of the request leaves the process:
the control blocks the output, or it passes the output and the request and model are stated
withheld, which this verifier reports as withheld, never as verified. A request the control cannot
judge in full is withheld whatever the intent. A blocked result and a run that exits 2 carry no
evidence. Each substitution
(another request, a model changed in one member, the result edited, the evidence edited or
re-paired) fails by the named digest. The same request run twice gives the same result digest.
A request outside the canonical profile is stated as unavailable. The Rust program's own checks
of the same runs are `core/runner/headless/tests/run_evidence_cli.rs`.
"""
from __future__ import annotations

import ast
from copy import deepcopy
from hashlib import sha256
import json
import os
from pathlib import Path
import re
import subprocess
import sys

import pytest

from tests.verifiers import canonical_json
from tests.verifiers import run_evidence as rv

ROOT = Path(__file__).resolve().parents[1]
# The runner whose output is checked (its crate selects this test) and the module that states its constants.
HEADLESS_CRATE = ROOT / "core" / "runner" / "headless"
RUST_MODULE = HEADLESS_CRATE / "src" / "run_evidence.rs"
INPUTS = ROOT / "validation" / "witness" / "inputs"
RESULTS = ROOT / "fixtures" / "results"
SOLVED_MODEL = ROOT / "fixtures" / "model_operations" / "precision_connected_ui_model.json"


def _load(path: Path):
    return json.loads(path.read_text(encoding="utf-8"))


def _solve_admitted() -> bytes:
    request = _load(INPUTS / "tp_runner_015_final_cli_solve_input.json")
    request["solve"]["preview_model"]["model"] = _load(SOLVED_MODEL)
    return json.dumps(request, indent=2).encode()


def _validate_admitted() -> bytes:
    request = _load(INPUTS / "tp_runner_015_final_cli_solve_input.json")
    request["request"]["operation"] = "validate_input"
    request["request"]["operation_ref"]["ref_id"] = "validate_input"
    del request["solve"]
    return json.dumps(request).encode()


def _validate_carrying_a_model() -> bytes:
    request = json.loads(_validate_admitted())
    request["solve"] = {"preview_model": {"model": _load(SOLVED_MODEL)}}
    return json.dumps(request).encode()


def _regression_admitted() -> bytes:
    request = _load(INPUTS / "del1005_payload_binding_benchmark_single_case_input.json")
    request["request"]["operation"] = "run_regression"
    request["request"]["operation_ref"]["ref_id"] = "run_regression"
    del request["benchmark"]
    request["regression"] = {"suite": "nonlinear", "cases": ["NL-NONCONVERGENCE-LIMIT-ORIGINAL"]}
    return json.dumps(request).encode()


def _cases() -> list[tuple[str, str, bytes, int]]:
    return [
        ("solve-admitted", "solve", _solve_admitted(), 0),
        ("solve-refused", "solve", (INPUTS / "tp_runner_015_final_cli_solve_input.json").read_bytes(), 1),
        ("validate-admitted", "validate-input", _validate_admitted(), 0),
        ("validate-carrying-a-model", "validate-input", _validate_carrying_a_model(), 0),
        ("validate-refused", "validate-input", (INPUTS / "tp_runner_015_final_cli_validation_blocking_input.json").read_bytes(), 1),
        ("export-admitted", "export-results", (INPUTS / "del1005_export_results_success_input.json").read_bytes(), 0),
        ("export-refused", "export-results", (INPUTS / "del1005_export_results_missing_payload_input.json").read_bytes(), 1),
        ("benchmark-admitted", "run-benchmark", (INPUTS / "del1005_payload_binding_benchmark_single_case_input.json").read_bytes(), 0),
        ("benchmark-refused", "run-benchmark", (INPUTS / "del1005_payload_binding_benchmark_payload_missing_input.json").read_bytes(), 1),
        ("regression-admitted", "run-regression", _regression_admitted(), 0),
        ("regression-refused", "run-regression", (INPUTS / "del1005_payload_binding_regression_payload_missing_input.json").read_bytes(), 1),
        ("sizing-admitted", "size-hangers", (RESULTS / "hanger_sizing_input_two_span.json").read_bytes(), 0),
        ("sizing-refused", "size-hangers", (RESULTS / "hanger_sizing_input_refused.json").read_bytes(), 1),
    ]


@pytest.fixture(scope="module")
def runner():
    binary = os.environ.get("OPENPIPESTRESS_RUNNER_BIN")
    if not binary:
        pytest.skip("OPENPIPESTRESS_RUNNER_BIN is neither supplied nor built")

    def run(args: list[str], stdin: bytes, *, stderr: bool = False):
        completed = subprocess.run([binary, *args], input=stdin, capture_output=True, timeout=600)
        if stderr:
            return completed.returncode, completed.stdout, completed.stderr
        return completed.returncode, completed.stdout
    return run


@pytest.fixture(scope="module")
def outputs(runner):
    return {name: (command, raw, code, *runner([command, "--explicit-local-private-intent"], raw))
            for name, command, raw, code in _cases()}


def _strict(stdout: bytes) -> dict:
    return json.loads(stdout, object_pairs_hook=rv._unique_members)


@pytest.mark.parametrize("name", [case[0] for case in _cases()])
def test_each_digest_equals_the_independent_one(outputs, name):
    command, raw, code, actual_code, stdout = outputs[name]
    assert actual_code == code, stdout[:2000]
    emitted = _strict(stdout)
    assert list(emitted) == [*rv.CONTROL_MEMBERS, rv.MEMBER]
    assert emitted["blocked"] is False
    assert rv.verify(raw, emitted) == []
    evidence = emitted[rv.MEMBER]
    hashed = lambda value: sha256(canonical_json.canonical_json(value).encode()).hexdigest()  # noqa: E731
    request = json.loads(raw)
    assert evidence["request"] == {"raw_sha256": sha256(raw).hexdigest(), "canonical_sha256": hashed(request)}
    pointer = rv.MODEL_POINTERS.get(command)
    if pointer is None:
        assert evidence["model"] == {"absent": rv.ABSENT_NOT_CARRIED}
    else:
        model = request
        for token in pointer.split("/")[1:]:
            model = model[token]
        assert evidence["model"] == {"pointer": pointer, "canonical_sha256": hashed(model)}
    assert evidence["result"] == {"canonical_sha256": hashed({k: v for k, v in emitted.items() if k != rv.MEMBER})}
    assert evidence["binding_sha256"] == hashed({"domain": rv.BINDING_DOMAIN,
                                                 "payload": {k: v for k, v in evidence.items() if k != "binding_sha256"}})
    assert evidence["invocation"] == {"command": command, "explicit_local_private_intent": True,
                                      "solver_mode": "sparse_interactive" if command == "solve" else None}
    assert evidence["evidence_version"] == rv.EVIDENCE_VERSION and evidence["canonical_profile"] == canonical_json.PROFILE


def test_a_blocked_result_and_a_run_that_exits_2_carry_no_run_evidence(runner):
    for command, raw in (("solve", _solve_admitted()), ("size-hangers", (RESULTS / "hanger_sizing_input_two_span.json").read_bytes())):
        code, stdout = runner([command], raw)
        emitted = _strict(stdout)
        assert code == 1 and emitted["blocked"] is True and emitted["payload"] is None
        assert rv.MEMBER not in emitted and b"run_evidence" not in stdout
    for args in (["solve", "--explicit-local-private-intent"], ["preview", "--explicit-local-private-intent"]):
        code, stdout = runner(args, b"{")
        assert code == 2 and b"run_evidence" not in stdout


def test_each_substitution_fails_by_the_named_digest(outputs):
    _, raw, _, _, stdout = outputs["solve-admitted"]
    emitted = _strict(stdout)
    request_and_model = ["model.canonical_sha256", "request.canonical_sha256", "request.raw_sha256"]
    other = outputs["solve-refused"][1]
    assert rv.verify(other, emitted) == request_and_model
    changed = json.loads(raw)
    changed["solve"]["preview_model"]["model"]["project"]["name"] += " (edited)"
    assert rv.verify(json.dumps(changed, indent=2).encode(), emitted) == request_and_model
    assert rv.verify(json.dumps(json.loads(raw)).encode(), emitted) == ["request.raw_sha256"]
    edited = deepcopy(emitted)
    edited["payload"]["runner_result"]["run_id"] = "run:headless-preview:another"
    assert rv.verify(raw, edited) == ["result.canonical_sha256"]
    for path, value in ((("invocation", "solver_mode"), "dense_scrutiny"), (("producer", "build_ref"), "another@0.0.0"),
                        (("result", "canonical_sha256"), rv.verify.__name__)):
        edited = deepcopy(emitted)
        edited[rv.MEMBER][path[0]][path[1]] = value
        expected = ["binding_sha256"] + (["result.canonical_sha256"] if path[0] == "result" else [])
        assert rv.verify(raw, edited) == expected, path
    for binding in (None, {"unavailable": "CHECKED-JSON-UNSAFE-INTEGER"}):
        edited = deepcopy(emitted)
        edited[rv.MEMBER]["producer"]["build_ref"] = 2**53 + 1
        edited[rv.MEMBER]["binding_sha256"] = binding
        assert rv.verify(raw, edited) == ["binding_sha256"]
    repaired = deepcopy(emitted)
    repaired[rv.MEMBER] = _strict(outputs["solve-refused"][4])[rv.MEMBER]
    assert rv.verify(raw, repaired) == request_and_model + ["result.canonical_sha256"]
    assert rv.verify(other, repaired) == ["result.canonical_sha256"]
    stripped = {k: v for k, v in emitted.items() if k != rv.MEMBER}
    assert rv.verify(raw, stripped) == [rv.MEMBER]


def test_the_same_request_run_twice_gives_the_same_result_digest(runner, outputs):
    for name in ("solve-admitted", "export-admitted", "sizing-admitted"):
        command, raw, _, _, stdout = outputs[name]
        again = runner([command, "--explicit-local-private-intent"], raw)[1]
        assert again == stdout, name
        assert _strict(again)[rv.MEMBER]["result"] == _strict(stdout)[rv.MEMBER]["result"]


@pytest.mark.parametrize("member", ["9007199254740993", "1e16"])
def test_a_request_outside_the_canonical_profile_is_stated_as_unavailable(runner, member):
    raw = _validate_admitted().rstrip()[:-1] + b',"note":' + member.encode() + b"}"
    code, stdout = runner(["validate-input", "--explicit-local-private-intent"], raw)
    assert code == 0
    emitted = _strict(stdout)
    stated = emitted[rv.MEMBER]["request"]["canonical_sha256"]
    assert type(stated) is dict and set(stated) == {"unavailable"} and stated["unavailable"].startswith("CHECKED-JSON-")
    with pytest.raises(ValueError):
        canonical_json.canonical_json(rv.checked_parse(raw))
    assert rv.verify(raw, emitted) == []
    assert "note" not in json.dumps(emitted[rv.MEMBER])


def _request_digests(command: str, raw: bytes) -> list[str]:
    digests = [sha256(raw).hexdigest()]
    try:
        request = rv.checked_parse(raw)
        digests.append(sha256(canonical_json.canonical_json(request).encode()).hexdigest())
    except ValueError:
        return digests
    found, model = rv._pointer(request, rv.MODEL_POINTERS.get(command, "/no/model"))
    if found:
        digests.append(sha256(canonical_json.canonical_json(model).encode()).hexdigest())
    return digests


@pytest.mark.parametrize("name", [case[0] for case in _cases()])
def test_without_intent_no_digest_of_the_request_leaves_the_process(runner, name):
    """Every verb and size-hangers, admitted and refused, without intent: the control blocks the output, or
    it passes it and the request and model are withheld; no request digest is on standard output or error."""
    command, raw, _ = next((c, r, k) for n, c, r, k in _cases() if n == name)
    code, stdout, stderr = runner([command], raw, stderr=True)
    for digest in _request_digests(command, raw):
        assert digest.encode() not in stdout and digest.encode() not in stderr, name
    emitted = _strict(stdout)
    if emitted["blocked"]:
        assert code == 1 and rv.MEMBER not in emitted
        return
    assert command == "validate-input", name  # the one command whose output the control passes without intent
    evidence = emitted[rv.MEMBER]
    assert evidence["request"] == evidence["model"] == {"withheld": "REQUEST_BLOCKED_BY_LOCAL_PRIVATE_CONTROL"}
    assert rv.verify(raw, emitted) == rv.WITHHELD_REPORT
    # Not comparable: another request is not detected, and the outcome is still withheld, never verified.
    assert rv.verify(b"{}", emitted) == rv.WITHHELD_REPORT
    edited = deepcopy(emitted)
    edited["summary"]["redacted_count"] = 7
    assert rv.verify(raw, edited) == ["result.canonical_sha256"]
    unwithheld = deepcopy(emitted)
    unwithheld[rv.MEMBER]["request"] = {"raw_sha256": sha256(raw).hexdigest(), "canonical_sha256": "0" * 64}
    assert "binding_sha256" in rv.verify(raw, unwithheld)


def test_withheld_absent_and_complete_evidence_are_told_apart_and_only_complete_binds(runner):
    """Withheld is stated member by member; it is distinguishable from a complete result and from one without
    run evidence; a check that requires the binding fails closed on the withheld and the absent forms."""
    raw = _validate_carrying_a_model()
    withheld = _strict(runner(["validate-input"], raw)[1])
    complete = _strict(runner(["validate-input", "--explicit-local-private-intent"], raw)[1])
    absent = {k: v for k, v in complete.items() if k != rv.MEMBER}
    for member in ("request", "model"):
        assert withheld[rv.MEMBER][member] == {"withheld": "REQUEST_BLOCKED_BY_LOCAL_PRIVATE_CONTROL"}
    assert rv.verify(raw, complete) == []
    assert rv.verify(raw, withheld) == rv.WITHHELD_REPORT
    assert rv.verify(raw, absent) == [rv.MEMBER]
    assert rv.bound(raw, complete) == rv.BOUND
    assert rv.bound(raw, withheld) == rv.NOT_BOUND_WITHHELD
    assert rv.bound(raw, absent) == rv.NOT_BOUND_NO_RUN_EVIDENCE
    half = deepcopy(withheld)
    half[rv.MEMBER]["model"] = complete[rv.MEMBER]["model"]
    assert rv.bound(raw, half).startswith("not bound: mismatch")


def test_a_request_the_control_passes_carries_complete_evidence_without_intent(runner):
    request = json.loads(_validate_admitted())
    for member in ("project_ref", "model_ref"):
        request["request"][member] = {"ref_type": "unknown", "ref_id": "unknown"}
    request["request"]["provenance"]["source_location"] = "unknown"
    raw = json.dumps(request).encode()
    code, stdout = runner(["validate-input"], raw)
    emitted = _strict(stdout)
    assert emitted["blocked"] is False
    assert rv.verify(raw, emitted) == []
    assert emitted[rv.MEMBER]["request"]["raw_sha256"] == sha256(raw).hexdigest()


@pytest.mark.parametrize("member", ['"source_location":"/an/invented/private/path","source_location":"unknown"',
                                    '"note":"\\ud800"', '"note":1e400'])
def test_a_request_the_control_cannot_judge_is_withheld_whatever_the_intent(runner, member):
    raw = _validate_admitted().rstrip()[:-1] + b"," + member.encode() + b"}"
    for args in (["validate-input", "--explicit-local-private-intent"], ["validate-input"]):
        code, stdout = runner(args, raw)
        emitted = _strict(stdout)
        if emitted["blocked"]:
            assert rv.MEMBER not in emitted
            continue
        evidence = emitted[rv.MEMBER]
        assert evidence["request"] == evidence["model"] == {"withheld": "REQUEST_NOT_JUDGED_BY_LOCAL_PRIVATE_CONTROL"}
        assert sha256(raw).hexdigest().encode() not in stdout
        assert rv.verify(raw, emitted) == rv.WITHHELD_REPORT


def test_the_rust_module_states_the_verifiers_constants():
    """The program's module names the same domain, version, pointers and absence reasons (read from its text)."""
    text = RUST_MODULE.read_text(encoding="utf-8")
    constants = dict(re.findall(r'pub const (\w+): &str = "([^"]*)";', text))
    assert constants["MEMBER"] == rv.MEMBER and constants["EVIDENCE_VERSION"] == rv.EVIDENCE_VERSION
    assert constants["BINDING_DOMAIN"] == rv.BINDING_DOMAIN
    assert {"solve": constants["SOLVE_MODEL_POINTER"], "size-hangers": constants["SIZE_HANGERS_MODEL_POINTER"]} == rv.MODEL_POINTERS
    assert (constants["ABSENT_NOT_CARRIED"], constants["ABSENT_NOT_PRESENT"]) == (rv.ABSENT_NOT_CARRIED, rv.ABSENT_NOT_PRESENT)
    assert (constants["WITHHELD_BLOCKED"], constants["WITHHELD_UNJUDGED"]) == rv.WITHHELD_CODES


def test_the_verifier_is_independent_of_the_producer(outputs, monkeypatch):
    """It imports only the standard library and its sibling `canonical_json`, and verifies without any process."""
    tree = ast.parse((ROOT / "tests" / "verifiers" / "run_evidence.py").read_text(encoding="utf-8"))
    for node in ast.walk(tree):
        if isinstance(node, ast.ImportFrom) and node.level:
            assert [alias.name for alias in node.names] == ["canonical_json"]
        elif isinstance(node, (ast.Import, ast.ImportFrom)):
            names = [alias.name for alias in node.names] if isinstance(node, ast.Import) else [node.module]
            assert all(name.split(".")[0] in sys.stdlib_module_names and name != "subprocess" for name in names), names

    def forbidden(*args, **kwargs):
        raise AssertionError("the verifier spawned a process")
    for name in ("run", "Popen", "call", "check_call", "check_output"):
        monkeypatch.setattr(subprocess, name, forbidden)
    _, raw, _, _, stdout = outputs["sizing-admitted"]
    assert rv.verify(raw, _strict(stdout)) == []
