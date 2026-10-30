#!/usr/bin/env python3
"""Focused DEL-02-04 adapter/plugin verification regressions."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import pytest


ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT))

from core.adapters.framework import (  # noqa: E402
    verify_adapter_plugin_contracts,
    verify_plugin_manifest,
)
from core.adapters.framework.adapter_framework import (  # noqa: E402
    MAX_CALLER_JSON_BYTES,
    MAX_CALLER_JSON_NODES,
    MAX_DIAGNOSTIC_PATH_BYTES,
    MAX_DIAGNOSTIC_PATH_SEGMENT_BYTES,
)
from core.adapters.framework.plugin_verification import (  # noqa: E402
    MAX_MANIFEST_JSON_NODES,
    MAX_MANIFEST_FALLBACK_IDENTIFIER_BYTES,
    MAX_MANIFEST_SERIALIZED_BYTES,
    MAX_MANIFEST_TEXT_BYTES,
    MAX_SCHEMA_MISMATCH_INSTANCE_BYTES,
    MAX_UNIT_EVIDENCE_FALLBACK_ITEMS,
    _normalize_unit_inputs,
    _schema_mismatches,
)


ADAPTER_FIXTURE = ROOT / "fixtures/adapters/invented/invented_adapter_framework.json"
PLUGIN_SCHEMA = json.loads(
    (ROOT / "schemas/plugin_manifest.schema.yaml").read_text(encoding="utf-8")
)
ADAPTER_SCHEMA = json.loads(
    (ROOT / "schemas/adapter_framework.schema.yaml").read_text(encoding="utf-8")
)
UNIT_CATALOG = {"N": "force", "m": "length"}


def _adapter():
    return json.loads(ADAPTER_FIXTURE.read_text(encoding="utf-8"))


def _manifest():
    """Return invented data that conforms to the canonical plugin schema."""

    return {
        "schema_version": "0.1.0",
        "manifest_kind": "open_pipe_stress_plugin_manifest",
        "metadata": {
            "plugin_id": "ops-plugin-invented-no-bypass",
            "name": "Invented no-bypass manifest",
            "version": "0.1.0",
            "author": "OpenPipeStress",
            "description": "Invented plugin contract verification data.",
            "status": "draft",
        },
        "api_boundary_compatibility": {
            "domain_contract_ref": "docs/architecture/extension_domain_contracts.md",
            "api_boundary_contract_ref": "api/api_boundary_contract.yaml",
            "boundary_api_version": "0.1.0",
            "schema_version_contract": "JSON Schema 2020-12",
            "host_api_status": "draft",
            "compatible_operation_categories": [
                "plugin_manifest", "permission", "privacy", "provenance",
                "checksum", "no_bypass", "diagnostic", "result_envelope",
            ],
            "result_envelope_required": True,
            "transport": "TBD",
        },
        "entrypoints": [{
            "entrypoint_id": "ops.plugin.invented.model_import",
            "extension_point": "model_import_adapter",
            "domain_surface": "canonical_model",
            "operation_mode": "job",
            "handler_ref": "ops.handler.invented.model_import",
            "governed_boundary_required": True,
            "persistence_access": {
                "access_mode": "application_service_command_query_job_only",
                "direct_sql_allowed": False,
                "raw_sqlite_handle_allowed": False,
                "table_name_coupling_allowed": False,
                "direct_store_mutation_allowed": False,
            },
        }],
        "permissions": {
            "denied_by_default": True,
            "requested": ["diagnostics_emit"],
            "grant_state": "not_granted",
            "user_consent_required": True,
            "revocation_supported": True,
        },
        "provenance": {
            "source_name": "PKG-02 invented plugin manifest",
            "source_location": "tests/test_adapter_plugin_verification_del_02_04.py",
            "source_license": "project-invented-test-data",
            "contributor": "OpenPipeStress",
            "contributor_certification": "invented non-engineering fixture",
            "redistribution_status": "public_permissive",
            "review_status": "accepted",
        },
        "privacy": {
            "local_first": True,
            "private_data_transmission_default": False,
            "telemetry_enabled_by_default": False,
            "private_data_access": "explicit_permission_required",
            "export_requires_permission": True,
            "redaction_supported": True,
        },
        "checksums": {
            "manifest_checksum": {
                "algorithm": "sha256",
                "canonicalization": "JCS",
                "payload_ref": "ops-plugin-invented-no-bypass",
                "value": "invented-plugin-manifest-hash",
            },
            "payload_checksums": [],
            "hash_basis": "Canonical JSON/JCS-compatible",
        },
        "sandbox": {
            "sandbox_required": True,
            "arbitrary_code_execution_allowed": False,
            "filesystem_access_default": "denied",
            "network_access_default": "denied",
            "process_spawn_default": "denied",
            "rule_pack_execution_mode": "not_applicable",
            "capability_declaration_required": True,
        },
        "no_bypass_constraints": {
            "unit_controls": True,
            "provenance_controls": True,
            "privacy_controls": True,
            "rule_sandbox_controls": True,
            "analysis_boundary_controls": True,
            "persistence_controls": True,
            "schema_validation": True,
            "diagnostic_envelope": True,
            "checksum_controls": True,
            "protected_content_controls": True,
            "report_boundary_controls": True,
            "solver_boundary_controls": True,
            "human_acceptance_controls": True,
        },
        "professional_boundary": {
            "software_makes_compliance_claim": False,
            "software_makes_certification_claim": False,
            "software_makes_sealing_claim": False,
            "human_review_required": True,
            "human_acceptance_record_software_generated": False,
        },
    }


def _deep_manifest():
    manifest = _manifest()
    nested = []
    manifest["deep_raw_probe"] = nested
    for _ in range(2_000):
        child = []
        nested.append(child)
        nested = child
    return manifest


def _cyclic_manifest():
    manifest = _manifest()
    cycle = []
    cycle.append(cycle)
    manifest["cyclic_raw_probe"] = cycle
    return manifest


def _nonfinite_manifest():
    manifest = _manifest()
    manifest["metadata"]["description"] = float("nan")
    return manifest


class _HostileManifest(dict):
    def get(self, key, default=None):
        raise RuntimeError("caller accessor must not execute")

    def items(self):
        raise RuntimeError("caller iteration must not execute")


def _hostile_manifest():
    return _HostileManifest(_manifest())


def _quantity_evidence():
    return [{
        "path": "plugin_output.force",
        "expected_dimension": "force",
        "quantity": {
            "value": 125.0,
            "unit": "N",
            "dimension": "force",
            "provenance": {
                "source_name": "Invented test quantity",
                "source_location": "tests/test_adapter_plugin_verification_del_02_04.py",
                "source_license": "project-invented-test-data",
                "contributor": "OpenPipeStress",
                "contributor_certification": "invented non-engineering fixture",
                "redistribution_status": "public_permissive",
                "review_status": "accepted",
            },
        },
    }]


def _verify(
    adapter=None,
    manifest=None,
    *,
    schema=PLUGIN_SCHEMA,
    unit_evidence="default",
    unit_catalog=UNIT_CATALOG,
):
    evidence = _quantity_evidence() if unit_evidence == "default" else unit_evidence
    return verify_adapter_plugin_contracts(
        _adapter() if adapter is None else adapter,
        _manifest() if manifest is None else manifest,
        plugin_schema=schema,
        unit_evidence=evidence,
        unit_catalog=unit_catalog,
    )


def _codes(result):
    return {finding.code for finding in result.findings}


def _diagnostics_by_code(result):
    return {item["code"]: item for item in result.result_envelope["diagnostics"]}


class _UnhashableString(str):
    __hash__ = None


class _RaisingEqualityString(str):
    __hash__ = str.__hash__

    def __eq__(self, other):
        raise RuntimeError("caller equality must not execute")


class _HostileCallerDict(dict):
    def __init__(self, value):
        super().__init__(value)
        self.accesses = 0

    def get(self, key, default=None):
        self.accesses += 1
        raise RuntimeError("caller accessor must not execute")

    def items(self):
        self.accesses += 1
        raise RuntimeError("caller iteration must not execute")

    def __iter__(self):
        self.accesses += 1
        raise RuntimeError("caller iteration must not execute")


class _HostileCallerList(list):
    def __init__(self, value):
        super().__init__(value)
        self.accesses = 0

    def __iter__(self):
        self.accesses += 1
        raise RuntimeError("caller iteration must not execute")

    def __getitem__(self, index):
        self.accesses += 1
        raise RuntimeError("caller accessor must not execute")


class _CollidingHostileKey:
    def __init__(self, target):
        self.target = target
        self.equality_calls = 0

    def __hash__(self):
        return hash(self.target)

    def __eq__(self, other):
        self.equality_calls += 1
        raise RuntimeError("hostile raw-key equality must not execute")


def _deep_caller_probe():
    root = []
    cursor = root
    for _ in range(80):
        child = []
        cursor.append(child)
        cursor = child
    return root


def _assert_canonical_fail_closed(result, expected_outcome="REJECTED"):
    assert result.outcome == expected_outcome
    assert result.verification_passed is False
    assert result.runtime_dispatched is False
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


def test_valid_pair_verifies_but_runtime_remains_undispatched():
    result = _verify()
    assert result.outcome == "BLOCKED_RUNTIME_NOT_SELECTED"
    assert result.verification_passed is True
    assert result.declaration_accepted is True
    assert result.manifest_verified is True
    assert result.runtime_dispatched is False
    assert _codes(result) == {"ADAPTER_PLUGIN_RUNTIME_NOT_SELECTED"}


def test_explicit_empty_unit_evidence_declares_no_dimensional_values():
    result = _verify(unit_evidence=[])
    assert result.outcome == "BLOCKED_RUNTIME_NOT_SELECTED"
    assert result.runtime_dispatched is False


@pytest.mark.parametrize("unit_catalog", [None, {}, {"N": "TBD"}, {1: "force"}])
def test_missing_or_malformed_caller_unit_catalog_fails_closed(unit_catalog):
    result = _verify(unit_catalog=unit_catalog)
    assert result.outcome == "REJECTED"
    assert _codes(result) & {"PLUGIN_UNIT_CATALOG_MISSING", "PLUGIN_UNIT_CATALOG_MALFORMED"}


@pytest.mark.parametrize(
    ("mutate", "expected_path"),
    [
        (lambda value: value.pop("professional_boundary"), "professional_boundary"),
        (lambda value: value.__setitem__("schema_version", 1), "schema_version"),
        (lambda value: value.__setitem__("unexpected", True), "unexpected"),
        (lambda value: value["entrypoints"][0]["persistence_access"].__setitem__("direct_sql_allowed", True), "direct_sql_allowed"),
    ],
)
def test_canonical_schema_rejects_invalid_manifest_shapes(mutate, expected_path):
    manifest = _manifest()
    mutate(manifest)
    result = _verify(manifest=manifest)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_MANIFEST_SCHEMA_MISMATCH" in _codes(result)
    assert any(expected_path in finding.path for finding in result.findings)


@pytest.mark.parametrize("schema", [None, "not-a-schema", {}])
def test_missing_or_malformed_schema_fails_closed(schema):
    result = _verify(schema=schema)
    assert result.outcome == "REJECTED"
    assert _codes(result) & {"PLUGIN_MANIFEST_SCHEMA_MISSING", "PLUGIN_MANIFEST_SCHEMA_MALFORMED"}


@pytest.mark.parametrize(
    "weaken",
    [
        lambda schema: schema["required"].remove("checksums"),
        lambda schema: schema["required"].remove("professional_boundary"),
        lambda schema: schema["$defs"]["ChecksumSet"].__setitem__("required", []),
        lambda schema: schema["$defs"]["ProfessionalBoundary"]["properties"][
            "human_review_required"
        ].pop("const"),
    ],
    ids=[
        "remove-checksum-required",
        "remove-professional-boundary-required",
        "relax-checksum-definition",
        "alter-professional-boundary-definition",
    ],
)
def test_weakened_lookalike_schema_is_not_authenticated(weaken):
    schema = json.loads(json.dumps(PLUGIN_SCHEMA))
    weaken(schema)

    result = _verify(schema=schema)

    assert result.outcome == "REJECTED"
    assert "PLUGIN_MANIFEST_SCHEMA_NOT_CANONICAL" in _codes(result)
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False


def test_hostile_schema_accessor_is_rejected_without_access():
    schema = _HostileCallerDict(json.loads(json.dumps(PLUGIN_SCHEMA)))

    result = _verify(schema=schema)

    assert result.outcome == "REJECTED"
    assert "PLUGIN_MANIFEST_SCHEMA_MALFORMED" in _codes(result)
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False
    assert schema.accesses == 0


@pytest.mark.parametrize("missing", ["unit", "dimension", "provenance"])
def test_missing_quantity_metadata_fails_closed(missing):
    evidence = _quantity_evidence()
    del evidence[0]["quantity"][missing]
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_QUANTITY_METADATA_MISSING" in _codes(result)


def test_incompatible_quantity_dimension_fails_closed():
    evidence = _quantity_evidence()
    evidence[0]["quantity"]["dimension"] = "length"
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert result.verification_passed is False
    assert "PLUGIN_QUANTITY_DIMENSION_MISMATCH" in _codes(result)


def test_unknown_quantity_unit_fails_closed():
    evidence = _quantity_evidence()
    evidence[0]["quantity"]["unit"] = "mystery-unit"
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_QUANTITY_UNIT_UNKNOWN" in _codes(result)
    assert _diagnostics_by_code(result)["PLUGIN_QUANTITY_UNIT_UNKNOWN"]["class"] == "UNIT_WARNING"


def test_catalog_unit_incompatible_with_quantity_dimension_fails_closed():
    evidence = _quantity_evidence()
    evidence[0]["quantity"]["unit"] = "m"
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_QUANTITY_UNIT_DIMENSION_MISMATCH" in _codes(result)
    assert _diagnostics_by_code(result)["PLUGIN_QUANTITY_UNIT_DIMENSION_MISMATCH"]["class"] == "UNIT_WARNING"


@pytest.mark.parametrize(
    ("value", "code"),
    [
        ("125", "PLUGIN_QUANTITY_VALUE_MALFORMED"),
        (True, "PLUGIN_QUANTITY_VALUE_MALFORMED"),
        (float("nan"), "PLUGIN_QUANTITY_VALUE_NONFINITE"),
        (float("inf"), "PLUGIN_QUANTITY_VALUE_NONFINITE"),
    ],
)
def test_nonnumeric_or_nonfinite_quantity_value_fails_closed(value, code):
    evidence = _quantity_evidence()
    evidence[0]["quantity"]["value"] = value
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert code in _codes(result)
    assert _diagnostics_by_code(result)[code]["class"] == "UNIT_WARNING"


@pytest.mark.parametrize(
    ("field", "code"),
    [
        ("expected_dimension", "PLUGIN_UNIT_EXPECTED_DIMENSION_NONCANONICAL"),
        ("dimension", "PLUGIN_QUANTITY_DIMENSION_NONCANONICAL"),
    ],
)
def test_noncanonical_dimension_fails_closed(field, code):
    evidence = _quantity_evidence()
    target = evidence[0] if field == "expected_dimension" else evidence[0]["quantity"]
    target[field] = "retired_dimension_alias"
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert code in _codes(result)
    assert _diagnostics_by_code(result)[code]["class"] == "UNIT_WARNING"


@pytest.mark.parametrize("provenance", [None, {}, {"source_name": "available source"}])
def test_malformed_quantity_provenance_fails_closed_without_invention(provenance):
    evidence = _quantity_evidence()
    evidence[0]["quantity"]["provenance"] = provenance
    result = _verify(unit_evidence=evidence)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_QUANTITY_PROVENANCE_MALFORMED" in _codes(result)
    diagnostic = _diagnostics_by_code(result)["PLUGIN_QUANTITY_PROVENANCE_MALFORMED"]
    assert diagnostic["provenance"]["review_status"] == "rejected"
    assert diagnostic["provenance"]["source_name"] == (
        "available source" if provenance else "TBD"
    )
    assert "Invented adapter framework fixture" not in diagnostic["provenance"].values()


@pytest.mark.parametrize("unit_evidence", [None, {}, [None]])
def test_malformed_unit_evidence_fails_closed(unit_evidence):
    result = _verify(unit_evidence=unit_evidence)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_UNIT_EVIDENCE_MALFORMED" in _codes(result)
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )


def test_unit_safety_control_fails_closed_when_disabled():
    manifest = _manifest()
    manifest["no_bypass_constraints"]["unit_controls"] = False
    result = _verify(manifest=manifest)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_UNIT_SAFETY_CONTROL_DISABLED" in _codes(result)


def test_missing_provenance_fails_closed():
    manifest = _manifest()
    del manifest["provenance"]["source_license"]
    result = _verify(manifest=manifest)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_PROVENANCE_INCOMPLETE" in _codes(result)


@pytest.mark.parametrize(
    ("status_field", "status_value"),
    [
        ("redistribution_status", _UnhashableString("public_permissive")),
        ("review_status", _UnhashableString("accepted")),
    ],
)
def test_manifest_hostile_provenance_status_fails_closed_directly(
    status_field,
    status_value,
):
    manifest = _manifest()
    manifest["provenance"][status_field] = status_value

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "REJECTED"
    assert result.verified is False
    assert result.quarantined is False
    assert {finding.code for finding in result.findings} == {
        "PLUGIN_PROVENANCE_INCOMPLETE"
    }


@pytest.mark.parametrize(
    ("status_field", "status_value"),
    [
        ("redistribution_status", _RaisingEqualityString("public_permissive")),
        ("review_status", _RaisingEqualityString("accepted")),
    ],
)
def test_manifest_raising_equality_status_fails_closed_directly(
    status_field,
    status_value,
):
    manifest = _manifest()
    manifest["provenance"][status_field] = status_value

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "REJECTED"
    assert result.verified is False
    assert result.quarantined is False
    assert {finding.code for finding in result.findings} == {
        "PLUGIN_PROVENANCE_INCOMPLETE"
    }


def test_manifest_raising_equality_metadata_status_is_structurally_rejected():
    manifest = _manifest()
    manifest["metadata"]["status"] = _RaisingEqualityString("draft")

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "REJECTED"
    assert result.verified is False
    assert result.quarantined is False
    assert [finding.code for finding in result.findings] == [
        "PLUGIN_MANIFEST_MALFORMED"
    ]
    assert result.findings[0].path == "plugin_manifest.metadata.status"


@pytest.mark.parametrize(
    "manifest_factory",
    [_deep_manifest, _cyclic_manifest, _nonfinite_manifest, _hostile_manifest],
    ids=["deep", "cyclic", "nonfinite", "hostile-container"],
)
def test_raw_manifest_failures_are_structurally_rejected(manifest_factory):
    result = verify_plugin_manifest(manifest_factory(), PLUGIN_SCHEMA)

    assert result.outcome == "REJECTED"
    assert result.verified is False
    assert [finding.code for finding in result.findings] == [
        "PLUGIN_MANIFEST_MALFORMED"
    ]


def test_diagnostic_envelope_incompatibility_fails_closed():
    manifest = _manifest()
    manifest["api_boundary_compatibility"]["result_envelope_required"] = False
    manifest["api_boundary_compatibility"]["compatible_operation_categories"].remove("diagnostic")
    result = _verify(manifest=manifest)
    assert result.outcome == "REJECTED"
    assert {"PLUGIN_RESULT_ENVELOPE_NOT_REQUIRED", "PLUGIN_DIAGNOSTIC_ENVELOPE_INCOMPATIBLE"} <= _codes(result)


def test_suspected_protected_content_quarantines():
    manifest = _manifest()
    manifest["provenance"]["redistribution_status"] = "protected_suspected"
    result = _verify(manifest=manifest)
    assert result.outcome == "QUARANTINE"
    assert result.quarantined is True
    assert result.runtime_dispatched is False
    assert "PLUGIN_PROTECTED_CONTENT_SUSPECTED" in _codes(result)


@pytest.mark.parametrize(
    ("redistribution_status", "expected_classification", "expected_outcome"),
    [
        ("private_only", "private_local_only", "BLOCKED_RUNTIME_NOT_SELECTED"),
        ("protected_suspected", "protected_suspected", "QUARANTINE"),
    ],
)
def test_manifest_boundary_controls_top_level_envelope(
    redistribution_status,
    expected_classification,
    expected_outcome,
):
    manifest = _manifest()
    provenance = manifest["provenance"]
    provenance["redistribution_status"] = redistribution_status

    result = _verify(manifest=manifest)

    assert result.outcome == expected_outcome
    assert result.runtime_dispatched is False
    assert result.result_envelope["privacy"]["classification"] == (
        expected_classification
    )
    assert result.result_envelope["provenance"] == provenance
    assert "Invented adapter framework fixture" not in (
        result.result_envelope["provenance"].values()
    )
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize(
    ("redistribution_status", "expected_classification", "expected_outcome"),
    [
        ("private_only", "private_local_only", "BLOCKED_RUNTIME_NOT_SELECTED"),
        ("protected_suspected", "protected_suspected", "QUARANTINE"),
    ],
)
def test_quantity_boundary_controls_top_level_envelope(
    redistribution_status,
    expected_classification,
    expected_outcome,
):
    unit_evidence = _quantity_evidence()
    provenance = unit_evidence[0]["quantity"]["provenance"]
    provenance["redistribution_status"] = redistribution_status

    result = _verify(unit_evidence=unit_evidence)

    assert result.outcome == expected_outcome
    assert result.runtime_dispatched is False
    assert result.result_envelope["privacy"]["classification"] == (
        expected_classification
    )
    assert result.result_envelope["provenance"] == provenance
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


def test_missing_quantity_provenance_controls_top_level_envelope_fail_closed():
    unit_evidence = _quantity_evidence()
    del unit_evidence[0]["quantity"]["provenance"]

    result = _verify(unit_evidence=unit_evidence)

    assert result.outcome == "REJECTED"
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )
    assert result.result_envelope["provenance"]["source_name"] == "TBD"
    assert result.result_envelope["provenance"]["review_status"] == "rejected"
    assert "Invented adapter framework fixture" not in (
        result.result_envelope["provenance"].values()
    )


def test_incomplete_public_manifest_provenance_is_not_public_reviewed():
    manifest = _manifest()
    provenance = manifest["provenance"]
    del provenance["source_license"]

    result = _verify(manifest=manifest)

    assert result.outcome == "REJECTED"
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )
    assert result.result_envelope["provenance"]["source_name"] == (
        provenance["source_name"]
    )
    assert result.result_envelope["provenance"]["source_license"] == "TBD"
    assert result.result_envelope["provenance"]["review_status"] == "rejected"


@pytest.mark.parametrize("source_kind", ["manifest", "quantity", "adapter_result"])
def test_incomplete_private_provenance_retains_private_boundary(source_kind):
    manifest = _manifest()
    unit_evidence = _quantity_evidence()
    adapter = _adapter()
    if source_kind == "manifest":
        provenance = manifest["provenance"]
    elif source_kind == "quantity":
        provenance = unit_evidence[0]["quantity"]["provenance"]
    else:
        provenance = adapter["operation_result"]["provenance"]
    provenance["redistribution_status"] = "private_only"
    del provenance["source_license"]

    result = _verify(
        adapter=adapter,
        manifest=manifest,
        unit_evidence=unit_evidence,
    )

    assert result.outcome == "REJECTED"
    assert result.result_envelope["privacy"]["classification"] == (
        "private_local_only"
    )
    assert result.result_envelope["provenance"]["redistribution_status"] == (
        "private_only"
    )
    assert result.result_envelope["provenance"]["source_license"] == "TBD"
    assert result.result_envelope["provenance"]["review_status"] == "rejected"


def test_missing_manifest_privacy_is_review_required_not_public():
    manifest = _manifest()
    del manifest["privacy"]

    result = _verify(manifest=manifest)

    assert result.outcome == "REJECTED"
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )
    assert result.result_envelope["provenance"] == manifest["provenance"]


@pytest.mark.parametrize(
    ("private_data_access", "expected_outcome"),
    [
        (None, "REJECTED"),
        ("not-a-status", "REJECTED"),
        ("TBD", "BLOCKED_RUNTIME_NOT_SELECTED"),
    ],
)
def test_manifest_private_data_access_must_be_cleared_for_public_envelope(
    private_data_access,
    expected_outcome,
):
    manifest = _manifest()
    if private_data_access is None:
        del manifest["privacy"]["private_data_access"]
    else:
        manifest["privacy"]["private_data_access"] = private_data_access

    result = _verify(manifest=manifest)

    assert result.outcome == expected_outcome
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )


def test_malformed_adapter_result_privacy_is_review_required_not_public():
    adapter = _adapter()
    adapter["operation_result"]["privacy"] = None

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )
    assert result.result_envelope["provenance"] == (
        adapter["operation_result"]["provenance"]
    )


@pytest.mark.parametrize(
    ("field", "value"),
    [
        ("export_review_required", None),
        ("export_review_required", "yes"),
        ("private_payload_redacted", None),
        ("private_payload_redacted", "yes"),
    ],
)
def test_adapter_required_privacy_booleans_fail_closed(field, value):
    adapter = _adapter()
    privacy = adapter["operation_result"]["privacy"]
    if value is None:
        del privacy[field]
    else:
        privacy[field] = value

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    assert "ADAPTER_PRIVACY_FIELD_INVALID" in _diagnostics_by_code(result)
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )


@pytest.mark.parametrize(
    ("classification", "expected_outcome"),
    [
        ("private_local_only", "REJECTED"),
        ("protected_suspected", "QUARANTINE"),
    ],
)
def test_positive_adapter_privacy_boundary_survives_incomplete_controls(
    classification,
    expected_outcome,
):
    adapter = _adapter()
    privacy = adapter["operation_result"]["privacy"]
    privacy["classification"] = classification
    del privacy["local_first"]

    result = _verify(adapter=adapter)

    assert result.outcome == expected_outcome
    assert result.result_envelope["privacy"]["classification"] == classification
    if classification == "protected_suspected":
        assert "ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED" in _codes(result)


@pytest.mark.parametrize(
    ("redistribution_status", "expected_classification", "expected_outcome"),
    [
        ("private_only", "private_local_only", "BLOCKED_RUNTIME_NOT_SELECTED"),
        ("protected_suspected", "protected_suspected", "QUARANTINE"),
    ],
)
def test_adapter_result_boundary_controls_top_level_envelope(
    redistribution_status,
    expected_classification,
    expected_outcome,
):
    adapter = _adapter()
    provenance = adapter["operation_result"]["provenance"]
    provenance["redistribution_status"] = redistribution_status

    result = _verify(adapter=adapter)

    assert result.outcome == expected_outcome
    assert result.runtime_dispatched is False
    assert result.result_envelope["privacy"]["classification"] == (
        expected_classification
    )
    assert result.result_envelope["provenance"] == provenance
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_plugin_quarantine_marker_precedes_incomplete_provenance(
    marker_field,
    marker_value,
):
    manifest = _manifest()
    manifest["provenance"][marker_field] = marker_value
    del manifest["provenance"]["source_license"]

    result = _verify(manifest=manifest)

    assert result.outcome == "QUARANTINE"
    assert result.runtime_dispatched is False
    diagnostic = _diagnostics_by_code(result)["PLUGIN_PROTECTED_CONTENT_SUSPECTED"]
    assert diagnostic["provenance"][marker_field] == marker_value
    assert diagnostic["provenance"]["source_license"] == "TBD"


def test_protected_content_control_fails_closed_when_missing():
    manifest = _manifest()
    del manifest["no_bypass_constraints"]["protected_content_controls"]
    result = _verify(manifest=manifest)
    assert result.outcome == "REJECTED"
    assert "PLUGIN_PROTECTED_CONTENT_CONTROL_DISABLED" in _codes(result)


def test_adapter_runtime_regression_cannot_bypass_existing_declaration_checks():
    adapter = _adapter()
    adapter["adapter_declaration"]["no_bypass_controls"]["must_use_unit_validation"] = False
    result = _verify(adapter=adapter)
    assert result.outcome == "REJECTED"
    assert result.declaration_accepted is False
    assert result.manifest_verified is True
    assert "ADAPTER_NO_BYPASS_CONTROL_DISABLED" in _codes(result)


@pytest.mark.parametrize("capabilities", [None, [["unhashable"]]])
def test_malformed_nested_adapter_capabilities_never_raise(capabilities):
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = capabilities
    result = _verify(adapter=adapter)
    assert result.outcome == "REJECTED"
    assert "ADAPTER_CAPABILITIES_MALFORMED" in _diagnostics_by_code(result)
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False


@pytest.mark.parametrize(
    "capabilities",
    [
        "import_model",
        ("import_model",),
        {"import_model"},
        range(1),
    ],
)
def test_non_list_adapter_capability_iterables_fail_closed(capabilities):
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = capabilities

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    assert "ADAPTER_CAPABILITIES_MALFORMED" in _diagnostics_by_code(result)
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False


@pytest.mark.parametrize(
    ("capabilities", "expected_code"),
    [
        (["arbitrary_hashable_token"], "ADAPTER_CAPABILITY_INVALID"),
        (["import_model", "arbitrary_hashable_token"], "ADAPTER_CAPABILITY_INVALID"),
        ([1], "ADAPTER_CAPABILITIES_MALFORMED"),
        (["TBD"], "ADAPTER_OPERATIONAL_CAPABILITY_MISSING"),
        (["import_library"], "ADAPTER_OPERATIONAL_CAPABILITY_MISSING"),
    ],
)
def test_noncanonical_adapter_capability_values_fail_closed(
    capabilities,
    expected_code,
):
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = capabilities

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    assert expected_code in _diagnostics_by_code(result)
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False


def test_duplicate_canonical_adapter_capabilities_are_schema_valid():
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = [
        "import_model",
        "import_model",
    ]

    result = _verify(adapter=adapter)

    assert result.outcome == "BLOCKED_RUNTIME_NOT_SELECTED"
    assert result.declaration_accepted is True
    assert result.runtime_dispatched is False


@pytest.mark.parametrize(
    ("capabilities", "capability_code"),
    [
        (None, "ADAPTER_CAPABILITIES_MALFORMED"),
        ([["unhashable"]], "ADAPTER_CAPABILITIES_MALFORMED"),
        ("import_model", "ADAPTER_CAPABILITIES_MALFORMED"),
        (["arbitrary_hashable_token"], "ADAPTER_CAPABILITY_INVALID"),
    ],
)
@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_adapter_quarantine_marker_survives_malformed_capabilities(
    capabilities,
    capability_code,
    marker_field,
    marker_value,
):
    adapter = _adapter()
    provenance = adapter["adapter_declaration"]["provenance"]
    provenance[marker_field] = marker_value
    adapter["adapter_declaration"]["capabilities"] = capabilities

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert capability_code in diagnostics
    quarantine_diagnostic = diagnostics["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine_diagnostic["provenance"][marker_field] == marker_value
    assert result.declaration_accepted is False


@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_unhashable_string_capability_never_masks_composed_quarantine(
    marker_field,
    marker_value,
):
    class UnhashableString(str):
        __hash__ = None

    adapter = _adapter()
    provenance = adapter["adapter_declaration"]["provenance"]
    provenance[marker_field] = marker_value
    adapter["adapter_declaration"]["capabilities"] = [
        UnhashableString("import_model")
    ]

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert "ADAPTER_CAPABILITIES_MALFORMED" in diagnostics
    assert "ADAPTER_DECLARATION_MALFORMED" not in diagnostics
    quarantine_diagnostic = diagnostics["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine_diagnostic["provenance"][marker_field] == marker_value
    assert result.declaration_accepted is False


@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_operation_result_quarantine_uses_its_provenance_with_malformed_capabilities(
    marker_field,
    marker_value,
):
    adapter = _adapter()
    operation_provenance = adapter["operation_result"]["provenance"]
    operation_provenance[marker_field] = marker_value
    adapter["adapter_declaration"]["capabilities"] = None

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert "ADAPTER_CAPABILITIES_MALFORMED" in diagnostics
    quarantine_diagnostic = diagnostics["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine_diagnostic["affected_object"]["ref_id"] == (
        "operation_result.provenance"
    )
    assert quarantine_diagnostic["provenance"] == operation_provenance
    assert quarantine_diagnostic["provenance"][marker_field] == marker_value


@pytest.mark.parametrize("object_name", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    ("field", "value"),
    [
        ("review_status", "rejected"),
        ("review_status", "needs_review"),
        ("review_status", "TBD"),
        ("redistribution_status", "unknown"),
        ("redistribution_status", "TBD"),
    ],
)
def test_composed_verifier_blocks_uncleared_adapter_provenance(
    object_name,
    field,
    value,
):
    adapter = _adapter()
    adapter[object_name]["provenance"][field] = value

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False
    diagnostic = _diagnostics_by_code(result)["ADAPTER_PROVENANCE_NOT_CLEARED"]
    assert diagnostic["affected_object"]["ref_id"] == f"{object_name}.provenance"
    assert diagnostic["provenance"] == adapter[object_name]["provenance"]
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("object_name", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize("capabilities", [["import_model"], None])
def test_composed_verifier_quarantines_protected_adapter_privacy(
    object_name,
    capabilities,
):
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = capabilities
    adapter[object_name]["privacy"]["classification"] = "protected_suspected"

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    quarantine = diagnostics["ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine["affected_object"]["ref_id"] == (
        f"{object_name}.privacy.classification"
    )
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    if capabilities is None:
        assert "ADAPTER_CAPABILITIES_MALFORMED" in diagnostics
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("object_name", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_composed_adapter_quarantine_precedes_truthy_malformed_provenance(
    object_name,
    marker_field,
    marker_value,
):
    adapter = _adapter()
    provenance = adapter[object_name]["provenance"]
    provenance[marker_field] = marker_value
    provenance["source_license"] = {"truthy": True}

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False
    diagnostic = _diagnostics_by_code(result)["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]
    assert diagnostic["affected_object"]["ref_id"] == f"{object_name}.provenance"
    assert diagnostic["provenance"][marker_field] == marker_value
    assert diagnostic["provenance"]["source_license"] == "TBD"
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("object_name", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    ("field", "value"),
    [
        ("source_license", {"truthy": True}),
        ("redistribution_status", _UnhashableString("public_permissive")),
        ("review_status", _UnhashableString("accepted")),
    ],
)
def test_composed_verifier_rejects_hostile_adapter_provenance_shape(
    object_name,
    field,
    value,
):
    adapter = _adapter()
    adapter[object_name]["provenance"][field] = value

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    diagnostic = diagnostics["ADAPTER_PROVENANCE_INCOMPLETE"]
    assert diagnostic["affected_object"]["ref_id"] == f"{object_name}.provenance"
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("object_name", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    ("status_field", "status_value"),
    [
        ("redistribution_status", _UnhashableString("public_permissive")),
        ("review_status", _UnhashableString("accepted")),
    ],
)
def test_composed_protected_privacy_dominates_hostile_provenance_and_capabilities(
    object_name,
    status_field,
    status_value,
):
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = None
    adapter[object_name]["provenance"][status_field] = status_value
    adapter[object_name]["privacy"]["classification"] = "protected_suspected"

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert "ADAPTER_PROVENANCE_INCOMPLETE" in diagnostics
    assert "ADAPTER_CAPABILITIES_MALFORMED" in diagnostics
    quarantine = diagnostics["ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine["affected_object"]["ref_id"] == (
        f"{object_name}.privacy.classification"
    )
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("privacy_object", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    ("status_field", "status_value"),
    [
        ("redistribution_status", _UnhashableString("public_permissive")),
        ("review_status", _UnhashableString("accepted")),
    ],
)
def test_adapter_protected_privacy_dominates_hostile_manifest_provenance(
    privacy_object,
    status_field,
    status_value,
):
    adapter = _adapter()
    adapter[privacy_object]["privacy"]["classification"] = "protected_suspected"
    manifest = _manifest()
    manifest["provenance"][status_field] = status_value

    result = _verify(adapter=adapter, manifest=manifest)

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert "PLUGIN_PROVENANCE_INCOMPLETE" in diagnostics
    quarantine = diagnostics["ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine["affected_object"]["ref_id"] == (
        f"{privacy_object}.privacy.classification"
    )
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("privacy_object", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    ("status_field", "status_value"),
    [
        ("redistribution_status", _RaisingEqualityString("public_permissive")),
        ("review_status", _RaisingEqualityString("accepted")),
    ],
)
def test_adapter_protected_privacy_dominates_raising_equality_manifest_status(
    privacy_object,
    status_field,
    status_value,
):
    adapter = _adapter()
    adapter[privacy_object]["privacy"]["classification"] = "protected_suspected"
    manifest = _manifest()
    manifest["provenance"][status_field] = status_value

    result = _verify(adapter=adapter, manifest=manifest)

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert "PLUGIN_PROVENANCE_INCOMPLETE" in diagnostics
    quarantine = diagnostics["ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine["affected_object"]["ref_id"] == (
        f"{privacy_object}.privacy.classification"
    )
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("privacy_object", ["adapter_declaration", "operation_result"])
def test_adapter_protected_privacy_dominates_raising_metadata_status(
    privacy_object,
):
    adapter = _adapter()
    adapter[privacy_object]["privacy"]["classification"] = "protected_suspected"
    manifest = _manifest()
    manifest["metadata"]["status"] = _RaisingEqualityString("draft")

    result = _verify(adapter=adapter, manifest=manifest)

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    malformed = diagnostics["PLUGIN_MANIFEST_MALFORMED"]
    assert malformed["affected_object"]["ref_id"] == (
        "plugin_manifest.metadata.status"
    )
    quarantine = diagnostics["ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine["affected_object"]["ref_id"] == (
        f"{privacy_object}.privacy.classification"
    )
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


@pytest.mark.parametrize("privacy_object", ["adapter_declaration", "operation_result"])
@pytest.mark.parametrize(
    "manifest_factory",
    [_deep_manifest, _cyclic_manifest, _nonfinite_manifest, _hostile_manifest],
    ids=["deep", "cyclic", "nonfinite", "hostile-container"],
)
def test_adapter_protected_privacy_dominates_raw_manifest_failures(
    privacy_object,
    manifest_factory,
):
    adapter = _adapter()
    adapter[privacy_object]["privacy"]["classification"] = "protected_suspected"

    result = _verify(adapter=adapter, manifest=manifest_factory())

    assert result.outcome == "QUARANTINE"
    assert result.declaration_accepted is False
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False
    diagnostics = _diagnostics_by_code(result)
    assert "PLUGIN_MANIFEST_MALFORMED" in diagnostics
    quarantine = diagnostics["ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED"]
    assert quarantine["affected_object"]["ref_id"] == (
        f"{privacy_object}.privacy.classification"
    )
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []


def test_missing_operation_result_does_not_inherit_declaration_provenance():
    adapter = _adapter()
    declaration_provenance = adapter["adapter_declaration"]["provenance"]
    declaration_provenance["source_name"] = "Distinct declaration provenance"
    adapter["operation_result"] = None

    result = _verify(adapter=adapter)

    assert result.outcome == "REJECTED"
    diagnostic = _diagnostics_by_code(result)["ADAPTER_RESULT_MISSING"]
    assert diagnostic["affected_object"]["ref_id"] == "operation_result"
    assert diagnostic["provenance"]["source_name"] == "TBD"
    assert diagnostic["provenance"]["review_status"] == "rejected"
    assert "Distinct declaration provenance" not in diagnostic["provenance"].values()
    assert result.result_envelope["privacy"]["classification"] == (
        "export_review_required"
    )
    assert result.result_envelope["provenance"]["review_status"] == "rejected"


def test_result_envelope_is_complete_and_propagates_finding_classes():
    unit_evidence = _quantity_evidence()
    unit_evidence[0]["quantity"]["dimension"] = "length"
    unit_result = _verify(unit_evidence=unit_evidence)
    provenance_manifest = _manifest()
    del provenance_manifest["provenance"]["source_license"]
    provenance_result = _verify(manifest=provenance_manifest)
    protected_manifest = _manifest()
    protected_manifest["provenance"]["redistribution_status"] = "protected_suspected"
    protected_result = _verify(manifest=protected_manifest)
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = None
    adapter_result = _verify(adapter=adapter)

    expected = (
        (unit_result, "PLUGIN_QUANTITY_DIMENSION_MISMATCH", "UNIT_WARNING"),
        (provenance_result, "PLUGIN_PROVENANCE_INCOMPLETE", "PROVENANCE_WARNING"),
        (protected_result, "PLUGIN_PROTECTED_CONTENT_SUSPECTED", "IP_BOUNDARY_WARNING"),
        (adapter_result, "ADAPTER_CAPABILITIES_MALFORMED", "ADAPTER_BLOCKING"),
    )
    for result, code, expected_class in expected:
        diagnostic = _diagnostics_by_code(result)[code]
        assert diagnostic["class"] == expected_class
        for field in ("severity", "source", "affected_object", "message", "remediation", "provenance"):
            assert diagnostic[field]


def test_result_envelope_propagates_exact_adapter_plugin_and_unit_context():
    adapter = _adapter()
    adapter["adapter_declaration"]["capabilities"] = None
    adapter_result = _verify(adapter=adapter)
    adapter_diagnostic = _diagnostics_by_code(adapter_result)["ADAPTER_CAPABILITIES_MALFORMED"]
    assert adapter_diagnostic["source"] == {
        "ref_type": "adapter",
        "ref_id": adapter["adapter_declaration"]["adapter_id"],
    }
    assert adapter_diagnostic["provenance"] == adapter["adapter_declaration"]["provenance"]
    assert adapter_diagnostic["affected_object"]["ref_id"] == "adapter_declaration.capabilities"

    manifest = _manifest()
    del manifest["provenance"]["source_license"]
    plugin_result = _verify(manifest=manifest)
    plugin_diagnostic = _diagnostics_by_code(plugin_result)["PLUGIN_PROVENANCE_INCOMPLETE"]
    assert plugin_diagnostic["source"] == {
        "ref_type": "payload",
        "ref_id": manifest["metadata"]["plugin_id"],
    }
    assert plugin_diagnostic["provenance"]["source_name"] == manifest["provenance"]["source_name"]
    assert plugin_diagnostic["provenance"]["source_license"] == "TBD"
    assert plugin_diagnostic["provenance"]["review_status"] == "rejected"
    assert plugin_diagnostic["affected_object"]["ref_id"] == "plugin_manifest.provenance"

    unit_evidence = _quantity_evidence()
    unit_evidence[0]["quantity"]["unit"] = "m"
    unit_result = _verify(unit_evidence=unit_evidence)
    unit_diagnostic = _diagnostics_by_code(unit_result)["PLUGIN_QUANTITY_UNIT_DIMENSION_MISMATCH"]
    assert unit_diagnostic["source"] == {
        "ref_type": "payload",
        "ref_id": unit_evidence[0]["path"],
    }
    assert unit_diagnostic["affected_object"]["ref_id"] == unit_evidence[0]["path"]
    assert unit_diagnostic["provenance"] == unit_evidence[0]["quantity"]["provenance"]


def test_quantity_provenance_protected_content_quarantines():
    for field, value in (
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ):
        unit_evidence = _quantity_evidence()
        unit_evidence[0]["quantity"]["provenance"][field] = value

        result = _verify(unit_evidence=unit_evidence)

        assert result.outcome == "QUARANTINE"
        assert result.runtime_dispatched is False
        diagnostic = _diagnostics_by_code(result)[
            "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED"
        ]
        assert diagnostic["class"] == "IP_BOUNDARY_WARNING"
        assert diagnostic["source"]["ref_type"] == "payload"
        assert diagnostic["provenance"] == unit_evidence[0]["quantity"]["provenance"]


def test_invalid_provenance_enums_fail_closed_in_diagnostic_envelope():
    unit_evidence = _quantity_evidence()
    provenance = unit_evidence[0]["quantity"]["provenance"]
    provenance["redistribution_status"] = "not-a-status"
    provenance["review_status"] = "accepted"

    result = _verify(unit_evidence=unit_evidence)

    assert result.outcome == "REJECTED"
    diagnostic = _diagnostics_by_code(result)["PLUGIN_QUANTITY_PROVENANCE_MALFORMED"]
    assert diagnostic["provenance"]["redistribution_status"] == "TBD"
    assert diagnostic["provenance"]["review_status"] == "rejected"


def test_composed_results_conform_to_adapter_operation_result_schema():
    protected_units = _quantity_evidence()
    protected_units[0]["quantity"]["provenance"][
        "redistribution_status"
    ] = "protected_suspected"

    malformed_units = _quantity_evidence()
    malformed_units[0]["quantity"]["provenance"][
        "redistribution_status"
    ] = "not-a-status"

    for result in (
        _verify(),
        _verify(unit_evidence=protected_units),
        _verify(unit_evidence=malformed_units),
    ):
        assert _schema_mismatches(
            result.result_envelope,
            ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
            ADAPTER_SCHEMA,
            "operation_result",
        ) == []


def test_adapter_provenance_invalid_or_quarantined_fails_closed():
    invalid_adapter = _adapter()
    invalid_adapter["adapter_declaration"]["provenance"][
        "redistribution_status"
    ] = "not-a-status"
    invalid_result = _verify(adapter=invalid_adapter)
    assert invalid_result.outcome == "REJECTED"
    assert invalid_result.declaration_accepted is False
    assert "ADAPTER_PROVENANCE_INVALID" in _diagnostics_by_code(invalid_result)

    quarantined_adapter = _adapter()
    quarantined_adapter["adapter_declaration"]["provenance"][
        "review_status"
    ] = "quarantined"
    quarantined_result = _verify(adapter=quarantined_adapter)
    assert quarantined_result.outcome == "QUARANTINE"
    assert quarantined_result.declaration_accepted is False
    assert "ADAPTER_PROTECTED_CONTENT_SUSPECTED" in _diagnostics_by_code(
        quarantined_result
    )


@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_adapter_quarantine_marker_precedes_incomplete_provenance(
    marker_field,
    marker_value,
):
    adapter = _adapter()
    provenance = adapter["adapter_declaration"]["provenance"]
    provenance[marker_field] = marker_value
    del provenance["source_license"]

    result = _verify(adapter=adapter)

    assert result.outcome == "QUARANTINE"
    assert result.runtime_dispatched is False
    diagnostic = _diagnostics_by_code(result)["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]
    assert diagnostic["provenance"][marker_field] == marker_value
    assert diagnostic["provenance"]["source_license"] == "TBD"


@pytest.mark.parametrize(
    ("redistribution_status", "review_status"),
    [
        ("unknown", "accepted"),
        ("TBD", "accepted"),
        ("public_permissive", "needs_review"),
        ("public_permissive", "rejected"),
        ("public_permissive", "TBD"),
    ],
)
def test_quantity_provenance_noncleared_statuses_block(
    redistribution_status,
    review_status,
):
    unit_evidence = _quantity_evidence()
    provenance = unit_evidence[0]["quantity"]["provenance"]
    provenance["redistribution_status"] = redistribution_status
    provenance["review_status"] = review_status

    result = _verify(unit_evidence=unit_evidence)

    assert result.outcome == "REJECTED"
    assert "PLUGIN_QUANTITY_PROVENANCE_NOT_CLEARED" in _diagnostics_by_code(result)


def test_quantity_quarantine_marker_precedes_other_malformed_status():
    for quarantine_field, quarantine_value, malformed_field in (
        ("redistribution_status", "protected_suspected", "review_status"),
        ("review_status", "quarantined", "redistribution_status"),
    ):
        unit_evidence = _quantity_evidence()
        provenance = unit_evidence[0]["quantity"]["provenance"]
        provenance[quarantine_field] = quarantine_value
        provenance[malformed_field] = "not-a-status"

        result = _verify(unit_evidence=unit_evidence)

        assert result.outcome == "QUARANTINE"
        assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" in _diagnostics_by_code(
            result
        )


@pytest.mark.parametrize(
    ("marker_field", "marker_value", "missing_field"),
    [
        ("redistribution_status", "protected_suspected", "unit"),
        ("review_status", "quarantined", "value"),
    ],
)
def test_quantity_quarantine_marker_precedes_missing_quantity_metadata(
    marker_field,
    marker_value,
    missing_field,
):
    unit_evidence = _quantity_evidence()
    quantity = unit_evidence[0]["quantity"]
    quantity["provenance"][marker_field] = marker_value
    del quantity["provenance"]["source_license"]
    del quantity[missing_field]

    result = _verify(unit_evidence=unit_evidence)

    assert result.outcome == "QUARANTINE"
    assert result.runtime_dispatched is False
    diagnostic = _diagnostics_by_code(result)[
        "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED"
    ]
    assert diagnostic["provenance"][marker_field] == marker_value
    assert diagnostic["provenance"]["source_license"] == "TBD"


@pytest.mark.parametrize(
    "failure_kind",
    [
        "hostile_mapping",
        "hostile_list",
        "cycle",
        "depth",
        "nonfinite",
        "serialization",
    ],
)
def test_composed_adapter_input_is_snapshotted_before_any_caller_traversal(
    failure_kind,
):
    adapter = _adapter()
    probe = None
    if failure_kind == "hostile_mapping":
        adapter = _HostileCallerDict(adapter)
        probe = adapter
    elif failure_kind == "hostile_list":
        probe = _HostileCallerList(["import_model"])
        adapter["adapter_declaration"]["capabilities"] = probe
    elif failure_kind == "cycle":
        probe = []
        probe.append(probe)
        adapter["caller_probe"] = probe
    elif failure_kind == "depth":
        probe = _deep_caller_probe()
        adapter["caller_probe"] = probe
    elif failure_kind == "nonfinite":
        probe = float("nan")
        adapter["caller_probe"] = probe
    else:
        probe = 10**5000
        adapter["caller_probe"] = probe

    result = _verify(adapter=adapter)

    _assert_canonical_fail_closed(result)
    assert result.declaration_accepted is False
    assert (
        "ADAPTER_CAPABILITIES_MALFORMED"
        if failure_kind == "hostile_list"
        else "ADAPTER_DECLARATION_MALFORMED"
    ) in _codes(result)
    if hasattr(probe, "accesses"):
        assert probe.accesses == 0
    if failure_kind == "cycle":
        assert probe[0] is probe
    if failure_kind == "nonfinite":
        assert probe != probe


@pytest.mark.parametrize(
    "failure_kind",
    [
        "hostile_mapping",
        "hostile_list",
        "cycle",
        "depth",
        "nonfinite",
        "serialization",
    ],
)
def test_composed_unit_catalog_is_independently_snapshotted(failure_kind):
    catalog = dict(UNIT_CATALOG)
    probe = None
    if failure_kind == "hostile_mapping":
        catalog = _HostileCallerDict(catalog)
        probe = catalog
    elif failure_kind == "hostile_list":
        probe = _HostileCallerList(["force"])
        catalog["probe"] = probe
    elif failure_kind == "cycle":
        probe = []
        probe.append(probe)
        catalog["probe"] = probe
    elif failure_kind == "depth":
        probe = _deep_caller_probe()
        catalog["probe"] = probe
    elif failure_kind == "nonfinite":
        probe = float("inf")
        catalog["probe"] = probe
    else:
        probe = 10**5000
        catalog["probe"] = probe

    result = _verify(unit_catalog=catalog)

    _assert_canonical_fail_closed(result)
    assert "PLUGIN_UNIT_CATALOG_MALFORMED" in _codes(result)
    if hasattr(probe, "accesses"):
        assert probe.accesses == 0
    if failure_kind == "cycle":
        assert probe[0] is probe
    if failure_kind == "nonfinite":
        assert probe == float("inf")


@pytest.mark.parametrize(
    "failure_kind",
    [
        "hostile_list",
        "hostile_mapping",
        "cycle",
        "depth",
        "nonfinite",
        "serialization",
    ],
)
def test_composed_unit_evidence_is_independently_snapshotted(failure_kind):
    evidence = _quantity_evidence()
    probe = None
    if failure_kind == "hostile_list":
        evidence = _HostileCallerList(evidence)
        probe = evidence
    elif failure_kind == "hostile_mapping":
        probe = _HostileCallerDict({"untrusted": True})
        evidence[0]["caller_probe"] = probe
    elif failure_kind == "cycle":
        probe = []
        probe.append(probe)
        evidence[0]["caller_probe"] = probe
    elif failure_kind == "depth":
        probe = _deep_caller_probe()
        evidence[0]["caller_probe"] = probe
    elif failure_kind == "nonfinite":
        probe = float("-inf")
        evidence[0]["caller_probe"] = probe
    else:
        probe = 10**5000
        evidence[0]["caller_probe"] = probe

    result = _verify(unit_evidence=evidence)

    _assert_canonical_fail_closed(result)
    assert "PLUGIN_UNIT_EVIDENCE_MALFORMED" in _codes(result)
    if hasattr(probe, "accesses"):
        assert probe.accesses == 0
    if failure_kind == "cycle":
        assert probe[0] is probe
    if failure_kind == "nonfinite":
        assert probe == float("-inf")


@pytest.mark.parametrize(
    "failure_kind", ["hostile", "cycle", "depth", "nonfinite"]
)
def test_composed_safe_adapter_quarantine_marker_survives_malformed_sibling(
    failure_kind,
):
    adapter = _adapter()
    provenance = adapter["operation_result"]["provenance"]
    provenance["review_status"] = "quarantined"
    if failure_kind == "hostile":
        probe = _HostileCallerDict({"untrusted": True})
    elif failure_kind == "cycle":
        probe = []
        probe.append(probe)
    elif failure_kind == "depth":
        probe = _deep_caller_probe()
    else:
        probe = float("nan")
    adapter["caller_probe"] = probe

    result = _verify(adapter=adapter)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    diagnostics = _diagnostics_by_code(result)
    assert "ADAPTER_DECLARATION_MALFORMED" in diagnostics
    assert "ADAPTER_PROTECTED_CONTENT_SUSPECTED" in diagnostics
    assert diagnostics["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]["provenance"][
        "review_status"
    ] == "quarantined"
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    if hasattr(probe, "accesses"):
        assert probe.accesses == 0


@pytest.mark.parametrize(
    "failure_kind", ["hostile", "cycle", "depth", "nonfinite"]
)
def test_composed_safe_unit_quarantine_marker_survives_malformed_unit_inputs(
    failure_kind,
):
    evidence = _quantity_evidence()
    provenance = evidence[0]["quantity"]["provenance"]
    provenance["redistribution_status"] = "protected_suspected"
    if failure_kind == "hostile":
        evidence_probe = _HostileCallerDict({"untrusted": True})
    elif failure_kind == "cycle":
        evidence_probe = []
        evidence_probe.append(evidence_probe)
    elif failure_kind == "depth":
        evidence_probe = _deep_caller_probe()
    else:
        evidence_probe = float("inf")
    evidence.append(evidence_probe)
    hostile_catalog = _HostileCallerDict(UNIT_CATALOG)

    result = _verify(unit_evidence=evidence, unit_catalog=hostile_catalog)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    diagnostics = _diagnostics_by_code(result)
    assert "PLUGIN_UNIT_EVIDENCE_MALFORMED" in diagnostics
    assert "PLUGIN_UNIT_CATALOG_MALFORMED" in diagnostics
    assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" in diagnostics
    assert diagnostics["PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED"][
        "provenance"
    ]["redistribution_status"] == "protected_suspected"
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    if hasattr(evidence_probe, "accesses"):
        assert evidence_probe.accesses == 0
    assert hostile_catalog.accesses == 0


def _manifest_with_marker_and_unrelated_failure(marker_kind, failure_kind):
    manifest = _manifest()
    if marker_kind == "redistribution":
        manifest["provenance"]["redistribution_status"] = "protected_suspected"
        marker_code = "PLUGIN_PROTECTED_CONTENT_SUSPECTED"
    elif marker_kind == "review":
        manifest["provenance"]["review_status"] = "quarantined"
        marker_code = "PLUGIN_PROTECTED_CONTENT_SUSPECTED"
    else:
        manifest["metadata"]["status"] = "quarantined"
        marker_code = "PLUGIN_MANIFEST_QUARANTINED"

    if failure_kind == "cycle":
        probe = []
        probe.append(probe)
    elif failure_kind == "nonfinite":
        probe = float("nan")
    elif failure_kind == "depth":
        probe = []
        cursor = probe
        for _ in range(600):
            child = []
            cursor.append(child)
            cursor = child
    else:
        probe = _HostileCallerDict({"untrusted": True})
    manifest["unrelated_raw_probe"] = probe
    return manifest, probe, marker_code


@pytest.mark.parametrize(
    "marker_kind", ["redistribution", "review", "metadata"]
)
@pytest.mark.parametrize(
    "failure_kind", ["cycle", "nonfinite", "depth", "hostile"]
)
def test_manifest_safe_quarantine_markers_survive_unrelated_raw_failure_directly(
    marker_kind,
    failure_kind,
):
    manifest, probe, marker_code = _manifest_with_marker_and_unrelated_failure(
        marker_kind, failure_kind
    )

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert result.quarantined is True
    assert {"PLUGIN_MANIFEST_MALFORMED", marker_code} <= _codes(result)
    if hasattr(probe, "accesses"):
        assert probe.accesses == 0
    if failure_kind == "cycle":
        assert probe[0] is probe
    if failure_kind == "nonfinite":
        assert probe != probe


@pytest.mark.parametrize(
    "marker_kind", ["redistribution", "review", "metadata"]
)
@pytest.mark.parametrize(
    "failure_kind", ["cycle", "nonfinite", "depth", "hostile"]
)
def test_manifest_safe_quarantine_markers_survive_unrelated_raw_failure_composed(
    marker_kind,
    failure_kind,
):
    manifest, probe, marker_code = _manifest_with_marker_and_unrelated_failure(
        marker_kind, failure_kind
    )

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert result.manifest_verified is False
    assert {"PLUGIN_MANIFEST_MALFORMED", marker_code} <= _codes(result)
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    marker_diagnostic = _diagnostics_by_code(result)[marker_code]
    assert marker_diagnostic["provenance"] == manifest["provenance"]
    if hasattr(probe, "accesses"):
        assert probe.accesses == 0


def _manifest_bound_overflow(bound_kind):
    manifest = _manifest()
    if bound_kind == "nodes":
        probe = [None] * (MAX_MANIFEST_JSON_NODES + 1)
    elif bound_kind == "text":
        probe = "x" * (MAX_MANIFEST_TEXT_BYTES + 1)
    else:
        manifest["serialized_probe"] = ""
        baseline = len(
            json.dumps(
                manifest,
                sort_keys=True,
                separators=(",", ":"),
                ensure_ascii=False,
                allow_nan=False,
            ).encode("utf-8")
        )
        probe = "x" * (MAX_MANIFEST_SERIALIZED_BYTES - baseline + 1)
    manifest["bounded_raw_probe" if bound_kind != "serialized" else "serialized_probe"] = probe
    return manifest, probe


@pytest.mark.parametrize("bound_kind", ["nodes", "text", "serialized"])
def test_manifest_deterministic_resource_bounds_fail_closed_directly(bound_kind):
    manifest, probe = _manifest_bound_overflow(bound_kind)

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "REJECTED"
    assert result.verified is False
    assert [finding.code for finding in result.findings] == [
        "PLUGIN_MANIFEST_MALFORMED"
    ]
    assert result.findings[0].path == (
        "plugin_manifest"
        if bound_kind == "serialized"
        else "plugin_manifest.bounded_raw_probe"
    )
    assert manifest[
        "bounded_raw_probe" if bound_kind != "serialized" else "serialized_probe"
    ] is probe


@pytest.mark.parametrize("bound_kind", ["nodes", "text", "serialized"])
def test_manifest_deterministic_resource_bounds_fail_closed_composed(bound_kind):
    manifest, probe = _manifest_bound_overflow(bound_kind)

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result)
    assert result.manifest_verified is False
    assert "PLUGIN_MANIFEST_MALFORMED" in _codes(result)
    assert manifest[
        "bounded_raw_probe" if bound_kind != "serialized" else "serialized_probe"
    ] is probe


def _large_unit_evidence_with_late_marker(marker_index, marker_field, marker_value):
    template = _quantity_evidence()[0]
    evidence = [
        json.loads(json.dumps(template)) for _ in range(marker_index + 1)
    ]
    for index, item in enumerate(evidence):
        item["path"] = f"plugin_output.force_{index}"
    evidence[marker_index]["quantity"]["provenance"][marker_field] = marker_value
    hostile = _HostileCallerDict({"untrusted": True})
    evidence[0] = hostile
    return evidence, hostile


@pytest.mark.parametrize("marker_index", [1024, 1500])
@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_complete_bounded_unit_marker_collection_includes_index_1024_and_later(
    marker_index,
    marker_field,
    marker_value,
):
    evidence, hostile = _large_unit_evidence_with_late_marker(
        marker_index, marker_field, marker_value
    )

    result = _verify(unit_evidence=evidence)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    diagnostics = _diagnostics_by_code(result)
    assert "PLUGIN_UNIT_EVIDENCE_MALFORMED" in diagnostics
    marker = diagnostics["PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED"]
    assert marker["affected_object"]["ref_id"] == (
        f"plugin_output.force_{marker_index}"
    )
    assert marker["provenance"][marker_field] == marker_value
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert result.verification_passed is False
    assert result.runtime_dispatched is False
    assert hostile.accesses == 0


@pytest.mark.parametrize("colliding_target", ["metadata", "provenance"])
def test_manifest_marker_fallback_skips_colliding_hostile_key_directly(
    colliding_target,
):
    manifest = _manifest()
    del manifest[colliding_target]
    hostile_key = _CollidingHostileKey(colliding_target)
    manifest[hostile_key] = {"untrusted": True}
    if colliding_target == "metadata":
        manifest["provenance"]["redistribution_status"] = "protected_suspected"
        marker_code = "PLUGIN_PROTECTED_CONTENT_SUSPECTED"
    else:
        manifest["metadata"] = {
            "plugin_id": "ops-plugin-collision-test",
            "status": "quarantined",
        }
        marker_code = "PLUGIN_MANIFEST_QUARANTINED"

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert {"PLUGIN_MANIFEST_MALFORMED", marker_code} <= _codes(result)
    assert hostile_key.equality_calls == 0


@pytest.mark.parametrize("colliding_target", ["metadata", "provenance"])
def test_manifest_marker_fallback_skips_colliding_hostile_key_composed(
    colliding_target,
):
    manifest = _manifest()
    del manifest[colliding_target]
    hostile_key = _CollidingHostileKey(colliding_target)
    manifest[hostile_key] = {"untrusted": True}
    if colliding_target == "metadata":
        manifest["provenance"]["review_status"] = "quarantined"
        marker_code = "PLUGIN_PROTECTED_CONTENT_SUSPECTED"
    else:
        manifest["metadata"] = {
            "plugin_id": "ops-plugin-collision-test",
            "status": "quarantined",
        }
        marker_code = "PLUGIN_MANIFEST_QUARANTINED"

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert result.manifest_verified is False
    assert {"PLUGIN_MANIFEST_MALFORMED", marker_code} <= _codes(result)
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert hostile_key.equality_calls == 0


def test_composed_adapter_marker_fallback_skips_colliding_hostile_key():
    adapter = _adapter()
    del adapter["adapter_declaration"]
    hostile_key = _CollidingHostileKey("adapter_declaration")
    adapter[hostile_key] = {"untrusted": True}
    adapter["operation_result"]["privacy"]["classification"] = (
        "protected_suspected"
    )

    result = _verify(adapter=adapter)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert "ADAPTER_PRIVACY_PROTECTED_CONTENT_SUSPECTED" in _codes(result)
    assert result.declaration_accepted is False
    assert hostile_key.equality_calls == 0


def test_composed_unit_marker_fallback_skips_colliding_hostile_key():
    evidence = _quantity_evidence()
    entry = evidence[0]
    del entry["path"]
    hostile_key = _CollidingHostileKey("path")
    entry[hostile_key] = "untrusted"
    entry["quantity"]["provenance"]["review_status"] = "quarantined"

    result = _verify(unit_evidence=evidence)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert "PLUGIN_UNIT_EVIDENCE_MALFORMED" in _codes(result)
    assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" in _codes(result)
    assert hostile_key.equality_calls == 0


@pytest.mark.parametrize(
    ("marker_index", "expected_outcome"),
    [
        (MAX_UNIT_EVIDENCE_FALLBACK_ITEMS - 1, "QUARANTINE"),
        (MAX_UNIT_EVIDENCE_FALLBACK_ITEMS, "REJECTED"),
    ],
)
def test_unit_fallback_budget_has_explicit_inclusive_marker_boundary(
    marker_index,
    expected_outcome,
):
    evidence, hostile = _large_unit_evidence_with_late_marker(
        marker_index,
        "redistribution_status",
        "protected_suspected",
    )
    if len(evidence) == MAX_UNIT_EVIDENCE_FALLBACK_ITEMS:
        evidence.append({"unrelated": True})
    assert len(evidence) > MAX_UNIT_EVIDENCE_FALLBACK_ITEMS

    result = _verify(unit_evidence=evidence)

    _assert_canonical_fail_closed(result, expected_outcome)
    assert "PLUGIN_UNIT_EVIDENCE_MALFORMED" in _codes(result)
    if expected_outcome == "QUARANTINE":
        assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" in _codes(result)
        assert result.result_envelope["privacy"]["classification"] == (
            "protected_suspected"
        )
    else:
        assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" not in _codes(result)
    assert hostile.accesses == 0


@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_over_limit_manifest_provenance_retains_only_bounded_marker_directly(
    marker_field,
    marker_value,
):
    manifest = _manifest()
    manifest["provenance"][marker_field] = marker_value
    oversized = "x" * (MAX_MANIFEST_TEXT_BYTES + 1)
    manifest["provenance"]["oversized_raw_evidence"] = oversized

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert "PLUGIN_PROTECTED_CONTENT_SUSPECTED" in _codes(result)
    assert manifest["provenance"]["oversized_raw_evidence"] is oversized


@pytest.mark.parametrize(
    ("marker_field", "marker_value"),
    [
        ("redistribution_status", "protected_suspected"),
        ("review_status", "quarantined"),
    ],
)
def test_over_limit_manifest_provenance_is_not_copied_into_composed_diagnostics(
    marker_field,
    marker_value,
):
    manifest = _manifest()
    manifest["provenance"][marker_field] = marker_value
    oversized = "x" * (MAX_MANIFEST_TEXT_BYTES + 1)
    manifest["provenance"]["oversized_raw_evidence"] = oversized

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    marker = _diagnostics_by_code(result)["PLUGIN_PROTECTED_CONTENT_SUSPECTED"]
    assert marker["provenance"][marker_field] == marker_value
    assert "oversized_raw_evidence" not in marker["provenance"]
    assert oversized not in marker["provenance"].values()
    assert "oversized_raw_evidence" not in result.result_envelope["provenance"]
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )


def _manifest_with_unsafe_fallback_plugin_id(identifier_kind):
    manifest = _manifest()
    manifest["metadata"]["status"] = "quarantined"
    if identifier_kind == "over_limit":
        plugin_id = "ops-plugin-" + (
            "a" * (MAX_MANIFEST_TEXT_BYTES + 1)
        )
    elif identifier_kind == "fallback_limit":
        plugin_id = "ops-plugin-" + (
            "a" * (MAX_MANIFEST_FALLBACK_IDENTIFIER_BYTES + 1)
        )
        cycle = []
        cycle.append(cycle)
        manifest["unrelated_raw_probe"] = cycle
    elif identifier_kind == "hostile":
        plugin_id = _RaisingEqualityString("ops-plugin-hostile-id")
    else:
        plugin_id = "NOT CANONICAL"
        cycle = []
        cycle.append(cycle)
        manifest["unrelated_raw_probe"] = cycle
    manifest["metadata"]["plugin_id"] = plugin_id
    return manifest, plugin_id


@pytest.mark.parametrize(
    "identifier_kind",
    ["over_limit", "fallback_limit", "hostile", "noncanonical"],
)
def test_unsafe_manifest_fallback_plugin_id_never_escapes_directly(identifier_kind):
    manifest, plugin_id = _manifest_with_unsafe_fallback_plugin_id(identifier_kind)

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert "PLUGIN_MANIFEST_QUARANTINED" in _codes(result)
    assert manifest["metadata"]["plugin_id"] is plugin_id


@pytest.mark.parametrize(
    "identifier_kind",
    ["over_limit", "fallback_limit", "hostile", "noncanonical"],
)
def test_unsafe_manifest_fallback_plugin_id_is_tbd_in_composed_diagnostic(
    identifier_kind,
):
    manifest, plugin_id = _manifest_with_unsafe_fallback_plugin_id(identifier_kind)

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    marker = _diagnostics_by_code(result)["PLUGIN_MANIFEST_QUARANTINED"]
    assert marker["source"] == {"ref_type": "payload", "ref_id": "TBD"}
    assert marker["source"]["ref_id"] != plugin_id
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    assert result.runtime_dispatched is False


def test_oversized_adapter_capabilities_are_bounded_in_composed_quarantine():
    adapter = _adapter()
    capabilities = ["import_model"] * (MAX_CALLER_JSON_NODES + 1)
    adapter["adapter_declaration"]["capabilities"] = capabilities
    adapter["operation_result"]["provenance"]["review_status"] = "quarantined"

    result = _verify(adapter=adapter)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    diagnostics = _diagnostics_by_code(result)
    assert "ADAPTER_CAPABILITIES_MALFORMED" in diagnostics
    assert "ADAPTER_PROTECTED_CONTENT_SUSPECTED" in diagnostics
    assert diagnostics["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]["provenance"][
        "review_status"
    ] == "quarantined"
    assert adapter["adapter_declaration"]["capabilities"] is capabilities


@pytest.mark.parametrize(
    "adapter_id",
    [
        "ops.adapter." + ("a" * (MAX_CALLER_JSON_BYTES + 1)),
        "ops.adapter.NOT-CANONICAL",
    ],
)
def test_unsafe_adapter_id_never_enters_composed_quarantine_diagnostic(adapter_id):
    adapter = _adapter()
    adapter["adapter_declaration"]["adapter_id"] = adapter_id
    adapter["adapter_declaration"]["provenance"][
        "redistribution_status"
    ] = "protected_suspected"

    result = _verify(adapter=adapter)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    marker = _diagnostics_by_code(result)["ADAPTER_PROTECTED_CONTENT_SUSPECTED"]
    assert marker["source"] == {"ref_type": "adapter", "ref_id": "TBD"}
    assert marker["source"]["ref_id"] != adapter_id
    assert adapter_id not in json.dumps(result.result_envelope, sort_keys=True)
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )


@pytest.mark.parametrize(
    "declared_path",
    [
        "plugin_output." + ("x" * (MAX_CALLER_JSON_BYTES + 1)),
        "plugin output/value",
    ],
)
def test_unsafe_unit_path_is_bounded_directly_during_quarantine(declared_path):
    evidence = _quantity_evidence()
    evidence[0]["path"] = declared_path
    evidence[0]["quantity"]["provenance"][
        "review_status"
    ] = "quarantined"
    cycle = []
    cycle.append(cycle)
    evidence[0]["caller_probe"] = cycle

    normalized = _normalize_unit_inputs(evidence, UNIT_CATALOG)

    assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" in {
        finding.code for finding in normalized.findings
    }
    marker_index = next(
        index
        for index, finding in enumerate(normalized.findings)
        if finding.code == "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED"
    )
    context = normalized.diagnostic_contexts[marker_index]
    assert context["source"]["ref_id"] == "unit_evidence[0]"
    assert context["affected_object"]["ref_id"] == "unit_evidence[0]"
    assert context["source"]["ref_id"] != declared_path


@pytest.mark.parametrize(
    "declared_path",
    [
        "plugin_output." + ("x" * (MAX_CALLER_JSON_BYTES + 1)),
        "plugin output/value",
    ],
)
def test_unsafe_unit_path_never_enters_composed_quarantine_diagnostic(
    declared_path,
):
    evidence = _quantity_evidence()
    evidence[0]["path"] = declared_path
    evidence[0]["quantity"]["provenance"][
        "redistribution_status"
    ] = "protected_suspected"

    result = _verify(unit_evidence=evidence)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    marker = _diagnostics_by_code(result)[
        "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED"
    ]
    assert marker["source"]["ref_id"] == "unit_evidence[0]"
    assert marker["affected_object"]["ref_id"] == "unit_evidence[0]"
    assert marker["source"]["ref_id"] != declared_path
    assert declared_path not in json.dumps(result.result_envelope, sort_keys=True)
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )


def _adversarial_preflight_key(key_kind):
    if key_kind == "over_limit":
        return "x" * (MAX_DIAGNOSTIC_PATH_SEGMENT_BYTES + 1)
    return "raw key/../not-canonical"


@pytest.mark.parametrize("key_kind", ["over_limit", "noncanonical"])
@pytest.mark.parametrize("surface", ["manifest", "catalog", "evidence"])
def test_adversarial_preflight_keys_are_sanitized_directly(surface, key_kind):
    raw_key = _adversarial_preflight_key(key_kind)
    if surface == "manifest":
        manifest = _manifest()
        manifest[raw_key] = float("nan")
        manifest["provenance"]["review_status"] = "quarantined"

        result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

        assert result.outcome == "QUARANTINE"
        malformed = next(
            finding
            for finding in result.findings
            if finding.code == "PLUGIN_MANIFEST_MALFORMED"
        )
        assert "PLUGIN_PROTECTED_CONTENT_SUSPECTED" in _codes(result)
    else:
        evidence = _quantity_evidence()
        catalog = dict(UNIT_CATALOG)
        expected_code = "PLUGIN_UNIT_CATALOG_MALFORMED"
        if surface == "catalog":
            catalog[raw_key] = float("nan")
        else:
            evidence[0][raw_key] = float("nan")
            evidence[0]["quantity"]["provenance"][
                "redistribution_status"
            ] = "protected_suspected"
            expected_code = "PLUGIN_UNIT_EVIDENCE_MALFORMED"

        normalized = _normalize_unit_inputs(evidence, catalog)

        malformed = next(
            finding
            for finding in normalized.findings
            if finding.code == expected_code
        )
        assert raw_key not in repr(normalized.diagnostic_contexts)
        if surface == "evidence":
            assert "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED" in {
                finding.code for finding in normalized.findings
            }
    assert raw_key not in malformed.path
    assert "key_" in malformed.path
    assert len(malformed.path.encode("utf-8")) < 512


@pytest.mark.parametrize("key_kind", ["over_limit", "noncanonical"])
@pytest.mark.parametrize(
    ("surface", "malformed_code"),
    [
        ("adapter", "ADAPTER_DECLARATION_MALFORMED"),
        ("manifest", "PLUGIN_MANIFEST_MALFORMED"),
        ("catalog", "PLUGIN_UNIT_CATALOG_MALFORMED"),
        ("evidence", "PLUGIN_UNIT_EVIDENCE_MALFORMED"),
    ],
)
def test_adversarial_preflight_keys_never_enter_composed_diagnostics(
    surface,
    malformed_code,
    key_kind,
):
    raw_key = _adversarial_preflight_key(key_kind)
    adapter = _adapter()
    manifest = _manifest()
    evidence = _quantity_evidence()
    catalog = dict(UNIT_CATALOG)
    if surface == "adapter":
        adapter[raw_key] = float("nan")
        adapter["operation_result"]["provenance"][
            "review_status"
        ] = "quarantined"
    elif surface == "manifest":
        manifest[raw_key] = float("nan")
        manifest["provenance"]["redistribution_status"] = (
            "protected_suspected"
        )
    elif surface == "catalog":
        catalog[raw_key] = float("nan")
        evidence[0]["quantity"]["provenance"]["review_status"] = (
            "quarantined"
        )
    else:
        evidence[0][raw_key] = float("nan")
        evidence[0]["quantity"]["provenance"][
            "redistribution_status"
        ] = "protected_suspected"

    result = _verify(
        adapter=adapter,
        manifest=manifest,
        unit_evidence=evidence,
        unit_catalog=catalog,
    )

    _assert_canonical_fail_closed(result, "QUARANTINE")
    malformed = _diagnostics_by_code(result)[malformed_code]
    assert raw_key not in json.dumps(result.result_envelope, sort_keys=True)
    assert "key_" in malformed["affected_object"]["ref_id"]
    assert len(malformed["affected_object"]["ref_id"].encode("utf-8")) < 512
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )


def _adversarial_plugin_schema(failure_kind):
    schema = json.loads(json.dumps(PLUGIN_SCHEMA))
    if failure_kind == "hostile":
        return _HostileCallerDict(schema)
    if failure_kind == "depth":
        probe = _deep_caller_probe()
    elif failure_kind == "cycle":
        probe = {}
        probe["self"] = probe
    else:
        probe = float("nan")
    schema["caller_probe"] = probe
    return schema


@pytest.mark.parametrize(
    "failure_kind", ["hostile", "depth", "cycle", "nonfinite"]
)
def test_adversarial_plugin_schema_fails_closed_directly_without_masking_quarantine(
    failure_kind,
):
    manifest = _manifest()
    manifest["provenance"]["review_status"] = "quarantined"
    schema = _adversarial_plugin_schema(failure_kind)

    result = verify_plugin_manifest(manifest, schema)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert {
        "PLUGIN_MANIFEST_SCHEMA_MALFORMED",
        "PLUGIN_PROTECTED_CONTENT_SUSPECTED",
    } <= _codes(result)
    if hasattr(schema, "accesses"):
        assert schema.accesses == 0


@pytest.mark.parametrize(
    "failure_kind", ["hostile", "depth", "cycle", "nonfinite"]
)
def test_adversarial_plugin_schema_fails_closed_composed_without_masking_quarantine(
    failure_kind,
):
    manifest = _manifest()
    manifest["provenance"]["redistribution_status"] = "protected_suspected"
    schema = _adversarial_plugin_schema(failure_kind)

    result = _verify(manifest=manifest, schema=schema)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert {
        "PLUGIN_MANIFEST_SCHEMA_MALFORMED",
        "PLUGIN_PROTECTED_CONTENT_SUSPECTED",
    } <= _codes(result)
    assert result.manifest_verified is False
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    if hasattr(schema, "accesses"):
        assert schema.accesses == 0


SCHEMA_DIAGNOSTIC_PATH_PATTERN = re.compile(
    r"^[A-Za-z_][A-Za-z0-9_]*(?:\.[A-Za-z_][A-Za-z0-9_]*|\[[0-9]+\])*$"
)


def _finite_schema_diagnostic_probe(probe_kind):
    manifest = _manifest()
    manifest["provenance"]["redistribution_status"] = "protected_suspected"
    if probe_kind == "adversarial_key":
        raw = 'raw/key\n../["not-canonical"]'
        manifest[raw] = True
    elif probe_kind == "huge_key":
        raw = "k" * (MAX_MANIFEST_SERIALIZED_BYTES - (64 * 1024))
        manifest[raw] = True
    elif probe_kind == "adversarial_instance":
        raw = 'raw enum\nvalue\x00../["not-canonical"]'
        manifest["metadata"]["status"] = raw
    else:
        raw = "v" * (MAX_MANIFEST_SERIALIZED_BYTES - (64 * 1024))
        manifest["metadata"]["status"] = raw
    return manifest, raw


@pytest.mark.parametrize(
    "probe_kind",
    ["adversarial_key", "huge_key", "adversarial_instance", "huge_instance"],
)
def test_finite_schema_diagnostics_are_canonical_and_bounded_directly(probe_kind):
    manifest, raw = _finite_schema_diagnostic_probe(probe_kind)

    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert {
        "PLUGIN_MANIFEST_SCHEMA_MISMATCH",
        "PLUGIN_PROTECTED_CONTENT_SUSPECTED",
    } <= _codes(result)
    mismatch = next(
        finding
        for finding in result.findings
        if finding.code == "PLUGIN_MANIFEST_SCHEMA_MISMATCH"
    )
    assert SCHEMA_DIAGNOSTIC_PATH_PATTERN.fullmatch(mismatch.path)
    assert len(mismatch.path.encode("utf-8")) <= MAX_DIAGNOSTIC_PATH_BYTES
    assert len(mismatch.message.encode("utf-8")) <= (
        MAX_SCHEMA_MISMATCH_INSTANCE_BYTES + 128
    )
    assert raw not in mismatch.path
    assert raw not in mismatch.message
    if probe_kind == "adversarial_instance":
        assert "\n" not in mismatch.message
        assert "\x00" not in mismatch.message
        assert "\\n" in mismatch.message
        assert "\\u0000" in mismatch.message
    elif probe_kind == "huge_instance":
        assert "sha256=" in mismatch.message


@pytest.mark.parametrize(
    "probe_kind",
    ["adversarial_key", "huge_key", "adversarial_instance", "huge_instance"],
)
def test_finite_schema_diagnostics_stay_safe_in_composed_envelope(probe_kind):
    manifest, raw = _finite_schema_diagnostic_probe(probe_kind)

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert result.runtime_dispatched is False
    assert result.manifest_verified is False
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    diagnostic = _diagnostics_by_code(result)["PLUGIN_MANIFEST_SCHEMA_MISMATCH"]
    reference = diagnostic["affected_object"]["ref_id"]
    source_reference = diagnostic["source"]["ref_id"]
    assert source_reference == "ops-plugin-invented-no-bypass"
    assert SCHEMA_DIAGNOSTIC_PATH_PATTERN.fullmatch(reference)
    assert len(reference.encode("utf-8")) <= MAX_DIAGNOSTIC_PATH_BYTES
    assert len(diagnostic["message"].encode("utf-8")) <= (
        MAX_SCHEMA_MISMATCH_INSTANCE_BYTES + 128
    )
    assert raw not in reference
    assert raw not in source_reference
    assert raw not in diagnostic["message"]
    if probe_kind == "adversarial_instance":
        assert "\n" not in diagnostic["message"]
        assert "\x00" not in diagnostic["message"]
        assert "\\n" in diagnostic["message"]
        assert "\\u0000" in diagnostic["message"]
    elif probe_kind == "huge_instance":
        assert "sha256=" in diagnostic["message"]


def _schema_valid_unsafe_plugin_reference(identifier_kind):
    prefix = "ops-plugin-"
    if identifier_kind == "over_safe_ref_limit":
        suffix_length = MAX_MANIFEST_FALLBACK_IDENTIFIER_BYTES - len(prefix) + 1
    else:
        suffix_length = MAX_MANIFEST_SERIALIZED_BYTES - (64 * 1024) - len(prefix)
    plugin_id = prefix + ("a" * suffix_length)
    manifest = _manifest()
    manifest["metadata"]["plugin_id"] = plugin_id
    manifest["provenance"]["redistribution_status"] = "protected_suspected"
    return manifest, plugin_id


@pytest.mark.parametrize(
    "identifier_kind", ["over_safe_ref_limit", "near_manifest_limit"]
)
def test_schema_valid_unsafe_plugin_ids_do_not_enter_direct_findings(
    identifier_kind,
):
    manifest, plugin_id = _schema_valid_unsafe_plugin_reference(identifier_kind)

    assert _schema_mismatches(
        manifest,
        PLUGIN_SCHEMA,
        PLUGIN_SCHEMA,
        "plugin_manifest",
    ) == []
    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert "PLUGIN_PROTECTED_CONTENT_SUSPECTED" in _codes(result)
    for finding in result.findings:
        assert plugin_id not in finding.path
        assert plugin_id not in finding.message
        assert plugin_id not in finding.remediation
        assert len(finding.path.encode("utf-8")) <= MAX_DIAGNOSTIC_PATH_BYTES
        assert len(finding.message.encode("utf-8")) <= 512


@pytest.mark.parametrize(
    "identifier_kind", ["over_safe_ref_limit", "near_manifest_limit"]
)
def test_schema_valid_unsafe_plugin_ids_are_tbd_in_composed_diagnostics(
    identifier_kind,
):
    manifest, plugin_id = _schema_valid_unsafe_plugin_reference(identifier_kind)

    result = _verify(manifest=manifest)

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert result.runtime_dispatched is False
    assert result.manifest_verified is False
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    marker = _diagnostics_by_code(result)["PLUGIN_PROTECTED_CONTENT_SUSPECTED"]
    assert marker["source"] == {"ref_type": "payload", "ref_id": "TBD"}
    assert marker["affected_object"]["ref_id"] == "plugin_manifest.provenance"
    assert len(marker["source"]["ref_id"].encode("utf-8")) <= (
        MAX_MANIFEST_FALLBACK_IDENTIFIER_BYTES
    )
    assert len(marker["message"].encode("utf-8")) <= 512
    assert plugin_id not in json.dumps(result.result_envelope, sort_keys=True)


@pytest.mark.parametrize(
    "redistribution_status",
    ["public_permissive", "private_only", "protected_suspected"],
)
def test_schema_valid_quarantined_metadata_is_quarantined_directly(
    redistribution_status,
):
    manifest = _manifest()
    manifest["metadata"]["status"] = "quarantined"
    manifest["provenance"]["redistribution_status"] = redistribution_status

    assert _schema_mismatches(
        manifest,
        PLUGIN_SCHEMA,
        PLUGIN_SCHEMA,
        "plugin_manifest",
    ) == []
    result = verify_plugin_manifest(manifest, PLUGIN_SCHEMA)

    assert result.outcome == "QUARANTINE"
    assert result.verified is False
    assert result.quarantined is True
    assert "PLUGIN_MANIFEST_QUARANTINED" in _codes(result)


@pytest.mark.parametrize(
    ("other_boundary", "expected_provenance_source"),
    [
        ("public", "manifest"),
        ("private", "manifest"),
        ("protected", "adapter"),
    ],
)
def test_schema_valid_quarantined_metadata_forces_protected_composed_envelope(
    other_boundary,
    expected_provenance_source,
):
    manifest = _manifest()
    manifest["metadata"]["status"] = "quarantined"
    adapter = _adapter()
    evidence = _quantity_evidence()
    if other_boundary == "private":
        evidence[0]["quantity"]["provenance"]["redistribution_status"] = (
            "private_only"
        )
    elif other_boundary == "protected":
        adapter["operation_result"]["provenance"][
            "redistribution_status"
        ] = "protected_suspected"

    result = _verify(
        adapter=adapter,
        manifest=manifest,
        unit_evidence=evidence,
    )

    _assert_canonical_fail_closed(result, "QUARANTINE")
    assert result.manifest_verified is False
    assert result.runtime_dispatched is False
    assert "PLUGIN_MANIFEST_QUARANTINED" in _codes(result)
    assert result.result_envelope["privacy"]["classification"] == (
        "protected_suspected"
    )
    expected_provenance = (
        adapter["operation_result"]["provenance"]
        if expected_provenance_source == "adapter"
        else manifest["provenance"]
    )
    assert result.result_envelope["provenance"] == expected_provenance
    assert _schema_mismatches(
        result.result_envelope,
        ADAPTER_SCHEMA["$defs"]["AdapterOperationResult"],
        ADAPTER_SCHEMA,
        "operation_result",
    ) == []
