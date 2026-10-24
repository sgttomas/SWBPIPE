"""W-12 S1: the desktop store's model-state record against the schema and the Python service.

DEL-14-01 Design `desktop-states-runs-comparison.md` section 2. The desktop store
(apps/desktop/src-tauri/src/model_states.rs) computes the `model_state_record`
hash with a sorted-compact serializer pinned to
`core/project_persistence/service.py`'s `canonical_json`. These tests check,
from the Python side:

- the committed sorted-compact vectors (non-ASCII text and keys, escapes,
  numbers) are exactly what `service.canonical_json` produces; the Rust test
  `sorted_compact_serializer_matches_the_python_service_vectors` checks the same
  file;
- the committed record witness, written by the Rust save and read path, is a
  valid `ModelStateRecord`, and its record hash is the service's hash of the
  stored record without `hashes`.
"""

from __future__ import annotations

import copy
import hashlib
import json
from pathlib import Path

from jsonschema import Draft202012Validator

from core.project_persistence import service


ROOT = Path(__file__).resolve().parents[1]
SCHEMA = json.loads((ROOT / "schemas" / "model_state.schema.json").read_text(encoding="utf-8"))
VECTORS = json.loads((ROOT / "fixtures" / "persistence" / "sorted_compact_json_vectors.json").read_text(encoding="utf-8"))
WITNESS = json.loads((ROOT / "fixtures" / "persistence" / "model_state_record_witness.json").read_text(encoding="utf-8"))

LABEL_FIELDS = ("state_name", "tags", "notes")


def record_validator() -> Draft202012Validator:
    return Draft202012Validator({"$ref": "#/$defs/ModelStateRecord", "$defs": SCHEMA["$defs"]})


def test_sorted_compact_vectors_are_the_service_serializer() -> None:
    vectors = VECTORS["vectors"]
    assert {vector["name"] for vector in vectors} >= {"non_ascii_text_and_keys", "escapes_and_controls", "numbers"}
    for vector in vectors:
        text = service.canonical_json(vector["value"])
        assert text == vector["sorted_compact_json"], vector["name"]
        assert hashlib.sha256(text.encode("utf-8")).hexdigest() == vector["sha256"], vector["name"]
        assert text.isascii(), "ensure_ascii=True escapes every non-ASCII character"
    non_ascii = next(vector for vector in vectors if vector["name"] == "non_ascii_text_and_keys")
    assert "\\ud83d\\ude00" in non_ascii["sorted_compact_json"], "astral characters are UTF-16 surrogate pairs"


def test_the_emitted_witness_record_validates_against_the_schema() -> None:
    record = WITNESS["emitted_record"]
    errors = sorted(record_validator().iter_errors(record), key=lambda error: list(error.path))
    assert not errors, [f"{list(error.path)}: {error.message}" for error in errors]
    # Populated: every list the mapping fills is non-empty in the witness.
    for member in ("parent_state_refs", "tags", "notes", "unresolved_assumptions", "warnings", "hashes"):
        assert record[member], member
    assert record["state_id"].startswith("state:")
    assert "HUMAN_REVIEW_REQUIRED" in record["analysis_status"]
    assert all(value is False for key, value in record["professional_boundary"].items() if key.startswith("software_makes_"))
    # F-PIP-3: the record claims no review or acceptance standing.
    reviews = [item["provenance"]["review_status"] for key in ("tags", "notes", "unresolved_assumptions", "warnings") for item in record[key]]
    assert set(reviews + [record["provenance"]["review_status"]]) == {"pending"}


def test_the_record_hash_is_the_service_hash_of_the_stored_record_without_hashes() -> None:
    stored = json.loads(WITNESS["stored_record_json"])
    hashes = {entry["payload_scope"]: entry for entry in stored["hashes"]}
    payload = {key: value for key, value in stored.items() if key != "hashes"}
    expected = hashlib.sha256(service.canonical_json(payload).encode("utf-8")).hexdigest()
    assert WITNESS["record_hash"] == expected
    assert hashes["model_state_record"]["value"] == expected
    assert hashes["model_state_record"]["canonicalization"] == service.CANONICALIZATION_LABEL
    assert hashes["model_payload"]["canonicalization"] == "JCS"
    # A non-ASCII vector: the stored record carries text the serializer escapes.
    assert not WITNESS["stored_record_json"].isascii()


def test_labels_are_joined_on_read_and_left_out_of_the_hash() -> None:
    stored = json.loads(WITNESS["stored_record_json"])
    for field in LABEL_FIELDS:
        assert field not in stored, field
    emitted = copy.deepcopy(WITNESS["emitted_record"])
    label_ids = set(WITNESS["label_assumption_ids"])
    assert label_ids and label_ids <= {item["assumption_id"] for item in emitted["unresolved_assumptions"]}
    for field in LABEL_FIELDS:
        del emitted[field]
    emitted["unresolved_assumptions"] = [item for item in emitted["unresolved_assumptions"] if item["assumption_id"] not in label_ids]
    assert emitted == stored
