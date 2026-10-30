"""tools/serialization/semantic_contract_tables.py: the derived table members are current, and `repin` scopes its
edits (frozen references and historical records are never edited; an old value found only there is refused; the
registered retained-memory profile's reviewed_inputs record is never edited)."""
import importlib.util
import sys
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]


def _tool():
    spec = importlib.util.spec_from_file_location("semantic_contract_tables", PROJECT / "tools/serialization/semantic_contract_tables.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules["semantic_contract_tables"] = module
    spec.loader.exec_module(module)
    return module


def test_derived_members_are_current():
    assert _tool().derive(check=True) == 0


def test_repin_never_edits_frozen_or_historical_trees():
    tool = _tool()
    editable, carriers, excluded = tool.classify([
        "core/reporting/result_export/src/semantic_contract.rs",
        "fixtures/results/pressure_load_state_connected_dense.analysis_run.json",
        "validation/references/t4_u5b/SHA256SUMS",
        "execution/PKG-04/DEL-04-07/Work/u4b-u5b-implementation.md",
        "tools/retained_pricing/base/text_args.g4.json",
    ])
    assert editable == ["core/reporting/result_export/src/semantic_contract.rs"]
    assert carriers == ["fixtures/results/pressure_load_state_connected_dense.analysis_run.json"]
    assert excluded == ["validation/references/t4_u5b/SHA256SUMS", "execution/PKG-04/DEL-04-07/Work/u4b-u5b-implementation.md",
                        "tools/retained_pricing/base/text_args.g4.json"]


def test_repin_routes_the_desktop_run_carriers_to_their_writers():
    tool = _tool()
    carriers = [
        "fixtures/persistence/recorded_run_witness.json",
        "fixtures/persistence/recorded_run_witness_rule_packs.json",
        "fixtures/persistence/s2_unresolved_rule_pack_run.json",
        "fixtures/persistence/transient_run_input.json",
        "fixtures/persistence/transient_run_input_rule_packs.json",
    ]
    others = ["fixtures/persistence/invented_persisted_preview_project.json",
              "fixtures/persistence/model_state_record_witness.json"]
    editable, found, excluded = tool.classify(carriers + others)
    assert found == carriers and editable == others and excluded == []
    for carrier in carriers:
        writers = [w for prefix, w in tool.CARRIER_WRITERS.items() if carrier.startswith(prefix)]
        assert writers == [tool.PERSISTENCE_RUN_WRITERS], carrier
        assert carrier in writers[0], carrier
    # The witnesses are emitted from the TypeScript-built inputs, so those writers come first.
    order = tool.PERSISTENCE_RUN_WRITERS
    assert order.index("recordedRunParity") < order.index("analysis_runs_tests")


def test_repin_dry_run_at_head_lists_nothing(capsys):
    assert _tool().repin(["HEAD"], dry_run=True) == 0
    assert "0 file edit(s) listed, none written" in capsys.readouterr().out


def test_repin_leaves_the_registered_profile_record_unchanged():
    tool = _tool()
    old, new = "a" * 64, "b" * 64
    text = f'pin: "{old}",\n    reviewed_inputs: "v1;Cargo.lock={old};x={old}",\n    other: "{old}"\n'
    updated, kept = tool.replace_unprotected("core/product_physics/src/retained_memory.rs", text, old, new)
    assert kept == 2
    assert updated == f'pin: "{new}",\n    reviewed_inputs: "v1;Cargo.lock={old};x={old}",\n    other: "{new}"\n'
    assert tool.replace_unprotected("core/product_physics/src/lib.rs", text, old, new) == (text.replace(old, new), 0)
    # The actual record is matched exactly once.
    source = (PROJECT / "core/product_physics/src/retained_memory.rs").read_text(encoding="utf-8")
    assert len(tool.PROTECTED["core/product_physics/src/retained_memory.rs"].findall(source)) == 1


def test_timoshenko_table_is_preview_physics_2_with_its_identity_profile_and_limitation_0():
    """DEL-05-03 Design pf-timoshenko.md §4: the derived table differs from preview-physics-2 in exactly three
    members (identity, profile, limitation 0), member order kept; `derive --check` holds it current."""
    import json
    tool = _tool()
    preview = json.loads((PROJECT / tool.PREVIEW).read_bytes())
    derived = json.loads((PROJECT / tool.TIMOSHENKO).read_bytes())
    assert list(derived) == list(preview)
    assert {k for k in preview if preview[k] != derived[k]} == {
        "semantic_contract_id", "formulation_profile_id", "supported_profile_limitations"}
    assert derived["semantic_contract_id"] == "openpipestress.result_semantics/0.3.0/preview-physics-timoshenko-1"
    assert derived["formulation_profile_id"] == "product_preview_mechanics_timoshenko_v1"
    assert derived["supported_profile_limitations"] == [tool.TIMOSHENKO_LIMITATION_0] + preview["supported_profile_limitations"][1:]
    assert (PROJECT / tool.TIMOSHENKO).read_bytes() == tool.timoshenko_table()
    # The results schema's Timoshenko branch states the same array (apply-u4b keeps it current).
    assert tool.apply_u4b(check=True) == 0
    schema = json.loads((PROJECT / "schemas/results.v0.3.schema.yaml").read_text(encoding="utf-8"))
    branch = next(b for b in schema["$defs"]["ResultEnvelope"]["oneOf"]
                  if b.get("properties", {}).get("producer", {}).get("properties", {}).get("semantic_contract_id", {}).get("const")
                  == derived["semantic_contract_id"])
    assert branch["properties"]["formulation_basis"]["properties"]["profile_id"]["const"] == derived["formulation_profile_id"]
    assert branch["properties"]["formulation_basis"]["properties"]["limitations"]["const"] == derived["supported_profile_limitations"]
