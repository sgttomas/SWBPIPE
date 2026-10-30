"""Shared JSON Schema validation helpers for PKG-02 contract tests."""

from __future__ import annotations

from functools import lru_cache
import json
import sys
from pathlib import Path
from copy import deepcopy
from urllib.parse import urljoin


INSTALL_MESSAGE = (
    "jsonschema>=4.18,<5 is required for full PKG-02 JSON Schema validation; "
    "install with: python3 -m pip install -r requirements-dev.txt"
)


class JsonSchemaDependencyMissing(RuntimeError):
    """Raised when optional JSON Schema validation tooling is unavailable."""


def _draft202012_validator():
    try:
        from jsonschema import Draft202012Validator
    except ModuleNotFoundError as exc:
        raise JsonSchemaDependencyMissing(INSTALL_MESSAGE) from exc
    return Draft202012Validator


def _schema_text(schema):
    """Cache only JSON-shaped schemas; preserve other Python values' validation."""
    def plain(value):
        if value is None or type(value) in (bool, int, float, str):
            return True
        if type(value) is list:
            return all(plain(item) for item in value)
        if type(value) is dict:
            return all(type(key) is str and plain(item) for key, item in value.items())
        return False
    return json.dumps(schema, separators=(",", ":")) if plain(schema) else None


@lru_cache(maxsize=128)
def _checked_schema(text):
    schema = json.loads(text)
    _draft202012_validator().check_schema(schema)
    return schema


def validate_schema_document(schema, *, schema_label="schema"):
    """Validate each distinct schema content with Draft 2020-12."""
    validator_class = _draft202012_validator()
    try:
        text = _schema_text(schema)
        if text is None:
            validator_class.check_schema(schema)
        else:
            _checked_schema(text)
    except Exception as exc:  # jsonschema raises SchemaError subclasses.
        raise AssertionError(f"{schema_label} is not a valid Draft 2020-12 schema: {exc}") from exc
    return True


def schema_for_definition(root_schema, definition_name):
    """Build a small root schema that validates one definition from a larger schema."""
    schema = {
        "$schema": root_schema.get("$schema", "https://json-schema.org/draft/2020-12/schema"),
        "$defs": deepcopy(root_schema["$defs"]),
        "$ref": f"#/$defs/{definition_name}",
    }
    return schema


SCHEMA_DIR = Path(__file__).resolve().parents[1] / "schemas"


def _resource_texts():
    # Read current bytes for the cache key: an edited resource or another root
    # cannot reuse a validator from the preceding schema content.
    resources = []
    for path in SCHEMA_DIR.glob("*.schema.*"):
        try:
            resources.append((path.resolve().as_uri(), path.name, path.read_text(encoding="utf-8")))
        except (OSError, UnicodeError):
            continue
    return tuple(resources)


def envelope_branches(results):
    """ResultEnvelope's oneOf branches of a results schema; a branch held in a
    sibling schema (`<file>#/$defs/<name>`, as pressure-2's) is read from it."""
    schema_dir = Path(__file__).resolve().parents[1] / "schemas"
    branches = []
    for branch in results["$defs"]["ResultEnvelope"]["oneOf"]:
        ref = branch.get("$ref", "#")
        if not ref.startswith("#"):
            name, definition = ref.split("#/$defs/")
            branch = json.loads((schema_dir / name).read_text(encoding="utf-8"))["$defs"][definition]
        branches.append(branch)
    return branches


@lru_cache(maxsize=32)
def _registry(resource_texts, base_uri):
    try:
        from referencing import Registry, Resource
        from referencing.jsonschema import DRAFT202012
    except ImportError as exc:
        raise JsonSchemaDependencyMissing(INSTALL_MESSAGE) from exc
    resources = {}
    for file_uri, name, text in resource_texts:
        try:
            candidate = json.loads(text)
        except ValueError:
            continue  # Only JSON-encoded checked-in schema resources are used.
        resource = Resource.from_contents(candidate, default_specification=DRAFT202012)
        retrieval_urls = {file_uri, urljoin(base_uri, name)}
        if "$id" in candidate:
            retrieval_urls.add(candidate["$id"])
            retrieval_urls.update(urljoin(uri, candidate["$id"]) for uri in list(retrieval_urls))
        resources.update({uri: resource for uri in retrieval_urls})
    # Immutable per-reference scopes avoid RefResolver's leaked receipt base.
    # The registry has no network retriever; every external resource is local.
    return Registry().with_resources(resources.items())


@lru_cache(maxsize=128)
def _instance_validator(text, resource_texts, base_uri):
    return _draft202012_validator()(_checked_schema(text), registry=_registry(resource_texts, base_uri))


def validate_instance(schema, instance, *, schema_label="schema", instance_label="instance"):
    """Validate every instance, reusing setup for identical schema content."""
    validate_schema_document(schema, schema_label=schema_label)
    resource_texts = _resource_texts()
    base_uri = schema.get("$id", "")
    text = _schema_text(schema)
    validator = (_instance_validator(text, resource_texts, base_uri) if text is not None
                 else _draft202012_validator()(schema, registry=_registry(resource_texts, base_uri)))
    errors = sorted(validator.iter_errors(instance), key=lambda error: list(error.path))
    if errors:
        formatted = "\n".join(_format_error(error) for error in errors[:10])
        remaining = len(errors) - 10
        suffix = f"\n... {remaining} more validation errors" if remaining > 0 else ""
        raise AssertionError(f"{instance_label} failed JSON Schema validation:\n{formatted}{suffix}")
    return True


def _format_error(error):
    path = "$"
    for part in error.path:
        if isinstance(part, int):
            path += f"[{part}]"
        else:
            path += f".{part}"
    return f"{path}: {error.message}"


# Shared stdlib schema-inspection helpers, consolidated from the per-file
# copies that the schema/contract tests previously duplicated verbatim.

def load_schema(schema_path):
    with schema_path.open(encoding="utf-8") as schema_file:
        return json.load(schema_file)


def required_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["required"])


def enum_at(schema, definition_name):
    return set(schema["$defs"][definition_name]["enum"])


def walk_keys(value):
    if isinstance(value, dict):
        for key, item in value.items():
            yield key
            yield from walk_keys(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_keys(item)


def walk_strings(value):
    if isinstance(value, str):
        yield value
    elif isinstance(value, dict):
        for item in value.values():
            yield from walk_strings(item)
    elif isinstance(value, list):
        for item in value:
            yield from walk_strings(item)


def _skip_or_note_missing_jsonschema(exc):
    if "pytest" in sys.modules:
        import pytest

        pytest.skip(str(exc))
    print(f"SKIP: {exc}")
