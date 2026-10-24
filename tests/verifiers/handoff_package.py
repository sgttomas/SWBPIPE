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
uses the standard library and its sibling `canonical_json` (its unchecked
text, W-6a's serializer, pinned to the Rust producer by
`fixtures/handoff/canonical_text_vectors.json`) and calls nothing the producer
links.

Semantic binding (review round 1, finding 3), the same rules as the desktop's
`manifest_binding_findings`: every checksum's `payload_ref` names the subject
it hashes, by type and id; the model member's project is the manifest's model;
the package ids, `entity_ids`, `unresolved_assumptions` and the library and
rule-pack reference identities are the ones derived from the model member; the
basis state references agree.
"""
from __future__ import annotations

from hashlib import sha256
import json
from pathlib import Path
from typing import Any

from .canonical_json import canonical_json_unchecked as canonical_json, parse_json

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
        value = parse_json(text)
    except (UnicodeDecodeError, ValueError) as error:
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


# --- The binding rules (twins of handoff_package.rs) --------------------------

ID_FIRST = set("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
ID_REST = ID_FIRST | set("0123456789_.:-")
# Rust `char::is_whitespace` (Unicode White_Space): what `str::trim` removes.
RUST_WHITESPACE = "\t\n\x0b\x0c\r \x85\xa0\u1680\u2000\u2001\u2002\u2003\u2004\u2005\u2006\u2007\u2008\u2009\u200a\u2028\u2029\u202f\u205f\u3000"
UNIT_SYSTEM_REF = "unit-system:dec-018-si-dual-display"


def _ref(object_type: str, ref: str) -> dict[str, str]:
    return {"object_type": object_type, "ref": ref}


def _checksum(object_type: str, ref: str, scope: str, canonicalization: str, value: str) -> dict[str, Any]:
    return {"algorithm": "sha256", "canonicalization": canonicalization, "payload_ref": _ref(object_type, ref), "payload_scope": scope, "value": value}


def _is_id(text: Any) -> bool:
    return isinstance(text, str) and bool(text) and text[0] in ID_FIRST and all(character in ID_REST for character in text[1:])


def _members(document: Any, member: str) -> list[tuple[int, dict[str, Any]]]:
    items = document.get(member) if isinstance(document, dict) else None
    if not isinstance(items, list):
        return []
    return [(index, item) for index, item in enumerate(items) if isinstance(item, dict)]


def package_token(project_id: str, model_hex: str) -> str:
    identity = project_id[len("project:"):] if project_id.startswith("project:") else project_id
    mapped = "".join(c if (c.isascii() and c.isalnum()) or c in "._-" else "-" for c in identity)[:96].strip("._-")
    return f"{mapped or 'model'}:{model_hex[:12]}"


def entity_ids(document: Any, project_id: str, state_id: str) -> dict[str, Any] | None:
    """`entity_ids` as the producer derives them; None when an id is not a handoff Id."""
    def slot(families: tuple[str, ...]) -> list[str] | None:
        ids = set()
        for family in families:
            for _, item in _members(document, family):
                identity = item.get("id")
                if not isinstance(identity, str):
                    continue
                if not _is_id(identity):
                    return None
                ids.add(identity)
        return sorted(ids)
    slots = {name: slot(families) for name, families in (
        ("node_ids", ("nodes",)), ("component_ids", ("components",)), ("element_ids", ("pipe_segments",)),
        ("load_case_ids", ("load_cases", "combinations")))}
    if any(value is None for value in slots.values()):
        return None
    mapped = set()
    for family, key in (("supports", "id"), ("materials", "id"), ("sections", "id"), ("lines", "id"),
                        ("constraints", "constraint_id"), ("equipment_interfaces", "id"), ("reference_configurations", "id")):
        for _, item in _members(document, family):
            if isinstance(item.get(key), str):
                mapped.add(f"{project_id}#{item[key]}")
    return {**slots, "model_state_ids": [state_id], "station_ids": [], "result_ids": [], "analysis_run_ids": [],
            "mapping_id_refs": [_ref("Model", entry) for entry in sorted(mapped)]}


def unresolved_assumptions(document: Any, project_id: str) -> list[dict[str, Any]] | None:
    records = _members(document, "unresolved_assumptions")
    ids = {record.get("assumption_id") for _, record in records if isinstance(record.get("assumption_id"), str)}
    found = []
    for _, record in records:
        if record.get("status") != "unresolved":
            continue
        identity = record.get("assumption_id") if isinstance(record.get("assumption_id"), str) else ""
        if not _is_id(identity):
            return None
        statement = record.get("statement") if isinstance(record.get("statement"), str) else ""
        if record.get("kind") == "open_issue":
            statement = f"Open issue: {statement}"
        refs = record.get("affected_refs")
        affected = []
        for entity in refs if isinstance(refs, list) else []:
            if not isinstance(entity, str):
                continue
            affected.append(_ref("Project", project_id) if entity == project_id else _ref("Assumption", entity) if entity in ids
                            else _ref("Model", f"{project_id}#{entity}"))
        found.append({
            "assumption_id": identity, "statement": statement, "status": "unresolved",
            "affected_refs": affected or [_ref("Project", project_id)],
            "provenance": {
                "source_name": "SWBPIPE model document unresolved_assumptions",
                "source_location": f"model document unresolved_assumptions/{identity}",
                "source_license": "user_private_project_data", "contributor": "local user",
                "contributor_attestation": "user-entered project data; no professional claim",
                "redistribution_status": "private_only", "review_classification": "unreviewed",
                "privacy_classification": "private_project_data"},
        })
    return found


def _identity(value: Any) -> str | None:
    if not isinstance(value, str) or not value:
        return None
    trimmed = value.strip(RUST_WHITESPACE)
    return trimmed or None


def library_identities(document: Any) -> list[tuple[str, str, str | None]]:
    """Distinct (kind, id, recorded version) of every `provenance_record.library_ref`, sorted as the producer sorts them."""
    found = set()

    def walk(value: Any) -> None:
        if isinstance(value, list):
            for item in value:
                walk(item)
        elif isinstance(value, dict):
            for key, child in value.items():
                if key == "provenance_record" and isinstance(child, dict) and isinstance(child.get("library_ref"), dict):
                    reference = child["library_ref"]
                    kind, identity = _identity(reference.get("library_kind")), _identity(reference.get("library_id"))
                    if kind and identity:
                        version = reference.get("library_version")
                        found.add((kind, identity, version if isinstance(version, str) and version else None))
                walk(child)
    walk(document)
    return sorted(found, key=lambda item: (item[0], item[1], (0, "") if item[2] is None else (1, item[2])))


def binding_findings(manifest: dict[str, Any], model: Any, model_hex: str) -> list[str]:
    findings: list[str] = []

    def expect(what: str, found: Any, wanted: Any) -> None:
        if found != wanted:
            findings.append(f"{what} is {json.dumps(found)}, not {json.dumps(wanted)}")

    project = model.get("project") if isinstance(model, dict) else None
    project_id = project.get("id") if isinstance(project, dict) else None
    if not isinstance(project_id, str) or not project_id:
        return ["the model member has no project id"]
    basis = manifest.get("model_basis") if isinstance(manifest.get("model_basis"), dict) else {}
    model_checksum = _checksum("Model", project_id, "model_payload", "JCS", model_hex)
    expect("model_basis.model_ref", basis.get("model_ref"), _ref("Model", project_id))
    expect("model_hash", manifest.get("model_hash"), model_checksum)
    expect("model_basis.hash_refs", basis.get("hash_refs"), [model_checksum])
    identity = manifest.get("package_identity") if isinstance(manifest.get("package_identity"), dict) else {}
    token = package_token(project_id, model_hex)
    expect("package_identity.handoff_package_id", identity.get("handoff_package_id"), f"handoff:{token}")
    expect("package_identity.manifest_id", identity.get("manifest_id"), f"manifest:{token}")
    basis_ref = basis.get("basis_state_ref")
    state_id = basis_ref.get("ref") if isinstance(basis_ref, dict) and basis_ref.get("object_type") == "ModelState" and len(basis_ref) == 2 else None
    if not isinstance(state_id, str):
        findings.append("model_basis.basis_state_ref is not a ModelState reference")
        state_id = ""
    states = manifest.get("model_state_refs") if isinstance(manifest.get("model_state_refs"), list) else []
    if len(states) != 1:
        findings.append(f"model_state_refs holds {len(states)} references, not the one basis state")
    if states and isinstance(states[0], dict):
        expect("model_state_refs[0].model_state_ref", states[0].get("model_state_ref"), _ref("ModelState", state_id))
        hashes = states[0].get("hash_refs") if isinstance(states[0].get("hash_refs"), list) else []
        if len(hashes) != 2:
            findings.append(f"model_state_refs[0].hash_refs holds {len(hashes)} checksums, not the model's and the state record's")
        expect("model_state_refs[0].hash_refs[0]", hashes[0] if hashes else None, model_checksum)
        if len(hashes) > 1 and isinstance(hashes[1], dict):
            expect("model_state_refs[0].hash_refs[1].payload_ref", hashes[1].get("payload_ref"), _ref("ModelState", state_id))
            expect("model_state_refs[0].hash_refs[1].payload_scope", hashes[1].get("payload_scope"), "model_state_record")
    derived = entity_ids(model, project_id, state_id)
    if derived is None:
        findings.append("the model member's entity ids are not handoff identifiers")
    else:
        expect("entity_ids", manifest.get("entity_ids"), derived)
    assumptions = unresolved_assumptions(model, project_id)
    if assumptions is None:
        findings.append("the model member's assumption ids are not handoff identifiers")
    else:
        expect("unresolved_assumptions", manifest.get("unresolved_assumptions"), assumptions)
    units = manifest.get("units_manifest")
    if isinstance(units, dict):
        block = {key: value for key, value in units.items() if key != "hash_refs"}
        system = units.get("unit_system_ref", {}).get("ref", "") if isinstance(units.get("unit_system_ref"), dict) else ""
        expect("units_manifest.hash_refs", units.get("hash_refs"),
               [_checksum("UnitSystem", system, "units_manifest", "JCS", sha256(canonical_json(block).encode("utf-8")).hexdigest())])
        expect("units_manifest.unit_system_ref", units.get("unit_system_ref"), _ref("UnitSystem", UNIT_SYSTEM_REF))
    else:
        findings.append("units_manifest is not an object")
    distinct = library_identities(model)
    libraries = manifest.get("library_refs") if isinstance(manifest.get("library_refs"), list) else []
    if len(libraries) != len(distinct):
        findings.append(f"library_refs holds {len(libraries)} references; the model member names {len(distinct)}")
    for index, ((kind, identity_id, version), entry) in enumerate(zip(distinct, libraries)):
        entry = entry if isinstance(entry, dict) else {}
        checksum = entry.get("checksum") if isinstance(entry.get("checksum"), dict) else {}
        expect(f"library_refs[{index}].library_ref", entry.get("library_ref"), _ref("Library", identity_id))
        expect(f"library_refs[{index}].library_kind", entry.get("library_kind"), kind if kind in ("material", "section", "component") else "TBD")
        expect(f"library_refs[{index}].version", entry.get("version"), version or "TBD")
        expect(f"library_refs[{index}].checksum.payload_ref", checksum.get("payload_ref"), _ref("Library", identity_id))
        expect(f"library_refs[{index}].checksum.payload_scope", checksum.get("payload_scope"), "library_reference_metadata")
    documented = []
    for _, entry in _members(model, "rule_pack_refs"):
        identity_id = entry.get("rule_pack_id") if isinstance(entry.get("rule_pack_id"), str) else ""
        version = entry.get("rule_pack_version") if isinstance(entry.get("rule_pack_version"), str) else ""
        documented.append((identity_id, version if version.strip(RUST_WHITESPACE) else "TBD"))
    packs = manifest.get("rule_pack_refs") if isinstance(manifest.get("rule_pack_refs"), list) else []
    if len(packs) != len(documented):
        findings.append(f"rule_pack_refs holds {len(packs)} references; the model member names {len(documented)}")
    for index, ((identity_id, version), entry) in enumerate(zip(documented, packs)):
        entry = entry if isinstance(entry, dict) else {}
        checksum = entry.get("checksum") if isinstance(entry.get("checksum"), dict) else {}
        expect(f"rule_pack_refs[{index}].rule_pack_id", entry.get("rule_pack_id"), identity_id)
        expect(f"rule_pack_refs[{index}].version", entry.get("version"), version)
        expect(f"rule_pack_refs[{index}].checksum.payload_ref", checksum.get("payload_ref"), _ref("RulePack", identity_id))
        expect(f"rule_pack_refs[{index}].checksum.payload_scope", checksum.get("payload_scope"), "rule_pack_reference_metadata")
    return findings


def verify_handoff_package(directory: Path | str) -> dict[str, str]:
    """Returns `{"model_hash", "export_hash"}` (`sha256:<hex>`) or raises HandoffVerificationError."""
    directory = Path(directory)
    if not _layout_ok(directory):
        raise HandoffVerificationError(f"the folder does not hold exactly {MANIFEST_MEMBER} and {MODEL_MEMBER}")
    model_bytes = (directory / MODEL_DIRECTORY / MODEL_FILE).read_bytes()
    model = _own_jcs(model_bytes, MODEL_MEMBER)
    model_hex = sha256(model_bytes).hexdigest()
    manifest_bytes = (directory / MANIFEST_MEMBER).read_bytes()
    instance = _own_jcs(manifest_bytes, MANIFEST_MEMBER)
    manifest = instance.get("handoff_package_manifest") if isinstance(instance, dict) else None
    if not isinstance(manifest, dict):
        raise HandoffVerificationError(f"{MANIFEST_MEMBER} has no handoff_package_manifest object")
    model_hash = manifest.get("model_hash")
    if not isinstance(model_hash, dict) or model_hash.get("value") != model_hex:
        raise HandoffVerificationError(f"model_hash is not the SHA-256 of {MODEL_MEMBER} ({model_hex})")
    findings = binding_findings(manifest, model, model_hex)
    if findings:
        raise HandoffVerificationError(f"{MANIFEST_MEMBER} does not bind {MODEL_MEMBER}: " + "; ".join(findings))
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
