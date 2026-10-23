#!/usr/bin/env python3
"""Committed model documents conform to schemas/model.schema.yaml, by rule.

Every committed JSON file is searched for model documents by `document_kind`,
including documents embedded in requests, envelopes and corpora. Rules
(DEL-02-01 Design model-document-0.7.0.md §5.1):

- a document at the current version (0.6.0) validates against the schema root;
- a desktop-profile document also validates against `$defs/DesktopDocument`: the
  bundled demo (named explicitly), the desktop e2e fixtures, and saved projects in
  the persistence fixtures;
- a retired-version document (0.1.0-0.5.0) is excluded: it is a refusal control or
  a frozen input. A 0.5.0 SHA-bound frozen request under validation/** is validated
  after its recorded SHA check and in-memory re-authoring by version alone
  (`qualification_physics.reauthor_model`);
- a partial applier operation fixture (no `project`) is excluded;
- any other exclusion is an explicit entry below, with its reason.

The schema itself is checked as strict JSON (no duplicate keys, no NaN/Infinity).
"""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
import sys
from functools import lru_cache
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
for extra in (ROOT / "tests", ROOT / "tools" / "validation"):
    if str(extra) not in sys.path:
        sys.path.insert(0, str(extra))

from schema_validation import JsonSchemaDependencyMissing, _skip_or_note_missing_jsonschema, validate_instance  # noqa: E402
from qualification_physics import reauthor_model  # noqa: E402


SCHEMA_PATH = ROOT / "schemas" / "model.schema.yaml"
SCHEMA_ID = "https://openpipestress.org/schemas/model.schema.yaml"
DOCUMENT_KIND = "openpipestress.product_preview.model"
CURRENT_VERSION = "0.6.0"
RETIRED_VERSIONS = {"0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0"}
REAUTHORED_FROZEN_VERSION = "0.5.0"
BUNDLED_DEMO = "fixtures/product_preview/invented_demo_model.json"
DESKTOP_PROFILE_PREFIXES = ("apps/desktop/e2e/", "fixtures/persistence/")
PARTIAL_OPERATION_PREFIX = "fixtures/model_operations/"
SKIPPED_DIRECTORIES = {".git", "node_modules", "target", "dist", "__pycache__", ".venv", "scratch"}

# (file, JSON pointer of the document) -> reason. Keep this list short.
EXPLICIT_EXCLUSIONS = {
    ("fixtures/model_operations/exact_pressure_authoring_model.json", ""):
        "stray member data_boundary.source (no reader decodes it, no writer emits it); S1a reports it and deletes nothing",
    ("fixtures/model_operations/physics_thermal_ui_model.json", ""):
        "stray member data_boundary.source (no reader decodes it, no writer emits it); S1a reports it and deletes nothing",
    ("fixtures/model_operations/precision_connected_ui_model.json", ""):
        "stray member data_boundary.source (no reader decodes it, no writer emits it); S1a reports it and deletes nothing",
}


def _reject_constant(token):
    raise ValueError(f"non-JSON constant {token}")


def _unique_object(pairs):
    keys = [key for key, _ in pairs]
    assert len(keys) == len(set(keys)), f"duplicate keys {keys}"
    return dict(pairs)


def strict_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"), parse_constant=_reject_constant,
                      object_pairs_hook=_unique_object)


def committed_json_files() -> list[str]:
    """Project-relative paths of the committed JSON files (git), else a pruned walk."""
    try:
        listed = subprocess.run(["git", "ls-files", "-z", "--", "*.json"], cwd=ROOT, capture_output=True,
                                check=True).stdout.decode("utf-8")
        return sorted(path for path in listed.split("\0") if path)
    except (OSError, subprocess.CalledProcessError):
        found = []
        for directory, names, files in os.walk(ROOT):
            names[:] = [name for name in names if name not in SKIPPED_DIRECTORIES]
            found.extend((Path(directory) / name).relative_to(ROOT).as_posix()
                         for name in files if name.endswith(".json"))
        return sorted(found)


def _documents_in(value, pointer, found):
    if isinstance(value, dict):
        if value.get("document_kind") == DOCUMENT_KIND:
            found.append((pointer, value))
        for key, item in value.items():
            _documents_in(item, f"{pointer}/{key.replace('~', '~0').replace('/', '~1')}", found)
    elif isinstance(value, list):
        for index, item in enumerate(value):
            _documents_in(item, f"{pointer}/{index}", found)


@lru_cache(maxsize=1)
def committed_documents():
    documents = []
    for relative in committed_json_files():
        path = ROOT / relative
        if not path.is_file():
            continue
        found = []
        _documents_in(json.loads(path.read_text(encoding="utf-8")), "", found)
        documents.extend((relative, pointer, document) for pointer, document in found)
    return documents


def recorded_sha256(relative: str) -> str | None:
    """The digest a SHA256SUMS in an ancestor directory under validation/ records for the file."""
    path = ROOT / relative
    for directory in path.parents:
        if directory == ROOT / "validation" or ROOT not in directory.parents:
            return None
        sums = directory / "SHA256SUMS"
        if sums.is_file():
            name = path.relative_to(directory).as_posix()
            for line in sums.read_text(encoding="utf-8").splitlines():
                digest, _, listed = line.partition("  ")
                if listed.strip() == name:
                    return digest.strip()
    return None


def is_desktop_profile(relative: str) -> bool:
    return relative == BUNDLED_DEMO or relative.startswith(DESKTOP_PROFILE_PREFIXES)


def classify():
    """Every committed model document, by rule: (bucket, file, pointer, document to validate)."""
    rows = []
    for relative, pointer, document in committed_documents():
        version = document.get("schema_version")
        if (relative, pointer) in EXPLICIT_EXCLUSIONS:
            rows.append(("explicit", relative, pointer, document))
        elif version == CURRENT_VERSION and "project" not in document:
            bucket = "partial_operation" if relative.startswith(PARTIAL_OPERATION_PREFIX) else "unclassified"
            rows.append((bucket, relative, pointer, document))
        elif version == CURRENT_VERSION:
            rows.append(("desktop" if is_desktop_profile(relative) else "format", relative, pointer, document))
        elif version == REAUTHORED_FROZEN_VERSION and relative.startswith("validation/"):
            rows.append(("reauthored_frozen", relative, pointer, document))
        elif version in RETIRED_VERSIONS:
            rows.append(("retired", relative, pointer, document))
        else:
            rows.append(("unclassified", relative, pointer, document))
    return rows


def _validate(schema, document, label):
    try:
        validate_instance(schema, document, schema_label=str(SCHEMA_PATH), instance_label=label)
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)


def format_schema():
    return strict_json(SCHEMA_PATH)


def desktop_schema():
    return {"$schema": "https://json-schema.org/draft/2020-12/schema", "$ref": f"{SCHEMA_ID}#/$defs/DesktopDocument"}


def test_schema_is_strict_json():
    schema = format_schema()
    assert schema["$id"] == SCHEMA_ID
    # No NaN/Infinity anywhere and no duplicate key at any depth (strict_json raises).
    assert json.loads(json.dumps(schema, allow_nan=False)) == schema


def test_every_committed_model_document_is_classified_by_rule():
    rows = classify()
    unclassified = [(relative, pointer, document.get("schema_version")) for bucket, relative, pointer, document in rows
                    if bucket == "unclassified"]
    assert not unclassified, unclassified
    buckets = {bucket for bucket, *_ in rows}
    assert {"format", "desktop", "retired", "reauthored_frozen", "partial_operation"} <= buckets
    assert any(relative == BUNDLED_DEMO and pointer == "" for _, relative, pointer, _ in rows)


def test_current_documents_validate_against_the_format():
    schema = format_schema()
    for bucket, relative, pointer, document in classify():
        if bucket in {"format", "desktop"}:
            _validate(schema, document, f"{relative}#{pointer}")


def test_desktop_profile_documents_validate_against_the_desktop_profile():
    schema = desktop_schema()
    profile = [(relative, pointer, document) for bucket, relative, pointer, document in classify() if bucket == "desktop"]
    assert any(relative == BUNDLED_DEMO for relative, _, _ in profile)
    assert any(relative.startswith("apps/desktop/e2e/") for relative, _, _ in profile)
    for relative, pointer, document in profile:
        _validate(schema, document, f"{relative}#{pointer} (DesktopDocument)")


def test_frozen_requests_validate_after_sha_check_and_reauthoring():
    schema = format_schema()
    for bucket, relative, pointer, document in classify():
        if bucket != "reauthored_frozen":
            continue
        digest = recorded_sha256(relative)
        if digest is not None:
            assert hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == digest, relative
        reauthored = reauthor_model(document)
        assert {key: value for key, value in reauthored.items() if key != "schema_version"} == \
            {key: value for key, value in document.items() if key != "schema_version"}
        assert reauthored["schema_version"] == CURRENT_VERSION
        _validate(schema, reauthored, f"{relative}#{pointer} (re-authored)")


def test_explicit_exclusions_are_live_and_still_fail():
    schema = format_schema()
    present = {(relative, pointer): document for bucket, relative, pointer, document in classify() if bucket == "explicit"}
    assert set(present) == set(EXPLICIT_EXCLUSIONS)
    for key, document in present.items():
        with pytest.raises(AssertionError):
            validate_instance(schema, document, schema_label=str(SCHEMA_PATH), instance_label=f"{key[0]}#{key[1]}")


@pytest.mark.parametrize(("definition", "target"), [
    ("Result", "results.v0.3.schema.yaml#/$defs/ResultEnvelope"),
    ("Report", "report_sections.schema.yaml#/$defs/ReportSectionEnvelope"),
])
def test_result_and_report_are_defined_by_resolvable_reference(definition, target):
    """REQ-02-01-02: Result and Report are defined by reference; the root uses neither."""
    schema = format_schema()
    assert schema["$defs"][definition]["$ref"] == target
    assert f"#/$defs/{definition}" not in json.dumps(schema["properties"])
    reference = {"$schema": "https://json-schema.org/draft/2020-12/schema",
                 "$ref": f"{SCHEMA_ID}#/$defs/{definition}"}
    # The shared registry resolves the sibling file; an empty object is then refused
    # by the referenced definition (an unresolvable reference raises instead).
    try:
        with pytest.raises(AssertionError, match="required property"):
            validate_instance(reference, {}, schema_label=str(SCHEMA_PATH), instance_label=f"empty {definition}")
    except JsonSchemaDependencyMissing as exc:
        _skip_or_note_missing_jsonschema(exc)
