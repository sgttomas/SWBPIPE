#!/usr/bin/env python3
"""Run the candidate-bound seven-case composition and independent verification."""
from __future__ import annotations
import argparse
from hashlib import sha256
import json
import os
from pathlib import Path
import subprocess
import sys

CASES = ["modulus", "combination_modulus", "curved", "friction", "hanger_constant_effort", "rotational_nonlinear", "zero_pressure_longitudinal"]

def run(argv: list[str], *, cwd: Path, env: dict[str, str] | None = None) -> None:
    completed = subprocess.run(argv, cwd=cwd, env=env, check=False, shell=False)
    if completed.returncode != 0:
        raise SystemExit(f"COMMAND_FAILED:{completed.returncode}:{json.dumps(argv)}")

p = argparse.ArgumentParser()
p.add_argument("--working-root", required=True, type=Path)
p.add_argument("--checked-json-bin", required=True, type=Path)
p.add_argument("--case-map", required=True, type=Path)
p.add_argument("--output-root", required=True, type=Path)
a = p.parse_args()
root, checked, case_map_path, output_root = a.working_root.resolve(), a.checked_json_bin.resolve(), a.case_map.resolve(), a.output_root.resolve()
v4 = Path(__file__).resolve().parent
case_map = json.loads(case_map_path.read_text(encoding="utf-8"))
if sorted(case_map.get("cases", {})) != sorted(CASES):
    raise SystemExit("CASE_MAP_EXACT_SEVEN_REQUIRED")
output_root.mkdir(parents=True, exist_ok=True)
env = dict(os.environ)
env.update({"FINAL_WORKING_ROOT": str(root), "FINAL_CONSUMER_CASE_MAP": str(case_map_path), "FINAL_CONSUMER_OUTPUT_DIR": str(output_root), "OPENPIPESTRESS_CHECKED_JSON_BIN": str(checked)})
run(["npm", "exec", "--workspace", "apps/desktop", "--", "vitest", "run", "--config", str(v4 / "vitest.config.ts")], cwd=root, env=env)
reports = []
for name in CASES:
    case_dir = output_root / name
    members = case_dir / "stress_members"
    report = case_dir / "independent_verification.json"
    run([sys.executable, str(v4 / "materialize_stress_members.py"), "--working-root", str(root), "--checked-json-bin", str(checked), "--packet", str(case_dir / "stress_neutral_v0_2.json"), "--output-dir", str(members)], cwd=root, env=env)
    spec = case_map["cases"][name]
    run([sys.executable, str(v4 / "verify_delivered_artifacts.py"), "--working-root", str(root), "--checked-json-bin", str(checked), "--case-dir", str(case_dir), "--source-request", spec["request_path"], "--source-output", spec["output_path"], "--stress-members", str(members), "--composition-receipt", str(case_dir / "composition_receipt.json"), "--report", str(report)], cwd=root, env=env)
    reports.append({"case": name, "report": str(report), "sha256": sha256(report.read_bytes()).hexdigest()})
index = output_root / "independent_verification_index.json"
index.write_text(json.dumps({"status": "PASS", "cases": reports}, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(json.dumps({"status": "PASS", "index": str(index), "sha256": sha256(index.read_bytes()).hexdigest()}, sort_keys=True))
