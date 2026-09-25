"""Replay the recorded stdin probe; annotation transport only, not full admission."""
from pathlib import Path
import hashlib
import json
import struct
import sys

OUT = Path(__file__).resolve().parent
ROOT = next(path for path in OUT.parents if (path / "core/analysis_runs/physics_source.py").is_file())
sys.path.insert(0, str(ROOT))
from core.handoff.stress_neutral import package_v0_3 as sn

bits = lambda value: struct.pack(">d", float(value)).hex()
results = []
for mode in ["sparse_interactive", "dense_scrutiny"]:
    path = ROOT / "fixtures/product_preview/physics_source" / f"mixed-{mode}.raw.json"
    raw = json.loads(path.read_text())
    before = sn._source_annotations(raw)
    after = json.loads(sn.canonical_json_checked_v1(before))
    expected = {row["id"]: bits(row["value"]) for row in raw["results"]}
    for annotation in after:
        sn._validate_source_value_bits(annotation)
        assert annotation["source_value_bits"] == expected[annotation["source_result_id"]]
        if annotation["source_row"]["value"] == 0:
            assert bits(annotation["source_row"]["value"]) == "0000000000000000"
    results.append({"mode": mode, "rows": len(after), "source_sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
        "negative_zeros": sum(value == "8000000000000000" for value in expected.values()),
        "positive_zeros": sum(value == "0000000000000000" for value in expected.values()),
        "annotation_hash_before": sn.canonical_sha256_checked_v1(before),
        "annotation_hash_after": sn.canonical_sha256_checked_v1(after)})
print(json.dumps({"scope": "Actual annotation transport only; no full validation or Current claim", "results": results}, indent=2))
