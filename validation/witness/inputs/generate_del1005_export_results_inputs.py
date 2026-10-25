#!/usr/bin/env python3
"""Derive the DEL-10-05 export-results failure inputs from the success input.

The success input and the alias overlay (package version 2.0.0) are written by
the Rust witness generator, ``core/runner/headless/tests/del1005_package_witness.rs``
(run it with ``OPENPIPESTRESS_UPDATE_DEL1005_WITNESS=1``), because Python
cannot derive a DEL-08-04 0.3.0 document (DEL-08-01 Design
``report-package-current-results.md`` §2.7). This script makes one edit each
to the success input:

- the missing-payload input, committed here (it carries no document);
- the binding-mismatch and producer-blocked inputs, written only with
  ``--out DIR``. They differ from the success input in one value each, so they
  are not committed; ``tests/test_headless_runner_contract.py`` makes the same
  edits in memory.

All data is invented.
"""

from __future__ import annotations

import argparse
import copy
import json
from pathlib import Path


HERE = Path(__file__).resolve().parent
SUCCESS = HERE / "del1005_export_results_success_input.json"


def derive(success: dict[str, object]) -> dict[str, dict[str, object]]:
    """The three failure inputs, by file name."""
    mismatch = copy.deepcopy(success)
    mismatch["request"]["model_ref"]["ref_id"] = "invented-mismatch"
    blocked = copy.deepcopy(success)
    blocked["export_results"]["package_id"] = ""
    return {
        "del1005_export_results_missing_payload_input.json": {"request": success["request"]},
        "del1005_export_results_binding_mismatch_input.json": mismatch,
        "del1005_export_results_producer_blocked_input.json": blocked,
    }


def write(directory: Path, name: str, value: dict[str, object]) -> None:
    (directory / name).write_text(
        json.dumps(value, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--out", type=Path, help="also write the mismatch and producer-blocked inputs here")
    args = parser.parse_args()
    success = json.loads(SUCCESS.read_text(encoding="utf-8"))
    inputs = derive(success)
    missing = "del1005_export_results_missing_payload_input.json"
    write(HERE, missing, inputs[missing])
    if args.out is not None:
        for name, value in inputs.items():
            write(args.out, name, value)


if __name__ == "__main__":
    main()
