"""RT-A5 minuend_final (DEL-05-02 Work operating-case.md section 7.6, recorded runs): the change restated the
preview-physics combination policy (v3 -> v4) and added the held_status_operand basis, so the preview-physics-2 table's
digest moved (f41b9d3e -> 563db3c7) while its rows and limitations did not. Main's records at b32a17761
(fixtures/persistence/recorded_pre_rt_a5_mf_pf, verbatim) bind f41b9d3e; they read on the recorded read under that
registry variant, and the current record validator, which binds the current table only, refuses the historical binding.
Reading changes no byte. The checks run the product validator and the Rust reader through ``openpipestress-runner read``."""
from __future__ import annotations

import copy
import json
from pathlib import Path

import pytest

from core.analysis_runs import compatibility as c

ROOT = Path(__file__).resolve().parents[1]
FIXTURES = ROOT / "fixtures/persistence/recorded_pre_rt_a5_mf_pf/preview_physics_2"
DIGEST = "f41b9d3ed0c1b7abaee0b98ca1a28aa1a2d109bce443b98d53a2080c1c504ce9"
RECORDS = ["transient_run_input.json#run_record", "transient_run_input_rule_packs.json#run_record"]


def load(name: str) -> dict:
    return json.loads((FIXTURES / name).read_text(encoding="utf-8"))


def record_of(path: str) -> dict:
    file, _, member = path.partition("#")
    document = load(file)
    return document[member] if member else document


def validate(record: dict, source: dict, **kwargs) -> None:
    c.validate_analysis_run_v0_3(record, source, expected_basis_refs=record["analysis_run"]["load_basis_refs"], **kwargs)


def test_the_registry_holds_mains_tables_verbatim():
    for (contract, digest), path in c.RECORDED_PRE_RT_A4_TABLES.items():
        assert c.recorded_pre_rt_a4_variant({"id": contract, "sha256": digest})[2] == path
        assert digest != c._TABLES[contract][0], "a variant is never the current table"
    assert (c.PREVIEW_PHYSICS_CONTRACT_ID, DIGEST) in c.RECORDED_PRE_RT_A4_TABLES
    assert c.PREVIEW_PHYSICS_CONTRACT_SHA256 == "563db3c7c44279face3adc60a76a512de38520357e89ea4b49ad11d20946db89"


@pytest.mark.parametrize("path", RECORDS)
def test_a_run_main_recorded_before_minuend_final_reads_on_the_recorded_read_only(path):
    record, source = record_of(path), load("preview_physics_connected_sparse.json")
    before = (copy.deepcopy(record), copy.deepcopy(source))
    binding = record["analysis_run"]["reproducibility"]["semantic_contract"]
    assert binding == {"id": c.PREVIEW_PHYSICS_CONTRACT_ID, "sha256": DIGEST}
    assert c.verify_analysis_run_record(record) == "match"
    validate(record, source, recorded_read=True)
    with pytest.raises(ValueError):
        validate(record, source)
    assert (record, source) == before, "reading changes no byte"


def test_mains_statement_still_builds_a_current_record_under_the_new_table():
    # Its rows and limitations are the current table's, so a current record of it binds the new digest.
    source = load("preview_physics_connected_sparse.json")
    manifest = {"input_manifest_ref": {"object_type": "InputManifest", "ref": "input-manifest:current"}, "input_manifest_hash": "1" * 64}
    built = c.build_analysis_run(source, **manifest)
    assert built["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": c.PREVIEW_PHYSICS_CONTRACT_ID, "sha256": c.PREVIEW_PHYSICS_CONTRACT_SHA256}
