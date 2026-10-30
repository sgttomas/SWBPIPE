"""Lossless test-fixture storage keeps scalar bytes and mutation independence."""
import json
import struct

import pytest

from retained_precision_corpus import _STORAGE as storage


@pytest.mark.parametrize("layout", ["compact", "indent-2"])
def test_storage_recovers_exact_bytes_and_expands_independent_containers(layout):
    repeated = {"items": [{"value": [1, 2]}], "negative_zero": -0.0, "integer": 9007199254740993, "numbers": [1, 1.0]}
    options = {"separators": (",", ":")} if layout == "compact" else {"indent": 2}
    original = (json.dumps([repeated, repeated], **options) + "\n").encode()
    packed = storage.compact(original)
    assert storage.expanded_bytes(packed) == original
    expanded = storage.expand(packed)
    assert expanded[0] is not expanded[1]
    assert expanded[0]["items"] is not expanded[1]["items"]
    assert expanded[0]["items"][0] is not expanded[1]["items"][0]
    assert expanded[0]["items"][0]["value"] is not expanded[1]["items"][0]["value"]
    expanded[0]["items"][0]["value"][0] = 9
    assert expanded[1]["items"][0]["value"] == [1, 2]
    assert struct.pack(">d", expanded[1]["negative_zero"]) == struct.pack(">d", -0.0)
    assert type(expanded[1]["integer"]) is int and expanded[1]["integer"] == 9007199254740993
    assert [type(value) for value in expanded[1]["numbers"]] == [int, float]


@pytest.mark.parametrize("definition", [
    {"cases": []},
    {"format": "retained-precision-corpus-dag/v2", "layout": "compact", "nodes": [1], "root": 0},
    {"format": storage.FORMAT, "layout": "indent-4", "nodes": [1], "root": 0},
    {"format": storage.FORMAT, "layout": "compact", "nodes": [[0, 0]], "root": 0},
    {"format": storage.FORMAT, "layout": "compact", "nodes": [1, [0, True]], "root": 1},
    {"format": storage.FORMAT, "layout": "compact", "nodes": [1, [0, 0.0]], "root": 1},
    {"format": storage.FORMAT, "layout": "compact", "nodes": [1], "root": True},
    {"format": storage.FORMAT, "layout": "compact", "nodes": [1], "root": 1},
    {"format": storage.FORMAT, "layout": "compact", "nodes": ["key", [1, 0, 0, 0, 0]], "root": 1},
])
def test_storage_refuses_unsupported_or_malformed_definitions(definition):
    with pytest.raises(ValueError):
        storage.expand(definition)
