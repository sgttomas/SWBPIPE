"""The analysis-run record builder and validator (DEL-14-02): 0.3.0 only.

One current record version is kept (b7-plan E1). A saved 0.1.0 or 0.2.0 record is
refused by name (`ANALYSIS_RUN_SCHEMA_VERSION_RETIRED`; re-run the analysis), and a
result that no current record contract reads is refused for recording. Every reading of a result
statement here is the Rust product reader's verdict (b7-plan R1, R2), taken through
`rust_reader` (`openpipestress-runner read`): the header dispatch and each route's reader
(`_source_contract`), standing, classification summary and rule binding. Nothing here
re-derives a verdict; the record builder projects the table the reader bound.
The Python reading these functions held before R2 is now a test verifier only.
"""
from __future__ import annotations

from copy import deepcopy
from pathlib import Path
import json
import re
from typing import Any, Callable, Mapping, Sequence

from core.serialization.canonical_json.adapter import canonical_json_checked_v1, canonical_sha256_checked_v1
from . import rust_reader

SCHEMA_VERSION = "0.3.0"
# Analysis-run versions that are no longer read or written; refused by name.
RETIRED_SCHEMA_VERSIONS = frozenset({"0.1.0", "0.2.0"})
ANALYSIS_RUN_SCHEMA_VERSION_RETIRED = "ANALYSIS_RUN_SCHEMA_VERSION_RETIRED"
PROFILE = "openpipestress_jcs_ijson_v1"
_RESULTS_DIR = Path(__file__).resolve().parents[2] / "fixtures" / "results"


def _semantic_rows(path: Path) -> list[dict[str, Any]]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    return payload["rows"]


def _semantic(row: Mapping[str, Any], path: Path) -> tuple[dict[str, Any] | None, list[str]]:
    kind = str(row.get("kind", ""))
    unit = str(row.get("unit", ""))
    candidates = [entry for entry in _semantic_rows(path) if entry["kind"] == kind]
    if not candidates:
        return None, ["SOURCE_SIGNATURE_UNKNOWN"]
    units = [entry for entry in candidates if entry["unit"] == unit]
    if not units:
        raise ValueError(f"SOURCE_UNIT_CONTRADICTION: {kind} / {unit}")
    metadata = row.get("metadata") if isinstance(row.get("metadata"), Mapping) else {}
    component = metadata.get("component") if isinstance(metadata, Mapping) else None
    variants = [entry for entry in units if entry.get("component") == component]
    exact = [entry for entry in variants if "source_basis" not in entry or entry["source_basis"] == metadata.get("basis")]
    if variants and not exact:
        raise ValueError(f"SOURCE_BASIS_CONTRADICTION: {kind}")
    if exact:
        legacy_dimension = exact[0].get("legacy_declared_dimension")
        if row.get("dimension") and legacy_dimension and row.get("dimension") != legacy_dimension:
            raise ValueError(f"ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: {row.get('id')}")
        return exact[0], []
    generic_variants = [entry for entry in units if entry.get("component") is None]
    generic = [entry for entry in generic_variants if "source_basis" not in entry or entry["source_basis"] == metadata.get("basis")]
    if generic_variants and not generic:
        raise ValueError(f"SOURCE_BASIS_CONTRADICTION: {kind}")
    if generic:
        legacy_dimension = generic[0].get("legacy_declared_dimension")
        if row.get("dimension") and legacy_dimension and row.get("dimension") != legacy_dimension:
            raise ValueError(f"ANALYSIS-RUN-RESULT-DIMENSION-MISMATCH: {row.get('id')}")
        return generic[0], ["OPTIONAL_SOURCE_METADATA_MISSING"] if component is None else []
    if component is not None:
        raise ValueError(f"SOURCE_COMPONENT_CONTRADICTION: {kind} / {component}")
    return None, ["SOURCE_COMPONENT_MISSING_SEMANTICS_UNAVAILABLE"]


def _checksum(scope: str, ref: dict[str, str], value: Any, hash_fn: Callable[[Any], str]) -> dict[str, Any]:
    return {"algorithm": "sha256", "canonicalization": PROFILE, "payload_ref": ref, "payload_scope": scope, "value": hash_fn(value)}


def analysis_record_projection(envelope: Mapping[str, Any]) -> dict[str, Any]:
    projected = deepcopy(dict(envelope))
    hashes = projected["analysis_run"]["hashes"]
    matches = [row for row in hashes if row.get("payload_scope") == "analysis_run_record"]
    if len(matches) > 1:
        raise ValueError("ANALYSIS-RUN-RECORD-CHECKSUM-DUPLICATE")
    projected["analysis_run"]["hashes"] = [row for row in hashes if row.get("payload_scope") != "analysis_run_record"]
    return projected


# Rule-pack reference diagnostics (FR-CMP-002; DEL-14-01 Design
# `desktop-states-runs-comparison.md` section 3). The desktop solve resolves the model's
# `rule_pack_refs` against the local rule-pack store; a reference left out of the input
# manifest's `active_rule_packs` is recorded as a diagnostic of the run record, after the
# producer's, under the record hash. Mirror of `rulePackRefDiagnostics.ts`: identical bytes.
RULE_PACK_REF_UNRESOLVED = "MODEL-RULE-PACK-REF-UNRESOLVED"
RULE_PACK_REF_CHECKSUM_MISMATCH = "MODEL-RULE-PACK-REF-CHECKSUM-MISMATCH"
RULE_PACK_REF_DIAGNOSTIC_SOURCE = "desktop/solve-rule-pack-resolution"
ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH = "ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH"
# A run recorded before rule-pack resolution (S2, #1596): the model has references, the
# manifest's active_rule_packs is empty and the record names no exclusion. Refused by name, not
# read leniently, so the user knows to re-record it (as Rust and the TypeScript refuse it).
ANALYSIS_RUN_RULE_PACKS_NOT_RESOLVED = "ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED"
_RULE_PACKS_NOT_RESOLVED_REFUSAL = f"{ANALYSIS_RUN_RULE_PACKS_NOT_RESOLVED}: recorded before rule-pack resolution; re-record this run"


def _rule_pack_diagnostics_refusal(identities: list[str], input_manifest: Mapping[str, Any]) -> str | None:
    """The refusal for record rule-pack diagnostics naming `identities` against the manifest's
    exclusions, or None when they agree (`rulePackDiagnosticsRefusal` in the TypeScript)."""
    excluded = manifest_excluded_rule_pack_identities(input_manifest)
    if identities == excluded:
        return None
    none_resolved = input_manifest.get("active_rule_packs") == []
    return _RULE_PACKS_NOT_RESOLVED_REFUSAL if not identities and excluded and none_resolved else ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH
_RULE_PACK_REF_CODES = (RULE_PACK_REF_UNRESOLVED, RULE_PACK_REF_CHECKSUM_MISMATCH)
_MESSAGE_IDENTITY_LIMIT = 160


def _identity_component(component: str) -> str:
    return component.replace("%", "%25").replace("@", "%40")


def rule_pack_identity(rule_pack_id: str, version: str) -> str:
    """`<rule_pack_id>@<version>`, as the recorded revision's `rule_pack_refs` and the rule-pack
    reference diagnostics name a pack: each component with `%` (as `%25`) and then `@` (as `%40`)
    percent-encoded, so distinct (id, version) pairs never share a string. An id and version
    without `%` or `@` are unchanged. Mirrors `rulePackIdentity` (TS) and Rust's `rule_pack_identity`."""
    return f"{_identity_component(rule_pack_id)}@{_identity_component(version)}"


def parse_rule_pack_identity(identity: str) -> tuple[str, str] | None:
    """The (id, version) pair an identity names, or None when the text is not an identity."""
    parts = identity.split("@")
    if len(parts) != 2 or any(re.search(r"%(?!25|40)", part) for part in parts):
        return None
    decoded = [re.sub(r"%40|%25", lambda match: "@" if match.group() == "%40" else "%", part) for part in parts]
    if rule_pack_identity(decoded[0], decoded[1]) != identity:
        return None
    return decoded[0], decoded[1]


def rule_pack_ref_diagnostic(code: str, identity: str) -> dict[str, Any]:
    """The diagnostic for an excluded reference, determined by its code and identity alone
    (the producer's diagnostic shape). The message shows at most 160 code points of the identity."""
    shown = identity if len(identity) <= _MESSAGE_IDENTITY_LIMIT else identity[:_MESSAGE_IDENTITY_LIMIT] + "\u2026"
    reason = ("the stored pack's checksum was not the reference's at solve time" if code == RULE_PACK_REF_CHECKSUM_MISMATCH
              else "the local rule-pack store held no such pack for this project at solve time")
    return {
        "id": f"diagnostic:rule-pack-ref:{identity}",
        "code": code,
        "severity": "warning",
        "message": f"Rule pack {shown} is referenced by the model, but {reason}; it is not among this run's rule-pack references.",
        "source": RULE_PACK_REF_DIAGNOSTIC_SOURCE,
        "affected_refs": [identity],
    }


def rule_pack_ref_diagnostic_entries(diagnostics: Sequence[Any]) -> list[tuple[str, str]] | None:
    """The (code, identity) pairs of a record's appended rule-pack reference diagnostics, or
    None when any is not exactly such a diagnostic or an identity repeats."""
    entries: list[tuple[str, str]] = []
    seen: set[str] = set()
    for item in diagnostics:
        if not isinstance(item, Mapping) or set(item) != {"source_annotation"} or not isinstance(item["source_annotation"], Mapping):
            return None
        annotation = item["source_annotation"]
        code, refs = annotation.get("code"), annotation.get("affected_refs")
        if code not in _RULE_PACK_REF_CODES or not isinstance(refs, list) or len(refs) != 1 or not isinstance(refs[0], str) or not refs[0]:
            return None
        identity = refs[0]
        if parse_rule_pack_identity(identity) is None or identity in seen or dict(annotation) != rule_pack_ref_diagnostic(code, identity):
            return None
        seen.add(identity)
        entries.append((code, identity))
    return entries


def manifest_excluded_rule_pack_identities(input_manifest: Mapping[str, Any]) -> list[str]:
    """The identities of the model's references the manifest's `active_rule_packs` leaves
    out, in the model's order: what a record's rule-pack diagnostics must name."""
    return [rule_pack_identity(rule_pack_id, version) for rule_pack_id, version in manifest_excluded_rule_packs(input_manifest)]


def manifest_excluded_rule_packs(input_manifest: Mapping[str, Any]) -> list[tuple[str, str]]:
    """The model's references the manifest's `active_rule_packs` leaves out, as (id, version)
    pairs in the model's order."""
    # As Rust's `manifest_excluded_rule_packs` and the TypeScript: a list of packs and
    # references with text ids and versions is required.
    def pair(item: Any, version_key: str, what: str) -> tuple[str, str]:
        rule_pack_id = item.get("rule_pack_id") if isinstance(item, Mapping) else None
        version = item.get(version_key) if isinstance(item, Mapping) else None
        if not isinstance(rule_pack_id, str) or not isinstance(version, str):
            raise ValueError(f"ANALYSIS-RUN-MANIFEST-INVALID: {what} has no id or version.")
        return rule_pack_id, version
    # A model without references excludes nothing, whatever the manifest holds.
    payload = input_manifest.get("model_basis", {}).get("model_payload", {})
    if not isinstance(payload, Mapping) or "rule_pack_refs" not in payload:
        return []
    references = payload["rule_pack_refs"]
    if not isinstance(references, list):
        raise ValueError("ANALYSIS-RUN-MANIFEST-INVALID: the model's rule_pack_refs is not a list.")
    if not references:
        return []
    packs = input_manifest.get("active_rule_packs")
    if not isinstance(packs, list):
        raise ValueError("ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no active_rule_packs list.")
    active = [pair(pack, "version", "an active rule pack") for pack in packs]
    pairs = [pair(reference, "rule_pack_version", "a model rule-pack reference") for reference in references]
    return [item for item in pairs if item not in active]


def _build_analysis_run(
    mechanics_result: Mapping[str, Any], *, input_manifest_ref: Mapping[str, str], input_manifest_hash: str,
    input_manifest: Mapping[str, Any] | None = None,
    created_at: str | None = None, rule_check_status: str | None = None,
    hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1,
    solver_name: str = "unavailable:not-supplied", solver_version: str = "unavailable:not-supplied",
    solver_build_ref: Mapping[str, str] | None = None, settings_ref: Mapping[str, str] | None = None,
    unit_system_ref: Mapping[str, str] | None = None,
    rule_pack_ref_exclusions: Sequence[Mapping[str, str]] = (),
) -> dict[str, Any]:
    received = deepcopy(dict(mechanics_result))
    contract_id, contract_hash, contract_path = _source_contract(received)
    if contract_id not in CURRENT_RECORD_CONTRACT_IDS:
        raise ValueError("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH")
    run_id = str(received.get("run_id", "run:unknown"))
    rows = [deepcopy(dict(row)) for row in received.get("results", []) if isinstance(row, Mapping)]
    refs = []
    for row in rows:
        semantic, findings = _semantic(row, contract_path)
        row_id = str(row.get("id", "result:unknown"))
        refs.append({
            "result_ref": {"object_type": "Result", "ref": row_id},
            "source_row_index": len(refs),
            "category": semantic.get("category") if semantic else "unknown",
            "source_dimension": semantic.get("source_physical_semantic_dimension") if semantic else None,
            "result_family": semantic.get("family") if semantic else None,
            "semantic_contract": {"id": contract_id, "sha256": contract_hash, "signature_id": semantic.get("signature_id") if semantic else None},
            "interpretation": {"status": semantic.get("canonical_disposition") if semantic else "unavailable", "findings": findings},
            "source_annotation": {"kind": row.get("kind"), "unit": row.get("unit"), "metadata": deepcopy(row.get("metadata"))},
            "hash_refs": [_checksum("result_row", {"object_type": "Result", "ref": row_id}, row, hash_fn)],
            "privacy_classification": "source_evidence",
        })
    statuses = {"HUMAN_REVIEW_REQUIRED"}
    status = received.get("status") if isinstance(received.get("status"), Mapping) else {}
    statuses.update(str(status[field]) for field in ("mechanics", "rule_check") if status.get(field))
    if rule_check_status:
        statuses.discard(str(status.get("rule_check", "")))
        statuses.add(rule_check_status)
    run_ref = {"object_type": "AnalysisRun", "ref": run_id}
    received_ref = {"object_type": "ResultEnvelope", "ref": f"result-envelope:{run_id}"}
    manifest = deepcopy(dict(input_manifest)) if input_manifest is not None else None
    if manifest is not None and manifest.get("model_basis", {}).get("model_ref") != received.get("model_ref"):
        raise ValueError("ANALYSIS-RUN-INPUT-MANIFEST-MODEL-MISMATCH")
    rule_pack_diagnostics = [{"source_annotation": rule_pack_ref_diagnostic(item["code"], rule_pack_identity(item["rule_pack_id"], item["rule_pack_version"]))}
                             for item in rule_pack_ref_exclusions]
    built_refusal = _rule_pack_diagnostics_refusal([item["source_annotation"]["affected_refs"][0] for item in rule_pack_diagnostics], manifest) if manifest is not None else None
    if built_refusal:
        raise ValueError(built_refusal)
    solver_basis = manifest.get("solver_basis", {}) if manifest is not None else {}
    solver_name = str(solver_basis.get("solver_name", solver_name))
    solver_version = str(solver_basis.get("solver_version", solver_version))
    if solver_build_ref is None and solver_basis.get("solver_build_ref"):
        solver_build_ref = {"object_type": "ExternalReference", "ref": str(solver_basis["solver_build_ref"])}
    manifest_identity = f"{input_manifest_ref['ref']}:{input_manifest_hash}"
    if settings_ref is None:
        settings_ref = {"object_type": "SolverSettings", "ref": f"solver-settings:{manifest_identity}"}
    if unit_system_ref is None:
        unit_system_ref = {"object_type": "UnitSystem", "ref": f"unit-system:{received.get('model_ref', 'unknown')}:{input_manifest_hash}"}
    load_basis_refs: list[dict[str, str]] = []
    seen_basis: set[str] = set()
    for row in rows:
        basis = row.get("basis_ref") if isinstance(row.get("basis_ref"), Mapping) else None
        if not basis:
            continue
        object_type = "Combination" if basis.get("ref_type") == "combination" else "LoadCase"
        key = f"{object_type}:{basis.get('ref_id')}"
        if key not in seen_basis:
            seen_basis.add(key)
            load_basis_refs.append({"object_type": object_type, "ref": str(basis.get("ref_id"))})
    provenance = {"source_name": "OpenPipeStress analysis record 0.3", "source_location": "analysis_run.compatibility.v0.3", "source_license": "project-governed", "review_status": "pending", "professional_claim": False}
    determinism_notes = (["created_at_unavailable"] if created_at is None else []) + (["model_state_ref_and_solver_settings_unit_basis_bound_by_input_manifest"] if manifest is not None else ["input_manifest_payload_unavailable_solver_settings_unit_basis_not_independently_verified"])
    for row in refs:
        row["provenance"] = deepcopy(provenance)
    envelope: dict[str, Any] = {
        "schema_version": SCHEMA_VERSION, "deliverable_id": "DEL-14-02", "package_id": "PKG-14", "scope_item": "SOW-072", "objectives": ["OBJ-016"],
        "run_contract_status": {"record_contract": "strict_analysis_run_v0_3", "result_binding": "received_mechanics_result", "external_validation_boundary": "reference_only_not_determined_by_software"},
        "analysis_run": {
            "run_id": run_id, "run_name": f"{run_id} analysis record", "run_kind": "mechanics_solve", "created_at": created_at,
            "model_state_ref": transient_model_state_ref(received.get('model_ref', 'unknown')),
            "solver_version": {"solver_name": solver_name, "solver_version": solver_version, "build_ref": deepcopy(dict(solver_build_ref or {"object_type": "ExternalReference", "ref": "unavailable:solver-build-not-supplied"}))},
            "settings_ref": deepcopy(dict(settings_ref or {"object_type": "SolverSettings", "ref": "unavailable:solver-settings-not-supplied"})), "unit_system_ref": deepcopy(dict(unit_system_ref or {"object_type": "UnitSystem", "ref": "unavailable:unit-system-not-supplied"})),
            "load_basis_refs": load_basis_refs, "diagnostics": [{"source_annotation": deepcopy(item)} for item in received.get("diagnostics", [])] + rule_pack_diagnostics, "result_refs": refs, "rule_pack_refs": [], "library_refs": [],
            "hashes": [_checksum("received_result", received_ref, received, hash_fn)], "analysis_status": sorted(statuses),
            "reproducibility": {"input_manifest_refs": [deepcopy(dict(input_manifest_ref))], "input_manifest_hashes": [{"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": deepcopy(dict(input_manifest_ref)), "payload_scope": "input_manifest", "value": input_manifest_hash}], "semantic_contract": {"id": contract_id, "sha256": contract_hash}, "determinism_notes": determinism_notes, "unresolved_tbd": []},
            "immutability_policy": {"run_record_is_read_only": True, "mutation_policy": "changes_create_new_immutable_record_revision", "new_mechanics_run_required_for_record_revision": False, "record_revision_identity": "analysis_run_record_sha256", "hash_invalidates_external_acceptance": True},
            "professional_boundary": {"human_review_required": True, "software_makes_compliance_claim": False, "software_makes_certification_claim": False, "software_makes_sealing_claim": False, "software_makes_approval_claim": False, "software_makes_authentication_claim": False},
            "provenance": provenance,
        },
    }
    if contract_id in SOURCE_RECORD_CONTRACT_IDS:
        envelope["analysis_run"]["source_block_recovery"] = deepcopy(received["source_block_recovery"])
        envelope["analysis_run"]["contract_evidence"] = deepcopy(received["contract_evidence"])
    # C1:162; D2 4.9.6: the successor's AnalysisRun carries its complete receipt (B3b: either successor).
    if contract_id in RETAINED_CONTRACTS:
        envelope["analysis_run"]["retained_precision"] = deepcopy(received["retained_precision"])
    envelope["analysis_run"]["hashes"].insert(0, _checksum("analysis_run_record", run_ref, analysis_record_projection(envelope), hash_fn))
    return envelope


# W-12 S2 (DEL-14-01 Design `desktop-states-runs-comparison.md` section 3): a recorded
# revision. The desktop store's Record Run (`apps/desktop/src-tauri/src/analysis_runs.rs`)
# writes a new immutable revision of a solve's transient record, tied to a saved model
# state; `recorded_revision` mirrors it and the TypeScript `recordedAnalysisRunRevision`,
# and the parity tests check all three against the store's committed witness.
CREATED_AT_UNAVAILABLE = "created_at_unavailable"


def transient_model_state_ref(model_ref: Any) -> dict[str, str]:
    """A transient record's synthetic state ref, `state:<model ref>:preview`."""
    return {"object_type": "ModelState", "ref": f"state:{model_ref}:preview"}


def recorded_rule_pack_refs(input_manifest: Mapping[str, Any]) -> list[dict[str, str]]:
    """The input manifest's `active_rule_packs` (what the solve used), in its order."""
    packs = input_manifest.get("active_rule_packs")
    if not isinstance(packs, list):
        raise ValueError("ANALYSIS-RUN-MANIFEST-INVALID: the input manifest has no active_rule_packs list.")
    refs = []
    for pack in packs:
        rule_pack_id = pack.get("rule_pack_id") if isinstance(pack, Mapping) else None
        version = pack.get("version") if isinstance(pack, Mapping) else None
        if not isinstance(rule_pack_id, str) or not rule_pack_id or not isinstance(version, str) or not version:
            raise ValueError("ANALYSIS-RUN-MANIFEST-INVALID: an active rule pack has no id or version.")
        refs.append({"object_type": "RulePack", "ref": rule_pack_identity(rule_pack_id, version)})
    return refs


# Unicode White_Space, the set Rust's `str::trim` strips (and `trimLikeRust` in the
# webview). Python's `str.strip()` also strips U+001C..U+001F, which are not White_Space.
_RUST_WHITE_SPACE = "\u0009\u000a\u000b\u000c\u000d\u0020\u0085\u00a0\u1680" + "".join(
    chr(point) for point in range(0x2000, 0x200B)) + "\u2028\u2029\u202f\u205f\u3000"


def _library_identity(value: Any) -> str | None:
    """A `library_ref` identity member as the native walk normalizes it
    (`library_ref_identity`): Rust's trim, and nothing when blank or not text."""
    if not isinstance(value, str):
        return None
    return value.strip(_RUST_WHITE_SPACE) or None


def library_ref_sites(document: Any) -> list[tuple[str, Mapping[str, Any]]]:
    """Every `library_ref` object of a model document at any depth, with its path: the
    one walk (`libraryRefSites`, `model_document_save::library_ref_sites`), pinned to
    `fixtures/model_document/library_ref_sites.json`."""
    sites: list[tuple[str, Mapping[str, Any]]] = []

    def walk(value: Any, path: str) -> None:
        if isinstance(value, list):
            for index, item in enumerate(value):
                walk(item, f"{path}[{index}]")
        elif isinstance(value, Mapping):
            for key, child in value.items():
                at = f"{path}.{key}" if path else key
                if key == "library_ref" and isinstance(child, Mapping):
                    sites.append((at, child))
                walk(child, at)

    walk(document, "")
    return sites


def recorded_library_refs(model_payload: Any) -> list[dict[str, str]]:
    """The recorded revision's `library_refs`: every `library_ref` site of the solved
    model that the one walk finds, its identity normalized as the native walk does
    (Rust's White_Space trim), `library_version` verbatim when it is text. A site whose
    kind or id is blank or not text is omitted. Once each, in code-point order, as
    `library:<kind>/<id>[@<version>][#<record id>]`."""
    found: set[str] = set()
    for _, ref in library_ref_sites(model_payload):
        kind, library_id = _library_identity(ref.get("library_kind")), _library_identity(ref.get("library_id"))
        if kind is None or library_id is None:
            continue
        text = f"library:{kind}/{library_id}"
        if isinstance(ref.get("library_version"), str):
            text += f"@{ref['library_version']}"
        record_id = _library_identity(ref.get("record_id"))
        if record_id is not None:
            text += f"#{record_id}"
        found.add(text)
    return [{"object_type": "Library", "ref": text} for text in sorted(found)]


def recorded_revision(transient: Mapping[str, Any], *, state_id: str, created_at: str, input_manifest: Mapping[str, Any],
                      hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1) -> dict[str, Any]:
    """The recorded revision of a transient record: `created_at`, the stored state's ref,
    the manifest's rule packs and the solved model's library references, the
    `created_at_unavailable` note dropped and the record hash recomputed. The received-result
    checksum and every other member are unchanged."""
    record = deepcopy(dict(transient))
    run = record["analysis_run"]
    run["created_at"] = created_at
    run["model_state_ref"] = {"object_type": "ModelState", "ref": state_id}
    run["rule_pack_refs"] = recorded_rule_pack_refs(input_manifest)
    run["library_refs"] = recorded_library_refs(input_manifest["model_basis"]["model_payload"])
    notes = run.get("reproducibility", {}).get("determinism_notes")
    if isinstance(notes, list):
        run["reproducibility"]["determinism_notes"] = [note for note in notes if note != CREATED_AT_UNAVAILABLE]
    claims = [row for row in run["hashes"] if row.get("payload_scope") == "analysis_run_record"]
    if len(claims) != 1:
        raise ValueError("ANALYSIS-RUN-RECORD-INVALID: the record has no single analysis_run_record checksum.")
    claims[0]["value"] = hash_fn(analysis_record_projection(record))
    return record


def _safe_token(value: str) -> str:
    """`inputManifestService.ts`'s `safeToken`."""
    return re.sub(r"[^A-Za-z0-9._-]+", "-", value).strip("-") or "current-session"


def manifest_load_basis_refs(input_manifest: Mapping[str, Any]) -> list[dict[str, str]]:
    """The recorded form's load basis: the stored manifest's load cases, then its combinations."""
    basis = input_manifest.get("load_basis", {})
    return ([{"object_type": "LoadCase", "ref": case["id"]} for case in basis.get("load_cases", [])]
            + [{"object_type": "Combination", "ref": combination["id"]} for combination in basis.get("combinations", [])])


def manifest_fixed_members(input_manifest: Mapping[str, Any]) -> list[tuple[str, str, Any, str]]:
    """Every record member the stored input manifest fixes, derived from the manifest (never
    from the record), as (section, member, expected, refusal code); mirrors the TypeScript
    `manifestFixedMembers`."""
    sha = canonical_sha256_checked_v1(input_manifest)
    model_ref = input_manifest["model_basis"]["model_ref"]
    manifest_ref = {"object_type": "InputManifest", "ref": f"input-manifest:{_safe_token(model_ref)}:{sha}"}
    solver = input_manifest["solver_basis"]
    return [
        ("reproducibility", "input_manifest_refs", [manifest_ref], "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH"),
        ("reproducibility", "input_manifest_hashes", [{"algorithm": "sha256", "canonicalization": "rfc8785_jcs", "payload_ref": manifest_ref, "payload_scope": "input_manifest", "value": sha}], "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH"),
        ("run", "solver_version", {"solver_name": solver["solver_name"], "solver_version": solver["solver_version"], "build_ref": {"object_type": "ExternalReference", "ref": solver["solver_build_ref"]}}, "ANALYSIS_RECORDED_SOLVER_MISMATCH"),
        ("run", "settings_ref", {"object_type": "SolverSettings", "ref": f"solver-settings:{manifest_ref['ref']}:{sha}"}, "ANALYSIS_RECORDED_SETTINGS_MISMATCH"),
        ("run", "unit_system_ref", {"object_type": "UnitSystem", "ref": f"unit-system:{model_ref}:{sha}"}, "ANALYSIS_RECORDED_UNITS_MISMATCH"),
        ("run", "load_basis_refs", manifest_load_basis_refs(input_manifest), "ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH"),
        ("run", "rule_pack_refs", recorded_rule_pack_refs(input_manifest), "ANALYSIS_RECORDED_REFERENCES_MISMATCH"),
        ("run", "library_refs", recorded_library_refs(input_manifest["model_basis"]["model_payload"]), "ANALYSIS_RECORDED_REFERENCES_MISMATCH"),
    ]


def _validate_recorded_binding(run: Mapping[str, Any], source: Mapping[str, Any], recorded_state: Mapping[str, Any]) -> None:
    """A recorded revision's binding to its stored state (as the TypeScript validator's):
    its ref names the state; its `created_at` is set and the `created_at_unavailable` note is
    gone; the stored input manifest is the source's project's and its model payload hashes to
    the state's model hash; and every member the manifest fixes is the manifest's (JCS: the
    checked profile's canonical text is JCS for the values it admits). Equality is
    structural: member order is not significant, array order is, extra members differ."""
    if run.get("model_state_ref") != {"object_type": "ModelState", "ref": recorded_state["state_id"]}:
        raise ValueError("ANALYSIS_SOURCE_MODEL_MISMATCH")
    reproducibility = run.get("reproducibility", {})
    if not isinstance(run.get("created_at"), str) or not run["created_at"] or CREATED_AT_UNAVAILABLE in reproducibility.get("determinism_notes", []):
        raise ValueError("ANALYSIS_RECORDED_REVISION_TIME_MISSING")
    manifest = recorded_state["input_manifest"]
    if manifest.get("model_basis", {}).get("model_ref") != source.get("model_ref"):
        raise ValueError("ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH")
    if f"sha256:{canonical_sha256_checked_v1(manifest['model_basis']['model_payload'])}" != recorded_state["model_hash"]:
        raise ValueError("ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH")
    for section, member, expected, code in manifest_fixed_members(manifest):
        holder = reproducibility if section == "reproducibility" else run
        if member not in holder or holder[member] != expected:
            raise ValueError(code)


def verify_analysis_run_record(envelope: Mapping[str, Any], hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1) -> str:
    version = envelope.get("schema_version")
    if version != SCHEMA_VERSION:
        if isinstance(version, str) and version in RETIRED_SCHEMA_VERSIONS:
            raise ValueError(f"{ANALYSIS_RUN_SCHEMA_VERSION_RETIRED}: analysis-run {version} is no longer read; re-run the analysis to record a current {SCHEMA_VERSION} analysis run")
        raise ValueError(f"ANALYSIS-RUN-SCHEMA-VERSION-UNSUPPORTED: {version}")
    matches = [row for row in envelope["analysis_run"]["hashes"] if row.get("payload_scope") == "analysis_run_record"]
    if len(matches) != 1:
        return "unverifiable"
    expected_ref = {"object_type": "AnalysisRun", "ref": envelope["analysis_run"].get("run_id")}
    claim = matches[0]
    if claim.get("algorithm") != "sha256" or claim.get("canonicalization") != PROFILE or claim.get("payload_ref") != expected_ref:
        return "mismatch"
    return "match" if claim.get("value") == hash_fn(analysis_record_projection(envelope)) else "mismatch"


# B7b (A4/A5): the current exact contract is `3.0.0/exact_pressure_v3` only. Its two identities take over the retired
# v2 identities' product capabilities (physics-1, physics-source-1, load-reference-1, load-reference-source-1, which
# the Rust reader refuses by name: SOURCE_EXACT_PRESSURE_V2_RETIRED). Rows are interpreted lazily per record, as every
# DEL-14-02 table is; no additional resident row table is held here.
# pressure-source-1: the retained-source composite (receipt required); its record carries source_block_recovery and
# contract_evidence.
PRESSURE_SOURCE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-source-1"
# T4-U6 (D-5, §2.4): pressure-2, pressure-source-1 and physics-retained-1 carry one stress signature per (recovery
# token x section basis); a record binding the former table hashes is refused (ANALYSIS_SEMANTIC_CONTRACT_MISMATCH).
PRESSURE_SOURCE_CONTRACT_SHA256 = "b5a4b0e585b9ec440d36144860baaf101b628498cfdab9a1de9248c2ed5069e7"
_PRESSURE_SOURCE_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_pressure_source_1.json"
# pressure-2: the ordinary exact route (a load/reference-state model publishes it with
# contract_evidence.load_reference_states). Its record carries neither source_block_recovery nor contract_evidence.
PRESSURE_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/pressure-2"
PRESSURE_CONTRACT_SHA256 = "d381ee8bb2e4dfad7d2a050eaa23b558b32cc2ebd3599c0d4eb0489e49246fd2"
_PRESSURE_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_pressure_2.json"
PREVIEW_PHYSICS_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-2"
PREVIEW_PHYSICS_CONTRACT_SHA256 = "5a6630d9157ad1f4232c558d0cd57414677add8d1c95cc0a5d19a2579fb3a05b"
_PREVIEW_PHYSICS_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_2.json"
# U6b (D-U6-6; D2 4.7 S-1, 4.9.6): the F2a preview successor. Its statements are
# checked only by the accepted reader (`retained_precision`), and its standing
# comes only from that reader's verified receipt (D1 5 item 3).
PREVIEW_PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/preview-physics-retained-1"
PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256 = "7d59a8e7fa7b3b997d3809bd90b7efbdda6c91ad46c0fbe31420f11dc4350520"
_PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_preview_physics_retained_1.json"
# B3b (B3-D §2, §6.1): the exact successor, whose base is pressure-2 since B7b D4 (a). As the preview successor, its
# statements are checked only by the accepted reader, which dispatches on this identity, and its standing comes only
# from that reader.
PHYSICS_RETAINED_CONTRACT_ID = "openpipestress.result_semantics/0.3.0/physics-retained-1"
PHYSICS_RETAINED_CONTRACT_SHA256 = "ef0191ccc74a01a81ce0a5b603629f7e1166b10f87bf66334b2ebbeb1c133ac8"
_PHYSICS_RETAINED_CONTRACT_PATH = _RESULTS_DIR / "semantic_contract_v0_3_physics_retained_1.json"
RETAINED_CONTRACTS = {PREVIEW_PHYSICS_RETAINED_CONTRACT_ID: (PREVIEW_PHYSICS_RETAINED_CONTRACT_SHA256, _PREVIEW_PHYSICS_RETAINED_CONTRACT_PATH),
                      PHYSICS_RETAINED_CONTRACT_ID: (PHYSICS_RETAINED_CONTRACT_SHA256, _PHYSICS_RETAINED_CONTRACT_PATH)}
RETAINED_METHOD = "contribution_preserving_multiprecision_v1"
RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH = "ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH"
ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN = "ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN"
RULE_QUANTITY_BELOW_VERIFIED_FLOOR = "RULE_QUANTITY_BELOW_VERIFIED_FLOOR"
RULE_QUANTITY_NOT_COVERED = "RULE_QUANTITY_NOT_COVERED"
# Static fresh-publication identities (S1 §10; Rust `FRESH_IDENTITIES`); no route predicate.
FRESH_CONTRACT_IDS = frozenset({PREVIEW_PHYSICS_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID,
                                # T4-U2: pressure-2 (`3.0.0/exact_pressure_v3`).
                                PRESSURE_CONTRACT_ID,
                                # U6b (D-U6-6): membership is not standing.
                                PREVIEW_PHYSICS_RETAINED_CONTRACT_ID})
# Every current-record identity, which the 0.3 AnalysisRun builder and validator admit (B7b: pressure-2 and
# pressure-source-1 in place of the retired v2 identities).
CURRENT_RECORD_CONTRACT_IDS = frozenset({PRESSURE_CONTRACT_ID, PRESSURE_SOURCE_CONTRACT_ID, PREVIEW_PHYSICS_CONTRACT_ID,
                                         PREVIEW_PHYSICS_RETAINED_CONTRACT_ID, PHYSICS_RETAINED_CONTRACT_ID})
# The identities whose record carries the received source_block_recovery and contract_evidence.
SOURCE_RECORD_CONTRACT_IDS = frozenset({PRESSURE_SOURCE_CONTRACT_ID})


# The members a carrier keeps of a statement: what a transport reading reads.
_TRANSPORTED = ("schema_version", "producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence")
# Each table a route binds, by identity: the reader's binding `{id, sha256}` must name one of these.
_TABLES = {PRESSURE_SOURCE_CONTRACT_ID: (PRESSURE_SOURCE_CONTRACT_SHA256, _PRESSURE_SOURCE_CONTRACT_PATH),
           PREVIEW_PHYSICS_CONTRACT_ID: (PREVIEW_PHYSICS_CONTRACT_SHA256, _PREVIEW_PHYSICS_CONTRACT_PATH),
           PRESSURE_CONTRACT_ID: (PRESSURE_CONTRACT_SHA256, _PRESSURE_CONTRACT_PATH),
           **RETAINED_CONTRACTS}


def _same_canonical(a: Any, b: Any) -> bool:
    """RV92 S-1: equal as checked canonical JSON bytes, as TS's same() compares, so
    false is not 0 and true is not 1, while 0.0 and 0 are one JSON value. A value
    outside the checked profile equals nothing."""
    try:
        return canonical_json_checked_v1(a) == canonical_json_checked_v1(b)
    except ValueError:
        return False


def _is_retained(source: Any) -> bool:
    producer = source.get("producer") if isinstance(source, Mapping) else None
    contract = producer.get("semantic_contract_id") if isinstance(producer, Mapping) else None
    return isinstance(contract, str) and contract in RETAINED_CONTRACTS


def _refused(refusal: rust_reader.ReaderRefusal) -> ValueError:
    """The reader's refusal as product code raises it: its whole text (detail), otherwise its code."""
    return ValueError(refusal.detail or refusal.code)


def _retained_validation(source: Mapping[str, Any], invocation: Any = None) -> dict[str, Any]:
    """The Rust reader's G0-G8 on a successor (`retained_precision::validate`, through
    `openpipestress-runner read`). A refusal raises the Rust reader's detail when it
    carries one (a G7 base failure's text, in Rust's base validator's words), otherwise
    its code."""
    try:
        return rust_reader.retained_precision(source, invocation)
    except rust_reader.ReaderRefusal as refusal:
        raise _refused(refusal) from refusal


def _retained_transport(source: Mapping[str, Any]) -> dict[str, Any]:
    """F-U6b-2 (B6): the Rust reader's transport checks on a successor
    (`retained_precision::validate_transport_metadata`, through `openpipestress-runner
    read`: G0-G2, then the base header and the base's transport metadata on its
    projection); never eligible. A refusal raises the Rust reader's detail when it
    carries one (a G7 base failure's text, in Rust's base validator's words), otherwise
    its code."""
    try:
        return rust_reader.retained_precision_transport(source)
    except rust_reader.ReaderRefusal as refusal:
        raise _refused(refusal) from refusal


def _retained_contract(source: Mapping[str, Any], *, check_receipt: bool) -> tuple[str, str, Path]:
    if not check_receipt:
        # F-U6b-2 (B6): a transported successor (its receipt, with or without raw
        # rows) is checked by the reader's transport validator, never admitted unchecked.
        _retained_transport(source)
    else:
        _retained_validation(source)
    contract = source["producer"]["semantic_contract_id"]
    return (contract, *RETAINED_CONTRACTS[contract])


def _source_contract(source: Mapping[str, Any], *, check_receipt: bool = True) -> tuple[str, str, Path]:
    """The Rust reader's reading of a statement: its table `(id, sha256, path)`, or its refusal raised.

    A successor goes to the retained reader (R1). Every other statement takes the Rust header
    dispatch (`reader_verdict::dispatch`, b7-plan R2 G5), then its route's reader: the raw
    validation (`check_receipt`), or the transport checks of a statement without raw rows. A
    stored v2 result is refused by the header (`SOURCE_EXACT_PRESSURE_V2_RETIRED`), and pressure-2
    has no transport reader (Rust refuses it by name, `SOURCE_PRESSURE_TRANSPORT_UNSUPPORTED`).
    Interpretation dispatch does not authenticate a producer or qualify Current."""
    if _is_retained(source):
        return _retained_contract(source, check_receipt=check_receipt)
    # A transport reading reads the transported members only (as Rust's header-only `for_source_metadata`
    # and the carriers that keep them), never raw rows the statement may still hold.
    verdict = rust_reader.dispatch(source if check_receipt or not isinstance(source, Mapping)
                                   else {key: source[key] for key in _TRANSPORTED if key in source})
    if not verdict["ok"]:
        error = verdict["error"]
        raise ValueError(error["detail"] or error["code"])
    route = verdict["route"]
    if route in {"retained_preview_physics", "retained_physics"}:
        # Only a successor identity routes here, and it went to the retained reader above.
        raise RuntimeError(f"RUST-READER-RESPONSE-INVALID: {rust_reader.DISPATCH}")
    reader = rust_reader.ROUTE_READERS.get(route)
    if reader is not None:
        try:
            if check_receipt:
                rust_reader.validate(reader, source)
            else:
                rust_reader.transport(reader, source)
        except rust_reader.ReaderRefusal as refusal:
            raise _refused(refusal) from refusal
    binding = verdict["binding"]
    table = _TABLES.get(binding["id"])
    if table is None or table[0] != binding["sha256"]:
        raise RuntimeError(f"RUST-READER-BINDING-UNKNOWN: {binding['id']}")
    return binding["id"], *table


def numerical_use_standing(source: Mapping[str, Any], requested_basis_refs: list[Mapping[str, str]], source_block_context: Mapping[str, Any] | None = None) -> str:
    """Derived numerical eligibility only; never mutates an authentic historical record.

    The Rust reader's `numerical_use_standing_with_context`. A positive result still
    requires the caller's existing model/input/build/source authentication. Matching
    an input hash alone supplies no numerical evidence. A statement that is not JSON
    is unsupported.
    """
    try:
        return rust_reader.standing(source, requested_basis_refs, source_block_context)["token"]
    except (ValueError, TypeError):
        return "unsupported"


def standing_reason(source: Mapping[str, Any]) -> str | None:
    """Derived reason a readable source is not Current; None when none applies.

    The Rust reader's `standing_reason` of a statement it reads (`_source_contract`).
    Only the named historical semantics are reported. Numerical standing is still
    decided by ``numerical_use_standing``; an unreadable source has no reason here
    and is ``unsupported`` there.
    """
    try:
        _source_contract(source)
    except (ValueError, KeyError, TypeError, AttributeError):
        return None
    return rust_reader.standing(source, [])["reason"]


def classification_summary(source: Mapping[str, Any], invocation: Any = None,
                           requested_basis_refs: list[Mapping[str, str]] | None = None) -> list[dict[str, Any]]:
    """D2 4.9.9: the Rust reader's per-case counts over a successor's validated G5c classes
    (`semantic_contract::classification_summary`). Any other identity, or a refused
    statement (one that is not JSON included), returns nothing."""
    try:
        return rust_reader.classification_summary(source, invocation, requested_basis_refs or [])
    except (ValueError, TypeError):
        return []


def is_fresh_contract_id(contract_id: Any) -> bool:
    return isinstance(contract_id, str) and contract_id in FRESH_CONTRACT_IDS


def rule_binding_refusal(envelope: Mapping[str, Any], row: Mapping[str, Any]) -> str | None:
    """The Rust reader's ``semantic_contract::rule_binding_refusal`` (S1 §10; D2 4.9.9).

    A successor row binds only as its validated class allows. A statement that is not JSON is refused
    (ValueError or TypeError): no reader reads it, so no row of it is bound.
    """
    return rust_reader.rule_binding(envelope, row)


RULE_STATUSES = {"RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED"}


def _source_reference_text(value: Any) -> str:
    if not isinstance(value, str) or not value:
        raise ValueError("ANALYSIS_SOURCE_REFERENCE_INVALID")
    return value


def _source_basis_reference(basis: Any) -> dict[str, str] | None:
    if basis is None:
        return None
    if not isinstance(basis, Mapping) or set(basis) != {"ref_type", "ref_id"}:
        raise ValueError("ANALYSIS_SOURCE_REFERENCE_INVALID")
    kind = _source_reference_text(basis.get("ref_type"))
    ref_id = _source_reference_text(basis.get("ref_id"))
    return {"object_type": {"load_case": "LoadCase", "combination": "Combination"}.get(kind, "ResultBasis"), "ref": ref_id}


def _validate_source_reference_fields(source: Mapping[str, Any]) -> None:
    for field in ("run_id", "model_ref"):
        _source_reference_text(source.get(field))
    for row in source.get("results", []):
        _source_reference_text(row.get("id"))
        _source_basis_reference(row.get("basis_ref"))


def _expected_load_basis(source: Mapping[str, Any], expected_basis_refs: list[Mapping[str, str]] | None) -> list[dict[str, str]]:
    """Available source bases are not evidence of requested-model completeness.

    An override is independent caller evidence and must be supplied again when
    validating; the record's own list is never used as its warrant.
    """
    if expected_basis_refs is not None:
        refs = deepcopy(expected_basis_refs)
        if not isinstance(refs, list) or any(not isinstance(ref, Mapping) or set(ref) != {"object_type", "ref"} or ref["object_type"] not in {"LoadCase", "Combination", "ResultBasis"} or not isinstance(ref["ref"], str) or not ref["ref"] for ref in refs) or any(ref in refs[:index] for index, ref in enumerate(refs)):
            raise ValueError("ANALYSIS_LOAD_BASIS_INVALID")
        for item in [*source.get("results", []), *source.get("numerical_quality", {}).get("cases", [])]:
            required = _source_basis_reference(item.get("basis_ref"))
            if required is not None:
                if required not in refs:
                    raise ValueError("ANALYSIS_LOAD_BASIS_SOURCE_SCOPE_MISMATCH")
        return refs
    refs = []
    for row in source.get("results", []):
        ref = _source_basis_reference(row.get("basis_ref"))
        if ref is not None and ref not in refs:
            refs.append(ref)
    return refs


def build_analysis_run_v0_3(mechanics_result: Mapping[str, Any], *, expected_basis_refs: list[Mapping[str, str]] | None = None, **kwargs: Any) -> dict[str, Any]:
    """Strict source-bound 0.3 record; optional basis is independent evidence."""
    if _source_contract(mechanics_result)[0] not in CURRENT_RECORD_CONTRACT_IDS:
        raise ValueError("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH")
    _validate_source_reference_fields(mechanics_result)
    producer = mechanics_result["producer"]
    kwargs.setdefault("solver_name", producer["component_name"])
    kwargs.setdefault("solver_version", producer["component_version"])
    source_rule = mechanics_result.get("status", {}).get("rule_check")
    if source_rule is not None and (not isinstance(source_rule, str) or source_rule not in RULE_STATUSES):
        raise ValueError("ANALYSIS_RULE_STATUS_INVALID")
    override = kwargs.get("rule_check_status")
    effective_rule = override if override is not None else (source_rule if source_rule is not None else "RULE_INPUTS_INCOMPLETE")
    if not isinstance(effective_rule, str) or effective_rule not in RULE_STATUSES:
        raise ValueError("ANALYSIS_RULE_STATUS_INVALID")
    kwargs["rule_check_status"] = effective_rule
    record = _build_analysis_run(mechanics_result, **kwargs)
    record["analysis_run"]["load_basis_refs"] = _expected_load_basis(mechanics_result, expected_basis_refs)
    hash_fn = kwargs.get("hash_fn", canonical_sha256_checked_v1)
    record["analysis_run"]["hashes"][0]["value"] = hash_fn(analysis_record_projection(record))
    validate_analysis_run_v0_3(record, mechanics_result, hash_fn=hash_fn, expected_basis_refs=expected_basis_refs)
    return record


def build_analysis_run(mechanics_result: Mapping[str, Any], **kwargs: Any) -> dict[str, Any]:
    """The current (0.3.0) record; a result no current record contract reads is
    refused (`ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH`), never recorded in an
    older format."""
    return build_analysis_run_v0_3(mechanics_result, **kwargs)


def validate_analysis_run_v0_3(envelope: Mapping[str, Any], source: Mapping[str, Any], *, hash_fn: Callable[[Any], str] = canonical_sha256_checked_v1, expected_basis_refs: list[Mapping[str, str]] | None = None,
                               recorded_state: Mapping[str, Any] | None = None) -> None:
    """Validate source interpretation/bindings separately from checksum verification.

    A transient record (no ``recorded_state``) names its synthetic state; a recorded
    revision (W-12 S2) names the stored state passed in as ``{"state_id", "model_hash",
    "input_manifest"}``, checked against that state's model hash.
    This accepts supplied evidence only; it does not authenticate a claimed build.
    """
    contract_id, contract_hash, path = _source_contract(source)
    _validate_source_reference_fields(source)
    if contract_id not in CURRENT_RECORD_CONTRACT_IDS or envelope.get("schema_version") != "0.3.0" or envelope.get("run_contract_status", {}).get("record_contract") != "strict_analysis_run_v0_3":
        raise ValueError("ANALYSIS_SOURCE_CONTRACT_VERSION_MISMATCH")
    run = envelope.get("analysis_run", {})
    # The recorded form (W-12 S2): every member the stored manifest fixes is checked first,
    # against the manifest; its load basis is the manifest's, never a caller's or the record's.
    if recorded_state is not None:
        _validate_recorded_binding(run, source, recorded_state)
        manifest_basis = manifest_load_basis_refs(recorded_state["input_manifest"])
        if expected_basis_refs is not None and list(expected_basis_refs) != manifest_basis:
            raise ValueError("ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH")
        expected_basis_refs = manifest_basis
    if contract_id in SOURCE_RECORD_CONTRACT_IDS:
        if not _same_canonical(run.get("source_block_recovery"), source["source_block_recovery"]):
            raise ValueError("ANALYSIS_SOURCE_BLOCK_RECEIPT_MISMATCH")
    elif "source_block_recovery" in run:
        raise ValueError("SOURCE_BLOCKS_DOWNGRADE_FORBIDDEN")
    if contract_id in SOURCE_RECORD_CONTRACT_IDS:
        if not _same_canonical(run.get("contract_evidence"), source["contract_evidence"]):
            raise ValueError("ANALYSIS_PHYSICS_SOURCE_EVIDENCE_MISMATCH")
    elif "contract_evidence" in run:
        raise ValueError("ANALYSIS_PHYSICS_SOURCE_DOWNGRADE_FORBIDDEN")
    if contract_id in RETAINED_CONTRACTS:
        if "retained_precision" not in run or not _same_canonical(run["retained_precision"], source["retained_precision"]):
            raise ValueError(ANALYSIS_RETAINED_PRECISION_RECEIPT_MISMATCH)
    elif "retained_precision" in run:
        raise ValueError(ANALYSIS_RETAINED_PRECISION_DOWNGRADE_FORBIDDEN)
    # The producer's diagnostics, then any rule-pack reference diagnostics (each exactly the one
    # its code and identity determine); a recorded revision's name exactly the references its
    # stored manifest leaves out of active_rule_packs. Their codes are under the record hash.
    producer = [{"source_annotation": deepcopy(item)} for item in source.get("diagnostics", [])]
    diagnostics = run.get("diagnostics")
    if not isinstance(diagnostics, list) or diagnostics[:len(producer)] != producer:
        raise ValueError("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH")
    rule_pack_entries = rule_pack_ref_diagnostic_entries(diagnostics[len(producer):])
    if rule_pack_entries is None:
        raise ValueError("ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH")
    recorded_refusal = _rule_pack_diagnostics_refusal([identity for _, identity in rule_pack_entries], recorded_state["input_manifest"]) if recorded_state is not None else None
    # The named refusal of a run recorded before rule-pack resolution comes only after every
    # other check below passes (the record checksum last), so corruption keeps its own code.
    deferred_refusal = recorded_refusal if recorded_refusal == _RULE_PACKS_NOT_RESOLVED_REFUSAL else None
    if recorded_refusal and not deferred_refusal:
        raise ValueError(recorded_refusal)
    statuses = run.get("analysis_status")
    source_rule = source.get("status", {}).get("rule_check")
    if source_rule is not None and (not isinstance(source_rule, str) or source_rule not in RULE_STATUSES):
        raise ValueError("ANALYSIS_RULE_STATUS_INVALID")
    mechanics = source.get("status", {}).get("mechanics")
    if mechanics not in {"MODEL_INCOMPLETE", "MECHANICS_SOLVED"}:
        raise ValueError("ANALYSIS_SOURCE_MECHANICS_STATUS_INVALID")
    if not isinstance(statuses, list) or any(not isinstance(item, str) for item in statuses) or len(statuses) != len(set(statuses)) or len(set(statuses) & RULE_STATUSES) != 1 or set(statuses) - RULE_STATUSES != {mechanics, "HUMAN_REVIEW_REQUIRED"}:
        raise ValueError("ANALYSIS_SOURCE_STATUS_MISMATCH")
    if run.get("load_basis_refs") != _expected_load_basis(source, expected_basis_refs):
        raise ValueError("ANALYSIS_SOURCE_LOAD_BASIS_MISMATCH")
    solver = run.get("solver_version", {})
    if solver.get("solver_name") != source["producer"]["component_name"] or solver.get("solver_version") != source["producer"]["component_version"]:
        raise ValueError("ANALYSIS_SOURCE_PRODUCER_MISMATCH")
    if recorded_state is None and run.get("model_state_ref") != transient_model_state_ref(source.get("model_ref")):
        raise ValueError("ANALYSIS_SOURCE_MODEL_MISMATCH")
    if run.get("run_id") != source.get("run_id"):
        raise ValueError("ANALYSIS_SOURCE_RUN_MISMATCH")
    semantic = {"id": contract_id, "sha256": contract_hash}
    if run.get("reproducibility", {}).get("semantic_contract") != semantic:
        raise ValueError("ANALYSIS_SEMANTIC_CONTRACT_MISMATCH")
    hashes = [item for item in run.get("hashes", []) if item.get("payload_scope") == "received_result"]
    expected_hash = _checksum("received_result", {"object_type": "ResultEnvelope", "ref": f"result-envelope:{source.get('run_id')}"}, source, hash_fn)
    if hashes != [expected_hash]:
        raise ValueError("ANALYSIS_RECEIVED_SOURCE_MISMATCH")
    rows = source.get("results", [])
    refs = run.get("result_refs", [])
    if len(rows) != len(refs) or len({row.get("id") for row in rows}) != len(rows):
        raise ValueError("ANALYSIS_ROW_ACCOUNTING_MISMATCH")
    for index, (row, ref) in enumerate(zip(rows, refs)):
        interpretation, findings = _semantic(row, path)
        expected_semantic = {**semantic, "signature_id": interpretation.get("signature_id") if interpretation else None}
        expected_ref = {"object_type": "Result", "ref": str(row.get("id", "result:unknown"))}
        if ref.get("source_row_index") != index or ref.get("result_ref") != expected_ref or ref.get("semantic_contract") != expected_semantic or ref.get("hash_refs") != [_checksum("result_row", expected_ref, row, hash_fn)]:
            raise ValueError("ANALYSIS_ROW_SOURCE_BINDING_MISMATCH")
        if ref.get("category") != (interpretation.get("category") if interpretation else "unknown") or ref.get("source_dimension") != (interpretation.get("source_physical_semantic_dimension") if interpretation else None) or ref.get("result_family") != (interpretation.get("family") if interpretation else None) or ref.get("interpretation") != {"status": interpretation.get("canonical_disposition") if interpretation else "unavailable", "findings": findings} or ref.get("source_annotation") != {"kind": row.get("kind"), "unit": row.get("unit"), "metadata": row.get("metadata")}:
            raise ValueError("ANALYSIS_ROW_INTERPRETATION_MISMATCH")
    if verify_analysis_run_record(envelope, hash_fn) != "match":
        raise ValueError("ANALYSIS_RECORD_CHECKSUM_MISMATCH")
    if deferred_refusal:
        raise ValueError(deferred_refusal)
