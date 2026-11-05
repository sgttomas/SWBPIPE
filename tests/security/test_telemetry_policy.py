#!/usr/bin/env python3
"""Policy checks for the telemetry-off-by-default contract."""

from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.telemetry_policy import (  # noqa: E402
    TelemetryConfig,
    TelemetryEventAttempt,
    guard_telemetry_event,
    resolve_telemetry_config,
)


def approved_metadata_config():
    return {
        "enabled": True,
        "explicit_opt_in": True,
        "consent_surface": "approved.settings.telemetry",
        "approved_consent_surfaces": ["approved.settings.telemetry"],
        "allowlist_approved": True,
        "allowlist_approval_record": "human-approved-placeholder",
        "event_allowlist": {
            "app_started": {
                "fields": ["app_version", "os_family"],
            }
        },
    }


def test_resolve_config_absent_empty_unknown_or_malformed_disables_telemetry():
    cases = [
        None,
        {},
        {"mode": "on"},
        {"enabled": "true"},
        {"enabled": True, "endpoint": "TBD"},
    ]

    for case in cases:
        config = resolve_telemetry_config(case)
        assert config.enabled is False
        assert config.requested_enabled is False or config.reason_code in {
            "telemetry_config_unknown",
            "telemetry_config_malformed",
        }
        assert config.as_schema_dict()["network_transport_initialized"] is False
        assert config.as_schema_dict()["telemetry_persistence_initialized"] is False


def test_enabled_config_requires_opt_in_approved_surface_and_allowlist():
    base = approved_metadata_config()
    missing_opt_in = {**base, "explicit_opt_in": False}
    unapproved_surface = {**base, "consent_surface": "unapproved.surface"}
    no_allowlist_approval = {**base, "allowlist_approved": False}
    empty_allowlist = {**base, "event_allowlist": {}}

    for case in (
        missing_opt_in,
        unapproved_surface,
        no_allowlist_approval,
        empty_allowlist,
    ):
        config = resolve_telemetry_config(case)
        assert config.enabled is False
        assert config.requested_enabled is True
        assert config.reason_code == "telemetry_gates_incomplete"


def test_prebuilt_enabled_config_is_still_rechecked_fail_closed():
    manually_enabled = TelemetryConfig(
        enabled=True,
        requested_enabled=True,
        explicit_opt_in=False,
        consent_surface="TBD",
        approved_consent_surfaces=(),
        allowlist_approved=False,
    )

    resolved = resolve_telemetry_config(manually_enabled)

    assert resolved.enabled is False
    assert resolved.requested_enabled is True
    assert resolved.reason_code == "telemetry_gates_incomplete"


def test_allowlisted_metadata_event_passes_without_payload_or_transport():
    result = guard_telemetry_event(
        approved_metadata_config(),
        TelemetryEventAttempt(
            event_name="app_started",
            field_names=("app_version", "os_family"),
            field_classes={
                "app_version": "operational_metadata",
                "os_family": "operational_metadata",
            },
            event_version="0",
            source_surface="unit_test",
        ),
    )

    assert result.blocked is False
    assert result.decision.action == "allow_metadata_event"
    assert result.decision.payload_constructed is False
    assert result.decision.network_behavior_initialized is False
    assert result.safe_metadata["field_names"] == ["app_version", "os_family"]
    summary = result.summary()
    assert summary["payload_constructed"] is False
    assert summary["network_transport_initialized"] is False
    assert summary["endpoint_initialized"] is False
    assert summary["vendor_initialized"] is False
    assert summary["upload_queue_initialized"] is False
    assert summary["upload_job_initialized"] is False
    assert summary["telemetry_persistence_initialized"] is False
    assert summary["external_service_client_initialized"] is False


def test_unknown_event_and_unknown_field_are_rejected_before_payload_construction():
    unknown_event = guard_telemetry_event(
        approved_metadata_config(),
        {
            "event_name": "solver_used",
            "fields": {"app_version": "operational_metadata"},
        },
    )
    unknown_field = guard_telemetry_event(
        approved_metadata_config(),
        {
            "event_name": "app_started",
            "fields": {
                "app_version": "operational_metadata",
                "session_identifier": "operational_metadata",
            },
        },
    )

    assert unknown_event.blocked is True
    assert unknown_event.decision.reason_code == "telemetry_event_not_allowlisted"
    assert unknown_event.summary()["payload_constructed"] is False
    assert unknown_field.blocked is True
    assert unknown_field.decision.reason_code == "telemetry_field_not_allowlisted"
    assert "session_identifier" in unknown_field.decision.rejected_field_names
    assert unknown_field.summary()["network_transport_initialized"] is False


def test_forbidden_private_path_hash_report_secret_and_claim_fields_are_rejected():
    for field_name, field_class in (
        ("app_version", "private_project_data"),
        ("app_version", "protected_suspected"),
        ("app_version", "secret_like_data"),
        ("app_version", "path_data"),
        ("app_version", "model_hash"),
        ("app_version", "generated_report"),
        ("app_version", "professional_claim"),
    ):
        result = guard_telemetry_event(
            approved_metadata_config(),
            {
                "event_name": "app_started",
                "fields": {
                    field_name: field_class,
                },
            },
        )
        assert result.blocked is True
        assert result.decision.reason_code == "telemetry_forbidden_field"
        assert result.decision.payload_constructed is False
        assert result.summary()["network_transport_initialized"] is False


def test_payload_shaped_event_attempt_is_rejected_before_allowlist_payload_build():
    result = guard_telemetry_event(
        approved_metadata_config(),
        {
            "event_name": "app_started",
            "fields": {
                "app_version": {
                    "field_class": "operational_metadata",
                    "value": "fixture-version",
                },
            },
        },
    )

    assert result.blocked is True
    assert result.decision.reason_code == "telemetry_payload_attempt_before_guard"
    assert result.decision.payload_constructed is False
    assert result.summary()["telemetry_persistence_initialized"] is False


def test_no_runtime_networking_or_product_dependency_was_added():
    assert not (ROOT / "core" / "telemetry").exists()
    assert not (ROOT / "apps" / "telemetry").exists()
    assert not (ROOT / "schemas" / "telemetry.schema.yaml").exists()
