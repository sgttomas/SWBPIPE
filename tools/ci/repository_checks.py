"""The Repository checks job, as one list that the hosted job and an agent's local run share.

The `repository-checks` job of `.github/workflows/governance-harness.yml` sets up Python, Rust and
the check dependencies, then calls this script with the event, base and head it was given. Before a
push, from a clean checkout at the repository root:

    python3 tools/ci/repository_checks.py --event pull_request --base origin/main --head HEAD

The steps run in order and the first failing step ends the run with its exit status. Each step
validates; none refreshes or rewrites an inventory, digest or declaration. The two output files the
job has always written (`repository-ci-plan.json`, `deliverable-check.json`) are written at the
repository root.

Base for the documents and conflict steps: the workflow gives the pull request's or merge-queue
group's base; workflow dispatch gives none. A base that does not name a commit fails the run for a
merge-queue group, whose group can hold several pull requests, and is otherwise replaced by `HEAD^1`.
"""
from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PLAN = "repository-ci-plan.json"
DELIVERABLE_CHECK = "deliverable-check.json"
# The bounded shared-tool and routing regressions; `tests/test_ci_python.py` keeps this equal to
# `python_test_reads.REPOSITORY_CHECK_TESTS`.
REGRESSION_TARGETS = ("tools/software_workflow", "tools/deliverables", "tools/validation",
                      "tests/test_ci_numerical.py", "tests/test_ci_apt_sources.py", "tests/test_release_members.py",
                      "tests/test_release_candidate_scan.py", "tests/test_repository_history.py")
# The routing invariants: every test in tests/test_ci_python.py whose truth an edit outside tools/ci/ can change.
# tests/test_ci_python.py is selected only by tools/ci/ changes, so without this step such an invariant is green on
# the pull request that breaks it and red on main after it. The rest of that file tests the selector's own algorithm
# and stays scoped to tools/ci/ changes; tests/test_ci_python.py keeps the two sets a partition of its tests.
ROUTING_INVARIANT_TESTS = tuple(f"tests/test_ci_python.py::{name}" for name in (
    "test_repository_numerical_routing_checks_are_not_duplicated_in_product_job",
    "test_merge_group_selects_python_tests_as_a_pull_request",
    "test_sensitive_families_follow_their_inputs",
    "test_unrelated_security_change_does_not_expand_to_full_suite",
    "test_dispatch_and_shared_runner_edits_keep_broad_coverage",
    "test_numerical_transform_and_invalid_operation_checks_follow_their_inputs",
    "test_established_helper_edits_keep_direct_importing_tests",
    "test_full_fallback_and_shared_controls_preserve_edited_test_and_broad_families",
    "test_j7_rf_census_generator_inputs_select_the_fixture_currency_check",
    "test_synthetic_base_writer_inputs_select_its_check",
    "test_reference_system_inputs_select_its_fixture_check",
    "test_pinned_fixture_regenerations_select_the_tests_that_pin_their_bytes",
    "test_verifier_edits_select_their_importing_tests",
    "test_unrelated_changes_select_no_verifier_importers",
    "test_any_python_module_outside_tests_selects_the_verifier_independence_check",
    "test_supplier_acquisition_inputs_select_offline_boundaries_and_notice_hashes",
    "test_supplier_bundle_inputs_keep_preparation_and_canonical_build_checks",
    "test_model_version_literal_guard_follows_every_root_it_scans",
    "test_every_maintained_test_runs_when_a_path_it_reads_changes",
    "test_a_specification_a_test_only_names_is_never_its_probe",
    "test_named_tests_also_follow_the_paths_they_read",
    "test_finite_parameters_resolve_to_the_scripts_they_run_and_their_reads",
    "test_repository_wide_scans_keep_their_scope",
    "test_declared_reads_stay_current",
    "test_the_runner_wrappers_collect_every_runner_test_class",
    "test_formerly_never_run_guards_follow_their_roots",
    "test_reads_follow_imports_and_path_loaded_models_not_imported_modules_reads",
    "test_repository_check_tests_are_the_governance_step_s",
    "test_a_path_only_a_python_test_reads_starts_the_python_job",
    "test_a_parameter_resolves_from_its_own_bindings_not_outer_ones",
    "test_an_unknown_f_string_slot_may_span_directories",
    "test_a_path_under_an_unresolved_base_keeps_its_known_suffix",
    "test_git_pathspecs_and_unshown_scans_are_derived_or_declared",
    "test_an_imported_helpers_input_reads_select_the_test",
    "test_the_python_job_leaves_out_only_the_declarative_specifications",
    "test_actual_full_selection_is_partitioned_without_losing_paths",
    "test_tests_that_build_offline_have_their_locked_dependencies_fetched_before_pytest",
    "test_the_audit_is_every_tool_test_but_the_source_independent_ones",
    "test_a_solver_core_pull_request_leaves_the_audit_out",
    "test_a_pull_request_changing_the_tool_or_the_registration_runs_the_whole_audit",
    "test_schedule_dispatch_and_queue_runs_keep_the_audit_as_their_selection_gives_it",
))
STEP_NAMES = ("Select repository regressions",
              "Bounded shared-tool and routing regressions",
              "Every maintained Python test follows the paths it reads",
              "Deliverable dependency structure",
              "Execution boundary and changed specification structure",
              "Conflict and credential checks")


class StepFailed(Exception):
    def __init__(self, name: str, code: int, message: str = ""):
        super().__init__(message or f"{name} failed with exit status {code}")
        self.name, self.code = name, code


def is_commit(root: Path, ref: str) -> bool:
    """As `git rev-parse --verify --quiet "${BASE}^{commit}"`: an empty base names no commit."""
    return subprocess.run(["git", "rev-parse", "--verify", "--quiet", f"{ref}^{{commit}}"], cwd=root,
                          stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode == 0


def diff_base(root: Path, event: str, base: str, step: str) -> str:
    """The documents and conflict steps' base: the given one if it names a commit, else HEAD^1,
    except that a merge-queue group fails closed."""
    if is_commit(root, base):
        return base
    if event == "merge_group":
        raise StepFailed(step, 1, f"merge_group base {base} is unavailable")
    return "HEAD^1"


def deselected(root: Path) -> list[str]:
    return [f"--deselect={test}" for test in json.loads((root / PLAN).read_text())["deselect"]]


def steps(root: Path, event: str, base: str, head: str, python: str = sys.executable):
    """(name, build) in the job's order. `build()` returns the step's commands, each with its stdout
    file or None; it is called when the step starts, after every earlier step has succeeded, so a step
    reads what an earlier one wrote and resolves its base only when it runs."""
    def documents():
        return [([python, "tools/software_workflow/documentation_ci.py", "documents",
                  "--base", diff_base(root, event, base, STEP_NAMES[4])], None)]

    def conflicts():
        resolved = diff_base(root, event, base, STEP_NAMES[5])
        return [([python, "tools/validation/validate_conflict_markers.py", "--base", resolved, "--head", "HEAD"], None),
                ([python, "tools/validation/validate_run_record_leaks.py", "--base", resolved, "--head", "HEAD"], None)]

    return [
        (STEP_NAMES[0], lambda: [([python, "tools/software_workflow/hosted_ci.py", "repository", "--event", event,
                                   "--base", base, "--head", head, "--output", PLAN], None)]),
        (STEP_NAMES[1], lambda: [([python, "-m", "pytest", "-q", "-n", "auto", *deselected(root),
                                   *REGRESSION_TARGETS], None)]),
        (STEP_NAMES[2], lambda: [([python, "-m", "pytest", "-q", *ROUTING_INVARIANT_TESTS], None)]),
        (STEP_NAMES[3], lambda: [([python, "-m", "tools.deliverables", "--project", ".", "check"], DELIVERABLE_CHECK)]),
        (STEP_NAMES[4], documents),
        (STEP_NAMES[5], conflicts),
    ]


def run(root: Path, event: str, base: str, head: str, python: str = sys.executable, runner=subprocess.run) -> int:
    """Run every step in order; the first failure ends the run with its exit status."""
    grouped = os.environ.get("GITHUB_ACTIONS") == "true"
    for name, build in steps(root, event, base, head, python):
        print(f"::group::{name}" if grouped else f"== {name}", flush=True)
        try:
            for command, output in build():
                if output is None:
                    code = runner(command, cwd=root).returncode
                else:
                    with open(root / output, "w", encoding="utf-8") as stream:
                        code = runner(command, cwd=root, stdout=stream).returncode
                if code != 0:
                    raise StepFailed(name, code)
        except StepFailed as failure:
            if grouped:
                print("::endgroup::", flush=True)
                print(f"::error::{failure.name}: {failure}", flush=True)
            else:
                print(f"Repository checks failed at: {failure.name}: {failure}", file=sys.stderr, flush=True)
            return failure.code
        if grouped:
            print("::endgroup::", flush=True)
    print("Repository checks passed.", flush=True)
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--event", required=True)
    parser.add_argument("--base", default="")
    parser.add_argument("--head", required=True)
    args = parser.parse_args(argv)
    return run(ROOT, args.event, args.base, args.head)


if __name__ == "__main__":
    raise SystemExit(main())
