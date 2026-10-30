"""Library import provenance checks for OpenPipeStress public/private data.

This module validates already-parsed material, section, and component library
payloads. It does not parse external file formats and does not make legal
license determinations.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any
import json
import math
import sys
from pathlib import Path
from functools import lru_cache


REQUIRED_PROVENANCE_FIELDS = {
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
}

PUBLIC_OK_REDIS_STATUS = "public_permissive"
PRIVATE_REDIS_STATUSES = {"private_only"}
UNKNOWN_REDIS_STATUSES = {"unknown", "TBD", None}
PROTECTED_REDIS_STATUS = "protected_suspected"

# A disposition member no verdict can read (W-17 object form): a
# redistribution_status or review_status holding an array or an object, or a
# provenance written as an array, anywhere in the library metadata object or a
# record. The same code as the desktop's export disposition screen and library
# apply; the import refuses it (blocking). Mirrors the Rust port's
# DISPOSITION_RECORD_MALFORMED.
DISPOSITION_RECORD_MALFORMED = "MODEL-DISPOSITION-RECORD-MALFORMED"

RECORD_KEYS = {
    "hanger": ("hanger_library", "hanger_records"),
    "material": ("material_library", "material_records"),
    "section": ("section_library", "section_records"),
    "component": ("component_library", "component_records"),
}

RECORD_ID_KEYS = {
    "hanger": "hanger_id",
    "material": "material_id",
    "section": "section_id",
    "component": "component_id",
}


@dataclass(frozen=True)
class ImportFinding:
    code: str
    severity: str
    path: str
    message: str
    remediation: str
    diagnostic_class: str = "import_boundary"
    source: str = "core.library_import.provenance_checker"
    affected_object: str | None = None
    provenance: dict[str, str] | None = None

    def to_diagnostic(self) -> dict[str, Any]:
        """Map import findings into the PKG-02 diagnostic envelope shape."""

        return {
            "code": self.code,
            "severity": self.severity,
            "class": self.diagnostic_class,
            "source": self.source,
            "affected_object": self.affected_object or self.path,
            "message": self.message,
            "remediation": self.remediation,
            "provenance": self.provenance
            or {
                "source_name": "library_import_payload",
                "source_location": self.path,
                "review_status": "diagnostic_generated",
            },
        }


@dataclass(frozen=True)
class ImportValidationResult:
    outcome: str
    findings: tuple[ImportFinding, ...]

    @property
    def accepted(self) -> bool:
        return self.outcome in {"ACCEPTED_PUBLIC", "PRIVATE_LOCAL_ONLY"}

    @property
    def diagnostics(self) -> tuple[dict[str, Any], ...]:
        return tuple(finding.to_diagnostic() for finding in self.findings)


def validate_library_import(
    payload: dict[str, Any],
    *,
    library_kind: str,
    intended_visibility: str,
) -> ImportValidationResult:
    """Validate provenance and redistribution controls on a library payload.

    `intended_visibility` is either `public` or `private`. Public imports must
    carry accepted public-permissive provenance before they can be accepted.
    Private imports may remain local/private, but still cannot bypass protected
    content quarantine or missing provenance diagnostics.
    """

    if library_kind not in RECORD_KEYS:
        raise ValueError(f"unsupported library_kind: {library_kind}")
    if intended_visibility not in {"public", "private"}:
        raise ValueError(f"unsupported intended_visibility: {intended_visibility}")

    if library_kind == "hanger":
        return _validate_hanger_import(payload, intended_visibility)

    library_key, records_key = RECORD_KEYS[library_kind]
    findings: list[ImportFinding] = []

    library = payload.get(library_key)
    library_label = _library_label(payload, library_kind)
    if not isinstance(library, dict):
        findings.append(
            ImportFinding(
                "IMPORT_LIBRARY_METADATA_MISSING",
                "blocking",
                library_key,
                "Library metadata object is missing.",
                "Provide library metadata with provenance before import.",
            )
        )
    else:
        findings.extend(
            _validate_object_disposition(
                library,
                path=library_key,
                intended_visibility=intended_visibility,
            )
        )
        findings.extend(_unreadable_dispositions(library, library_key, "T" + library_label[1:]))

    records = payload.get(records_key)
    if not isinstance(records, list):
        findings.append(
            ImportFinding(
                "IMPORT_RECORD_SET_MISSING",
                "blocking",
                records_key,
                "Library record array is missing.",
                "Provide records for the declared library kind before import.",
            )
        )
        records = []

    for index, record in enumerate(records):
        if not isinstance(record, dict):
            findings.append(
                ImportFinding(
                    "IMPORT_RECORD_INVALID",
                    "blocking",
                    f"{records_key}[{index}]",
                    "Library record is not an object.",
                    "Provide an object with provenance and review metadata.",
                )
            )
            continue
        record_path = f"{records_key}[{index}]"
        findings.extend(
            _validate_object_disposition(
                record,
                path=record_path,
                intended_visibility=intended_visibility,
            )
        )
        findings.extend(_validate_nested_values(record, path=record_path))
        subject = _record_label(record, library_kind, record_path, library_label)
        findings.extend(_unreadable_dispositions(record, record_path, subject))

    return ImportValidationResult(
        outcome=_determine_outcome(findings, intended_visibility),
        findings=tuple(findings),
    )


def _validate_object_disposition(
    item: dict[str, Any], *, path: str, intended_visibility: str
) -> list[ImportFinding]:
    findings: list[ImportFinding] = []
    provenance = item.get("provenance")
    if not isinstance(provenance, dict):
        findings.append(
            ImportFinding(
                "IMPORT_PROVENANCE_MISSING",
                "blocking",
                f"{path}.provenance",
                "Required provenance object is missing.",
                "Record source, license, contributor, redistribution, and review metadata.",
            )
        )
        return findings

    missing = sorted(field for field in REQUIRED_PROVENANCE_FIELDS if not provenance.get(field))
    if missing:
        findings.append(
            ImportFinding(
                "IMPORT_PROVENANCE_INCOMPLETE",
                "blocking",
                f"{path}.provenance",
                f"Required provenance fields are missing: {', '.join(missing)}.",
                "Complete required provenance fields before import acceptance.",
            )
        )

    # The item's own status wins when it is non-empty text, else its
    # provenance's (the Rust port's `truthy_str` fallback): a status that is
    # not text states no disposition here. One that is an array or an object
    # is refused by name (_unreadable_dispositions).
    redistribution_status = _status_text(item.get("redistribution_status")) or _status_text(
        provenance.get("redistribution_status")
    )
    review_status = _status_text(item.get("review_status")) or _status_text(
        provenance.get("review_status")
    )
    privacy_class = _status_text(item.get("privacy_class"))

    if redistribution_status == PROTECTED_REDIS_STATUS or review_status == "quarantined":
        findings.append(
            ImportFinding(
                "IMPORT_PROTECTED_CONTENT_SUSPECTED",
                "quarantine",
                path,
                "Import metadata indicates suspected protected content.",
                "Quarantine metadata and request human/legal review; do not publish values.",
            )
        )
    elif redistribution_status == "rejected" or review_status == "rejected":
        findings.append(
            ImportFinding(
                "IMPORT_REJECTED_SOURCE",
                "blocking",
                path,
                "Import metadata has a rejected source or review disposition.",
                "Reject this import or supply a reviewed source.",
            )
        )
    elif intended_visibility == "public":
        findings.extend(
            _validate_public_disposition(
                path=path,
                redistribution_status=redistribution_status,
                review_status=review_status,
                privacy_class=privacy_class,
            )
        )

    return findings


def _status_text(value: Any) -> str | None:
    """A non-empty text status, else None (the Rust port's `truthy_str`)."""
    return value if isinstance(value, str) and value else None


def _validate_public_disposition(
    *,
    path: str,
    redistribution_status: Any,
    review_status: Any,
    privacy_class: Any,
) -> list[ImportFinding]:
    findings: list[ImportFinding] = []
    if redistribution_status in PRIVATE_REDIS_STATUSES or str(privacy_class).startswith(
        "private"
    ):
        findings.append(
            ImportFinding(
                "IMPORT_PRIVATE_DATA_PUBLIC_BLOCKED",
                "blocking",
                path,
                "Private-only library data cannot be accepted as public data.",
                "Keep the import private or provide public-permissive reviewed provenance.",
            )
        )
    elif redistribution_status in UNKNOWN_REDIS_STATUSES:
        findings.append(
            ImportFinding(
                "IMPORT_REDIS_RIGHTS_MISSING",
                "blocking",
                path,
                "Redistribution rights are missing or unresolved for public import.",
                "Record public-permissive redistribution evidence and review disposition.",
            )
        )
    elif redistribution_status != PUBLIC_OK_REDIS_STATUS:
        findings.append(
            ImportFinding(
                "IMPORT_REDIS_RIGHTS_UNACCEPTED",
                "blocking",
                path,
                "Redistribution status is not accepted for public import.",
                "Use public-permissive reviewed data or keep the data private.",
            )
        )

    if review_status != "accepted":
        findings.append(
            ImportFinding(
                "IMPORT_REVIEW_REQUIRED",
                "review_required",
                path,
                "Public import requires an accepted review disposition.",
                "Record maintainer review before accepting public data.",
            )
        )
    return findings


def _validate_nested_values(item: dict[str, Any], *, path: str) -> list[ImportFinding]:
    findings: list[ImportFinding] = []
    for nested_path, value in _walk_value_objects(item, path):
        if "magnitude" not in value:
            continue
        if not _has_unit_metadata(value):
            findings.append(
                ImportFinding(
                    "IMPORT_UNIT_METADATA_MISSING",
                    "blocking",
                    nested_path,
                    "Unit-bearing imported value is missing unit or dimension metadata.",
                    "Carry unit and dimension metadata through the import boundary.",
                )
            )
        if not isinstance(value.get("provenance"), dict):
            findings.append(
                ImportFinding(
                    "IMPORT_VALUE_PROVENANCE_MISSING",
                    "blocking",
                    nested_path,
                    "Imported value is missing value-level provenance.",
                    "Record value-level provenance for imported numeric data.",
                )
            )
    return findings


def _library_label(payload: Any, library_kind: str) -> str:
    """The library as an unreadable-disposition finding names it (Rust `library_label`)."""
    library_key, _ = RECORD_KEYS[library_kind]
    library = payload.get(library_key) if isinstance(payload, dict) else None
    identity = library.get("library_id") if isinstance(library, dict) else None
    if isinstance(identity, str) and identity:
        return f"the {library_kind} library `{identity}`"
    return f"the {library_kind} library (no library_id)"


def _record_label(record: dict[str, Any], library_kind: str, path: str, library: str) -> str:
    """A record as an unreadable-disposition finding names it (Rust `record_label`)."""
    identity = record.get(RECORD_ID_KEYS[library_kind])
    if isinstance(identity, str) and identity:
        return f"Record `{identity}` of {library}"
    return f"Record {path} of {library}"


def _unreadable_dispositions(item: dict[str, Any], path: str, subject: str) -> list[ImportFinding]:
    """Every disposition member of `item`, at any depth, no verdict can read.

    A redistribution_status or review_status holding an array or an object, or
    a provenance written as an array (the desktop export screen's
    StoredMembers::Refuse rule), is refused by name rather than skipped. Other
    members (classification, the privacy members) and non-text scalars are not
    refused here. Keys are visited in sorted order (Rust
    `unreadable_dispositions`).
    """

    findings: list[ImportFinding] = []

    def walk_object(obj: dict[str, Any], relative: str) -> None:
        for key in sorted(obj):
            value = obj[key]
            member = key if not relative else f"{relative}.{key}"
            shape = None
            if key in ("redistribution_status", "review_status") and isinstance(value, dict):
                shape = "an object"
            elif key in ("redistribution_status", "review_status", "provenance") and isinstance(value, list):
                shape = "an array"
            if shape is not None:
                findings.append(
                    ImportFinding(
                        DISPOSITION_RECORD_MALFORMED,
                        "blocking",
                        f"{path}.{member}",
                        f"{subject} holds `{member}` as {shape}, so its disposition cannot be read.",
                        "Write each redistribution_status and review_status as text and each "
                        "provenance as an object, then import the library again.",
                    )
                )
            walk_value(value, member)

    def walk_value(value: Any, relative: str) -> None:
        if isinstance(value, dict):
            walk_object(value, relative)
        elif isinstance(value, list):
            for index, nested in enumerate(value):
                walk_value(nested, f"{relative}[{index}]")

    walk_object(item, "")
    return findings


def _walk_value_objects(value: Any, path: str):
    if isinstance(value, dict):
        yield path, value
        for key, nested in value.items():
            yield from _walk_value_objects(nested, f"{path}.{key}")
    elif isinstance(value, list):
        for index, nested in enumerate(value):
            yield from _walk_value_objects(nested, f"{path}[{index}]")


def _has_unit_metadata(value: dict[str, Any]) -> bool:
    has_material_unit = "unit_ref" in value and "dimension_id" in value
    has_component_unit = "unit" in value and "dimension" in value
    return has_material_unit or has_component_unit


def _determine_outcome(
    findings: list[ImportFinding], intended_visibility: str
) -> str:
    if any(finding.severity == "quarantine" for finding in findings):
        return "QUARANTINE"
    if any(finding.severity == "blocking" for finding in findings):
        return "REJECTED"
    if any(finding.severity == "review_required" for finding in findings):
        return "REVIEW_REQUIRED"
    if intended_visibility == "private":
        return "PRIVATE_LOCAL_ONLY"
    return "ACCEPTED_PUBLIC"


@lru_cache(maxsize=1)
def _hanger_schema():
    return json.loads((Path(__file__).resolve().parents[2] / "schemas/hanger.schema.yaml").read_text())


def _hanger_shape(value, schema, root, path, findings):
    """Interpret only the bounded keyword set used by the embedded hanger schema.

    Unsupported keywords fail closed. This is not a general JSON Schema engine.
    """
    allowed = {"$schema", "$id", "$defs", "title", "description", "$ref", "type",
               "additionalProperties", "required", "properties", "items", "enum",
               "const", "pattern", "exclusiveMinimum"}
    invalid = lambda: findings.append(ImportFinding(
        "IMPORT_HANGER_SCHEMA_INVALID", "blocking", path,
        "Hanger value does not satisfy the declared schema.",
        "Supply explicit supported fields, compatible quantities and complete provenance."))
    if set(schema) - allowed:
        invalid()
        return
    if "$ref" in schema:
        _hanger_shape(value, root["$defs"][schema["$ref"].split("/")[-1]], root, path, findings)
        return
    kind = schema.get("type")
    valid_type = {"object": isinstance(value, dict), "array": isinstance(value, list),
                  "string": isinstance(value, str),
                  "number": type(value) in (int, float) and -sys.float_info.max <= value <= sys.float_info.max and math.isfinite(value)}
    if kind not in valid_type or not valid_type[kind]:
        invalid()
        return
    if (("enum" in schema and value not in schema["enum"])
            or ("const" in schema and value != schema["const"])
            or ("pattern" in schema and (schema["pattern"] != r"\S" or not value.strip()))
            or ("exclusiveMinimum" in schema and value <= schema["exclusiveMinimum"])):
        invalid()
    if kind == "object":
        props = schema.get("properties", {})
        for key in sorted(set(schema.get("required", [])) - value.keys()):
            findings.append(ImportFinding("IMPORT_HANGER_SCHEMA_INVALID", "blocking", f"{path}.{key}",
                "Required hanger field is missing.", "Supply the required field explicitly."))
        for key in sorted(value):
            if key in props:
                _hanger_shape(value[key], props[key], root, f"{path}.{key}", findings)
            elif schema.get("additionalProperties") is False:
                findings.append(ImportFinding("IMPORT_HANGER_SCHEMA_INVALID", "blocking", f"{path}.{key}",
                    "Unknown hanger field.", "Remove unsupported fields."))
    elif kind == "array":
        for i, item in enumerate(value):
            _hanger_shape(item, schema["items"], root, f"{path}[{i}]", findings)


def _validate_hanger_import(payload, visibility):
    findings = []
    schema = _hanger_schema()
    _hanger_shape(payload, schema, schema, "$", findings)
    # Deterministic traversal also examines malformed/unknown wrappers: schema
    # rejection must never conceal protected metadata deeper in the input.
    objects = sorted(_walk_value_objects(payload, "$"), key=lambda pair: pair[0])
    for path, item in objects:
        if "provenance" in item or (not path.endswith(".provenance") and any(
            key in item for key in ("redistribution_status", "review_status", "privacy_class")
        )):
            provenance = item.get("provenance")
            # Legacy item-level precedence must not mask a protected/rejected
            # provenance disposition in this new strict family.
            findings.extend(_hanger_disposition(item, path, visibility))
            if isinstance(provenance, dict) and any(
                key in item and item[key] != provenance.get(key)
                for key in ("redistribution_status", "review_status")
            ):
                findings.extend(_hanger_disposition({"provenance": provenance}, path + ".provenance", visibility))
    # The schema already refuses these shapes; they are also refused by name.
    library = _library_label(payload, "hanger")
    metadata = payload.get("hanger_library") if isinstance(payload, dict) else None
    if isinstance(metadata, dict):
        findings.extend(_unreadable_dispositions(metadata, "$.hanger_library", "T" + library[1:]))
    records = payload.get("hanger_records", []) if isinstance(payload, dict) else []
    if isinstance(records, list):
        for i, record in enumerate(records):
            if isinstance(record, dict):
                path = f"$.hanger_records[{i}]"
                findings.extend(_unreadable_dispositions(record, path, _record_label(record, "hanger", path, library)))
    seen = set()
    if isinstance(records, list):
        for i, record in enumerate(records):
            identity = record.get("hanger_id") if isinstance(record, dict) else None
            if isinstance(identity, str):
                if identity in seen:
                    findings.append(ImportFinding("IMPORT_HANGER_DUPLICATE_ID", "blocking",
                        f"$.hanger_records[{i}].hanger_id", "Duplicate hanger identity.",
                        "Use a unique hanger_id within the library."))
                seen.add(identity)
    return ImportValidationResult(_determine_outcome(findings, visibility), tuple(findings))


def _hanger_disposition(item, path, visibility):
    # Shape findings reject nonstring disposition fields; use the Rust string
    # fallback semantics for diagnostic generation, without mutating input.
    clean = dict(item)
    if clean.get("privacy_class") == "project_private":
        clean["privacy_class"] = "private_project_data"
    provenance = item.get("provenance")
    if isinstance(provenance, dict):
        clean["provenance"] = dict(provenance)
        for key in ("redistribution_status", "review_status"):
            if not isinstance(provenance.get(key), str):
                clean["provenance"][key] = None
    for key in ("redistribution_status", "review_status", "privacy_class"):
        if key in clean and not isinstance(clean[key], str):
            clean[key] = None
    result = _validate_object_disposition(clean, path=path, intended_visibility=visibility)
    if isinstance(provenance, dict):
        result = [f for f in result if f.code != "IMPORT_PROVENANCE_INCOMPLETE"]
        missing = sorted(field for field in REQUIRED_PROVENANCE_FIELDS if not provenance.get(field))
        if missing:
            result.insert(0, ImportFinding("IMPORT_PROVENANCE_INCOMPLETE", "blocking", f"{path}.provenance",
                f"Required provenance fields are missing: {', '.join(missing)}.",
                "Complete required provenance fields before import acceptance."))
    if not isinstance(provenance, dict) and (
        item.get("redistribution_status") == "protected_suspected"
        or item.get("review_status") == "quarantined"
    ):
        result.append(ImportFinding("IMPORT_PROTECTED_CONTENT_SUSPECTED", "quarantine", path,
            "Import metadata indicates suspected protected content.",
            "Quarantine metadata and request human/legal review; do not publish values."))
    return result
