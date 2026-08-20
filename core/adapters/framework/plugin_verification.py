"""Pure adapter/plugin contract verification for DEL-02-04.

The verifier consumes already-loaded mappings. It never loads a plugin,
dispatches an adapter, reads files, opens a network connection, or selects a
runtime or permission mechanism. Those implementation decisions remain TBD.
"""

from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass
import hashlib
import json
import math
import re
from typing import Any

from .adapter_framework import AdapterFinding, build_result, validate_adapter_declaration


REQUIRED_PLUGIN_PROVENANCE_FIELDS = (
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
)

REQUIRED_PLUGIN_NO_BYPASS_CONTROLS = (
    "unit_controls",
    "provenance_controls",
    "privacy_controls",
    "rule_sandbox_controls",
    "analysis_boundary_controls",
    "persistence_controls",
    "schema_validation",
    "diagnostic_envelope",
    "checksum_controls",
    "protected_content_controls",
    "report_boundary_controls",
    "solver_boundary_controls",
    "human_acceptance_controls",
)

NO_BYPASS_CODES = {
    "unit_controls": "PLUGIN_UNIT_SAFETY_CONTROL_DISABLED",
    "provenance_controls": "PLUGIN_PROVENANCE_CONTROL_DISABLED",
    "diagnostic_envelope": "PLUGIN_DIAGNOSTIC_ENVELOPE_CONTROL_DISABLED",
    "protected_content_controls": "PLUGIN_PROTECTED_CONTENT_CONTROL_DISABLED",
}

CANONICAL_PLUGIN_SCHEMA_SHA256 = (
    "99e126316bca0faf43da1833a211698618ce9e3432b25e4e27c17d438f756f83"
)
MAX_MANIFEST_JSON_DEPTH = 512


@dataclass(frozen=True)
class PluginManifestVerificationResult:
    """Structured result for the manifest-only verification layer."""

    outcome: str
    findings: tuple[AdapterFinding, ...]

    @property
    def verified(self) -> bool:
        return self.outcome == "VERIFIED"

    @property
    def quarantined(self) -> bool:
        return self.outcome == "QUARANTINE"


@dataclass(frozen=True)
class AdapterPluginVerificationResult:
    """Composed declaration and manifest result with an explicit runtime gate."""

    outcome: str
    declaration_accepted: bool
    manifest_verified: bool
    runtime_dispatched: bool
    findings: tuple[AdapterFinding, ...]
    result_envelope: Mapping[str, Any]

    @property
    def verification_passed(self) -> bool:
        return self.outcome == "BLOCKED_RUNTIME_NOT_SELECTED"

    @property
    def quarantined(self) -> bool:
        return self.outcome == "QUARANTINE"


def _raw_json_shape_error(
    value: Any,
    path: str,
) -> str | None:
    """Return the first non-exact JSON path without invoking caller overloads."""

    stack: list[tuple[str, Any, str, int]] = [("visit", value, path, 0)]
    active_containers: set[int] = set()
    current_path = path
    try:
        while stack:
            action, current, current_path, depth = stack.pop()
            if action == "exit":
                active_containers.remove(current)
                continue
            if depth > MAX_MANIFEST_JSON_DEPTH:
                return current_path

            value_type = type(current)
            if current is None or value_type in {bool, int, str}:
                continue
            if value_type is float:
                if not math.isfinite(current):
                    return current_path
                continue
            if value_type not in {dict, list}:
                return current_path

            identity = id(current)
            if identity in active_containers:
                return current_path
            active_containers.add(identity)
            stack.append(("exit", identity, current_path, depth))
            if value_type is list:
                for index in range(len(current) - 1, -1, -1):
                    stack.append(
                        (
                            "visit",
                            current[index],
                            f"{current_path}[{index}]",
                            depth + 1,
                        )
                    )
            else:
                items = list(dict.items(current))
                for key, _ in items:
                    if type(key) is not str:
                        return f"{current_path}.<key>"
                for key, item in reversed(items):
                    stack.append(
                        ("visit", item, f"{current_path}.{key}", depth + 1)
                    )
    except Exception:
        return current_path
    return None


def _plain_manifest_snapshot(
    manifest: Any,
) -> tuple[dict[str, Any] | None, AdapterFinding | None]:
    invalid_path = _raw_json_shape_error(manifest, "plugin_manifest")
    if invalid_path is not None or type(manifest) is not dict:
        provenance_invalid = invalid_path == "plugin_manifest.provenance" or (
            isinstance(invalid_path, str)
            and invalid_path.startswith("plugin_manifest.provenance.")
        )
        return None, _finding(
            "PLUGIN_PROVENANCE_INCOMPLETE"
            if provenance_invalid
            else "PLUGIN_MANIFEST_MALFORMED",
            "blocking",
            invalid_path or "plugin_manifest",
            "Plugin manifest contains a noncanonical raw JSON shape.",
            "Provide exact JSON objects, arrays, and primitive values without subclasses or cycles.",
        )
    try:
        manifest_bytes = json.dumps(
            manifest,
            sort_keys=True,
            separators=(",", ":"),
            ensure_ascii=False,
            allow_nan=False,
        ).encode("utf-8")
        snapshot = json.loads(manifest_bytes)
    except Exception:
        return None, _finding(
            "PLUGIN_MANIFEST_MALFORMED",
            "blocking",
            "plugin_manifest",
            "Plugin manifest cannot be normalized to exact JSON evidence.",
            "Provide an acyclic manifest composed only of canonical JSON primitives and containers.",
        )
    if type(snapshot) is not dict:
        return None, _finding(
            "PLUGIN_MANIFEST_MALFORMED",
            "blocking",
            "plugin_manifest",
            "Plugin manifest JSON snapshot is not an object.",
            "Provide a schema-shaped manifest JSON object.",
        )
    return snapshot, None


def verify_plugin_manifest(
    manifest: Any,
    plugin_schema: Any,
) -> PluginManifestVerificationResult:
    """Verify fail-closed controls on one already-loaded plugin manifest."""

    snapshot, finding = _plain_manifest_snapshot(manifest)
    if finding is not None or snapshot is None:
        assert finding is not None
        return PluginManifestVerificationResult("REJECTED", (finding,))
    return _verify_plugin_manifest_snapshot(snapshot, plugin_schema)


def _verify_plugin_manifest_snapshot(
    manifest: dict[str, Any],
    plugin_schema: Any,
) -> PluginManifestVerificationResult:
    findings: list[AdapterFinding] = []
    findings.extend(_verify_manifest_schema(manifest, plugin_schema))
    findings.extend(_verify_no_bypass_controls(manifest.get("no_bypass_constraints")))
    findings.extend(_verify_provenance(manifest.get("provenance")))
    findings.extend(
        _verify_diagnostic_compatibility(manifest.get("api_boundary_compatibility"))
    )
    findings.extend(_verify_permissions(manifest.get("permissions")))
    findings.extend(_verify_sandbox(manifest.get("sandbox")))
    findings.extend(_verify_privacy(manifest.get("privacy")))
    findings.extend(_verify_entrypoints(manifest.get("entrypoints")))
    findings.extend(_verify_manifest_quarantine_status(manifest.get("metadata")))

    severities = {finding.severity for finding in findings}
    if "quarantine" in severities:
        outcome = "QUARANTINE"
    elif "blocking" in severities:
        outcome = "REJECTED"
    else:
        outcome = "VERIFIED"
    return PluginManifestVerificationResult(outcome, tuple(findings))


def verify_adapter_plugin_contracts(
    adapter_declaration: Any,
    plugin_manifest: Any,
    *,
    plugin_schema: Any,
    unit_evidence: Any,
    unit_catalog: Any,
) -> AdapterPluginVerificationResult:
    """Compose existing adapter checks with manifest verification.

    A valid pair proves only that the declaration surfaces preserve the
    currently executable gates. Runtime dispatch remains blocked because the
    adapter execution model, plugin runtime, and permission persistence are
    unresolved owner-held decisions.
    """

    if isinstance(adapter_declaration, dict):
        try:
            adapter_result = validate_adapter_declaration(adapter_declaration)
        except Exception as error:
            adapter_findings = (
                _finding(
                    "ADAPTER_DECLARATION_MALFORMED",
                    "blocking",
                    "adapter_declaration",
                    f"Adapter declaration contains malformed nested values ({type(error).__name__}).",
                    "Provide the existing format-neutral adapter declaration with well-formed nested values.",
                ),
            )
            declaration_accepted = False
        else:
            adapter_findings = adapter_result.findings
            declaration_accepted = adapter_result.accepted
    else:
        adapter_findings = (
            _finding(
                "ADAPTER_DECLARATION_MALFORMED",
                "blocking",
                "adapter_declaration",
                "Adapter declaration payload must be an in-memory dictionary.",
                "Provide the existing format-neutral adapter declaration payload.",
            ),
        )
        declaration_accepted = False

    manifest_snapshot, manifest_finding = _plain_manifest_snapshot(plugin_manifest)
    if manifest_finding is not None or manifest_snapshot is None:
        assert manifest_finding is not None
        manifest_result = PluginManifestVerificationResult(
            "REJECTED", (manifest_finding,)
        )
        semantic_manifest: Mapping[str, Any] = {}
    else:
        manifest_result = _verify_plugin_manifest_snapshot(
            manifest_snapshot, plugin_schema
        )
        semantic_manifest = manifest_snapshot
    unit_findings, unit_contexts = _verify_unit_evidence(unit_evidence, unit_catalog)
    plugin_context = _plugin_diagnostic_context(semantic_manifest)
    envelope_privacy, envelope_provenance = _derive_envelope_boundaries(
        adapter_declaration,
        semantic_manifest,
        unit_evidence,
    )
    findings = adapter_findings + manifest_result.findings + tuple(unit_findings)
    diagnostic_contexts = (
        tuple(
            _context_for_finding(
                _adapter_diagnostic_context(adapter_declaration, finding.path),
                finding,
            )
            for finding in adapter_findings
        )
        + tuple(
            _context_for_finding(plugin_context, finding)
            for finding in manifest_result.findings
        )
        + tuple(unit_contexts)
    )
    if any(finding.severity == "quarantine" for finding in findings):
        return _composed_result(
            outcome="QUARANTINE",
            declaration_accepted=declaration_accepted,
            manifest_verified=manifest_result.verified,
            findings=findings,
            diagnostic_contexts=diagnostic_contexts,
            envelope_privacy=envelope_privacy,
            envelope_provenance=envelope_provenance,
        )
    if (
        not declaration_accepted
        or not manifest_result.verified
        or any(finding.severity == "blocking" for finding in findings)
    ):
        return _composed_result(
            outcome="REJECTED",
            declaration_accepted=declaration_accepted,
            manifest_verified=manifest_result.verified,
            findings=findings,
            diagnostic_contexts=diagnostic_contexts,
            envelope_privacy=envelope_privacy,
            envelope_provenance=envelope_provenance,
        )

    runtime_finding = _finding(
        "ADAPTER_PLUGIN_RUNTIME_NOT_SELECTED",
        "blocking",
        "runtime",
        "The adapter declaration and plugin manifest passed verification, but no plugin runtime or adapter execution model is selected.",
        "Keep runtime dispatch blocked until a separately governed runtime, capability, and permission design is accepted.",
    )
    return _composed_result(
        outcome="BLOCKED_RUNTIME_NOT_SELECTED",
        declaration_accepted=True,
        manifest_verified=True,
        findings=(runtime_finding,),
        diagnostic_contexts=(_context_for_finding(plugin_context, runtime_finding),),
        envelope_privacy=envelope_privacy,
        envelope_provenance=envelope_provenance,
    )


def _composed_result(
    *,
    outcome: str,
    declaration_accepted: bool,
    manifest_verified: bool,
    findings: tuple[AdapterFinding, ...],
    diagnostic_contexts: tuple[Mapping[str, Any], ...],
    envelope_privacy: Mapping[str, Any],
    envelope_provenance: Any,
) -> AdapterPluginVerificationResult:
    return AdapterPluginVerificationResult(
        outcome=outcome,
        declaration_accepted=declaration_accepted,
        manifest_verified=manifest_verified,
        runtime_dispatched=False,
        findings=findings,
        result_envelope=build_result(
            operation_id="ops.adapter.plugin.verify",
            operation_class="validate",
            diagnostics=findings,
            diagnostic_contexts=diagnostic_contexts,
            privacy_context=envelope_privacy,
            provenance=envelope_provenance,
        ),
    )


class _SchemaDefinitionError(ValueError):
    """Raised when the already-loaded schema is not internally usable."""


def _verify_manifest_schema(
    manifest: Mapping[str, Any],
    plugin_schema: Any,
) -> list[AdapterFinding]:
    if plugin_schema is None:
        return [
            _finding(
                "PLUGIN_MANIFEST_SCHEMA_MISSING",
                "blocking",
                "plugin_manifest_schema",
                "Canonical plugin-manifest schema evidence is missing.",
                "Provide the already-loaded canonical plugin schema mapping.",
            )
        ]
    if not isinstance(plugin_schema, Mapping):
        return [
            _finding(
                "PLUGIN_MANIFEST_SCHEMA_MALFORMED",
                "blocking",
                "plugin_manifest_schema",
                "Canonical plugin-manifest schema evidence is malformed.",
                "Provide the already-loaded canonical plugin schema mapping.",
            )
        ]
    try:
        schema_bytes = json.dumps(
            plugin_schema,
            sort_keys=True,
            separators=(",", ":"),
            ensure_ascii=False,
        ).encode("utf-8")
        schema_snapshot = json.loads(schema_bytes)
    except (TypeError, ValueError, json.JSONDecodeError):
        return [
            _finding(
                "PLUGIN_MANIFEST_SCHEMA_MALFORMED",
                "blocking",
                "plugin_manifest_schema",
                "Canonical plugin-manifest schema cannot be normalized to a plain JSON mapping.",
                "Provide the already-loaded canonical plugin schema mapping.",
            )
        ]
    expected_markers = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://openpipestress.org/schemas/plugin_manifest.schema.yaml",
        "type": "object",
    }
    if (
        not isinstance(schema_snapshot, dict)
        or any(schema_snapshot.get(key) != value for key, value in expected_markers.items())
        or not isinstance(schema_snapshot.get("properties"), dict)
        or not isinstance(schema_snapshot.get("$defs"), dict)
    ):
        return [
            _finding(
                "PLUGIN_MANIFEST_SCHEMA_MALFORMED",
                "blocking",
                "plugin_manifest_schema",
                "Schema snapshot does not expose the canonical plugin-manifest identity and definitions.",
                "Provide the already-loaded canonical plugin schema mapping.",
            )
        ]
    schema_fingerprint = hashlib.sha256(schema_bytes).hexdigest()
    if schema_fingerprint != CANONICAL_PLUGIN_SCHEMA_SHA256:
        return [
            _finding(
                "PLUGIN_MANIFEST_SCHEMA_NOT_CANONICAL",
                "blocking",
                "plugin_manifest_schema",
                "Schema mapping does not match the canonical plugin-manifest contract fingerprint.",
                "Provide the exact already-loaded canonical plugin schema without weakened or altered rules.",
            )
        ]
    try:
        mismatches = _schema_mismatches(
            manifest,
            schema_snapshot,
            schema_snapshot,
            "plugin_manifest",
        )
    except _SchemaDefinitionError as error:
        return [
            _finding(
                "PLUGIN_MANIFEST_SCHEMA_MALFORMED",
                "blocking",
                "plugin_manifest_schema",
                str(error),
                "Repair or reload the canonical plugin schema before verification.",
            )
        ]
    return [
        _finding(
            "PLUGIN_MANIFEST_SCHEMA_MISMATCH",
            "blocking",
            path,
            message,
            "Conform the manifest to the canonical plugin-manifest schema.",
        )
        for path, message in mismatches
    ]


def _schema_mismatches(
    instance: Any,
    schema: Mapping[str, Any],
    root_schema: Mapping[str, Any],
    path: str,
) -> list[tuple[str, str]]:
    if "$ref" in schema:
        reference = schema["$ref"]
        if not isinstance(reference, str) or not reference.startswith("#/"):
            raise _SchemaDefinitionError(f"Unsupported schema reference: {reference!r}.")
        target: Any = root_schema
        for raw_token in reference[2:].split("/"):
            token = raw_token.replace("~1", "/").replace("~0", "~")
            if not isinstance(target, Mapping) or token not in target:
                raise _SchemaDefinitionError(f"Unresolvable schema reference: {reference}.")
            target = target[token]
        if not isinstance(target, Mapping):
            raise _SchemaDefinitionError(f"Schema reference is not an object: {reference}.")
        return _schema_mismatches(instance, target, root_schema, path)

    expected_type = schema.get("type")
    if expected_type is not None:
        if not isinstance(expected_type, str):
            raise _SchemaDefinitionError("Schema type must be a string.")
        if not _matches_json_type(instance, expected_type):
            return [(path, f"Expected {expected_type}, got {_json_type_name(instance)}.")]

    mismatches: list[tuple[str, str]] = []
    if "const" in schema and not _json_equal(instance, schema["const"]):
        mismatches.append((path, f"Value must equal {schema['const']!r}."))
    if "enum" in schema:
        choices = schema["enum"]
        if not isinstance(choices, list):
            raise _SchemaDefinitionError("Schema enum must be an array.")
        if not any(_json_equal(instance, choice) for choice in choices):
            mismatches.append((path, f"Value {instance!r} is not in the allowed enum."))

    if isinstance(instance, str):
        minimum = schema.get("minLength")
        maximum = schema.get("maxLength")
        if minimum is not None and len(instance) < _schema_integer(minimum, "minLength"):
            mismatches.append((path, f"String length must be at least {minimum}."))
        if maximum is not None and len(instance) > _schema_integer(maximum, "maxLength"):
            mismatches.append((path, f"String length must be at most {maximum}."))
        if "pattern" in schema:
            pattern = schema["pattern"]
            if not isinstance(pattern, str):
                raise _SchemaDefinitionError("Schema pattern must be a string.")
            try:
                matches = re.search(pattern, instance)
            except re.error as error:
                raise _SchemaDefinitionError(f"Invalid schema pattern: {error}.") from error
            if matches is None:
                mismatches.append((path, f"String does not match pattern {pattern!r}."))

    if isinstance(instance, list):
        minimum = schema.get("minItems")
        maximum = schema.get("maxItems")
        if minimum is not None and len(instance) < _schema_integer(minimum, "minItems"):
            mismatches.append((path, f"Array must contain at least {minimum} item(s)."))
        if maximum is not None and len(instance) > _schema_integer(maximum, "maxItems"):
            mismatches.append((path, f"Array must contain at most {maximum} item(s)."))
        if schema.get("uniqueItems") is True:
            for index, item in enumerate(instance):
                if any(_json_equal(item, prior) for prior in instance[:index]):
                    mismatches.append((f"{path}[{index}]", "Array items must be unique."))
        item_schema = schema.get("items")
        if item_schema is not None:
            if not isinstance(item_schema, Mapping):
                raise _SchemaDefinitionError("Schema items must be an object.")
            for index, item in enumerate(instance):
                mismatches.extend(
                    _schema_mismatches(item, item_schema, root_schema, f"{path}[{index}]")
                )

    if isinstance(instance, Mapping):
        required = schema.get("required", [])
        if not isinstance(required, list) or any(
            not isinstance(item, str) for item in required
        ):
            raise _SchemaDefinitionError("Schema required must be an array of strings.")
        for key in required:
            if key not in instance:
                mismatches.append((f"{path}.{key}", "Required property is missing."))
        properties = schema.get("properties", {})
        if not isinstance(properties, Mapping):
            raise _SchemaDefinitionError("Schema properties must be an object.")
        for key, value in instance.items():
            if key in properties:
                property_schema = properties[key]
                if not isinstance(property_schema, Mapping):
                    raise _SchemaDefinitionError(
                        f"Schema property definition for {key!r} must be an object."
                    )
                mismatches.extend(
                    _schema_mismatches(
                        value,
                        property_schema,
                        root_schema,
                        f"{path}.{key}",
                    )
                )
            elif schema.get("additionalProperties") is False:
                mismatches.append((f"{path}.{key}", "Additional property is not allowed."))
    return mismatches


def _schema_integer(value: Any, keyword: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        raise _SchemaDefinitionError(f"Schema {keyword} must be a non-negative integer.")
    return value


def _matches_json_type(value: Any, expected: str) -> bool:
    checks = {
        "object": lambda item: isinstance(item, Mapping),
        "array": lambda item: isinstance(item, list),
        "string": lambda item: isinstance(item, str),
        "boolean": lambda item: isinstance(item, bool),
        "number": lambda item: not isinstance(item, bool) and isinstance(item, (int, float)),
        "integer": lambda item: not isinstance(item, bool) and isinstance(item, int),
        "null": lambda item: item is None,
    }
    if expected not in checks:
        raise _SchemaDefinitionError(f"Unsupported schema type: {expected!r}.")
    return checks[expected](value)


def _json_type_name(value: Any) -> str:
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "boolean"
    if isinstance(value, Mapping):
        return "object"
    if isinstance(value, list):
        return "array"
    if isinstance(value, str):
        return "string"
    if isinstance(value, (int, float)):
        return "number"
    return type(value).__name__


def _json_equal(left: Any, right: Any) -> bool:
    if _json_type_name(left) != _json_type_name(right):
        return False
    return left == right


def _verify_unit_evidence(
    unit_evidence: Any,
    unit_catalog: Any,
) -> tuple[list[AdapterFinding], list[Mapping[str, Any]]]:
    path = "unit_evidence"
    findings: list[AdapterFinding] = []
    contexts: list[Mapping[str, Any]] = []

    def add(finding: AdapterFinding, context: Mapping[str, Any]) -> None:
        findings.append(finding)
        contexts.append(context)

    catalog_dimensions = _validated_unit_catalog(unit_catalog)
    if catalog_dimensions is None:
        code = (
            "PLUGIN_UNIT_CATALOG_MISSING"
            if unit_catalog is None
            else "PLUGIN_UNIT_CATALOG_MALFORMED"
        )
        add(
            _finding(
                code,
                "blocking",
                "unit_catalog",
                "Caller-loaded unit catalog evidence is missing or malformed.",
                "Provide a non-empty in-memory mapping from accepted unit IDs or symbols to canonical dimensions.",
            ),
            _unit_diagnostic_context("unit_catalog", None),
        )

    if not isinstance(unit_evidence, list):
        add(
            _finding(
                "PLUGIN_UNIT_EVIDENCE_MALFORMED",
                "blocking",
                path,
                "Unit evidence must be an explicit in-memory list, including an empty list when no dimensional values are present.",
                "Provide one path, expected_dimension, and canonical quantity mapping per dimensional value.",
            ),
            _unit_diagnostic_context(path, None),
        )
        return findings, contexts

    canonical_dimensions = (
        frozenset(catalog_dimensions.values())
        if catalog_dimensions is not None
        else frozenset()
    )
    for index, evidence in enumerate(unit_evidence):
        evidence_path = f"{path}[{index}]"
        if not isinstance(evidence, Mapping):
            add(
                _finding(
                    "PLUGIN_UNIT_EVIDENCE_MALFORMED",
                    "blocking",
                    evidence_path,
                    "Unit evidence entry must be a mapping.",
                    "Provide path, expected_dimension, and quantity fields.",
                ),
                _unit_diagnostic_context(evidence_path, None),
            )
            continue
        declared_path = evidence.get("path")
        expected_dimension = evidence.get("expected_dimension")
        quantity = evidence.get("quantity")
        affected_path = (
            declared_path
            if isinstance(declared_path, str) and declared_path.strip()
            else evidence_path
        )
        provenance = quantity.get("provenance") if isinstance(quantity, Mapping) else None
        context = _unit_diagnostic_context(affected_path, provenance)
        provenance_requires_quarantine = isinstance(
            provenance, Mapping
        ) and _provenance_requires_quarantine(provenance)
        if provenance_requires_quarantine:
            add(
                _finding(
                    "PLUGIN_QUANTITY_PROTECTED_CONTENT_SUSPECTED",
                    "quarantine",
                    affected_path,
                    "Quantity provenance indicates suspected protected or quarantined content.",
                    "Quarantine the quantity and payload for human/legal review.",
                ),
                context,
            )
        if not isinstance(declared_path, str) or not declared_path.strip():
            add(
                _finding(
                    "PLUGIN_UNIT_PATH_MISSING",
                    "blocking",
                    f"{evidence_path}.path",
                    "Caller-declared quantity path is missing or malformed.",
                    "Declare the affected quantity path explicitly.",
                ),
                context,
            )
        if not isinstance(expected_dimension, str) or not expected_dimension.strip():
            add(
                _finding(
                    "PLUGIN_UNIT_EXPECTED_DIMENSION_MISSING",
                    "blocking",
                    f"{evidence_path}.expected_dimension",
                    "Caller-declared expected dimension is missing or malformed.",
                    "Declare the expected canonical dimension explicitly.",
                ),
                context,
            )
        elif expected_dimension not in canonical_dimensions:
            add(
                _finding(
                    "PLUGIN_UNIT_EXPECTED_DIMENSION_NONCANONICAL",
                    "blocking",
                    affected_path,
                    f"Expected dimension {expected_dimension!r} is not canonical.",
                    "Use an accepted canonical dimension identifier.",
                ),
                context,
            )
        if not isinstance(quantity, Mapping):
            add(
                _finding(
                    "PLUGIN_QUANTITY_MALFORMED",
                    "blocking",
                    f"{evidence_path}.quantity",
                    "Quantity evidence must be a mapping.",
                    "Provide value, unit, dimension, and provenance explicitly.",
                ),
                context,
            )
            continue
        missing = [key for key in ("value", "unit", "dimension", "provenance") if key not in quantity]
        if missing:
            add(
                _finding(
                    "PLUGIN_QUANTITY_METADATA_MISSING",
                    "blocking",
                    f"{evidence_path}.quantity",
                    f"Quantity fields are missing or empty: {', '.join(missing)}.",
                    "Provide value, unit, dimension, and provenance explicitly.",
                ),
                context,
            )
            continue
        value = quantity.get("value")
        unit = quantity.get("unit")
        dimension = quantity.get("dimension")
        if isinstance(value, bool) or not isinstance(value, (int, float)):
            add(
                _finding(
                    "PLUGIN_QUANTITY_VALUE_MALFORMED",
                    "blocking",
                    affected_path,
                    "Quantity value must be numeric.",
                    "Provide a finite JSON number for the quantity value.",
                ),
                context,
            )
        elif not math.isfinite(value):
            add(
                _finding(
                    "PLUGIN_QUANTITY_VALUE_NONFINITE",
                    "blocking",
                    affected_path,
                    "Quantity value must be finite.",
                    "Replace NaN or infinity with a finite numeric value or reject the payload.",
                ),
                context,
            )
        if (
            not isinstance(unit, str)
            or not quantity["unit"].strip()
            or not isinstance(dimension, str)
            or not quantity["dimension"].strip()
        ):
            add(
                _finding(
                    "PLUGIN_QUANTITY_METADATA_MALFORMED",
                    "blocking",
                    f"{evidence_path}.quantity",
                    "Quantity unit, dimension, or provenance is malformed.",
                    "Provide a non-empty unit, canonical dimension, and provenance mapping.",
                ),
                context,
            )
            continue
        if dimension not in canonical_dimensions:
            add(
                _finding(
                    "PLUGIN_QUANTITY_DIMENSION_NONCANONICAL",
                    "blocking",
                    affected_path,
                    f"Quantity dimension {dimension!r} is not canonical.",
                    "Use an accepted canonical dimension identifier.",
                ),
                context,
            )
        quantity_provenance = quantity.get("provenance")
        if provenance_requires_quarantine:
            pass
        elif not _canonical_provenance(quantity_provenance):
            add(
                _finding(
                    "PLUGIN_QUANTITY_PROVENANCE_MALFORMED",
                    "blocking",
                    affected_path,
                    "Quantity provenance is incomplete or malformed.",
                    "Provide complete canonical source, license, contributor, redistribution, and review metadata.",
                ),
                context,
            )
        elif not _provenance_is_cleared(quantity_provenance):
            add(
                _finding(
                    "PLUGIN_QUANTITY_PROVENANCE_NOT_CLEARED",
                    "blocking",
                    affected_path,
                    "Quantity provenance is canonical but not cleared for verified use.",
                    "Resolve redistribution and review status before plugin verification.",
                ),
                context,
            )
        if catalog_dimensions is not None:
            catalog_dimension = catalog_dimensions.get(unit)
            if catalog_dimension is None:
                add(
                    _finding(
                        "PLUGIN_QUANTITY_UNIT_UNKNOWN",
                        "blocking",
                        affected_path,
                        f"Quantity unit {unit!r} is absent from caller-loaded catalog evidence.",
                        "Use an accepted catalog unit or reject the payload.",
                    ),
                    context,
                )
            elif catalog_dimension != dimension:
                add(
                    _finding(
                        "PLUGIN_QUANTITY_UNIT_DIMENSION_MISMATCH",
                        "blocking",
                        affected_path,
                        f"Catalog unit {unit!r} maps to {catalog_dimension!r}, not quantity dimension {dimension!r}.",
                        "Use a catalog unit compatible with the quantity's canonical dimension.",
                    ),
                    context,
                )
        if (
            isinstance(expected_dimension, str)
            and expected_dimension in canonical_dimensions
            and dimension in canonical_dimensions
            and dimension != expected_dimension
        ):
            add(
                _finding(
                    "PLUGIN_QUANTITY_DIMENSION_MISMATCH",
                    "blocking",
                    str(declared_path) if isinstance(declared_path, str) else evidence_path,
                    f"Quantity dimension {quantity.get('dimension')!r} does not match expected dimension {expected_dimension!r}.",
                    "Provide a quantity with the caller-declared canonical dimension; no conversion or inference is performed.",
                ),
                context,
            )
    return findings, contexts


def _validated_unit_catalog(unit_catalog: Any) -> dict[str, str] | None:
    if not isinstance(unit_catalog, Mapping) or not unit_catalog:
        return None
    if any(
        not isinstance(unit, str)
        or not unit.strip()
        or not isinstance(dimension, str)
        or not dimension.strip()
        or dimension == "TBD"
        for unit, dimension in unit_catalog.items()
    ):
        return None
    return dict(unit_catalog)


def _provenance_field(provenance: Mapping[str, Any], field: str) -> Any:
    """Read caller provenance without allowing a hostile accessor to escape."""

    try:
        return provenance.get(field)
    except Exception:
        return None


def _canonical_provenance_values(provenance: Any) -> dict[str, str] | None:
    if not isinstance(provenance, Mapping):
        return None
    values = {
        field: _provenance_field(provenance, field)
        for field in REQUIRED_PLUGIN_PROVENANCE_FIELDS
    }
    if any(
        type(value) is not str or not value.strip()
        for value in values.values()
    ):
        return None
    return values


def _canonical_provenance(provenance: Any) -> bool:
    values = _canonical_provenance_values(provenance)
    if values is None:
        return False
    return (
        values["redistribution_status"]
        in {"public_permissive", "private_only", "unknown", "protected_suspected", "TBD"}
        and values["review_status"]
        in {"accepted", "needs_review", "quarantined", "rejected", "TBD"}
    )


def _provenance_requires_quarantine(provenance: Mapping[str, Any]) -> bool:
    redistribution = _provenance_field(provenance, "redistribution_status")
    review = _provenance_field(provenance, "review_status")
    return (
        type(redistribution) is str and redistribution == "protected_suspected"
    ) or (
        type(review) is str and review == "quarantined"
    )


def _provenance_is_cleared(provenance: Mapping[str, Any]) -> bool:
    values = _canonical_provenance_values(provenance)
    if values is None:
        return False
    return (
        values["redistribution_status"] in {"public_permissive", "private_only"}
        and values["review_status"] == "accepted"
    )


def _derive_envelope_boundaries(
    adapter_payload: Any,
    plugin_manifest: Any,
    unit_evidence: Any,
) -> tuple[Mapping[str, Any], Any]:
    """Select the most restrictive caller provenance/privacy without invention."""

    candidates: list[tuple[Any, Any]] = []
    manifest = plugin_manifest if isinstance(plugin_manifest, Mapping) else {}
    candidates.append(
        (
            manifest.get("provenance"),
            _plugin_privacy_boundary(manifest.get("privacy")),
        )
    )

    if isinstance(unit_evidence, list):
        for evidence in unit_evidence:
            quantity = evidence.get("quantity") if isinstance(evidence, Mapping) else None
            if isinstance(quantity, Mapping):
                candidates.append((quantity.get("provenance"), None))
            else:
                candidates.append((None, None))
    else:
        candidates.append((None, None))

    adapter = adapter_payload if isinstance(adapter_payload, Mapping) else {}
    operation_result = adapter.get("operation_result")
    if isinstance(operation_result, Mapping):
        candidates.append(
            (
                operation_result.get("provenance"),
                _adapter_privacy_boundary(operation_result.get("privacy")),
            )
        )
    else:
        candidates.append((None, None))
    declaration = adapter.get("adapter_declaration")
    if isinstance(declaration, Mapping):
        candidates.append(
            (
                declaration.get("provenance"),
                _adapter_privacy_boundary(declaration.get("privacy")),
            )
        )
    else:
        candidates.append((None, None))

    selected_provenance: Any = None
    selected_privacy: Any = None
    selected_score = (0, 0)
    for provenance, privacy in candidates:
        provenance_rank = _provenance_boundary_rank(provenance)
        rank = max(provenance_rank, _privacy_boundary_rank(privacy))
        score = (rank, provenance_rank)
        if score > selected_score:
            selected_provenance = provenance
            selected_privacy = privacy
            selected_score = score

    classification = {
        4: "protected_suspected",
        3: "private_local_only",
        2: "export_review_required",
        1: "public_permissive_reviewed",
    }.get(selected_score[0], "TBD")
    privacy_context = {
        "classification": classification,
        "local_first": True,
        "telemetry_allowed": False,
        "export_review_required": True,
        "private_payload_redacted": (
            isinstance(selected_privacy, Mapping)
            and selected_privacy.get("private_payload_redacted") is True
        ),
    }
    return privacy_context, selected_provenance


def _provenance_boundary_rank(provenance: Any) -> int:
    if not isinstance(provenance, Mapping):
        return 2
    if _provenance_requires_quarantine(provenance):
        return 4
    redistribution = _provenance_field(provenance, "redistribution_status")
    if type(redistribution) is str and redistribution == "private_only":
        return 3
    if not _canonical_provenance(provenance):
        return 2
    return 1 if _provenance_is_cleared(provenance) else 2


def _privacy_boundary_rank(privacy: Any) -> int:
    if not isinstance(privacy, Mapping):
        return 0
    return {
        "protected_suspected": 4,
        "private_local_only": 3,
        "export_review_required": 2,
        "TBD": 2,
        "public_permissive_reviewed": 1,
    }.get(privacy.get("classification"), 0)


def _plugin_privacy_boundary(privacy: Any) -> Mapping[str, Any]:
    expected = {
        "local_first": True,
        "private_data_transmission_default": False,
        "telemetry_enabled_by_default": False,
        "export_requires_permission": True,
        "redaction_supported": True,
    }
    if not isinstance(privacy, Mapping) or any(
        privacy.get(key) != value for key, value in expected.items()
    ) or privacy.get("private_data_access") not in {
        "none",
        "explicit_permission_required",
    }:
        return {"classification": "TBD"}
    return privacy


def _adapter_privacy_boundary(privacy: Any) -> Mapping[str, Any]:
    accepted_classifications = {
        "public_permissive_reviewed",
        "private_local_only",
        "protected_suspected",
        "export_review_required",
        "TBD",
    }
    if isinstance(privacy, Mapping) and privacy.get("classification") in {
        "private_local_only",
        "protected_suspected",
    }:
        return privacy
    if (
        not isinstance(privacy, Mapping)
        or privacy.get("local_first") is not True
        or privacy.get("telemetry_allowed") is not False
        or privacy.get("classification") not in accepted_classifications
        or not isinstance(privacy.get("export_review_required"), bool)
        or not isinstance(privacy.get("private_payload_redacted"), bool)
    ):
        return {"classification": "TBD"}
    return privacy


def _adapter_diagnostic_context(
    adapter_declaration: Any,
    finding_path: str,
) -> Mapping[str, Any]:
    root = adapter_declaration if isinstance(adapter_declaration, Mapping) else {}
    declaration = (
        root.get("adapter_declaration")
        if isinstance(root, Mapping)
        else None
    )
    declaration = declaration if isinstance(declaration, Mapping) else {}
    operation_result = root.get("operation_result")
    operation_result = (
        operation_result if isinstance(operation_result, Mapping) else {}
    )
    if finding_path == "operation_result" or finding_path.startswith(
        "operation_result."
    ):
        provenance = operation_result.get("provenance")
    elif finding_path == "adapter_declaration" or finding_path.startswith(
        "adapter_declaration."
    ):
        provenance = declaration.get("provenance")
    else:
        provenance = None
    adapter_id = declaration.get("adapter_id")
    return {
        "source": {
            "ref_type": "adapter",
            "ref_id": adapter_id if isinstance(adapter_id, str) and adapter_id.strip() else "TBD",
        },
        "affected_object": {"ref_type": "diagnostic", "ref_id": "adapter_declaration"},
        "provenance": provenance,
    }


def _plugin_diagnostic_context(plugin_manifest: Any) -> Mapping[str, Any]:
    manifest = plugin_manifest if isinstance(plugin_manifest, Mapping) else {}
    metadata = manifest.get("metadata")
    metadata = metadata if isinstance(metadata, Mapping) else {}
    plugin_id = metadata.get("plugin_id")
    return {
        "source": {
            "ref_type": "payload",
            "ref_id": plugin_id if isinstance(plugin_id, str) and plugin_id.strip() else "TBD",
        },
        "affected_object": {"ref_type": "diagnostic", "ref_id": "plugin_manifest"},
        "provenance": manifest.get("provenance"),
    }


def _unit_diagnostic_context(path: str, provenance: Any) -> Mapping[str, Any]:
    return {
        "source": {"ref_type": "payload", "ref_id": path},
        "affected_object": {"ref_type": "diagnostic", "ref_id": path},
        "provenance": provenance,
    }


def _context_for_finding(
    context: Mapping[str, Any],
    finding: AdapterFinding,
) -> Mapping[str, Any]:
    return {
        **context,
        "affected_object": {"ref_type": "diagnostic", "ref_id": finding.path},
    }


def _verify_no_bypass_controls(controls: Any) -> list[AdapterFinding]:
    if not isinstance(controls, Mapping):
        return [
            _finding(
                "PLUGIN_NO_BYPASS_CONTROLS_MALFORMED",
                "blocking",
                "plugin_manifest.no_bypass_constraints",
                "Plugin no-bypass controls are missing or malformed.",
                "Provide every required no-bypass control explicitly enabled.",
            )
        ]

    findings: list[AdapterFinding] = []
    for control in REQUIRED_PLUGIN_NO_BYPASS_CONTROLS:
        if controls.get(control) is not True:
            findings.append(
                _finding(
                    NO_BYPASS_CODES.get(
                        control, "PLUGIN_NO_BYPASS_CONTROL_DISABLED"
                    ),
                    "blocking",
                    f"plugin_manifest.no_bypass_constraints.{control}",
                    f"{control} must be explicitly enabled.",
                    "Restore the required fail-closed plugin control.",
                )
            )
    return findings


def _verify_provenance(provenance: Any) -> list[AdapterFinding]:
    path = "plugin_manifest.provenance"
    if not isinstance(provenance, Mapping):
        return [
            _finding(
                "PLUGIN_PROVENANCE_MALFORMED",
                "blocking",
                path,
                "Plugin provenance is missing or malformed.",
                "Provide complete source, license, contributor, redistribution, and review metadata.",
            )
        ]

    redistribution = _provenance_field(provenance, "redistribution_status")
    review = _provenance_field(provenance, "review_status")
    protected_marker = (
        type(redistribution) is str and redistribution == "protected_suspected"
    )
    quarantined_marker = type(review) is str and review == "quarantined"
    if protected_marker or quarantined_marker:
        return [
            _finding(
                "PLUGIN_PROTECTED_CONTENT_SUSPECTED",
                "quarantine",
                path,
                "Plugin provenance indicates suspected protected content.",
                "Quarantine the manifest and payload for human/legal review.",
            )
        ]

    canonical = _canonical_provenance_values(provenance)
    if canonical is None:
        return [
            _finding(
                "PLUGIN_PROVENANCE_INCOMPLETE",
                "blocking",
                path,
                "Required provenance fields are missing or malformed.",
                "Complete provenance before plugin verification.",
            )
        ]

    redistribution = canonical["redistribution_status"]
    review = canonical["review_status"]
    if redistribution not in {"public_permissive", "private_only"} or review != "accepted":
        return [
            _finding(
                "PLUGIN_PROVENANCE_NOT_CLEARED",
                "blocking",
                path,
                "Plugin provenance is not cleared for verified use.",
                "Resolve redistribution and review status without importing protected content.",
            )
        ]
    return []


def _verify_diagnostic_compatibility(compatibility: Any) -> list[AdapterFinding]:
    path = "plugin_manifest.api_boundary_compatibility"
    if not isinstance(compatibility, Mapping):
        return [
            _finding(
                "PLUGIN_DIAGNOSTIC_COMPATIBILITY_MALFORMED",
                "blocking",
                path,
                "API-boundary compatibility is missing or malformed.",
                "Declare result-envelope and diagnostic compatibility.",
            )
        ]

    findings: list[AdapterFinding] = []
    if compatibility.get("result_envelope_required") is not True:
        findings.append(
            _finding(
                "PLUGIN_RESULT_ENVELOPE_NOT_REQUIRED",
                "blocking",
                f"{path}.result_envelope_required",
                "Plugin results must require the project result envelope.",
                "Set result_envelope_required to true.",
            )
        )
    categories = compatibility.get("compatible_operation_categories")
    if (
        not isinstance(categories, list)
        or any(not isinstance(category, str) for category in categories)
        or not {"diagnostic", "result_envelope"}.issubset(categories)
    ):
        findings.append(
            _finding(
                "PLUGIN_DIAGNOSTIC_ENVELOPE_INCOMPATIBLE",
                "blocking",
                f"{path}.compatible_operation_categories",
                "Plugin compatibility must include diagnostic and result-envelope categories.",
                "Declare both diagnostic and result_envelope compatibility categories.",
            )
        )
    if compatibility.get("transport") != "TBD":
        findings.append(
            _finding(
                "PLUGIN_TRANSPORT_DECISION_NOT_TBD",
                "blocking",
                f"{path}.transport",
                "Plugin transport must remain TBD in this verification seam.",
                "Do not select a transport until separately governed authority resolves it.",
            )
        )
    return findings


def _verify_permissions(permissions: Any) -> list[AdapterFinding]:
    path = "plugin_manifest.permissions"
    if not isinstance(permissions, Mapping):
        return [
            _finding(
                "PLUGIN_PERMISSIONS_MALFORMED",
                "blocking",
                path,
                "Plugin permission controls are missing or malformed.",
                "Provide explicit deny-by-default permission controls.",
            )
        ]
    expected = {
        "denied_by_default": True,
        "grant_state": "not_granted",
        "user_consent_required": True,
        "revocation_supported": True,
    }
    return [
        _finding(
            "PLUGIN_PERMISSION_CONTROL_DISABLED",
            "blocking",
            f"{path}.{key}",
            f"{key} must remain {value!r} before a permission mechanism is selected.",
            "Restore the deny-by-default unresolved permission posture.",
        )
        for key, value in expected.items()
        if permissions.get(key) != value
    ]


def _verify_sandbox(sandbox: Any) -> list[AdapterFinding]:
    path = "plugin_manifest.sandbox"
    if not isinstance(sandbox, Mapping):
        return [
            _finding(
                "PLUGIN_SANDBOX_CONTROLS_MALFORMED",
                "blocking",
                path,
                "Plugin sandbox controls are missing or malformed.",
                "Provide explicit fail-closed sandbox declarations.",
            )
        ]
    expected = {
        "sandbox_required": True,
        "arbitrary_code_execution_allowed": False,
        "filesystem_access_default": "denied",
        "network_access_default": "denied",
        "process_spawn_default": "denied",
        "capability_declaration_required": True,
    }
    return [
        _finding(
            "PLUGIN_SANDBOX_CONTROL_DISABLED",
            "blocking",
            f"{path}.{key}",
            f"{key} must remain {value!r} in the verification seam.",
            "Restore the fail-closed sandbox declaration without selecting a runtime.",
        )
        for key, value in expected.items()
        if sandbox.get(key) != value
    ]


def _verify_privacy(privacy: Any) -> list[AdapterFinding]:
    path = "plugin_manifest.privacy"
    if not isinstance(privacy, Mapping):
        return [
            _finding(
                "PLUGIN_PRIVACY_CONTROLS_MALFORMED",
                "blocking",
                path,
                "Plugin privacy controls are missing or malformed.",
                "Provide explicit local-first and no-transmission defaults.",
            )
        ]
    expected = {
        "local_first": True,
        "private_data_transmission_default": False,
        "telemetry_enabled_by_default": False,
        "export_requires_permission": True,
        "redaction_supported": True,
    }
    return [
        _finding(
            "PLUGIN_PRIVACY_CONTROL_DISABLED",
            "blocking",
            f"{path}.{key}",
            f"{key} must remain {value!r}.",
            "Restore the fail-closed plugin privacy posture.",
        )
        for key, value in expected.items()
        if privacy.get(key) != value
    ]


def _verify_entrypoints(entrypoints: Any) -> list[AdapterFinding]:
    path = "plugin_manifest.entrypoints"
    if not isinstance(entrypoints, list) or not entrypoints:
        return [
            _finding(
                "PLUGIN_ENTRYPOINTS_MALFORMED",
                "blocking",
                path,
                "Plugin entrypoints must be a non-empty list.",
                "Provide declared metadata-only or governed-boundary entrypoints.",
            )
        ]
    findings: list[AdapterFinding] = []
    for index, entrypoint in enumerate(entrypoints):
        if not isinstance(entrypoint, Mapping):
            findings.append(
                _finding(
                    "PLUGIN_ENTRYPOINT_MALFORMED",
                    "blocking",
                    f"{path}[{index}]",
                    "Plugin entrypoint must be a mapping.",
                    "Provide a schema-shaped entrypoint declaration.",
                )
            )
        elif entrypoint.get("governed_boundary_required") is not True:
            findings.append(
                _finding(
                    "PLUGIN_GOVERNED_BOUNDARY_DISABLED",
                    "blocking",
                    f"{path}[{index}].governed_boundary_required",
                    "Every plugin entrypoint must require the governed boundary.",
                    "Set governed_boundary_required to true.",
                )
            )
    return findings


def _verify_manifest_quarantine_status(metadata: Any) -> list[AdapterFinding]:
    if not isinstance(metadata, Mapping):
        return [
            _finding(
                "PLUGIN_METADATA_MALFORMED",
                "blocking",
                "plugin_manifest.metadata",
                "Plugin metadata is missing or malformed.",
                "Provide schema-shaped plugin metadata.",
            )
        ]
    if metadata.get("status") == "quarantined":
        return [
            _finding(
                "PLUGIN_MANIFEST_QUARANTINED",
                "quarantine",
                "plugin_manifest.metadata.status",
                "Plugin manifest is already marked quarantined.",
                "Keep it quarantined pending human review.",
            )
        ]
    return []


def _finding(
    code: str,
    severity: str,
    path: str,
    message: str,
    remediation: str,
) -> AdapterFinding:
    return AdapterFinding(code, severity, path, message, remediation)
