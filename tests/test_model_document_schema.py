#!/usr/bin/env python3
"""Committed model documents conform to schemas/model.schema.yaml, by rule.

Every committed JSON file and every line of every committed JSON Lines file is
searched for model documents by `document_kind`, including documents embedded in
requests, envelopes and corpora. Rules (DEL-02-01 Design model-document-0.7.0.md §5.1):

- a document at the current version (0.6.0) validates against the schema root;
- a desktop-profile document also validates against `$defs/DesktopDocument`: the
  bundled demo (named explicitly), the desktop e2e fixtures, and saved projects in
  the persistence fixtures;
- a SHA-bound frozen input (a retired-version document under validation/** whose
  file a SHA256SUMS lists) is validated after its recorded digest is checked and the
  copy is re-authored in memory (`qualification_physics.reauthor_model`);
- any other retired-version document (0.1.0-0.5.0) is excluded: it is a refusal
  control or an unbound frozen input;
- a partial applier operation fixture is excluded: the material-only base and
  applied documents of the applier contract corpus, pinned below;
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
BUNDLED_DEMO = "fixtures/product_preview/invented_demo_model.json"
DESKTOP_PROFILE_PREFIXES = ("apps/desktop/e2e/", "fixtures/persistence/")
# The applier contract corpus's partial documents: a case file's base or expected
# applied model carrying only the members its operation reads (no `project`).
PARTIAL_OPERATION_PREFIX = "fixtures/model_operations/contract_corpus/case_"
PARTIAL_OPERATION_POINTERS = {"/base_model", "/expected/applied_model"}
PARTIAL_OPERATION_MEMBERS = {"schema_version", "document_kind", "materials"}
PARTIAL_OPERATION_DOCUMENTS = {
    ("fixtures/model_operations/contract_corpus/case_76_accept_set_field_material_temperature_point_shear_modulus.json", "/base_model"),
    ("fixtures/model_operations/contract_corpus/case_76_accept_set_field_material_temperature_point_shear_modulus.json", "/expected/applied_model"),
    ("fixtures/model_operations/contract_corpus/case_77_block_set_field_material_temperature_point_shear_modulus_missing_point.json", "/base_model"),
    ("fixtures/model_operations/contract_corpus/case_78_block_set_field_material_temperature_point_shear_modulus_invalid.json", "/base_model"),
}
SKIPPED_DIRECTORIES = {".git", "node_modules", "target", "dist", "__pycache__", ".venv", "scratch"}

# (file, JSON pointer of the document) -> reason. Keep this list short.
EXPLICIT_EXCLUSIONS: dict[tuple[str, str], str] = {}


def _reject_constant(token):
    raise ValueError(f"non-JSON constant {token}")


def _unique_object(pairs):
    keys = [key for key, _ in pairs]
    assert len(keys) == len(set(keys)), f"duplicate keys {keys}"
    return dict(pairs)


def strict_json(path: Path):
    return json.loads(path.read_text(encoding="utf-8"), parse_constant=_reject_constant,
                      object_pairs_hook=_unique_object)


def committed_files(pattern: str) -> list[str]:
    """Project-relative paths of the committed files matching `pattern` (git), else a pruned walk."""
    suffix = pattern.lstrip("*")
    try:
        listed = subprocess.run(["git", "ls-files", "-z", "--", pattern], cwd=ROOT, capture_output=True,
                                check=True).stdout.decode("utf-8")
        return sorted(path for path in listed.split("\0") if path)
    except (OSError, subprocess.CalledProcessError):
        found = []
        for directory, names, files in os.walk(ROOT):
            names[:] = [name for name in names if name not in SKIPPED_DIRECTORIES]
            found.extend((Path(directory) / name).relative_to(ROOT).as_posix()
                         for name in files if name.endswith(suffix))
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
    """(file, pointer, document); a JSON Lines document's pointer starts with its line `#<n>`."""
    documents = []
    for relative in committed_files("*.json"):
        path = ROOT / relative
        if path.is_file():
            found = []
            _documents_in(json.loads(path.read_text(encoding="utf-8")), "", found)
            documents.extend((relative, pointer, document) for pointer, document in found)
    for relative in committed_files("*.jsonl"):
        path = ROOT / relative
        if path.is_file():
            for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
                if line.strip():
                    found = []
                    _documents_in(json.loads(line), f"#{number}", found)
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


def is_partial_operation(relative: str, pointer: str, document) -> bool:
    """The intended partial structure, recognized positively; a full document missing `project` is not one."""
    return (relative.startswith(PARTIAL_OPERATION_PREFIX) and pointer in PARTIAL_OPERATION_POINTERS
            and set(document) == PARTIAL_OPERATION_MEMBERS)


def is_sha_bound_frozen(relative: str) -> bool:
    return relative.startswith("validation/") and recorded_sha256(relative) is not None


def is_desktop_profile(relative: str) -> bool:
    return relative == BUNDLED_DEMO or relative.startswith(DESKTOP_PROFILE_PREFIXES)


def classify():
    """Every committed model document, by rule: (bucket, file, pointer, document to validate)."""
    rows = []
    for relative, pointer, document in committed_documents():
        version = document.get("schema_version")
        if (relative, pointer) in EXPLICIT_EXCLUSIONS:
            rows.append(("explicit", relative, pointer, document))
        elif version == CURRENT_VERSION and is_partial_operation(relative, pointer, document):
            rows.append(("partial_operation", relative, pointer, document))
        elif version == CURRENT_VERSION:
            rows.append(("desktop" if is_desktop_profile(relative) else "format", relative, pointer, document))
        elif version in RETIRED_VERSIONS and is_sha_bound_frozen(relative):
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
        assert digest is not None, relative
        assert hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == digest, relative
        reauthored = reauthor_model(document)
        assert reauthored["schema_version"] == CURRENT_VERSION
        if document["schema_version"] == "0.5.0":
            # 0.5.0 is re-authored by its version alone.
            assert {key: value for key, value in reauthored.items() if key != "schema_version"} == \
                {key: value for key, value in document.items() if key != "schema_version"}
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


def test_partial_operation_documents_are_the_pinned_ones():
    partial = {(relative, pointer) for bucket, relative, pointer, _ in classify() if bucket == "partial_operation"}
    assert partial == PARTIAL_OPERATION_DOCUMENTS
    assert len(partial) == 4


def test_a_full_document_missing_project_is_not_partial_and_fails():
    document = json.loads((ROOT / BUNDLED_DEMO).read_text(encoding="utf-8"))
    del document["project"]
    contract_case = f"{PARTIAL_OPERATION_PREFIX}00_invented.json"
    assert not is_partial_operation(contract_case, "/base_model", document)
    with pytest.raises(AssertionError, match="'project' is a required property"):
        validate_instance(format_schema(), document, schema_label=str(SCHEMA_PATH), instance_label="demo without project")


def test_frozen_coverage_includes_every_retired_version_and_json_lines():
    frozen = [(relative, document["schema_version"]) for bucket, relative, _, document in classify()
              if bucket == "reauthored_frozen"]
    # The 0.1.0 frozen requests (validation/references/kd5_nonlinear) are listed in no SHA256SUMS.
    assert {version for _, version in frozen} == RETIRED_VERSIONS - {"0.1.0"}
    assert any(relative.endswith(".jsonl") for relative, _ in frozen)

