#!/usr/bin/env python3
"""Parity of the desktop's analysis-run comparison with the reference engine.

W-12 S3b (DEL-14-01 Design ``desktop-states-runs-comparison.md`` section 4.2):
``fixtures/comparison/analysis_run_comparison_golden.json`` holds runs as the
desktop flattened them (committed producer results and invented edits of one),
with the desktop's comparison of each pair under an open standing gate. It is
written by ``apps/desktop/src/features/comparison/runComparisonParity.test.ts``.
This test runs ``derive_exact_result_id_mappings`` and ``compare_analysis_runs``
on the same rows through an adapter and requires, on the shared projection:

- the same mapped rows, apart from rows the desktop leaves unmapped because only
  its extra identity fields (the five row metadata members) differ, which the
  reference maps;
- for each pair the desktop compares, the reference's delta, with equal raw
  magnitudes and raw ``right - left``;
- for each pair the desktop does not compare, the reference's refusal of it
  (``ARC-RESULT-FAMILY-UNSUPPORTED`` for ``family_not_compared``,
  ``ARC-UNIT-CONVERSION-UNSUPPORTED`` for ``units_differ``) and no delta;
- the same settings deltas over the runs' contexts.

The adapter maps a desktop row to a ``quantity_result``: ``result_id``,
``family``, ``object_ref`` (as an ``Entity`` reference), ``basis_ref``,
``dimension``, ``unit`` and ``magnitude`` (the row's value). The standing gate
and the desktop's extra identity fields are tested in the desktop's vitest.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.comparison.analysis_run.engine import (  # noqa: E402
    SUPPORTED_RESULT_FAMILIES,
    compare_analysis_runs,
    derive_exact_result_id_mappings,
)


GOLDEN = ROOT / "fixtures" / "comparison" / "analysis_run_comparison_golden.json"
METADATA_FIELDS = {"component", "location", "basis", "coordinate_system", "sign_convention"}
REFUSAL_OF_REASON = {
    "family_not_compared": "ARC-RESULT-FAMILY-UNSUPPORTED",
    "units_differ": "ARC-UNIT-CONVERSION-UNSUPPORTED",
}


def _golden() -> dict:
    return json.loads(GOLDEN.read_text(encoding="utf-8"))


def _quantity(row: dict) -> dict:
    identity = row["identity"]
    return {
        "result_id": identity["result_id"],
        "family": identity["family"],
        "object_ref": {"object_type": "Entity", "ref": identity["object_ref"]},
        "basis_ref": identity["basis_ref"],
        "dimension": identity["dimension"],
        "unit": row["unit"],
        "magnitude": row["value"],
    }


def _envelope(name: str, run: dict) -> dict:
    return {
        "result_envelope": {
            "envelope_id": f"envelope:{name}",
            "result_sets": [{"result_set_id": f"set:{name}", "quantity_results": [_quantity(row) for row in run["rows"]]}],
        }
    }


def _reference(golden: dict, pair: dict):
    left, right = golden["runs"][pair["left"]], golden["runs"][pair["right"]]
    left_results, right_results = _envelope("left", left), _envelope("right", right)
    mappings = derive_exact_result_id_mappings(left_results, right_results)
    comparison = compare_analysis_runs(
        left_run={"run_id": "left"},
        right_run={"run_id": "right"},
        left_results=left_results,
        right_results=right_results,
        mappings=mappings,
        left_settings=left["context"],
        right_settings=right["context"],
    )
    return mappings, comparison


def test_the_golden_records_the_reference_families_and_identity():
    golden = _golden()
    assert tuple(golden["supported_result_families"]) == SUPPORTED_RESULT_FAMILIES
    assert set(golden["identity_fields"]) == {"result_id", "family", "object_ref", "basis_ref", "dimension"} | METADATA_FIELDS
    assert [pair["name"] for pair in golden["pairs"]] == ["identical", "sparse_against_dense", "sparse_against_edited"]


def test_the_desktop_maps_the_rows_the_reference_maps_apart_from_its_metadata_fields():
    golden = _golden()
    for pair in golden["pairs"]:
        mappings, _ = _reference(golden, pair)
        desktop = pair["desktop"]
        reference_mapped = {mapping["left_ref"]["ref"] for mapping in mappings}
        assert all(mapping["left_ref"]["ref"] == mapping["right_ref"]["ref"] for mapping in mappings)
        desktop_mapped = {entry["result_id"] for entry in desktop["pairs"]}
        metadata_only = {
            entry["result_id"]
            for entry in desktop["unmatched"]
            if entry["reason"] == "identity_differs" and entry["differing_fields"] and set(entry["differing_fields"]) <= METADATA_FIELDS
        }
        assert desktop_mapped.isdisjoint(metadata_only), pair["name"]
        assert reference_mapped == desktop_mapped | metadata_only, pair["name"]
        # Every other unmatched row is unmapped by the reference too.
        others = {entry["result_id"] for entry in desktop["unmatched"]} - metadata_only
        assert others.isdisjoint(reference_mapped), pair["name"]
    dense = next(pair for pair in golden["pairs"] if pair["name"] == "sparse_against_dense")
    assert {entry["result_id"] for entry in dense["desktop"]["unmatched"]} == {"result:solver-mode:linear-solve-basis"}


def test_the_desktop_deltas_and_refusals_are_the_reference_engines():
    golden = _golden()
    compared_somewhere = refused_somewhere = 0
    for pair in golden["pairs"]:
        _, comparison = _reference(golden, pair)
        deltas = {delta.left_result_id: delta for delta in comparison.result_deltas}
        refusals: dict[str, set[str]] = {}
        for diagnostic in comparison.diagnostics:
            for ref in diagnostic.affected_refs:
                if ref["object_type"] == "MappingRecord":
                    refusals.setdefault(ref["ref"], set()).add(diagnostic.code)
        metadata_only = {
            entry["result_id"]
            for entry in pair["desktop"]["unmatched"]
            if entry["reason"] == "identity_differs" and set(entry["differing_fields"]) <= METADATA_FIELDS
        }
        for entry in pair["desktop"]["pairs"]:
            result_id = entry["result_id"]
            if entry["status"] == "compared":
                compared_somewhere += 1
                delta = deltas[result_id]
                assert delta.right_result_id == result_id
                assert (delta.left_magnitude, delta.right_magnitude) == (entry["left"], entry["right"]), result_id
                assert delta.raw_delta == entry["delta"], result_id
                assert delta.normalized_unit == entry["unit"], result_id
                assert delta.classification == "not_classified", result_id
                assert f"AUTO-{result_id}" not in refusals, result_id
            else:
                refused_somewhere += 1
                assert result_id not in deltas, result_id
                assert REFUSAL_OF_REASON[entry["reasons"][0]] in refusals[f"AUTO-{result_id}"], result_id
        # The reference compares no other row, apart from the rows mapped on its narrower identity.
        assert set(deltas) - {entry["result_id"] for entry in pair["desktop"]["pairs"]} <= metadata_only, pair["name"]
    assert compared_somewhere and refused_somewhere


def test_the_desktop_context_deltas_are_the_reference_settings_deltas():
    golden = _golden()
    for pair in golden["pairs"]:
        _, comparison = _reference(golden, pair)
        reference = [
            {"key": delta.setting_key, "left": delta.left_value, "right": delta.right_value, "kind": delta.delta_kind}
            for delta in comparison.settings_deltas
        ]
        assert reference == pair["desktop"]["context_deltas"], pair["name"]
    edited = next(pair for pair in golden["pairs"] if pair["name"] == "sparse_against_edited")
    assert [delta["key"] for delta in edited["desktop"]["context_deltas"]] == [
        "load_case:case:invented-added",
        "settings.invented_added_setting",
        "settings.sparse_evidence_lane",
    ]
