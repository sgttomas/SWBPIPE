"""U6-RL (T4-U6/U8 D3 review R2): route E's census counts each source-selected extremum at the componentwise maximum
over the three `locations` shapes composite.rs builds, whichever shape the fixture shows; an unreviewed shape or a
changed producer stops the census."""
from pathlib import Path
import json
import shutil
import subprocess
import sys

import pytest

PRICING = Path(__file__).resolve().parents[1]
SOURCE = PRICING.parents[1]
CENSUS = PRICING / "lib/census_exact.py"
FX = "fixtures/product_preview/pressure_source/physics_source"
COMPOSITE = "core/product_physics/src/source_receipt/composite.rs"
MODES = ("sparse_interactive", "dense_scrutiny")
SHAPES = {
    "strict_endpoint": {"endpoint": "j", "kind": "strict_endpoint"},
    "whole_span_constant": {"kind": "whole_span_constant"},
    "endpoint_candidates": {"exact_tie_proven": True, "interior_equal_possible": False, "kind": "endpoint_candidates"},
}


def snapshot(tmp_path, shape=None, composite=None):
    root = tmp_path / "p"
    (root / FX).mkdir(parents=True)
    shutil.copyfile(SOURCE / FX / "fields.request.json", root / FX / "fields.request.json")
    for mode in MODES:
        raw = json.loads((SOURCE / FX / f"fields-{mode}.raw.json").read_text())
        if shape is not None:
            for case in raw["contract_evidence"]["exact_cases"]:
                for extremum in case["pipe_stress_extrema"]:
                    extremum["locations"] = shape
        (root / FX / f"fields-{mode}.raw.json").write_text(json.dumps(raw))
    (root / COMPOSITE).parent.mkdir(parents=True)
    text = (SOURCE / COMPOSITE).read_text()
    if composite is not None:
        assert text.count(composite[0]) == 1
        text = text.replace(*composite)
    (root / COMPOSITE).write_text(text)
    return root


def census(root, tmp_path):
    out = tmp_path / "census.json"
    done = subprocess.run([sys.executable, "-I", str(CENSUS), str(root), str(out)], capture_output=True, text=True)
    return done, (json.loads(out.read_text()) if done.returncode == 0 else None)


def test_each_extremum_is_counted_at_its_largest_locations_shape(tmp_path):
    done, data = census(snapshot(tmp_path), tmp_path)
    assert done.returncode == 0, done.stderr
    shapes = data["locations_shapes"]
    assert {k: (v["ent"], v["keyb"], v["strb"]) for k, v in shapes.items()} == {
        "strict_endpoint": (2, 12, 608), "whole_span_constant": (1, 4, 304), "endpoint_candidates": (3, 43, 304)}
    extrema = data["exact_evidence_case_parts"]["per_pipe_by_array"]["pipe_stress_extrema"]
    assert (extrema["ent"], extrema["keyb"]) == (92, 1042)       # two extrema, each 45 + 1 entries and 490 + 31 B
    per_pipe = data["exact_evidence_case_parts"]["per_pipe"]
    assert (per_pipe["ent"], per_pipe["keyb"]) == (127, 1348)
    caps = data["exact_evidence_per_case_at_caps"]
    assert (caps["ent"], caps["keyb"]) == (29 + 127 * 32, 495 + 1348 * 32)


@pytest.mark.parametrize("shape", sorted(SHAPES))
def test_the_census_does_not_depend_on_the_fixture_shape(tmp_path, shape):
    base_done, base = census(snapshot(tmp_path / "a"), tmp_path / "a")
    done, data = census(snapshot(tmp_path / "b", SHAPES[shape]), tmp_path / "b")
    assert base_done.returncode == 0 and done.returncode == 0, done.stderr
    assert data["exact_evidence_case_parts"] == base["exact_evidence_case_parts"]
    assert data["componentwise_max"] == base["componentwise_max"]


def test_the_committed_census_carries_the_locations_maximum():
    data = json.loads((PRICING / "data/census_d5_79bf8656dc.json").read_text())
    per_pipe = data["exact_evidence_case_parts"]["per_pipe"]
    assert data["rows_per_member"] == 72 and (per_pipe["ent"], per_pipe["keyb"]) == (127, 1348)
    assert set(data["locations_shapes"]) == set(SHAPES)


def test_an_unreviewed_locations_shape_stops(tmp_path):
    done, _ = census(snapshot(tmp_path, {"kind": "interior_maximum", "station": 0.5}), tmp_path)
    assert done.returncode != 0 and "unreviewed locations shape" in done.stderr + done.stdout


@pytest.mark.parametrize("old,new", [
    ('json!({"kind":"whole_span_constant"})', 'json!({"kind":"whole_span_constant","station":0.5})'),
    ('"exact_tie_proven":exact_tie_proven,', '"exact_tie_proven":exact_tie_proven,"witness":"i",'),
])
def test_a_changed_producer_shape_stops(tmp_path, old, new):
    done, _ = census(snapshot(tmp_path, composite=(old, new)), tmp_path)
    assert done.returncode != 0 and "U6-RL" in done.stderr + done.stdout
