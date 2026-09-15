#!/usr/bin/env python3
"""Thin execution-time call into the owning product materializer."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import sys

p = argparse.ArgumentParser()
p.add_argument("--working-root", required=True, type=Path)
p.add_argument("--checked-json-bin", required=True, type=Path)
p.add_argument("--packet", required=True, type=Path)
p.add_argument("--output-dir", required=True, type=Path)
a = p.parse_args()
root = a.working_root.resolve()
checked = a.checked_json_bin.resolve()
if not checked.is_file():
    raise SystemExit(f"CHECKED-JSON-AUTHORITY-MISSING: {checked}")
os.environ["OPENPIPESTRESS_CHECKED_JSON_BIN"] = str(checked)
sys.path.insert(0, str(root))
from core.handoff.stress_neutral import write_materialized_members_v0_2  # noqa: E402

packet = json.loads(a.packet.read_text(encoding="utf-8"))
written = write_materialized_members_v0_2(packet, a.output_dir)
print(json.dumps({"status": "PASS", "owning_api": "core.handoff.stress_neutral.write_materialized_members_v0_2", "members": [str(item.resolve()) for item in written]}, sort_keys=True))
