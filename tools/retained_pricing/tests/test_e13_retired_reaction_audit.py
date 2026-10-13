"""E13 removes exactly the retired reaction audit under the full source guard."""
import ast
from pathlib import Path
import runpy

import pytest

PRICING = Path(__file__).resolve().parents[1]
PIPING = PRICING.parents[1]
MODEL = runpy.run_path(str(PRICING / "base/kd5_text_model.py"))
KEY = "core/product_physics/src/lib.rs:13085"
VALUE = {"row.id": {"bytes": 1024, "source": "RES"}}


def original_common_audits():
    source = (PRICING / "rules/sq2_decisions_common.py").read_text()
    namespace = {}
    for node in ast.parse(source).body:
        if not isinstance(node, ast.Assign):
            continue
        names = {name.id for target in node.targets for name in ast.walk(target) if isinstance(name, ast.Name)}
        if names & {"IN", "P_", "A_", "ID_AUDIT"}:
            exec(ast.get_source_segment(source, node), namespace)
            if "ID_AUDIT" in names:
                return namespace["ID_AUDIT"]
    raise AssertionError("original common ID audit declaration missing")


def test_only_one_reaction_decision_leaves_all_other_canonical_audits_unchanged():
    original = original_common_audits()
    assert original.count((KEY, VALUE)) == 1
    current = MODEL["retire_combination_reaction_audit"](PIPING, original)
    expected = list(original)
    expected.remove((KEY, VALUE))
    assert current == expected and len(current) == len(original) - 1
    assert original.count((KEY, VALUE)) == 1
    # Current component/source/reference/combination owners keep their positive classes.
    for key in ("lib.rs:13054", "lib.rs:13070", "lib.rs:13018", "lib.rs:13094"):
        entry = next(a for a in original if a[0] == "core/product_physics/src/" + key)
        assert entry in current


@pytest.mark.parametrize("drift", ["missing", "duplicate", "value"])
def test_changed_exact_decision_preimage_refuses(drift):
    audits = original_common_audits()
    if drift == "missing":
        audits.remove((KEY, VALUE))
    elif drift == "duplicate":
        audits.append((KEY, VALUE))
    else:
        index = audits.index((KEY, VALUE))
        audits[index] = (KEY, {"row.id": {"bytes": 128, "source": "IN128"}})
    with pytest.raises(ValueError, match="preimage changed"):
        MODEL["retire_combination_reaction_audit"](PIPING, audits)


def test_a_restored_reaction_row_clone_refuses_source_binding(tmp_path):
    file = tmp_path / "core/product_physics/src/lib.rs"
    file.parent.mkdir(parents=True)
    raw = (PIPING / "core/product_physics/src/lib.rs").read_bytes()
    start = raw.index(b"fn append_combined_vector_magnitude(")
    site = raw.index(b"    let mut combined = reference.clone();", start)
    restored = b"    let row = reference;\n    source_refs.push(row.id.clone());\n"
    file.write_bytes(raw[:site] + restored + raw[site:])
    with pytest.raises(ValueError, match="full-file source changed"):
        MODEL["retire_combination_reaction_audit"](tmp_path, original_common_audits())
