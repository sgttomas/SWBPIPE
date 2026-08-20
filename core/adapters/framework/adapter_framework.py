"""Bounded in-memory adapter framework validation.

The DEL-10-02 framework is deliberately format-neutral. It checks adapter
declarations and invented fixtures for no-bypass controls, provenance, unit
metadata, privacy/export posture, and professional-boundary controls without
parsing real external files or selecting a concrete import/export format.
"""

from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
import re
from typing import Any


REQUIRED_TBD_DECISIONS = {
    "external_format_list",
    "public_transport_protocol",
    "endpoint_syntax",
    "adapter_execution_model",
    "plugin_runtime",
    "permission_grant_persistence",
    "package_scripts",
    "ci_provider",
    "release_matrix",
    "physical_project_container",
    "local_fea_package_format",
    "redaction_workflow",
}

REQUIRED_PROVENANCE_FIELDS = {
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
}

REQUIRED_NO_BYPASS_TRUE = {
    "must_use_public_api_boundary",
    "must_use_unit_validation",
    "must_preserve_provenance",
    "must_preserve_redistribution_review",
    "must_preserve_privacy_classification",
    "must_screen_protected_content",
    "must_preserve_diagnostics",
    "must_preserve_rule_pack_sandbox",
    "must_preserve_persistence_hash_controls",
    "must_route_persistence_through_application_services",
    "must_not_expose_sql_or_raw_sqlite",
    "must_not_expose_table_names",
    "must_not_mutate_project_store_directly",
    "must_preserve_report_controls",
    "must_preserve_human_acceptance_boundary",
    "must_not_execute_arbitrary_code",
    "must_not_access_network",
    "must_not_choose_filesystem_roots",
    "must_not_claim_code_compliance",
    "must_not_transmit_private_data_by_default",
}

REQUIRED_VALIDATION_PLAN = {
    "schema_validation",
    "unit_validation",
    "dimension_validation",
    "provenance_validation",
    "redistribution_review",
    "privacy_classification",
    "protected_content_screening",
}

FORBIDDEN_STATUS_TERMS = {
    "CODE_COMPLIANT",
    "CERTIFIED",
    "SEALED",
    "APPROVED_FOR_PROFESSIONAL_RELIANCE",
    "SECURITY_CERTIFIED",
    "COMPLIANCE_ATTESTED",
}

FORBIDDEN_PERSISTENCE_PATTERNS = {
    r"\braw[_ -]?sqlite\b",
    r"\bsqlite[_ -]?(handle|connection|cursor)?\b",
    r"\bdatabase[_ -]?(handle|connection|cursor)\b",
    r"\bsql[_ -]?(string|query|statement)?\b",
    r"\btable[_ -]?name(s)?\b",
    r"\bdirect[_ -]?(project[_ -]?)?store[_ -]?mutation\b",
    r"\bproject[_ -]?store[_ -]?(handle|mutator)\b",
}

PUBLIC_REVIEWED = "public_permissive_reviewed"


@dataclass(frozen=True)
class AdapterFinding:
    code: str
    severity: str
    path: str
    message: str
    remediation: str


@dataclass(frozen=True)
class AdapterValidationResult:
    outcome: str
    findings: tuple[AdapterFinding, ...]

    @property
    def accepted(self) -> bool:
        return self.outcome == "ACCEPTED_FORMAT_NEUTRAL_DECLARATION"


@dataclass(frozen=True)
class AdapterRuntimeGateResult:
    outcome: str
    declaration_accepted: bool
    runtime_dispatched: bool
    findings: tuple[AdapterFinding, ...]


def gate_adapter_runtime_dispatch(payload: dict[str, Any]) -> AdapterRuntimeGateResult:
    """Gate the selected declaration-to-runtime seam without creating a loader.

    Invalid or quarantined declarations stop at their existing validation
    outcome. A valid declaration also stops because adapter execution and the
    plugin runtime remain owner-held TBDs. There is intentionally no executor
    argument or callback that could bypass this gate.
    """

    validation = validate_adapter_declaration(payload)
    if not validation.accepted:
        return AdapterRuntimeGateResult(
            outcome=validation.outcome,
            declaration_accepted=False,
            runtime_dispatched=False,
            findings=validation.findings,
        )
    runtime_finding = AdapterFinding(
        "ADAPTER_RUNTIME_NOT_SELECTED",
        "blocking",
        "framework_status.adapter_execution_model",
        "The format-neutral declaration passed validation, but no adapter execution model or plugin runtime is selected.",
        "Keep runtime dispatch blocked until a separately governed runtime and capability design is accepted.",
    )
    return AdapterRuntimeGateResult(
        outcome="BLOCKED_RUNTIME_NOT_SELECTED",
        declaration_accepted=True,
        runtime_dispatched=False,
        findings=(runtime_finding,),
    )


def validate_adapter_declaration(payload: dict[str, Any]) -> AdapterValidationResult:
    """Validate a format-neutral adapter declaration payload."""

    findings: list[AdapterFinding] = []
    findings.extend(_validate_identity(payload))
    findings.extend(_validate_framework_status(payload.get("framework_status")))
    findings.extend(_validate_tbd_decisions(payload.get("tbd_decisions")))
    findings.extend(
        _validate_adapter(payload.get("adapter_declaration"), "adapter_declaration")
    )
    findings.extend(_validate_validation_plan(payload.get("validation_plan")))
    findings.extend(_validate_operation_result(payload.get("operation_result")))
    findings.extend(_scan_forbidden_terms(payload))
    findings.extend(_scan_forbidden_persistence_access(payload))

    return AdapterValidationResult(
        outcome=_determine_outcome(findings),
        findings=tuple(findings),
    )


def build_result(
    *,
    operation_id: str,
    operation_class: str,
    diagnostics: tuple[AdapterFinding, ...],
    diagnostic_contexts: tuple[Mapping[str, Any], ...] | None = None,
    privacy_context: Mapping[str, Any] | None = None,
    provenance: Any = None,
) -> dict[str, Any]:
    """Build a deterministic operation result envelope for tests and callers."""

    if diagnostic_contexts is None:
        diagnostic_contexts = tuple(
            {
                "source": {"ref_type": "adapter", "ref_id": "ops.adapter.framework"},
                "affected_object": {
                    "ref_type": "diagnostic",
                    "ref_id": finding.path,
                },
                "provenance": None,
            }
            for finding in diagnostics
        )
    if len(diagnostic_contexts) != len(diagnostics):
        raise ValueError("diagnostic_contexts must align one-to-one with diagnostics")

    return {
        "operation_id": operation_id,
        "operation_class": operation_class,
        "parse_status": "not_parsed_by_framework",
        "validation_plan_ref": {
            "ref_type": "schema",
            "ref_id": "schemas/adapter_framework.schema.yaml#/$defs/ValidationPlan",
        },
        "diagnostics": [
            {
                "code": finding.code,
                "class": _diagnostic_class(finding),
                "severity": finding.severity,
                "source": dict(context["source"]),
                "affected_object": dict(context["affected_object"]),
                "message": finding.message,
                "remediation": finding.remediation,
                "provenance": _diagnostic_provenance(context.get("provenance")),
            }
            for finding, context in zip(diagnostics, diagnostic_contexts, strict=True)
        ],
        "privacy": _result_privacy(privacy_context),
        "provenance": _diagnostic_provenance(provenance),
        "checksums": [],
        "audit_manifest_refs": [],
        "result_envelope_ref": {
            "schema_ref": "schemas/results.schema.yaml",
            "compatibility": "schema_first_json_result_envelope",
            "ref": {"ref_type": "result_envelope", "ref_id": "TBD"},
        },
        "professional_boundary": professional_boundary(),
    }


def _result_privacy(privacy: Mapping[str, Any] | None) -> dict[str, Any]:
    """Return a schema-valid, local-first context without inventing clearance."""

    source = privacy if isinstance(privacy, Mapping) else {}
    classification = source.get("classification")
    if classification not in {
        PUBLIC_REVIEWED,
        "private_local_only",
        "protected_suspected",
        "export_review_required",
        "TBD",
    }:
        classification = "TBD"
    return {
        "classification": classification,
        "local_first": True,
        "telemetry_allowed": False,
        "export_review_required": source.get("export_review_required") is not False,
        "private_payload_redacted": source.get("private_payload_redacted") is True,
    }


def _diagnostic_provenance(provenance: Any) -> dict[str, str]:
    """Preserve canonical input provenance and fail closed without invention."""

    fields = (
        "source_name",
        "source_location",
        "source_license",
        "contributor",
        "contributor_certification",
        "redistribution_status",
        "review_status",
    )
    source = provenance if isinstance(provenance, Mapping) else {}
    complete = all(
        isinstance(source.get(field), str) and source[field].strip()
        for field in fields
    )
    result = {
        field: value if isinstance((value := source.get(field)), str) and value.strip() else "TBD"
        for field in fields
    }
    valid_redistribution = {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "TBD",
    }
    valid_review = {"accepted", "needs_review", "quarantined", "rejected", "TBD"}
    if result["redistribution_status"] not in valid_redistribution:
        result["redistribution_status"] = "TBD"
        complete = False
    if result["review_status"] not in valid_review:
        result["review_status"] = "TBD"
        complete = False
    quarantine_marker = (
        result["redistribution_status"] == "protected_suspected"
        or result["review_status"] == "quarantined"
    )
    if not complete and not quarantine_marker:
        result["review_status"] = "rejected"
    return result


def invented_provenance() -> dict[str, str]:
    return {
        "source_name": "Invented adapter framework fixture",
        "source_location": "fixtures/adapters/invented/invented_adapter_framework.json",
        "source_license": "project-invented-test-data",
        "contributor": "OpenPipeStress",
        "contributor_certification": "invented non-engineering fixture",
        "redistribution_status": "public_permissive",
        "review_status": "accepted",
    }


def professional_boundary() -> dict[str, bool]:
    return {
        "human_review_required": True,
        "mechanics_solve_distinct": True,
        "user_rule_check_distinct": True,
        "software_makes_compliance_claim": False,
        "software_makes_certification_claim": False,
        "software_makes_sealing_claim": False,
        "software_makes_approval_claim": False,
        "software_makes_security_certification_claim": False,
    }


def _validate_identity(payload: dict[str, Any]) -> list[AdapterFinding]:
    findings: list[AdapterFinding] = []
    expected = {
        "deliverable_id": "DEL-10-02",
        "package_id": "PKG-10",
        "scope_item": "SOW-030",
        "objective": "OBJ-009",
    }
    for key, value in expected.items():
        if payload.get(key) != value:
            findings.append(
                AdapterFinding(
                    "ADAPTER_TRACEABILITY_INVALID",
                    "blocking",
                    key,
                    f"{key} must be {value}.",
                    "Restore DEL-10-02 traceability metadata.",
                )
            )
    return findings


def _validate_framework_status(status: Any) -> list[AdapterFinding]:
    if not isinstance(status, dict):
        return [
            AdapterFinding(
                "ADAPTER_FRAMEWORK_STATUS_MISSING",
                "blocking",
                "framework_status",
                "Framework status object is missing.",
                "Provide framework status with unresolved choices held at TBD.",
            )
        ]

    findings: list[AdapterFinding] = []
    for key in REQUIRED_TBD_DECISIONS - {"package_scripts"}:
        if status.get(key) != "TBD":
            findings.append(
                AdapterFinding(
                    "ADAPTER_RUNTIME_DECISION_NOT_TBD",
                    "blocking",
                    f"framework_status.{key}",
                    f"{key} must remain TBD in DEL-10-02.",
                    "Do not select concrete runtime, transport, format, CI, release, or packaging behavior in this deliverable.",
                )
            )
    if status.get("interface_kind") != "schema_first_format_neutral_adapter_framework":
        findings.append(
            AdapterFinding(
                "ADAPTER_INTERFACE_KIND_INVALID",
                "blocking",
                "framework_status.interface_kind",
                "Interface kind must remain schema-first and format-neutral.",
                "Use schema_first_format_neutral_adapter_framework.",
            )
        )
    return findings


def _validate_tbd_decisions(decisions: Any) -> list[AdapterFinding]:
    if not isinstance(decisions, dict):
        return [
            AdapterFinding(
                "ADAPTER_TBD_DECISIONS_MISSING",
                "blocking",
                "tbd_decisions",
                "TBD decisions object is missing.",
                "Record unresolved decisions explicitly as TBD.",
            )
        ]

    findings: list[AdapterFinding] = []
    missing = sorted(REQUIRED_TBD_DECISIONS - set(decisions))
    if missing:
        findings.append(
            AdapterFinding(
                "ADAPTER_TBD_DECISIONS_INCOMPLETE",
                "blocking",
                "tbd_decisions",
                f"TBD decision keys are missing: {', '.join(missing)}.",
                "Add all unresolved adapter/framework decisions.",
            )
        )
    for key in REQUIRED_TBD_DECISIONS & set(decisions):
        if decisions.get(key) != "TBD":
            findings.append(
                AdapterFinding(
                    "ADAPTER_DECISION_PREMATURE",
                    "blocking",
                    f"tbd_decisions.{key}",
                    f"{key} was resolved without a separate human ruling.",
                    "Keep this decision as TBD in DEL-10-02.",
                )
            )
    return findings


def _validate_adapter(adapter: Any, path: str) -> list[AdapterFinding]:
    if not isinstance(adapter, dict):
        return [
            AdapterFinding(
                "ADAPTER_DECLARATION_MISSING",
                "blocking",
                path,
                "Adapter declaration object is missing.",
                "Provide a format-neutral adapter declaration.",
            )
        ]

    findings: list[AdapterFinding] = []
    if adapter.get("format_status") != "TBD":
        findings.append(
            AdapterFinding(
                "ADAPTER_FORMAT_SELECTED",
                "blocking",
                f"{path}.format_status",
                "Concrete external formats must remain TBD.",
                "Do not select external formats in DEL-10-02.",
            )
        )
    capabilities_malformed = False
    try:
        capabilities = set(adapter.get("capabilities", ()))
    except (TypeError, ValueError):
        capabilities = set()
        capabilities_malformed = True
    if capabilities_malformed:
        findings.append(
            AdapterFinding(
                "ADAPTER_CAPABILITIES_MALFORMED",
                "blocking",
                f"{path}.capabilities",
                "Adapter capabilities are malformed.",
                "Declare bounded format-neutral capabilities as hashable values.",
            )
        )
    elif not capabilities:
        findings.append(
            AdapterFinding(
                "ADAPTER_CAPABILITIES_MISSING",
                "blocking",
                f"{path}.capabilities",
                "Adapter capabilities are missing.",
                "Declare bounded format-neutral capabilities.",
            )
        )
    findings.extend(_validate_provenance(adapter.get("provenance"), f"{path}.provenance"))
    findings.extend(_validate_privacy(adapter.get("privacy"), f"{path}.privacy"))
    findings.extend(
        _validate_no_bypass(adapter.get("no_bypass_controls"), f"{path}.no_bypass_controls")
    )
    findings.extend(
        _validate_professional_boundary(
            adapter.get("professional_boundary"),
            f"{path}.professional_boundary",
        )
    )
    return findings


def _validate_validation_plan(plan: Any) -> list[AdapterFinding]:
    if not isinstance(plan, dict):
        return [
            AdapterFinding(
                "ADAPTER_VALIDATION_PLAN_MISSING",
                "blocking",
                "validation_plan",
                "Validation plan is missing.",
                "Provide the mandatory adapter validation plan.",
            )
        ]

    findings: list[AdapterFinding] = []
    for key in REQUIRED_VALIDATION_PLAN:
        if plan.get(key) != "required":
            findings.append(
                AdapterFinding(
                    "ADAPTER_VALIDATION_HOOK_MISSING",
                    "blocking",
                    f"validation_plan.{key}",
                    f"{key} must be required.",
                    "Adapters cannot bypass validation, provenance, privacy, or protected-content hooks.",
                )
            )
    if plan.get("human_review_required") is not True:
        findings.append(
            AdapterFinding(
                "ADAPTER_HUMAN_REVIEW_BOUNDARY_MISSING",
                "blocking",
                "validation_plan.human_review_required",
                "Adapter workflow must preserve human review boundary.",
                "Set human_review_required to true.",
            )
        )
    return findings


def _validate_operation_result(result: Any) -> list[AdapterFinding]:
    if not isinstance(result, dict):
        return [
            AdapterFinding(
                "ADAPTER_RESULT_MISSING",
                "blocking",
                "operation_result",
                "Operation result object is missing.",
                "Provide a deterministic adapter operation result envelope.",
            )
        ]

    findings: list[AdapterFinding] = []
    if result.get("parse_status") != "not_parsed_by_framework":
        findings.append(
            AdapterFinding(
                "ADAPTER_PARSE_BOUNDARY_VIOLATED",
                "blocking",
                "operation_result.parse_status",
                "The framework must not parse real external files.",
                "Use not_parsed_by_framework for DEL-10-02.",
            )
        )
    findings.extend(_validate_privacy(result.get("privacy"), "operation_result.privacy"))
    findings.extend(
        _validate_provenance(result.get("provenance"), "operation_result.provenance")
    )
    findings.extend(
        _validate_professional_boundary(
            result.get("professional_boundary"),
            "operation_result.professional_boundary",
        )
    )
    return findings


def _validate_provenance(provenance: Any, path: str) -> list[AdapterFinding]:
    if not isinstance(provenance, dict):
        return [
            AdapterFinding(
                "ADAPTER_PROVENANCE_MISSING",
                "blocking",
                path,
                "Required provenance object is missing.",
                "Record source, license, contributor, redistribution, and review metadata.",
            )
        ]
    redistribution = provenance.get("redistribution_status")
    review = provenance.get("review_status")
    if redistribution == "protected_suspected" or review == "quarantined":
        return [
            AdapterFinding(
                "ADAPTER_PROTECTED_CONTENT_SUSPECTED",
                "quarantine",
                path,
                "Adapter provenance indicates suspected protected content.",
                "Quarantine the payload and request human/legal review.",
            )
        ]
    missing = sorted(field for field in REQUIRED_PROVENANCE_FIELDS if not provenance.get(field))
    if missing:
        return [
            AdapterFinding(
                "ADAPTER_PROVENANCE_INCOMPLETE",
                "blocking",
                path,
                f"Required provenance fields are missing: {', '.join(missing)}.",
                "Complete provenance before adapter declaration acceptance.",
            )
        ]
    if redistribution not in {
        "public_permissive",
        "private_only",
        "unknown",
        "protected_suspected",
        "TBD",
    } or review not in {"accepted", "needs_review", "quarantined", "rejected", "TBD"}:
        return [
            AdapterFinding(
                "ADAPTER_PROVENANCE_INVALID",
                "blocking",
                path,
                "Adapter provenance contains an invalid redistribution or review status.",
                "Use canonical provenance status values before adapter declaration acceptance.",
            )
        ]
    return []


def _validate_privacy(privacy: Any, path: str) -> list[AdapterFinding]:
    if not isinstance(privacy, dict):
        return [
            AdapterFinding(
                "ADAPTER_PRIVACY_CONTEXT_MISSING",
                "blocking",
                path,
                "Privacy context is missing.",
                "Provide local-first privacy classification.",
            )
        ]
    findings: list[AdapterFinding] = []
    if privacy.get("classification") not in {
        PUBLIC_REVIEWED,
        "private_local_only",
        "protected_suspected",
        "export_review_required",
        "TBD",
    }:
        findings.append(
            AdapterFinding(
                "ADAPTER_PRIVACY_CLASSIFICATION_INVALID",
                "blocking",
                f"{path}.classification",
                "Privacy classification is missing or invalid.",
                "Use a canonical privacy classification.",
            )
        )
    if privacy.get("local_first") is not True:
        findings.append(
            AdapterFinding(
                "ADAPTER_LOCAL_FIRST_REQUIRED",
                "blocking",
                f"{path}.local_first",
                "Adapter payloads must preserve local-first posture.",
                "Set local_first to true.",
            )
        )
    if privacy.get("telemetry_allowed") is not False:
        findings.append(
            AdapterFinding(
                "ADAPTER_TELEMETRY_MUST_BE_DISABLED",
                "blocking",
                f"{path}.telemetry_allowed",
                "Telemetry must not carry adapter payloads by default.",
                "Set telemetry_allowed to false.",
            )
        )
    for field in ("export_review_required", "private_payload_redacted"):
        if not isinstance(privacy.get(field), bool):
            findings.append(
                AdapterFinding(
                    "ADAPTER_PRIVACY_FIELD_INVALID",
                    "blocking",
                    f"{path}.{field}",
                    f"{field} must be an explicit boolean.",
                    "Provide the complete canonical privacy context.",
                )
            )
    return findings


def _validate_no_bypass(controls: Any, path: str) -> list[AdapterFinding]:
    if not isinstance(controls, dict):
        return [
            AdapterFinding(
                "ADAPTER_NO_BYPASS_CONTROLS_MISSING",
                "blocking",
                path,
                "No-bypass controls are missing.",
                "Provide all mandatory no-bypass controls.",
            )
        ]
    findings: list[AdapterFinding] = []
    for key in REQUIRED_NO_BYPASS_TRUE:
        if controls.get(key) is not True:
            findings.append(
                AdapterFinding(
                    "ADAPTER_NO_BYPASS_CONTROL_DISABLED",
                    "blocking",
                    f"{path}.{key}",
                    f"{key} must be true.",
                    "Adapters cannot bypass validation, privacy, provenance, diagnostics, or human boundaries.",
                )
            )
    return findings


def _validate_professional_boundary(boundary: Any, path: str) -> list[AdapterFinding]:
    if not isinstance(boundary, dict):
        return [
            AdapterFinding(
                "ADAPTER_PROFESSIONAL_BOUNDARY_MISSING",
                "blocking",
                path,
                "Professional-boundary controls are missing.",
                "Record no-claim and human-review controls.",
            )
        ]
    findings: list[AdapterFinding] = []
    for key, expected in professional_boundary().items():
        if boundary.get(key) is not expected:
            findings.append(
                AdapterFinding(
                    "ADAPTER_AUTHORITY_BOUNDARY_VIOLATED",
                    "blocking",
                    f"{path}.{key}",
                    f"{key} must be {expected}.",
                    "Adapter outputs must remain decision-support artifacts.",
                )
            )
    return findings


def _scan_forbidden_terms(payload: dict[str, Any]) -> list[AdapterFinding]:
    text = repr(payload).upper()
    matches = sorted(term for term in FORBIDDEN_STATUS_TERMS if term in text)
    if not matches:
        return []
    return [
        AdapterFinding(
            "ADAPTER_FORBIDDEN_STATUS_TERM",
            "blocking",
            "payload",
            f"Forbidden authority terms found: {', '.join(matches)}.",
            "Remove software-generated compliance, certification, approval, or attestation terms.",
        )
    ]


def _scan_forbidden_persistence_access(payload: dict[str, Any]) -> list[AdapterFinding]:
    text = repr(payload).lower()
    matches = sorted(
        pattern
        for pattern in FORBIDDEN_PERSISTENCE_PATTERNS
        if re.search(pattern, text)
    )
    if not matches:
        return []
    return [
        AdapterFinding(
            "ADAPTER_FORBIDDEN_PERSISTENCE_ACCESS",
            "blocking",
            "payload",
            "Adapter payload exposes direct persistence access outside application-service commands, queries, or jobs.",
            "Remove SQL, raw SQLite handles, table names, and direct project-store mutation hooks from adapter/plugin surfaces.",
        )
    ]


def _determine_outcome(findings: list[AdapterFinding]) -> str:
    severities = {finding.severity for finding in findings}
    if "quarantine" in severities:
        return "QUARANTINE"
    if "blocking" in severities:
        return "REJECTED"
    return "ACCEPTED_FORMAT_NEUTRAL_DECLARATION"


def _diagnostic_class(finding: AdapterFinding) -> str:
    if "PROTECTED" in finding.code:
        return "IP_BOUNDARY_WARNING"
    if (
        "UNIT" in finding.code
        or "DIMENSION" in finding.code
        or ("QUANTITY" in finding.code and "PROVENANCE" not in finding.code)
    ):
        return "UNIT_WARNING"
    if "PROVENANCE" in finding.code:
        return "PROVENANCE_WARNING"
    if "PRIVACY" in finding.code or "TELEMETRY" in finding.code:
        return "PRIVACY_WARNING"
    return "ADAPTER_BLOCKING"
