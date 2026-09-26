"""Digest the existing-method Python carrier outputs for a before/after comparison.

Run with cwd = a project root (the HEAD mirror for "before", the worktree for
"after"). For every pre-existing raw fixture it builds the AnalysisRun 0.3
record and the stress-neutral 0.3 package exactly as the existing tests do,
and prints sha256 digests of their compact JSON bytes. Read-only.
Usage: python existing_outputs_digest.py <out.json>
"""
import glob
import hashlib
import json
import sys
from pathlib import Path

PROJECT = Path.cwd()
sys.path.insert(0, str(PROJECT))
sys.path.insert(0, str(PROJECT / "tests"))

from core.analysis_runs.compatibility import _source_contract, build_analysis_run_v0_3  # noqa: E402
from core.handoff.stress_neutral import package_v0_3 as sn  # noqa: E402
from test_stress_neutral_physics_source import arguments  # noqa: E402
from test_stress_neutral_precision import prepare as prepare_precision  # noqa: E402


def digest(value):
    return hashlib.sha256(json.dumps(value, separators=(",", ":"), ensure_ascii=False, sort_keys=False).encode()).hexdigest()


paths = sorted(
    glob.glob("fixtures/results/physics_*.json")
    + glob.glob("fixtures/results/precision_*_mechanics_*.json")
    + glob.glob("fixtures/product_preview/source_blocks/ui/*.raw.json")
    + glob.glob("fixtures/product_preview/physics_source/*.raw.json")
)
out = {}
for path in paths:
    raw = json.loads(Path(path).read_text())
    entry = {}
    try:
        entry["contract"] = _source_contract(raw)[0]
        analysis = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": "manifest:pure-projection-test"}, input_manifest_hash="1" * 64)
        entry["analysis_run_sha256"] = digest(analysis)
        packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=raw, analysis_record=analysis, **arguments(raw, analysis))
        entry["stress_neutral_sha256"] = digest(packet)
        entry["stress_neutral_package_checksum"] = packet["package_checksum"]["value"]
    except Exception as error:  # recorded, compared before/after
        entry["error"] = f"{type(error).__name__}: {error}"
    out[path] = entry
raw, analysis, args = prepare_precision()
packet = sn.build_stress_neutral_export_package_v0_3(source_envelope=raw, analysis_record=analysis, **args)
out["test_stress_neutral_precision.prepare()"] = {"analysis_run_sha256": digest(analysis), "stress_neutral_sha256": digest(packet), "stress_neutral_package_checksum": packet["package_checksum"]["value"]}
Path(sys.argv[1]).write_text(json.dumps(out, indent=1, sort_keys=True) + "\n")
print(json.dumps({"entries": len(out), "errors": sum("error" in v for v in out.values())}))
