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
import re
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH, CREATED_AT_UNAVAILABLE, RULE_PACK_REF_CHECKSUM_MISMATCH, RULE_PACK_REF_UNRESOLVED,
    analysis_record_projection, build_analysis_run_v0_3, library_ref_sites, manifest_excluded_rule_pack_identities, manifest_excluded_rule_packs,
    parse_rule_pack_identity, recorded_library_refs, recorded_revision, recorded_rule_pack_refs, rule_pack_identity, rule_pack_ref_diagnostic,
    rule_pack_ref_diagnostic_entries, validate_analysis_run_v0_3, verify_analysis_run_record,
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
        "library_ref": {"library_kind": "material", "library_id": "outside-a-provenance-record"},
    }
    assert recorded_library_refs(payload) == [
        {"object_type": "Library", "ref": "library:component/valves"},
        {"object_type": "Library", "ref": "library:material/invented-lib@sha256:aa#steel"},
        {"object_type": "Library", "ref": "library:material/outside-a-provenance-record"}]
    # Rust's White_Space trim: U+3000, U+0085 and U+2003 go; U+FEFF and U+001F stay.
    spaced = {"a": {"library_ref": {"library_kind": "\u3000k\u0085", "library_id": "\ufeffid\u2003", "record_id": "\u001f"}}}
    assert recorded_library_refs(spaced) == [{"object_type": "Library", "ref": "library:k/\ufeffid#\u001f"}]
    astral = {"a": {"provenance_record": {"library_ref": {"library_kind": "k", "library_id": "\U0001F600"}}},
              "b": {"provenance_record": {"library_ref": {"library_kind": "k", "library_id": "ｚ"}}}}
    assert [ref["ref"] for ref in recorded_library_refs(astral)] == ["library:k/ｚ", "library:k/\U0001F600"]


def test_library_refs_follow_the_shared_site_table() -> None:
    # As the Rust test `library_refs_follow_the_shared_site_table` and the TypeScript
    # parity test: the one walk finds every site of the table and no other, and the
    # recorded refs are the sites with a table identity, normalized as the table is; a
    # null identity is omitted.
    table = json.loads((ROOT / "fixtures" / "model_document" / "library_ref_sites.json").read_text(encoding="utf-8"))
    document = table["document"]
    assert [path for path, _ in library_ref_sites(document)] == [site["path"] for site in table["sites"]]

    def at(path: str) -> dict:
        value = document
        for key in re.sub(r"\[(\d+)\]", r".\1", path).split("."):
            value = value[int(key)] if isinstance(value, list) else value[key]
        return value

    expected = set()
    for site in table["sites"]:
        if site["identity"] is None:
            continue
        kind, library_id, record_id = site["identity"]
        version = at(site["path"]).get("library_version")
        expected.add(f"library:{kind}/{library_id}" + (f"@{version}" if isinstance(version, str) else "") + (f"#{record_id}" if record_id is not None else ""))
    refs = [ref["ref"] for ref in recorded_library_refs(document)]
    assert refs == sorted(expected) == [
        "library:component/complib.invented.sites#comp.inner",
        "library:hanger/hanglib.invented.sites",
        "library:material/matlib.invented.root#mat.root",
        "library:material/matlib.invented.sites#mat.padded",
        "library:material/matlib.invented.sites@sha256:recorded#mat.top",
        "library:section/seclib.invented.sites",
    ]


def _reseal(record: dict) -> dict:
    claim = next(row for row in record["analysis_run"]["hashes"] if row["payload_scope"] == "analysis_run_record")
    claim["value"] = canonical_sha256_checked_v1(analysis_record_projection(record))
    return record


def _set(path: tuple, value):
    def edit(run: dict) -> None:
        target = run
        for key in path[:-1]:
            target = target[key]
        target[path[-1]] = value
    return edit


def _manifest_ref(run: dict) -> None:
    other = {"object_type": "InputManifest", "ref": "input-manifest:other"}
    run["reproducibility"]["input_manifest_refs"] = [other]
    run["reproducibility"]["input_manifest_hashes"][0]["payload_ref"] = dict(other)


# Mirrors MANIFEST_CONTRADICTIONS in apps/desktop/src/services/recordedRunParity.test.ts: each
# edits one member the stored manifest fixes, coherently resealed; the manifest is unchanged.
MANIFEST_CONTRADICTIONS = [
    ("settings ref", _set(("settings_ref",), {"object_type": "SolverSettings", "ref": "solver-settings:other"}), "ANALYSIS_RECORDED_SETTINGS_MISMATCH"),
    ("solver build ref", _set(("solver_version", "build_ref"), {"object_type": "ExternalReference", "ref": "other-solver@9"}), "ANALYSIS_RECORDED_SOLVER_MISMATCH"),
    ("solver version", _set(("solver_version", "solver_version"), "9.9.9"), "ANALYSIS_RECORDED_SOLVER_MISMATCH"),
    ("unit-system ref", _set(("unit_system_ref",), {"object_type": "UnitSystem", "ref": "unit-system:other"}), "ANALYSIS_RECORDED_UNITS_MISMATCH"),
    ("load case absent from the manifest", lambda run: run["load_basis_refs"].append({"object_type": "LoadCase", "ref": "LC-ABSENT"}), "ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH"),
    ("load case replaced", lambda run: run["load_basis_refs"].__setitem__(0, {"object_type": "Combination", "ref": run["load_basis_refs"][0]["ref"]}), "ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH"),
    ("rule-pack ref", _set(("rule_pack_refs",), [{"object_type": "RulePack", "ref": "rule-pack:other@1"}]), "ANALYSIS_RECORDED_REFERENCES_MISMATCH"),
    ("library ref", _set(("library_refs",), [{"object_type": "Library", "ref": "library:material/other"}]), "ANALYSIS_RECORDED_REFERENCES_MISMATCH"),
    ("input manifest ref", _manifest_ref, "ANALYSIS_RECORDED_INPUT_MANIFEST_MISMATCH"),
    ("extra member in a ref", lambda run: run["settings_ref"].__setitem__("label", "x"), "ANALYSIS_RECORDED_SETTINGS_MISMATCH"),
]


@pytest.mark.parametrize(("name", "edit", "code"), MANIFEST_CONTRADICTIONS, ids=[case[0] for case in MANIFEST_CONTRADICTIONS])
def test_a_resealed_contradiction_of_the_stored_manifest_is_refused(name, edit, code) -> None:
    record = deepcopy(WITNESS["recorded_record"])
    edit(record["analysis_run"])
    _reseal(record)
    assert verify_analysis_run_record(record) == "match", name
    with pytest.raises(ValueError, match=f"^{code}$"):
        validate_analysis_run_v0_3(record, SOURCE, recorded_state=binding())


def test_a_callers_basis_may_not_stand_in_for_the_manifests() -> None:
    validate_analysis_run_v0_3(WITNESS["recorded_record"], SOURCE, recorded_state=binding())
    with pytest.raises(ValueError, match="^ANALYSIS_RECORDED_LOAD_BASIS_MISMATCH$"):
        validate_analysis_run_v0_3(WITNESS["recorded_record"], SOURCE, expected_basis_refs=[{"object_type": "LoadCase", "ref": "LC-ABSENT"}], recorded_state=binding())


def test_references_compare_structurally_not_by_member_order() -> None:
    # Mirrors "compares references structurally" in recordedRunParity.test.ts.
    record = deepcopy(WITNESS["recorded_record"])
    run = record["analysis_run"]
    run["model_state_ref"] = {"ref": run["model_state_ref"]["ref"], "object_type": "ModelState"}
    run["settings_ref"] = {"ref": run["settings_ref"]["ref"], "object_type": run["settings_ref"]["object_type"]}
    run["load_basis_refs"] = [{"ref": ref["ref"], "object_type": ref["object_type"]} for ref in run["load_basis_refs"]]
    build = run["solver_version"]["build_ref"]
    run["solver_version"] = {"build_ref": {"ref": build["ref"], "object_type": build["object_type"]},
                             "solver_version": run["solver_version"]["solver_version"], "solver_name": run["solver_version"]["solver_name"]}
    assert verify_analysis_run_record(record) == "match"
    validate_analysis_run_v0_3(record, SOURCE, recorded_state=binding())


# Rule-pack reference diagnostics (FR-CMP-002): the pair with invented model rule_pack_refs,
# whose transient record (TypeScript-built) carries a diagnostic per excluded reference after
# the producer's, and the witness the Rust Record Run path emits from it. Mirrors the
# rule-pack cases of `recordedRunParity.test.ts` and `modelRulePackResolution.test.tsx`.
RP_INPUT = json.loads((PERSISTENCE / "transient_run_input_rule_packs.json").read_text(encoding="utf-8"))
RP_WITNESS = json.loads((PERSISTENCE / "recorded_run_witness_rule_packs.json").read_text(encoding="utf-8"))
RP_MANIFEST = json.loads(RP_INPUT["input_manifest_canonical"])


def rp_binding() -> dict:
    return {"state_id": RP_WITNESS["state_id"], "model_hash": RP_WITNESS["state_model_hash"], "input_manifest": deepcopy(RP_MANIFEST)}


def rp_basis() -> list[dict[str, str]]:
    model = RP_MANIFEST["model_basis"]["model_payload"]
    return ([{"object_type": "LoadCase", "ref": case["id"]} for case in model["load_cases"]]
            + [{"object_type": "Combination", "ref": combination["id"]} for combination in model.get("combinations", [])])


def test_the_rule_pack_witness_keeps_the_transient_diagnostics_and_python_rebuilds_it() -> None:
    recorded = RP_WITNESS["recorded_record"]
    validate_instance(SCHEMA, recorded, schema_label="analysis_run.v0.3.schema.json", instance_label="recorded revision with rule-pack diagnostics")
    validate_instance(SCHEMA, RP_INPUT["run_record"], schema_label="analysis_run.v0.3.schema.json", instance_label="transient record with rule-pack diagnostics")
    rebuilt = recorded_revision(RP_INPUT["run_record"], state_id=RP_WITNESS["state_id"], created_at=RP_WITNESS["created_at"], input_manifest=RP_MANIFEST)
    assert rebuilt == recorded
    assert verify_analysis_run_record(recorded) == "match"
    run = recorded["analysis_run"]
    assert run["diagnostics"] == RP_INPUT["run_record"]["analysis_run"]["diagnostics"]
    producer = [{"source_annotation": item} for item in SOURCE["diagnostics"]]
    assert run["diagnostics"][:len(producer)] == producer
    # Python gives the same bytes for the same code and identity (the TypeScript builder wrote them).
    # The two unresolved references would share `invented-pack@branch@1.0.0` flattened.
    assert run["diagnostics"][len(producer):] == [
        {"source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, rule_pack_identity("invented-pack@branch", "1.0.0"))},
        {"source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, rule_pack_identity("invented-pack", "branch@1.0.0"))},
        {"source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_CHECKSUM_MISMATCH, rule_pack_identity("invented-pack-changed", "2.0%"))},
    ]
    assert manifest_excluded_rule_pack_identities(RP_MANIFEST) == ["invented-pack%40branch@1.0.0", "invented-pack@branch%401.0.0", "invented-pack-changed@2.0%25"]
    assert manifest_excluded_rule_packs(RP_MANIFEST) == [("invented-pack@branch", "1.0.0"), ("invented-pack", "branch@1.0.0"), ("invented-pack-changed", "2.0%")]
    assert run["rule_pack_refs"] == [{"object_type": "RulePack", "ref": "invented-pack%40held@1.0.0"}]
    assert set(RP_MANIFEST) == set(MANIFEST), "the manifest gains no member"


def test_the_validator_accepts_rule_pack_diagnostics_and_refuses_a_recorded_revision_that_does_not_name_the_exclusions() -> None:
    validate_analysis_run_v0_3(RP_INPUT["run_record"], SOURCE, expected_basis_refs=rp_basis())
    validate_analysis_run_v0_3(RP_WITNESS["recorded_record"], SOURCE, expected_basis_refs=rp_basis(), recorded_state=rp_binding())
    dropped = deepcopy(RP_WITNESS["recorded_record"])
    dropped["analysis_run"]["diagnostics"].pop()
    _reseal(dropped)
    assert verify_analysis_run_record(dropped) == "match"
    with pytest.raises(ValueError, match=f"^{ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH}$"):
        validate_analysis_run_v0_3(dropped, SOURCE, expected_basis_refs=rp_basis(), recorded_state=rp_binding())
    swapped = deepcopy(RP_WITNESS["recorded_record"])
    swapped["analysis_run"]["diagnostics"].append(swapped["analysis_run"]["diagnostics"].pop(-2))
    _reseal(swapped)
    with pytest.raises(ValueError, match=f"^{ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH}$"):
        validate_analysis_run_v0_3(swapped, SOURCE, expected_basis_refs=rp_basis(), recorded_state=rp_binding())


@pytest.mark.parametrize("edit", [
    lambda annotation: annotation.update(message="Rule pack invented-pack-missing@1.0.0 was applied."),
    lambda annotation: annotation.update(code="MODEL-RULE-PACK-REF-INVENTED"),
    lambda annotation: annotation.update(note="extra"),
    lambda annotation: annotation.update(severity="info"),
], ids=["reworded", "recoded", "extra member", "severity"])
def test_a_rule_pack_diagnostic_not_exactly_its_code_and_identitys_is_refused(edit) -> None:
    record = deepcopy(RP_INPUT["run_record"])
    edit(record["analysis_run"]["diagnostics"][len(SOURCE["diagnostics"])]["source_annotation"])
    _reseal(record)
    with pytest.raises(ValueError, match="^ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH$"):
        validate_analysis_run_v0_3(record, SOURCE, expected_basis_refs=rp_basis())


def test_a_long_identity_is_bounded_in_the_message_and_whole_in_affected_refs() -> None:
    # The same vector as `modelRulePackResolution.test.tsx`.
    identity = "x" * 200 + "@1.0.0"
    annotation = rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, identity)
    assert annotation["affected_refs"] == [identity]
    assert "x" * 160 + "\u2026" in annotation["message"] and "x" * 161 not in annotation["message"]
    assert rule_pack_ref_diagnostic_entries([{"source_annotation": annotation}]) == [(RULE_PACK_REF_UNRESOLVED, identity)]


def test_the_python_builder_appends_the_exclusions_and_refuses_ones_the_manifest_does_not_leave_out() -> None:
    manifest_ref = RP_INPUT["run_record"]["analysis_run"]["reproducibility"]["input_manifest_refs"][0]
    manifest_hash = RP_INPUT["run_record"]["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["value"]
    exclusions = [
        {"rule_pack_id": "invented-pack@branch", "rule_pack_version": "1.0.0", "code": RULE_PACK_REF_UNRESOLVED},
        {"rule_pack_id": "invented-pack", "rule_pack_version": "branch@1.0.0", "code": RULE_PACK_REF_UNRESOLVED},
        {"rule_pack_id": "invented-pack-changed", "rule_pack_version": "2.0%", "code": RULE_PACK_REF_CHECKSUM_MISMATCH},
    ]
    record = build_analysis_run_v0_3(SOURCE, expected_basis_refs=rp_basis(), input_manifest_ref=manifest_ref, input_manifest_hash=manifest_hash,
                                     input_manifest=RP_MANIFEST, rule_pack_ref_exclusions=exclusions)
    assert record["analysis_run"]["diagnostics"] == RP_INPUT["run_record"]["analysis_run"]["diagnostics"]
    with pytest.raises(ValueError, match=f"^{ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH}$"):
        build_analysis_run_v0_3(SOURCE, expected_basis_refs=rp_basis(), input_manifest_ref=manifest_ref, input_manifest_hash=manifest_hash,
                                input_manifest=RP_MANIFEST, rule_pack_ref_exclusions=exclusions[:1])


def test_rule_pack_identities_percent_encode_each_component() -> None:
    # The same vectors as the Rust test `rule_pack_identities_percent_encode_each_component` and
    # `modelRulePackResolution.test.tsx`.
    assert rule_pack_identity("rule-pack:b", "2") == "rule-pack:b@2"
    assert rule_pack_identity("invented-pack@branch", "1.0.0") == "invented-pack%40branch@1.0.0"
    assert rule_pack_identity("invented-pack", "branch@1.0.0") == "invented-pack@branch%401.0.0"
    assert rule_pack_identity("pack%40x@y", "1%@2") == "pack%2540x%40y@1%25%402"
    for pair in [("pack%40x@y", "1%@2"), ("invented-pack@branch", "1.0.0"), ("invented-pack", "branch@1.0.0"), ("%", "@"), ("a", "")]:
        assert parse_rule_pack_identity(rule_pack_identity(*pair)) == pair
    for text in ["a@b@c", "a", "a%2Fb@1", "a%4@1", "a%40@1%"]:
        assert parse_rule_pack_identity(text) is None
    assert recorded_rule_pack_refs({"active_rule_packs": [{"rule_pack_id": "invented-pack@branch", "version": "1.0.0", "checksum_sha256": "a", "source_notice": "n"}]}) == [
        {"object_type": "RulePack", "ref": "invented-pack%40branch@1.0.0"}]


def test_a_repeated_exclusion_identity_is_refused_even_when_the_model_repeats_the_pair() -> None:
    # As the Rust tests `a_repeated_exclusion_identity_is_refused_at_record_run_and_at_read` and
    # `diagnostics_difference_refuses_a_repeat_even_when_the_model_repeats_the_pair`, and the
    # TypeScript `modelRulePackResolution.test.tsx`: no identity twice, whatever the model names.
    identity = rule_pack_identity("invented-pack@branch", "1.0.0")
    one = {"source_annotation": rule_pack_ref_diagnostic(RULE_PACK_REF_UNRESOLVED, identity)}
    assert rule_pack_ref_diagnostic_entries([one, deepcopy(one)]) is None
    manifest = deepcopy(RP_MANIFEST)
    refs = manifest["model_basis"]["model_payload"]["rule_pack_refs"]
    refs.append(deepcopy(refs[1]))
    assert manifest_excluded_rule_pack_identities(manifest).count(identity) == 2
    manifest_ref = RP_INPUT["run_record"]["analysis_run"]["reproducibility"]["input_manifest_refs"][0]
    manifest_hash = RP_INPUT["run_record"]["analysis_run"]["reproducibility"]["input_manifest_hashes"][0]["value"]
    exclusions = [
        {"rule_pack_id": "invented-pack@branch", "rule_pack_version": "1.0.0", "code": RULE_PACK_REF_UNRESOLVED},
        {"rule_pack_id": "invented-pack", "rule_pack_version": "branch@1.0.0", "code": RULE_PACK_REF_UNRESOLVED},
        {"rule_pack_id": "invented-pack-changed", "rule_pack_version": "2.0%", "code": RULE_PACK_REF_CHECKSUM_MISMATCH},
    ]
    with pytest.raises(ValueError, match="^ANALYSIS_SOURCE_DIAGNOSTICS_MISMATCH$"):
        build_analysis_run_v0_3(SOURCE, expected_basis_refs=rp_basis(), input_manifest_ref=manifest_ref, input_manifest_hash=manifest_hash,
                                input_manifest=manifest, rule_pack_ref_exclusions=exclusions + exclusions[:1])
    with pytest.raises(ValueError, match=f"^{ANALYSIS_RULE_PACK_REF_DIAGNOSTICS_MISMATCH}$"):
        build_analysis_run_v0_3(SOURCE, expected_basis_refs=rp_basis(), input_manifest_ref=manifest_ref, input_manifest_hash=manifest_hash,
                                input_manifest=manifest, rule_pack_ref_exclusions=exclusions)


@pytest.mark.parametrize(("edit", "detail"), [
    (lambda manifest: manifest.pop("active_rule_packs"), "no active_rule_packs list"),
    (lambda manifest: manifest["model_basis"]["model_payload"].update(rule_pack_refs=None), "is not a list"),
    (lambda manifest: manifest["model_basis"]["model_payload"]["rule_pack_refs"].append({"rule_pack_id": "x"}), "has no id or version"),
], ids=["no active list", "references not a list", "a reference without a version"])
def test_unreadable_rule_pack_references_are_refused_as_rust_refuses_them(edit, detail) -> None:
    manifest = deepcopy(RP_MANIFEST)
    edit(manifest)
    with pytest.raises(ValueError, match=f"^ANALYSIS-RUN-MANIFEST-INVALID: .*{detail}"):
        manifest_excluded_rule_packs(manifest)


def test_a_model_without_references_excludes_nothing_whatever_the_manifest_holds() -> None:
    # As Rust's `manifest_excluded_rule_packs` and the TypeScript `manifestExcludedRulePacks`.
    assert manifest_excluded_rule_packs({"model_basis": {"model_payload": {}}}) == []
    assert manifest_excluded_rule_packs({"model_basis": {"model_payload": {"rule_pack_refs": []}}}) == []
