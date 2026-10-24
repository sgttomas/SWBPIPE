"""Independent verifier of a handoff directory package (DEL-15-01 Design
`canonical-model-export.md` §3, W-6b): the Python twin of the desktop's
`verify_handoff_package` (`apps/desktop/src-tauri/src/handoff_package_publish.rs`).

A package `<name>.swbhandoff/` holds exactly `handoff_package.json` and
`model/model.json` (Finder's `.DS_Store` files are tolerated, not members). The
verifier requires
- both members to be UTF-8 JSON that is its own RFC 8785 (JCS) text;
- the manifest's `model_hash.value` to be the SHA-256 of the model member's bytes;
- the manifest member to be exactly the contract envelope around its
  `handoff_package_manifest`, whose `checksums` are exactly the member
  inventory: the model member's SHA-256 and the export hash, the SHA-256 of the
  JCS of the manifest with its `checksums` member removed.

So a change to any byte of either member fails. Test-only and independent: it
uses the standard library and its sibling `canonical_json` (the checked JCS
profile) and calls nothing the producer links.
"""
from __future__ import annotations

from hashlib import sha256
import json
from pathlib import Path
from typing import Any

from .canonical_json import canonical_json

MODEL_DIRECTORY = "model"
MODEL_FILE = "model.json"
MODEL_MEMBER = "model/model.json"
MANIFEST_MEMBER = "handoff_package.json"
FINDER_METADATA = ".DS_Store"
HANDOFF_SCHEMA_VERSION = "0.1.0"

CONTRACT_STATUS = {
    "record_contract": "schema_first_canonical_handoff_package_records",
    "manifest_contract": "schema_first_handoff_manifest_metadata",
    "model_schema_binding": "schemas/model.schema.yaml",
    "units_schema_binding": "schemas/units.schema.yaml",
    "model_state_binding": "schemas/model_state.schema.json",
    "analysis_run_binding": "schemas/analysis_run.schema.json",
    "result_export_binding": "schemas/results.schema.yaml",
    "local_fea_handoff_binding": "schemas/local_fea_handoff.schema.yaml",
    "audit_hash_binding": "hash_records_follow_audit_manifest_predecessor_semantics",
    "target_mapping_taxonomy": "reserved_for_DEL-15-02",
    "physical_package_container": "TBD",
    "external_prover_status": "not_declared_by_handoff_package",
    "professional_boundary": "downstream_modeling_and_review_support_only",
}


class HandoffVerificationError(ValueError):
    """The package does not verify."""


def _entries(directory: Path) -> list[Path]:
    return sorted(entry for entry in directory.iterdir() if not (entry.name == FINDER_METADATA and entry.is_file() and not entry.is_symlink()))


def _layout_ok(directory: Path) -> bool:
    if directory.is_symlink() or not directory.is_dir():
        return False
    top = _entries(directory)
    names = {entry.name: entry for entry in top}
    if len(top) != 2 or set(names) != {MANIFEST_MEMBER, MODEL_DIRECTORY}:
        return False
    manifest, model = names[MANIFEST_MEMBER], names[MODEL_DIRECTORY]
    if manifest.is_symlink() or not manifest.is_file() or model.is_symlink() or not model.is_dir():
        return False
    inner = _entries(model)
    return len(inner) == 1 and inner[0].name == MODEL_FILE and inner[0].is_file() and not inner[0].is_symlink()


def _own_jcs(raw: bytes, member: str) -> Any:
    try:
        text = raw.decode("utf-8")
        value = json.loads(text)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise HandoffVerificationError(f"{member} is not UTF-8 JSON ({error})") from error
    try:
        canonical = canonical_json(value)
    except ValueError as error:
        raise HandoffVerificationError(f"{member} is outside the canonical JSON profile ({error})") from error
    if canonical != text:
        raise HandoffVerificationError(f"{member} is not its own RFC 8785 (JCS) text")
    return value


def export_hash(manifest: dict[str, Any]) -> str:
    """SHA-256 hex of the JCS of the manifest with `checksums` removed."""
    payload = {key: value for key, value in manifest.items() if key != "checksums"}
    return sha256(canonical_json(payload).encode("utf-8")).hexdigest()


def checksum_inventory(model_ref: Any, handoff_package_id: Any, model_hex: str, export_hex: str) -> list[dict[str, Any]]:
    member_ref = dict(model_ref) if isinstance(model_ref, dict) else {}
    member_ref["label"] = MODEL_MEMBER
    return [
        {"algorithm": "sha256", "canonicalization": "JCS", "payload_ref": member_ref, "payload_scope": "model_payload", "value": model_hex},
        {
            "algorithm": "sha256",
            "canonicalization": "JCS",
            "payload_ref": {"object_type": "HandoffPackage", "ref": handoff_package_id, "label": MANIFEST_MEMBER},
            "payload_scope": "handoff_package_manifest",
            "value": export_hex,
        },
    ]


def verify_handoff_package(directory: Path | str) -> dict[str, str]:
    """Returns `{"model_hash", "export_hash"}` (`sha256:<hex>`) or raises HandoffVerificationError."""
    directory = Path(directory)
    if not _layout_ok(directory):
        raise HandoffVerificationError(f"the folder does not hold exactly {MANIFEST_MEMBER} and {MODEL_MEMBER}")
    model_bytes = (directory / MODEL_DIRECTORY / MODEL_FILE).read_bytes()
    _own_jcs(model_bytes, MODEL_MEMBER)
    model_hex = sha256(model_bytes).hexdigest()
    manifest_bytes = (directory / MANIFEST_MEMBER).read_bytes()
    instance = _own_jcs(manifest_bytes, MANIFEST_MEMBER)
    manifest = instance.get("handoff_package_manifest") if isinstance(instance, dict) else None
    if not isinstance(manifest, dict):
        raise HandoffVerificationError(f"{MANIFEST_MEMBER} has no handoff_package_manifest object")
    model_hash = manifest.get("model_hash")
    if not isinstance(model_hash, dict) or model_hash.get("value") != model_hex:
        raise HandoffVerificationError(f"model_hash is not the SHA-256 of {MODEL_MEMBER} ({model_hex})")
    export_hex = export_hash(manifest)
    identity = manifest.get("package_identity")
    package_id = identity.get("handoff_package_id") if isinstance(identity, dict) else None
    sealed = {key: value for key, value in manifest.items() if key != "checksums"}
    sealed["checksums"] = checksum_inventory(model_hash.get("payload_ref"), package_id, model_hex, export_hex)
    expected = {
        "schema_version": HANDOFF_SCHEMA_VERSION,
        "deliverable_id": "DEL-15-01",
        "package_id": "PKG-15",
        "scope_item": "SOW-074",
        "objectives": ["OBJ-017"],
        "handoff_contract_status": CONTRACT_STATUS,
        "handoff_package_manifest": sealed,
    }
    if canonical_json(expected).encode("utf-8") != manifest_bytes:
        raise HandoffVerificationError(
            f"{MANIFEST_MEMBER} is not the contract envelope with the checksum inventory of {MODEL_MEMBER} "
            f"(sha256 {model_hex}) and the manifest's export hash (sha256 {export_hex})"
        )
    return {"model_hash": f"sha256:{model_hex}", "export_hash": f"sha256:{export_hex}"}
