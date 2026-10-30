"""Metadata-only telemetry guard controls.

This module resolves telemetry configuration and event attempts without
constructing telemetry payloads, opening network transports, selecting vendors,
starting queues, or persisting telemetry data.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from types import MappingProxyType
from typing import Any, Iterable, Mapping


APPROVED_OPERATIONAL_FIELD_CLASSES = {
    "operational_metadata",
    "public_metadata",
    "low_sensitivity_metadata",
}

CONFIG_KEYS = {
    "enabled",
    "explicit_opt_in",
    "consent_surface",
    "approved_consent_surfaces",
    "allowlist_approved",
    "allowlist_approval_record",
    "event_allowlist",
    "config_source",
}

RUNTIME_SURFACE_KEYS = {
    "endpoint",
    "endpoint_url",
    "external_service_client",
    "network_transport",
    "persistence",
    "queue",
    "telemetry_persistence",
    "transport",
    "upload",
    "upload_job",
    "upload_queue",
    "vendor",
}

PAYLOAD_SHAPED_KEYS = {
    "content",
    "contents",
    "payload",
    "private_payload",
    "raw",
    "raw_value",
    "text",
    "value",
    "values",
}

FORBIDDEN_FIELD_CLASS_TOKENS = {
    "code_specific_rule_data",
    "company_design_basis_data",
    "credential_reference",
    "generated_report",
    "hash_data",
    "model_hash",
    "owner_standard_data",
    "path_data",
    "private_component_data",
    "private_library_data",
    "private_material_data",
    "private_project_data",
    "private_rule_pack_data",
    "professional_claim",
    "protected_suspected",
    "report_payload",
    "secret_like_data",
}

FORBIDDEN_FIELD_MARKERS = {
    "api_key",
    "approval",
    "auth_header",
    "authorization",
    "certificate",
    "certification",
    "checksum",
    "claim",
    "code_compliance",
    "company_design_basis",
    "component",
    "compliance",
    "credential",
    "directory",
    "export",
    "file_name",
    "file_path",
    "filepath",
    "formula",
    "hash",
    "license_key",
    "material",
    "model",
    "owner_standard",
    "password",
    "path",
    "private",
    "professional",
    "project",
    "protected",
    "report",
    "rule",
    "seal",
    "secret",
    "standard",
    "token",
}

FORBIDDEN_EVENT_MARKERS = {
    "component",
    "export",
    "material",
    "model",
    "private",
    "project",
    "protected",
    "report",
    "rule",
    "secret",
    "standard",
}


@dataclass(frozen=True)
class TelemetryDiagnostic:
    code: str
    severity: str
    message: str
    remediation: str
    event_name: str = "TBD"
    field_name: str = "TBD"

    def as_schema_dict(self) -> dict[str, str]:
        return {
            "code": self.code,
            "severity": self.severity,
            "message": self.message,
            "remediation": self.remediation,
            "event_name": self.event_name,
            "field_name": self.field_name,
        }


@dataclass(frozen=True)
class TelemetryConfig:
    enabled: bool
    requested_enabled: bool
    explicit_opt_in: bool
    consent_surface: str
    approved_consent_surfaces: tuple[str, ...]
    allowlist_approved: bool
    event_allowlist: Mapping[str, tuple[str, ...]] = field(
        default_factory=lambda: MappingProxyType({})
    )
    diagnostics: tuple[TelemetryDiagnostic, ...] = ()
    reason_code: str = "telemetry_disabled"
    config_source: str = "metadata_only"
    allowlist_approval_record: str = "TBD"

    def allowed_fields_for(self, event_name: str) -> tuple[str, ...]:
        return self.event_allowlist.get(event_name, ())

    def as_schema_dict(self) -> dict[str, Any]:
        return {
            "enabled": self.enabled,
            "requested_enabled": self.requested_enabled,
            "explicit_opt_in": self.explicit_opt_in,
            "consent_surface": self.consent_surface,
            "approved_consent_surfaces": list(self.approved_consent_surfaces),
            "allowlist_approved": self.allowlist_approved,
            "event_allowlist": {
                event_name: list(fields)
                for event_name, fields in self.event_allowlist.items()
            },
            "diagnostics": [
                diagnostic.as_schema_dict() for diagnostic in self.diagnostics
            ],
            "reason_code": self.reason_code,
            "config_source": self.config_source,
            "allowlist_approval_record": self.allowlist_approval_record,
            "network_transport_initialized": False,
            "endpoint_initialized": False,
            "vendor_initialized": False,
            "upload_queue_initialized": False,
            "upload_job_initialized": False,
            "telemetry_persistence_initialized": False,
            "external_service_client_initialized": False,
        }


@dataclass(frozen=True)
class TelemetryEventAttempt:
    event_name: str
    field_names: tuple[str, ...] = ()
    field_classes: Mapping[str, str] = field(default_factory=lambda: MappingProxyType({}))
    event_version: str = "TBD"
    source_surface: str = "TBD"
    metadata_only: bool = True

    def as_schema_dict(self) -> dict[str, Any]:
        return {
            "event_name": self.event_name,
            "field_names": list(self.field_names),
            "field_classes": dict(self.field_classes),
            "event_version": self.event_version,
            "source_surface": self.source_surface,
            "metadata_only": self.metadata_only,
        }


@dataclass(frozen=True)
class TelemetryDecision:
    decision_id: str
    event_name: str
    action: str
    reason_code: str
    allowed_field_names: tuple[str, ...] = ()
    rejected_field_names: tuple[str, ...] = ()
    payload_constructed: bool = False
    network_behavior_initialized: bool = False

    def as_schema_dict(self) -> dict[str, Any]:
        return {
            "decision_id": self.decision_id,
            "event_name": self.event_name,
            "action": self.action,
            "reason_code": self.reason_code,
            "allowed_field_names": list(self.allowed_field_names),
            "rejected_field_names": list(self.rejected_field_names),
            "payload_constructed": self.payload_constructed,
            "network_behavior_initialized": self.network_behavior_initialized,
        }


@dataclass(frozen=True)
class TelemetryGuardResult:
    config: TelemetryConfig
    attempt: TelemetryEventAttempt | None
    decision: TelemetryDecision
    diagnostics: tuple[TelemetryDiagnostic, ...]
    safe_metadata: Mapping[str, Any] = field(default_factory=lambda: MappingProxyType({}))

    @property
    def blocked(self) -> bool:
        return self.decision.action != "allow_metadata_event"

    def summary(self) -> dict[str, Any]:
        return {
            "event_name": self.decision.event_name,
            "blocked": self.blocked,
            "action": self.decision.action,
            "reason_code": self.decision.reason_code,
            "diagnostic_count": len(self.diagnostics),
            "payload_constructed": False,
            "network_transport_initialized": False,
            "endpoint_initialized": False,
            "vendor_initialized": False,
            "upload_queue_initialized": False,
            "upload_job_initialized": False,
            "telemetry_persistence_initialized": False,
            "external_service_client_initialized": False,
            "professional_claims_made": False,
        }

    def as_schema_dict(self) -> dict[str, Any]:
        return {
            "config": self.config.as_schema_dict(),
            "attempt": self.attempt.as_schema_dict() if self.attempt else None,
            "decision": self.decision.as_schema_dict(),
            "diagnostics": [
                diagnostic.as_schema_dict() for diagnostic in self.diagnostics
            ],
            "safe_metadata": dict(self.safe_metadata),
            "summary": self.summary(),
        }


def resolve_telemetry_config(
    config: TelemetryConfig | Mapping[str, Any] | None,
) -> TelemetryConfig:
    """Resolve telemetry config to an operational default-off state."""

    if isinstance(config, TelemetryConfig):
        return _resolve_existing_config(config)

    if config is None:
        return _disabled_config(
            reason_code="telemetry_config_absent",
            diagnostics=(
                _diagnostic(
                    "TEL-CONFIG-ABSENT",
                    "INFO",
                    "Telemetry configuration is absent, so telemetry is disabled.",
                    "Create an explicit approved opt-in config before collecting events.",
                ),
            ),
        )

    if not isinstance(config, Mapping):
        return _disabled_config(
            reason_code="telemetry_config_malformed",
            diagnostics=(
                _diagnostic(
                    "TEL-CONFIG-MALFORMED",
                    "WARNING",
                    "Telemetry configuration is malformed, so telemetry is disabled.",
                    "Provide a metadata-only mapping with explicit opt-in and allowlist evidence.",
                ),
            ),
        )

    if len(config) == 0:
        return _disabled_config(
            reason_code="telemetry_config_empty",
            diagnostics=(
                _diagnostic(
                    "TEL-CONFIG-EMPTY",
                    "INFO",
                    "Telemetry configuration is empty, so telemetry is disabled.",
                    "Record explicit opt-in, approved consent surface, and allowlist evidence.",
                ),
            ),
        )

    diagnostics: list[TelemetryDiagnostic] = []
    unknown_keys = sorted(set(config) - CONFIG_KEYS)
    runtime_keys = sorted(set(config) & RUNTIME_SURFACE_KEYS)
    if unknown_keys:
        diagnostics.append(
            _diagnostic(
                "TEL-CONFIG-UNKNOWN",
                "WARNING",
                "Telemetry configuration contains unsupported keys, so telemetry is disabled.",
                "Remove unsupported keys and use only the metadata-only guard schema.",
            )
        )
    if runtime_keys:
        diagnostics.append(
            _diagnostic(
                "TEL-CONFIG-RUNTIME-SURFACE",
                "BLOCKING",
                "Telemetry configuration attempted to define runtime transport or persistence surfaces.",
                "Do not configure endpoints, vendors, transports, queues, upload jobs, or telemetry persistence here.",
            )
        )

    raw_enabled = config.get("enabled")
    if raw_enabled is not True and raw_enabled is not False:
        diagnostics.append(
            _diagnostic(
                "TEL-CONFIG-MALFORMED-ENABLED",
                "WARNING",
                "Telemetry enabled state must be an explicit boolean, so telemetry is disabled.",
                "Set enabled to true only with complete opt-in and allowlist evidence.",
            )
        )
        return _disabled_from_mapping(
            config,
            reason_code="telemetry_config_malformed",
            diagnostics=tuple(diagnostics),
        )

    requested_enabled = raw_enabled is True
    explicit_opt_in = _strict_bool(config.get("explicit_opt_in"))
    allowlist_approved = _strict_bool(config.get("allowlist_approved"))
    consent_surface = _string_token(config.get("consent_surface", "TBD"))
    approved_surfaces, approved_surfaces_ok = _string_tuple(
        config.get("approved_consent_surfaces", ())
    )
    approval_record = _string_token(config.get("allowlist_approval_record", "TBD"))
    event_allowlist, allowlist_diagnostics = _normalize_allowlist(
        config.get("event_allowlist", {})
    )
    diagnostics.extend(allowlist_diagnostics)

    if config.get("explicit_opt_in") is not None and explicit_opt_in is None:
        diagnostics.append(
            _diagnostic(
                "TEL-CONFIG-MALFORMED-OPT-IN",
                "WARNING",
                "Telemetry opt-in state must be an explicit boolean.",
                "Set explicit_opt_in to true only after affirmative user action.",
            )
        )
    if config.get("allowlist_approved") is not None and allowlist_approved is None:
        diagnostics.append(
            _diagnostic(
                "TEL-CONFIG-MALFORMED-ALLOWLIST-APPROVAL",
                "WARNING",
                "Telemetry allowlist approval state must be an explicit boolean.",
                "Set allowlist_approved to true only with human approval evidence.",
            )
        )
    if not approved_surfaces_ok:
        diagnostics.append(
            _diagnostic(
                "TEL-CONFIG-MALFORMED-CONSENT-SURFACES",
                "WARNING",
                "Approved consent surfaces must be a sequence of strings.",
                "Provide approved consent surface identifiers as metadata only.",
            )
        )

    if diagnostics and any(d.severity in {"BLOCKING", "WARNING"} for d in diagnostics):
        return TelemetryConfig(
            enabled=False,
            requested_enabled=requested_enabled,
            explicit_opt_in=bool(explicit_opt_in),
            consent_surface=consent_surface,
            approved_consent_surfaces=approved_surfaces,
            allowlist_approved=bool(allowlist_approved),
            event_allowlist=event_allowlist,
            diagnostics=tuple(diagnostics),
            reason_code="telemetry_config_malformed"
            if not unknown_keys and not runtime_keys
            else "telemetry_config_unknown",
            config_source=_string_token(config.get("config_source", "metadata_only")),
            allowlist_approval_record=approval_record,
        )

    if not requested_enabled:
        return TelemetryConfig(
            enabled=False,
            requested_enabled=False,
            explicit_opt_in=bool(explicit_opt_in),
            consent_surface=consent_surface,
            approved_consent_surfaces=approved_surfaces,
            allowlist_approved=bool(allowlist_approved),
            event_allowlist=event_allowlist,
            diagnostics=(
                _diagnostic(
                    "TEL-CONFIG-DISABLED",
                    "INFO",
                    "Telemetry is explicitly disabled by configuration.",
                    "No action is required unless the user later opts in through an approved surface.",
                ),
            ),
            reason_code="telemetry_disabled_by_config",
            config_source=_string_token(config.get("config_source", "metadata_only")),
            allowlist_approval_record=approval_record,
        )

    gate_diagnostics = _gate_diagnostics(
        explicit_opt_in=bool(explicit_opt_in),
        consent_surface=consent_surface,
        approved_surfaces=approved_surfaces,
        allowlist_approved=bool(allowlist_approved),
        event_allowlist=event_allowlist,
    )
    if gate_diagnostics:
        return TelemetryConfig(
            enabled=False,
            requested_enabled=True,
            explicit_opt_in=bool(explicit_opt_in),
            consent_surface=consent_surface,
            approved_consent_surfaces=approved_surfaces,
            allowlist_approved=bool(allowlist_approved),
            event_allowlist=event_allowlist,
            diagnostics=gate_diagnostics,
            reason_code="telemetry_gates_incomplete",
            config_source=_string_token(config.get("config_source", "metadata_only")),
            allowlist_approval_record=approval_record,
        )

    return TelemetryConfig(
        enabled=True,
        requested_enabled=True,
        explicit_opt_in=True,
        consent_surface=consent_surface,
        approved_consent_surfaces=approved_surfaces,
        allowlist_approved=True,
        event_allowlist=event_allowlist,
        diagnostics=(
            _diagnostic(
                "TEL-CONFIG-ENABLED-METADATA-ONLY",
                "INFO",
                "Telemetry metadata guard is enabled for approved event names and field names only.",
                "Construct payloads only after this guard returns allow_metadata_event.",
            ),
        ),
        reason_code="telemetry_enabled_metadata_only",
        config_source=_string_token(config.get("config_source", "metadata_only")),
        allowlist_approval_record=approval_record,
    )


def guard_telemetry_event(
    config: TelemetryConfig | Mapping[str, Any] | None,
    attempt: TelemetryEventAttempt | Mapping[str, Any] | None,
) -> TelemetryGuardResult:
    """Evaluate a telemetry event attempt before payload construction."""

    resolved = resolve_telemetry_config(config)
    normalized_attempt, attempt_diagnostics = _normalize_event_attempt(attempt)
    diagnostics: list[TelemetryDiagnostic] = [
        *resolved.diagnostics,
        *attempt_diagnostics,
    ]

    if normalized_attempt is None:
        decision = TelemetryDecision(
            decision_id="TEL-DECISION-0001",
            event_name="TBD",
            action="drop_event",
            reason_code="telemetry_event_malformed",
        )
        return TelemetryGuardResult(
            config=resolved,
            attempt=None,
            decision=decision,
            diagnostics=tuple(diagnostics),
        )

    if not resolved.enabled:
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-DROPPED-DISABLED",
                "INFO",
                "Telemetry event was dropped locally because telemetry is disabled.",
                "Enable telemetry only through explicit opt-in, approved consent surface, and approved allowlist evidence.",
                event_name=normalized_attempt.event_name,
            )
        )
        decision = TelemetryDecision(
            decision_id="TEL-DECISION-0001",
            event_name=normalized_attempt.event_name,
            action="drop_event",
            reason_code=resolved.reason_code,
            rejected_field_names=normalized_attempt.field_names,
        )
        return TelemetryGuardResult(
            config=resolved,
            attempt=normalized_attempt,
            decision=decision,
            diagnostics=tuple(diagnostics),
        )

    if not normalized_attempt.metadata_only:
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-PAYLOAD-SHAPED",
                "BLOCKING",
                "Telemetry event attempt included payload-shaped data before guard approval.",
                "Pass only event names, field names, and field classifications to this guard.",
                event_name=normalized_attempt.event_name,
            )
        )
        return _blocked_event_result(
            resolved,
            normalized_attempt,
            diagnostics,
            reason_code="telemetry_payload_attempt_before_guard",
            rejected_fields=normalized_attempt.field_names,
        )

    event_reason = _forbidden_event_reason(normalized_attempt.event_name)
    if event_reason is not None:
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-FORBIDDEN-CLASS",
                "BLOCKING",
                "Telemetry event name refers to a forbidden private, protected, report, rule, or secret class.",
                "Remove the event from telemetry and use explicit user-selected support/export workflows where applicable.",
                event_name=normalized_attempt.event_name,
            )
        )
        return _blocked_event_result(
            resolved,
            normalized_attempt,
            diagnostics,
            reason_code=event_reason,
            rejected_fields=normalized_attempt.field_names,
        )

    allowed_fields = resolved.allowed_fields_for(normalized_attempt.event_name)
    if normalized_attempt.event_name not in resolved.event_allowlist:
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-NOT-ALLOWLISTED",
                "BLOCKING",
                "Telemetry event name is not present in the human-approved allowlist.",
                "Add the event only after human/security approval of metadata-only fields.",
                event_name=normalized_attempt.event_name,
            )
        )
        return _blocked_event_result(
            resolved,
            normalized_attempt,
            diagnostics,
            reason_code="telemetry_event_not_allowlisted",
            rejected_fields=normalized_attempt.field_names,
        )

    unknown_fields = tuple(
        field_name
        for field_name in normalized_attempt.field_names
        if field_name not in allowed_fields
    )
    forbidden_fields = _forbidden_fields(normalized_attempt)
    for field_name in unknown_fields:
        diagnostics.append(
            _diagnostic(
                "TEL-FIELD-NOT-ALLOWLISTED",
                "BLOCKING",
                "Telemetry field name is not present in the event allowlist.",
                "Reject the field before constructing any telemetry payload.",
                event_name=normalized_attempt.event_name,
                field_name=field_name,
            )
        )
    for field_name, reason_code in forbidden_fields.items():
        diagnostics.append(
            _diagnostic(
                "TEL-FIELD-FORBIDDEN",
                "BLOCKING",
                "Telemetry field refers to a forbidden private, protected, secret, path, hash, report, or professional-claim class.",
                "Remove the field from telemetry; this guard cannot approve private engineering or code data.",
                event_name=normalized_attempt.event_name,
                field_name=field_name,
            )
        )

    if unknown_fields or forbidden_fields:
        reason = (
            "telemetry_forbidden_field"
            if forbidden_fields
            else "telemetry_field_not_allowlisted"
        )
        rejected_fields = tuple(
            dict.fromkeys((*unknown_fields, *forbidden_fields.keys()))
        )
        return _blocked_event_result(
            resolved,
            normalized_attempt,
            diagnostics,
            reason_code=reason,
            rejected_fields=rejected_fields,
        )

    safe_metadata = MappingProxyType(
        {
            "event_name": normalized_attempt.event_name,
            "event_version": normalized_attempt.event_version,
            "field_names": list(normalized_attempt.field_names),
            "source_surface": normalized_attempt.source_surface,
            "metadata_only": True,
        }
    )
    decision = TelemetryDecision(
        decision_id="TEL-DECISION-0001",
        event_name=normalized_attempt.event_name,
        action="allow_metadata_event",
        reason_code="telemetry_metadata_event_allowlisted",
        allowed_field_names=normalized_attempt.field_names,
    )
    diagnostics.append(
        _diagnostic(
            "TEL-EVENT-ALLOWLISTED",
            "INFO",
            "Telemetry event metadata is allowlisted; no payload has been constructed by the guard.",
            "Construct only approved low-sensitivity metadata fields after this decision.",
            event_name=normalized_attempt.event_name,
        )
    )
    return TelemetryGuardResult(
        config=resolved,
        attempt=normalized_attempt,
        decision=decision,
        diagnostics=tuple(diagnostics),
        safe_metadata=safe_metadata,
    )


def _resolve_existing_config(config: TelemetryConfig) -> TelemetryConfig:
    if not config.enabled:
        return config

    allowlist, allowlist_diagnostics = _normalize_allowlist(config.event_allowlist)
    gate_diagnostics = _gate_diagnostics(
        explicit_opt_in=config.explicit_opt_in,
        consent_surface=config.consent_surface,
        approved_surfaces=config.approved_consent_surfaces,
        allowlist_approved=config.allowlist_approved,
        event_allowlist=allowlist,
    )
    blocking_diagnostics = tuple(
        diagnostic
        for diagnostic in config.diagnostics
        if diagnostic.severity in {"BLOCKING", "WARNING"}
    )
    if allowlist_diagnostics or gate_diagnostics or blocking_diagnostics:
        return TelemetryConfig(
            enabled=False,
            requested_enabled=config.requested_enabled or config.enabled,
            explicit_opt_in=config.explicit_opt_in,
            consent_surface=config.consent_surface,
            approved_consent_surfaces=config.approved_consent_surfaces,
            allowlist_approved=config.allowlist_approved,
            event_allowlist=allowlist,
            diagnostics=(
                *config.diagnostics,
                *allowlist_diagnostics,
                *gate_diagnostics,
            ),
            reason_code="telemetry_gates_incomplete",
            config_source=config.config_source,
            allowlist_approval_record=config.allowlist_approval_record,
        )
    return TelemetryConfig(
        enabled=True,
        requested_enabled=True,
        explicit_opt_in=True,
        consent_surface=config.consent_surface,
        approved_consent_surfaces=config.approved_consent_surfaces,
        allowlist_approved=True,
        event_allowlist=allowlist,
        diagnostics=config.diagnostics,
        reason_code=config.reason_code,
        config_source=config.config_source,
        allowlist_approval_record=config.allowlist_approval_record,
    )


def _disabled_config(
    *,
    reason_code: str,
    diagnostics: tuple[TelemetryDiagnostic, ...],
) -> TelemetryConfig:
    return TelemetryConfig(
        enabled=False,
        requested_enabled=False,
        explicit_opt_in=False,
        consent_surface="TBD",
        approved_consent_surfaces=(),
        allowlist_approved=False,
        event_allowlist=MappingProxyType({}),
        diagnostics=diagnostics,
        reason_code=reason_code,
    )


def _disabled_from_mapping(
    config: Mapping[str, Any],
    *,
    reason_code: str,
    diagnostics: tuple[TelemetryDiagnostic, ...],
) -> TelemetryConfig:
    return TelemetryConfig(
        enabled=False,
        requested_enabled=False,
        explicit_opt_in=False,
        consent_surface=_string_token(config.get("consent_surface", "TBD")),
        approved_consent_surfaces=(),
        allowlist_approved=False,
        event_allowlist=MappingProxyType({}),
        diagnostics=diagnostics,
        reason_code=reason_code,
        config_source=_string_token(config.get("config_source", "metadata_only")),
        allowlist_approval_record=_string_token(
            config.get("allowlist_approval_record", "TBD")
        ),
    )


def _gate_diagnostics(
    *,
    explicit_opt_in: bool,
    consent_surface: str,
    approved_surfaces: tuple[str, ...],
    allowlist_approved: bool,
    event_allowlist: Mapping[str, tuple[str, ...]],
) -> tuple[TelemetryDiagnostic, ...]:
    diagnostics: list[TelemetryDiagnostic] = []
    if not explicit_opt_in:
        diagnostics.append(
            _diagnostic(
                "TEL-GATE-NO-OPT-IN",
                "BLOCKING",
                "Enabled telemetry requires an explicit user opt-in.",
                "Record affirmative user opt-in through an approved product surface.",
            )
        )
    if consent_surface == "TBD" or consent_surface not in approved_surfaces:
        diagnostics.append(
            _diagnostic(
                "TEL-GATE-CONSENT-SURFACE",
                "BLOCKING",
                "Enabled telemetry requires an approved consent surface.",
                "Use only consent surfaces that have been approved for telemetry opt-in.",
            )
        )
    if not allowlist_approved:
        diagnostics.append(
            _diagnostic(
                "TEL-GATE-ALLOWLIST-APPROVAL",
                "BLOCKING",
                "Enabled telemetry requires a human-approved event allowlist.",
                "Record human/security approval before any event is eligible.",
            )
        )
    if not event_allowlist:
        diagnostics.append(
            _diagnostic(
                "TEL-GATE-EMPTY-ALLOWLIST",
                "BLOCKING",
                "Enabled telemetry requires at least one approved event allowlist entry.",
                "Keep telemetry disabled until approved metadata-only events are recorded.",
            )
        )
    return tuple(diagnostics)


def _normalize_allowlist(
    raw_allowlist: Any,
) -> tuple[Mapping[str, tuple[str, ...]], tuple[TelemetryDiagnostic, ...]]:
    diagnostics: list[TelemetryDiagnostic] = []
    if raw_allowlist in ({}, None):
        return MappingProxyType({}), ()
    if not isinstance(raw_allowlist, Mapping):
        return MappingProxyType({}), (
            _diagnostic(
                "TEL-ALLOWLIST-MALFORMED",
                "WARNING",
                "Telemetry event allowlist must be a mapping.",
                "Provide event names mapped to metadata-only field-name lists.",
            ),
        )

    normalized: dict[str, tuple[str, ...]] = {}
    for event_name, entry in raw_allowlist.items():
        event = _string_token(event_name)
        if event == "TBD":
            diagnostics.append(
                _diagnostic(
                    "TEL-ALLOWLIST-MALFORMED-EVENT",
                    "WARNING",
                    "Telemetry event allowlist contains a malformed event name.",
                    "Use non-empty string event names.",
                )
            )
            continue
        fields_raw = entry.get("fields", ()) if isinstance(entry, Mapping) else entry
        fields, fields_ok = _field_names(fields_raw)
        if not fields_ok:
            diagnostics.append(
                _diagnostic(
                    "TEL-ALLOWLIST-MALFORMED-FIELDS",
                    "WARNING",
                    "Telemetry allowlist fields must be field-name strings.",
                    "Provide only metadata field names, not values or payload objects.",
                    event_name=event,
                )
            )
            continue
        forbidden = [
            field_name
            for field_name in fields
            if _forbidden_field_reason(field_name, None) is not None
        ]
        if forbidden:
            diagnostics.append(
                _diagnostic(
                    "TEL-ALLOWLIST-FORBIDDEN-FIELD",
                    "BLOCKING",
                    "Telemetry allowlist contains a forbidden field class.",
                    "Remove private, protected, secret, path, hash, report, or professional-claim fields.",
                    event_name=event,
                    field_name=forbidden[0],
                )
            )
            continue
        normalized[event] = fields
    return MappingProxyType(normalized), tuple(diagnostics)


def _normalize_event_attempt(
    attempt: TelemetryEventAttempt | Mapping[str, Any] | None,
) -> tuple[TelemetryEventAttempt | None, tuple[TelemetryDiagnostic, ...]]:
    if attempt is None:
        return None, (
            _diagnostic(
                "TEL-EVENT-ABSENT",
                "WARNING",
                "Telemetry event attempt is absent.",
                "Pass an event name and metadata-only field names to the guard.",
            ),
        )
    if isinstance(attempt, TelemetryEventAttempt):
        diagnostics = _validate_event_attempt(attempt)
        return attempt, diagnostics
    if not isinstance(attempt, Mapping):
        return None, (
            _diagnostic(
                "TEL-EVENT-MALFORMED",
                "WARNING",
                "Telemetry event attempt is malformed.",
                "Pass a metadata-only mapping or TelemetryEventAttempt.",
            ),
        )

    event_name = _string_token(attempt.get("event_name", attempt.get("name", "TBD")))
    if event_name == "TBD":
        return None, (
            _diagnostic(
                "TEL-EVENT-MALFORMED-NAME",
                "WARNING",
                "Telemetry event name is missing or malformed.",
                "Pass a non-empty event_name string.",
            ),
        )

    fields_raw = attempt.get("fields", ())
    fields, fields_ok = _field_names(fields_raw)
    field_classes = _field_classes_from_attempt(attempt)
    metadata_only = not _contains_payload_shape(attempt)
    if isinstance(fields_raw, Mapping):
        metadata_only = metadata_only and not any(
            _contains_payload_shape(value) for value in fields_raw.values()
        )

    diagnostics: list[TelemetryDiagnostic] = []
    if not fields_ok:
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-MALFORMED-FIELDS",
                "WARNING",
                "Telemetry event fields must be field-name strings.",
                "Pass only metadata field names to the guard.",
                event_name=event_name,
            )
        )
    normalized = TelemetryEventAttempt(
        event_name=event_name,
        field_names=fields,
        field_classes=field_classes,
        event_version=_string_token(attempt.get("event_version", "TBD")),
        source_surface=_string_token(attempt.get("source_surface", "TBD")),
        metadata_only=metadata_only,
    )
    diagnostics.extend(_validate_event_attempt(normalized))
    return normalized, tuple(diagnostics)


def _validate_event_attempt(
    attempt: TelemetryEventAttempt,
) -> tuple[TelemetryDiagnostic, ...]:
    diagnostics: list[TelemetryDiagnostic] = []
    if _string_token(attempt.event_name) == "TBD":
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-MALFORMED-NAME",
                "WARNING",
                "Telemetry event name is missing or malformed.",
                "Pass a non-empty event_name string.",
            )
        )
    malformed_fields = [
        field_name
        for field_name in attempt.field_names
        if _string_token(field_name) == "TBD"
    ]
    if malformed_fields:
        diagnostics.append(
            _diagnostic(
                "TEL-EVENT-MALFORMED-FIELDS",
                "WARNING",
                "Telemetry event fields must be non-empty strings.",
                "Pass only metadata field names to the guard.",
                event_name=attempt.event_name,
            )
        )
    return tuple(diagnostics)


def _field_names(value: Any) -> tuple[tuple[str, ...], bool]:
    if value is None:
        return (), True
    if isinstance(value, Mapping):
        raw_names = tuple(value.keys())
    elif isinstance(value, str):
        return (), False
    elif isinstance(value, Iterable):
        raw_names = tuple(value)
    else:
        return (), False

    normalized: list[str] = []
    for raw_name in raw_names:
        name = _string_token(raw_name)
        if name == "TBD":
            return (), False
        normalized.append(name)
    return tuple(dict.fromkeys(normalized)), True


def _field_classes_from_attempt(attempt: Mapping[str, Any]) -> Mapping[str, str]:
    classes: dict[str, str] = {}
    fields = attempt.get("fields", {})
    if isinstance(fields, Mapping):
        for field_name, descriptor in fields.items():
            field = _string_token(field_name)
            field_class = _field_class_from_descriptor(descriptor)
            if field != "TBD" and field_class != "TBD":
                classes[field] = field_class
    raw_classes = attempt.get("field_classes", {})
    if isinstance(raw_classes, Mapping):
        for field_name, raw_class in raw_classes.items():
            field = _string_token(field_name)
            field_class = _string_token(raw_class)
            if field != "TBD" and field_class != "TBD":
                classes[field] = field_class
    return MappingProxyType(classes)


def _field_class_from_descriptor(descriptor: Any) -> str:
    if isinstance(descriptor, str):
        return _string_token(descriptor)
    if isinstance(descriptor, Mapping):
        for key in ("field_class", "sensitivity", "privacy_classification"):
            value = _string_token(descriptor.get(key, "TBD"))
            if value != "TBD":
                return value
    return "TBD"


def _contains_payload_shape(value: Any) -> bool:
    if isinstance(value, Mapping):
        keys = {str(key) for key in value.keys()}
        if PAYLOAD_SHAPED_KEYS & keys:
            return True
        return any(_contains_payload_shape(item) for item in value.values())
    if isinstance(value, list | tuple):
        return any(_contains_payload_shape(item) for item in value)
    return False


def _forbidden_event_reason(event_name: str) -> str | None:
    normalized = _normalize_marker_text(event_name)
    for marker in FORBIDDEN_EVENT_MARKERS:
        if marker in normalized:
            return "telemetry_forbidden_event_class"
    return None


def _forbidden_fields(
    attempt: TelemetryEventAttempt,
) -> dict[str, str]:
    blocked: dict[str, str] = {}
    for field_name in attempt.field_names:
        reason = _forbidden_field_reason(
            field_name,
            attempt.field_classes.get(field_name),
        )
        if reason is not None:
            blocked[field_name] = reason
    return blocked


def _forbidden_field_reason(field_name: str, field_class: str | None) -> str | None:
    class_token = _normalize_marker_text(field_class or "")
    if class_token in FORBIDDEN_FIELD_CLASS_TOKENS:
        return "telemetry_forbidden_field_class"
    if class_token and class_token not in APPROVED_OPERATIONAL_FIELD_CLASSES:
        for token in FORBIDDEN_FIELD_CLASS_TOKENS:
            if token in class_token:
                return "telemetry_forbidden_field_class"

    name_token = _normalize_marker_text(field_name)
    for marker in FORBIDDEN_FIELD_MARKERS:
        if marker in name_token:
            return "telemetry_forbidden_field_name"
    return None


def _blocked_event_result(
    config: TelemetryConfig,
    attempt: TelemetryEventAttempt,
    diagnostics: list[TelemetryDiagnostic],
    *,
    reason_code: str,
    rejected_fields: tuple[str, ...],
) -> TelemetryGuardResult:
    decision = TelemetryDecision(
        decision_id="TEL-DECISION-0001",
        event_name=attempt.event_name,
        action="drop_event",
        reason_code=reason_code,
        rejected_field_names=rejected_fields,
    )
    return TelemetryGuardResult(
        config=config,
        attempt=attempt,
        decision=decision,
        diagnostics=tuple(diagnostics),
    )


def _string_tuple(value: Any) -> tuple[tuple[str, ...], bool]:
    if value is None:
        return (), True
    if isinstance(value, str):
        return (), False
    if not isinstance(value, Iterable):
        return (), False
    normalized: list[str] = []
    for item in value:
        token = _string_token(item)
        if token == "TBD":
            return (), False
        normalized.append(token)
    return tuple(dict.fromkeys(normalized)), True


def _strict_bool(value: Any) -> bool | None:
    if value is True:
        return True
    if value is False:
        return False
    return None


def _string_token(value: Any) -> str:
    if not isinstance(value, str):
        return "TBD"
    stripped = value.strip()
    return stripped if stripped else "TBD"


def _normalize_marker_text(value: str) -> str:
    normalized = []
    for char in value.lower():
        normalized.append(char if char.isalnum() else "_")
    return "_".join(part for part in "".join(normalized).split("_") if part)


def _diagnostic(
    code: str,
    severity: str,
    message: str,
    remediation: str,
    *,
    event_name: str = "TBD",
    field_name: str = "TBD",
) -> TelemetryDiagnostic:
    return TelemetryDiagnostic(
        code=code,
        severity=severity,
        message=message,
        remediation=remediation,
        event_name=event_name,
        field_name=field_name,
    )
