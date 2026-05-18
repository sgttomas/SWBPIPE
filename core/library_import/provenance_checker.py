"""Library import provenance checks for OpenPipeStress public/private data.

This module validates already-parsed material, section, and component library
payloads. It does not parse external file formats and does not make legal
license determinations.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any


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

RECORD_KEYS = {
    "material": ("material_library", "material_records"),
    "section": ("section_library", "section_records"),
    "component": ("component_library", "component_records"),
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

    library_key, records_key = RECORD_KEYS[library_kind]
    findings: list[ImportFinding] = []

    library = payload.get(library_key)
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

    redistribution_status = item.get("redistribution_status") or provenance.get(
        "redistribution_status"
    )
    review_status = item.get("review_status") or provenance.get("review_status")
    privacy_class = item.get("privacy_class")

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
