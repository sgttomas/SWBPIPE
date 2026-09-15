from __future__ import annotations
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
