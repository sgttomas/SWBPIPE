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
import shutil
import subprocess
import sys

import pytest


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
from core.handoff.caepipe_external.harness import (  # noqa: E402
    ENV_VAR,
    ContainmentError,
    _quote,
    last_json_object,
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
                        lambda case, config_, export_only: calls.append((case["case_id"], export_only)) or True)
    assert run_pilot.main(["--authored", "--case", "cpa-12-guide-gap"], environ={}) == 0
    assert run_pilot.main(["--authored", "--export-only", "--case", "cpa-12-guide-gap"], environ={}) == 0
    assert calls == [("cpa-12-guide-gap", False), ("cpa-12-guide-gap", True)]
    assert run_pilot.main(["--authored", "--case", "cpc-01-cantilever-force"], environ={}) == 2
    for mode in ("--offline", "--export-mbf"):
        with pytest.raises(SystemExit) as raised:
            run_pilot.main(["--authored", mode], environ={})
        assert raised.value.code == 2
    assert "--authored takes only --case and --export-only" in capsys.readouterr().err


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
