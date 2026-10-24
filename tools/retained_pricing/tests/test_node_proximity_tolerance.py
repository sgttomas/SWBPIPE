"""W8-BF, NP-MS and NP-PT (retained-pricing-rules.md §8.18): the branch warning's applied-factors statement, added
after publication over the model's components, stays at multiplicity 0 under W8-BT's premise with its texts bounded, the node-proximity tolerance's refusal and the accessor's
five texts are priced positively at their audited bytes, and a changed text or call stops the run."""
from pathlib import Path
import runpy
import shutil

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
VALIDATION = "core/product_physics/src/validation.rs"
SETTINGS = "core/product_physics/src/model_settings.rs"
IN = {"bytes": 128, "source": "IN128"}


def decisions(monkeypatch, snapshot=SOURCE):
    monkeypatch.setenv("SQ2_SNAP", str(snapshot))
    return runpy.run_path(str(PRICING / "rules/sq2_decisions_E.py"))


def line_of(path, text):
    hits = [i + 1 for i, line in enumerate((SOURCE / path).read_text(encoding="utf-8").split("\n")) if text in line]
    assert len(hits) == 1, (text, hits)
    return hits[0]


def audits(d):
    """The ID audits per site, merged as lib/sq2_rules.py merges them (one site may carry several rules' keys)."""
    merged = {}
    for key, value in d["O10_ID_AUDIT"]:
        merged.setdefault(str(key), {}).update(value)
    return merged


def snapshot_with(tmp_path, path, old, new):
    shutil.copytree(SOURCE / "core", tmp_path / "core", ignore=shutil.ignore_patterns("target"))
    target = tmp_path / path
    text = target.read_text(encoding="utf-8")
    assert text.count(old) == 1
    target.write_text(text.replace(old, new), encoding="utf-8")
    return tmp_path


def test_the_tolerance_refusal_is_priced_at_its_audited_bytes(monkeypatch):
    d = decisions(monkeypatch)
    audit = audits(d)
    path = len("model_settings.node_proximity_tolerance")
    for fn, has_unit in (("tolerance_not_finite_message", False), ("tolerance_not_positive_message", False),
                         ("tolerance_unit_not_length_message", True), ("tolerance_zero_in_metres_message", True),
                         ("tolerance_not_finite_in_metres_message", True)):
        line = line_of(SETTINGS, f"pub fn {fn}(") + 1
        entry = audit[f"{SETTINGS}:{line}"]
        assert entry["NODE_PROXIMITY_TOLERANCE_PATH"]["bytes"] == path
        assert ("unit" in entry) is has_unit and entry.get("unit", IN) == IN
    refs = line_of(VALIDATION, "vec![crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH.to_string()],")
    assert audit[f"{VALIDATION}:{refs}"]["crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH"]["bytes"] == path
    # The refusal is not zeroed or guarded at zero: D1 admits model_settings, so it keeps its multiplicity.
    diag = line_of(VALIDATION, '"diagnostic:model-settings:node-proximity-tolerance",')
    assert not any(str(k) == f"validation.rs:{diag - 1}" for k, _ in d["O10_MULT0_SITES"])
    assert not any(str(k).startswith("validation.rs:") and str(k).endswith(f":{diag - 1}") for k, _ in d.get("SITE_ZERO", []))


def test_the_applied_factors_statement_is_guarded_at_zero_and_bounded(monkeypatch):
    d = decisions(monkeypatch)
    side = line_of(VALIDATION, 'let named = format!("{side} SIF to {}", record["pipe_id"].as_str().unwrap_or_default());')
    join = line_of(VALIDATION, 'let sides = applied.join(", ");')
    statement = line_of(VALIDATION, 'warning.message.push_str(&format!("; its branch stress factors are still applied ({sides})"));')
    mult0 = [str(k) for k, why in d["O10_MULT0_SITES"] if "W8-BF" in why]
    assert mult0 == [f"validation.rs:{side}", f"validation.rs:{statement}"]
    loops = [(h, b, fns) for h, b, why, fns in d["O10_LOOPS_FIRST"] if "W8-BF" in why]
    assert loops == [("for component in model_components", "0", ["validation.rs:state_applied_branch_factors"])]
    audit = audits(d)
    assert audit[f"{VALIDATION}:{side}"]["side"]["bytes"] == 6
    assert audit[f"{VALIDATION}:{side}"]['record["pipe_id"].as_str().unwrap_or_default()'] == IN
    assert audit[f"{VALIDATION}:{statement}"]["sides"]["bytes"] == 286
    lex = {str(k): v["class"] for k, v in d["LEX_SITE_SIZE"] if "W8-BF" in v["why"]}
    assert lex == {f"validation.rs:{join}": 286, f"validation.rs:{statement}": 334}
    message = line_of(VALIDATION, "joins {degree}\",") - 1
    assert audit[f"{VALIDATION}:{message}"] == {"degree": {"bytes": 20, "source": "O-10 (T4): W8-BT: a usize member count"}}


@pytest.mark.parametrize("path, old, new, match", [
    (SETTINGS, ".value is {value}; the tolerance must be greater than zero.",
     ".value is {value} (as authored); the tolerance must be greater than zero.", "NP-MS"),
    (VALIDATION, "vec![crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH.to_string()],",
     "vec![crate::model_settings::NODE_PROXIMITY_TOLERANCE_PATH.to_string(), error.message.clone()],", "NP-MS"),
    (VALIDATION, "                    applied.push(named);\n",
     "                    applied.push(named.clone());\n                    applied.push(named);\n", "W8-BF"),
    (VALIDATION, "    for component in model_components {\n        let Some(warning)",
     "    let _all = model_components.to_vec();\n    for component in model_components {\n        let Some(warning)", "W8-BF"),
    (VALIDATION, "            warning.message.push_str(&format!(\"; its branch stress factors are still applied ({sides})\"));\n        }\n    }\n",
     "            warning.message.push_str(&format!(\"; its branch stress factors are still applied ({sides})\"));\n        }\n    }\n    let _all = diagnostics.to_vec();\n", "W8-BF"),
])
def test_a_changed_text_or_call_stops(monkeypatch, tmp_path, path, old, new, match):
    with pytest.raises(SystemExit, match=match):
        decisions(monkeypatch, snapshot_with(tmp_path, path, old, new))
