from __future__ import annotations
import hashlib
import json
import os
from pathlib import Path
import pytest

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1, canonicalize_batch


@pytest.fixture(autouse=True)
def configured_checked_binary(monkeypatch: pytest.MonkeyPatch) -> None:
    configured = os.environ.get("OPENPIPESTRESS_CHECKED_JSON_BIN")
    if configured:
        monkeypatch.setenv("OPENPIPESTRESS_CHECKED_JSON_BIN", configured)


def test_checked_adapter_vectors() -> None:
    assert canonical_json_checked_v1({"\uff5a": 1, "😀": 2, "n": -0.0}) == '{"n":0,"😀":2,"ｚ":1}'
    assert len(canonical_sha256_checked_v1({"one": 1.0})) == 64


def test_checked_fixture_matches_rust_wasm_contract() -> None:
    corpus = json.loads((Path(__file__).parents[1] / "fixtures/canonical_hash/cases.json").read_text())
    supported = {"scalar-string-unicode", "number-negative-zero-renders-zero", "number-ecma-notation-boundaries", "number-beyond-2-53-integer-kept-exact"}
    cases = [case | {"outcome": "reject" if case["case_id"] in {"number-beyond-2-53-integer-kept-exact", "number-ecma-notation-boundaries"} else "accept"} for case in corpus["cases"] if case["case_id"] in supported]
    cases.append({"case_id": "one-and-one-point-zero", "input_json": "[1,1.0]", "outcome": "accept", "expected_canonical": "[1,1]", "expected_sha256": "e61b9f584dbe27741cef6e9ee440831d7d94470c0871b0871541f0308916efea"})
    cases.append({"case_id": "safe-exponent-boundaries", "input_json": "[1e-6,1e-7]", "outcome": "accept", "expected_canonical": "[0.000001,1e-7]", "expected_sha256": "19ca01c5d07894d9ce68294ad32b64d9c2a851c244ae8010e0a2b8a26f3734a0"})
    for case in cases:
        value = json.loads(case["input_json"])
        if case["outcome"] == "reject":
            with pytest.raises(ValueError, match="UNSAFE|OUTSIDE"):
                canonical_json_checked_v1(value)
            continue
        canonical = canonical_json_checked_v1(value)
        assert canonical == case["expected_canonical"], case["case_id"]
        assert hashlib.sha256(canonical.encode()).hexdigest() == case["expected_sha256"]
        assert canonical_sha256_checked_v1(value) == case["expected_sha256"]


@pytest.mark.parametrize("value", [9007199254740992, 1.7976931348623157e308, float("inf"), {1: "bad"}])
def test_checked_adapter_rejects_outside_profile(value: object) -> None:
    with pytest.raises(ValueError):
        canonical_json_checked_v1(value)


@pytest.mark.parametrize("value", ["\ud800", {"\udfff": "bad"}])
def test_checked_adapter_rejects_lone_surrogate_before_transport(value: object) -> None:
    with pytest.raises(ValueError, match="LONE-SURROGATE"):
        canonical_json_checked_v1(value)


def test_batch_rejects_non_string_empty_and_duplicate_ids_before_transport() -> None:
    for items in [((1, {}),), (("", {}),), (("\ud800", {}),), (("same", {}), ("same", {}))]:
        with pytest.raises(ValueError, match="ITEM-ID-INVALID"):
            canonicalize_batch(items)  # type: ignore[arg-type]


def test_invalid_batch_id_does_not_start_process(monkeypatch: pytest.MonkeyPatch) -> None:
    monkeypatch.setattr("core.serialization.canonical_json.adapter.subprocess.run", lambda *args, **kwargs: pytest.fail("process started"))
    with pytest.raises(ValueError, match="ITEM-ID-INVALID"):
        canonicalize_batch((("\udfff", {}),))
