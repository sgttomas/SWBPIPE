#!/usr/bin/env python3
"""Focused tests for the DEL-17-05 correlation pilot: comparison record, harness configuration and hand calculations.

Every number here is synthetic or invented; nothing is CAEPIPE output.
"""

from __future__ import annotations

import dataclasses
import hashlib
import json
import math
import os
from pathlib import Path
import re
import shutil
import signal
import subprocess
import sys
import threading
import time

import pytest
from types import SimpleNamespace
from typing import Callable

# Reads its directory scans stand for, for CI selection (tools/ci/python_test_reads.py). Its one project scan; `out.iterdir()` reads the solved case's tmp_path outputs.
CI_READS = ("validation/caepipe_correlation/cases/*.json",)


ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from core.handoff.caepipe_external import (  # noqa: E402
    HarnessConfig,
    HarnessSkip,
    build_comparison_record,
    case_paths,
    resolve_harness_config,
    run_caepipe_mbf_export,
    run_caepipe_round_trip,
)
from core.handoff.caepipe_external.correlation import (  # noqa: E402
    ALTERNATIVE_CHECK,
    evaluate_hypothesis_series,
)
from core.handoff.caepipe_external import harness  # noqa: E402
from core.handoff.caepipe_external.harness import (  # noqa: E402
    ENV_VAR,
    ContainmentError,
    SettingsGuard,
    _quote,
    check_no_foreign_instance,
    last_json_object,
    parse_reg_query,
)
from core.handoff.caepipe_mbf import render_caepipe_mbf_si  # noqa: E402
from tools.caepipe_correlation import run_pilot  # noqa: E402
from validation.caepipe_correlation import hand_calcs  # noqa: E402


CASES_DIR = ROOT / "validation" / "caepipe_correlation" / "cases"
PARSER_FIXTURE = ROOT / "fixtures" / "caepipe_external" / "invented" / "print_to_file_results.csv"
SUS = "Sustained (W+P)"
TOL = {
    "analytic_vs_swbpipe": {"rel": 1e-6, "abs": {"m": 1e-9, "rad": 1e-9, "N": 1e-6, "N*m": 1e-6}},
    "external": {"rel": 5e-3, "abs": {"m": 2e-6, "rad": 2e-6, "N": 1.0, "N*m": 1.0}},
    "caepipe_print_resolution": {"rel": 1e-6, "abs": {"m": 5e-7, "rad": 8.7266e-7, "N": 0.5, "N*m": 0.5}},
}


# --------------------------------------------------------------------------
# comparison record (synthetic numbers)
# --------------------------------------------------------------------------
def swb_rows(*, uy_mm=-10.0, rz_rad=-0.01, fy=2000.0, restraint_fy=50.0):
    def row(kind, entity, value, unit, component=None):
        item = {"id": f"r:{kind}:{entity}:{component}", "kind": kind, "entity_ref": entity, "value": value,
                "unit": unit, "basis_ref": {"ref_type": "load_case", "ref_id": "LC1"}}
        if component:
            item["metadata"] = {"component": component}
        return item
    rows = [row("global_nodal_displacement_y", "N1", uy_mm, "mm"), row("global_nodal_rotation_z", "N1", rz_rad, "rad")]
    if fy is not None:
        rows.append(row("support_reaction_component_v2", "A0", fy, "N", "Fy"))
    rows.append(row("support_reaction_component_v2", "G1", restraint_fy, "N", "Fy"))
    return {"mechanics_envelope": {"results": rows}}


MBF = {
    "profile_id": "caepipe-mbf-si-pilot-1",
    "node_map": {"N0": "10", "N1": "20", "N2": "30"},
    "support_map": {"A0": {"node": "10", "caepipe_kind": "anchor"}, "G1": {"node": "30", "caepipe_kind": "restraint"}},
    "load_case_map": {"LC1": SUS},
    "assumptions": ["invented assumption"],
    "loss_report": [{"entity": "x", "code": "MBF-SI-X"}],
}


def parsed(*, uy_mm=-10.0, rz_deg=None, anchor_fy=-2000.0, restraint_fy=-50.0, drop_node=False):
    rz_deg = math.degrees(-0.01) if rz_deg is None else rz_deg
    displacements = {} if drop_node else {"20": {"ux": 0.0, "uy": uy_mm, "uz": 0.0, "rx": 0.0, "ry": 0.0, "rz": rz_deg}}
    return {
        "parser_version": "caepipe-print-csv-1",
        "header": {"product_label": "Invented Product", "version": "0.0-invented"},
        "units": {"length": "mm", "rotation": "deg", "force": "N", "moment": "N*m"},
        "load_cases": {SUS: {
            "displacements": displacements,
            "anchor_loads_global": {"10": {"fx": 0.0, "fy": anchor_fy, "fz": 0.0, "mx": 0.0, "my": 0.0, "mz": 0.0}},
            "restraint_loads_global": {"30": {"fx": 0.0, "fy": restraint_fy, "fz": 0.0}},
        }},
    }


def synthetic_case(**extra):
    case = {
        "case_id": "invented-case",
        "quantities": [
            {"id": "uy", "quantity": "displacement", "node": "N1", "component": "y", "load_case": "LC1"},
            {"id": "rz", "quantity": "rotation", "node": "N1", "component": "z", "load_case": "LC1"},
            {"id": "fy", "quantity": "reaction_force", "support": "A0", "component": "y", "load_case": "LC1"},
            {"id": "gy", "quantity": "reaction_force", "support": "G1", "component": "y", "load_case": "LC1"},
        ],
        "tolerances": json.loads(json.dumps(TOL)),
        "declared_differences": [],
    }
    case.update(extra)
    return case


ANALYTIC = {
    "uy": {"value": -0.010, "unit": "m", "formula": "invented", "basis": "Euler-Bernoulli"},
    "rz": {"value": -0.01, "unit": "rad", "formula": "invented", "basis": "Euler-Bernoulli"},
    "fy": {"value": 2000.0, "unit": "N", "formula": "invented", "basis": "Euler-Bernoulli"},
    "gy": {"value": 50.0, "unit": "N", "formula": "invented", "basis": "Euler-Bernoulli"},
}
MODEL = {"invented": True, "nodes": [{"id": "N1"}]}


def record(case=None, payload=None, caepipe=None, analytic=ANALYTIC, mbf_text="INVENTED\r\n", alternative=None,
           series=None):
    return build_comparison_record(case or synthetic_case(), swbpipe_model=MODEL,
                                   swbpipe_result_payload=payload or swb_rows(), mbf_result=MBF,
                                   mbf_text=mbf_text, parsed_caepipe=caepipe if caepipe is not None else parsed(),
                                   analytic=analytic, alternative_analytic=alternative, hypothesis_series=series)


def quantity(result, qid):
    return next(row for row in result["quantities"] if row["id"] == qid)


def outcomes(row):
    return {name: check["outcome"] for name, check in row["checks"].items()}


def test_all_matched_with_unit_conversion_and_sign_flip():
    result = record()
    for qid in ("uy", "rz", "fy", "gy"):
        row = quantity(result, qid)
        assert set(outcomes(row).values()) == {"matched_within_tolerance"}, qid
        assert row["cause"] is None and row["disposition"] == "no_action"
    assert quantity(result, "uy")["values"]["caepipe"] == pytest.approx(-0.010, rel=1e-15)
    assert quantity(result, "rz")["values"]["caepipe"] == pytest.approx(-0.01, rel=1e-15)  # deg -> rad
    assert quantity(result, "fy")["values"]["caepipe"] == 2000.0  # pipe-on-anchor -2000 flipped
    assert quantity(result, "fy")["value_sources"]["caepipe"]["sign_flip"] is True
    assert quantity(result, "gy")["value_sources"]["caepipe"]["section"] == "restraint_loads_global"
    assert result["summary"]["by_cause"] == {"none": 4}


def test_unflipped_reaction_exceeds_tolerance():
    result = record(caepipe=parsed(anchor_fy=2000.0))
    row = quantity(result, "fy")
    assert outcomes(row) == {"analytic_vs_swbpipe": "matched_within_tolerance",
                             "swbpipe_vs_caepipe": "delta_exceeds_tolerance",
                             "analytic_vs_caepipe": "delta_exceeds_tolerance"}
    assert row["checks"]["swbpipe_vs_caepipe"]["difference"] == -4000.0


def test_triangle_rule_caepipe_side():
    row = quantity(record(caepipe=parsed(uy_mm=-11.0)), "uy")
    assert row["cause"]["label"] == "export_mapping_issue"
    assert row["cause"]["evidence"] == "triangle_rule"
    assert row["cause"]["basis"] == "ASSUMPTION"
    assert row["disposition"] == "requires_professional_review"


def test_triangle_rule_swbpipe_alone():
    row = quantity(record(payload=swb_rows(uy_mm=-11.0)), "uy")
    assert outcomes(row)["analytic_vs_caepipe"] == "matched_within_tolerance"
    assert row["cause"]["label"] == "solver_defect_candidate"
    assert row["disposition"] == "requires_professional_review"


def test_triangle_rule_analytic_alone():
    analytic = dict(ANALYTIC, uy={**ANALYTIC["uy"], "value": -0.011})
    row = quantity(record(analytic=analytic), "uy")
    assert outcomes(row) == {"analytic_vs_swbpipe": "delta_exceeds_tolerance",
                             "swbpipe_vs_caepipe": "matched_within_tolerance",
                             "analytic_vs_caepipe": "delta_exceeds_tolerance"}
    assert row["cause"]["label"] == "missing_model_parameter"


def test_row_unmatched_both_ways():
    row = quantity(record(caepipe=parsed(drop_node=True)), "uy")
    assert outcomes(row)["swbpipe_vs_caepipe"] == "row_unmatched_internal_only"
    assert outcomes(row)["analytic_vs_caepipe"] == "row_unmatched_internal_only"
    assert row["cause"]["label"] == "parser_mapping_issue"
    row = quantity(record(payload=swb_rows(fy=None)), "fy")
    # In each check the first-named source is the internal side of the pair.
    assert outcomes(row)["analytic_vs_swbpipe"] == "row_unmatched_internal_only"
    assert outcomes(row)["swbpipe_vs_caepipe"] == "row_unmatched_external_only"
    assert row["cause"]["label"] == "missing_model_parameter"
    assert row["disposition"] == "requires_professional_review"


def test_tolerance_tbd_and_missing_analytic():
    case = synthetic_case()
    del case["tolerances"]["external"]
    result = record(case=case)
    row = quantity(result, "uy")
    assert outcomes(row)["swbpipe_vs_caepipe"] == "tolerance_tbd"
    assert row["status"] == "no_verdict" and row["cause"] is None
    assert result["tolerance_basis"]["swbpipe_vs_caepipe"]["status"] == "tolerance_tbd"
    analytic = {key: value for key, value in ANALYTIC.items() if key != "rz"}
    row = quantity(record(analytic=analytic), "rz")
    assert outcomes(row) == {"analytic_vs_swbpipe": "not_applicable", "swbpipe_vs_caepipe": "matched_within_tolerance",
                             "analytic_vs_caepipe": "not_applicable"}
    assert row["disposition"] == "no_action"
    row = quantity(record(analytic=analytic, caepipe=parsed(rz_deg=1.0)), "rz")
    assert row["cause"]["label"] == "caepipe_profile_unknown"


def declaration(**changes):
    item = {"id": "dd", "hypothesis": "invented-hypothesis", "applies_to": ["uy"],
            "cause": "documented_behavior_difference", "description": "invented",
            "alternative_analytic": {"basis": "Timoshenko"},
            "source_basis": [{"id": "INVENTED-SOURCE", "url": "https://invented.example/page"}]}
    item.update(changes)
    return item


# Invented alternative analytic: -11.0 mm against SWBPIPE and the Euler-Bernoulli witness at -10.0 mm.
ALTERNATIVE = {"invented-hypothesis": {"uy": {"value": -0.011, "unit": "m", "formula": "invented",
                                              "basis": "Timoshenko", "assumption": "ASSUMPTION: invented"}}}
CONSISTENT = {"invented-hypothesis": {"status": "consistent", "fitted_slope": -2.0}}


def declared_row(*, uy_mm=-11.0, declared=None, alternative=ALTERNATIVE, series=CONSISTENT, case=None, **kwargs):
    case = case or synthetic_case(declared_differences=[declared or declaration()])
    return quantity(record(case=case, caepipe=parsed(uy_mm=uy_mm), alternative=alternative, series=series,
                           **kwargs), "uy")


def missing(row):
    return [(item["condition"], item["state"]) for item in row["cause"]["missing"]]


def test_declared_difference_established_when_all_five_conditions_hold():
    result = record(case=synthetic_case(declared_differences=[declaration()]), caepipe=parsed(uy_mm=-11.0),
                    alternative=ALTERNATIVE, series=CONSISTENT)
    row = quantity(result, "uy")
    assert outcomes(row) == {"analytic_vs_swbpipe": "matched_within_tolerance",
                             "swbpipe_vs_caepipe": "delta_exceeds_tolerance",
                             "analytic_vs_caepipe": "delta_exceeds_tolerance",
                             ALTERNATIVE_CHECK: "matched_within_tolerance"}
    assert row["values"]["alternative"] == -0.011
    assert row["checks"][ALTERNATIVE_CHECK]["tolerance"]["abs"] == 5e-7
    assert row["value_sources"]["alternative"]["basis"] == "Timoshenko"
    assert row["value_sources"]["alternative"]["declared_difference"] == "dd"
    assert row["cause"]["label"] == "documented_behavior_difference"
    assert row["cause"]["evidence_status"] == "established"
    assert row["cause"]["missing"] == [] and row["cause"]["hypothesis"] == "invented-hypothesis"
    assert row["disposition"] == "assumption_recorded"
    # The fourth check is present only on a covered quantity.
    assert ALTERNATIVE_CHECK not in quantity(result, "rz")["checks"]
    assert result["hypothesis_series"]["invented-hypothesis"]["status"] == "consistent"
    assert result["summary"]["by_evidence_status"] == {"established": 1}
    assert result["tolerance_basis"][ALTERNATIVE_CHECK]["abs"]["N"] == 0.5


@pytest.mark.parametrize("label,kwargs,expected", [
    ("alternative exceeds", {"uy_mm": -11.002}, [("alternative_analytic_matched", "failed")]),
    ("no alternative value", {"alternative": {}}, [("alternative_analytic_matched", "unevaluated")]),
    ("no source basis", {"declared": declaration(source_basis=[])}, [("source_basis", "failed")]),
    ("source basis absent", {"declared": {key: value for key, value in declaration().items()
                                          if key != "source_basis"}}, [("source_basis", "failed")]),
    ("series inconsistent", {"series": {"invented-hypothesis": {"status": "inconsistent"}}},
     [("hypothesis_series_consistent", "failed")]),
    ("series not evaluated", {"series": {"invented-hypothesis": {"status": "not_evaluated"}}},
     [("hypothesis_series_consistent", "unevaluated")]),
    ("series absent", {"series": None}, [("hypothesis_series_consistent", "unevaluated")]),
    ("no hypothesis", {"declared": declaration(hypothesis=None), "series": None},
     [("alternative_analytic_matched", "unevaluated"), ("hypothesis_series_consistent", "unevaluated")]),
    ("all three", {"uy_mm": -11.002, "declared": declaration(source_basis=[]), "series": None},
     [("alternative_analytic_matched", "failed"), ("source_basis", "failed"),
      ("hypothesis_series_consistent", "unevaluated")]),
])
def test_declared_difference_stays_a_hypothesis_naming_what_is_missing(label, kwargs, expected):
    row = declared_row(**kwargs)
    assert outcomes(row)["swbpipe_vs_caepipe"] == "delta_exceeds_tolerance", label
    assert row["cause"]["label"] == "documented_behavior_difference", label
    assert row["cause"]["evidence_status"] == "hypothesis", label
    assert missing(row) == expected, label
    assert row["disposition"] == "requires_professional_review", label


def test_print_resolution_tolerance_missing_leaves_the_alternative_check_unevaluated():
    case = synthetic_case(declared_differences=[declaration()])
    del case["tolerances"]["caepipe_print_resolution"]
    row = declared_row(case=case)
    assert outcomes(row)[ALTERNATIVE_CHECK] == "tolerance_tbd"
    assert missing(row) == [("alternative_analytic_matched", "unevaluated")]


def test_declared_difference_never_masks_a_swbpipe_departure_from_the_analytic_witness():
    # All three disagree (analytic -10 mm, SWBPIPE -12 mm, invented prover value -13.05 mm), and the alternative
    # analytic even matches the prover value with a consistent series and a source basis: the triangle rule still
    # names the cause.
    alternative = {"invented-hypothesis": {"uy": {"value": -0.01305, "unit": "m", "basis": "Timoshenko"}}}
    row = quantity(record(case=synthetic_case(declared_differences=[declaration()]), payload=swb_rows(uy_mm=-12.0),
                          caepipe=parsed(uy_mm=-13.05), alternative=alternative, series=CONSISTENT), "uy")
    assert {name: value for name, value in outcomes(row).items() if name != ALTERNATIVE_CHECK} == {
        "analytic_vs_swbpipe": "delta_exceeds_tolerance", "swbpipe_vs_caepipe": "delta_exceeds_tolerance",
        "analytic_vs_caepipe": "delta_exceeds_tolerance"}
    assert outcomes(row)[ALTERNATIVE_CHECK] == "matched_within_tolerance"
    assert row["cause"]["label"] == "solver_defect_candidate"
    assert row["cause"]["evidence"] == "triangle_rule"
    assert "evidence_status" not in row["cause"]
    assert row["cause"]["declared_difference_covers"] == "dd"
    assert "documented_behavior_difference" in row["cause"]["alternatives"]
    assert row["disposition"] == "requires_professional_review"
    # Without an analytic witness a declaration is not enough either.
    analytic = {key: value for key, value in ANALYTIC.items() if key != "uy"}
    row = declared_row(analytic=analytic)
    assert row["cause"]["label"] == "caepipe_profile_unknown"
    assert row["cause"]["declared_difference_covers"] == "dd"


def test_declared_quantity_that_matches_needs_no_cause():
    row = declared_row(uy_mm=-10.0)
    assert row["status"] == "matched" and row["cause"] is None and row["disposition"] == "no_action"
    assert outcomes(row)[ALTERNATIVE_CHECK] == "delta_exceeds_tolerance"  # recorded, not used for a cause


def test_fault_injection_is_classified_as_deliberate_export_issue():
    fault = {"target": "mbf", "description": "invented deliberate fault", "material_modulus_factor": 1.1}
    result = record(case=synthetic_case(fault_injection=fault), caepipe=parsed(uy_mm=-9.0))
    row = quantity(result, "uy")
    assert row["cause"]["label"] == "export_mapping_issue"
    assert row["cause"]["evidence"] == "deliberate_fault_injection"
    assert row["disposition"] == "bug_report"
    assert quantity(result, "fy")["disposition"] == "no_action"
    assert result["manual_changes"]["description"] == "invented deliberate fault"
    # A declaration covering the faulted quantity does not displace the known deliberate cause.
    row = declared_row(uy_mm=-9.0, case=synthetic_case(fault_injection=fault, declared_differences=[declaration()]))
    assert row["cause"]["evidence"] == "deliberate_fault_injection"
    assert row["cause"]["alternatives"] == ["documented_behavior_difference"]
    assert row["cause"]["declared_difference_covers"] == "dd"


# --------------------------------------------------------------------------
# hypothesis series (synthetic numbers)
# --------------------------------------------------------------------------
SERIES_DEFINITION = {
    "hypothesis_id": "invented-hypothesis", "quantity": "q", "expected_exponent": -2.0,
    "expected_relative_gap_sign": 1, "slope_tolerance": {"abs": 0.05, "basis": "ASSUMPTION"},
    "members": [{"case_id": "c-short", "span": 3.0}, {"case_id": "c-mid", "span": 6.0},
                {"case_id": "c-long", "span": 12.0}],
}


def member_record(swb, cae, *, a_s="matched_within_tolerance", digest="h", resolution=5e-7):
    return {"exported_model_hash": digest,
            "tolerance_basis": {ALTERNATIVE_CHECK: {"abs": {"m": resolution}}},
            "quantities": [{"id": "q", "unit": "m", "values": {"swbpipe": swb, "caepipe": cae},
                            "checks": {"analytic_vs_swbpipe": {"outcome": a_s}}}]}


def series_records(exponent, *, sign=1.0, base=0.01, swb=-0.1):
    """Members with relative gap sign * base * (L / 3)^exponent and SWBPIPE value `swb`."""
    out = {}
    for member in SERIES_DEFINITION["members"]:
        relative = sign * base * (member["span"] / 3.0) ** exponent
        out[member["case_id"]] = member_record(swb, swb * (1.0 + relative))
    return out


def test_series_slope_on_synthetic_numbers():
    result = evaluate_hypothesis_series(SERIES_DEFINITION, series_records(-2.0))
    assert result["status"] == "consistent" and result["reasons"] == []
    assert result["fitted_slope"] == pytest.approx(-2.0, abs=1e-9)
    assert result["least_squares_slope"] == pytest.approx(-2.0, abs=1e-9)
    assert [member["relative_gap"] for member in result["members"]] == pytest.approx([0.01, 0.0025, 0.000625])
    # Endpoint slope ln(r12 / r3) / ln(4) with r12 / r3 = 4^-1.97 and 4^-1.9.
    assert evaluate_hypothesis_series(SERIES_DEFINITION, series_records(-1.97))["status"] == "consistent"
    steep = evaluate_hypothesis_series(SERIES_DEFINITION, series_records(-1.9))
    assert steep["status"] == "inconsistent" and steep["fitted_slope"] == pytest.approx(-1.9, abs=1e-9)
    flat = evaluate_hypothesis_series(SERIES_DEFINITION, series_records(0.0))
    assert flat["status"] == "inconsistent" and flat["fitted_slope"] == pytest.approx(0.0, abs=1e-12)
    # Right slope but the invented prover value stiffer than SWBPIPE: shear flexibility cannot do that.
    wrong = evaluate_hypothesis_series(SERIES_DEFINITION, series_records(-2.0, sign=-1.0))
    assert wrong["status"] == "inconsistent" and "unexpected sign" in wrong["reasons"][0]


def test_series_not_evaluated_reasons():
    records = series_records(-2.0)
    for name, change in (
            ("missing", {"c-mid": None}),
            ("witness", {"c-mid": member_record(-0.1, -0.1003, a_s="delta_exceeds_tolerance")}),
            ("unresolved", {"c-long": member_record(-0.1, -0.1 - 4e-7)}),
            ("no value", {"c-short": member_record(-0.1, None)}),
            ("no resolution", {"c-short": member_record(-0.1, -0.101, resolution=None)})):
        result = evaluate_hypothesis_series(SERIES_DEFINITION, {**records, **change})
        assert result["status"] == "not_evaluated" and result["fitted_slope"] is None, name
        assert len(result["reasons"]) == 1, (name, result["reasons"])
    stale = evaluate_hypothesis_series(SERIES_DEFINITION, records, current_mbf_sha256={"c-long": "other"})
    assert stale["status"] == "not_evaluated" and "different MBF" in stale["reasons"][0]


def test_record_carries_prd_22_5_fields():
    result = record()
    for key in ("internal_model_hash", "exported_model_hash", "target_tool", "target_version", "product_label",
                "import_export_assumptions", "manual_changes", "load_case_mapping", "support_mapping",
                "comparison_basis", "tolerance_basis", "quantities", "summary", "discrepancy_disposition",
                "evidence_classification"):
        assert key in result, key
    canonical = json.dumps(MODEL, sort_keys=True, separators=(",", ":"), ensure_ascii=False)
    assert result["internal_model_hash"] == hashlib.sha256(canonical.encode("utf-8")).hexdigest()
    assert result["exported_model_hash"] == hashlib.sha256(b"INVENTED\r\n").hexdigest()
    assert result["target_tool"] == "CAEPIPE" and result["target_version"] == "0.0-invented"
    assert result["manual_changes"] == "none"
    assert result["comparison_basis"]["caepipe_load_cases_compared"] == [SUS]
    assert "not validation" in result["evidence_classification"]
    assert all(entry["basis"] == "ASSUMPTION" for name, entry in result["tolerance_basis"].items()
               if isinstance(entry, dict))


# --------------------------------------------------------------------------
# harness configuration
# --------------------------------------------------------------------------
def git_only_path(tmp_path: Path) -> str:
    """A PATH holding only `git` (so prlctl is absent while the Git work-tree check still runs)."""
    directory = tmp_path / "bin-git-only"
    directory.mkdir()
    git = shutil.which("git")
    assert git, "git is required for this test"
    (directory / "git").symlink_to(git)
    return str(directory)


def exchange(tmp_path: Path, *, ignore="*\n", config=None) -> Path:
    root = tmp_path / "exchange"
    root.mkdir()
    if ignore is not None:
        (root / ".gitignore").write_text(ignore, encoding="utf-8")
    if config is not False:
        (root / "harness_config.json").write_text(json.dumps(config or {
            "vm_name": "invented-vm", "guest_executable": "C:\\Invented\\tool.exe",
            "guest_exchange_root": "Z:\\invented", "guest_work_root": "C:\\invented-work",
            "license_and_environment_responsibility_acknowledged": True,
        }), encoding="utf-8")
    return root


def skip_reason(environ) -> str:
    result = resolve_harness_config(environ)
    assert isinstance(result, HarnessSkip), result
    return result.reason


def test_harness_skips_when_unset_or_missing(tmp_path):
    assert "unset" in skip_reason({})
    assert "unset" in skip_reason({ENV_VAR: "  "})
    assert "does not exist" in skip_reason({ENV_VAR: str(tmp_path / "absent")})
    file_path = tmp_path / "file"
    file_path.write_text("x", encoding="utf-8")
    assert "directory" in skip_reason({ENV_VAR: str(file_path)})


def test_harness_skips_without_exact_gitignore(tmp_path):
    path = git_only_path(tmp_path)
    root = exchange(tmp_path, ignore=None)
    assert ".gitignore" in skip_reason({ENV_VAR: str(root), "PATH": path})
    (root / ".gitignore").write_text("*.csv\n", encoding="utf-8")
    assert ".gitignore" in skip_reason({ENV_VAR: str(root), "PATH": path})


def test_harness_refuses_folder_inside_git_work_tree(tmp_path):
    repo = tmp_path / "repo"
    repo.mkdir()
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    root = repo / "exchange"
    root.mkdir()
    (root / ".gitignore").write_text("*", encoding="utf-8")
    reason = skip_reason({ENV_VAR: str(root), "PATH": git_only_path(tmp_path)})
    assert "inside a Git work tree" in reason


def test_harness_skips_without_config_or_acknowledgement(tmp_path):
    path = git_only_path(tmp_path)
    root = exchange(tmp_path, config=False)
    assert "harness_config.json" in skip_reason({ENV_VAR: str(root), "PATH": path})
    (root / "harness_config.json").write_text(json.dumps({"vm_name": "invented-vm"}), encoding="utf-8")
    assert "license_and_environment_responsibility_acknowledged" in skip_reason({ENV_VAR: str(root), "PATH": path})
    (root / "harness_config.json").write_text(json.dumps({
        "vm_name": "invented-vm", "license_and_environment_responsibility_acknowledged": True}), encoding="utf-8")
    assert "guest_executable" in skip_reason({ENV_VAR: str(root), "PATH": path})


@pytest.mark.parametrize("key,value", [
    ("guest_executable", 'C:\\Invented\\tool.exe" & calc & "'),
    ("guest_executable", "C:\\Invented\\%COMSPEC%\\tool.exe"),
    ("guest_executable", "C:\\Invented\\tool.exe\\"),
    ("guest_executable", "C:\\Invented\\a^b|c<d>e!f.exe"),
    ("guest_exchange_root", "\\\\invented-host\\share"),
    ("guest_exchange_root", "Z:\\invented\nnext"),
    ("guest_exchange_root", "invented\\relative"),
    ("guest_work_root", "C:\\invented-work\\..\\elsewhere"),
    ("guest_work_root", "C:/invented-work"),
])
def test_harness_refuses_guest_paths_outside_the_allow_list(tmp_path, key, value):
    config = {"vm_name": "invented-vm", "guest_executable": "C:\\Invented Tools (x86)\\tool_1.0-a.exe",
              "guest_exchange_root": "Z:\\invented", "guest_work_root": "C:\\invented-work\\",
              "license_and_environment_responsibility_acknowledged": True}
    root = exchange(tmp_path, config=config)
    environ = {ENV_VAR: str(root), "PATH": git_only_path(tmp_path)}
    assert isinstance(resolve_harness_config(environ, require_vm_tools=False), HarnessConfig)
    (root / "harness_config.json").write_text(json.dumps({**config, key: value}), encoding="utf-8")
    result = resolve_harness_config(environ, require_vm_tools=False)
    assert isinstance(result, HarnessSkip) and key in result.reason


def test_guest_command_quoting_refuses_shell_metacharacters():
    assert _quote("C:\\invented-work\\cpc-x\\model*.csv") == '"C:\\invented-work\\cpc-x\\model*.csv"'
    for bad in ('C:\\a" & calc', "C:\\%PATH%", "C:\\a\nb"):
        with pytest.raises(ValueError):
            _quote(bad)


def test_harness_skips_without_prlctl_and_resolves_with_it(tmp_path):
    path = git_only_path(tmp_path)
    root = exchange(tmp_path)
    result = resolve_harness_config({ENV_VAR: str(root), "PATH": path})
    assert isinstance(result, HarnessSkip) and "prlctl" in result.reason
    assert "prlctl on PATH" in result.checked
    offline = resolve_harness_config({ENV_VAR: str(root), "PATH": path}, require_vm_tools=False)
    assert isinstance(offline, HarnessConfig) and offline.prlctl is None
    fake = tmp_path / "bin-fake-prlctl"
    fake.mkdir()
    prlctl = fake / "prlctl"
    prlctl.write_text("#!/bin/sh\nexit 97\n", encoding="utf-8")
    prlctl.chmod(0o755)
    live = resolve_harness_config({ENV_VAR: str(root), "PATH": os.pathsep.join([path, str(fake)])})
    assert isinstance(live, HarnessConfig)
    assert live.prlctl == str(prlctl) and live.vm_name == "invented-vm"


def test_case_paths_layout_and_id_check(tmp_path):
    paths = case_paths(tmp_path, "cpc-99-invented")
    for layer in ("swbpipe", "caepipe_in", "caepipe_out", "parsed", "comparison"):
        assert paths[layer].is_dir()
    assert paths["manifest"] == tmp_path / "cases" / "cpc-99-invented" / "case_manifest.json"
    for bad in ("Upper", "-lead", "a/b", "x" * 64, "", "trailing-newline\n"):
        with pytest.raises(ValueError):
            case_paths(tmp_path, bad)


def test_last_json_object_takes_the_final_top_level_object():
    text = 'noise {not json\n{"a": {"b": 1}}\nmore\n{"status": "x", "gui": {"used": true}}\ntrailing'
    assert last_json_object(text) == {"status": "x", "gui": {"used": True}}
    assert last_json_object("no json here") is None


def fake_config(tmp_path: Path) -> HarnessConfig:
    root = exchange(tmp_path)
    return HarnessConfig(exchange_root=root, vm_name="invented-vm", guest_executable="C:\\Invented\\tool.exe",
                         guest_exchange_root="Z:\\invented", guest_work_root="C:\\invented-work",
                         prlctl="/invented/prlctl", acknowledged=True, trial_first_launch_date="2026-10-26",
                         checked=("invented",))


MBF_TEXT = "INVENTED\r\nMBF\r\n"
MBF_SHA = hashlib.sha256(MBF_TEXT.encode("utf-8")).hexdigest()


def test_round_trip_records_route_and_never_runs_a_real_prlctl(tmp_path):
    config = fake_config(tmp_path)
    calls = []

    def runner(argv, **kwargs):
        calls.append(argv)
        assert argv[:6] == ["/invented/prlctl", "exec", "invented-vm", "--current-user", "cmd", "/c"]
        assert kwargs["timeout"] == 600
        command = argv[6]
        stdout = ""
        if command.startswith("powershell"):
            stdout = 'WARNING: banner\n' + json.dumps({"status": "csv_from_gui_print", "gui": {"used": True},
                                                        "mbf_sha256": MBF_SHA.upper(),
                                                        "csv_path": "C:\\invented-work\\cpc-x\\model_results.csv"})
        if "copy /Y" in command and "caepipe_out" in command:
            (config.exchange_root / "cases" / "cpc-x" / "caepipe_out" / "model_results.csv").write_text("Invented\n")
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    record_ = run_caepipe_round_trip(config, "cpc-x", MBF_TEXT, runner=runner)
    assert [call[6].split(" ")[0] for call in calls] == ["(if", "powershell", "copy"]
    assert '"C:\\invented-work\\cpc-x\\model.mbf"' in calls[0][6].split(" & copy ")[0]  # old guest MBF deleted
    assert record_["guest_mbf_sha256"] == MBF_SHA and record_["input_hash_verified"] is True
    assert "-File \"Z:\\invented\\_harness\\Invoke-CaepipeRoundTrip.ps1\"" in calls[1][6]
    assert record_["route"] == "csv_from_gui_print"
    assert record_["command_shape"]["gui_print_fallback"] == {
        **record_["command_shape"]["gui_print_fallback"], "used": True, "label": "print_to_file_fallback"}
    assert record_["mbf"]["sha256"] == MBF_SHA
    case_dir = config.exchange_root / "cases" / "cpc-x"
    assert (case_dir / "caepipe_in" / "model.mbf").read_bytes() == b"INVENTED\r\nMBF\r\n"
    assert (config.exchange_root / "_harness" / "Invoke-CaepipeRoundTrip.ps1").is_file()
    assert record_["observed_csv_path"]["host"] == "caepipe_out/model_results.csv"
    assert record_["expected_batch_csv_path"] == "C:\\invented-work\\cpc-x\\model.csv"
    assert record_["boundary_note"].startswith("regression/handoff evidence only")
    assert "invented-vm" not in json.dumps(record_)
    stored = json.loads((case_dir / "caepipe_out" / "run_record.json").read_text(encoding="utf-8"))
    assert stored["route"] == "csv_from_gui_print"
    assert not (config.exchange_root / "_harness" / "round_trip.lock").exists()


PRINT_FIXTURE = Path(__file__).resolve().parents[1] / "fixtures" / "caepipe_external" / "invented" / \
    "print_to_file_results.csv"


def _batch_print_round_trip(tmp_path, files: dict[str, bytes], status: str = "csv_from_batch",
                            reported: str | None = "model.txt") -> dict:
    """A round trip whose batch print leaves `files` in the guest work folder (collected into caepipe_out)."""
    tmp_path.mkdir(parents=True, exist_ok=True)
    config = fake_config(tmp_path)

    def runner(argv, **kwargs):
        command = argv[6]
        stdout = ""
        if command.startswith("powershell"):
            stdout = json.dumps({"status": status, "gui": {"used": False}, "mbf_sha256": MBF_SHA}
                                | ({"csv_path": f"C:\\invented-work\\cpc-p\\{reported}"} if reported else {}))
        if command.startswith("copy /Y") and "caepipe_out" in command:
            for name, data in files.items():
                (config.exchange_root / "cases" / "cpc-p" / "caepipe_out" / name).write_bytes(data)
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    return run_caepipe_round_trip(config, "cpc-p", MBF_TEXT, runner=runner)


def test_round_trip_accepts_a_txt_batch_print_only_in_the_print_csv_layout(tmp_path):
    record_ = _batch_print_round_trip(tmp_path / "valid", {"model.txt": PRINT_FIXTURE.read_bytes()})
    assert record_["route"] == "csv_from_batch"
    assert record_["observed_csv_path"]["host"] == "caepipe_out/model.txt"
    assert record_["print_file"] == {"collected": "model.txt", "layout": "print_csv"}

    record_ = _batch_print_round_trip(tmp_path / "invalid", {"model.txt": b"Invented report\r\nplain text, not a print\r\n"})
    assert record_["route"] == "print_file_unrecognised"  # refused by name, never read as results
    assert record_["observed_csv_path"]["host"] is None and record_["print_file"]["collected"] is None
    assert record_["print_file"]["layout"].startswith("not the print-CSV layout")

    # A .csv print is taken as named, as before.
    record_ = _batch_print_round_trip(tmp_path / "csv", {"model.csv": b"Invented\n"}, reported="model.csv")
    assert record_["route"] == "csv_from_batch"
    assert record_["print_file"] == {"collected": "model.csv", "layout": "as_named"}


def test_round_trip_refuses_two_batch_print_files(tmp_path):
    record_ = _batch_print_round_trip(tmp_path, {"model.csv": PRINT_FIXTURE.read_bytes(),
                                                 "model.txt": PRINT_FIXTURE.read_bytes()},
                                      status="print_file_ambiguous", reported=None)
    assert record_["route"] == "print_file_ambiguous" and record_["observed_csv_path"]["host"] is None
    assert record_["print_file"]["collected"] is None
    # The guest script refuses both before reporting either, and never falls back to the GUI then.
    script = harness.GUEST_SCRIPT.read_text(encoding="utf-8")
    both = script.index('$log.batch.csv_present -and $log.batch.txt_present')
    assert both < script.index('if ($log.batch.csv_present) { $log.csv_path')
    assert both < script.index('if ($log.batch.txt_present) { $log.csv_path')
    assert 'print_file_ambiguous"; $log | ConvertTo-Json -Depth 6; exit 5' in script


def test_round_trip_falls_back_to_a_guest_local_script_copy(tmp_path):
    config = fake_config(tmp_path)
    commands = []

    def runner(argv, **kwargs):
        commands.append(argv[6])
        stdout = ""
        if argv[6].startswith("powershell") and "invented-work" in argv[6].split(" -Exe ")[0]:
            stdout = json.dumps({"status": "no_results_after_batch", "gui": {"used": False},
                                 "mbf_sha256": hashlib.sha256(b"X\r\n").hexdigest()})
        return subprocess.CompletedProcess(argv, 0, stdout, "cannot run script" if not stdout else "")

    record_ = run_caepipe_round_trip(config, "cpc-y", "X\r\n", runner=runner)
    assert len(commands) == 5
    assert record_["guest_script"]["run_from"] == "guest_work_dir"
    assert record_["route"] == "no_results_after_batch"
    assert record_["observed_csv_path"]["host"] is None


def test_round_trip_stops_before_solving_when_preparation_fails(tmp_path):
    config = fake_config(tmp_path)
    commands = []

    def runner(argv, **kwargs):
        commands.append(argv[6])
        if argv[6].startswith("powershell"):
            raise AssertionError("solve must not run after a failed prepare step")
        return subprocess.CompletedProcess(argv, 1, "", "The system cannot find the file specified.")

    record_ = run_caepipe_round_trip(config, "cpc-p", MBF_TEXT, runner=runner)
    assert len(commands) == 1 and commands[0].startswith("(if not exist")
    assert record_["route"] == "prepare_failed"
    assert record_["observed_csv_path"]["host"] is None and record_["guest_mbf_sha256"] is None


def test_prepare_steps_bracket_the_folder_check_so_a_rerun_still_replaces_the_guest_copy(tmp_path):
    # In cmd, an unbracketed `if` takes the rest of the line as its body: `if not exist W mkdir W & del ... & copy ...`
    # deletes and copies nothing once W exists, so a re-run would leave the previous guest copy in place.
    (tmp_path / "r").mkdir()
    (tmp_path / "s").mkdir()
    config, out = solved_case(tmp_path / "r", "cpc-r")
    commands: list[str] = []
    run_caepipe_round_trip(config, "cpc-r", MBF_TEXT, runner=lambda argv, **k: commands.append(argv[6]) or
                           subprocess.CompletedProcess(argv, 1, "", ""))
    config, out = solved_case(tmp_path / "s", "cpc-s")
    run_caepipe_mbf_export(config, "cpc-s", MBF_SHA, runner=lambda argv, **k: commands.append(argv[6]) or
                           subprocess.CompletedProcess(argv, 1, "", ""))
    work = {"cpc-r": "C:\\invented-work\\cpc-r", "cpc-s": "C:\\invented-work\\cpc-s\\export"}
    assert commands[0].startswith(f'(if not exist "{work["cpc-r"]}" mkdir "{work["cpc-r"]}") & del /Q ')
    assert commands[1].startswith(f'(if not exist "{work["cpc-s"]}" mkdir "{work["cpc-s"]}") & del /Q ')
    assert all(command.count("(if ") == 1 and " & copy /Y " in command for command in commands)


def test_round_trip_accepts_a_drive_root_exchange_share(tmp_path):
    # A shared folder mapped as a whole drive (`Z:\\`) is held as `Z:` after resolution and must still run.
    config = {"vm_name": "invented-vm", "guest_executable": "C:\\Invented\\tool.exe",
              "guest_exchange_root": "Z:\\", "guest_work_root": "C:\\invented-work",
              "license_and_environment_responsibility_acknowledged": True}
    root = exchange(tmp_path, config=config)
    resolved = resolve_harness_config({ENV_VAR: str(root), "PATH": git_only_path(tmp_path)}, require_vm_tools=False)
    assert isinstance(resolved, HarnessConfig) and resolved.guest_exchange_root == "Z:"
    commands = []

    def runner(argv, **kwargs):
        commands.append(argv[6])
        return subprocess.CompletedProcess(argv, 1, "", "invented failure")

    record_ = run_caepipe_round_trip(dataclasses.replace(resolved, prlctl="/invented/prlctl"), "cpc-z", MBF_TEXT,
                                     runner=runner)
    assert record_["route"] == "prepare_failed"
    assert "Z:\\cases\\cpc-z\\caepipe_in\\model.mbf" in commands[0]


def test_round_trip_refuses_outputs_of_a_different_guest_mbf(tmp_path):
    config = fake_config(tmp_path)
    commands = []
    out = config.exchange_root / "cases" / "cpc-h" / "caepipe_out"

    def runner(argv, **kwargs):
        commands.append(argv[6])
        stdout = ""
        if argv[6].startswith("powershell"):
            stdout = json.dumps({"status": "csv_from_batch", "mbf_sha256": "0" * 64,
                                 "csv_path": "C:\\invented-work\\cpc-h\\model.csv"})
        if "caepipe_out" in argv[6]:
            (out / "model.csv").write_text("Invented\n", encoding="utf-8")
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    record_ = run_caepipe_round_trip(config, "cpc-h", MBF_TEXT, runner=runner)
    assert record_["route"] == "input_hash_mismatch"
    assert record_["guest_mbf_sha256"] == "0" * 64 and record_["input_hash_verified"] is False
    assert record_["observed_csv_path"]["host"] is None
    assert not any("caepipe_out" in command for command in commands)  # nothing is collected
    assert not (out / "model.csv").exists()


# --------------------------------------------------------------------------
# exchange containment and offline binding (no prlctl, no runner)
# --------------------------------------------------------------------------
def git_repo(tmp_path: Path) -> Path:
    repo = tmp_path / "victim-repo"
    repo.mkdir()
    subprocess.run(["git", "init", "-q", str(repo)], check=True)
    return repo


def repo_entries(repo: Path) -> list[str]:
    return sorted(str(path.relative_to(repo)) for path in repo.rglob("*") if ".git" not in path.parts)


def test_symlinked_cases_dir_is_refused(tmp_path):
    config = fake_config(tmp_path)
    repo = git_repo(tmp_path)
    (config.exchange_root / "cases").symlink_to(repo, target_is_directory=True)
    with pytest.raises(ContainmentError):
        case_paths(config, "cpc-s")
    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-s", MBF_TEXT, runner=lambda *a, **k: pytest.fail("prlctl called"))
    lines = []
    assert run_pilot.run_case({"case_id": "cpc-s"}, config, runner="unused", offline=True, hand_calcs=None,
                              emit=lines.append) is False
    assert lines and "route=containment_violation" in lines[0]
    assert repo_entries(repo) == []


def test_symlinked_layer_dir_and_file_are_refused(tmp_path):
    config = fake_config(tmp_path)
    repo = git_repo(tmp_path)
    (repo / "tracked.txt").write_text("keep\n", encoding="utf-8")
    paths = case_paths(config, "cpc-l")
    paths["caepipe_out"].rmdir()
    paths["caepipe_out"].symlink_to(repo, target_is_directory=True)
    with pytest.raises(ContainmentError):
        case_paths(config, "cpc-l")
    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-l", MBF_TEXT, runner=lambda *a, **k: pytest.fail("prlctl called"))
    paths["caepipe_out"].unlink()
    paths["caepipe_out"].mkdir()
    (paths["caepipe_out"] / "run_record.json").symlink_to(repo / "tracked.txt")
    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-l", MBF_TEXT, runner=lambda *a, **k: pytest.fail("prlctl called"))
    assert repo_entries(repo) == ["tracked.txt"]
    assert (repo / "tracked.txt").read_text(encoding="utf-8") == "keep\n"
    assert not (config.exchange_root / "_harness" / "round_trip.lock").exists()


def test_case_variant_output_links_are_refused_before_collection(tmp_path):
    # On a case-insensitive file system a link named MODEL_RESULTS.CSV would receive the guest's copy of
    # model_results.csv; a link must be refused whatever its letter case, before the guest collects anything.
    victim = tmp_path / "outside" / "victim.txt"
    victim.parent.mkdir()
    victim.write_text("keep\n", encoding="utf-8")

    (tmp_path / "pre").mkdir()
    config = fake_config(tmp_path / "pre")
    paths = case_paths(config, "cpc-v")
    (paths["caepipe_out"] / "MODEL_RESULTS.CSV").symlink_to(victim)
    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-v", MBF_TEXT, runner=lambda *a, **k: pytest.fail("prlctl called"))

    (tmp_path / "during").mkdir()
    config = fake_config(tmp_path / "during")
    out = case_paths(config, "cpc-w")["caepipe_out"]
    commands = []

    def runner(argv, **kwargs):
        commands.append(argv[6])
        stdout = ""
        if argv[6].startswith("powershell"):
            (out / "Model_Results.csv").symlink_to(victim)  # planted while the guest solves
            stdout = json.dumps({"status": "csv_from_batch", "mbf_sha256": MBF_SHA,
                                 "csv_path": "C:\\invented-work\\cpc-w\\model_results.csv"})
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-w", MBF_TEXT, runner=runner)
    assert not any("caepipe_out" in command and command.startswith("copy") for command in commands)
    assert victim.read_text(encoding="utf-8") == "keep\n"


def test_directories_at_output_names_are_refused_before_collection(tmp_path):
    # A directory named like an output file would make the guest's `copy` write inside it, through any link there.
    victim = tmp_path / "outside" / "victim.txt"
    victim.parent.mkdir()
    victim.write_text("keep\n", encoding="utf-8")
    (tmp_path / "pre").mkdir()
    config = fake_config(tmp_path / "pre")
    nested = case_paths(config, "cpc-n")["caepipe_out"] / "model.res"
    nested.mkdir()
    (nested / "model.res").symlink_to(victim)
    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-n", MBF_TEXT, runner=lambda *a, **k: pytest.fail("prlctl called"))

    (tmp_path / "during").mkdir()
    config = fake_config(tmp_path / "during")
    out = case_paths(config, "cpc-m")["caepipe_out"]
    commands = []

    def runner(argv, **kwargs):
        commands.append(argv[6])
        stdout = ""
        if argv[6].startswith("powershell"):
            (out / "model.mod").mkdir()  # planted while the guest solves
            (out / "model.mod" / "model.mod").symlink_to(victim)
            stdout = json.dumps({"status": "csv_from_batch", "mbf_sha256": MBF_SHA,
                                 "csv_path": "C:\\invented-work\\cpc-m\\model_results.csv"})
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    with pytest.raises(ContainmentError):
        run_caepipe_round_trip(config, "cpc-m", MBF_TEXT, runner=runner)
    assert not any("caepipe_out" in command and command.startswith("copy") for command in commands)
    assert victim.read_text(encoding="utf-8") == "keep\n"


# --------------------------------------------------------------------------
# MBF export of the stored solved model (invented bytes, fake prlctl runner)
# --------------------------------------------------------------------------
MOD_BYTES = b"INVENTED SOLVED MODEL\r\n"
MOD_SHA = hashlib.sha256(MOD_BYTES).hexdigest()
EXPORTED_BYTES = b"INVENTED\r\nEXPORTED MBF\r\n"
EXPORTED_SHA = hashlib.sha256(EXPORTED_BYTES).hexdigest()


def solved_case(tmp_path: Path, case_id: str, *, hashed: bool = True, **extra) -> tuple[HarnessConfig, Path]:
    """An invented stored solve whose run record binds caepipe_out/model.mod to MBF_TEXT."""
    config = fake_config(tmp_path)
    out = case_paths(config, case_id)["caepipe_out"]
    (out / "model.mod").write_bytes(MOD_BYTES)
    record = {"document_kind": "openpipestress.caepipe_harness_run_record", "route": "csv_from_batch",
              "mbf": {"sha256": MBF_SHA}, "guest_mbf_sha256": MBF_SHA, "outputs_collected": ["model.csv", "model.mod"],
              **({"outputs_sha256": {"model.mod": MOD_SHA}} if hashed else {}), **extra}
    (out / "run_record.json").write_text(json.dumps(record), encoding="utf-8")
    return config, out


def export_runner(out: Path, commands: list[str], *, guest: dict | None = None, collected: bytes = EXPORTED_BYTES,
                  on_export=None):
    """A fake prlctl: the export step reports the invented hashes, the collect step writes the invented export."""

    def runner(argv, **kwargs):
        assert argv[:6] == ["/invented/prlctl", "exec", "invented-vm", "--current-user", "cmd", "/c"]
        command = argv[6]
        commands.append(command)
        stdout = ""
        if command.startswith("powershell"):
            if on_export:
                on_export()
            stdout = "banner\n" + json.dumps(guest if guest is not None else {
                "status": "mbf_exported", "mod_sha256": MOD_SHA.upper(), "exported_mbf_sha256": EXPORTED_SHA,
                "exported_mbf_path": "C:\\invented-work\\cpc-e\\export\\exported.mbf"})
        elif command.startswith("copy") and "caepipe_out" in command:
            (out / "exported.mbf").write_bytes(collected)
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    return runner


def test_mbf_export_collects_the_export_and_binds_it_to_the_run_record_by_hash(tmp_path):
    config, out = solved_case(tmp_path, "cpc-e")
    commands: list[str] = []
    entry = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(out, commands))
    assert [command.split(" ")[0] for command in commands] == ["(if", "powershell", "copy"]
    assert ('del /Q "C:\\invented-work\\cpc-e\\export\\model.mod" "C:\\invented-work\\cpc-e\\export\\exported.mbf"'
            in commands[0])
    assert 'copy /Y "Z:\\invented\\cases\\cpc-e\\caepipe_out\\model.mod" "C:\\invented-work\\cpc-e\\export\\model.mod"' \
        in commands[0]
    assert '-ExportMod "C:\\invented-work\\cpc-e\\export\\model.mod" -ExportName exported.mbf' in commands[1]
    assert "-Mbf" not in commands[1]
    assert commands[2] == ('copy /Y "C:\\invented-work\\cpc-e\\export\\exported.mbf" '
                           '"Z:\\invented\\cases\\cpc-e\\caepipe_out\\exported.mbf"')
    assert entry["route"] == "mbf_exported"
    assert entry["mod"]["sha256"] == MOD_SHA and entry["mod_hash_verified"] is True
    assert entry["solved_mbf_sha256"] == MBF_SHA
    assert entry["exported_mbf"] == {"path": "caepipe_out/exported.mbf", "sha256": EXPORTED_SHA,
                                     "bytes": len(EXPORTED_BYTES)}
    stored = json.loads((out / "run_record.json").read_text(encoding="utf-8"))
    assert stored["route"] == "csv_from_batch" and stored["mbf"]["sha256"] == MBF_SHA  # the solve record is kept
    assert stored["mbf_export"]["route"] == "mbf_exported"
    assert stored["mbf_export"]["exported_mbf"]["sha256"] == EXPORTED_SHA
    assert (out / "exported.mbf").read_bytes() == EXPORTED_BYTES
    assert json.loads((out / "export_guest_log.json").read_text(encoding="utf-8"))["status"] == "mbf_exported"
    assert "invented-vm" not in json.dumps(stored)
    assert not (config.exchange_root / "_harness" / "round_trip.lock").exists()

    # Idempotent: a second run clears the previous export before the guest runs and records the same binding.
    seen = []
    again = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(
        out, [], on_export=lambda: seen.append((out / "exported.mbf").exists())))
    assert seen == [False] and again["route"] == "mbf_exported"
    assert json.loads((out / "run_record.json").read_text(encoding="utf-8"))["mbf_export"]["exported_mbf"] == \
        entry["exported_mbf"]


def test_mbf_export_accepts_a_run_record_that_binds_the_model_by_name_only(tmp_path):
    config, out = solved_case(tmp_path, "cpc-e", hashed=False)
    assert run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(out, []))["route"] == "mbf_exported"


@pytest.mark.parametrize("label, extra, reason", [
    ("stale solve", {"mbf": {"sha256": "1" * 64}}, "not of the current MBF"),
    ("guest unverified", {"guest_mbf_sha256": None}, "not of the current MBF"),
    ("model not collected", {"outputs_collected": ["model.csv"]}, "names no collected caepipe_out/model.mod"),
    ("model replaced", {"outputs_sha256": {"model.mod": "2" * 64}}, "differs from the solved model"),
])
def test_mbf_export_refuses_a_solve_not_bound_to_the_current_mbf(tmp_path, label, extra, reason):
    config, out = solved_case(tmp_path, "cpc-e", **extra)
    before = sorted(path.name for path in out.iterdir())
    entry = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=lambda *a, **k: pytest.fail("prlctl called"))
    assert entry["route"] == "not_bound" and reason in entry["reason"], label
    assert sorted(path.name for path in out.iterdir()) == before
    (out / "run_record.json").unlink()
    entry = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=lambda *a, **k: pytest.fail("prlctl called"))
    assert entry["route"] == "not_bound" and "no valid caepipe_out/run_record.json" in entry["reason"]


def test_mbf_export_collects_nothing_when_the_guest_opened_a_different_model(tmp_path):
    config, out = solved_case(tmp_path, "cpc-e")
    commands: list[str] = []
    guest = {"status": "mbf_exported", "mod_sha256": "0" * 64, "exported_mbf_sha256": EXPORTED_SHA}
    entry = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(out, commands, guest=guest))
    assert entry["route"] == "input_hash_mismatch" and entry["mod_hash_verified"] is False
    assert len(commands) == 2 and not (out / "exported.mbf").exists()
    assert entry["exported_mbf"]["path"] is None


def test_mbf_export_drops_a_collected_file_whose_hash_differs_from_the_guest_report(tmp_path):
    config, out = solved_case(tmp_path, "cpc-e")
    entry = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(out, [], collected=b"OTHER\r\n"))
    assert entry["route"] == "export_hash_mismatch"
    assert not (out / "exported.mbf").exists() and entry["exported_mbf"]["sha256"] is None


def test_mbf_export_records_a_guest_failure_and_retries_the_prlctl_client(tmp_path):
    config, out = solved_case(tmp_path, "cpc-e")
    calls = []

    def runner(argv, **kwargs):
        calls.append(argv[6])
        if len(calls) == 1:  # the client's own intermittent failure, before the guest command ran
            return subprocess.CompletedProcess(argv, 255, "", "Failed to exec: PrlJob_invented")
        stdout = json.dumps({"status": "export_failed", "mod_sha256": MOD_SHA}) if argv[6].startswith("powershell") else ""
        return subprocess.CompletedProcess(argv, 0 if not stdout else 4, stdout, "")

    entry = run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=runner)
    assert entry["route"] == "export_failed"
    assert entry["steps"][0]["name"] == "export_prepare" and entry["steps"][0]["attempts"] == 2
    assert not any(command.startswith("copy") for command in calls)
    assert json.loads((out / "run_record.json").read_text(encoding="utf-8"))["mbf_export"]["route"] == "export_failed"


def test_mbf_export_refuses_links_planted_in_the_output_layer(tmp_path):
    victim = tmp_path / "outside" / "victim.txt"
    victim.parent.mkdir()
    victim.write_text("keep\n", encoding="utf-8")
    (tmp_path / "pre").mkdir()
    config, out = solved_case(tmp_path / "pre", "cpc-e")
    (out / "EXPORTED.MBF").symlink_to(victim)
    with pytest.raises(ContainmentError):
        run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=lambda *a, **k: pytest.fail("prlctl called"))

    (tmp_path / "during").mkdir()
    config, out = solved_case(tmp_path / "during", "cpc-e")
    commands: list[str] = []
    with pytest.raises(ContainmentError):
        run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(
            out, commands, on_export=lambda: (out / "Exported.mbf").symlink_to(victim)))
    assert not any(command.startswith("copy") for command in commands)
    assert victim.read_text(encoding="utf-8") == "keep\n"
    assert not (config.exchange_root / "_harness" / "round_trip.lock").exists()


def test_a_new_solve_clears_the_previous_export_and_hashes_its_outputs(tmp_path):
    config, out = solved_case(tmp_path, "cpc-x")
    run_caepipe_mbf_export(config, "cpc-x", MBF_SHA, runner=export_runner(out, []))
    assert (out / "exported.mbf").is_file()

    def runner(argv, **kwargs):
        stdout = ""
        if argv[6].startswith("powershell"):
            stdout = json.dumps({"status": "csv_from_batch", "mbf_sha256": MBF_SHA,
                                 "csv_path": "C:\\invented-work\\cpc-x\\model.csv"})
        elif argv[6].startswith("copy"):
            (out / "model.csv").write_text("Invented\n", encoding="utf-8")
            (out / "model.mod").write_bytes(MOD_BYTES)
        return subprocess.CompletedProcess(argv, 0, stdout, "")

    record_ = run_caepipe_round_trip(config, "cpc-x", MBF_TEXT, runner=runner)
    assert not any((out / name).exists() for name in ("exported.mbf", "export_guest_log.json",
                                                      "export_guest_stdout.txt"))
    assert "mbf_export" not in record_
    assert record_["outputs_sha256"]["model.mod"] == MOD_SHA


def test_pilot_export_step_prints_operational_status_only(tmp_path):
    case = next(item for item in run_pilot.load_cases() if item["case_id"] == "cpc-01-cantilever-force")
    sha = current_mbf_sha(case["case_id"])
    config = fake_config(tmp_path)
    calls = []

    def export(config_, case_id, mbf_sha256):
        calls.append((case_id, mbf_sha256))
        return {"route": "mbf_exported"} if len(calls) == 1 else {"route": "not_bound", "reason": "invented reason"}

    lines: list[str] = []
    assert run_pilot.export_case(case, config, emit=lines.append, export=export) is True
    assert run_pilot.export_case(case, config, emit=lines.append, export=export) is False
    assert calls == [(case["case_id"], sha)] * 2
    assert lines == [
        "cpc-01-cantilever-force: export route=mbf_exported; exported MBF written: "
        "cases/cpc-01-cantilever-force/caepipe_out/exported.mbf",
        "cpc-01-cantilever-force: export route=not_bound; invented reason; no exported MBF",
    ]

    def failed(config_, case_id, mbf_sha256):
        return {"route": "export_failed", "exported_mbf": {"sha256": None}}

    lines.clear()
    assert run_pilot.export_case(case, config, emit=lines.append, export=failed) is False
    assert lines == ["cpc-01-cantilever-force: export route=export_failed; the guest export did not complete "
                     "(cases/cpc-01-cantilever-force/caepipe_out/run_record.json); no exported MBF"]


def test_pilot_runs_the_export_step_after_each_case_only_when_asked(tmp_path, monkeypatch):
    cases = [item for item in run_pilot.load_cases() if item["case_id"] in ("cpc-01-cantilever-force",
                                                                            "cpc-02-anchored-thermal")]
    order = []
    monkeypatch.setattr(run_pilot, "run_case", lambda case, *a, **k: order.append(("case", case["case_id"])) or True)
    monkeypatch.setattr(run_pilot, "run_series", lambda *a, **k: {})
    export = lambda config_, case_id, sha: order.append(("export", case_id)) or {"route": "mbf_exported"}
    config = fake_config(tmp_path)
    results = run_pilot.run_cases(cases, config, runner="unused", offline=False, hand_calcs=None,
                                  emit=lambda line: None, export_mbf=True, export=export)
    assert order == [("case", "cpc-01-cantilever-force"), ("export", "cpc-01-cantilever-force"),
                     ("case", "cpc-02-anchored-thermal"), ("export", "cpc-02-anchored-thermal")]
    assert results == {"cpc-01-cantilever-force": True, "cpc-01-cantilever-force export": True,
                       "cpc-02-anchored-thermal": True, "cpc-02-anchored-thermal export": True}
    order.clear()
    run_pilot.run_cases(cases, config, runner="unused", offline=False, hand_calcs=None, emit=lambda line: None,
                        export=export)
    assert [kind for kind, _ in order] == ["case", "case"]


def test_pilot_export_only_needs_no_runner_and_refuses_offline(tmp_path, monkeypatch, capsys):
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "run_cases", lambda *a, **k: pytest.fail("comparison run"))
    exported = []
    monkeypatch.setattr(run_pilot, "export_case", lambda case, config_: exported.append(case["case_id"]) or True)
    assert run_pilot.main(["--export-only", "--case", "cpc-03-anchored-weight"], environ={}) == 0
    assert exported == ["cpc-03-anchored-weight"]
    with pytest.raises(SystemExit) as raised:
        run_pilot.main(["--offline", "--export-mbf"], environ={})
    assert raised.value.code == 2
    assert "not allowed with argument" in capsys.readouterr().err


def test_pilot_authored_mode_round_trips_the_authored_file_then_exports_it(tmp_path):
    case = next(item for item in run_pilot.load_authored_cases() if item["case_id"] == "cpa-05-guide")
    data = run_pilot.authored_mbf_bytes(case)
    sha = hashlib.sha256(data).hexdigest()
    assert data.startswith(b"INVENTED AUTHORED FIXTURE\r\n") and data.endswith(b"\r\n")
    config = fake_config(tmp_path)
    order: list[tuple] = []

    def round_trip(config_, case_id, mbf_text):
        order.append(("round_trip", case_id, mbf_text.encode("utf-8") == data))
        return {"route": "csv_from_batch", "mbf": {"sha256": sha}, "guest_mbf_sha256": sha}

    def export(config_, case_id, mbf_sha256):
        order.append(("export", case_id, mbf_sha256 == sha))
        return {"route": "mbf_exported"}

    lines: list[str] = []
    assert run_pilot.run_authored_case(case, config, emit=lines.append, round_trip=round_trip, export=export)
    assert order == [("round_trip", "cpa-05-guide", True), ("export", "cpa-05-guide", True)]
    assert lines == ["cpa-05-guide: route=csv_from_batch; round trip recorded: cases/cpa-05-guide/caepipe_out/run_record.json",
                     "cpa-05-guide: export route=mbf_exported; exported MBF written: "
                     "cases/cpa-05-guide/caepipe_out/exported.mbf"]
    order.clear()
    lines.clear()
    assert run_pilot.run_authored_case(case, config, export_only=True, emit=lines.append,
                                       round_trip=lambda *a: pytest.fail("round trip"), export=export)
    assert order == [("export", "cpa-05-guide", True)]

    # A round trip whose guest did not report importing this file stops before the export.
    order.clear()
    lines.clear()
    assert not run_pilot.run_authored_case(
        case, config, emit=lines.append, export=export,
        round_trip=lambda *a: {"route": "input_hash_mismatch", "mbf": {"sha256": sha}, "guest_mbf_sha256": "0" * 64})
    assert order == [] and lines == ["cpa-05-guide: route=input_hash_mismatch; the guest did not report importing "
                                     "this MBF; no exported MBF"]


def test_pilot_authored_option_needs_no_runner_and_refuses_comparison_modes(tmp_path, monkeypatch, capsys):
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "run_cases", lambda *a, **k: pytest.fail("comparison run"))
    calls = []
    monkeypatch.setattr(run_pilot, "run_authored_case",
                        lambda case, config_, export_only, directory, no_export, gui_fallback: calls.append(
                            (case["case_id"], export_only)) or True)
    assert run_pilot.main(["--authored", "--case", "cpa-12-guide-gap"], environ={}) == 0
    assert run_pilot.main(["--authored", "--export-only", "--case", "cpa-12-guide-gap"], environ={}) == 0
    assert calls == [("cpa-12-guide-gap", False), ("cpa-12-guide-gap", True)]
    assert run_pilot.main(["--authored", "--case", "cpc-01-cantilever-force"], environ={}) == 2
    for mode in ("--offline", "--export-mbf"):
        with pytest.raises(SystemExit) as raised:
            run_pilot.main(["--authored", mode], environ={})
        assert raised.value.code == 2
    assert "--authored takes only --case and --export-only" in capsys.readouterr().err


INVENTED_GUARD = SettingsGuard(key="HKCU\\Software\\Invented Vendor\\Tool\\1.0", recent_files_subkey="Recent list")
RESTORING_GUARD = dataclasses.replace(INVENTED_GUARD, restore_recent_files=True)
INVENTED_KEY = "HKEY_CURRENT_USER\\Software\\Invented Vendor\\Tool\\1.0"


def _hex(text: str) -> str:
    return (text.encode("latin-1") + b"\x00").hex().upper()


class FakeGuest:
    """An invented guest: a registry answering `reg query /s`, the owned-process id file and `tasklist`.

    Any `reg add`/`reg delete` is recorded as a write; the guard must never issue one.
    """

    def __init__(self):
        self.values = {
            (INVENTED_KEY + "\\Options", "Use something"): ("REG_BINARY", "01"),
            (INVENTED_KEY + "\\Recent list", "N"): ("REG_BINARY", "02000000"),
            (INVENTED_KEY + "\\Recent list", "1"): ("REG_BINARY", _hex("C:\\invented-work\\old-a\\model.mod")),
            (INVENTED_KEY + "\\Recent list", "2"): ("REG_BINARY", _hex("D:\\user files\\plant.mod")),
        }
        self.owned: list[int] = []      # ids the guest script recorded
        self.running: list[int] = []    # ids of the tool's image still listed
        self.writes: list[str] = []
        self.commands: list[str] = []
        self.empty_keys: set[str] = {INVENTED_KEY + "\\Empty sub"}  # a subkey with no values
        self.pid_text: str | None = None       # replaces the owned-process record when set
        self.tasklist_text: str | None = None  # replaces the process list after the solve when set
        self.tasklist_error: Exception | None = None  # raised by the process list after the solve when set
        self.foreign: list[int] = []           # another instance of the tool, listed throughout
        self.foreign_after: list[int] = []     # another instance that appears during the solve
        self.apply_writes = True               # False: writes are accepted but change nothing
        self.solved = False
        self.counts = {"tasklist": 0, "query": 0, "write": 0, "readback": 0}
        self.foreign_on_query: dict[int, list[int]] = {}     # query number -> an instance that appears then
        self.foreign_on_tasklist: dict[int, list[int]] = {}  # process-list number -> an instance that appears then
        self.faults: dict[tuple[str, int], BaseException] = {}  # (kind, number) -> raised by that command
        self.write_results: dict[int, tuple[int, str, str]] = {}  # write number -> (exit, stdout, stderr), not applied
        self.applied_anyway: set[int] = set()  # write numbers in write_results that take effect all the same
        # read-back number -> what it shows: a (type, data) value, None (absent), "unreadable", or for a subkey a
        # dict of its values {(key, name): (type, data)}
        self.readback: dict[int, object] = {}
        self.flaky: dict[str, int] = {}  # kind -> how many of its next calls fail in the prlctl transport (exit 255)
        self.signal_on_write: dict[int, int] = {}  # write number -> a real signal raised from inside that command
        self.signal_on: dict[tuple[str, int], int] = {}  # (kind, number) -> a real signal raised from that command
        self.timeouts: list[tuple[str, float]] = []      # (command, timeout passed to the runner)
        self.tick: Callable[[], None] | None = None      # called on every command (a fake clock)
        self.raised = False                         # set once an injected fault has been raised

    def query(self) -> str:
        # As `reg query /s` prints it: CRLF lines, a blank line first, and no header for the queried key itself
        # while it holds no values of its own.
        lines = [""]
        for key in sorted({key for key, _ in self.values} | self.empty_keys):
            lines += [key] + [f"    {name}    {kind}    {data}" if data else f"    {name}    {kind}"
                              for (k, name), (kind, data) in sorted(self.values.items()) if k == key] + [""]
        return "\r\n".join(lines)

    def open_model(self, path: str) -> None:
        """What a tool that keeps a recent-files list does when it opens `path`: shift the list, put it first."""
        recent = INVENTED_KEY + "\\Recent list"
        old = [self.values[(recent, str(i))] for i in (1, 2)]
        self.values[(recent, "1")] = ("REG_BINARY", _hex(path))
        self.values[(recent, "2")] = old[0]
        self.values[(recent, "3")] = old[1]
        self.values[(recent, "N")] = ("REG_BINARY", "03000000")

    def runner(self, on_solve, config: HarnessConfig):
        def run(argv, **kwargs):
            command = argv[6]
            self.commands.append(command)
            kind = ("readback" if command.startswith("reg query ") and (" /v " in command
                                                                       or command.startswith('reg query "HKEY_'))
                    else "query" if command.startswith("reg query ") else "write" if command.startswith(("reg add", "reg delete"))
                    else "tasklist" if command == "tasklist /FO CSV /NH" else None)
            self.timeouts.append((command, kwargs.get("timeout")))
            if self.tick is not None:
                self.tick()
            if kind and self.flaky.get(kind):
                self.flaky[kind] -= 1
                return subprocess.CompletedProcess(argv, 255, "", "Failed to exec: PrlJob_invented transport")
            if kind:
                self.counts[kind] += 1
                if kind == "write" and self.counts[kind] in self.signal_on_write:
                    signal.raise_signal(self.signal_on_write[self.counts[kind]])  # the callee receives a real signal
                if (kind, self.counts[kind]) in self.signal_on:
                    signal.raise_signal(self.signal_on[(kind, self.counts[kind])])
                if (kind, self.counts[kind]) in self.faults:
                    self.raised = True
                    if kind == "write":
                        self.writes.append(command)  # issued, not applied
                    raise self.faults[(kind, self.counts[kind])]
            if kind == "tasklist":
                self.foreign += self.foreign_on_tasklist.get(self.counts[kind], [])
            if kind == "readback":  # one value (`reg query "<key>" /v "<name>"`) or one subkey (`... /s`)
                not_found = subprocess.CompletedProcess(argv, 1, "", "ERROR: The system was unable to find the "
                                                                     "specified registry key or value.")
                one = re.fullmatch(r'reg query "([^"]+)" /v "([^"]+)"', command)
                override = self.readback.get(self.counts["readback"], "none")
                if override == "unreadable":
                    return subprocess.CompletedProcess(argv, 2, "garbled", "")
                if one:
                    key, name = one.groups()
                    found = self.values.get((key, name)) if override == "none" else override
                    if found is None:
                        return not_found
                    return subprocess.CompletedProcess(argv, 0, f"\r\n{key}\r\n    {name}    {found[0]}    "
                                                                f"{found[1]}\r\n\r\n", "")
                key = re.fullmatch(r'reg query "([^"]+)" /s', command)[1]
                inside = lambda k: k == key or k.startswith(key + "\\")  # noqa: E731
                values = ({item: value for item, value in self.values.items() if inside(item[0])}
                          if override == "none" else override)
                if values is None or (not values and not any(inside(k) for k in self.empty_keys)):
                    return not_found
                lines = [""]
                for k in sorted({k for k, _ in values}):
                    lines += [k] + [f"    {n}    {t}    {d}" for (kk, n), (t, d) in sorted(values.items()) if kk == k] + [""]
                return subprocess.CompletedProcess(argv, 0, "\r\n".join(lines), "")
            if command.startswith("reg query "):
                self.foreign += self.foreign_on_query.get(self.counts["query"], [])
                return subprocess.CompletedProcess(argv, 0, self.query(), "")
            if command.startswith(("reg add", "reg delete")):
                self.writes.append(command)
                if self.counts["write"] in self.write_results:
                    code, out, err = self.write_results[self.counts["write"]]
                    if self.counts["write"] not in self.applied_anyway:
                        return subprocess.CompletedProcess(argv, code, out, err)
                    self.write_results.pop(self.counts["write"])
                    run(argv, **kwargs)  # applied (counted once more, then put back)
                    self.counts["write"] -= 1
                    self.writes.pop()
                    self.commands.pop()
                    self.timeouts.pop()
                    return subprocess.CompletedProcess(argv, code, out, err)
                add = re.fullmatch(r'reg add "([^"]+)" /v "([^"]+)" /t (REG_\w+) /d "([^"]*)" /f', command)
                delete = re.fullmatch(r'reg delete "([^"]+)" /v "([^"]+)" /f', command)
                add_key = re.fullmatch(r'reg add "([^"]+)" /f', command)
                delete_key = re.fullmatch(r'reg delete "([^"]+)" /f', command)
                assert add or delete or add_key or delete_key, command
                if self.apply_writes and add:
                    self.values[(add[1], add[2])] = (add[3], add[4])
                if self.apply_writes and delete:
                    del self.values[(delete[1], delete[2])]
                if self.apply_writes and add_key:
                    self.empty_keys.add(add_key[1])
                if self.apply_writes and delete_key:  # the subkey and everything in it
                    gone = lambda key: key == delete_key[1] or key.startswith(delete_key[1] + "\\")  # noqa: E731
                    self.values = {item: value for item, value in self.values.items() if not gone(item[0])}
                    self.empty_keys = {key for key in self.empty_keys if not gone(key)}
                return subprocess.CompletedProcess(argv, 0, "The operation completed successfully.\r\n", "")
            if command.startswith("type ") and "owned_pids.txt" in command:
                if self.pid_text is not None:
                    return subprocess.CompletedProcess(argv, 0, self.pid_text, "")
                if not self.owned:
                    return subprocess.CompletedProcess(argv, 1, "", "The system cannot find the file specified.")
                return subprocess.CompletedProcess(argv, 0, "".join(f"{pid}\r\n" for pid in self.owned), "")
            if command == "tasklist /FO CSV /NH":
                if self.solved and self.tasklist_error is not None:
                    raise self.tasklist_error
                if self.solved and self.tasklist_text is not None:
                    return subprocess.CompletedProcess(argv, 0, self.tasklist_text, "")
                rows = ['"other.exe","9999","Console","2","61,744 K"']  # another program: never ours, never foreign
                tool = self.foreign + (self.foreign_after + self.running if self.solved else [])
                rows += [f'"tool.exe","{pid}","Console","2","12,000 K"' for pid in tool]
                return subprocess.CompletedProcess(argv, 0, "\r\n".join(rows) + "\r\n", "")
            if command.startswith("powershell"):
                self.solved = True
                self.owned.append(4242)
                return on_solve(self, argv, config)
            return subprocess.CompletedProcess(argv, 0, "", "")
        return run


def solved(guest, argv, config):
    """A solve step that ends normally, its own process exited, with a batch CSV."""
    return subprocess.CompletedProcess(argv, 0, json.dumps({"status": "csv_from_batch", "mbf_sha256": MBF_SHA,
                                                            "owned_pids": list(guest.owned),
                                                            "csv_path": "C:\\invented-work\\cx-g\\model.csv"}), "")


def guarded(tmp_path, guest: FakeGuest, on_solve, guard: SettingsGuard = INVENTED_GUARD, **kwargs) -> dict:
    config = dataclasses.replace(fake_config(tmp_path), settings_guard=guard)
    return run_caepipe_round_trip(config, "cx-g", MBF_TEXT, runner=guest.runner(on_solve, config), **kwargs)


def test_settings_guard_reads_the_whole_key_confirms_exit_and_never_writes(tmp_path):
    guest = FakeGuest()
    record_ = guarded(tmp_path, guest, solved, gui_fallback=False)
    assert record_["settings_guard"]["status"] == "unchanged"
    assert record_["settings_guard"]["owned_processes"] == {"status": "exited", "owned_pids": [4242]}
    assert [step["name"] for step in record_["steps"]] == [
        "precondition_process_list", "settings_before", "prepare", "solve", "collect", "owned_pids", "process_list",
        "settings_after"]
    # The whole version key with every subkey, not the recent-files subkey alone.
    assert guest.commands[1] == 'reg query "HKCU\\Software\\Invented Vendor\\Tool\\1.0" /s'
    solve = next(command for command in guest.commands if command.startswith("powershell"))
    assert '-PidFile "C:\\invented-work\\cx-g\\owned_pids.txt"' in solve and solve.endswith(" -NoGuiFallback")
    assert record_["command_shape"]["gui_print_fallback"]["allowed"] is False
    assert guest.writes == []


def test_settings_guard_reports_a_recent_list_change_and_writes_nothing(tmp_path):
    guest = FakeGuest()

    def solve(guest_, argv, config):
        guest_.open_model("C:\\invented-work\\cx-g\\model.mod")  # the solve route itself updates the list
        return solved(guest_, argv, config)

    record_ = guarded(tmp_path, guest, solve)
    guard = record_["settings_guard"]
    assert guard["status"] == "settings_changed" and "restoring is not enabled" in guard["reason"]
    changed = {item["value"]: item for item in guard["changed_values"]}
    assert set(changed) == {INVENTED_KEY + "\\Recent list\\" + name for name in ("1", "2", "3", "N")}
    assert changed[INVENTED_KEY + "\\Recent list\\3"]["before"] is None
    assert guest.writes == []


def test_settings_guard_stops_on_a_concurrent_change_with_no_write(tmp_path):
    guest = FakeGuest()

    def solve(guest_, argv, config):
        # The user's own instance changes a setting while the run is under way.
        guest_.values[(INVENTED_KEY + "\\Options", "Use something")] = ("REG_BINARY", "00")
        return solved(guest_, argv, config)

    record_ = guarded(tmp_path, guest, solve)
    assert record_["settings_guard"]["status"] == "settings_changed"
    assert record_["settings_guard"]["changed_values"] == [{
        "value": INVENTED_KEY + "\\Options\\Use something", "before": {"type": "REG_BINARY", "data": "01"},
        "after": {"type": "REG_BINARY", "data": "00"}}]
    assert guest.writes == []
    assert guest.values[(INVENTED_KEY + "\\Options", "Use something")] == ("REG_BINARY", "00")


def test_settings_guard_still_checks_after_a_collection_exception(tmp_path):
    guest = FakeGuest()

    def solve(guest_, argv, config):
        # A link appears in the output layer, so the harness refuses to collect into it and raises.
        out = config.exchange_root / "cases" / "cx-g" / "caepipe_out"
        (out / "model.csv").symlink_to(tmp_path / "elsewhere.csv")
        return solved(guest_, argv, config)

    with pytest.raises(ContainmentError) as raised:
        guarded(tmp_path, guest, solve)
    after_solve = guest.commands[guest.commands.index(next(c for c in guest.commands if c.startswith("powershell"))):]
    assert [c.split(" ")[0] for c in after_solve] == ["powershell", "type", "tasklist", "reg"]
    assert raised.value.settings_guard["status"] == "unchanged"
    assert guest.writes == []


def test_settings_guard_stops_when_a_host_timeout_leaves_our_process_running_or_unknown(tmp_path):
    def timed_out(guest_, argv, config):
        raise subprocess.TimeoutExpired(argv, 600)

    guest = FakeGuest()
    guest.running = [4242]
    record_ = guarded(tmp_path, guest, timed_out)
    assert record_["route"] == "timeout"
    assert record_["settings_guard"]["status"] == "owned_process_running"
    assert record_["settings_guard"]["owned_processes"]["running"] == [4242]

    # The guest's record of its processes cannot be read after a timeout: nothing shows they ended.
    guest = FakeGuest()

    def timed_out_unrecorded(guest_, argv, config):
        guest_.owned.clear()
        raise subprocess.TimeoutExpired(argv, 600)

    second = tmp_path / "second"
    second.mkdir()
    record_ = guarded(second, guest, timed_out_unrecorded)
    assert record_["settings_guard"]["status"] == "owned_process_state_unknown"
    assert guest.writes == []


def test_settings_guard_sees_an_empty_subkey_added_or_deleted(tmp_path):
    def add_empty(guest_, argv, config):
        guest_.empty_keys.add(INVENTED_KEY + "\\Another empty sub")
        return solved(guest_, argv, config)

    def delete_empty(guest_, argv, config):
        guest_.empty_keys.clear()
        return solved(guest_, argv, config)

    for name, solve, before, after in (("added", add_empty, None, "present"),
                                       ("deleted", delete_empty, "present", None)):
        (tmp_path / name).mkdir()
        guest = FakeGuest()
        record_ = guarded(tmp_path / name, guest, solve)
        assert record_["settings_guard"]["status"] == "settings_changed", name
        assert [item for item in record_["settings_guard"]["changed_values"] if "key" in item] == [{
            "key": INVENTED_KEY + ("\\Another empty sub" if name == "added" else "\\Empty sub"),
            "before": before, "after": after}]
        assert guest.writes == []


def test_settings_guard_needs_a_complete_valid_ownership_record_and_process_list(tmp_path):
    def timed_out(guest_, argv, config):
        raise subprocess.TimeoutExpired(argv, 600)

    variants = {
        # A timeout with our tool still running, and an empty or malformed record of what the guest started.
        "empty_record": dict(pid_text="\r\n", running=[4242], solve=timed_out),
        "malformed_record": dict(pid_text="4242\r\n42x\r\n", running=[4242], solve=timed_out),
        # The guest's own report names a process the record lacks.
        "record_short_of_report": dict(
            pid_text="4242\r\n", solve=lambda g, a, c: subprocess.CompletedProcess(a, 0, json.dumps(
                {"status": "csv_from_batch", "mbf_sha256": MBF_SHA, "owned_pids": [4242, 4300],
                 "csv_path": "C:\\invented-work\\cx-g\\model.csv"}), "")),
        "empty_process_list": dict(tasklist_text="\r\n", solve=solved),
        "malformed_process_list": dict(tasklist_text="INFO: No tasks are running which match the specified criteria.\r\n",
                                       solve=solved),
    }
    for name, variant in variants.items():
        (tmp_path / name).mkdir()
        guest = FakeGuest()
        guest.pid_text = variant.get("pid_text")
        guest.tasklist_text = variant.get("tasklist_text")
        guest.running = variant.get("running", [])
        record_ = guarded(tmp_path / name, guest, variant["solve"])
        assert record_["settings_guard"]["status"] == "owned_process_state_unknown", name
        assert record_["settings_guard"]["owned_processes"]["status"] == "unknown", name
        assert guest.writes == []


def test_settings_guard_reads_the_settings_and_attaches_a_result_when_the_process_query_raises(tmp_path):
    guest = FakeGuest()
    guest.tasklist_error = UnicodeDecodeError("utf-8", b"\xff", 0, 1, "invented decoding failure")
    record_ = guarded(tmp_path, guest, solved)
    guard = record_["settings_guard"]
    assert guard["status"] == "owned_process_state_unknown"
    assert "UnicodeDecodeError" in guard["owned_processes"]["reason"]
    assert guest.commands[-1] == 'reg query "HKCU\\Software\\Invented Vendor\\Tool\\1.0" /s'
    assert [step["name"] for step in record_["steps"]][-1] == "settings_after"
    assert guest.writes == []


def test_settings_guard_runs_nothing_when_the_settings_cannot_be_read(tmp_path):
    config = dataclasses.replace(fake_config(tmp_path), settings_guard=INVENTED_GUARD)
    commands: list[str] = []

    def runner(argv, **kwargs):
        commands.append(argv[6])
        if argv[6] == "tasklist /FO CSV /NH":
            return subprocess.CompletedProcess(argv, 0, '"other.exe","9999","Console","2","1 K"\r\n', "")
        return subprocess.CompletedProcess(argv, 1, "", "ERROR: invented")

    record_ = run_caepipe_round_trip(config, "cx-h", MBF_TEXT, runner=runner)
    assert record_["route"] == "settings_check_failed"
    assert record_["settings_guard"]["status"] == "settings_check_failed"
    assert commands == ["tasklist /FO CSV /NH", 'reg query "HKCU\\Software\\Invented Vendor\\Tool\\1.0" /s']


def test_settings_guard_starts_nothing_while_another_instance_runs(tmp_path):
    guest = FakeGuest()
    guest.foreign = [16000]
    record_ = guarded(tmp_path, guest, solved, guard=RESTORING_GUARD)
    assert record_["route"] == "foreign_instance_running"
    assert record_["settings_guard"]["status"] == "foreign_instance_running"
    assert record_["settings_guard"]["foreign_pids"] == [16000]
    assert guest.commands == ["tasklist /FO CSV /NH"] and guest.writes == []

    # An unreadable process list is no precondition either.
    second = tmp_path / "second"
    second.mkdir()
    guest = FakeGuest()
    guest.tasklist_text, guest.solved = "garbage\r\n", True
    record_ = guarded(second, guest, solved, guard=RESTORING_GUARD)
    assert record_["route"] == "precondition_unknown" and guest.commands == ["tasklist /FO CSV /NH"]

    # The session precondition reads the same list and starts nothing.
    third = tmp_path / "third"
    third.mkdir()
    session_config = dataclasses.replace(fake_config(third), settings_guard=RESTORING_GUARD)
    guest = FakeGuest()
    guest.foreign = [16000]
    assert check_no_foreign_instance(session_config, runner=guest.runner(solved, session_config))["status"] == \
        "foreign_instance_running"
    guest.foreign = []
    assert check_no_foreign_instance(session_config, runner=guest.runner(solved, session_config)) == {"status": "clear"}
    assert set(guest.commands) == {"tasklist /FO CSV /NH"}
    assert guest.timeouts and all(0 < timeout <= 20.0 for _, timeout in guest.timeouts)  # the guard's call timeout


def test_settings_guard_restores_a_recent_files_only_change_and_verifies_it(tmp_path):
    guest = FakeGuest()
    before = dict(guest.values)

    def solve(guest_, argv, config):
        guest_.open_model("C:\\invented-work\\cx-g\\model.mod")
        return solved(guest_, argv, config)

    record_ = guarded(tmp_path, guest, solve, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "recent_files_restored" and guard["restored_values"] == 4
    assert guest.values == before
    assert guest.writes and all('\\Recent list" /v' in command for command in guest.writes)
    assert any(command.startswith("reg delete") and '/v "3"' in command for command in guest.writes)
    # Written back in the first reading's order (1, 2, N), then the value that appeared is deleted.
    assert [re.search(r'/v "([^"]+)"', command)[1] for command in guest.writes] == ["1", "2", "N", "3"]
    assert [entry["value"].rsplit("\\", 1)[1] for entry in guard["write_log"]] == ["1", "2", "N", "3"]
    assert all(entry["completed"] for entry in guard["write_log"])
    names = [step["name"] for step in record_["steps"]]
    assert names[-11:] == ["restore_check_before_write_1", "settings_restore_1", "restore_check_before_write_2",
                           "settings_restore_2", "restore_check_before_write_3", "settings_restore_3",
                           "restore_check_before_write_4", "settings_restore_4", "restore_check_after_writes",
                           "settings_verify", "restore_check_before_success"][-11:]


def test_settings_guard_restores_nothing_when_more_than_recent_files_changed(tmp_path):
    guest = FakeGuest()

    def solve(guest_, argv, config):
        guest_.open_model("C:\\invented-work\\cx-g\\model.mod")
        guest_.values[(INVENTED_KEY + "\\Options", "Use something")] = ("REG_BINARY", "00")
        return solved(guest_, argv, config)

    record_ = guarded(tmp_path, guest, solve, guard=RESTORING_GUARD)
    assert record_["settings_guard"]["status"] == "settings_changed"
    assert guest.writes == []

    # Nor when another instance appeared during the run, however small the change.
    second = tmp_path / "second"
    second.mkdir()
    guest = FakeGuest()

    def solve_with_visitor(guest_, argv, config):
        guest_.open_model("C:\\invented-work\\cx-g\\model.mod")
        guest_.foreign_after = [17000]
        return solved(guest_, argv, config)

    record_ = guarded(second, guest, solve_with_visitor, guard=RESTORING_GUARD)
    assert record_["settings_guard"]["status"] == "foreign_instance_running"
    assert record_["settings_guard"]["owned_processes"]["foreign_pids"] == [17000]
    assert guest.writes == []


def _open_model(guest_, argv, config):
    guest_.open_model("C:\\invented-work\\cx-g\\model.mod")  # four restore writes: set 1, 2, N; delete 3
    return solved(guest_, argv, config)


# Process-list numbers: 1 precondition, 2 our processes, 3-6 before writes 1-4, 7 after the writes, 8 before
# success. Readings: 1 before, 2 after, 3 verification, 4 the failure path's own reading.
FOREIGN_ARRIVALS = [  # (where, injection, writes completed)
    ("before write 1", ("tasklist", 3), 0), ("before write 2", ("tasklist", 4), 1),
    ("before write 3", ("tasklist", 5), 2), ("before write 4", ("tasklist", 6), 3),
    ("after write 4", ("tasklist", 7), 4), ("during verification", ("query", 3), 4),
]
FAULT_STEPS = [  # (where, injection, writes attempted, writes completed)
    ("write 1", ("write", 1), 1, 0), ("write 2", ("write", 2), 2, 1), ("write 3", ("write", 3), 3, 2),
    ("write 4", ("write", 4), 4, 3), ("check before write 1", ("tasklist", 3), 0, 0),
    ("check before write 2", ("tasklist", 4), 1, 1), ("check before write 3", ("tasklist", 5), 2, 2),
    ("check before write 4", ("tasklist", 6), 3, 3), ("check after the writes", ("tasklist", 7), 4, 4),
    ("verification reading", ("query", 3), 4, 4), ("verification comparison", ("compare", 2), 4, 4),
    ("check before success", ("tasklist", 8), 4, 4),
]


@pytest.mark.parametrize("where, injection, completed", FOREIGN_ARRIVALS, ids=[f[0] for f in FOREIGN_ARRIVALS])
def test_settings_restore_stops_on_another_instance_at_any_check(tmp_path, where, injection, completed):
    guest = FakeGuest()
    kind, number = injection
    (guest.foreign_on_tasklist if kind == "tasklist" else guest.foreign_on_query)[number] = [18000]
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "settings_restore_failed" and guard["stopped_by"]["status"] == "foreign_instance_running"
    assert guard["writes_completed"] == completed and len(guard["write_log"]) == completed
    assert all(entry["completed"] for entry in guard["write_log"]) and len(guest.writes) == completed
    assert ("nothing was written" in guard["reason"]) == (completed == 0)
    # A partial restore is shown as what remains different; a complete one (then interrupted) leaves nothing.
    assert guard["final_reading"] == "read" and bool(guard["remaining_differences"]) == (completed < 4)
    assert guest.commands[-1].startswith("reg query ")


@pytest.mark.parametrize("error_type", [RuntimeError, KeyboardInterrupt])
@pytest.mark.parametrize("where, injection, attempted, completed", FAULT_STEPS, ids=[f[0] for f in FAULT_STEPS])
def test_settings_restore_records_an_exception_at_any_step_then_raises_it(tmp_path, monkeypatch, where, injection,
                                                                         attempted, completed, error_type):
    guest = FakeGuest()
    error = error_type("invented failure")
    if injection[0] == "compare":
        real, calls = harness.settings_difference, []

        def failing(before, after):
            calls.append(1)
            if len(calls) == injection[1]:
                raise error
            return real(before, after)

        monkeypatch.setattr(harness, "settings_difference", failing)
    else:
        guest.faults[injection] = error
    with pytest.raises(error_type) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is error  # the original error, unchanged
    guard = raised.value.settings_guard
    assert guard["status"] == "settings_restore_failed"
    assert len(guard["write_log"]) == attempted and guard["writes_completed"] == completed
    assert [entry["completed"] for entry in guard["write_log"]] == [True] * completed + [False] * (attempted - completed)
    assert ("nothing was written" in guard["reason"]) == (attempted == 0)
    assert guard["final_reading"] == "read"
    record_ = json.loads((tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / "run_record.json")
                         .read_text(encoding="utf-8"))
    assert record_["settings_guard"] == guard and record_["route"] == "harness_error"


@pytest.mark.parametrize("primary", ["exception at write 2", "another instance before write 2"])
def test_settings_restore_failure_path_keeps_the_original_outcome_when_its_own_reading_is_interrupted(tmp_path,
                                                                                                    primary):
    guest = FakeGuest()
    # Readings: 1 before, 2 after; the restore stops before verifying, so 3 is the failure path's own reading.
    guest.faults[("query", 3)] = KeyboardInterrupt("invented, during the failure path's own reading")
    if primary.startswith("exception"):
        error = RuntimeError("invented failure")
        guest.faults[("write", 2)] = error
        with pytest.raises(RuntimeError) as raised:
            guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
        assert raised.value is error
        guard = raised.value.settings_guard
        assert guard["secondary_errors"][0].startswith("KeyboardInterrupt")
        assert guard["writes_completed"] == 1 and len(guard["write_log"]) == 2
    else:
        guest.foreign_on_tasklist[4] = [18000]
        with pytest.raises(KeyboardInterrupt) as raised:  # nothing else to raise: the cancellation is kept
            guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
        guard = raised.value.settings_guard
        assert guard["stopped_by"]["status"] == "foreign_instance_running"
        assert guard["writes_completed"] == 1 and len(guard["write_log"]) == 1
    assert guard["status"] == "settings_restore_failed" and guard["final_reading"].startswith("interrupted")
    assert "nothing was written" not in guard["reason"]


WRITE_FAILURES = {  # kind -> (how the write ends, the outcome logged, the error raised is the injected one)
    "reg reports failure": (dict(result=(1, "", "ERROR: Access is denied.")), "failed", False),
    "transport failure": (dict(result=(255, "", "PrlJob_GetRetCode: Invalid argument")), "unknown", False),
    "unexpected output": (dict(result=(0, "", "")), "unknown", False),
    "launch failure": (dict(error=OSError("invented launch failure")), "failed", True),
    "host timeout": (dict(error=subprocess.TimeoutExpired(["invented"], 600)), "unknown", True),
    "cancellation": (dict(error=KeyboardInterrupt("invented")), "unknown", True),
}


SETTLED = ("transport failure", "host timeout")  # on a set or a delete these are settled, not final


@pytest.mark.parametrize(("kind", "number"), [(kind, number) for kind in WRITE_FAILURES for number in (1, 2, 3, 4)
                                              if kind not in SETTLED])
def test_settings_restore_write_runs_once_and_its_failure_stops_the_restore(tmp_path, number, kind):
    how, outcome, injected = WRITE_FAILURES[kind]
    guest = FakeGuest()
    if "result" in how:
        guest.write_results[number] = how["result"]
        expected_type = harness.RestoreWriteFailed
    else:
        guest.faults[("write", number)] = how["error"]
        expected_type = type(how["error"])
    with pytest.raises(expected_type) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    if injected:
        assert raised.value is how["error"]  # the first error, unchanged
    else:
        assert raised.value.outcome == outcome
    guard = raised.value.settings_guard
    assert guard["status"] == "settings_restore_failed" and guard["final_reading"] == "read"
    log = guard["write_log"]
    assert len(log) == number and guard["writes_completed"] == number - 1
    assert [entry["outcome"] for entry in log] == ["completed"] * (number - 1) + [outcome]
    assert all(entry["attempts"] == 1 for entry in log)
    # Written once, and nothing after the failed write.
    assert len(guest.writes) == number and len(set(guest.writes)) == number
    assert ("nothing was written" in guard["reason"]) is False


TRANSPORT = (255, "", "PrlJob_GetRetCode: Invalid argument")


def _write_steps(tmp_path) -> list[tuple[str, int]]:
    record_ = json.loads((tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / "run_record.json")
                         .read_text(encoding="utf-8"))
    return [step["name"] for step in record_["steps"] if step.get("mutating") or "readback" in step["name"]
            or "before_retry" in step["name"]]


@pytest.mark.parametrize("how", ["transport", "timeout"])
def test_settings_restore_retries_a_value_set_once_when_the_read_back_shows_it_did_not_take(tmp_path, how):
    guest = FakeGuest()
    before = dict(guest.values)
    if how == "transport":
        guest.write_results[2] = TRANSPORT
    else:
        guest.faults[("write", 2)] = subprocess.TimeoutExpired(["invented"], 20)
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "recent_files_restored" and guest.values == before
    entry = guard["write_log"][1]
    assert entry["attempts"] == 2 and entry["readback"] == "unchanged" and entry["completed"]
    assert [item["outcome"] for item in entry["attempt_log"]] == ["unknown", "completed"]
    assert _write_steps(tmp_path) == ["settings_restore_1", "settings_restore_2", "restore_check_before_retry_2",
                                      "settings_readback_2", "settings_restore_2_attempt_2", "settings_restore_3",
                                      "settings_restore_4"]
    assert all(item["attempts"] == 1 for item in guard["write_log"] if item is not entry)


def test_settings_restore_completes_a_write_the_read_back_shows_already_applied(tmp_path):
    guest = FakeGuest()
    before = dict(guest.values)
    guest.write_results[2], guest.applied_anyway = TRANSPORT, {2}
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "recent_files_restored" and guest.values == before
    entry = guard["write_log"][1]
    assert entry["attempts"] == 1 and entry["readback"] == "target" and entry["completed"]
    assert len(guest.writes) == 4  # never issued again


@pytest.mark.parametrize("found", ["third value", "unreadable"])
def test_settings_restore_stops_when_the_read_back_shows_neither_value(tmp_path, found):
    guest = FakeGuest()
    guest.write_results[2] = TRANSPORT
    guest.readback[1] = ("REG_BINARY", "DEAD") if found == "third value" else "unreadable"
    with pytest.raises(harness.RestoreWriteFailed) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = raised.value.settings_guard
    assert guard["status"] == "settings_restore_failed" and len(guest.writes) == 2  # nothing issued again
    entry = guard["write_log"][1]
    assert entry["readback"] == found.replace(" ", "_") and entry["outcome"] == "unknown" and not entry["completed"]
    assert entry["attempts"] == 1


def _delete_entry(guard_result: dict, action: str = "delete") -> dict:
    return next(entry for entry in guard_result["write_log"] if entry["action"] == action)


def test_settings_restore_retries_a_value_delete_once_when_the_value_is_still_there(tmp_path):
    guest = FakeGuest()
    before = dict(guest.values)
    guest.write_results[4] = TRANSPORT  # write 4 deletes the value the run added
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "recent_files_restored" and guest.values == before
    entry = _delete_entry(guard)
    assert entry["attempts"] == 2 and entry["readback"] == "unchanged" and entry["completed"]
    assert [item["outcome"] for item in entry["attempt_log"]] == ["unknown", "completed"]
    assert _write_steps(tmp_path)[-3:] == ["restore_check_before_retry_4", "settings_readback_4",
                                           "settings_restore_4_attempt_2"]


def test_settings_restore_completes_a_delete_the_read_back_shows_already_absent(tmp_path):
    guest = FakeGuest()
    before = dict(guest.values)
    guest.write_results[4], guest.applied_anyway = TRANSPORT, {4}
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    entry = _delete_entry(guard)
    assert guard["status"] == "recent_files_restored" and guest.values == before
    assert entry["attempts"] == 1 and entry["readback"] == "target" and len(guest.writes) == 4


def test_settings_restore_stops_when_a_deleted_value_reads_back_different(tmp_path):
    guest = FakeGuest()
    guest.write_results[4] = TRANSPORT
    guest.readback[1] = ("REG_BINARY", "BEEF")  # neither absent nor the value the run left
    with pytest.raises(harness.RestoreWriteFailed):
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = _guard_file(tmp_path)
    entry = _delete_entry(guard)
    assert guard["status"] == "settings_restore_failed" and entry["readback"] == "third_value"
    assert entry["attempts"] == 1 and not entry["completed"] and len(guest.writes) == 4


def test_settings_restore_stops_when_the_second_delete_fails_too(tmp_path):
    guest = FakeGuest()
    guest.write_results[4] = TRANSPORT
    guest.write_results[5] = TRANSPORT  # attempt 2 of the same delete
    with pytest.raises(harness.RestoreWriteFailed):
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = _guard_file(tmp_path)
    entry = _delete_entry(guard)
    assert guard["status"] == "settings_restore_failed" and entry["attempts"] == 2 and not entry["completed"]
    assert [item["outcome"] for item in entry["attempt_log"]] == ["unknown", "unknown"] and len(guest.writes) == 5


@pytest.mark.parametrize("found", ["still there", "already absent", "different content", "second failure"])
def test_settings_restore_settles_a_subkey_delete(tmp_path, found):
    guest = FakeGuest()
    before = (dict(guest.values), set(guest.empty_keys))
    # The whole-key restore of `_wide_change` ends by deleting the subkey that appeared; make that write fail.
    probe, probe_dir = FakeGuest(), tmp_path / "probe"
    probe_dir.mkdir()
    guarded(probe_dir, probe, _wide_change, guard=KEY_GUARD)
    number = len(probe.writes)
    guest.write_results[number] = TRANSPORT
    if found == "already absent":
        guest.applied_anyway = {number}
    if found == "different content":
        guest.readback[1] = {(INVENTED_KEY + "\\Window\\Placement", "Last"): ("REG_BINARY", "FF")}
    if found == "second failure":
        guest.write_results[number + 1] = TRANSPORT
    run = tmp_path / "run"
    run.mkdir()
    if found in ("still there", "already absent"):
        guard = guarded(run, guest, _wide_change, guard=KEY_GUARD)["settings_guard"]
        entry = _delete_entry(guard, "delete_key")
        assert guard["status"] == "settings_restored" and (guest.values, guest.empty_keys) == before
        assert entry["completed"] and entry["attempts"] == (2 if found == "still there" else 1)
        assert entry["readback"] == ("unchanged" if found == "still there" else "target")
        return
    with pytest.raises(harness.RestoreWriteFailed):
        guarded(run, guest, _wide_change, guard=KEY_GUARD)
    guard = _guard_file(run)
    entry = _delete_entry(guard, "delete_key")
    assert guard["status"] == "settings_restore_failed" and not entry["completed"]
    if found == "different content":
        assert entry["readback"] == "third_value" and entry["attempts"] == 1
    else:
        assert entry["attempts"] == 2 and [item["outcome"] for item in entry["attempt_log"]] == ["unknown", "unknown"]


def test_settings_restore_stops_when_another_instance_appears_before_the_retry(tmp_path):
    guest = FakeGuest()
    guest.write_results[2] = TRANSPORT
    guest.foreign_on_tasklist[5] = [16000]  # 1 precondition, 2 post-run, 3-4 before writes 1-2, 5 before the retry
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "settings_restore_failed"
    assert guard["stopped_by"]["status"] == "foreign_instance_running"
    assert guest.counts["readback"] == 0 and len(guest.writes) == 2  # neither read back nor issued again
    assert guard["write_log"][1]["outcome"] == "unknown" and not guard["write_log"][1]["completed"]


def test_settings_restore_stops_when_the_second_attempt_fails_too(tmp_path):
    guest = FakeGuest()
    guest.write_results[2] = TRANSPORT
    guest.write_results[3] = TRANSPORT  # attempt 2 of the same write
    with pytest.raises(harness.RestoreWriteFailed) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = raised.value.settings_guard
    entry = guard["write_log"][1]
    assert guard["status"] == "settings_restore_failed" and entry["attempts"] == 2 and not entry["completed"]
    assert [item["outcome"] for item in entry["attempt_log"]] == ["unknown", "unknown"]
    assert len(guest.writes) == 3 and guest.counts["readback"] == 1  # at most two attempts, never a third
    assert _write_steps(tmp_path)[-1] == "settings_restore_2_attempt_2"


RECOVERY_STEPS = ["bookkeeping", "final reading", "final comparison", "reason", "record", "attachment"]


@pytest.mark.parametrize("where", RECOVERY_STEPS)
def test_settings_restore_recovery_keeps_the_primary_error_through_a_later_cancellation(tmp_path, monkeypatch, where):
    guest = FakeGuest()
    primary = RuntimeError("invented primary failure")
    guest.faults[("write", 2)] = primary
    late = KeyboardInterrupt(f"invented, during the {where}")
    if where == "bookkeeping":
        monkeypatch.setattr(harness, "_restore_log_summary", lambda result, state: (_ for _ in ()).throw(late))
    elif where == "final reading":
        guest.faults[("query", 3)] = late  # readings: before, after, then the failure path's own
    elif where == "final comparison":
        real, calls = harness.settings_difference, []

        def failing(before, after):
            calls.append(1)
            if len(calls) == 2:  # 1: the guard's comparison; 2: the failure path's
                raise late
            return real(before, after)

        monkeypatch.setattr(harness, "settings_difference", failing)
    elif where == "reason":
        monkeypatch.setattr(harness, "_restore_reason", lambda result, state: (_ for _ in ()).throw(late))
    elif where == "record":
        real_write = harness.write_private

        def failing_write(root, path, data):
            if Path(path).name == "run_record.json":
                raise late
            return real_write(root, path, data)

        monkeypatch.setattr(harness, "write_private", failing_write)
    else:
        monkeypatch.setattr(harness, "_attach_guard", lambda error, settings: (_ for _ in ()).throw(late))
    with pytest.raises(RuntimeError) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is primary  # never replaced by the later cancellation
    record_path = tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / "run_record.json"
    if where == "record":
        assert not record_path.exists()
        guard = raised.value.settings_guard
    else:
        guard = json.loads(record_path.read_text(encoding="utf-8"))["settings_guard"]
    assert guard["status"] == "settings_restore_failed"
    assert guard["write_log"] and guard["write_log"][-1]["outcome"] == "unknown"
    if where in ("bookkeeping", "final reading", "final comparison", "reason"):
        assert any(text.startswith("KeyboardInterrupt") for text in guard["secondary_errors"])
    elif where == "record":  # kept in the result attached to the raised error
        assert any(text.startswith("run record: KeyboardInterrupt") for text in guard["secondary_errors"])
    else:  # the attachment failed first, so the written record says so
        assert any(text.startswith("attachment: KeyboardInterrupt") for text in guard["secondary_errors"])
    # The final reading is attempted whatever failed before it.
    assert guest.counts["query"] == 3
    assert guard.get("final_reading") == ("interrupted" if where == "final reading" else "read")


def _record_of(tmp_path) -> dict:
    return json.loads((tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / "run_record.json")
                      .read_text(encoding="utf-8"))


@pytest.mark.parametrize("signum", [signal.SIGINT, signal.SIGTERM], ids=["SIGINT", "SIGTERM"])
def test_settings_guard_defers_a_real_signal_until_the_restore_is_recorded(tmp_path, signum):
    handler_before = signal.getsignal(signum)
    guest = FakeGuest()
    guest.signal_on_write[1] = signum  # a real Ctrl-C or termination request arrives while the first write runs
    with pytest.raises(KeyboardInterrupt) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    name = signal.Signals(signum).name
    assert f"deferred signal {name}" in str(raised.value)
    record_ = _record_of(tmp_path)
    guard = record_["settings_guard"]
    # The restore ran to its verified end and was recorded before the interrupt was acted on.
    assert guard["status"] == "recent_files_restored"
    assert [entry["completed"] for entry in guard["write_log"]] == [True] * 4
    assert guard["signal_deferral"] == "active" and guard["deferred_signals"] == [name]
    assert raised.value.settings_guard == guard
    assert signal.getsignal(signum) is handler_before  # the previous handler is back


def test_settings_guard_keeps_a_deferred_signal_beside_a_primary_error(tmp_path):
    handler_before = signal.getsignal(signal.SIGINT)
    guest = FakeGuest()
    guest.signal_on_write[1] = signal.SIGINT
    primary = RuntimeError("invented primary failure")
    guest.faults[("write", 2)] = primary
    with pytest.raises(RuntimeError) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is primary
    guard = raised.value.settings_guard
    assert guard["status"] == "settings_restore_failed" and guard["deferred_signals"] == ["SIGINT"]
    assert any(text.startswith("deferred signal: KeyboardInterrupt") for text in guard["secondary_errors"])
    assert signal.getsignal(signal.SIGINT) is handler_before


def test_settings_guard_reports_signal_deferral_unavailable_off_the_main_thread(tmp_path):
    outcome = {}

    def run():
        outcome["record"] = guarded(tmp_path, FakeGuest(), _open_model, guard=RESTORING_GUARD)

    worker = threading.Thread(target=run)
    worker.start()
    worker.join()
    guard = outcome["record"]["settings_guard"]
    assert guard["status"] == "recent_files_restored"
    assert guard["signal_deferral"] == "unavailable off the main thread"


def test_settings_guard_write_error_logging_interrupted_by_a_callee_keeps_the_primary(tmp_path, monkeypatch):
    guest = FakeGuest()
    primary = RuntimeError("invented primary failure")
    guest.faults[("write", 2)] = primary
    real_now = harness._now

    def now():
        if guest.raised:  # the first diagnostic after the write failed is interrupted
            guest.raised = False
            raise KeyboardInterrupt("invented, during the write's error logging")
        return real_now()

    monkeypatch.setattr(harness, "_now", now)
    with pytest.raises(RuntimeError) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is primary
    guard = raised.value.settings_guard
    assert guard["status"] == "settings_restore_failed"
    assert any(text.startswith("KeyboardInterrupt") for text in guard["secondary_errors"])
    assert [entry["outcome"] for entry in guard["write_log"]] == ["completed", "unknown"]
    assert "settings_restore_2" in [step["name"] for step in _record_of(tmp_path)["steps"]]  # the step is kept


def test_settings_guard_recovers_after_a_check_failure_and_an_interrupted_recovery_entry(tmp_path, monkeypatch):
    guest = FakeGuest()
    primary = RuntimeError("invented check failure")
    guest.faults[("tasklist", 3)] = primary  # the check before the first write: nothing logged yet

    def entering(*args, **kwargs):
        raise KeyboardInterrupt("invented, entering recovery")

    monkeypatch.setattr(harness, "_failed_restore", entering)
    with pytest.raises(RuntimeError) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is primary
    guard = _record_of(tmp_path)["settings_guard"]
    assert guard["status"] == "settings_restore_failed" and guard["write_log"] == []
    assert guard["final_reading"] == "read" and guest.counts["query"] == 3  # the final read still happened
    assert any(text.startswith("KeyboardInterrupt") for text in guard["secondary_errors"])
    assert guest.writes == []


def test_settings_guard_cancellation_in_the_process_query_still_reads_the_settings(tmp_path):
    guest = FakeGuest()
    interrupt = KeyboardInterrupt("invented, from the process query")
    guest.faults[("tasklist", 2)] = interrupt  # 1: precondition; 2: the post-run process query
    with pytest.raises(KeyboardInterrupt) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is interrupt
    assert guest.counts["query"] == 2  # the reading after the run was still taken
    guard = _record_of(tmp_path)["settings_guard"]
    assert guard["status"] == "owned_process_state_unknown" and guest.writes == []
    assert raised.value.settings_guard == guard


@pytest.mark.parametrize("first_failing", ["guest_stdout.txt", "run_record.json"])
def test_settings_guard_result_survives_the_first_record_write_failure(tmp_path, monkeypatch, first_failing):
    guest = FakeGuest()
    failure = OSError(f"invented failure writing {first_failing}")
    real_write = harness.write_private

    def failing_write(root, path, data):
        if Path(path).name == first_failing:
            raise failure
        return real_write(root, path, data)

    monkeypatch.setattr(harness, "write_private", failing_write)
    with pytest.raises(OSError) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert raised.value is failure
    guard = raised.value.settings_guard  # attached although no record was written
    assert guard["status"] == "recent_files_restored"
    assert [entry["completed"] for entry in guard["write_log"]] == [True] * 4
    assert guard["run_record_error"].startswith("OSError")
    assert not (tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / "run_record.json").exists()


def test_settings_guard_blocked_record_write_is_interruptible_by_a_signal(tmp_path, monkeypatch):
    guest = FakeGuest()
    real_write = harness.write_private
    released = {"after": None}

    def blocking_write(root, path, data):
        if Path(path).name == "run_record.json":
            deadline = time.monotonic() + 20.0  # a stuck write, released here only if the signal never arrives
            while time.monotonic() < deadline:
                time.sleep(0.02)
            released["after"] = "deadline"
        return real_write(root, path, data)

    monkeypatch.setattr(harness, "write_private", blocking_write)
    timer = threading.Timer(0.5, lambda: os.kill(os.getpid(), signal.SIGINT))
    started = time.monotonic()
    timer.start()
    try:
        with pytest.raises(KeyboardInterrupt) as raised:
            guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    finally:
        timer.cancel()
    elapsed = time.monotonic() - started
    assert elapsed < 5.0 and released["after"] is None  # interrupted within the bound, not held to the deadline
    guard = raised.value.settings_guard  # attached before the record I/O began
    assert guard["status"] == "recent_files_restored" and guard["run_record_error"].startswith("KeyboardInterrupt")


def _guard_file(tmp_path) -> dict:
    return json.loads((tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / "settings_guard.json")
                      .read_text(encoding="utf-8"))


SIGTERM_POINTS = {  # where a real SIGTERM arrives -> (how, status, raised type)
    "during the verification reading": ("query", 3, "recent_files_restored", KeyboardInterrupt),
    "during the check after the writes": ("tasklist", 7, "recent_files_restored", KeyboardInterrupt),
    "at the check before success, after the section": ("tasklist", 8, "settings_restore_failed",
                                                       harness.TerminationRequested),
    "during the record write": ("record", 0, "recent_files_restored", harness.TerminationRequested),
}


@pytest.mark.parametrize("where", list(SIGTERM_POINTS))
def test_settings_guard_keeps_the_write_log_through_a_real_sigterm(tmp_path, monkeypatch, where):
    kind, number, status, raised_type = SIGTERM_POINTS[where]
    handler_before = signal.getsignal(signal.SIGTERM)
    guest = FakeGuest()
    if kind == "record":
        real_write = harness.write_private

        def terminating_write(root, path, data):
            if Path(path).name == "run_record.json":
                signal.raise_signal(signal.SIGTERM)  # a termination request while the record is written
            return real_write(root, path, data)

        monkeypatch.setattr(harness, "write_private", terminating_write)
    else:
        guest.signal_on[(kind, number)] = signal.SIGTERM
    with pytest.raises(raised_type) as raised:
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert isinstance(raised.value, KeyboardInterrupt)  # a cancellation, never a silent end of the process
    saved = _guard_file(tmp_path)  # persisted on its own, before the run record
    assert saved["status"] == status and [entry["completed"] for entry in saved["write_log"]] == [True] * 4
    assert raised.value.settings_guard["write_log"] == saved["write_log"]
    if status == "recent_files_restored" and kind != "record":
        assert saved["deferred_signals"] == ["SIGTERM"]
    assert signal.getsignal(signal.SIGTERM) is handler_before  # restored when the round trip ends


# The reporting transitions after the guard's checks, named by their statement in `_round_trip_body`.
REPORTING_POINTS = {
    "merging the guard's deferred error": "if deferred is not None:",
    "attaching the guard result": "if error is not None and settings is not None:",
    "persisting the guard result": "if guarded and settings is not None:",
}


def _signal_at_statement(statement: str, signum: int, times: int = 1):
    """A tracer that raises a real signal when `_round_trip_body` reaches the line of `statement`."""

    import inspect
    lines, first = inspect.getsourcelines(harness._round_trip_body)
    found = [first + index for index, line in enumerate(lines) if line.strip() == statement]
    assert len(found) == 1, statement
    fired: list[int] = []

    def local(frame, event, arg):  # noqa: ARG001
        if event == "line" and frame.f_lineno == found[0] and len(fired) < times:
            fired.append(signum)
            signal.raise_signal(signum)
        return local

    def tracer(frame, event, arg):  # noqa: ARG001
        return local if frame.f_code is harness._round_trip_body.__code__ else None

    return tracer, fired


@pytest.mark.parametrize("where", list(REPORTING_POINTS))
def test_settings_guard_keeps_the_write_log_through_a_sigterm_while_reporting(tmp_path, where):
    handler_before = signal.getsignal(signal.SIGTERM)
    tracer, fired = _signal_at_statement(REPORTING_POINTS[where], signal.SIGTERM)
    guest = FakeGuest()
    sys.settrace(tracer)
    try:
        with pytest.raises(harness.TerminationRequested) as raised:
            guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    finally:
        sys.settrace(None)
    assert fired == [signal.SIGTERM] and len(guest.writes) == 4
    carried = raised.value.settings_guard  # the exception carries the log
    assert carried["status"] == "recent_files_restored"
    assert [entry["completed"] for entry in carried["write_log"]] == [True] * 4
    saved = _guard_file(tmp_path)  # and at least the guard result is on file
    assert saved["write_log"] == carried["write_log"] and saved["status"] == "recent_files_restored"
    assert saved["round_trip_error"].startswith("TerminationRequested")
    assert signal.getsignal(signal.SIGTERM) is handler_before


def test_settings_guard_log_stays_on_the_exception_when_its_file_write_is_interrupted_again(tmp_path, monkeypatch):
    real_write = harness.write_private

    def terminating_write(root, path, data):
        if Path(path).name == harness.GUARD_RESULT_FILE:
            signal.raise_signal(signal.SIGTERM)  # every attempt to write the guard result is interrupted
        return real_write(root, path, data)

    monkeypatch.setattr(harness, "write_private", terminating_write)
    tracer, _ = _signal_at_statement(REPORTING_POINTS["attaching the guard result"], signal.SIGTERM)
    guest = FakeGuest()
    sys.settrace(tracer)
    try:
        with pytest.raises(harness.TerminationRequested) as raised:
            guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    finally:
        sys.settrace(None)
    carried = raised.value.settings_guard
    assert [entry["completed"] for entry in carried["write_log"]] == [True] * 4
    assert carried["guard_result_file"].startswith("not written: TerminationRequested")
    assert not (tmp_path / "exchange" / "cases" / "cx-g" / "caepipe_out" / harness.GUARD_RESULT_FILE).exists()


def test_settings_guard_reads_retry_transport_failures_with_a_backoff(tmp_path, monkeypatch):
    waits = []
    monkeypatch.setattr(harness, "_backoff", waits.append)
    guest = FakeGuest()
    guest.flaky = {"tasklist": 5, "query": 3}  # the precondition's list succeeds at attempt 6, the reading at 4
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    attempts = {step["name"]: step["attempts"] for step in record_["steps"]}
    assert attempts["precondition_process_list"] == 6 and attempts["settings_before"] == 4
    assert waits == [2.0] * 8 and record_["settings_guard"]["status"] == "recent_files_restored"
    # Within the restore the read-back and the checks take the same patience; a write never does.
    second = tmp_path / "second"
    second.mkdir()
    guest = FakeGuest()
    guest.write_results[2] = TRANSPORT
    guest.flaky = {}

    def solve(guest_, argv, config):
        guest_.flaky = {"readback": 2}
        return _open_model(guest_, argv, config)

    record_ = guarded(second, guest, solve, guard=RESTORING_GUARD)
    assert record_["settings_guard"]["status"] == "recent_files_restored"
    attempts = {step["name"]: step["attempts"] for step in record_["steps"]}
    assert attempts["settings_readback_2"] == 3 and attempts["settings_restore_2_attempt_2"] == 1


def test_settings_guard_read_that_keeps_failing_stops_the_run(tmp_path, monkeypatch):
    monkeypatch.setattr(harness, "_backoff", lambda seconds: None)
    guest = FakeGuest()
    guest.flaky = {"tasklist": 6}
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    assert record_["route"] == "precondition_unknown" and guest.writes == [] and not guest.solved
    assert [(step["name"], step["attempts"]) for step in record_["steps"]] == [("precondition_process_list", 6)]
    # The attempts stay within the call's timeout: with 2 s between attempts and a 7 s call, four attempts at most.
    clock = {"t": 1000.0}
    monkeypatch.setattr(harness, "time", SimpleNamespace(monotonic=lambda: clock["t"]))
    monkeypatch.setattr(harness, "_backoff", lambda seconds: clock.__setitem__("t", clock["t"] + seconds))
    second = tmp_path / "second"
    second.mkdir()
    guest = FakeGuest()
    guest.flaky = {"tasklist": 6}
    record_ = guarded(second, guest, _open_model, guard=dataclasses.replace(RESTORING_GUARD, call_timeout_s=7.0))
    step = record_["steps"][0]
    assert record_["route"] == "precondition_unknown" and step["attempts"] == 4
    assert step["attempts_stopped_by"] == "deadline" and all(timeout <= 7.0 for _, timeout in guest.timeouts)


def test_settings_guard_calls_use_the_guard_timeout_not_the_solve_timeout(tmp_path):
    guest = FakeGuest()
    guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    solve = [timeout for command, timeout in guest.timeouts if command.startswith("powershell")]
    guard = [timeout for command, timeout in guest.timeouts if not command.startswith(("powershell", "(if", "copy"))]
    assert solve == [600] and guard and all(0 < timeout <= 20.0 for timeout in guard)
    assert {command.split(" ")[0] for command, _ in guest.timeouts} >= {"reg", "tasklist", "type"}

    # Configured: every guard call takes the configured timeout.
    second = tmp_path / "second"
    second.mkdir()
    guest = FakeGuest()
    guarded(second, guest, _open_model, guard=dataclasses.replace(RESTORING_GUARD, call_timeout_s=7.0))
    guard = [timeout for command, timeout in guest.timeouts if not command.startswith(("powershell", "(if", "copy"))]
    assert all(0 < timeout <= 7.0 for timeout in guard)


def test_settings_guard_read_that_times_out_stops_the_restore_with_nothing_written(tmp_path):
    guest = FakeGuest()
    guest.faults[("tasklist", 3)] = subprocess.TimeoutExpired(["invented"], 20)  # the check before write 1
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "settings_restore_failed" and guard["stopped_by"]["status"] == "precondition_unknown"
    assert guard["write_log"] == [] and "nothing was written" in guard["reason"] and guest.writes == []


def test_settings_guard_section_budget_stops_the_restore_with_the_log_kept(tmp_path, monkeypatch):
    clock = {"t": 1000.0}
    monkeypatch.setattr(harness, "time", SimpleNamespace(monotonic=lambda: clock["t"]))
    guest = FakeGuest()
    guest.tick = lambda: clock.__setitem__("t", clock["t"] + 30.0)  # every guest call takes 30 s
    with pytest.raises(harness.SectionBudgetExceeded):
        guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)  # budget 120 s: two checks and two writes
    saved = _guard_file(tmp_path)
    assert saved["status"] == "settings_restore_failed"
    assert saved["section_budget_exceeded"] == "restore_check_before_write_3"
    assert [entry["completed"] for entry in saved["write_log"]] == [True, True] and len(guest.writes) == 2
    # Within the section no call was given more time than remained.
    section = [timeout for command, timeout in guest.timeouts if command.startswith("reg add")]
    assert all(timeout <= 20.0 for timeout in section)


def test_settings_restore_verified_when_no_fault_is_injected(tmp_path):
    guest = FakeGuest()
    before = dict(guest.values)
    record_ = guarded(tmp_path, guest, _open_model, guard=RESTORING_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "recent_files_restored" and guard["restored_values"] == 4
    assert [entry["completed"] for entry in guard["write_log"]] == [True] * 4 and guest.values == before


def test_settings_guard_stops_when_the_restore_cannot_be_verified(tmp_path):
    guest = FakeGuest()
    guest.apply_writes = False  # the writes go nowhere: the third reading still differs

    def solve(guest_, argv, config):
        guest_.open_model("C:\\invented-work\\cx-g\\model.mod")
        return solved(guest_, argv, config)

    record_ = guarded(tmp_path, guest, solve, guard=RESTORING_GUARD)
    assert record_["settings_guard"]["status"] == "settings_restore_failed"
    assert record_["settings_guard"]["remaining_differences"]

    # A value that cannot be written back exactly is not written at all.
    second = tmp_path / "second"
    second.mkdir()
    guest = FakeGuest()
    guest.values[(INVENTED_KEY + "\\Recent list", "Odd")] = ("REG_MULTI_SZ", "a\\0b")

    def solve_odd(guest_, argv, config):
        del guest_.values[(INVENTED_KEY + "\\Recent list", "Odd")]
        return solved(guest_, argv, config)

    record_ = guarded(second, guest, solve_odd, guard=RESTORING_GUARD)
    assert record_["settings_guard"]["status"] == "settings_restore_failed"
    assert guest.writes == []


def test_parse_reg_query_keeps_empty_keys_and_refuses_what_it_does_not_understand():
    parsed = parse_reg_query("\r\nHKEY_CURRENT_USER\\Software\\X\r\n    A name    REG_BINARY    0A0B\r\n"
                             "    Empty    REG_SZ\r\n\r\nHKEY_CURRENT_USER\\Software\\X\\No values\r\n")
    assert parsed == {
        "keys": {"hkcu\\software\\x": "HKEY_CURRENT_USER\\Software\\X",
                 "hkcu\\software\\x\\no values": "HKEY_CURRENT_USER\\Software\\X\\No values"},
        "values": {("hkcu\\software\\x", "A name"): ("HKEY_CURRENT_USER\\Software\\X", "REG_BINARY", "0A0B"),
                   ("hkcu\\software\\x", "Empty"): ("HKEY_CURRENT_USER\\Software\\X", "REG_SZ", "")}}
    assert parse_reg_query("ERROR: The system was unable to find the specified registry key or value.") is None
    assert parse_reg_query("HKEY_CURRENT_USER\\Software\\X\r\nsomething unexpected\r\n") is None


# Invented `reg query <key> /s` output in the structure the guest prints: CRLF lines, a leading blank line, no
# header for the queried key while it holds no values, REG_BINARY data as hex.
ROOTLESS_READING = ("\r\n" + INVENTED_KEY + "\\Options\r\n    Use something    REG_BINARY    01\r\n\r\n"
                    + INVENTED_KEY + "\\Recent list\r\n    1    REG_BINARY    "
                    + _hex("C:\\work\\case.mod") + "\r\n    N    REG_BINARY    01000000\r\n\r\n\r\n")


def _snapshot_of(text: str, exit_status: int = 0):
    return harness._settings_snapshot(fake_config_with_guard(), lambda name, command: {
        "exit_status": exit_status, "stdout": text, "stderr": ""}, "settings_before")


def fake_config_with_guard() -> HarnessConfig:
    return SimpleNamespace(settings_guard=INVENTED_GUARD)


def test_settings_snapshot_accepts_a_queried_key_printed_without_a_header():
    snapshot = _snapshot_of(ROOTLESS_READING)
    root = harness._registry_key_id(INVENTED_GUARD.key)
    assert snapshot is not None and snapshot["keys"][root] == INVENTED_KEY  # implied by its printed subkeys
    assert set(snapshot["keys"]) == {root, root + "\\options", root + "\\recent list"}
    assert snapshot["values"][(root + "\\recent list", "N")] == (INVENTED_KEY + "\\Recent list", "REG_BINARY",
                                                                   "01000000")
    # Printed with values of its own, the queried key is read as printed; the result is the same shape.
    with_root = "\r\n" + INVENTED_KEY + "\r\n    Own    REG_DWORD    0x1\r\n" + ROOTLESS_READING
    assert set(_snapshot_of(with_root)["keys"]) == set(snapshot["keys"])
    # The same text with LF line endings (as decoded in text mode) reads the same.
    assert _snapshot_of(ROOTLESS_READING.replace("\r\n", "\n")) == snapshot


def test_settings_snapshot_still_refuses_what_it_cannot_place_or_parse():
    outside = ROOTLESS_READING + "HKEY_CURRENT_USER\\Software\\Elsewhere\r\n    A    REG_BINARY    00\r\n"
    assert _snapshot_of(outside) is None                    # a key not under the configured one
    assert _snapshot_of("\r\n\r\n") is None                   # no key printed at all
    assert _snapshot_of(ROOTLESS_READING, exit_status=1) is None
    assert _snapshot_of(ROOTLESS_READING + "    orphan text\r\n") is None  # a line that is not understood
    assert _snapshot_of(ROOTLESS_READING.replace("REG_BINARY    01\r\n", "REG_BINARY\t01\r\n")) is None
    sibling = ROOTLESS_READING.replace(INVENTED_KEY + "\\Options", INVENTED_KEY + "-other\\Options")
    assert _snapshot_of(sibling) is None                    # a name that merely starts like the configured key


def test_settings_guard_reads_a_reading_without_the_queried_key_header_as_unchanged(tmp_path):
    guest = FakeGuest()
    assert not guest.query().splitlines()[1] == INVENTED_KEY  # the fake prints no header for the queried key
    record_ = guarded(tmp_path, guest, solved, gui_fallback=False)
    assert record_["settings_guard"]["status"] == "unchanged"


KEY_GUARD = dataclasses.replace(INVENTED_GUARD, restore_scope="key")


def _wide_change(guest_, argv, config):
    """A solve that changes settings across the whole key: the recent-files list, a value in another subkey, a new
    subkey with a value, and the removal of an empty subkey."""
    guest_.open_model("C:\\work\\case.mod")
    guest_.values[(INVENTED_KEY + "\\Options", "Use something")] = ("REG_BINARY", "00")
    guest_.values[(INVENTED_KEY + "\\Window\\Placement", "Last")] = ("REG_BINARY", "2C000000")
    guest_.empty_keys.discard(INVENTED_KEY + "\\Empty sub")
    return solved(guest_, argv, config)


def test_settings_guard_key_scope_restores_inside_the_key_only_and_verifies(tmp_path):
    guest = FakeGuest()
    before = (dict(guest.values), set(guest.empty_keys))
    record_ = guarded(tmp_path, guest, _wide_change, guard=KEY_GUARD)
    guard = record_["settings_guard"]
    assert guard["status"] == "settings_restored" and (guest.values, guest.empty_keys) == before
    assert [entry["completed"] for entry in guard["write_log"]] == [True] * len(guest.writes)
    actions = [entry["action"] for entry in guard["write_log"]]
    assert actions[0] == "create_key" and actions[-1] == "delete_key" and "set" in actions and "delete" in actions
    # Every write names a key strictly inside the configured key: never the key itself, its parent or a sibling.
    named = [re.match(r'reg (?:add|delete) "([^"]+)"', command)[1] for command in guest.writes]
    assert named and all(key.startswith(INVENTED_KEY + "\\") for key in named)
    # The new subkey goes as a whole (its value is not deleted on its own); the removed empty one is created again.
    assert f'reg delete "{INVENTED_KEY}\\Window\\Placement" /f' in guest.writes
    assert f'reg add "{INVENTED_KEY}\\Empty sub" /f' in guest.writes
    assert not any("Placement" in command and "/v" in command for command in guest.writes)
    # The checks ran before each write and after the last, and the key was read a third time.
    assert guest.counts["query"] == 3 and guest.counts["tasklist"] >= len(guest.writes) + 3


def test_settings_guard_recent_files_scope_still_writes_nothing_for_a_wider_change(tmp_path):
    guest = FakeGuest()
    record_ = guarded(tmp_path, guest, _wide_change, guard=RESTORING_GUARD)
    assert record_["settings_guard"]["status"] == "settings_changed" and guest.writes == []


def test_restore_plan_refuses_anything_outside_the_configured_key_before_any_write(tmp_path):
    root = harness._registry_key_id(INVENTED_GUARD.key)

    def snapshot(extra_values=(), extra_keys=()):
        values = {(harness._registry_key_id(key), name): (key, kind, data) for key, name, kind, data in (
            (INVENTED_KEY + "\\Options", "Use something", "REG_BINARY", "01"), *extra_values)}
        keys = {harness._registry_key_id(key): key for key in (INVENTED_KEY, INVENTED_KEY + "\\Options",
                                                              *extra_keys, *(v[0] for v in extra_values))}
        return {"keys": keys, "values": values}

    before = snapshot()
    inside = snapshot([(INVENTED_KEY + "\\Options", "New", "REG_BINARY", "02")])
    assert [entry["action"] for entry in harness._restore_plan(before, inside, root, whole_key=True)] == ["delete"]
    parent = INVENTED_KEY.rsplit("\\", 1)[0]
    for outside in ([(parent, "Up", "REG_BINARY", "02")],                                  # the parent's value
                    [(INVENTED_KEY + "-other\\Options", "X", "REG_BINARY", "02")],        # a sibling's subkey
                    [("HKEY_CURRENT_USER\\Software\\Elsewhere", "Y", "REG_BINARY", "02")]):
        with pytest.raises(ValueError, match="outside the restore scope"):
            harness._restore_plan(before, snapshot(outside), root, whole_key=True)
        with pytest.raises(ValueError, match="outside the restore scope"):  # also mixed with a change inside
            harness._restore_plan(inside, snapshot(outside), root, whole_key=True)
    # The configured key itself is never created or deleted.
    without_root = {"keys": {k: v for k, v in before["keys"].items() if k != root}, "values": before["values"]}
    with pytest.raises(ValueError, match="outside the restore scope"):
        harness._restore_plan(before, without_root, root, whole_key=True)
    # A name a guest command could not carry exactly is refused too.
    odd = snapshot([(INVENTED_KEY + "\\Options", "a&b", "REG_BINARY", "02")])
    with pytest.raises(ValueError):
        harness._restore_plan(before, odd, root, whole_key=True)

    # In a round trip a reading that prints a key outside the configured one is no reading: nothing is written.
    guest = FakeGuest()

    def outside_solve(guest_, argv, config):
        guest_.values[("HKEY_CURRENT_USER\\Software\\Invented Vendor\\Tool\\Other", "Z")] = ("REG_BINARY", "01")
        return _wide_change(guest_, argv, config)

    record_ = guarded(tmp_path, guest, outside_solve, guard=KEY_GUARD)
    assert record_["settings_guard"]["status"] == "settings_check_failed" and guest.writes == []


def test_restore_recorded_settings_goes_through_the_restore_routine(tmp_path):
    guest = FakeGuest()

    def value_changes(guest_, argv, config):  # values only, one of them new, across two subkeys
        guest_.open_model("C:\\work\\case.mod")
        guest_.values[(INVENTED_KEY + "\\Options", "Use something")] = ("REG_BINARY", "00")
        return solved(guest_, argv, config)

    record_ = guarded(tmp_path, guest, value_changes, guard=INVENTED_GUARD)  # detect only: nothing restored
    recorded = [entry for entry in record_["settings_guard"]["changed_values"] if "value" in entry]
    second = tmp_path / "second"
    second.mkdir()
    config = dataclasses.replace(fake_config(second), settings_guard=KEY_GUARD)
    # Changed since the record: nothing is written.
    guest.values[(INVENTED_KEY + "\\Options", "Use something")] = ("REG_BINARY", "07")
    stopped = harness.restore_recorded_settings(config, recorded, runner=guest.runner(solved, config))
    assert stopped["status"] == "changed_since_record" and guest.writes == []
    assert [item["value"] for item in stopped["values"]] == [INVENTED_KEY + "\\Options\\Use something"]
    guest.values[(INVENTED_KEY + "\\Options", "Use something")] = ("REG_BINARY", "00")
    # Without the whole-key scope it is not enabled.
    narrow = dataclasses.replace(config, settings_guard=RESTORING_GUARD)
    assert harness.restore_recorded_settings(narrow, recorded, runner=guest.runner(solved, narrow))["status"] == \
        "not_enabled"
    result = harness.restore_recorded_settings(config, recorded, runner=guest.runner(solved, config))
    assert result["status"] == "settings_restored"
    assert {item["status"] for item in result["values"]} == {"restored_and_verified"}
    assert len(result["values"]) == len(recorded) and len(result["write_log"]) == len(recorded)
    for entry in recorded:  # every recorded value is back at its before-value
        key, name = entry["value"].rsplit("\\", 1)
        found = guest.values.get((key, name))
        assert (None if found is None else {"type": found[0], "data": found[1]}) == entry["before"]
    assert [step["name"] for step in result["steps"]][:2] == ["precondition_process_list", "settings_current"]


def test_pilot_authored_dir_allows_the_gui_fallback_only_with_the_whole_key_restore(tmp_path, monkeypatch):
    seen = []
    for guard, expected in ((RESTORING_GUARD, False), (KEY_GUARD, True)):
        place = tmp_path / guard.restore_scope
        place.mkdir()
        config = dataclasses.replace(fake_config(place), settings_guard=guard)
        directory = authored_dir(config.exchange_root, ("cx-01-invented",))
        monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools, c=config: c)
        monkeypatch.setattr(run_pilot, "check_no_foreign_instance", lambda config_: {"status": "clear"})
        monkeypatch.setattr(run_pilot, "run_authored_case",
                            lambda case, config_, **kw: seen.append(kw["gui_fallback"]) or True)
        assert run_pilot.main(["--authored", "--authored-dir", str(directory)], environ={}) == 0
        assert seen[-1] is expected


def test_harness_config_reads_and_checks_the_settings_guard(tmp_path):
    base = {"vm_name": "invented-vm", "guest_executable": "C:\\Invented\\tool.exe",
            "guest_exchange_root": "Z:\\invented", "guest_work_root": "C:\\invented-work",
            "license_and_environment_responsibility_acknowledged": True}
    path = git_only_path(tmp_path)

    def place(name: str) -> Path:
        (tmp_path / name).mkdir()
        return tmp_path / name

    good = exchange(place("a"), config=base | {"settings_guard": {
        "key": "HKCU\\Software\\Invented Vendor\\Tool\\1.0", "recent_files_subkey": "Recent list"}})
    resolved = resolve_harness_config({ENV_VAR: str(good), "PATH": path}, require_vm_tools=False)
    assert resolved.settings_guard == INVENTED_GUARD
    restoring = exchange(place("r"), config=base | {"settings_guard": {
        "key": "HKCU\\Software\\Invented Vendor\\Tool\\1.0", "recent_files_subkey": "Recent list",
        "restore_recent_files": True}})
    assert resolve_harness_config({ENV_VAR: str(restoring), "PATH": path},
                                  require_vm_tools=False).settings_guard == RESTORING_GUARD
    assert (RESTORING_GUARD.call_timeout_s, RESTORING_GUARD.section_budget_s) == (20.0, 120.0)  # the defaults
    limited = exchange(place("l"), config=base | {"settings_guard": {
        "key": "HKCU\\Software\\Invented Vendor\\Tool\\1.0", "call_timeout_s": 7, "section_budget_s": 45.5}})
    resolved = resolve_harness_config({ENV_VAR: str(limited), "PATH": path}, require_vm_tools=False).settings_guard
    assert (resolved.call_timeout_s, resolved.section_budget_s) == (7.0, 45.5)
    assert resolved.restore_scope == "recent_files" and not resolved.restores_whole_key  # the default
    whole = exchange(place("k"), config=base | {"settings_guard": {
        "key": "HKCU\\Software\\Invented Vendor\\Tool\\1.0", "recent_files_subkey": "Recent list",
        "restore_scope": "key"}})
    assert resolve_harness_config({ENV_VAR: str(whole), "PATH": path},
                                  require_vm_tools=False).settings_guard == KEY_GUARD
    for number, bad in enumerate(({"key": "HKLM\\Software\\X", "recent_files_subkey": "R"},
                                  {"key": "HKCU\\Software\\X\" & del", "recent_files_subkey": "R"},
                                  {"key": "HKCU\\Software\\X", "recent_files_subkey": "R\\S"}, "HKCU\\Software\\X",
                                  {"key": "HKCU\\Software\\X", "restore_recent_files": True},
                                  {"key": "HKCU\\Software\\X", "recent_files_subkey": "R",
                                   "restore_recent_files": "yes"},
                                  {"key": "HKCU\\Software\\X", "call_timeout_s": 0},
                                  {"key": "HKCU\\Software\\X", "call_timeout_s": True},
                                  {"key": "HKCU\\Software\\X", "section_budget_s": "120"},
                                  {"key": "HKCU\\Software\\X", "section_budget_s": -1},
                                  {"key": "HKCU\\Software\\X", "restore_scope": "everything"},
                                  {"key": "HKCU\\Software\\X", "restore_scope": None})):
        root = exchange(place(f"b{number}"), config=base | {"settings_guard": bad})
        assert "settings_guard must give key" in skip_reason({ENV_VAR: str(root), "PATH": path})
    plain = resolve_harness_config({ENV_VAR: str(exchange(place("c"))), "PATH": path}, require_vm_tools=False)
    assert plain.settings_guard is None


def authored_dir(root: Path, case_ids: tuple[str, ...] = ("cx-01-invented",), name: str = "authored-private") -> Path:
    """An invented authored-case directory inside the exchange folder, laid out as validation/.../authored/."""
    directory = root / name
    directory.mkdir()
    for case_id in case_ids:
        (directory / f"{case_id}.mbf").write_bytes(b"INVENTED AUTHORED FIXTURE\r\n" + case_id.encode() + b"\r\n")
    (directory / "manifest.json").write_text(json.dumps({
        "document_kind": run_pilot.AUTHORED_MANIFEST_KIND,
        "cases": [{"case_id": case_id, "file": f"{case_id}.mbf"} for case_id in case_ids],
    }), encoding="utf-8")
    return directory


def test_pilot_authored_dir_runs_the_cases_of_a_directory_inside_the_exchange_folder(tmp_path, monkeypatch):
    config = dataclasses.replace(fake_config(tmp_path), settings_guard=INVENTED_GUARD)
    directory = authored_dir(config.exchange_root, ("cx-01-invented", "cx-02-invented"))
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    calls = []

    def round_trip(config_, case_id, mbf_text, **kwargs):
        sha = hashlib.sha256(mbf_text.encode("ascii")).hexdigest()
        calls.append(("round_trip", case_id, mbf_text, kwargs))
        outcome = outcomes.pop(0)
        if isinstance(outcome, BaseException):
            raise outcome
        return {"route": "csv_from_batch", "mbf": {"sha256": sha}, "guest_mbf_sha256": sha} | outcome

    def export(config_, case_id, mbf_sha256):
        calls.append(("export", case_id))
        return {"route": "mbf_exported"}

    real = run_pilot.run_authored_case
    monkeypatch.setattr(run_pilot, "run_authored_case",
                        lambda case, config_, **kw: real(case, config_, round_trip=round_trip, export=export, **kw))
    session = {"status": "clear"}
    monkeypatch.setattr(run_pilot, "check_no_foreign_instance", lambda config_: session)
    clean = {"settings_guard": {"status": "unchanged"}}
    # An --authored-dir run never exports and never lets the guest fall back to the GUI.
    outcomes = [clean]
    assert run_pilot.main(["--authored", "--authored-dir", str(directory), "--case", "cx-02-invented"],
                          environ={}) == 0
    assert calls == [("round_trip", "cx-02-invented", "INVENTED AUTHORED FIXTURE\r\ncx-02-invented\r\n",
                      {"gui_fallback": False})]

    def first_case_only(outcome) -> list:
        calls.clear()
        outcomes[:] = [outcome, clean]
        assert run_pilot.main(["--authored", "--authored-dir", str(directory)], environ={}) == 1
        return [call[:2] for call in calls]

    # Any difference in the saved settings stops the run before the next case.
    assert first_case_only({"settings_guard": {"status": "settings_changed"}}) == [("round_trip", "cx-01-invented")]
    # A rejected hash does not bypass the guard: the guard is enforced first.
    assert first_case_only({"guest_mbf_sha256": "0" * 64, "route": "input_hash_mismatch",
                            "settings_guard": {"status": "settings_check_failed"}}) == [("round_trip", "cx-01-invented")]
    # A round trip that raised stops the run, whatever its guard result.
    error = ContainmentError("invented")
    error.settings_guard = {"status": "unchanged"}
    assert first_case_only(error) == [("round_trip", "cx-01-invented")]
    # A batch run that left no CSV is not rescued by the GUI: the run stops.
    assert first_case_only({"route": "batch_no_csv", **clean}) == [("round_trip", "cx-01-invented")]
    # A host timeout whose guest processes are not confirmed ended stops the run.
    assert first_case_only({"route": "timeout", "settings_guard": {"status": "owned_process_running"}}) == [
        ("round_trip", "cx-01-invented")]
    # A restored recent-files list is clean: the run goes on.
    calls.clear()
    outcomes[:] = [{"settings_guard": {"status": "recent_files_restored"}}, clean]
    assert run_pilot.main(["--authored", "--authored-dir", str(directory)], environ={}) == 0
    assert len(calls) == 2
    # Another instance of the tool before the session: no case starts.
    calls.clear()
    session = {"status": "foreign_instance_running", "reason": "invented"}
    assert run_pilot.main(["--authored", "--authored-dir", str(directory)], environ={}) == 1
    assert calls == []
    session = {"status": "clear"}
    # A rejected hash with a clean guard is an ordinary failed case: the run goes on.
    calls.clear()
    outcomes[:] = [{"guest_mbf_sha256": "0" * 64, **clean}, clean]
    assert run_pilot.main(["--authored", "--authored-dir", str(directory)], environ={}) == 1
    assert [call[:2] for call in calls] == [("round_trip", "cx-01-invented"), ("round_trip", "cx-02-invented")]

    # --no-export applies to the tracked authored cases too.
    calls.clear()
    outcomes[:] = [clean]
    case = next(item for item in run_pilot.load_authored_cases() if item["case_id"] == "cpa-05-guide")
    sha = hashlib.sha256(run_pilot.authored_mbf_bytes(case)).hexdigest()
    assert real(case, config, round_trip=lambda c, i, t: round_trip(c, i, t) | {
        "mbf": {"sha256": sha}, "guest_mbf_sha256": sha}, export=export, no_export=True, emit=lambda line: None)
    assert [call[0] for call in calls] == ["round_trip"]


def test_pilot_authored_dir_refusals(tmp_path, monkeypatch, capsys):
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)
    monkeypatch.setattr(run_pilot, "run_authored_case", lambda *a, **k: pytest.fail("ran a case"))
    for argv, message in ((["--authored-dir", str(tmp_path)], "--authored-dir and --no-export need --authored"),
                          (["--no-export"], "--authored-dir and --no-export need --authored"),
                          (["--authored", "--export-only", "--no-export"], "--export-only is not allowed with"),
                          (["--authored", "--export-only", "--authored-dir", str(tmp_path)],
                           "--export-only is not allowed with")):
        with pytest.raises(SystemExit) as raised:
            run_pilot.main(argv, environ={})
        assert raised.value.code == 2
        assert message in capsys.readouterr().err

    # Without a settings guard in the configuration an --authored-dir run does not start.
    assert run_pilot.main(["--authored", "--authored-dir", str(tmp_path)], environ={}) == 2
    assert "--authored-dir runs need settings_guard in harness_config.json" in capsys.readouterr().err
    config = dataclasses.replace(config, settings_guard=INVENTED_GUARD)
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: config)

    outside = tmp_path / "outside"
    outside.mkdir()
    authored_dir(outside)
    assert run_pilot.main(["--authored", "--authored-dir", str(outside / "authored-private")], environ={}) == 2
    assert "is not inside the exchange folder" in capsys.readouterr().err

    (config.exchange_root / "linked").symlink_to(outside / "authored-private", target_is_directory=True)
    assert run_pilot.main(["--authored", "--authored-dir", str(config.exchange_root / "linked")], environ={}) == 2
    assert "is a symbolic link; refused" in capsys.readouterr().err

    clashing = authored_dir(config.exchange_root, ("cpa-05-guide", "cpc-01-cantilever-force"), name="clashing")
    assert run_pilot.main(["--authored", "--authored-dir", str(clashing)], environ={}) == 2
    assert "case id(s) cpa-05-guide, cpc-01-cantilever-force name tracked cases" in capsys.readouterr().err

    directory = authored_dir(config.exchange_root)
    assert run_pilot.main(["--authored", "--authored-dir", str(directory), "--case", "cx-09-absent"], environ={}) == 2
    assert "unknown authored case id(s): cx-09-absent" in capsys.readouterr().err

    # Without a configured harness nothing is read or run, as for the tracked cases.
    monkeypatch.setattr(run_pilot, "resolve_harness_config", lambda environ, require_vm_tools: HarnessSkip("invented"))
    assert run_pilot.main(["--authored", "--authored-dir", str(outside)], environ={}) == 0
    assert "CAEPIPE harness skipped: invented" in capsys.readouterr().out


def offline_case(tmp_path, monkeypatch, *, run_record=None, csv=True):
    """Prepare an offline run of the cantilever case with a stubbed SWBPIPE solve and an invented CSV."""
    case = next(item for item in run_pilot.load_cases() if item["case_id"] == "cpc-01-cantilever-force")
    monkeypatch.setattr(run_pilot, "solve_swbpipe", lambda runner, case_: (
        {"invented": True}, {"payload": {"mechanics_envelope": {"results": []}}}, ""))
    tmp_path.mkdir(exist_ok=True)
    config = fake_config(tmp_path)
    paths = case_paths(config, case["case_id"])
    if csv:
        shutil.copyfile(PARSER_FIXTURE, paths["caepipe_out"] / "model_results.csv")
    if run_record is not None:
        (paths["caepipe_out"] / "run_record.json").write_text(json.dumps(run_record), encoding="utf-8")
    lines = []
    ok = run_pilot.run_case(case, config, runner="unused", offline=True, hand_calcs=hand_calcs, emit=lines.append)
    return ok, lines, paths


def current_mbf_sha(case_id: str) -> str:
    case = next(item for item in run_pilot.load_cases() if item["case_id"] == case_id)
    text = render_caepipe_mbf_si(case["model"], title=case["title"],
                                 reference_temperature_c=case["reference_temperature_c"]).text
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def bound_record(sha: str, **extra):
    return {"document_kind": "openpipestress.caepipe_harness_run_record", "route": "csv_from_gui_print",
            "mbf": {"sha256": sha}, "guest_mbf_sha256": sha,
            "observed_csv_path": {"host": "caepipe_out/model_results.csv"}, **extra}


def test_offline_refuses_a_csv_without_a_run_record(tmp_path, monkeypatch):
    ok, lines, paths = offline_case(tmp_path, monkeypatch)
    assert ok is False and "route=unbound_output" in lines[0]
    assert not (paths["comparison"] / "comparison.json").exists() and not (paths["parsed"] / "caepipe_results.json").exists()


def test_offline_refuses_stale_and_guest_unverified_outputs(tmp_path, monkeypatch):
    sha = current_mbf_sha("cpc-01-cantilever-force")
    ok, lines, paths = offline_case(tmp_path / "stale", monkeypatch, run_record=bound_record("1" * 64))
    assert ok is False and "route=stale" in lines[0]
    assert not (paths["caepipe_in"] / "model.mbf").exists()  # the solved MBF is not overwritten
    ok, lines, _ = offline_case(tmp_path / "legacy", monkeypatch, run_record=bound_record(sha, guest_mbf_sha256=None))
    assert ok is False and "route=unbound_output" in lines[0]


def test_offline_bound_csv_is_compared_and_stdout_carries_status_only(tmp_path, monkeypatch):
    sha = current_mbf_sha("cpc-01-cantilever-force")
    ok, lines, paths = offline_case(tmp_path, monkeypatch, run_record=bound_record(sha))
    assert ok is True
    assert lines == ["cpc-01-cantilever-force: route=csv_from_gui_print; comparison record written: "
                     "cases/cpc-01-cantilever-force/comparison/comparison.json"]
    stored = json.loads((paths["comparison"] / "comparison.json").read_text(encoding="utf-8"))
    assert "by_outcome" in stored["summary"]
    id_map = json.loads((paths["caepipe_in"] / "id_map.json").read_text(encoding="utf-8"))
    assert id_map["mbf_sha256"] == sha  # binds the sidecar to the MBF bytes for the importer
    assert {"node_map", "element_map", "support_map", "load_map", "material_map"} <= set(id_map)


# --------------------------------------------------------------------------
# hand calculations and cases
# --------------------------------------------------------------------------
def test_cantilever_hand_calc_against_textbook_numbers():
    # Worked independently: D = 0.1683 m, Di = 0.1683 - 2 * 0.00711 = 0.15408 m.
    inertia = math.pi / 64.0 * (0.1683**4 - 0.15408**4)
    assert inertia == pytest.approx(1.1714e-5, rel=1e-3)
    tip = -2000.0 * 6.0**3 / (3.0 * 200.0e9 * inertia)
    values = hand_calcs.analytic_values("cpc-01-cantilever-force")
    assert values["n1_uy"]["value"] == pytest.approx(tip, rel=1e-12)
    assert values["n1_uy"]["value"] == pytest.approx(-0.06145, rel=1e-3)
    assert values["n1_rz"]["value"] == pytest.approx(-2000.0 * 36.0 / (2.0 * 200.0e9 * inertia), rel=1e-12)
    assert values["a0_fy"]["value"] == 2000.0 and values["a0_mz"]["value"] == 12000.0



def test_timoshenko_alternatives_against_independently_worked_numbers():
    # Worked independently: A = pi/4 (0.1683^2 - 0.15408^2) = 3.60046e-3 m2, I = 1.17162e-5 m4, G = E / 2.6,
    # kappa = 0.5 (shape factor 2): kappa G A = 0.5 * 7.6923e10 * 3.60046e-3 = 1.38479e8 N.
    area = math.pi / 4.0 * (0.1683**2 - 0.15408**2)
    inertia = math.pi / 64.0 * (0.1683**4 - 0.15408**4)
    assert area == pytest.approx(3.60046e-3, rel=1e-5)
    kga = 0.5 * 200.0e9 / 2.6 * area
    assert kga == pytest.approx(1.38479e8, rel=1e-5)
    ei = 200.0e9 * inertia
    # Cantilever tip: -2000 L^3 / (3 E I) - 2000 L / (kappa G A); shear parts 0.04333, 0.08666, 0.17331 mm.
    for case_id, span, worked in (("cpc-07-cantilever-span-3m", 3.0, -7.72498e-3),
                                  ("cpc-01-cantilever-force", 6.0, -6.153986e-2),
                                  ("cpc-06-cantilever-fault-injected", 6.0, -6.153986e-2),
                                  ("cpc-08-cantilever-span-12m", 12.0, -0.4917990)):
        alternative = hand_calcs.alternative_analytic_values(case_id)
        assert set(alternative) == {"shear-deformation"}
        row = alternative["shear-deformation"]["n1_uy"]
        assert row["basis"] == "Timoshenko" and row["unit"] == "m" and "ASSUMPTION" in row["assumption"]
        assert row["value"] == pytest.approx(worked, rel=2e-6), case_id
        assert row["value"] == pytest.approx(-2000.0 * span**3 / (3.0 * ei) - 2000.0 * span / kga, rel=1e-12)
        euler = hand_calcs.analytic_values(case_id)["n1_uy"]["value"]
        assert row["value"] - euler == pytest.approx(-2000.0 * span / kga, rel=1e-9)
    # Fixed-fixed midspan under w = -rho A g: w L^4 / (384 E I) + w L^2 / (8 kappa G A), L = 6 m.
    w = -7850.0 * area * 9.80665
    mid = hand_calcs.alternative_analytic_values("cpc-03-anchored-weight")["shear-deformation"]["mid_uy"]["value"]
    assert mid == pytest.approx(w * 6.0**4 / (384.0 * ei) + w * 36.0 / (8.0 * kga), rel=1e-12)
    assert mid == pytest.approx(-4.082191e-4, rel=2e-6)
    # Tee: each X-leg is a fixed-guided Timoshenko beam, Fy = -12 E I / ((1 + Phi) L^3) uy_J,
    # Phi = 12 E I / (kappa G A L^2) = 0.012691 at L = 4 m; shear softens every reaction.
    phi = 12.0 * ei / (kga * 16.0)
    assert phi == pytest.approx(0.012691, rel=1e-4)
    branch = {key: row["value"] for key, row in
              hand_calcs.alternative_analytic_values("cpc-05-branch-imposed-displacement")["shear-deformation"].items()}
    assert branch["a1_fy"] == pytest.approx(-12.0 * ei / ((1.0 + phi) * 64.0) * branch["j_uy"], rel=1e-12)
    assert branch["a1_fy"] + branch["a2_fy"] + branch["a3_fy"] == pytest.approx(0.0, abs=1e-9)
    euler = {key: row["value"] for key, row in hand_calcs.analytic_values("cpc-05-branch-imposed-displacement").items()}
    assert all(abs(branch[key]) < abs(euler[key]) for key in ("a1_fy", "a3_fy", "a3_mx"))
    # L-bend: the Timoshenko reactions still satisfy equilibrium and are smaller than the Euler-Bernoulli ones.
    bend = {key: row["value"] for key, row in
            hand_calcs.alternative_analytic_values("cpc-04-l-bend-thermal")["shear-deformation"].items()}
    assert bend["a_fx"] + bend["b_fx"] == pytest.approx(0.0, abs=1e-9)
    assert bend["a_my"] + bend["b_my"] + 3.0 * bend["b_fx"] - 4.0 * bend["b_fz"] == pytest.approx(0.0, abs=1e-6)
    euler = {key: row["value"] for key, row in hand_calcs.analytic_values("cpc-04-l-bend-thermal").items()}
    assert all(abs(bend[key]) < abs(euler[key]) for key in bend)
    assert hand_calcs.alternative_analytic_values("cpc-02-anchored-thermal") == {}


def test_thermal_and_weight_hand_calcs():
    area = math.pi / 4.0 * (0.1683**2 - 0.15408**2)
    thermal = hand_calcs.analytic_values("cpc-02-anchored-thermal")
    assert thermal["a0_fx"]["value"] == pytest.approx(200.0e9 * area * 1.2e-5 * 100.0, rel=1e-12)
    assert thermal["a1_fx"]["value"] == -thermal["a0_fx"]["value"]
    weight = hand_calcs.analytic_values("cpc-03-anchored-weight")
    w = 7850.0 * area * 9.80665
    assert weight["a0_fy"]["value"] == pytest.approx(w * 3.0, rel=1e-12)
    assert weight["a0_mz"]["value"] == pytest.approx(w * 36.0 / 12.0, rel=1e-12)
    assert weight["a1_mz"]["value"] == -weight["a0_mz"]["value"]


def test_l_bend_and_branch_hand_calcs_satisfy_equilibrium():
    values = {key: item["value"] for key, item in hand_calcs.analytic_values("cpc-04-l-bend-thermal").items()}
    assert values["a_fx"] + values["b_fx"] == pytest.approx(0.0, abs=1e-9)
    assert values["a_fz"] + values["b_fz"] == pytest.approx(0.0, abs=1e-9)
    # Moments about A (origin): My_A + My_B + (zB Fx_B - xB Fz_B) = 0 with B = (4, 0, 3).
    assert values["a_my"] + values["b_my"] + 3.0 * values["b_fx"] - 4.0 * values["b_fz"] == pytest.approx(0.0, abs=1e-6)
    assert values["a_fx"] > 0 and values["a_fz"] > 0  # the expanding L pushes away from A; A pushes back
    branch = hand_calcs.branch_solution(shear=False)
    assert branch["a1_fy"] + branch["a2_fy"] + branch["a3_fy"] == pytest.approx(0.0, abs=1e-9)
    # Moments about X through J: Mx_A1 + Mx_A2 + Mx_A3 + (y Fz - z Fy)_A3 = 0, z_A3 = 4.
    assert branch["a1_mx"] + branch["a2_mx"] + branch["a3_mx"] - 4.0 * branch["a3_fy"] == pytest.approx(0.0, abs=1e-6)
    assert branch["j_uy"] > 0 and branch["j_rx"] < 0  # J follows the +Y motion; the Z-leg slope is +dv/dz


PRINT_RESOLUTION = {"rel": 1e-6, "abs": {"m": 5e-7, "rad": 8.7266e-7, "N": 0.5, "N*m": 0.5}}


def test_case_files_render_within_the_configured_layout_row_threshold_and_match_hand_calcs():
    paths = sorted(CASES_DIR.glob("*.json"))
    assert [path.stem for path in paths] == sorted(hand_calcs.HAND_CALCS)
    for path in paths:
        case = json.loads(path.read_text(encoding="utf-8"))
        result = render_caepipe_mbf_si(case["model"], title=case["title"],
                                       reference_temperature_c=case["reference_temperature_c"])
        assert result.status == "written", (path.stem, result.diagnostics)
        assert result.layout_row_count <= 20, path.stem
        analytic = hand_calcs.analytic_values(case["case_id"])
        assert {item["id"] for item in case["quantities"]} == set(analytic), path.stem
        assert all(item["basis"] == "Euler-Bernoulli" for item in analytic.values())
        alternatives = hand_calcs.alternative_analytic_values(case["case_id"])
        covered = {}
        for declared in case["declared_differences"]:
            assert declared["cause"] == "documented_behavior_difference"
            assert declared["hypothesis"] in alternatives, path.stem
            assert declared["alternative_analytic"]["basis"] == "Timoshenko"
            assert declared["source_basis"] and all(item["url"].startswith("https://docs.sstcae.com/")
                                                    for item in declared["source_basis"])
            assert "predicted_caepipe_minus_swbpipe" not in declared
            covered.setdefault(declared["hypothesis"], set()).update(declared["applies_to"])
        assert covered == {hypothesis: set(rows) for hypothesis, rows in alternatives.items()}, path.stem
        for name in ("analytic_vs_swbpipe", "external", "caepipe_print_resolution"):
            assert case["tolerances"][name]["basis"] == "ASSUMPTION"
        assert {key: case["tolerances"]["caepipe_print_resolution"][key] for key in ("rel", "abs")} == PRINT_RESOLUTION
        assert case["provenance"]["origin"] == "invented"
    weight = json.loads((CASES_DIR / "cpc-03-anchored-weight.json").read_text(encoding="utf-8"))
    assert [item["applies_to"] for item in weight["declared_differences"]] == [["mid_uy"]]  # no gravity declaration


def test_span_series_cases_differ_from_the_cantilever_only_in_span():
    def normalized(case_id):
        case = json.loads((CASES_DIR / f"{case_id}.json").read_text(encoding="utf-8"))
        span = case["model"]["nodes"][1]["position"]["x"]
        text = json.dumps({key: value for key, value in case.items() if key not in ("case_id", "title", "purpose")})
        return span, json.loads(text.replace(case_id, "CASE").replace('"x": %r' % span, '"x": "SPAN"'))
    span_6, base = normalized("cpc-01-cantilever-force")
    assert span_6 == 6.0
    for case_id, span in (("cpc-07-cantilever-span-3m", 3.0), ("cpc-08-cantilever-span-12m", 12.0)):
        found, other = normalized(case_id)
        assert found == span and other == base, case_id


def test_shear_series_definition_matches_the_cases_and_is_resolvable_at_print_resolution():
    (definition,) = run_pilot.load_series()
    assert definition["hypothesis_id"] == "shear-deformation" and definition["quantity"] == "n1_uy"
    assert definition["expected_exponent"] == -2.0 and definition["expected_relative_gap_sign"] == 1
    assert definition["slope_tolerance"] == {**definition["slope_tolerance"], "abs": 0.05, "basis": "ASSUMPTION"}
    cases = {case["case_id"]: case for case in run_pilot.load_cases()}
    for member in definition["members"]:
        case = cases[member["case_id"]]
        assert case["model"]["nodes"][1]["position"]["x"] == member["span"]
        assert hand_calcs.CANTILEVER_SPANS[member["case_id"]] == member["span"]
        assert any(item["hypothesis"] == "shear-deformation" and "n1_uy" in item["applies_to"]
                   for item in case["declared_differences"])
    # Worst-case print rounding (5e-7 m on the external prover's value) of the shape-factor-2 gaps moves the endpoint
    # slope by about 0.011, well inside the 0.05 tolerance; at shape factor 1 (half the gap) by about 0.021.
    spans = sorted(member["span"] for member in definition["members"])
    gaps = {span: abs(hand_calcs.cantilever_timoshenko(span)["n1_uy"]["value"]
                      - hand_calcs.cantilever(span)["n1_uy"]["value"]) for span in (spans[0], spans[-1])}
    bound = sum(-math.log1p(-5e-7 / gap) for gap in gaps.values()) / math.log(spans[-1] / spans[0])
    assert bound == pytest.approx(0.0105, abs=5e-4)
    assert 2.0 * bound < definition["slope_tolerance"]["abs"]
    # The exact Timoshenko gaps follow the exponent.
    relative = {span: gaps[span] / abs(hand_calcs.cantilever(span)["n1_uy"]["value"]) for span in gaps}
    slope = math.log(relative[spans[-1]] / relative[spans[0]]) / math.log(spans[-1] / spans[0])
    assert slope == pytest.approx(-2.0, abs=1e-9)


def write_member_record(config, case_id, *, caepipe_mm):
    """An invented comparison record of a cantilever member, bound to the case's current MBF."""
    paths = case_paths(config, case_id)
    swb = hand_calcs.analytic_values(case_id)["n1_uy"]["value"]
    (paths["comparison"] / "comparison.json").write_text(json.dumps({
        "document_kind": "invented", "exported_model_hash": current_mbf_sha(case_id),
        "tolerance_basis": {ALTERNATIVE_CHECK: {"abs": {"m": 5e-7}}},
        "hypothesis_series": {},
        "quantities": [{"id": "n1_uy", "unit": "m", "values": {"swbpipe": swb, "caepipe": caepipe_mm * 1e-3},
                        "checks": {"analytic_vs_swbpipe": {"outcome": "matched_within_tolerance"}}}],
    }), encoding="utf-8")


def test_run_series_writes_a_private_record_and_rebuilds_declaring_cases(tmp_path, monkeypatch):
    config = fake_config(tmp_path)
    # Invented external-prover values: the Timoshenko tip deflections rounded to the 0.001 mm print resolution.
    for case_id in ("cpc-07-cantilever-span-3m", "cpc-01-cantilever-force", "cpc-08-cantilever-span-12m"):
        span = hand_calcs.CANTILEVER_SPANS[case_id]
        write_member_record(config, case_id,
                            caepipe_mm=round(hand_calcs.cantilever_timoshenko(span)["n1_uy"]["value"] * 1e3, 3))
    calls = []

    def fake_run_case(case, config_, **kwargs):
        calls.append((case["case_id"], kwargs["offline"], kwargs["hypothesis_series"]["shear-deformation"]["status"]))
        kwargs["emit"](f"{case['case_id']}: route=invented; comparison record written")
        return True

    monkeypatch.setattr(run_pilot, "run_case", fake_run_case)
    lines = []
    rebuilt = run_pilot.run_series(config, runner="unused", hand_calcs=hand_calcs, emit=lines.append)
    assert rebuilt == {"cpc-01-cantilever-force": True, "cpc-07-cantilever-span-3m": True,
                       "cpc-08-cantilever-span-12m": True}
    assert sorted(calls) == [(case_id, True, "consistent") for case_id in sorted(rebuilt)]
    stored = json.loads((config.exchange_root / "series" / "shear-deformation.json").read_text(encoding="utf-8"))
    assert stored["status"] == "consistent" and stored["fitted_slope"] == pytest.approx(-2.0, abs=0.011)
    assert stored["document_kind"] == "openpipestress.caepipe_correlation.series_record"
    assert lines[0] == "series shear-deformation: series record written: series/shear-deformation.json"
    assert all("consistent" not in line and "slope" not in line for line in lines)  # operational status only
    # A case without a stored record (cpc-03, cpc-04, cpc-05, cpc-06 here) is not rebuilt.
    assert "cpc-03-anchored-weight" not in rebuilt


def test_run_series_without_member_records_is_not_evaluated(tmp_path, monkeypatch):
    config = fake_config(tmp_path)
    monkeypatch.setattr(run_pilot, "run_case", lambda *a, **k: pytest.fail("nothing to rebuild"))
    lines = []
    assert run_pilot.run_series(config, runner="unused", hand_calcs=hand_calcs, emit=lines.append) == {}
    stored = json.loads((config.exchange_root / "series" / "shear-deformation.json").read_text(encoding="utf-8"))
    assert stored["status"] == "not_evaluated" and len(stored["reasons"]) == 3



def test_a_later_unbound_run_removes_the_earlier_export_and_records_why(tmp_path):
    config, out = solved_case(tmp_path, "cpc-e")
    assert run_caepipe_mbf_export(config, "cpc-e", MBF_SHA, runner=export_runner(out, []))["route"] == "mbf_exported"
    assert (out / "exported.mbf").is_file()
    other = "3" * 64
    entry = run_caepipe_mbf_export(config, "cpc-e", other, runner=lambda *a, **k: pytest.fail("prlctl called"))
    assert entry["route"] == "not_bound"
    assert not any(path.name.lower() in ("exported.mbf", "export_guest_log.json", "export_guest_stdout.txt",
                                         "export_guest_stderr.txt") for path in out.iterdir())
    stored = json.loads((out / "run_record.json").read_text(encoding="utf-8"))
    assert stored["mbf_export"]["route"] == "not_bound" and stored["mbf_export"]["solved_mbf_sha256"] == other
    assert stored["route"] == "csv_from_batch"  # the solve record itself is kept
