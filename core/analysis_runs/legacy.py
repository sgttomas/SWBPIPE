"""Immutable legacy analysis checksum verification.

Verification consumes the exact historical preimage evidence. It never
reconstructs or rewrites a received claim under another profile.
"""
from __future__ import annotations
from hashlib import sha256
import json
import re
from typing import Any, Mapping

PYTHON_PROFILE = "analysis_run_python_sorted_compact_v0_1"
DESKTOP_PROFILE = "analysis_run_desktop_locale_v0_1"


def verify_legacy_checksum(
    claimed: Mapping[str, Any], *, profile: str | None, preimage: Any | None = None,
    serialized_preimage: bytes | None = None, numeric_types_preserved: bool = True,
    expected_payload_scope: str, expected_payload_ref: Mapping[str, Any],
    preimage_provenance_established: bool,
) -> dict[str, Any]:
    result = {"status": "unverifiable", "profile": profile, "received_claim": dict(claimed), "claim_rewritten": False}
    expected_label = {PYTHON_PROFILE: "SORTED_COMPACT_JSON", DESKTOP_PROFILE: "rfc8785_jcs"}.get(profile)
    if (
        expected_label is None
        or not numeric_types_preserved
        or not preimage_provenance_established
        or claimed.get("algorithm") != "sha256"
        or claimed.get("canonicalization") != expected_label
        or claimed.get("payload_scope") != expected_payload_scope
        or claimed.get("payload_ref") != expected_payload_ref
        or re.fullmatch(r"(?:sha256:)?[0-9a-f]{64}", str(claimed.get("value", ""))) is None
    ):
        return result
    if profile == PYTHON_PROFILE:
        if preimage is None:
            return result
        encoded = json.dumps(preimage, sort_keys=True, separators=(",", ":"), ensure_ascii=True).encode("utf-8")
    elif profile == DESKTOP_PROFILE:
        if serialized_preimage is None:
            return result
        encoded = bytes(serialized_preimage)
    else:
        return result
    received = str(claimed.get("value", "")).removeprefix("sha256:")
    result["status"] = "match" if sha256(encoded).hexdigest() == received else "mismatch"
    return result
