"""RT-A4 §10.4 (DEL-05-02 Design absolute-sum-srss-combinations.md, "Recorded results from before RT-A4 (historical
read)"): runs recorded before RT-A4 keep main's read, through a scoped historical path only.

RT-A4 restated the preview-physics combination policy and limitation 6, so the preview-physics tables' digests moved;
a preview-physics statement carries its table's limitations and the reader requires them to equal the table's. The
fixtures (fixtures/persistence/recorded_pre_rt_a4_pf, README there) are main's records at 6c3cb46d3 verbatim where
main has one (preview-physics-2), and records built by main's own builder from main's verbatim statements where it has
none (preview-physics-timoshenko-1, preview-physics-retained-1, physics-retained-1; and RT-A3 #101's physics-retained-1
at 01e05a486). Every check runs the product validator and the Rust reader through `openpipestress-runner read`.

At a4ff570dc (before this path) every one of these records was refused (SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS, or
ANALYSIS_SEMANTIC_CONTRACT_MISMATCH for physics-retained-1); the current-path assertions below are that refusal, kept.
"""
from __future__ import annotations

import copy
import json
import shutil
from pathlib import Path

import pytest

from core.analysis_runs import compatibility as c
from core.analysis_runs import rust_reader
from core.serialization.canonical_json.adapter import canonical_sha256_checked_v1

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/persistence/recorded_pre_rt_a4_pf"
MAIN = "6c3cb46d3"


def load(path: str | Path) -> dict:
    return json.loads((FIXTURES / path).read_text(encoding="utf-8"))


def statement(document: dict) -> dict:
    return document["source"] if "producer" not in document else document


# (name, record path, statement path, recorded digest, refusal on the current path)
VARIANTS = [
    ("preview-physics-2", "preview_physics_2/transient_run_input.json#run_record", "preview_physics_2/preview_physics_connected_sparse.json",
     "1ccc9561cb4ce59d047f3dab7927a927c44a707efb3acb90e7b07a21b3bad0a2", "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"),
    ("preview-physics-timoshenko-1", "preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.analysis_run.json",
     "preview_physics_timoshenko_1/preview_physics_timoshenko_straight_sparse.json",
     "a7b83c37a4af9bf70f56513f80883003a2a3860387319311ea2e967dd0cf764b", "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"),
    ("preview-physics-retained-1", "preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.analysis_run.json",
     "preview_physics_retained_1/retained_precision_milestone_successor_sparse_interactive.json",
     "b001070e5fedeeb101522b22ce0d61bb7cf85f82e86b8d851713f8d9fc8c4c46", "SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"),
    ("physics-retained-1 (main)", "physics_retained_1/main/retained_precision_exact_successor_sparse_interactive.analysis_run.json",
     "physics_retained_1/main/retained_precision_exact_successor_sparse_interactive.json",
     "cf29fd5fc5d61de0f1133e9668b05f3d62ae8f696944fe04de7470029f5357dd", "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"),
    ("physics-retained-1 (#101)", "physics_retained_1/pr101/retained_precision_exact_successor_sparse_interactive.analysis_run.json",
     "physics_retained_1/pr101/retained_precision_exact_successor_sparse_interactive.json",
     "045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b", "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"),
    ("physics-retained-1 (main, RT-A4)", "physics_retained_1/main_bbf384dec/retained_precision_exact_successor_sparse_interactive.analysis_run.json",
     "physics_retained_1/main_bbf384dec/retained_precision_exact_successor_sparse_interactive.json",
     "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034", "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"),
    ("physics-retained-1 (main, before the combination certificate)", "physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.analysis_run.json",
     "physics_retained_1/main_252bef969/retained_precision_exact_successor_sparse_interactive.json",
     "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691", "ANALYSIS_SEMANTIC_CONTRACT_MISMATCH"),
]
IDS = [v[0] for v in VARIANTS]


def record_of(path: str) -> dict:
    file, _, member = path.partition("#")
    document = load(file)
    return document[member] if member else document


def pair(variant):
    _, record_path, source_path, digest, refusal = variant
    return record_of(record_path), statement(load(source_path)), digest, refusal


def validate(record: dict, source: dict, **kwargs) -> None:
    c.validate_analysis_run_v0_3(record, source, expected_basis_refs=record["analysis_run"]["load_basis_refs"], **kwargs)


def reseal(record: dict) -> dict:
    """The record's own checksum recomputed: a coherent edit the checksum alone cannot see."""
    claim = next(row for row in record["analysis_run"]["hashes"] if row["payload_scope"] == "analysis_run_record")
    claim["value"] = canonical_sha256_checked_v1(c.analysis_record_projection(record))
    return record


def rebind(record: dict, sha256: str, *, rows: bool = True) -> dict:
    record = copy.deepcopy(record)
    record["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"] = sha256
    if rows:
        for ref in record["analysis_run"]["result_refs"]:
            ref["semantic_contract"]["sha256"] = sha256
    return reseal(record)


def resource(source: dict, record: dict) -> dict:
    """The record's received-result checksum recomputed for an edited statement."""
    record = copy.deepcopy(record)
    claim = next(row for row in record["analysis_run"]["hashes"] if row["payload_scope"] == "received_result")
    claim["value"] = canonical_sha256_checked_v1(source)
    return reseal(record)


@pytest.mark.parametrize("variant", VARIANTS, ids=IDS)
def test_a_run_recorded_before_rt_a4_reads_as_main_read_it_on_the_recorded_read_only(variant):
    record, source, digest, refusal = pair(variant)
    before_record, before_source = copy.deepcopy(record), copy.deepcopy(source)
    binding = record["analysis_run"]["reproducibility"]["semantic_contract"]
    assert binding == {"id": source["producer"]["semantic_contract_id"], "sha256": digest}
    assert digest != c._TABLES[binding["id"]][0], "a variant is never the current table"
    assert c.verify_analysis_run_record(record) == "match"
    validate(record, source, recorded_read=True)
    # The current path (a4ff570dc's only path) still refuses it.
    with pytest.raises(ValueError, match=refusal):
        validate(record, source)
    # Reading changes nothing: no migration, no reseal.
    assert record == before_record and source == before_source


@pytest.mark.parametrize("variant", VARIANTS, ids=IDS)
def test_a_statement_recorded_before_rt_a4_is_refused_on_every_current_admission_path(variant):
    record, source, digest, _ = pair(variant)
    binding = {"id": source["producer"]["semantic_contract_id"], "sha256": digest}
    current = c._TABLES[binding["id"]][0]
    manifest = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "input-manifest:current"}, "input_manifest_hash": "1" * 64}
    if binding["id"] == c.PHYSICS_RETAINED_CONTRACT_ID:
        # The exact successor's statements read under the current reader (rows and receipt bindings unchanged), so a
        # current record of them binds the current table; the variant is a recorded binding only.
        assert c.build_analysis_run(source, **manifest)["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": binding["id"], "sha256": current}
    else:
        with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"):
            c.build_analysis_run(source, **manifest)
        assert c.numerical_use_standing(source, []) == "unsupported"
        assert c.standing_reason(source) is None
        # The reader without a recorded binding: the header admits, the evidence reader refuses.
        if not c._is_retained(source):
            with pytest.raises(rust_reader.ReaderRefusal, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"):
                rust_reader.validate("preview-physics", source)
        else:
            with pytest.raises(rust_reader.ReaderRefusal, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"):
                rust_reader.retained_precision(source)
    # A historical binding presented to the current record validator is refused.
    with pytest.raises(ValueError):
        validate(record, source)


def test_main_recorded_preview_physics_revisions_read_and_resave_as_main_wrote_them():
    witness = load("preview_physics_2/recorded_run_witness.json")
    transient = load("preview_physics_2/transient_run_input.json")
    source = load("preview_physics_2/preview_physics_connected_sparse.json")
    manifest = json.loads(transient["input_manifest_canonical"])
    state = {"state_id": witness["state_id"], "model_hash": witness["state_model_hash"], "input_manifest": manifest}
    recorded = witness["recorded_record"]
    assert c.verify_analysis_run_record(recorded) == "match"
    c.validate_analysis_run_v0_3(recorded, source, recorded_state=state, recorded_read=True)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"):
        c.validate_analysis_run_v0_3(recorded, source, recorded_state=state)
    # Re-save: the recorded revision of main's transient record is main's witness, byte for byte in canonical form:
    # its binding, its received-result checksum and every row binding are kept, never resealed to the current table.
    resaved = c.recorded_revision(transient["run_record"], state_id=witness["state_id"], created_at=witness["created_at"], input_manifest=manifest)
    assert canonical_sha256_checked_v1(resaved) == canonical_sha256_checked_v1(recorded)
    assert resaved["analysis_run"]["reproducibility"]["semantic_contract"]["sha256"].startswith("1ccc9561")
    c.validate_analysis_run_v0_3(resaved, source, recorded_state=state, recorded_read=True)
    # The rule-pack witness, and main's S2 run with its own refusal (not the table's).
    rule_packs = load("preview_physics_2/recorded_run_witness_rule_packs.json")
    rp_manifest = json.loads(load("preview_physics_2/transient_run_input_rule_packs.json")["input_manifest_canonical"])
    c.validate_analysis_run_v0_3(rule_packs["recorded_record"], source, recorded_read=True,
                                 recorded_state={"state_id": rule_packs["state_id"], "model_hash": rule_packs["state_model_hash"], "input_manifest": rp_manifest})
    s2 = load("preview_physics_2/s2_unresolved_rule_pack_run.json")
    s2_state = {"state_id": s2["state_id"], "model_hash": s2["state_model_hash"], "input_manifest": json.loads(s2["input_manifest_canonical"])}
    with pytest.raises(ValueError, match="ANALYSIS-RUN-RULE-PACKS-NOT-RESOLVED"):
        c.validate_analysis_run_v0_3(s2["recorded_record"], source, recorded_state=s2_state, recorded_read=True)
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"):
        c.validate_analysis_run_v0_3(s2["recorded_record"], source, recorded_state=s2_state)


@pytest.mark.parametrize("variant", VARIANTS, ids=IDS)
def test_a_tampered_or_mismatched_variant_is_refused_on_the_recorded_read(variant):
    record, source, digest, _ = pair(variant)
    identity = source["producer"]["semantic_contract_id"]
    current = c._TABLES[identity][0]
    # Wrong or unknown digest: no variant is selected, so the current read refuses.
    for sha in ("0" * 64, digest[:-1] + ("0" if digest[-1] != "0" else "1")):
        with pytest.raises(ValueError):
            validate(rebind(record, sha), source, recorded_read=True)
    # Mixed record and row bindings (a row bound to the current table).
    mixed = copy.deepcopy(record)
    mixed["analysis_run"]["result_refs"][0]["semantic_contract"]["sha256"] = current
    with pytest.raises(ValueError, match="ANALYSIS_ROW_SOURCE_BINDING_MISMATCH"):
        validate(reseal(mixed), source, recorded_read=True)
    # A record claiming the current table over a statement published before RT-A4.
    if identity != c.PHYSICS_RETAINED_CONTRACT_ID:
        with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS"):
            validate(rebind(record, current), source, recorded_read=True)
    # A variant of another identity (a successor under the wrong variant, and the converse).
    for other_id, other_sha in c.RECORDED_PRE_RT_A4_TABLES:
        if other_id != identity:
            wrong = rebind(record, other_sha)
            wrong["analysis_run"]["reproducibility"]["semantic_contract"]["id"] = other_id
            with pytest.raises(ValueError, match="SOURCE_RECORDED_BINDING_IDENTITY|ANALYSIS_ROW_SOURCE_BINDING_MISMATCH"):
                validate(reseal(wrong), source, recorded_read=True)
    # A binding with a third member names no variant.
    extra = copy.deepcopy(record)
    extra["analysis_run"]["reproducibility"]["semantic_contract"]["note"] = "x"
    with pytest.raises(ValueError):
        validate(reseal(extra), source, recorded_read=True)
    # The record's own bytes tampered (its checksum), and the statement's (the received-result checksum).
    tampered = copy.deepcopy(record)
    tampered["analysis_run"]["run_name"] += " "
    with pytest.raises(ValueError, match="ANALYSIS_RECORD_CHECKSUM_MISMATCH"):
        validate(tampered, source, recorded_read=True)
    changed = copy.deepcopy(source)
    changed["model_ref"] += " "
    with pytest.raises(ValueError):
        validate(record, changed, recorded_read=True)
    renamed = copy.deepcopy(source)
    renamed["diagnostics"] = copy.deepcopy(renamed["diagnostics"])
    renamed["status"]["rule_check"] = "USER_RULE_CHECKED" if renamed["status"]["rule_check"] != "USER_RULE_CHECKED" else "USER_RULE_FAILED"
    with pytest.raises(ValueError):
        validate(record, renamed, recorded_read=True)


@pytest.mark.parametrize("variant", VARIANTS[:3], ids=IDS[:3])
def test_a_variant_binding_with_the_current_limitations_is_refused(variant):
    """Old digest, new limitations: a statement restated with the current limitation text (and its record resealed
    over it) does not read under the variant the record names."""
    record, source, digest, _ = pair(variant)
    identity = source["producer"]["semantic_contract_id"]
    table = json.loads(c._TABLES[identity][1].read_text(encoding="utf-8"))
    restated = copy.deepcopy(source)
    restated["formulation_basis"]["limitations"] = table["supported_profile_limitations"]
    with pytest.raises(ValueError, match="SOURCE_PREVIEW_PHYSICS_FORMULATION_BASIS|RECEIPT"):
        validate(resource(restated, record), restated, recorded_read=True)


def test_a_variant_table_whose_bytes_differ_from_its_digest_is_refused(tmp_path, monkeypatch):
    record, source, digest, _ = pair(VARIANTS[0])
    key = (c.PREVIEW_PHYSICS_CONTRACT_ID, digest)
    copied = tmp_path / c.RECORDED_PRE_RT_A4_TABLES[key].name
    shutil.copyfile(c.RECORDED_PRE_RT_A4_TABLES[key], copied)
    copied.write_bytes(copied.read_bytes() + b" ")
    monkeypatch.setitem(c.RECORDED_PRE_RT_A4_TABLES, key, copied)
    with pytest.raises(ValueError, match="SOURCE_RECORDED_TABLE_HASH"):
        validate(record, source, recorded_read=True)


def test_the_registry_is_main_verbatim_and_disjoint_from_the_current_tables():
    import hashlib
    import subprocess
    origin = {c.PREVIEW_PHYSICS_CONTRACT_ID: "preview_physics_2", c.PREVIEW_PHYSICS_TIMOSHENKO_CONTRACT_ID: "preview_physics_timoshenko_1",
              c.PREVIEW_PHYSICS_RETAINED_CONTRACT_ID: "preview_physics_retained_1", c.PHYSICS_RETAINED_CONTRACT_ID: "physics_retained_1"}
    for (identity, digest), path in c.RECORDED_PRE_RT_A4_TABLES.items():
        assert hashlib.sha256(path.read_bytes()).hexdigest() == digest
        assert digest != c._TABLES[identity][0]
        table = json.loads(path.read_text(encoding="utf-8"))
        assert table["semantic_contract_id"] == identity
        # Rows are the current table's: only limitations, policy text, vocabulary and inherited digests moved,
        # and R-6/R-7 S-K1 added one kind (support_reaction_along_direction_v1) without moving any other row.
        current = json.loads(c._TABLES[identity][1].read_text(encoding="utf-8"))["rows"]
        assert table["rows"] == [row for row in current if row["kind"] != "support_reaction_along_direction_v1"]
        # Increment 4's and operating-case slice 1b's variants are main's at bbf384dec (1b's under
        # fixtures/results/recorded_pre_rt_a5_1b).
        rev = {"045f6f0d8ce350246b21fb712d2dd14bf3a7f7f3690e66e062cbbf77d3696a3b": "01e05a486",
               "040ef9a613607cb6cb4aab6f34646481efd81e060681c364b72df4e129c8c034": "bbf384dec",
               "e7b93f416f700f8efdb31bf1ba26dedde52963d768d6ead5a73dbf1c645a360c": "bbf384dec",
               "ce49ebe07bc05431ea1f770c1933b5bf58f4b1c8b66fb6f36810f2ef7a58cd43": "bbf384dec",
               "26dce9c21261b1ca894af05a3b6f4a94db88904894cb47f7296f892e72ecfb7e": "bbf384dec",
               # RT-A5 minuend_final's variants are main's at b32a17761 (fixtures/results/recorded_pre_rt_a5_mf).
               "f41b9d3ed0c1b7abaee0b98ca1a28aa1a2d109bce443b98d53a2080c1c504ce9": "b32a17761",
               "6f6494907e3ead979ea27e5ff85f54617589c9195dc84d40a02752a0b006ef44": "b32a17761",
               "b4377e73e8a83edb57b4f0c41dbaef384385e0f6b4810103a106a334c8edbe14": "b32a17761",
               # RT-A3 §4's variant is main's exact successor table at 252bef969.
               "85c2678a78e794b44761c11efb73c215645c7e2560d802ac8ad6075e7804c691": "252bef969",
               # R-6/R-7 S-K1's variants are main's at 9e5c69797, before S-K1 (fixtures/results/recorded_pre_sk1).
               "563db3c7c44279face3adc60a76a512de38520357e89ea4b49ad11d20946db89": "9e5c69797",
               "f64473f5a60ac2bb34d8006b40bf044800f0c97454c570bbe88d1f1d96b7bdfc": "9e5c69797",
               "d0bab3502b4018ce0596757a933bd6306f402dc3741a10af24a094e8c66544e8": "9e5c69797"}.get(digest, MAIN)
        try:
            committed = subprocess.run(["git", "show", f"{rev}:fixtures/results/semantic_contract_v0_3_{origin[identity]}.json"],
                                       cwd=ROOT, capture_output=True, check=True).stdout
        except (subprocess.CalledProcessError, FileNotFoundError):
            continue  # a shallow checkout without that revision: the digest above still binds the bytes
        assert committed == path.read_bytes()


@pytest.mark.parametrize("variant", VARIANTS[:2], ids=IDS[:2])
def test_the_independent_verifier_reads_the_recorded_statement_under_its_variant_limitations_only(variant):
    """The Python verifier (tests/verifiers, the independent check of the Rust preview-physics reader) agrees: main's
    statement satisfies every evidence check under the variant's limitation array, and fails only the limitation
    check under the current one. The verifier mirrors the current reader; the variant is substituted here only."""
    from unittest.mock import patch
    from tests.verifiers import preview_physics_evidence as verifier
    _, source, digest, _ = pair(variant)
    identity = source["producer"]["semantic_contract_id"]
    with pytest.raises(ValueError, match="formulation profile or limitations"):
        verifier.validate_preview_physics_evidence(source)
    table = json.loads(c.RECORDED_PRE_RT_A4_TABLES[(identity, digest)].read_text(encoding="utf-8"))
    profile = verifier.IDENTITIES[identity][0]
    with patch.dict(verifier.IDENTITIES, {identity: (profile, list(table["supported_profile_limitations"]))}):
        verifier.validate_preview_physics_evidence(source)
