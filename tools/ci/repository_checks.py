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
# The routing invariant over all maintained tests. An added or changed test does not otherwise select
# tests/test_ci_python.py, so without this step a test whose routing cannot be shown is green on its own
# pull request and red on main after it.
ROUTING_INVARIANT_TESTS = ("tests/test_ci_python.py::test_every_maintained_test_runs_when_a_path_it_reads_changes",
                           "tests/test_ci_python.py::test_a_specification_a_test_only_names_is_never_its_probe")
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
