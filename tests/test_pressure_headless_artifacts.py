"""T4-U2 phase 2: consumers of the actual headless pressure-1 export artifacts.

The Rust lane ``core/runner/headless/tests/pressure_export.rs`` writes each
case's raw producer envelope and canonical export document when
``HEADLESS_PRESSURE_OUTPUT_DIR`` is set. These tests read them: the document
validates against the results 0.3 schema on the pressure-1 branch only; the
Python reader, transport check and standing admit the raw envelope; and the
AnalysisRun built from it is verified and schema valid. They skip only when
the variable is unset and must be run with it set. All inputs are invented.
"""
from __future__ import annotations

import json
import os
from pathlib import Path

import pytest

from core.analysis_runs.compatibility import (
    PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256, _source_contract, build_analysis_run_v0_3,
    numerical_use_standing, validate_analysis_run_v0_3, verify_analysis_run_record,
)
from schema_validation import validate_instance

PROJECT = Path(__file__).resolve().parents[1]
STEMS = ["arc-u2-l-anch-ptw-k2-0-3-0", "arc-u2-l-anch-ptw-k2-0-4-0", "arc-u2-l-kink-anch-p-k2-0-3-0",
         "arc-l-line-with-connector-0-3-0", "arc-replaced-span-only-0-3-0",
         "connector-u3-sys-demo-connector-001", "connector-u3-sys-demo-connector-002-lr1"]
NAMES = [f"{stem}-{mode}" for stem in STEMS for mode in ("sparse_interactive", "dense_scrutiny")]


def _folder() -> Path:
    folder = os.environ.get("HEADLESS_PRESSURE_OUTPUT_DIR")
    if not folder:
        pytest.skip("set HEADLESS_PRESSURE_OUTPUT_DIR and run core/runner/headless tests/pressure_export.rs first")
    return Path(folder)


def _schema(name):
    return json.loads((PROJECT / "schemas" / name).read_text(encoding="utf-8"))


def _branches(instance):
    document = _schema("results.v0.3.schema.yaml")
    found = []
    for index, branch in enumerate(document["$defs"]["ResultEnvelope"]["oneOf"]):
        alone = {"$schema": document["$schema"], "$id": document["$id"], "$defs": document["$defs"], **branch}
        try:
            validate_instance(alone, instance["result_envelope"], instance_label=f"branch {index}")
        except AssertionError:
            continue
        found.append(branch["properties"]["producer"]["properties"]["semantic_contract_id"]["const"])
    return found


@pytest.mark.parametrize("name", NAMES)
def test_actual_pressure_1_document_selects_its_branch_and_analysis_run_matches(name):
    folder = _folder()
    raw = json.loads((folder / f"{name}.raw.json").read_text())
    document = json.loads((folder / f"{name}.document.json").read_text())
    validate_instance(_schema("results.v0.3.schema.yaml"), document, instance_label=f"actual headless {name}")
    assert _branches(document) == [PRESSURE_CONTRACT_ID]
    envelope = document["result_envelope"]
    for key in ("producer", "numerical_quality", "formulation_basis", "contract_evidence"):
        assert envelope[key] == raw[key], key
    assert _source_contract(raw)[:2] == (PRESSURE_CONTRACT_ID, PRESSURE_CONTRACT_SHA256)
    assert _source_contract(raw, check_receipt=False)[0] == PRESSURE_CONTRACT_ID
    bases = [case["basis_ref"] for case in raw["numerical_quality"]["cases"]]
    assert numerical_use_standing(raw, bases) == "numerically_eligible"
    record = build_analysis_run_v0_3(raw, input_manifest_ref={"object_type": "InputManifest", "ref": f"manifest:{name}"}, input_manifest_hash="1" * 64)
    validate_analysis_run_v0_3(record, raw)
    assert verify_analysis_run_record(record) == "match"
    validate_instance(_schema("analysis_run.v0.3.schema.json"), record, instance_label=f"AnalysisRun {name}")
    assert record["analysis_run"]["reproducibility"]["semantic_contract"] == {"id": PRESSURE_CONTRACT_ID, "sha256": PRESSURE_CONTRACT_SHA256}


def test_artifact_folder_holds_exactly_the_expected_cases():
    folder = _folder()
    assert {p.name[: -len(".raw.json")] for p in folder.glob("*.raw.json")} == set(NAMES)
    assert {p.name[: -len(".document.json")] for p in folder.glob("*.document.json")} == set(NAMES)
