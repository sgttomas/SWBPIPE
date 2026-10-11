"""The Rust product reader's verdict for Python product code.

Rust is the only product reader of result statements. Python product code that
meets one takes the Rust reader's verdict through `openpipestress-runner read
<reader>` and never re-derives it: JSON in on stdin, the reader's `Validation`
or refusal (gate, code and detail) out on stdout, exit 0 when it passed and 1
when it refused.

The runner is an explicitly built executable named by OPENPIPESTRESS_RUNNER_BIN,
an absolute path. There is no runtime build, PATH search, shell or fallback.
Development and test setup: `tools/runner/build_runner.py`.
"""
from __future__ import annotations

from collections.abc import Mapping
import json
import os
from pathlib import Path
import subprocess
from typing import Any

ENV_EXECUTABLE = "OPENPIPESTRESS_RUNNER_BIN"
ARTIFACT = "openpipestress.result_reader_verdict"
SCHEMA_VERSION = "1.0.0"
RETAINED_PRECISION = "retained-precision"
RETAINED_PRECISION_TRANSPORT = "retained-precision-transport"


class ReaderRefusal(ValueError):
    """The Rust reader refused the statement: its gate, bare code and optional detail."""

    def __init__(self, gate: str, code: str, detail: str | None):
        self.gate, self.code, self.detail = gate, code, detail
        super().__init__(detail or code)


def runner_executable() -> Path:
    configured = os.environ.get(ENV_EXECUTABLE)
    if not configured or not Path(configured).is_absolute():
        raise RuntimeError(f"RUST-READER-MISSING: set {ENV_EXECUTABLE} to an explicitly built absolute openpipestress-runner path")
    path = Path(configured)
    if not path.is_file():
        raise RuntimeError(f"RUST-READER-MISSING: {path}")
    return path


def read(reader: str, payload: Mapping[str, Any]) -> dict[str, Any]:
    """Run one Rust reader over `payload`; return its validation or raise its refusal.

    A payload that is not JSON (a non-finite number, a non-JSON type) raises
    ValueError or TypeError before the reader runs."""
    text = json.dumps(payload, ensure_ascii=False, allow_nan=False, separators=(",", ":"))
    completed = subprocess.run([str(runner_executable()), "read", reader], input=text.encode("utf-8"),
                               capture_output=True, check=False, shell=False)
    stderr = completed.stderr.decode("utf-8", "replace").strip()
    if completed.returncode not in (0, 1):
        raise RuntimeError(f"RUST-READER-FAILED: {reader} exited {completed.returncode}: {stderr}")
    try:
        verdict = json.loads(completed.stdout)
    except ValueError as error:
        raise RuntimeError(f"RUST-READER-RESPONSE-INVALID: {reader}") from error
    expected = "pass" if completed.returncode == 0 else "refused"
    member = "validation" if completed.returncode == 0 else "refusal"
    if (not isinstance(verdict, dict) or set(verdict) != {"artifact", "schema_version", "reader", "verdict", member}
            or verdict["artifact"] != ARTIFACT or verdict["schema_version"] != SCHEMA_VERSION
            or verdict["reader"] != reader or verdict["verdict"] != expected or not isinstance(verdict[member], dict)):
        raise RuntimeError(f"RUST-READER-RESPONSE-INVALID: {reader}")
    if completed.returncode == 1:
        refusal = verdict["refusal"]
        raise ReaderRefusal(refusal["gate"], refusal["code"], refusal["detail"])
    return verdict["validation"]


def retained_precision(source: Mapping[str, Any], invocation: Any = None) -> dict[str, Any]:
    """`retained_precision::validate`: G0-G8 on a successor, bound to `invocation` when one is given."""
    return read(RETAINED_PRECISION, {"source": source, "invocation": invocation})


def retained_precision_transport(source: Mapping[str, Any]) -> dict[str, Any]:
    """`retained_precision::validate_transport_metadata`: a transported successor's checks; never eligible."""
    return read(RETAINED_PRECISION_TRANSPORT, {"source": source})
