"""Independent J4 verifier contexts; no relabelled source is a numerical pass.

Genuine existing v2 producer bytes supply the unchanged selected-route control.
New genuine v3 admission and shared J2 mutation parity await the producer corpus.
"""
from copy import deepcopy
import ast
import re
import json
from pathlib import Path

import pytest

from tests.verifiers import compatibility as c
from tests.verifiers import physics_evidence as physical
from tests.verifiers import physics_source as ps
from tests.verifiers import reader_verdict as verdict
from tests.verifiers import source_blocks as blocks

ROOT = Path(__file__).resolve().parents[1]


def v2():
    return json.loads((ROOT / "fixtures/product_preview/physics_source/n06-sparse_interactive.raw.json").read_text())


def invocation():
    return {"request": json.loads((ROOT / "fixtures/product_preview/physics_source/n06.request.json").read_text()), "solver_mode": "sparse_interactive"}


def test_caller_selected_source_cross_reads_precede_generic_shape_errors():
    source = v2()
    for read in (ps.validate_physics_source, ps.validate_transport_metadata):
        with pytest.raises(ValueError, match=physical.V2_READ_AS_V3):
            read(source, source_contract=ps.SourceContract.PRESSURE_V3)
    relabelled = deepcopy(source)
    relabelled["producer"]["semantic_contract_id"] = ps.PRESSURE_CONTRACT_ID
    relabelled["formulation_basis"]["profile_id"] = ps.PRESSURE_PROFILE
    relabelled["source_block_recovery"] = {}  # Generic shape failure must not hide the cross-read.
    for context, code in ((ps.SourceContract.PHYSICS_V2, physical.V3_READ_AS_V2), (ps.SourceContract.PRESSURE_V3, physical.V2_READ_AS_V3)):
        for read in (ps.validate_physics_source, ps.validate_transport_metadata):
            with pytest.raises(ValueError, match=code): read(relabelled, source_contract=context)
    assert verdict.validate("pressure-source", source)["error"]["code"] == physical.V2_READ_AS_V3
    assert verdict.transport("pressure-source", source)["error"]["code"] == physical.V2_READ_AS_V3


def test_captured_contract_fixes_context_independently_of_received_labels():
    captured = invocation()
    captured["request"]["model"]["pressure_contract"] = {"version": "3.0.0", "mode": ps.PRESSURE_PROFILE}
    assert ps.SourceContract.from_invocation(captured) is ps.SourceContract.PRESSURE_V3
    with pytest.raises(ValueError, match=physical.V3_READ_AS_V2):
        ps.validate_physics_source(v2(), captured)
    with pytest.raises(ValueError, match=physical.V2_READ_AS_V3):
        ps.validate_physics_source({}, invocation(), source_contract=ps.SourceContract.PRESSURE_V3)


@pytest.mark.parametrize("context", [None, "pressure_v3", {}, physical.PRESSURE_V3])
def test_malformed_explicit_context_never_defaults_to_a_reader(context):
    for read in (ps.validate_physics_source, ps.validate_transport_metadata):
        with pytest.raises(ValueError, match="PHYSICS_SOURCE_CONTEXT_UNSUPPORTED"):
            read(v2(), source_contract=context)


def test_captured_0_4_requires_the_received_resolved_state_namespace():
    # Contradictory context input only; this is never a fabricated v3 admission.
    source = v2()
    source["producer"]["semantic_contract_id"] = ps.PRESSURE_CONTRACT_ID
    source["formulation_basis"]["profile_id"] = ps.PRESSURE_PROFILE
    for case in source["contract_evidence"]["exact_cases"]: case["profile_mode"] = ps.PRESSURE_PROFILE
    captured = invocation()
    captured["request"]["model"]["schema_version"] = "0.4.0"
    captured["request"]["model"]["pressure_contract"] = {"version": "3.0.0", "mode": ps.PRESSURE_PROFILE}
    with pytest.raises(ValueError, match="PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT"):
        ps.validate_physics_source(source, captured, source_contract=ps.SourceContract.PRESSURE_V3)


def test_pressure_source_receipt_downgrade_is_refused_on_raw_header_and_transport():
    source = v2()
    source["producer"]["semantic_contract_id"] = physical.PRESSURE_ID
    source["formulation_basis"]["profile_id"] = ps.PRESSURE_PROFILE
    with pytest.raises(ValueError, match="SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"):
        c._source_contract(source)
    with pytest.raises(ValueError, match="SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"):
        c._source_contract(source, check_receipt=False)
    with pytest.raises(ValueError, match="SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"):
        ps.validate_transport_metadata(source, source_contract=ps.SourceContract.PRESSURE_V3)
    assert verdict.dispatch(source)["error"]["code"] == "SOURCE_BLOCKS_LEGACY_DOWNGRADE_FORBIDDEN"


def test_v2_selected_source_admission_and_standing_stay_unchanged():
    source, captured = v2(), invocation()
    before = deepcopy(source)
    assert ps.validate_physics_source(source, captured) is True
    assert ps.validate_physics_source(source) is False
    ps.validate_transport_metadata(source)
    assert c.numerical_use_standing(source, [{"ref_type": "load_case", "ref_id": "case"}], captured) == "numerically_eligible"
    assert source == before


def test_pressure_binding_pin_and_shared_row_alias_are_checked():
    data = ps.PRESSURE_TABLE_PATH.read_bytes()
    table = ps.verify_pressure_source_table(data)
    assert table["semantic_contract_id"] == ps.PRESSURE_CONTRACT_ID
    assert table["rows"] == ps.TABLE["rows"]
    assert ps.pressure_source_table() is ps.TABLE
    assert verdict._table(ps.PRESSURE_CONTRACT_ID) is ps.TABLE
    with pytest.raises(ValueError, match="SOURCE_PRESSURE_SOURCE_TABLE_HASH"):
        ps.verify_pressure_source_table(data + b"\n")


def _python_function(path, name):
    return next(node for node in ast.parse(path.read_text()).body if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and node.name == name)


def _calls(node):
    return [call for call in ast.walk(node) if isinstance(call, ast.Call)]


def _call_name(call):
    return ast.unparse(call.func)


def _assert_direct_python_join(node):
    calls = _calls(node)
    assert not any(_call_name(call).rsplit(".", 1)[-1] in {"_project", "project", "project_load_reference", "deepcopy", "_reseal", "reseal"} for call in calls)
    prepass = next(call for call in calls if _call_name(call) == "lr._prepass")
    assert [ast.unparse(arg) for arg in prepass.args] == ["source", "raw", "lr.PRESSURE_JOINED"]


def _rust_function(path, name):
    # A bounded source guard, not a Rust parser. Comments and quoted strings are
    # skipped for brace counting so JSON macros and diagnostics cannot end it.
    text = path.read_text()
    match = re.search(r"\bfn\s+" + re.escape(name) + r"(?:<[^\n]*>)?\s*\(", text)
    assert match is not None
    start = text.index("{", match.end())
    depth, index, quoted = 1, start + 1, False
    while index < len(text):
        if quoted:
            if text[index] == "\\": index += 2; continue
            if text[index] == '"': quoted = False
        elif text.startswith("//", index):
            index = text.find("\n", index)
            if index < 0: break
            continue
        elif text[index] == '"': quoted = True
        elif text[index] == "{": depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0: return text[start + 1:index]
        index += 1
    raise AssertionError(f"Unclosed Rust function {name}")


def _compact_rust(text):
    return re.sub(r"\s+", "", re.sub(r"//[^\n]*", "", text))


def _assert_direct_rust_join(text):
    compact = _compact_rust(text)
    for forbidden in ("project(", "project_load_reference(", "reseal(", "receipt_sha256="):
        assert forbidden not in compact
    assert "prepass(source,raw,Method::PressureJoined)" in compact


def test_pressure_joined_guard_traces_actual_python_caller_context_and_keeps_v2_projection_isolated():
    base = ROOT / "tests/verifiers"
    dispatch = _python_function(base / "reader_verdict.py", "validate")
    selector = next(node for node in ast.walk(dispatch) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "context" for target in node.targets))
    assert isinstance(selector.value, ast.IfExp)
    assert ast.unparse(selector.value.test) == "reader == 'pressure-source'"
    assert ast.unparse(selector.value.body) == "physics_source.SourceContract.PRESSURE_V3"
    relay = _python_function(base / "reader_verdict.py", "_source_validate")
    read = next(call for call in _calls(relay) if _call_name(call) == "physics_source.validate_physics_source")
    assert ast.unparse(read.args[0]) == "source"
    assert {kw.arg: ast.unparse(kw.value) for kw in read.keywords}["source_contract"] == "context"
    raw = _python_function(base / "physics_source.py", "validate_physics_source")
    assert not any(_call_name(call).rsplit(".", 1)[-1] in {"_project", "project", "project_load_reference", "deepcopy", "_reseal", "reseal"} for call in _calls(raw))
    resolved = next(node for node in ast.walk(raw) if isinstance(node, ast.If) and isinstance(node.test, ast.Name) and node.test.id == "resolved")
    call = next(call for call in _calls(resolved) if _call_name(call) == "validate_pressure_source_evidence")
    assert ast.unparse(call.args[0]) == "source" and {kw.arg: ast.unparse(kw.value) for kw in call.keywords} == {"raw": "True"}
    physical = next(call for call in _calls(raw) if _call_name(call) == "physical._validate_physics_evidence")
    assert ast.unparse(physical.args[0]) == "source"
    assert {kw.arg: ast.unparse(kw.value) for kw in physical.keywords} == {"context": "contract", "contract": "contract.pressure"}
    direct = _python_function(base / "load_reference_source.py", "validate_pressure_source_evidence")
    _assert_direct_python_join(direct)
    # This guard deliberately permits the still-current V2 joined path.
    legacy = _python_function(base / "load_reference_source.py", "_validate")
    assert any(_call_name(call) == "_project" for call in _calls(legacy))
    assert any(_call_name(call) == "lr._prepass" and ast.unparse(call.args[2]) == "lr.JOINED" for call in _calls(legacy))


def test_pressure_joined_guard_traces_actual_rust_caller_context_and_keeps_v2_projection_isolated():
    base = ROOT / "core/reporting/result_export/src"
    dispatch = _compact_rust(_rust_function(base / "reader_verdict.rs", "validate"))
    assert 'ifreader=="pressure-source"{crate::physics_source::SourceContract::PressureV3}' in dispatch
    assert "source_validate_in(source,invocation,composite,contract)" in dispatch
    relay = _compact_rust(_rust_function(base / "reader_verdict.rs", "source_validate_in"))
    assert "crate::physics_source::validate_in(source,invocation,contract)" in relay
    raw = _compact_rust(_rust_function(base / "physics_source.rs", "validate_in"))
    assert all(forbidden not in raw for forbidden in ("project(", "project_load_reference(", "reseal("))
    assert "letresolved=contract==SourceContract::PressureV3&&" in raw
    assert "ifresolved{crate::load_reference_source::validate_pressure_source_evidence(source,true)?;}" in raw
    assert "validate_physics_evidence_in(source,true,contract.pressure())" in raw
    assert "validate_transport_metadata_in(source,contract)" in raw
    direct = _rust_function(base / "load_reference_source.rs", "validate_pressure_source_evidence")
    _assert_direct_rust_join(direct)
    legacy = _compact_rust(_rust_function(base / "load_reference_source.rs", "validate"))
    assert "prepass(source,raw,Method::Joined)" in legacy and "project(source,raw)" in legacy


def test_source_qualified_join_guards_kill_a_hash_checked_v2_projection_shim():
    # MJ4: checking incoming hashes does not excuse converting received V3 data.
    python_mutant = ast.parse("def direct(source, raw):\n    _receipt(source, raw)\n    projected = _project(source, raw)\n    lr._prepass(projected, raw, lr.PRESSURE_JOINED)\n").body[0]
    with pytest.raises(AssertionError): _assert_direct_python_join(python_mutant)
    rust_mutant = "receipt(source,raw)?; let projected=project(source,raw)?; prepass(&projected,raw,Method::PressureJoined)"
    with pytest.raises(AssertionError): _assert_direct_rust_join(rust_mutant)


def test_schema_resources_are_explicit_and_unknown_names_or_fragments_fail_closed():
    local = {"$defs": {"number": {"type": "number"}}}
    external = {"$defs": {"value/key~": {"type": "integer"}, "bad~2": {"type": "integer"}}}
    assert blocks._shape(1, {"$ref": "#/$defs/number"}, local)
    rule = {"$ref": "load_reference_state.schema.json#/$defs/value~1key~0"}
    assert not blocks._shape(1, rule, local)
    assert blocks._shape(1, rule, local, resources={"load_reference_state.schema.json": external})
    assert not blocks._shape(1, {"$ref": "unregistered.json#/$defs/value"}, local, resources={"load_reference_state.schema.json": external})
    assert not blocks._shape(1, {"$ref": "load_reference_state.schema.json#/$defs/missing"}, local, resources={"load_reference_state.schema.json": external})
    assert not blocks._shape(1, {"$ref": "load_reference_state.schema.json#anchor"}, local, resources={"load_reference_state.schema.json": external})
    assert not blocks._shape(1, {"$ref": "load_reference_state.schema.json#/$defs/bad~2"}, local, resources={"load_reference_state.schema.json": external})
