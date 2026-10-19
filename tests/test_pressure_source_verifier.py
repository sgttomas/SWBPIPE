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
from tests.verifiers import pressure_2
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
        assert dispatch["ok"] is False and dispatch["error"]["code"] == pressure_2.SOURCE_EXACT_PRESSURE_V2_RETIRED
        assert dispatch["error"]["detail"] == pressure_2.V2_RETIRED_TEXT
    with pytest.raises(ValueError) as refused:
        c._source_contract(deepcopy(source))
    assert str(refused.value) == pressure_2.V2_RETIRED_TEXT
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
    # O4 (§3.4): either shear model is the exact contract.
    timoshenko = invocation()
    timoshenko["request"]["model"]["pressure_contract"]["shear_model"] = "timoshenko_annulus_energy_v1"
    assert ps.SourceContract.from_invocation(timoshenko) is ps.SourceContract.PRESSURE_V3
    for change in (lambda m: m.update(pressure_contract={"version": "2.0.0", "mode": "exact_straight_pressure_v2"}),
                   lambda m: m.update(schema_version="0.4.0"),
                   lambda m: m.update(shear_model="euler_bernoulli"),
                   # O4 (§3.4): an unknown, absent, null or non-string shear model and an extra contract member.
                   lambda m: m["pressure_contract"].update(shear_model="cowper"),
                   lambda m: m["pressure_contract"].pop("shear_model"),
                   lambda m: m["pressure_contract"].update(shear_model=None),
                   lambda m: m["pressure_contract"].update(shear_model=5),
                   lambda m: m["pressure_contract"].update(shear_factor=0.5)):
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
    source["producer"]["semantic_contract_id"] = pressure_2.CONTRACT_ID
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


CA_REFUSED = {
    "absent": lambda s: s.pop("corrosion_allowance"),
    "null": lambda s: s.update(corrosion_allowance=None),
    "wrong dimension (0 Pa)": lambda s: s.update(corrosion_allowance={"value": 0.0, "unit": "Pa"}),
    "malformed value (string)": lambda s: s.update(corrosion_allowance={"value": "0", "unit": "m"}),
    # O4 (§3.4): a negative or over-wall allowance, or one that with the mill tolerance reaches the wall (t = 0.01 m).
    "negative": lambda s: s.update(corrosion_allowance={"value": -0.001, "unit": "m"}),
    "the wall": lambda s: s.update(corrosion_allowance={"value": 0.01, "unit": "m"}),
    "above the wall (mm)": lambda s: s.update(corrosion_allowance={"value": 20.0, "unit": "mm"}),
    "with the mill tolerance, the wall": lambda s: s.update(corrosion_allowance={"value": 0.006, "unit": "m"}, mill_tolerance={"value": 4.0, "unit": "mm"}),
    # c_m + c_a rounds to t while (t - c_m) - c_a stays positive: the section sum decides.
    "the section sum at the rounding boundary": lambda s: s.update(corrosion_allowance={"value": 0.0029999999999999996, "unit": "m"}, mill_tolerance={"value": 0.007, "unit": "m"}),
}
CA_ADMITTED = {
    "unchanged": lambda s: None,
    "zero in mm": lambda s: s.update(corrosion_allowance={"value": 0.0, "unit": "mm"}),
}
# O4 (§3.4): zero is no longer required. An admitted nonzero allowance passes the allowance check; the statement's
# sections carry 0 m, so it is refused later by the by-value section binding (raw reading of the 0.3 statement;
# never eligible on a load/reference state, whose reader verdict refuses it by G2's model check).
CA_NONZERO = {
    "nonzero 0.001 m": lambda s: s.update(corrosion_allowance={"value": 0.001, "unit": "m"}),
    "1 mm beside a 0.5 mm mill tolerance": lambda s: s.update(corrosion_allowance={"value": 1.0, "unit": "mm"}, mill_tolerance={"value": 0.5, "unit": "mm"}),
}
CA_FAMILIES = {"0.3": ("physics_source", True), "load-reference-state": ("load_reference_source", False)}


def _ca_statement(family, name, mode, change):
    """A genuine pressure-source-1 statement resealed to an invocation whose last pipe section's allowance is changed:
    the receipt's invocation digest and receipt hash follow the edited invocation, so only the allowance differs."""
    folder = ROOT / "fixtures/product_preview/pressure_source" / CA_FAMILIES[family][0]
    source = json.loads((folder / f"{name}-{mode}.raw.json").read_text())
    captured = {"request": json.loads((folder / f"{name}.request.json").read_text()), "solver_mode": mode}
    change(captured["request"]["model"]["pipe_segments"][-1]["section"])
    body = source["source_block_recovery"]["body"]
    body["invocation"]["value"] = blocks.domain_hash("source_blocks_invocation_v1", captured)
    source["source_block_recovery"]["receipt_sha256"] = blocks.domain_hash("source_blocks_receipt_v1", body)
    return source, captured


@pytest.mark.parametrize("label", sorted(CA_REFUSED) + sorted(CA_ADMITTED) + sorted(CA_NONZERO))
@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
@pytest.mark.parametrize("name", ["n05", "n06"])
@pytest.mark.parametrize("family", sorted(CA_FAMILIES))
def test_the_captured_exact_document_carries_an_admitted_corrosion_allowance(family, name, mode, label):
    """B7b review finding (b7-plan "Exact-contract fields") and O4 (§3.4): a resealed pressure-source-1 statement whose
    captured invocation's allowance was absent, null, non-length, negative or over the wall is refused first by Rust
    `SourceContract::check_invocation` and the verifier, PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE; zero in any length
    unit keeps the statement's own standing (eligible on 0.3, never on a load/reference state). An admitted nonzero
    allowance passes that check and is refused by the statement's by-value section binding instead."""
    source, captured = _ca_statement(family, name, mode, {**CA_REFUSED, **CA_ADMITTED, **CA_NONZERO}[label])
    refs = [case["basis_ref"] for case in source["numerical_quality"]["cases"]]
    if label in CA_NONZERO:
        if CA_FAMILIES[family][1]:
            with pytest.raises(ValueError, match="^PHYSICS_SOURCE_ACTUAL_SOURCE_GEOMETRY$"):
                ps.validate_physics_source(deepcopy(source), deepcopy(captured))
        else:
            assert ps.validate_physics_source(deepcopy(source), deepcopy(captured)) is False
        with pytest.raises(rust_reader.ReaderRefusal) as refused:
            rust_reader.validate("pressure-source", source, invocation=captured)
        assert refused.value.code == ("PHYSICS_SOURCE_ACTUAL_SOURCE_GEOMETRY" if CA_FAMILIES[family][1] else "SOURCE_PHYSICS_MODEL_SECTION_GEOMETRY")
    elif label in CA_REFUSED:
        with pytest.raises(ValueError, match="^PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE$"):
            ps.validate_physics_source(deepcopy(source), deepcopy(captured))
        with pytest.raises(rust_reader.ReaderRefusal) as refused:
            rust_reader.validate("pressure-source", source, invocation=captured)
        assert refused.value.code == "PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE"
        # The allowance binds only the invocation: without one the statement still reads, never eligible.
        assert ps.validate_physics_source(deepcopy(source)) is False
    else:
        eligible = CA_FAMILIES[family][1]
        assert ps.validate_physics_source(deepcopy(source), deepcopy(captured)) is eligible
        assert rust_reader.validate("pressure-source", source, invocation=captured)["eligible"] is eligible
    assert c.numerical_use_standing(deepcopy(source), refs, deepcopy(captured)) == rust_reader.standing(source, refs, captured)["token"]


def _ca_second_pipe_statement(family, name, mode, change):
    """A genuine pressure-source-1 statement resealed to an invocation with a second pipe segment appended to its model
    (a copy of the first under a new id, its section changed by `change`); the first pipe keeps its explicit 0 m."""
    folder = ROOT / "fixtures/product_preview/pressure_source" / CA_FAMILIES[family][0]
    source = json.loads((folder / f"{name}-{mode}.raw.json").read_text())
    captured = {"request": json.loads((folder / f"{name}.request.json").read_text()), "solver_mode": mode}
    pipes = captured["request"]["model"]["pipe_segments"]
    assert len(pipes) == 1, "every committed pressure-source-1 request has one pipe segment"
    second = deepcopy(pipes[0])
    second["id"] = f"{second['id']}-second"
    change(second["section"])
    pipes.append(second)
    body = source["source_block_recovery"]["body"]
    body["invocation"]["value"] = blocks.domain_hash("source_blocks_invocation_v1", captured)
    source["source_block_recovery"]["receipt_sha256"] = blocks.domain_hash("source_blocks_receipt_v1", body)
    return source, captured


CA_SECOND_ZERO = "zero in m"


@pytest.mark.parametrize("label", sorted(CA_REFUSED) + [CA_SECOND_ZERO] + sorted(CA_NONZERO))
@pytest.mark.parametrize("mode", ["sparse_interactive", "dense_scrutiny"])
@pytest.mark.parametrize("name", ["n05", "n06"])
@pytest.mark.parametrize("family", sorted(CA_FAMILIES))
def test_every_captured_pipe_section_carries_an_admitted_allowance(family, name, mode, label):
    """B7b review N-1: every committed request has one pipe, so the controls above cannot tell an every-section check
    from a first-section one. With a second pipe appended (the first keeps 0 m) whose allowance is absent, null,
    non-length, malformed, negative or over the wall, both readers refuse PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE (the
    check is the first step after the captured contract). With 0 m or (O4 §3.4) an admitted nonzero allowance on the
    second pipe the allowance check passes and both refuse the statement later, by the receipt's coverage of the
    captured model (SOURCE_BLOCKS_CURRENT_MODEL_SOURCE_COVERAGE: its sources name one member), never for the allowance;
    a first-section-only check would give that code throughout."""
    change = {**CA_REFUSED, **CA_NONZERO}.get(label, lambda s: s.update(corrosion_allowance={"value": 0.0, "unit": "m"}))
    source, captured = _ca_second_pipe_statement(family, name, mode, change)
    assert captured["request"]["model"]["pipe_segments"][0]["section"]["corrosion_allowance"] == {"value": 0.0, "unit": "m"}
    expected = "PHYSICS_SOURCE_ACTUAL_CORROSION_ALLOWANCE" if label in CA_REFUSED else "SOURCE_BLOCKS_CURRENT_MODEL_SOURCE_COVERAGE"
    with pytest.raises(ValueError) as python:
        ps.validate_physics_source(deepcopy(source), deepcopy(captured))
    with pytest.raises(rust_reader.ReaderRefusal) as rust:
        rust_reader.validate("pressure-source", source, invocation=captured)
    assert (str(python.value), rust.value.code) == (expected, expected)
    assert ps.validate_physics_source(deepcopy(source)) is False
    refs = [case["basis_ref"] for case in source["numerical_quality"]["cases"]]
    standing = c.numerical_use_standing(deepcopy(source), refs, deepcopy(captured))
    assert standing != "numerically_eligible" and standing == rust_reader.standing(source, refs, captured)["token"]


@pytest.mark.parametrize("path",["pressure_source/load_reference_source/n05-sparse_interactive.raw.json",
                                  "pressure/load_reference/connected-sparse_interactive.raw.json"])
def test_a_load_reference_state_statement_owns_no_carrier_namespace(path):
    """Rust `current_header` (the retired load-reference routes' guard, kept on the current group): after the identity
    allowlist and before the recovery downgrade check, a statement with load_reference_states and a carrier_evidence
    member is refused, in both header orders; the route refuses any carrier first, in both readers."""
    source = json.loads((ROOT / "fixtures/product_preview" / path).read_text())
    source["carrier_evidence"] = {}
    for order in (False, True):
        with pytest.raises(ValueError, match="^SOURCE_PRODUCER_CONTRACT_UNSUPPORTED$"):
            c._source_contract(deepcopy(source), check_receipt=not order, rust_header_order=order)
    with pytest.raises(verdict.Refused, match="^SOURCE_PRODUCER_CONTRACT_UNSUPPORTED$"):
        verdict._metadata_header(deepcopy(source))
    for dispatch in (verdict.dispatch(deepcopy(source)), rust_reader.dispatch(deepcopy(source), None)):
        assert dispatch["error"]["code"] == "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED"


@pytest.mark.parametrize("evidence", ["removed", None, [], "x"])
@pytest.mark.parametrize("folder", ["physics_source", "load_reference_source"])
def test_the_pressure_source_route_needs_its_evidence_object_before_its_receipt(folder, evidence):
    """Rust `reader_verdict::route`: as the retired load-reference-source-1 route did, the composite needs its
    evidence object before its receipt is read (SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED), in both readers."""
    source = json.loads((ROOT / "fixtures/product_preview/pressure_source" / folder / "n05-sparse_interactive.raw.json").read_text())
    if evidence == "removed":
        del source["contract_evidence"]
    else:
        source["contract_evidence"] = evidence
    with pytest.raises(verdict.Refused, match="^SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED$"):
        verdict.route(deepcopy(source))
    for dispatch in (verdict.dispatch(deepcopy(source)), rust_reader.dispatch(deepcopy(source), None)):
        assert (dispatch["route"], dispatch["error"]["code"]) == ("unsupported", "SOURCE_NUMERICAL_CONTRACT_UNSUPPORTED")


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
