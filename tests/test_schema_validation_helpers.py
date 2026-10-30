"""Caching must preserve mutations, schema roots and every instance verdict."""
from copy import deepcopy
import json
import os

import pytest

from tests import schema_validation as validation


def test_in_memory_schema_mutations_are_rechecked():
    schema = {"type": "integer"}
    assert validation.validate_instance(schema, 7)
    schema["type"] = "string"
    with pytest.raises(AssertionError):
        validation.validate_instance(schema, 7)
    assert validation.validate_instance(schema, "seven")
    schema["type"] = 17
    with pytest.raises(AssertionError, match="not a valid Draft"):
        validation.validate_schema_document(schema)


def test_changed_resources_and_different_roots_are_isolated(monkeypatch, tmp_path):
    schema = {"$id": "https://fixture.invalid/root.schema.json", "$ref": "value.schema.json"}
    first, second = tmp_path / "first", tmp_path / "second"
    for directory, kind in ((first, "integer"), (second, "boolean")):
        directory.mkdir()
        (directory / "value.schema.json").write_text(json.dumps({"type": kind}))
    monkeypatch.setattr(validation, "SCHEMA_DIR", first)
    assert validation.validate_instance(schema, 7)
    monkeypatch.setattr(validation, "SCHEMA_DIR", second)
    with pytest.raises(AssertionError):
        validation.validate_instance(schema, 7)
    assert validation.validate_instance(schema, True)
    monkeypatch.setattr(validation, "SCHEMA_DIR", first)
    resource = first / "value.schema.json"
    prior = resource.stat()
    resource.write_text(json.dumps({"type": "boolean"}))
    os.utime(resource, ns=(prior.st_atime_ns, prior.st_mtime_ns))
    assert resource.stat().st_size == prior.st_size
    with pytest.raises(AssertionError):
        validation.validate_instance(schema, 7)
    assert validation.validate_instance(schema, True)


def test_mutated_instances_are_validated_each_time():
    schema = {"type": "object", "properties": {"value": {"type": "integer"}},
              "required": ["value"], "additionalProperties": False}
    instance = {"value": 7}
    assert validation.validate_instance(schema, instance)
    instance["value"] = "seven"
    with pytest.raises(AssertionError):
        validation.validate_instance(schema, instance)
    assert validation.validate_instance(deepcopy(schema), {"value": 9})


def test_non_json_schema_container_is_not_coerced_to_an_array():
    with pytest.raises(AssertionError, match="not a valid Draft"):
        validation.validate_schema_document({"type": ("integer", "string")})
