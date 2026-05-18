#!/usr/bin/env python3
"""Policy checks for the telemetry-off-by-default contract."""

from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
POLICY_PATH = ROOT / "docs" / "security" / "telemetry_policy.md"

TRACEABILITY = {
    "deliverable_id: DEL-12-03",
    "package_id: PKG-12",
    "SOW-037",
    "OBJ-010",
}

DEFAULT_OFF_TERMS = {
    "Telemetry is disabled by default.",
    "Absent",
    "unset",
    "unknown",
    "unsupported",
    "malformed",
    "resolves to disabled",
    "Fail-closed behavior is required",
}

INITIALIZATION_BANS = {
    "network transport",
    "background upload jobs",
    "upload queues",
    "local telemetry persistence",
    "endpoints",
    "vendors",
    "external service clients",
}

ALLOWLIST_TERMS = {
    "human-approved event allowlist",
    "No event is collectable until it appears in a human-approved allowlist.",
    "Unknown events",
    "fields not listed on the allowlist",
    "rejected before payload construction",
}

FORBIDDEN_FIELD_CLASSES = {
    "Private project models",
    "Code-specific rule data",
    "Private rule packs",
    "Private material or component libraries",
    "Generated reports and exports",
    "Model hashes",
    "Local file paths",
    "Secrets and credentials",
    "Protected standards content",
    "Professional or code-compliance claims",
}

TBD_DECISIONS = {
    "Product configuration schema",
    "Consent UI or CLI surface",
    "Endpoint, vendor, transport, and retention policy",
    "Concrete event schema and event allowlist",
}

DISALLOWED_IMPLEMENTATION_COMMITMENTS = {
    "https://",
    "http://",
    "segment",
    "amplitude",
    "posthog",
    "sentry",
    "telemetry endpoint is",
    "vendor is",
}


def policy_text():
    return POLICY_PATH.read_text(encoding="utf-8")


def lower_policy_text():
    return policy_text().lower()


def test_policy_is_traceable_to_deliverable_scope():
    text = policy_text()
    for required in TRACEABILITY:
        assert required in text


def test_absent_or_malformed_config_fails_closed_to_disabled():
    text = policy_text()
    for required in DEFAULT_OFF_TERMS:
        assert required in text


def test_transport_and_persistence_require_opt_in_and_allowlist():
    text = policy_text()
    assert "unless all of these are true" in text
    assert "the user has explicitly opted in" in text
    assert "a human-approved event allowlist exists" in text
    assert "event is dropped locally without network behavior" in text
    for banned_surface in INITIALIZATION_BANS:
        assert banned_surface in text


def test_event_allowlist_rejects_unknown_or_unapproved_fields():
    text = policy_text()
    for required in ALLOWLIST_TERMS:
        assert required in text


def test_forbidden_payload_field_classes_are_explicit():
    text = policy_text()
    for field_class in FORBIDDEN_FIELD_CLASSES:
        assert field_class in text


def test_open_decisions_remain_tbd_without_vendor_or_endpoint():
    text = policy_text()
    lowered = lower_policy_text()
    for decision in TBD_DECISIONS:
        assert decision in text
    assert "no endpoint, vendor, transport, or retention behavior is authorized" in text
    for disallowed in DISALLOWED_IMPLEMENTATION_COMMITMENTS:
        assert disallowed not in lowered


def test_no_runtime_networking_or_product_dependency_was_added():
    assert not (ROOT / "core" / "telemetry").exists()
    assert not (ROOT / "apps" / "telemetry").exists()
    assert not (ROOT / "schemas" / "telemetry.schema.yaml").exists()
