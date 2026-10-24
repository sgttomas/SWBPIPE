"""W-12 S2: the desktop store's recorded analysis-run revision against the schema and Python.

DEL-14-01 Design `desktop-states-runs-comparison.md` section 3. Record Run in the
desktop store (apps/desktop/src-tauri/src/analysis_runs.rs) writes a new immutable
revision of a solve's transient analysis_run.v0.3 record, tied to a saved model
state. The committed witness `fixtures/persistence/recorded_run_witness.json` was
written by that Rust path from `fixtures/persistence/transient_run_input.json` (a
transient record built by the TypeScript session builder for a real producer
result). These tests check, from the Python side:

- the witness is a valid analysis_run.v0.3 record and Python's mirror
  (`recorded_revision`) rebuilds it exactly, hash included;
- the record validator accepts the transient record in its transient form and the
  recorded revision in its recorded form, bound to the stored state's model hash, and
  refuses each in the other's place and every broken binding (mirroring
  `recordedRunParity.test.ts`).
"""
from __future__ import annotations

from copy import deepcopy
import json
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    CREATED_AT_UNAVAILABLE, analysis_record_projection, recorded_library_refs, recorded_revision, recorded_rule_pack_refs,
    validate_analysis_run_v0_3, verify_analysis_run_record,
)
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1
from schema_validation import validate_instance

ROOT = Path(__file__).resolve().parents[1]
PERSISTENCE = ROOT / "fixtures" / "persistence"
INPUT = json.loads((PERSISTENCE / "transient_run_input.json").read_text(encoding="utf-8"))
WITNESS = json.loads((PERSISTENCE / "recorded_run_witness.json").read_text(encoding="utf-8"))
SOURCE = json.loads((ROOT / INPUT["source_result_ref"]).read_text(encoding="utf-8"))
MANIFEST = json.loads(INPUT["input_manifest_canonical"])
SCHEMA = json.loads((ROOT / "schemas" / "analysis_run.v0.3.schema.json").read_text(encoding="utf-8"))


def basis_refs() -> list[dict[str, str]]:
    """The session's expected basis: the solved model's load cases and combinations
    (`modelLoadBasisRefs`)."""
    model = MANIFEST["model_basis"]["model_payload"]
    return ([{"object_type": "LoadCase", "ref": case["id"]} for case in model["load_cases"]]
            + [{"object_type": "Combination", "ref": combination["id"]} for combination in model.get("combinations", [])])


def binding() -> dict:
    return {"state_id": WITNESS["state_id"], "model_hash": WITNESS["state_model_hash"], "input_manifest": deepcopy(MANIFEST)}


def test_the_witness_is_a_valid_v0_3_record_that_python_rebuilds_exactly() -> None:
    recorded = WITNESS["recorded_record"]
    validate_instance(SCHEMA, recorded, schema_label="analysis_run.v0.3.schema.json", instance_label="recorded revision")
    validate_instance(SCHEMA, INPUT["run_record"], schema_label="analysis_run.v0.3.schema.json", instance_label="transient record")
    rebuilt = recorded_revision(INPUT["run_record"], state_id=WITNESS["state_id"], created_at=WITNESS["created_at"], input_manifest=MANIFEST)
    assert rebuilt == recorded
    assert verify_analysis_run_record(recorded) == "match"
    claim = next(row for row in recorded["analysis_run"]["hashes"] if row["payload_scope"] == "analysis_run_record")
    assert claim["value"] == WITNESS["run_record_hash"] == canonical_sha256_checked_v1(analysis_record_projection(recorded))
    assert WITNESS["recording_id"] == f"runrec:{WITNESS['run_record_hash'][:32]}"
    run = recorded["analysis_run"]
    assert run["created_at"] == WITNESS["created_at"]
    assert run["model_state_ref"] == {"object_type": "ModelState", "ref": WITNESS["state_id"]}
    assert CREATED_AT_UNAVAILABLE in INPUT["run_record"]["analysis_run"]["reproducibility"]["determinism_notes"]
    assert CREATED_AT_UNAVAILABLE not in run["reproducibility"]["determinism_notes"]
    # The received-result checksum is the transient record's and the producer's bytes'.
    received = [row for row in run["hashes"] if row["payload_scope"] == "received_result"]
    assert received == [row for row in INPUT["run_record"]["analysis_run"]["hashes"] if row["payload_scope"] == "received_result"]
    assert received[0]["value"] == canonical_sha256_checked_v1(SOURCE)
    assert WITNESS["state_model_hash"] == f"sha256:{canonical_sha256_checked_v1(MANIFEST['model_basis']['model_payload'])}"


def test_the_validator_accepts_both_forms_and_refuses_each_in_the_others_place() -> None:
    transient, recorded = INPUT["run_record"], WITNESS["recorded_record"]
    validate_analysis_run_v0_3(transient, SOURCE, expected_basis_refs=basis_refs())
    validate_analysis_run_v0_3(recorded, SOURCE, expected_basis_refs=basis_refs(), recorded_state=binding())
    with pytest.raises(ValueError, match="^ANALYSIS_SOURCE_MODEL_MISMATCH$"):
        validate_analysis_run_v0_3(recorded, SOURCE, expected_basis_refs=basis_refs())
    with pytest.raises(ValueError, match="^ANALYSIS_SOURCE_MODEL_MISMATCH$"):
        validate_analysis_run_v0_3(transient, SOURCE, expected_basis_refs=basis_refs(), recorded_state=binding())


@pytest.mark.parametrize(("change", "code"), [
    (lambda state: state.update(state_id="state:other"), "ANALYSIS_SOURCE_MODEL_MISMATCH"),
    (lambda state: state.update(model_hash="sha256:" + "0" * 64), "ANALYSIS_RECORDED_MODEL_STATE_HASH_MISMATCH"),
    (lambda state: state["input_manifest"]["solver_basis"].update(solver_mode="dense_scrutiny"), "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH"),
])
def test_a_broken_binding_is_refused(change, code) -> None:
    state = binding()
    change(state)
    with pytest.raises(ValueError, match=f"^{code}$"):
        validate_analysis_run_v0_3(WITNESS["recorded_record"], SOURCE, expected_basis_refs=basis_refs(), recorded_state=state)


def test_an_untimed_or_re_referenced_revision_is_refused() -> None:
    untimed = deepcopy(WITNESS["recorded_record"])
    untimed["analysis_run"]["created_at"] = None
    with pytest.raises(ValueError, match="^ANALYSIS_RECORDED_REVISION_TIME_MISSING$"):
        validate_analysis_run_v0_3(untimed, SOURCE, expected_basis_refs=basis_refs(), recorded_state=binding())
    edited = deepcopy(WITNESS["recorded_record"])
    edited["analysis_run"]["rule_pack_refs"] = [{"object_type": "RulePack", "ref": "rule-pack:x@1"}]
    edited["analysis_run"]["hashes"][0]["value"] = canonical_sha256_checked_v1(analysis_record_projection(edited))
    with pytest.raises(ValueError, match="^ANALYSIS_RECORDED_REFERENCES_MISMATCH$"):
        validate_analysis_run_v0_3(edited, SOURCE, expected_basis_refs=basis_refs(), recorded_state=binding())


def test_rule_pack_and_library_refs_follow_the_manifest() -> None:
    # The same vectors as the Rust and TypeScript tests.
    assert recorded_rule_pack_refs({"active_rule_packs": [
        {"rule_pack_id": "rule-pack:b", "version": "2", "checksum_sha256": "b", "source_notice": "n"},
        {"rule_pack_id": "rule-pack:a", "version": "1", "checksum_sha256": "a", "source_notice": "n"}]}) == [
        {"object_type": "RulePack", "ref": "rule-pack:b@2"}, {"object_type": "RulePack", "ref": "rule-pack:a@1"}]
    with pytest.raises(ValueError, match="^ANALYSIS-RUN-MANIFEST-INVALID"):
        recorded_rule_pack_refs({})
    payload = {
        "materials": [
            {"id": "material:a", "provenance_record": {"library_ref": {"library_kind": "material", "library_id": "invented-lib", "library_version": "sha256:aa", "record_id": "steel"}}},
            {"id": "material:b", "provenance_record": {"library_ref": {"library_kind": "material", "library_id": "invented-lib", "library_version": "sha256:aa", "record_id": "steel"}}},
        ],
        "components": [{"id": "component:c", "nested": {"provenance_record": {"library_ref": {"library_kind": "component", "library_id": "valves"}}}}],
        "library_ref": {"library_kind": "material", "library_id": "not-under-a-provenance-record"},
    }
    assert recorded_library_refs(payload) == [
        {"object_type": "Library", "ref": "library:component/valves"},
        {"object_type": "Library", "ref": "library:material/invented-lib@sha256:aa#steel"}]
    astral = {"a": {"provenance_record": {"library_ref": {"library_kind": "k", "library_id": "\U0001F600"}}},
              "b": {"provenance_record": {"library_ref": {"library_kind": "k", "library_id": "ｚ"}}}}
    assert [ref["ref"] for ref in recorded_library_refs(astral)] == ["library:k/ｚ", "library:k/\U0001F600"]
