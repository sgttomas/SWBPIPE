#!/usr/bin/env python3
"""Release-quality gate-outcome record emitter (DEL-09-05, completion-plan E8).

Evaluates, for a named commit, the agent-checkable criteria of the five
`docs/RELEASE_QUALITY_GATES.md` gate families (Solver, Rule-engine, GUI,
Report-template, Mixed) from ALREADY-GOVERNED sources only — the commit-bound
`DEC-025` five-surface sweep artifacts (`validation/evidence/sweeps/`), the
governed `DEC-024`/`DEC-026` verification-tolerance classes and `DEC-046`
convergence-tolerance record (as codified in
`execution/_Decomposition/SOFTWARE_DECOMP.md` section 12), `DEC-058` scan
records (`validation/evidence/releases/`), and `DEC-060` coverage-telemetry
artifacts (`validation/evidence/coverage/`) — and writes one commit-bound,
schema-versioned JSON record per family to `validation/evidence/gates/`.

Each criterion is recorded as exactly one of `pass` / `fail` / `TBD`:

- `pass` / `fail` only where a governed artifact or governed decision record
  establishes the outcome (sweep-surface granularity is recorded explicitly
  where the evidence is an aggregate surface, not a per-suite result);
- `TBD` everywhere else, with the reason recorded. A criterion whose governed
  threshold is `TBD` stays `TBD` — this tool never invents, estimates, or
  defaults a value. Human-gated criteria (governance acceptance, waivers,
  wording/provenance review) are recorded `TBD` as not evaluable by an agent.

Boundary posture (F-PIP-2, PB-TBD-003, RGAP-007):

- A gate record documents measured gate outcomes at a commit. It is NOT a
  release, release-readiness, or publication claim and it mints no release
  label or status — the release-label vocabulary remains human-gated
  (`PB-TBD-003`, `docs/PROFESSIONAL_BOUNDARY.md`).
- No numeric coverage floor exists (`DEC-060`); this tool records the live
  clean-head coverage-telemetry artifact count and never promotes a floor —
  promotion needs at least five clean-head artifacts across at least two
  distinct commits AND a new owner-ruled decision row.
- The `DEC-025` five-surface sweep is read, never modified or re-run, by this
  tool; the sweep remains the merge gate and this emitter sits beside it.
- The process exit code reflects only record integrity (git binding, schema
  validity, writability). Criterion outcomes never affect the exit code: the
  record is evidence, not a gate.

Local-only: no network services, no signing, no publication credentials. Not
a professional approval, certification, sealing, authentication, or
code-compliance determination.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import subprocess
import sys
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 1
ARTIFACT_KIND = "openpipestress.release_gate_record"
GATE_DOC = "docs/RELEASE_QUALITY_GATES.md"
DECISION_BASIS = ["DEC-024", "DEC-025", "DEC-026", "DEC-046", "DEC-058", "DEC-060"]
DEFAULT_OUTPUT_DIR = "validation/evidence/gates"
SWEEP_DIR = "validation/evidence/sweeps"
COVERAGE_DIR = "validation/evidence/coverage"
SCAN_DIR = "validation/evidence/releases"
SCHEMA_FILENAME = "release_gate_record_schema.json"

SWEEP_ARTIFACT_KIND = "openpipestress.evidence_sweep_summary"
COVERAGE_ARTIFACT_KIND = "openpipestress.coverage_telemetry"
SCAN_ARTIFACT_KIND = "openpipestress.release_protected_content_scan_record"

STATUS_PASS = "pass"
STATUS_FAIL = "fail"
STATUS_TBD = "TBD"

# TBD reasons — every TBD criterion carries exactly one.
TBD_HUMAN_GATED = "human_gated"
TBD_GOVERNED_VALUE_TBD = "governed_value_tbd"
TBD_NOT_EVALUABLE = "not_evaluable_by_agent"
TBD_NO_EVIDENCE = "evidence_not_available"

BASIS_SWEEP = "dec025_sweep_surface"
BASIS_GOVERNED = "governed_decision_record"
BASIS_SCAN = "dec058_scan_record"
BASIS_HUMAN = "human_gated"
BASIS_TBD_POLICY = "governed_value_tbd"

FAMILY_IDS = ["solver", "rule_engine", "gui", "report_template", "mixed"]

BOUNDARY_NOTE = (
    "Commit-bound record of measured release-quality gate outcomes per "
    "docs/RELEASE_QUALITY_GATES.md, built from already-governed sources only. "
    "NOT a release, release-readiness, or publication claim; mints no release "
    "label or status (release-label vocabulary is human-gated, PB-TBD-003); "
    "not a professional approval, certification, sealing, authentication, or "
    "code-compliance determination. TBD criteria are unresolved: no value is "
    "invented, estimated, or defaulted for them."
)

RELEASE_LABELS_BLOCK = {
    "minted": False,
    "policy": (
        "This record does not evaluate, assign, or propose any release label "
        "or readiness status. The release-label vocabulary and final release "
        "policy language remain human-gated (PB-TBD-003, "
        "docs/PROFESSIONAL_BOUNDARY.md); the docs/RELEASE_QUALITY_GATES.md "
        "section 8 minimum engineering-beta condition is a human "
        "release-label consideration outside this record's scope."
    ),
}

# Governed tolerance sources named by solver criterion S6 and recorded in
# every record's inputs. Values quoted here are ALREADY governed (SOFTWARE_
# DECOMP.md section 12); nothing is invented.
GOVERNED_TOLERANCE_SOURCES = [
    {
        "source": "DEC-026 (supersedes DEC-024 Part 1; D-04 Option T-C + riders)",
        "record": "execution/_Decomposition/SOFTWARE_DECOMP.md section 12, DEC-026",
        "summary": (
            "Verification tolerances organized by reference-result class "
            "(analytic benchmark; cross-engine-exact; regression-golden-exact) "
            "as governed relative+absolute pairs; the analytic class is seeded "
            "at the measured 1.0e-9 relative where suites pass; unmeasured "
            "per-kind values remain TBD with the tolerance_policy_tbd_diagnostic "
            "active; fixture-local overrides may only tighten."
        ),
    },
    {
        "source": "DEC-046 (D-19 Option CV-B/B)",
        "record": "execution/_Decomposition/SOFTWARE_DECOMP.md section 12, DEC-046",
        "summary": (
            "Governed convergence-tolerance record keyed by nonlinear support "
            "class (gap, one-way, lift-off, friction): relative residual "
            "tolerance, absolute residual floor, governed max-iteration cap; "
            "unmeasured entries remain TBD and keep TolerancePolicyTbd active; "
            "loosening or cap-raising is a new governance event."
        ),
    },
    {
        "source": "docs/RELEASE_QUALITY_GATES.md section 10",
        "record": GATE_DOC,
        "summary": (
            "Final numerical tolerance policy for solver, stress, and nonlinear "
            "benchmarks: TBD. Performance thresholds and permitted variance "
            "policy: TBD. These stay TBD in this record; no value is invented."
        ),
    },
]


@dataclass(frozen=True)
class CriterionSpec:
    """One checkable (or explicitly non-checkable) gate criterion."""

    criterion_id: str
    section: str
    text: str
    basis_kind: str
    sweep_surfaces: tuple[str, ...] = ()
    governed_sources: tuple[str, ...] = ()
    tbd_reason: str | None = None
    notes: str | None = None


def _load_sibling(module_name: str):
    """Load a sibling tools/release script as a module (test-suite pattern)."""
    if module_name in sys.modules:
        return sys.modules[module_name]
    path = Path(__file__).resolve().parent / f"{module_name}.py"
    spec = importlib.util.spec_from_file_location(module_name, path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def _sweep_module():
    """Git-binding and runtime-capture helpers come from the DEC-025 sweep."""
    return _load_sibling("run_evidence_sweep")


SURFACE_GRANULARITY_NOTE = (
    "Evidence granularity is the DEC-025 sweep surface (aggregate suite "
    "outcome), not an isolated per-benchmark result; the named suites run "
    "inside the listed surfaces."
)


def common_criteria() -> list[CriterionSpec]:
    """docs/RELEASE_QUALITY_GATES.md section 3 items that are criteria.

    The remaining section 3 items (change identifier, commands and results,
    artifacts reviewed, known limitations and open TBDs, software-quality-only
    statement) are satisfied structurally by record fields, not criteria.
    """
    return [
        CriterionSpec(
            criterion_id="common.benchmark_provenance_documented",
            section="3",
            text=(
                "Benchmark source/provenance and redistribution status where "
                "examples are public."
            ),
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_NOT_EVALUABLE,
            notes=(
                "Per-case provenance lives in the validation manual (DEL-09-04) "
                "and witness records; confirming redistribution status is a "
                "human review act."
            ),
        ),
        CriterionSpec(
            criterion_id="common.scan_disposition_recorded",
            section="3",
            text=(
                "Protected-content, private-data, and real-secret scan "
                "disposition."
            ),
            basis_kind=BASIS_SCAN,
            tbd_reason=TBD_NO_EVIDENCE,
            notes=(
                "DEC-058 scan records are emitted UNSIGNED; per-finding "
                "dispositions and the release-gate signature are owner acts, "
                "so this criterion can never exceed TBD in an agent-emitted "
                "record."
            ),
        ),
        CriterionSpec(
            criterion_id="common.human_acceptance_or_waiver",
            section="3",
            text="Human governance acceptance or waiver record.",
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_HUMAN_GATED,
            notes="Only the human release authority can supply this record.",
        ),
    ]


def solver_criteria() -> list[CriterionSpec]:
    return [
        CriterionSpec(
            criterion_id="solver.mechanics_benchmarks_pass",
            section="4",
            text="Applicable mechanics benchmarks pass.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep",),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="solver.stress_recovery_benchmarks_pass",
            section="4",
            text="Applicable stress-recovery benchmarks pass.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep",),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="solver.nonlinear_support_regression_pass",
            section="4",
            text="Applicable nonlinear support regression checks pass.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="solver.unit_schema_checks_pass",
            section="4",
            text=(
                "Unit/schema checks pass where units or serialization are "
                "touched."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="solver.diagnostics_result_envelope_tested",
            section="4",
            text=(
                "Diagnostics and warning/result-envelope behavior are tested "
                "where affected."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep",),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="solver.tolerance_source_named_or_tbd",
            section="4",
            text="Tolerance source is named, or the tolerance remains TBD.",
            basis_kind=BASIS_GOVERNED,
            governed_sources=("DEC-026", "DEC-046"),
            notes=(
                "Satisfied by both arms: verification-tolerance sources are "
                "named (DEC-026 reference-result classes; DEC-046 convergence "
                "record) and every unmeasured or release-final value remains "
                "TBD per docs/RELEASE_QUALITY_GATES.md section 10. No numeric "
                "threshold is introduced by this record."
            ),
        ),
        CriterionSpec(
            criterion_id="solver.warning_visibility_preserved",
            section="4",
            text=(
                "Numerical warnings, non-convergence, and missing result "
                "states remain visible."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "desktop_vitest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
    ]


def rule_engine_criteria() -> list[CriterionSpec]:
    return [
        CriterionSpec(
            criterion_id="rule_engine.deterministic_evaluator_tests_pass",
            section="5",
            text="Deterministic evaluator tests pass for invented examples.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep",),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="rule_engine.required_input_completeness_pass",
            section="5",
            text="Required-input completeness checks pass.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="rule_engine.unit_aware_checks_pass",
            section="5",
            text="Unit-aware rule checks reject or flag incompatible units.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep",),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="rule_engine.sandbox_boundaries_verified",
            section="5",
            text=(
                "Sandboxing and arbitrary-code-execution boundaries are "
                "verified where touched."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="rule_engine.checksum_provenance_preserved",
            section="5",
            text="Rule-pack checksum and provenance fields are preserved.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="rule_engine.public_fixtures_invented_only",
            section="5",
            text="Public fixtures use invented or redistributable data only.",
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_NOT_EVALUABLE,
            notes=(
                "Data-provenance judgment; the protected-content lint gives "
                "partial machine evidence but redistribution review is human."
            ),
        ),
        CriterionSpec(
            criterion_id="rule_engine.user_rule_computation_wording",
            section="5",
            text=(
                "Pass/fail states are described as user-rule computations, "
                "not professional authentication."
            ),
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_NOT_EVALUABLE,
            notes=(
                "Wording review is human and adjacent to the human-gated "
                "release-label vocabulary (PB-TBD-003)."
            ),
        ),
    ]


def gui_criteria() -> list[CriterionSpec]:
    return [
        CriterionSpec(
            criterion_id="gui.missing_data_visibility",
            section="6",
            text=(
                "Missing solve data, missing rule-check data, and missing "
                "provenance are visible to users."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("desktop_vitest", "desktop_playwright_e2e"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="gui.warning_states_not_collapsed",
            section="6",
            text=(
                "Warnings, assumptions, nonlinear uncertainty, and "
                "result-envelope states are not collapsed into generic "
                "success states."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("desktop_vitest", "desktop_playwright_e2e"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="gui.workflow_tests_recorded",
            section="6",
            text="Workflow tests or screenshots are recorded where practical.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("desktop_vitest", "desktop_playwright_e2e"),
            notes=(
                "The commit-bound sweep artifact records the desktop unit and "
                "Playwright e2e workflow runs. "
            )
            + SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="gui.private_data_stays_local",
            section="6",
            text=(
                "Private data stays local and is not sent to unapproved "
                "services."
            ),
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_NOT_EVALUABLE,
            notes=(
                "Architecture/network-behavior audit is human; "
                "tests/security suites give partial machine evidence inside "
                "the python_pytest surface."
            ),
        ),
        CriterionSpec(
            criterion_id="gui.labels_avoid_compliance_claims",
            section="6",
            text=(
                "GUI labels avoid compliance, certification, endorsement, and "
                "professional approval claims."
            ),
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_NOT_EVALUABLE,
            notes=(
                "Wording review is human and adjacent to the human-gated "
                "release-label vocabulary (PB-TBD-003)."
            ),
        ),
        CriterionSpec(
            criterion_id="gui.browser_device_matrix_accessibility",
            section="6",
            text=(
                "Final browser/device matrix, accessibility threshold, and "
                "screenshot tooling."
            ),
            basis_kind=BASIS_TBD_POLICY,
            tbd_reason=TBD_GOVERNED_VALUE_TBD,
            notes=(
                "docs/RELEASE_QUALITY_GATES.md section 6 rules these TBD "
                "until governed by a release-policy decision; they stay TBD "
                "here and no value is invented."
            ),
        ),
    ]


def report_template_criteria() -> list[CriterionSpec]:
    return [
        CriterionSpec(
            criterion_id="report_template.reproducibility_checks_pass",
            section="7",
            text=(
                "Report reproducibility checks pass where the generator is "
                "touched."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="report_template.manifest_checksum_stability",
            section="7",
            text=(
                "Audit manifest and checksum fields remain stable or changes "
                "are explained."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep",),
            notes=(
                "The stability arm is test-derived; the changes-are-explained "
                "arm is change-scoped human review. "
            )
            + SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="report_template.provenance_warnings_visible",
            section="7",
            text=(
                "Warnings, assumptions, limitations, and rule-pack provenance "
                "remain visible."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="report_template.protected_content_lint_pass",
            section="7",
            text=(
                "Protected-content lint passes for public examples and "
                "templates."
            ),
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=(
                "Linter unit suites run in cargo_crate_sweep; the pytest "
                "surface exercises the linter contract "
                "(tests/test_report_protected_content_linter.py). "
            )
            + SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="report_template.boundary_notices_present",
            section="7",
            text="Professional-boundary notices are present.",
            basis_kind=BASIS_SWEEP,
            sweep_surfaces=("cargo_crate_sweep", "python_pytest"),
            notes=(
                "Notice-presence assertions run inside the listed surfaces; "
                "notice wording adequacy remains human review. "
            )
            + SURFACE_GRANULARITY_NOTE,
        ),
        CriterionSpec(
            criterion_id="report_template.no_private_data_in_public_fixtures",
            section="7",
            text=(
                "Private project data is not introduced into public fixtures "
                "or templates."
            ),
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_NOT_EVALUABLE,
            notes=(
                "Data-provenance judgment; the protected-content lint gives "
                "partial machine evidence but the private-data determination "
                "is human."
            ),
        ),
    ]


def mixed_only_criteria() -> list[CriterionSpec]:
    return [
        CriterionSpec(
            criterion_id="mixed.waiver_and_risk_disposition",
            section="2",
            text=(
                "Mixed changes run the union of applicable gates unless the "
                "human release authority records a waiver and risk "
                "disposition."
            ),
            basis_kind=BASIS_HUMAN,
            tbd_reason=TBD_HUMAN_GATED,
            notes=(
                "Only the human release authority can record a waiver "
                "(docs/RELEASE_QUALITY_GATES.md section 9); this record "
                "evaluates the full union and waives nothing."
            ),
        ),
    ]


FAMILY_DEFINITIONS = {
    "solver": (
        "Solver mechanics, loads, stress recovery, units, diagnostics, result "
        "envelopes (docs/RELEASE_QUALITY_GATES.md sections 2 and 4)."
    ),
    "rule_engine": (
        "Rule schema, expression evaluation, completeness, rule-pack "
        "lifecycle (docs/RELEASE_QUALITY_GATES.md sections 2 and 5)."
    ),
    "gui": (
        "Model editing, solve workflow, warning display, results viewing "
        "(docs/RELEASE_QUALITY_GATES.md sections 2 and 6)."
    ),
    "report_template": (
        "Public report templates, export fields, notices, manifest data "
        "(docs/RELEASE_QUALITY_GATES.md sections 2 and 7)."
    ),
    "mixed": (
        "Any change spanning families: the union of the Solver, Rule-engine, "
        "GUI, and Report-template gates plus the waiver criterion "
        "(docs/RELEASE_QUALITY_GATES.md section 2)."
    ),
}


def family_criteria(family_id: str) -> list[CriterionSpec]:
    """The criteria evaluated for one gate family (common items included)."""
    per_family = {
        "solver": solver_criteria,
        "rule_engine": rule_engine_criteria,
        "gui": gui_criteria,
        "report_template": report_template_criteria,
    }
    if family_id in per_family:
        return common_criteria() + per_family[family_id]()
    if family_id == "mixed":
        return (
            common_criteria()
            + mixed_only_criteria()
            + solver_criteria()
            + rule_engine_criteria()
            + gui_criteria()
            + report_template_criteria()
        )
    raise ValueError(f"unknown gate family: {family_id}")


def resolve_commit(commitish: str, root: Path) -> str | None:
    """Resolve a commit-ish to a full commit hash, or None."""
    try:
        completed = subprocess.run(
            ("git", "rev-parse", "--verify", f"{commitish}^{{commit}}"),
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout.strip() or None


def _load_json_artifacts(directory: Path, artifact_kind: str) -> list[tuple[Path, dict]]:
    """Load every parseable artifact of one kind from a directory, sorted."""
    if not directory.is_dir():
        return []
    artifacts: list[tuple[Path, dict]] = []
    for path in sorted(directory.glob("*.json")):
        try:
            body = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError):
            continue
        if isinstance(body, dict) and body.get("artifact") == artifact_kind:
            artifacts.append((path, body))
    return artifacts


def _is_clean_commit_bound(body: dict) -> bool:
    git_state = body.get("git") or {}
    return (
        bool(git_state.get("commit_hash"))
        and not git_state.get("status_capture_failed")
        and git_state.get("working_tree_dirty") is False
    )


def select_sweep_evidence(sweep_dir: Path, commit: str) -> dict:
    """Pick the commit-bound DEC-025 sweep evidence for the evaluated commit.

    Only clean-head artifacts qualify as commit-bound gate evidence; dirty or
    git-unverified artifacts at the commit are counted but never selected.
    Among clean matches the latest by started_utc wins.
    """
    artifacts = _load_json_artifacts(sweep_dir, SWEEP_ARTIFACT_KIND)
    matching = [
        (path, body)
        for path, body in artifacts
        if (body.get("git") or {}).get("commit_hash") == commit
    ]
    clean = [(p, b) for p, b in matching if _is_clean_commit_bound(b)]
    clean.sort(key=lambda item: item[1].get("started_utc") or "")
    selected = clean[-1] if clean else None
    evidence: dict = {
        "sweep_dir": sweep_dir.as_posix(),
        "artifacts_at_commit": len(matching),
        "clean_artifacts_at_commit": len(clean),
        "selected": None,
    }
    if selected is not None:
        path, body = selected
        evidence["selected"] = {
            "path": path.name,
            "commit_hash": body["git"]["commit_hash"],
            "started_utc": body.get("started_utc"),
            "overall_status": body.get("overall_status"),
            "surfaces": {
                surface.get("surface_id"): surface.get("status")
                for surface in body.get("surfaces", [])
            },
        }
    return evidence


def select_scan_evidence(scan_dir: Path, commit: str) -> dict:
    """List DEC-058 scan records bound to the evaluated commit (if any)."""
    artifacts = _load_json_artifacts(scan_dir, SCAN_ARTIFACT_KIND)
    matching = [
        path.name
        for path, body in artifacts
        if (body.get("git") or {}).get("commit_hash") == commit
    ]
    return {
        "scan_dir": scan_dir.as_posix(),
        "records_at_commit": matching,
    }


def coverage_floor_context(coverage_dir: Path) -> dict:
    """Record the live DEC-060 floor-promotion evidence base — never promote.

    Floor promotion requires at least five clean-head telemetry artifacts
    spanning at least two distinct commits AND a new owner-ruled decision
    row; this tool only counts and records.
    """
    artifacts = _load_json_artifacts(coverage_dir, COVERAGE_ARTIFACT_KIND)
    clean = [body for _, body in artifacts if _is_clean_commit_bound(body)]
    distinct_commits = {body["git"]["commit_hash"] for body in clean}
    prerequisites_met = len(clean) >= 5 and len(distinct_commits) >= 2
    return {
        "coverage_dir": coverage_dir.as_posix(),
        "clean_head_artifacts": len(clean),
        "distinct_clean_head_commits": len(distinct_commits),
        "promotion_prerequisites": (
            "at least five clean-head telemetry artifacts spanning at least "
            "two distinct commits per lane, then a NEW owner-ruled decision "
            "row (DEC-060)"
        ),
        "artifact_count_prerequisite_met": prerequisites_met,
        "promotion_performed": False,
        "numeric_floors": (
            "none exist (DEC-060); coverage is recorded, never blocking, and "
            "is not a criterion of any gate family in this record"
        ),
    }


def evaluate_criterion(spec: CriterionSpec, sweep_evidence: dict, scan_evidence: dict) -> dict:
    """Evaluate one criterion deterministically from governed evidence."""
    criterion = {
        "criterion_id": spec.criterion_id,
        "gate_doc_section": spec.section,
        "text": spec.text,
        "basis_kind": spec.basis_kind,
        "sweep_surfaces": list(spec.sweep_surfaces),
        "governed_sources": list(spec.governed_sources),
        "status": STATUS_TBD,
        "tbd_reason": spec.tbd_reason,
        "notes": spec.notes,
    }

    if spec.basis_kind == BASIS_SWEEP:
        selected = sweep_evidence.get("selected")
        if selected is None:
            criterion["tbd_reason"] = TBD_NO_EVIDENCE
            criterion["notes"] = (
                "No clean-head DEC-025 sweep artifact is bound to the "
                "evaluated commit; the criterion outcome is not evaluable. "
                f"({spec.notes})"
            )
            return criterion
        statuses = [
            selected["surfaces"].get(surface_id) for surface_id in spec.sweep_surfaces
        ]
        if any(status == "fail" for status in statuses):
            criterion["status"] = STATUS_FAIL
            criterion["tbd_reason"] = None
        elif all(status == "pass" for status in statuses):
            criterion["status"] = STATUS_PASS
            criterion["tbd_reason"] = None
        else:
            criterion["tbd_reason"] = TBD_NO_EVIDENCE
            criterion["notes"] = (
                "One or more required sweep surfaces were not_run or absent "
                "in the selected sweep artifact; the criterion outcome is "
                f"not evaluable. ({spec.notes})"
            )
        return criterion

    if spec.basis_kind == BASIS_GOVERNED:
        # The only governed-record criterion is tolerance-source naming: the
        # sources exist as accepted section-12 codifications, so the
        # named-or-TBD criterion is established without inventing any value.
        criterion["status"] = STATUS_PASS
        criterion["tbd_reason"] = None
        return criterion

    if spec.basis_kind == BASIS_SCAN:
        records = scan_evidence.get("records_at_commit", [])
        if records:
            criterion["notes"] = (
                f"DEC-058 scan record(s) bound to this commit: {records}. "
                "Records are UNSIGNED by construction; per-finding "
                "dispositions and sign-off remain owner acts, so the "
                "criterion stays TBD."
            )
            criterion["tbd_reason"] = TBD_HUMAN_GATED
        return criterion

    # BASIS_HUMAN / BASIS_TBD_POLICY: stays TBD as specified.
    return criterion


def status_counts(criteria: list[dict]) -> dict:
    return {
        "pass": sum(1 for c in criteria if c["status"] == STATUS_PASS),
        "fail": sum(1 for c in criteria if c["status"] == STATUS_FAIL),
        "TBD": sum(1 for c in criteria if c["status"] == STATUS_TBD),
    }


def build_family_record(
    family_id: str,
    evaluated_commit: str,
    sweep_evidence: dict,
    scan_evidence: dict,
    coverage_context: dict,
    emitted_git: dict,
    emitted_runtime: dict,
    started_utc: str,
) -> dict:
    criteria = [
        evaluate_criterion(spec, sweep_evidence, scan_evidence)
        for spec in family_criteria(family_id)
    ]
    return {
        "artifact": ARTIFACT_KIND,
        "schema_version": SCHEMA_VERSION,
        "decision_basis": DECISION_BASIS,
        "gate_doc": GATE_DOC,
        "boundary_note": BOUNDARY_NOTE,
        "gate_family": family_id,
        "gate_family_definition": FAMILY_DEFINITIONS[family_id],
        "evaluated_commit": evaluated_commit,
        "emitted": {
            "git": emitted_git,
            "runtime": emitted_runtime,
            "started_utc": started_utc,
        },
        "inputs": {
            "sweep_evidence": sweep_evidence,
            "scan_evidence": scan_evidence,
            "governed_tolerance_sources": GOVERNED_TOLERANCE_SOURCES,
        },
        "coverage_telemetry_context": coverage_context,
        "release_labels": dict(RELEASE_LABELS_BLOCK),
        "criteria": criteria,
        "status_counts": status_counts(criteria),
    }


def record_filename(family_id: str, started_utc: str, evaluated_commit: str, emitted_git: dict) -> str:
    """`GATE_<FAMILY>_<stamp>_<evaluated12>[flags].json`.

    The commit token is the EVALUATED commit; the dirty/unverified flag
    describes the EMITTING working tree (matching the sweep convention so a
    record emitted from a dirty tree says so in its name).
    """
    sweep = _sweep_module()
    if sweep.git_state_unverified(emitted_git):
        flag = "-gitunverified"
    elif emitted_git["working_tree_dirty"]:
        flag = "-dirty"
    else:
        flag = ""
    stamp = started_utc.replace("-", "").replace(":", "")
    stamp = stamp.split("+")[0] + "Z"
    return f"GATE_{family_id.upper()}_{stamp}_{evaluated_commit[:12]}{flag}.json"


def schema_path() -> Path:
    return Path(__file__).resolve().parent / SCHEMA_FILENAME


def validate_record(record: dict) -> list[str]:
    """Validate one record against the artifact schema (empty when valid)."""
    try:
        import jsonschema
    except ImportError:
        return [
            "jsonschema is not importable (see requirements-dev.txt); "
            "cannot validate the gate record"
        ]
    schema = json.loads(schema_path().read_text(encoding="utf-8"))
    validator = jsonschema.Draft202012Validator(schema)
    return [
        f"{'/'.join(str(part) for part in error.absolute_path) or '<root>'}: "
        f"{error.message}"
        for error in validator.iter_errors(record)
    ]


def write_record(record: dict, output_dir: Path, filename: str) -> Path:
    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / filename
    output_path.write_text(
        json.dumps(record, indent=2, sort_keys=False) + "\n", encoding="utf-8"
    )
    return output_path


def select_families(families_arg: str) -> list[str] | None:
    requested = [token.strip() for token in families_arg.split(",") if token.strip()]
    if not requested or any(token not in FAMILY_IDS for token in requested):
        return None
    return [family for family in FAMILY_IDS if family in set(requested)]


def print_plan(families: list[str], root: Path, commitish: str, execute: bool) -> None:
    mode = "execute" if execute else "dry-run"
    print(
        f"OpenPipeStress release gate records ({mode}) — DEL-09-05 / "
        f"{GATE_DOC} (documents outcomes; mints no release label)"
    )
    print(f"repo: {root}")
    print(f"evaluated commit-ish: {commitish}")
    print("")
    print(f"gate families: {len(families)}")
    for order, family in enumerate(families, start=1):
        criteria = family_criteria(family)
        print(f"{order}. {family} ({len(criteria)} criteria)")
        print(f"   {FAMILY_DEFINITIONS[family]}")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Evaluate the docs/RELEASE_QUALITY_GATES.md gate families for a "
            "commit from already-governed evidence and write one commit-bound "
            "record per family. Documents pass/fail/TBD outcomes only; never "
            "mints a release label; TBD thresholds stay TBD."
        )
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Evaluate and write records. Without this flag, only print the plan.",
    )
    parser.add_argument(
        "--commit",
        default="HEAD",
        help="Commit-ish whose gate outcomes are evaluated (default: HEAD).",
    )
    parser.add_argument(
        "--families",
        default=",".join(FAMILY_IDS),
        help=f"Comma-separated gate families (default: all of {','.join(FAMILY_IDS)}).",
    )
    parser.add_argument(
        "--output-dir",
        default=DEFAULT_OUTPUT_DIR,
        help=f"Record directory relative to the repo root (default: {DEFAULT_OUTPUT_DIR}).",
    )
    parser.add_argument(
        "--repo-root",
        default=str(ROOT),
        help="Repository root. Defaults to the root containing this script.",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.repo_root).resolve()
    if not root.exists():
        print(f"missing repository root: {root}", file=sys.stderr)
        return 2

    families = select_families(args.families)
    if families is None:
        print(
            f"unknown family selection {args.families!r}; known: "
            f"{','.join(FAMILY_IDS)}",
            file=sys.stderr,
        )
        return 2

    print_plan(families, root, args.commit, args.execute)
    sys.stdout.flush()

    if not args.execute:
        return 0

    evaluated_commit = resolve_commit(args.commit, root)
    if evaluated_commit is None:
        print(
            f"cannot resolve commit-ish {args.commit!r} in {root}; records "
            "must be commit-bound",
            file=sys.stderr,
        )
        return 1

    sweep = _sweep_module()
    emitted_git = sweep.collect_git_state(root)
    if sweep.git_state_unverified(emitted_git):
        print(
            "git state of the emitting tree could not be verified — records "
            "would not be honestly bound",
            file=sys.stderr,
        )
        return 1
    emitted_runtime = sweep.collect_runtime_versions(root)
    started_utc = datetime.now(timezone.utc).isoformat(timespec="seconds")

    sweep_evidence = select_sweep_evidence(root / SWEEP_DIR, evaluated_commit)
    scan_evidence = select_scan_evidence(root / SCAN_DIR, evaluated_commit)
    coverage_context = coverage_floor_context(root / COVERAGE_DIR)

    written: list[Path] = []
    for family in families:
        record = build_family_record(
            family,
            evaluated_commit,
            sweep_evidence,
            scan_evidence,
            coverage_context,
            emitted_git,
            emitted_runtime,
            started_utc,
        )
        errors = validate_record(record)
        if errors:
            print(
                f"[gate-records] {family}: record failed its own schema "
                "validation:",
                file=sys.stderr,
            )
            for error in errors:
                print(f"  - {error}", file=sys.stderr)
            return 1
        filename = record_filename(family, started_utc, evaluated_commit, emitted_git)
        output_path = write_record(record, root / args.output_dir, filename)
        written.append(output_path)
        counts = record["status_counts"]
        print(
            f"[gate-records] {family}: pass={counts['pass']} "
            f"fail={counts['fail']} TBD={counts['TBD']} -> "
            f"{output_path.relative_to(root).as_posix()}"
        )

    print(f"[gate-records] evaluated commit: {evaluated_commit}")
    if sweep_evidence["selected"] is None:
        print(
            "[gate-records] no clean-head DEC-025 sweep artifact at the "
            "evaluated commit — sweep-derived criteria are TBD"
        )
    else:
        print(
            "[gate-records] sweep evidence: "
            f"{sweep_evidence['selected']['path']} "
            f"(overall {sweep_evidence['selected']['overall_status']})"
        )
    print(
        "[gate-records] records document measured gate outcomes only; no "
        "release label or readiness status is minted (PB-TBD-003 is human)."
    )
    # Criterion outcomes never affect the exit code: the record is evidence,
    # not a gate. Integrity failures returned above are the only failures.
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
