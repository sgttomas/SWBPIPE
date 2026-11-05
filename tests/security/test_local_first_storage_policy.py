#!/usr/bin/env python3
"""Policy checks for the local-first storage contract."""

import json
from pathlib import Path
import sys


ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))

from core.security.local_first_storage import (  # noqa: E402
    classify_storage_record,
    enforce_local_first_route,
    guard_storage_records,
    storage_record,
)


def decision_codes(result):
    return {decision.reason_code for decision in result.decisions}


def diagnostic_codes(result):
    return {diagnostic.code for diagnostic in result.diagnostics}


def invented_project_storage_record():
    return storage_record(
        record_id="invented.project.metadata",
        record_kind="project_model",
        label="Invented project metadata reference",
        checksum="sha256:0000000000000000000000000000000000000000000000000000000000000000",
        checksum_status="placeholder_checksum_recorded",
        source_note="invented local placeholder source note",
        redistribution_status="private_only",
        review_status="pending",
    )


def test_storage_record_classification_is_deterministic_and_metadata_only():
    first = classify_storage_record(invented_project_storage_record())
    second = classify_storage_record(invented_project_storage_record())

    assert first == second
    assert first.decision_id.startswith("LFS-CLS-")
    assert first.reason_code == "PRIVATE_METADATA_LOCAL_FIRST"
    assert first.default_posture == "local_private_metadata_requires_explicit_intent"
    assert first.metadata["storage_locality"] == "USER_CHOSEN_PROJECT_PACKAGE"
    assert first.metadata["contains_payload"] is False
    assert first.metadata["secret_material_present"] is False
    assert first.metadata["direct_sql_access"] is False


def test_public_repository_blocks_private_storage_payloads_without_leaking_values():
    unsafe_records = [
        {
            "record_id": "invented.project.payload",
            "record_kind": "project_model",
            "label": "Invented project payload",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "project_payload": {"invented": "PROJECT_PAYLOAD_SHOULD_NOT_SURVIVE"},
        },
        {
            "record_id": "invented.rule.payload",
            "record_kind": "private_rule_pack",
            "label": "Invented rule payload",
            "privacy_classification": "private_rule_pack_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "rule_payload": "RULE_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.material.payload",
            "record_kind": "private_material_library",
            "label": "Invented material payload",
            "privacy_classification": "private_material_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "material_values": {"invented": "MATERIAL_PAYLOAD_SHOULD_NOT_SURVIVE"},
        },
        {
            "record_id": "invented.component.payload",
            "record_kind": "private_component_library",
            "label": "Invented component payload",
            "privacy_classification": "private_component_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "component_values": {"invented": "COMPONENT_PAYLOAD_SHOULD_NOT_SURVIVE"},
        },
        {
            "record_id": "invented.report.payload",
            "record_kind": "private_report",
            "label": "Invented report payload",
            "privacy_classification": "private_report_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "report_payload": "REPORT_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.diagnostic.payload",
            "record_kind": "private_diagnostic",
            "label": "Invented diagnostic payload",
            "privacy_classification": "private_diagnostic_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "diagnostic_payload": "DIAGNOSTIC_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.cache.payload",
            "record_kind": "private_cache",
            "label": "Invented cache payload",
            "privacy_classification": "private_cache_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "cache_payload": "CACHE_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.secret.payload",
            "record_kind": "secret_reference",
            "label": "Invented secret payload",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "credential_reference",
            "secret_value": "SECRET_PAYLOAD_SHOULD_NOT_SURVIVE",
        },
    ]

    result = guard_storage_records(
        unsafe_records,
        target_context="public_repository",
    )
    serialized = json.dumps(result.as_schema_dict(), sort_keys=True)

    assert result.blocked is True
    assert result.summary()["metadata_only"] is True
    assert decision_codes(result) == {
        "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED",
        "SECRET_MATERIAL_METADATA_ONLY_REQUIRED",
    }
    assert "STORAGE_PAYLOAD_METADATA_ONLY_REQUIRED" in diagnostic_codes(result)
    for leaked in {
        "PROJECT_PAYLOAD_SHOULD_NOT_SURVIVE",
        "RULE_PAYLOAD_SHOULD_NOT_SURVIVE",
        "MATERIAL_PAYLOAD_SHOULD_NOT_SURVIVE",
        "COMPONENT_PAYLOAD_SHOULD_NOT_SURVIVE",
        "REPORT_PAYLOAD_SHOULD_NOT_SURVIVE",
        "DIAGNOSTIC_PAYLOAD_SHOULD_NOT_SURVIVE",
        "CACHE_PAYLOAD_SHOULD_NOT_SURVIVE",
        "SECRET_PAYLOAD_SHOULD_NOT_SURVIVE",
    }:
        assert leaked not in serialized


def test_local_private_storage_requires_explicit_user_intent_for_private_metadata():
    blocked = guard_storage_records(
        [invented_project_storage_record()],
        target_context="local_private",
        explicit_user_intent=False,
    )
    allowed = guard_storage_records(
        [invented_project_storage_record()],
        target_context="local_private",
        explicit_user_intent=True,
    )

    assert blocked.blocked is True
    assert "LOCAL_PRIVATE_INTENT_REQUIRED" in decision_codes(blocked)
    assert allowed.blocked is False
    assert "PRIVATE_LOCAL_METADATA_ALLOWED" in decision_codes(allowed)
    assert allowed.safe_manifest[0]["record_id"] == "invented.project.metadata"
    assert allowed.safe_manifest[0]["contains_payload"] is False


def test_governed_route_enforcement_is_metadata_only_and_fails_closed():
    public = enforce_local_first_route(
        route_id="REXC-CORE-001",
        export_context="downstream_tool",
    )
    public_example = enforce_local_first_route(
        route_id="DOTH-JSON-001",
        export_context="public_example",
    )
    blocked_local = enforce_local_first_route(
        route_id="REXC-CORE-007",
        export_context="local_private",
    )
    allowed_local = enforce_local_first_route(
        route_id="REXC-CORE-007",
        export_context="local_private",
        explicit_local_private_intent=True,
    )
    unknown = enforce_local_first_route(
        route_id="INVENTED-UNKNOWN-ROUTE",
        export_context="downstream_tool",
        explicit_local_private_intent=True,
    )

    assert public.blocked is False
    assert public.reason_code == "SAFE_PUBLIC_METADATA"
    assert public_example.blocked is False
    assert public_example.reason_code == "SAFE_PUBLIC_METADATA"
    assert blocked_local.blocked is True
    assert blocked_local.reason_code == "LOCAL_PRIVATE_INTENT_REQUIRED"
    assert allowed_local.blocked is False
    assert allowed_local.reason_code == "PRIVATE_LOCAL_METADATA_ALLOWED"
    assert unknown.blocked is True
    assert unknown.reason_code == "LOCAL_FIRST_ROUTE_UNKNOWN"
    assert all(
        decision.metadata_only
        for decision in (public, public_example, blocked_local, allowed_local, unknown)
    )


def test_public_repository_locality_blocks_even_with_local_private_intent():
    result = guard_storage_records(
        [
            storage_record(
                record_id="invented.public.repo.private.target",
                record_kind="project_model",
                label="Invented public repository private target",
                storage_locality="PUBLIC_REPOSITORY_CONTENT",
                redistribution_status="private_only",
                review_status="pending",
                source_note="invented local placeholder source note",
            )
        ],
        target_context="local_private",
        explicit_user_intent=True,
    )

    assert result.blocked is True
    assert result.decisions[0].blocked is True
    assert "PUBLIC_REPOSITORY_PRIVATE_STORAGE_BLOCKED" in diagnostic_codes(result)


def test_cloud_network_direct_sql_secret_and_concrete_path_details_are_sanitized():
    records = [
        {
            "record_id": "invented.cloud.reference",
            "record_kind": "project_model",
            "label": "Invented cloud reference",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "cloud_or_network_reference": True,
            "cloud_sync_target": "CLOUD_SYNC_TARGET_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.direct.sql",
            "record_kind": "project_store",
            "label": "Invented direct SQL reference",
            "privacy_classification": "private_project_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "table_name": "SQL_TABLE_DETAIL_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.secret.material",
            "record_kind": "secret_reference",
            "label": "Invented secret material",
            "privacy_classification": "secret_like_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "credential_reference",
            "api_key": "SECRET_MATERIAL_SHOULD_NOT_SURVIVE",
        },
        {
            "record_id": "invented.concrete.path",
            "record_kind": "private_report",
            "label": "Invented concrete path reference",
            "privacy_classification": "private_report_data",
            "redistribution_status": "private_only",
            "review_status": "pending",
            "source_state": "private_user_supplied",
            "file_path": "FAKE_CONCRETE_PRIVATE_PATH_SHOULD_NOT_SURVIVE",
        },
    ]

    result = guard_storage_records(
        records,
        target_context="local_private",
        explicit_user_intent=True,
    )
    serialized = json.dumps(result.as_schema_dict(), sort_keys=True)

    assert result.blocked is True
    assert "CLOUD_OR_NETWORK_STORAGE_BLOCKED" in decision_codes(result)
    assert "DIRECT_SQL_ACCESS_BLOCKED" in decision_codes(result)
    assert "SECRET_MATERIAL_METADATA_ONLY_REQUIRED" in decision_codes(result)
    assert "CONCRETE_PATH_REDUCED_TO_SAFE_METADATA" in decision_codes(result)
    assert any(
        decision.record_id == "invented.concrete.path"
        and decision.action == "include_metadata_only"
        for decision in result.decisions
    )
    for leaked in {
        "CLOUD_SYNC_TARGET_SHOULD_NOT_SURVIVE",
        "SQL_TABLE_DETAIL_SHOULD_NOT_SURVIVE",
        "SECRET_MATERIAL_SHOULD_NOT_SURVIVE",
        "FAKE_CONCRETE_PRIVATE_PATH_SHOULD_NOT_SURVIVE",
    }:
        assert leaked not in serialized


def test_no_runtime_storage_code_or_schema_was_added():
    assert not (ROOT / "core" / "storage").exists()
    assert not (ROOT / "apps" / "storage").exists()
    assert not (ROOT / "schemas" / "storage.schema.yaml").exists()


# RT-R6 (DEL-08-04-R14, AC-002): the CSV table route ``DOTH-CSV-011`` in the three route
# catalogues: Python (here), TypeScript (``redactionExportControls.ts``) and the Rust
# native save (``native_result_download.rs``).
CSV_ROUTE = "DOTH-CSV-011"
TS_CATALOGUE = ROOT / "apps/desktop/src/features/redaction-controls/redactionExportControls.ts"
RUST_NATIVE_SAVE = ROOT / "apps/desktop/src-tauri/src/native_result_download.rs"
ROUTE_ID_PATTERN = r'"([A-Z][A-Z0-9]*(?:-[A-Z0-9]+)+(?:/[A-Z0-9-]+)?)"'


def test_csv_route_is_admitted_only_for_local_private_with_explicit_intent():
    from core.security.local_first_storage.route_control import (
        CSV_TABLE_ROUTE_ID,
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        LOCAL_PRIVATE_ONLY_ROUTE_IDS,
    )

    assert CSV_TABLE_ROUTE_ID == CSV_ROUTE
    assert CSV_ROUTE in GOVERNED_LOCAL_FIRST_ROUTE_IDS
    # The CSV save and, since the MBF export's review round, DOTH-MBF-012 (DEL-17-04).
    assert LOCAL_PRIVATE_ONLY_ROUTE_IDS == frozenset({CSV_ROUTE, "DOTH-MBF-012"})
    allowed = enforce_local_first_route(
        route_id=CSV_ROUTE,
        export_context="local_private",
        explicit_local_private_intent=True,
    ).as_schema_dict()
    assert allowed == {
        "route_id": CSV_ROUTE,
        "export_context": "local_private",
        "storage_context": "local_private",
        "action": "include_metadata_only",
        "reason_code": "PRIVATE_LOCAL_METADATA_ALLOWED",
        "blocked": False,
        "metadata_only": True,
        "explicit_local_private_intent": True,
    }
    for intent in (False, None, "true", 1):
        without = enforce_local_first_route(
            route_id=CSV_ROUTE,
            export_context="local_private",
            explicit_local_private_intent=intent,
        )
        assert (without.blocked, without.reason_code) == (
            True,
            "LOCAL_PRIVATE_INTENT_REQUIRED",
        )
    for context in ("public_report", "public_example", "shared_model", "downstream_tool"):
        for intent in (False, True):
            refused = enforce_local_first_route(
                route_id=CSV_ROUTE,
                export_context=context,
                explicit_local_private_intent=intent,
            ).as_schema_dict()
            assert refused == {
                "route_id": CSV_ROUTE,
                "export_context": context,
                "storage_context": context,
                "action": "block_storage",
                "reason_code": "LOCAL_PRIVATE_ROUTE_REQUIRED",
                "blocked": True,
                "metadata_only": True,
                "explicit_local_private_intent": intent,
            }
    released = enforce_local_first_route(
        route_id=CSV_ROUTE,
        export_context="user_released_report",
        explicit_user_release_intent=True,
    )
    assert (released.blocked, released.reason_code) == (True, "USER_RELEASE_ROUTE_REQUIRED")
    invalid = enforce_local_first_route(route_id=CSV_ROUTE, export_context="private_report")
    assert (invalid.blocked, invalid.reason_code) == (True, "LOCAL_FIRST_EXPORT_CONTEXT_INVALID")
    for lookalike in ("DOTH-CSV-010", "DOTH-CSV-012", "doth-csv-011", "DOTH-CSV-011 "):
        unknown = enforce_local_first_route(
            route_id=lookalike,
            export_context="local_private",
            explicit_local_private_intent=True,
        )
        assert (unknown.blocked, unknown.reason_code) == (True, "LOCAL_FIRST_ROUTE_UNKNOWN")
    # The other governed routes keep their public-context behaviour.
    public = enforce_local_first_route(route_id="DOTH-JSON-001", export_context="public_example")
    assert (public.blocked, public.reason_code) == (False, "SAFE_PUBLIC_METADATA")


def test_csv_route_catalogues_agree_across_python_typescript_and_rust():
    import re

    from core.security.local_first_storage.route_control import (
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        LOCAL_PRIVATE_ONLY_ROUTE_IDS,
    )

    ts = TS_CATALOGUE.read_text(encoding="utf-8")
    ts_block = re.search(
        r"GOVERNED_LOCAL_FIRST_ROUTE_IDS: ReadonlySet<string> = new Set\(\[(.*?)\]\);",
        ts,
        re.S,
    )
    assert ts_block is not None
    ts_routes = set(re.findall(ROUTE_ID_PATTERN, ts_block.group(1)))
    # Python also carries the core-only REXC-* routes, which no desktop surface uses.
    desktop_routes = {r for r in GOVERNED_LOCAL_FIRST_ROUTE_IDS if not r.startswith("REXC-")}
    assert ts_routes == desktop_routes
    assert CSV_ROUTE in ts_routes
    assert f'export const CSV_TABLE_ROUTE_ID = "{CSV_ROUTE}";' in ts
    assert (
        "export const LOCAL_PRIVATE_ONLY_ROUTE_IDS: ReadonlySet<string> = "
        "new Set([CSV_TABLE_ROUTE_ID, MBF_EXPORT_ROUTE_ID]);"
    ) in ts
    assert LOCAL_PRIVATE_ONLY_ROUTE_IDS == frozenset({CSV_ROUTE, "DOTH-MBF-012"})

    rust = RUST_NATIVE_SAVE.read_text(encoding="utf-8")
    assert f'const CSV_ROUTE: &str = "{CSV_ROUTE}";' in rust
    profiles = re.search(r"fn profile\(route_id: &str\) -> Option<Profile> \{(.*?)\n\}", rust, re.S)
    assert profiles is not None
    rust_routes = set(re.findall(ROUTE_ID_PATTERN, profiles.group(1)))
    if "CSV_ROUTE =>" in profiles.group(1):
        rust_routes.add(CSV_ROUTE)
    assert rust_routes == {"DOTH-JSON-001", "DOTH-FORMAT-003", CSV_ROUTE}
    assert rust_routes <= GOVERNED_LOCAL_FIRST_ROUTE_IDS
    # The native save admits only local_private, and the CSV profile alone states
    # lossless_required=false.
    assert 's.export_context != "local_private"' in rust
    assert "lossless_required: false" in profiles.group(1)
    assert profiles.group(1).count("lossless_required: true") == 1  # the shared JSON profile


def test_mbf_export_route_agrees_across_python_typescript_and_rust():
    """DEL-17-04 A-4: DOTH-MBF-012 in the three catalogues; the native admission is the MBF save module's."""
    import re

    from core.security.local_first_storage.route_control import (
        GOVERNED_LOCAL_FIRST_ROUTE_IDS,
        LOCAL_PRIVATE_ONLY_ROUTE_IDS,
    )

    route = "DOTH-MBF-012"
    assert route in GOVERNED_LOCAL_FIRST_ROUTE_IDS
    ts = TS_CATALOGUE.read_text(encoding="utf-8")
    ts_block = re.search(r"GOVERNED_LOCAL_FIRST_ROUTE_IDS: ReadonlySet<string> = new Set\(\[(.*?)\]\);", ts, re.S)
    assert ts_block is not None and route in re.findall(ROUTE_ID_PATTERN, ts_block.group(1))
    # Like the CSV save, the route is in the local-private-only set of both catalogues (the native save itself
    # admits only local_private evidence with explicit intent, lossless and an exact payload, and stays authoritative).
    assert route in LOCAL_PRIVATE_ONLY_ROUTE_IDS
    ts_only = re.search(r"LOCAL_PRIVATE_ONLY_ROUTE_IDS: ReadonlySet<string> = new Set\(\[(.*?)\]\);", ts, re.S)
    assert ts_only is not None
    ts_constants = dict(re.findall(r'export const ([A-Z_]+_ROUTE_ID) = "([^"]+)";', ts))
    ts_set = {ts_constants.get(name.strip(), name.strip().strip('"')) for name in ts_only.group(1).split(",") if name.strip()}
    assert ts_set == set(LOCAL_PRIVATE_ONLY_ROUTE_IDS)
    rust = (ROOT / "apps/desktop/src-tauri/src/mbf_export.rs").read_text(encoding="utf-8")
    assert f'pub const MBF_EXPORT_ROUTE_ID: &str = "{route}";' in rust
    assert "validate_local_private_route_evidence(MBF_EXPORT_ROUTE_ID," in rust
    # The shared native check it calls admits only local_private, lossless, exact-payload evidence with intent.
    shared = (ROOT / "apps/desktop/src-tauri/src/model_document_save.rs").read_text(encoding="utf-8")
    for clause in ('s.export_context == "local_private"', "s.explicit_local_private_intent", "s.lossless_required",
                   "s.exact_payload_match", "!s.materialization_withheld", 'l.action == "include_metadata_only"'):
        assert clause in shared, clause
