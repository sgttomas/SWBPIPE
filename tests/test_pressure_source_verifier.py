"""Independent J4 verifier contexts; no relabelled source is a numerical pass.

B7b: genuine pressure-source-1 producer bytes (`pressure_source/physics_source` n06) supply the selected-route
control; the retired v2 source contexts and their cross-reads went with physics-source-1 and
load-reference-source-1, so a retired v2 label is refused by name at the header (dispatch) and by identity at
the source reader, and a captured v2 contract is refused before any received label is read.
"""
from copy import deepcopy
import ast
import re
import json
from pathlib import Path

import pytest

from core.analysis_runs import rust_reader
from tests.verifiers import compatibility as c
from tests.verifiers import physics_evidence as physical
from tests.verifiers import physics_source as ps
from tests.verifiers import pressure_1
from tests.verifiers import reader_verdict as verdict
from tests.verifiers import source_blocks as blocks

ROOT = Path(__file__).resolve().parents[1]
FOLDER = ROOT / "fixtures/product_preview/pressure_source/physics_source"
RETIRED_SOURCE_IDS = ("openpipestress.result_semantics/0.3.0/physics-source-1", "openpipestress.result_semantics/0.3.0/load-reference-source-1")


def v3():
    return json.loads((FOLDER / "n06-sparse_interactive.raw.json").read_text())


def invocation():
    return {"request": json.loads((FOLDER / "n06.request.json").read_text()), "solver_mode": "sparse_interactive"}


@pytest.mark.parametrize("identity", RETIRED_SOURCE_IDS)
@pytest.mark.parametrize("profile", [None, "exact_straight_pressure_v2"])
def test_a_retired_v2_source_label_is_refused_by_name_and_never_cross_read(identity, profile):
    source = v3()
    source["producer"]["semantic_contract_id"] = identity
    if profile is not None:
        source["formulation_basis"]["profile_id"] = profile
    # The header names the retirement (Rust `current_header`); the source reader refuses the identity.
    for dispatch in (verdict.dispatch(deepcopy(source)), rust_reader.dispatch(deepcopy(source), None)):
        assert dispatch["ok"] is False and dispatch["error"]["code"] == pressure_1.SOURCE_EXACT_PRESSURE_V2_RETIRED
        assert dispatch["error"]["detail"] == pressure_1.V2_RETIRED_TEXT
    with pytest.raises(ValueError) as refused:
        c._source_contract(deepcopy(source))
    assert str(refused.value) == pressure_1.V2_RETIRED_TEXT
    for read in (ps.validate_physics_source, ps.validate_transport_metadata):
        with pytest.raises(ValueError, match="PHYSICS_SOURCE_IDENTITY"):
            read(deepcopy(source), source_contract=ps.SourceContract.PRESSURE_V3)
    assert verdict.validate("pressure-source", deepcopy(source))["error"]["code"] == "PHYSICS_SOURCE_IDENTITY"
    assert verdict.transport("pressure-source", deepcopy(source))["error"]["code"] == "PHYSICS_SOURCE_IDENTITY"
    with pytest.raises(rust_reader.ReaderRefusal, match="PHYSICS_SOURCE_IDENTITY"):
        rust_reader.validate("pressure-source", deepcopy(source))


def test_captured_contract_fixes_context_independently_of_received_labels():
    assert ps.SourceContract.from_invocation(invocation()) is ps.SourceContract.PRESSURE_V3
    assert list(ps.SourceContract) == [ps.SourceContract.PRESSURE_V3]
    for change in (lambda m: m.update(pressure_contract={"version": "2.0.0", "mode": "exact_straight_pressure_v2"}),
                   lambda m: m.update(schema_version="0.4.0"),
                   lambda m: m.update(shear_model="euler_bernoulli")):
        captured = invocation()
        change(captured["request"]["model"])
        with pytest.raises(ValueError, match="PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED"):
            ps.SourceContract.from_invocation(captured)
        # The captured contract is read before any received label: a genuine v3 source does not rescue it.
        with pytest.raises(ValueError, match="PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED"):
            ps.validate_physics_source(v3(), captured)
        with pytest.raises(rust_reader.ReaderRefusal, match="PHYSICS_SOURCE_ACTUAL_CONTRACT_UNSUPPORTED"):
            rust_reader.validate("pressure-source", v3(), invocation=captured)


@pytest.mark.parametrize("context", [None, "pressure_v3", {}, physical.PRESSURE_V3])
def test_malformed_explicit_context_never_defaults_to_a_reader(context):
    for read in (ps.validate_physics_source, ps.validate_transport_metadata):
        with pytest.raises(ValueError, match="PHYSICS_SOURCE_CONTEXT_UNSUPPORTED"):
            read(v3(), source_contract=context)


def test_captured_load_state_presence_requires_the_received_resolved_state_namespace():
    # Contradictory context input only; this is never a fabricated admission. The group keys on its authored
    # presence (here an explicit reference_configurations member), never on a version.
    captured = invocation()
    captured["request"]["model"]["reference_configurations"] = []
    assert ps.model_carries_load_state(captured["request"]["model"])
    with pytest.raises(ValueError, match="PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT"):
        ps.validate_physics_source(v3(), captured, source_contract=ps.SourceContract.PRESSURE_V3)
    with pytest.raises(rust_reader.ReaderRefusal, match="PHYSICS_SOURCE_ACTUAL_LOAD_REFERENCE_CONTEXT"):
        rust_reader.validate("pressure-source", v3(), invocation=captured)


def test_pressure_source_receipt_downgrade_is_refused_on_raw_header_and_transport():
    source = v3()
    source["producer"]["semantic_contract_id"] = pressure_1.CONTRACT_ID
    source["formulation_basis"]["profile_id"] = ps.PRESSURE_PROFILE
    with pytest.raises(ValueError, match="SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"):
        c._source_contract(source)
    with pytest.raises(ValueError, match="SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"):
        c._source_contract(source, check_receipt=False)
    with pytest.raises(ValueError, match="SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"):
        ps.validate_transport_metadata(source, source_contract=ps.SourceContract.PRESSURE_V3)
    assert verdict.dispatch(source)["error"]["code"] == "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"
    assert rust_reader.dispatch(deepcopy(source), None)["error"]["code"] == "SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN"


def test_selected_source_admission_and_standing_stay_unchanged():
    source, captured = v3(), invocation()
    before = deepcopy(source)
    assert ps.validate_physics_source(source, captured) is True
    assert ps.validate_physics_source(source) is False
    ps.validate_transport_metadata(source)
    assert c.numerical_use_standing(source, [{"ref_type": "load_case", "ref_id": "case"}], captured) == "numerically_eligible"
    assert source == before


def test_pressure_binding_pin_is_checked_and_resident_on_its_own():
    data = ps.PRESSURE_TABLE_PATH.read_bytes()
    table = ps.verify_pressure_source_table(data)
    assert table["semantic_contract_id"] == ps.PRESSURE_CONTRACT_ID == c.PRESSURE_SOURCE_CONTRACT_ID
    assert table["source_block_policy"] == "PRESSURE-SOURCE-1"
    assert ps.pressure_source_table()["rows"] == table["rows"]
    assert ps.SourceContract.PRESSURE_V3.TABLE["rows"] == table["rows"]
    assert verdict._table(ps.PRESSURE_CONTRACT_ID)["rows"] == table["rows"]
    # B7b: no v2 table remains to alias.
    assert not hasattr(ps, "TABLE")
    assert not (ROOT / "fixtures/results/semantic_contract_v0_3_physics_source_1.json").exists()
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


def test_pressure_joined_guard_traces_actual_python_caller_context_and_has_no_v2_projection():
    base = ROOT / "tests/verifiers"
    dispatch = _python_function(base / "reader_verdict.py", "validate")
    relay_call = next(call for call in _calls(dispatch) if _call_name(call) == "_source_validate")
    assert [ast.unparse(arg) for arg in relay_call.args] == ["source", "invocation"]
    relay = _python_function(base / "reader_verdict.py", "_source_validate")
    # The relay's only context is the current one (its default); no received label selects it.
    assert [ast.unparse(default) for default in relay.args.defaults] == ["physics_source.SourceContract.PRESSURE_V3"]
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
    # B7b: the v2 joined path (its projection and reseal) is gone, not merely isolated.
    module = ast.parse((base / "load_reference_source.py").read_text())
    names = {node.name for node in module.body if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))}
    assert names == {"validate_pressure_source_evidence"}
    evidence = ast.parse((base / "load_reference_evidence.py").read_text())
    assert not any(isinstance(node, ast.Assign) and any(isinstance(t, ast.Name) and t.id in {"JOINED", "LOAD_REFERENCE"} for t in node.targets) for node in evidence.body)


def test_pressure_joined_guard_traces_actual_rust_caller_context_and_has_no_v2_projection():
    base = ROOT / "core/reporting/result_export/src"
    dispatch = _compact_rust(_rust_function(base / "reader_verdict.rs", "validate"))
    assert '"pressure-source"=>{letcontract=crate::physics_source::SourceContract::PressureV3;' in dispatch
    assert "source_validate_in(source,invocation,contract)" in dispatch
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
    # B7b: the v2 joined reader (prepass Method::Joined, then project) is gone.
    module = _compact_rust((base / "load_reference_source.rs").read_text())
    assert "Method::Joined)" not in module and "project(source,raw)" not in module
    assert re.findall(r"\bfn\s+(\w+)", (base / "load_reference_source.rs").read_text()) == ["validate_pressure_source_evidence"]


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
