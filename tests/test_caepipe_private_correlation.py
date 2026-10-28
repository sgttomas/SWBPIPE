#!/usr/bin/env python3
"""Private CAEPIPE correlation regression check (DEL-17-05 Design §9).

Reads the private corpus through SWBPIPE_CAEPIPE_EXCHANGE. Skips, with the
reason, when the harness is not configured, when no case holds a stored
comparison record and CAEPIPE CSV, or when no runner executable is configured
(OPENPIPESTRESS_RUNNER_BIN). Otherwise each stored case is re-run offline in a
temporary copy (the private corpus is never written): the stored CSV is
re-parsed, the case is re-solved in SWBPIPE and the comparison is rebuilt, then
the series step evaluates the hypothesis series and rebuilds the declaring
cases, as the pilot command does. The check fails when any quantity's outcome,
cause or cause evidence status moves from its recorded classification. Failure
messages name case, quantity and check ids only, never values.
"""

from __future__ import annotations

import json
import os
from pathlib import Path
import shutil
import sys

import pytest


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import HarnessConfig, HarnessSkip, resolve_harness_config  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402


def classification(record: dict) -> dict[tuple[str, str], tuple[str, str | None, str | None]]:
    out = {}
    for row in record["quantities"]:
        cause = row["cause"]["label"] if row.get("cause") else None
        evidence = row["cause"].get("evidence_status") if row.get("cause") else None
        for check, result in row["checks"].items():
            out[(row["id"], check)] = (result["outcome"], cause, evidence)
    return out


def stored_cases(config: HarnessConfig) -> list[str]:
    known = {case["case_id"] for case in run_pilot.load_cases()}
    found = []
    for case_dir in sorted((config.exchange_root / "cases").glob("*")):
        if case_dir.name not in known:
            continue
        if (case_dir / "comparison" / "comparison.json").is_file() and any((case_dir / "caepipe_out").glob("model*.csv")):
            found.append(case_dir.name)
    return found


def test_stored_caepipe_correlation_classifications_are_stable(tmp_path):
    config = resolve_harness_config(os.environ, require_vm_tools=False)
    if isinstance(config, HarnessSkip):
        pytest.skip(f"private CAEPIPE corpus not available: {config.reason}")
    case_ids = stored_cases(config)
    if not case_ids:
        pytest.skip("the exchange folder holds no case with a stored comparison record and CAEPIPE CSV")
    runner = os.environ.get(run_pilot.RUNNER_ENV)
    if not runner or not Path(runner).is_file():
        pytest.skip(f"no runner executable configured ({run_pilot.RUNNER_ENV})")

    copy_root = tmp_path / "exchange"
    for case_id in case_ids:
        shutil.copytree(config.exchange_root / "cases" / case_id, copy_root / "cases" / case_id)
    copy = HarnessConfig(exchange_root=copy_root, vm_name="not-used-offline", guest_executable="not-used-offline",
                         guest_exchange_root="not-used-offline", guest_work_root="not-used-offline", prlctl=None,
                         acknowledged=True, trial_first_launch_date=config.trial_first_launch_date)
    hand_calcs = run_pilot.load_hand_calcs()
    cases = {case["case_id"]: case for case in run_pilot.load_cases(case_ids)}

    results = run_pilot.run_cases([cases[case_id] for case_id in case_ids], copy, runner=runner, offline=True,
                                  hand_calcs=hand_calcs, emit=lambda line: None)
    failed_cases = sorted(key for key, ok in results.items() if not ok)
    moved = []
    for case_id in case_ids:
        if case_id in failed_cases:
            continue
        stored = json.loads((config.exchange_root / "cases" / case_id / "comparison" / "comparison.json")
                            .read_text(encoding="utf-8"))
        rebuilt = json.loads((copy_root / "cases" / case_id / "comparison" / "comparison.json").read_text(encoding="utf-8"))
        before, after = classification(stored), classification(rebuilt)
        moved.extend(f"{case_id}/{quantity}/{check}" for quantity, check in sorted(set(before) | set(after))
                     if before.get((quantity, check)) != after.get((quantity, check)))
    assert not failed_cases, f"cases without a rebuilt comparison record: {', '.join(failed_cases)}"
    assert not moved, f"classification moved for: {', '.join(moved)}"
