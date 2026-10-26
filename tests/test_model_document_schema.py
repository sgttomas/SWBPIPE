#!/usr/bin/env python3
"""Committed model documents conform to schemas/model.schema.yaml, by rule.

Every committed JSON file and every line of every committed JSON Lines file is
searched for model documents by `document_kind`, including documents embedded in
requests, envelopes and corpora. Rules (DEL-02-01 Design model-document-0.7.0.md §5.1):

- a document at the current version (0.7.0) validates against the schema root;
- a desktop-profile document also validates against `$defs/DesktopDocument`: the
  bundled demo (named explicitly), the desktop e2e fixtures, and saved projects in
  the persistence fixtures;
- a retired-version document under validation/** is a frozen input, by path. Its
  file's SHA256SUMS digest is required and checked, then the copy is re-authored in
  memory (`qualification_physics.reauthor_model`) and validated. A frozen input
  that no SHA256SUMS binds is excluded only by its entry in UNBOUND_FROZEN_INPUTS;
- any other retired-version document (0.1.0-0.6.0) is excluded: it is a refusal
  control;
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
CURRENT_VERSION = "0.7.0"
RETIRED_VERSIONS = {"0.1.0", "0.2.0", "0.3.0", "0.4.0", "0.5.0", "0.6.0"}
# W-15 slice c guard preparation (DEL-02-01 Design model-document-0.8.0.md §5): a
# retired version listed here classifies a non-frozen committed document as a
# refusal control only when the document is listed under it. Any other document at
# that version is unclassified, and the test fails, so a current fixture the next
# version ripple did not sweep cannot pass silently as a refusal control. The ripple
# adds the version it retires with its explicit controls; earlier versions keep the
# open rule.
EXPLICIT_RETIRED_CONTROLS: dict[str, set[str]] = {}


def is_retired_refusal_control(version, relative):
    """Whether a non-frozen document at a retired version is a refusal control."""
    if version not in RETIRED_VERSIONS:
        return False
    listed = EXPLICIT_RETIRED_CONTROLS.get(version)
    return listed is None or relative in listed
BUNDLED_DEMO = "fixtures/product_preview/invented_demo_model.json"
DESKTOP_PROFILE_PREFIXES = ("apps/desktop/e2e/", "fixtures/persistence/")
# The desktop emission golden (§5.2, TP-PMM-GUIEMIT-001): documents the desktop wrote.
DESKTOP_EMISSION_GOLDEN = "fixtures/model_document/desktop_emitted.json"
# The applier golden (§5.2): one applied document per W-13 model-data operation kind.
APPLIER_EMISSION_GOLDEN = "fixtures/model_document/applier_emitted.json"
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

# Retired-version documents under this root are frozen inputs (references,
# qualification fixtures and witnesses), whatever their digest lookup finds.
FROZEN_ROOT = "validation/"
_QUALIFICATION_LOAD_REFERENCE = "validation/qualification/fixtures/load_reference/"
_LOAD_REFERENCE_CASES = (
    "coefficient_definition", "constant_alpha_interval", "multi_segment_free_length", "persistent_source_once",
    "prescribed_rotation_all_fixed", "prescribed_rotation_free_tip", "prescribed_translation_all_fixed",
    "prescribed_translation_two_bar", "shared_material_serial_companion", "signed_fit_states.fixed",
    "signed_fit_states.released", "temperature_unit_identity", "thermal_datum_ratio.fixed",
    "thermal_datum_ratio.free",
)
# Frozen inputs no SHA256SUMS lists: file -> reason. Their bytes are bound elsewhere
# or not at all, so they are not re-authored here. The count is asserted.
UNBOUND_FROZEN_INPUTS = {
    **{f"{_QUALIFICATION_LOAD_REFERENCE}{case}.{kind}.json":
       "0.4.0 qualification load/reference fixture; bound by the sha256 its package MANIFEST.json records, not a SHA256SUMS"
       for case in _LOAD_REFERENCE_CASES for kind in ("preview_request", "runner_input")},
    **{f"validation/qualification/fixtures/first_static/{name}.runner_input.candidate.json":
       "0.3.0 first-static qualification candidate input; bound by the sha256 INPUT_REFERENCE_BINDINGS.json records, not a SHA256SUMS"
       for name in ("axial", "bending_torsion")},
    **{f"validation/references/kd5_member_bound/{name}.json":
       "0.4.0 KD5 member-bound reference request; its reference set carries no SHA256SUMS"
       for name in ("request", "two_body_request")},
    **{f"validation/references/kd5_nonlinear/{name}.json":
       "0.1.0 KD5 nonlinear reference request; its reference set carries no SHA256SUMS"
       for name in ("f1_explicit_request", "f1_mu0_request", "f1_request", "f2_request", "u1_request",
                    "u1_rx_request", "u1_spring_request")},
}

# (file, JSON pointer of the document) -> reason. Keep this list short.
EXPLICIT_EXCLUSIONS: dict[tuple[str, str], str] = {
    **{("fixtures/comparison/model_state_comparison_golden.json", f"/pairs/{pair}/{side}_entities/0/fields"):
       "the `model` entity of a state-comparison parity pair: the document's own members as flattened comparison "
       "fields of invented inputs (DEL-14-01 Design §4.1), not a model document"
       for pair in range(8) for side in ("left", "right")},
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


def is_frozen_input(relative: str) -> bool:
    return relative.startswith(FROZEN_ROOT)


def is_desktop_profile(relative: str) -> bool:
    return relative in {BUNDLED_DEMO, DESKTOP_EMISSION_GOLDEN} or relative.startswith(DESKTOP_PROFILE_PREFIXES)


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
        elif version in RETIRED_VERSIONS and is_frozen_input(relative) and relative in UNBOUND_FROZEN_INPUTS:
            rows.append(("unbound_frozen", relative, pointer, document))
        elif version in RETIRED_VERSIONS and is_frozen_input(relative):
            rows.append(("reauthored_frozen", relative, pointer, document))
        elif is_retired_refusal_control(version, relative):
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
    assert {"format", "desktop", "retired", "reauthored_frozen", "unbound_frozen", "partial_operation"} <= buckets
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


def checked_frozen_copy(relative: str, document):
    """The re-authored copy of a frozen input, after its required SHA256SUMS digest is checked."""
    digest = recorded_sha256(relative)
    assert digest is not None, f"{relative}: a frozen input needs a SHA256SUMS digest or an UNBOUND_FROZEN_INPUTS entry"
    assert hashlib.sha256((ROOT / relative).read_bytes()).hexdigest() == digest, relative
    reauthored = reauthor_model(document)
    assert reauthored["schema_version"] == CURRENT_VERSION
    if document["schema_version"] == "0.5.0":
        # 0.5.0 is re-authored by its version alone.
        assert {key: value for key, value in reauthored.items() if key != "schema_version"} == \
            {key: value for key, value in document.items() if key != "schema_version"}
    return reauthored


def test_frozen_requests_validate_after_sha_check_and_reauthoring():
    schema = format_schema()
    for bucket, relative, pointer, document in classify():
        if bucket == "reauthored_frozen":
            _validate(schema, checked_frozen_copy(relative, document), f"{relative}#{pointer} (re-authored)")


def test_a_frozen_input_without_its_digest_fails(monkeypatch):
    bucket, relative, _, document = next(row for row in classify() if row[0] == "reauthored_frozen")
    assert recorded_sha256(relative) is not None
    monkeypatch.setattr(sys.modules[__name__], "recorded_sha256", lambda path: None)
    with pytest.raises(AssertionError, match="needs a SHA256SUMS digest"):
        checked_frozen_copy(relative, document)


def test_unbound_frozen_inputs_are_live_and_unbound():
    rows = [(relative, document["schema_version"]) for bucket, relative, _, document in classify()
            if bucket == "unbound_frozen"]
    assert {relative for relative, _ in rows} == set(UNBOUND_FROZEN_INPUTS)
    assert len(UNBOUND_FROZEN_INPUTS) == len(rows) == 39
    for relative, version in rows:
        assert version in RETIRED_VERSIONS and recorded_sha256(relative) is None, relative


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


ANNOTATED_FIXTURE = "fixtures/model_document/invented_annotated_model.json"
# PRD §20.2: the fields a public data record carries (library import REQUIRED_PROVENANCE_FIELDS).
PUBLIC_PROVENANCE_FIELDS = ("source_name", "source_location", "source_license", "contributor",
                            "contributor_certification", "redistribution_status", "review_status")
PROVENANCE_RECORD_COLLECTIONS = ("materials", "sections", "components", "supports")


def test_the_invented_annotated_model_carries_every_new_member():
    """DEL-02-01 Design model-document-0.7.0.md §3: one committed invented document authors every slot."""
    rows = [(bucket, document) for bucket, relative, pointer, document in classify()
            if relative == ANNOTATED_FIXTURE and pointer == ""]
    assert [bucket for bucket, _ in rows] == ["format"]
    document = rows[0][1]
    for member in ("lines", "unresolved_assumptions", "rule_pack_refs", "constraints", "equipment_interfaces"):
        assert document[member], member
    for collection in ("nodes", "pipe_segments", "sections", "materials", "supports", "components", "load_cases",
                       "combinations", "lines", "unresolved_assumptions", "constraints", "equipment_interfaces"):
        assert any("tags" in record and "notes" in record for record in document[collection]), collection
    assert "tags" in document["project"] and "notes" in document["project"]
    for collection in PROVENANCE_RECORD_COLLECTIONS:
        assert any("provenance_record" in record for record in document[collection]), collection
    assert {record["kind"] for record in document["unresolved_assumptions"]} == {"assumption", "open_issue"}


def test_committed_provenance_records_are_complete_and_public():
    """§3.5: a committed model document is public data, so each provenance_record carries all seven §20.2 fields
    with a public disposition (public_permissive, never rejected or quarantined). Committed fixtures stay invented
    (F-PIP-1)."""
    seen = 0
    for relative, pointer, document in committed_documents():
        for collection in PROVENANCE_RECORD_COLLECTIONS:
            records = document.get(collection)
            for index, record in enumerate(records if isinstance(records, list) else []):
                provenance = record.get("provenance_record") if isinstance(record, dict) else None
                if provenance is None:
                    continue
                seen += 1
                label = f"{relative}#{pointer}/{collection}/{index}/provenance_record"
                missing = [field for field in PUBLIC_PROVENANCE_FIELDS
                           if not (isinstance(provenance.get(field), str) and provenance[field].strip())]
                assert not missing, (label, missing)
                assert provenance["redistribution_status"] == "public_permissive", label
                assert provenance["review_status"] not in {"rejected", "quarantined"}, label
    assert seen >= len(PROVENANCE_RECORD_COLLECTIONS)


def test_the_canonical_load_bindings_name_defined_schema_records():
    """§4 "Pointer strings": the primitive_loads model bindings resolve in the committed schemas (ModelResultValue
    names the results schema's QuantityResult, the result value its consumers bind)."""
    source = (ROOT / "core" / "loads" / "primitive_loads" / "src" / "lib.rs").read_text(encoding="utf-8")
    bindings = {}
    for variant in ("ModelLoadCase", "ModelLoadRecord", "ModelResultValue"):
        marker = f"Self::{variant} => \""
        start = source.index(marker) + len(marker)
        bindings[variant] = source[start:source.index('"', start)]
    assert bindings["ModelResultValue"] == "schemas/results.v0.3.schema.yaml#/$defs/QuantityResult"
    for variant, reference in bindings.items():
        file, _, pointer = reference.partition("#")
        node = strict_json(ROOT / file)
        for part in pointer.strip("/").split("/"):
            assert isinstance(node, dict) and part in node, (variant, reference)
            node = node[part]
        assert node.get("type") == "object" and node.get("additionalProperties") is False, (variant, reference)


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
    # The 0.1.0 frozen requests (validation/references/kd5_nonlinear) are unbound (UNBOUND_FROZEN_INPUTS). No frozen
    # 0.6.0 model document exists under validation/**: the t4_u4b and t4_u5b references are case descriptions that
    # the builders author requests from, and the committed 0.6.0 requests elsewhere were re-authored to 0.7.0.
    assert {version for _, version in frozen} == RETIRED_VERSIONS - {"0.1.0", "0.6.0"}
    assert not any(relative.startswith(FROZEN_ROOT) for bucket, relative, _, _ in classify() if bucket == "retired")
    assert any(relative.endswith(".jsonl") for relative, _ in frozen)



DESKTOP_EMISSION_PATHS = [
    "blank_project", "blank_load_reference_project", "node_creation", "material_creation", "section_creation",
    "pipe_creation", "support_creation", "load_creation", "combination_creation", "project_tags_and_notes",
    "entity_tags_and_notes", "provenance_records", "line", "assumptions_and_open_issues",
    "constraints_and_equipment_interfaces", "rule_pack_ref", "library_material_insert",
]


def test_tp_pmm_guiemit_001_desktop_emission_golden_validates_against_the_desktop_profile():
    """TP-PMM-GUIEMIT-001 (§5.2): every document the desktop emission golden records — one per authoring path,
    written by `desktopEmission.golden.test.ts` under a fixed clock — validates against `$defs/DesktopDocument`
    and against the format root, and the paths cover the blank projects, entity creation and every §3 slot."""
    golden = strict_json(ROOT / DESKTOP_EMISSION_GOLDEN)
    assert golden["document_kind"] == "openpipestress.test.desktop_emission_golden"
    assert [entry["path"] for entry in golden["entries"]] == DESKTOP_EMISSION_PATHS
    rows = {(relative, pointer): bucket for bucket, relative, pointer, _ in classify() if relative == DESKTOP_EMISSION_GOLDEN}
    assert set(rows.values()) == {"desktop"}
    assert len(rows) == len(DESKTOP_EMISSION_PATHS)
    desktop, root = desktop_schema(), format_schema()
    for index, entry in enumerate(golden["entries"]):
        document = entry["document"]
        assert document["schema_version"] == CURRENT_VERSION, entry["path"]
        _validate(desktop, document, f"{DESKTOP_EMISSION_GOLDEN}#/entries/{index}/document (DesktopDocument)")
        _validate(root, document, f"{DESKTOP_EMISSION_GOLDEN}#/entries/{index}/document")
    last = golden["entries"][-1]["document"]
    for member in ("lines", "unresolved_assumptions", "constraints", "equipment_interfaces", "rule_pack_refs"):
        assert last[member], member
    assert "tags" in last["project"] and "notes" in last["project"]
    assert any("provenance_record" in record for record in last["materials"])
    assert any("provenance_record" in record for record in last["supports"])


APPLIER_EMISSION_KINDS = [
    "create_line", "update_line", "delete_line",
    "create_assumption", "update_assumption", "delete_assumption",
    "create_constraint", "update_constraint", "delete_constraint",
    "create_equipment_interface", "update_equipment_interface", "delete_equipment_interface",
    "add_rule_pack_ref", "remove_rule_pack_ref",
    "tags", "notes", "provenance_record",
]


def test_applier_model_data_golden_validates_against_the_format():
    """§5.2: the Rust applier golden (`tests/model_data_authoring.rs`) records one applied document per new
    operation kind; each validates against the format root."""
    golden = strict_json(ROOT / APPLIER_EMISSION_GOLDEN)
    assert golden["document_kind"] == "openpipestress.test.applier_model_data_golden"
    assert [entry["operation"] for entry in golden["entries"]] == APPLIER_EMISSION_KINDS
    buckets = {bucket for bucket, relative, _, _ in classify() if relative == APPLIER_EMISSION_GOLDEN}
    assert buckets == {"format"}
    root = format_schema()
    for index, entry in enumerate(golden["entries"]):
        document = entry["applied_model"]
        assert document["schema_version"] == CURRENT_VERSION, entry["operation"]
        _validate(root, document, f"{APPLIER_EMISSION_GOLDEN}#/entries/{index}/applied_model")


def test_shape_corpus_does_not_silently_enter_the_current_format_population():
    # Shape-only and deliberate malformed cases must have an explicit reason
    # in the document population rules; otherwise the format scan treats them
    # as claims of current-format conformance.
    rows = [row for row in classify() if row[1] == "fixtures/model_document/desktop_shape_cases.json"]
    assert rows, "the shared shape corpus must be discovered"
    schema = format_schema()
    for bucket, relative, pointer, document in rows:
        if bucket in {"format", "desktop"}:
            _validate(schema, document, f"{relative}#{pointer}")


def test_an_explicitly_controlled_retired_version_admits_only_its_listed_controls(monkeypatch):
    """W-15 slice c guard preparation: once a retired version is listed, an unlisted
    non-frozen document at it is not a refusal control."""
    version = sorted(RETIRED_VERSIONS)[-1]
    assert is_retired_refusal_control(version, "fixtures/any.json")
    monkeypatch.setitem(EXPLICIT_RETIRED_CONTROLS, version, {"fixtures/listed.json"})
    assert is_retired_refusal_control(version, "fixtures/listed.json")
    assert not is_retired_refusal_control(version, "fixtures/unswept.json")
    assert not is_retired_refusal_control(CURRENT_VERSION, "fixtures/listed.json")
