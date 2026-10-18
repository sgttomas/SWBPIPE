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
        "execution/PKG-04/DEL-04-07/Design/u4b-u5b-implementation.md",
    ])
    assert editable == ["core/reporting/result_export/src/semantic_contract.rs"]
    assert carriers == ["fixtures/results/pressure_load_state_connected_dense.analysis_run.json"]
    assert excluded == ["validation/references/t4_u5b/SHA256SUMS", "execution/PKG-04/DEL-04-07/Design/u4b-u5b-implementation.md"]


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
