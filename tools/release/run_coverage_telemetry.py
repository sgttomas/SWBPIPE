#!/usr/bin/env python3
"""DEC-060 opt-in coverage telemetry — recorded, never blocking.

Runs the three D-04b coverage lanes (Rust `cargo llvm-cov` across the
discovered crate manifests, Python `coverage.py` over the pytest suite,
desktop Vitest `--coverage` with the v8 provider) and writes a commit-bound,
schema-versioned JSON telemetry artifact to `validation/evidence/coverage/`
(`DEC-060`, recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12;
basis `execution/_Coordination/_DECISIONS/
D-04b_coverage_tooling_floor_promotion.md` Option O-A, seam 2).

Telemetry posture:

- This runner sits BESIDE the DEC-025 five-surface merge gate, never inside
  it. `tools/release/run_evidence_sweep.py` is not modified and does not
  call this script.
- Numbers are recorded, never blocking. A lane whose tool is absent or whose
  run fails is recorded as `not_measured` with the reason — never as a gate
  failure. The process exit code reflects only artifact integrity (git
  binding, schema validity, writability), not any coverage value.
- No numeric floor exists. Floor promotion is a separate future human ruling
  once at least five clean-head artifacts spanning at least two distinct
  commits exist per lane (`DEC-060`).

The run is local-only: no network services, no upload, no publication. A
telemetry artifact is development evidence, not a release claim, professional
approval, certification, sealing, authentication, or code-compliance
determination.
"""

from __future__ import annotations

import argparse
import importlib.util
import json
import shutil
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 1
ARTIFACT_KIND = "openpipestress.coverage_telemetry"
DECISION_BASIS = "DEC-060"
DEFAULT_OUTPUT_DIR = "validation/evidence/coverage"
SCHEMA_FILENAME = "coverage_telemetry_schema.json"

# Recorded implementation choice for the D-04b §6.3 open question (Rust
# aggregation form, delegated to this tranche): the cross-crate aggregate is
# the count-weighted sum — total covered counts divided by total counts across
# every measured crate manifest. Per-crate numbers remain the primary record.
RUST_AGGREGATION = "count_weighted_sum_across_crates"

# Recorded implementation choice for the Python lane measurement scope: the
# whole repository Python surface (`--source=.`), so unexecuted repo modules
# count in the denominator, minus dependency/build/archive trees that are not
# project source.
PYTHON_COVERAGE_SOURCE = "."
PYTHON_COVERAGE_OMIT = "*/node_modules/*,.archive/*,*/target/*"

LANE_STATUS_MEASURED = "measured"
LANE_STATUS_NOT_MEASURED = "not_measured"

BOUNDARY_NOTE = (
    "Local-only opt-in coverage telemetry per DEC-060: recorded, never "
    "blocking; no numeric floor exists. Not a release claim, professional "
    "approval, certification, sealing, authentication, or code-compliance "
    "determination."
)


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


def _readiness_module():
    """Crate discovery reuses the release-readiness manifest walk."""
    return _load_sibling("check_release_readiness")


@dataclass(frozen=True)
class Lane:
    """One telemetry lane: a language surface measured by one selected tool."""

    lane_id: str
    tool: str
    description: str


def build_lane_plan() -> list[Lane]:
    """The three DEC-060 lanes in sweep-order (cargo, python, desktop)."""
    return [
        Lane(
            lane_id="cargo_llvm_cov",
            tool="cargo-llvm-cov",
            description=(
                "Rust line/function/region coverage per discovered crate "
                "manifest via host-installed cargo llvm-cov."
            ),
        ),
        Lane(
            lane_id="python_coverage",
            tool="coverage.py",
            description=(
                "Python line coverage over the repository pytest suite via "
                "coverage.py (requirements-dev.txt)."
            ),
        ),
        Lane(
            lane_id="desktop_vitest_coverage",
            tool="@vitest/coverage-v8",
            description=(
                "Desktop TS unit-test coverage via Vitest --coverage with "
                "the v8 provider (apps/desktop devDependency); wasm engine "
                "built first. Playwright e2e coverage stays unmeasured per "
                "the D-04b O-A lane scope."
            ),
        ),
    ]


def _record_command(entry: dict, argv: tuple[str, ...], exit_code: int, started: float) -> None:
    entry["commands"].append(
        {
            "argv": list(argv),
            "exit_code": exit_code,
            "duration_seconds": round(time.monotonic() - started, 3),
        }
    )


def _run_streaming(
    argv: tuple[str, ...], cwd: Path, env: dict | None = None
) -> int:
    """Run one lane command streaming output to the console."""
    completed = subprocess.run(argv, cwd=cwd, env=env, check=False)
    return completed.returncode


def _run_captured(
    argv: tuple[str, ...], cwd: Path, env: dict | None = None
) -> tuple[int, str]:
    """Run one lane command capturing stdout (stderr streams through)."""
    completed = subprocess.run(
        argv, cwd=cwd, env=env, check=False, stdout=subprocess.PIPE, text=True
    )
    return completed.returncode, completed.stdout


def _tool_version(argv: tuple[str, ...], root: Path) -> str | None:
    try:
        completed = subprocess.run(
            argv, cwd=root, capture_output=True, text=True, check=False
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout.strip() or None


def _not_measured(entry: dict, reason: str) -> dict:
    entry["status"] = LANE_STATUS_NOT_MEASURED
    entry["reason"] = reason
    print(f"[coverage-telemetry] {entry['lane_id']}: not_measured — {reason}", flush=True)
    return entry


def _lane_entry(lane: Lane) -> dict:
    return {
        "lane_id": lane.lane_id,
        "tool": lane.tool,
        "description": lane.description,
        "tool_version": None,
        "status": LANE_STATUS_NOT_MEASURED,
        "reason": None,
        "commands": [],
        "metrics": None,
    }


def _totals_counter() -> dict:
    return {"count": 0, "covered": 0}


def _percent(covered: int, count: int) -> float | None:
    if count <= 0:
        return None
    return round(100.0 * covered / count, 4)


def run_rust_lane(lane: Lane, root: Path) -> dict:
    """Measure every discovered crate manifest with cargo llvm-cov."""
    entry = _lane_entry(lane)

    if shutil.which("cargo") is None:
        return _not_measured(entry, "tool_missing: cargo is not on PATH")
    version = _tool_version(("cargo", "llvm-cov", "--version"), root)
    if version is None:
        return _not_measured(
            entry,
            "tool_missing: cargo llvm-cov is not installed on this host "
            "(host-installed subcommand per DEC-060)",
        )
    entry["tool_version"] = version

    manifests = _readiness_module().discover_cargo_manifests(root)
    if not manifests:
        return _not_measured(entry, "run_failed: no crate manifests discovered")

    crates: list[dict] = []
    aggregate = {
        "lines": _totals_counter(),
        "functions": _totals_counter(),
        "regions": _totals_counter(),
    }
    failed_manifest: str | None = None

    for manifest in manifests:
        argv = (
            "cargo",
            "llvm-cov",
            "--manifest-path",
            manifest.as_posix(),
            "--summary-only",
            "--json",
        )
        print(f"[coverage-telemetry] {lane.lane_id}: {' '.join(argv)}", flush=True)
        started = time.monotonic()
        exit_code, stdout = _run_captured(argv, root)
        _record_command(entry, argv, exit_code, started)

        crate_record: dict = {"manifest": manifest.as_posix(), "measured": False}
        if exit_code != 0:
            failed_manifest = manifest.as_posix()
            crates.append(crate_record)
            break
        try:
            totals = json.loads(stdout)["data"][0]["totals"]
        except (json.JSONDecodeError, KeyError, IndexError):
            failed_manifest = manifest.as_posix()
            crates.append(crate_record)
            break

        crate_record["measured"] = True
        for axis in ("lines", "functions", "regions"):
            axis_totals = totals.get(axis, {})
            count = int(axis_totals.get("count", 0))
            covered = int(axis_totals.get("covered", 0))
            crate_record[axis] = {
                "count": count,
                "covered": covered,
                "percent": _percent(covered, count),
            }
            aggregate[axis]["count"] += count
            aggregate[axis]["covered"] += covered
        crates.append(crate_record)

    if failed_manifest is not None:
        entry["metrics"] = {"crates": crates}
        return _not_measured(
            entry,
            f"run_failed: cargo llvm-cov failed for {failed_manifest}; the "
            "lane reverts to not_measured per DEC-060",
        )

    entry["status"] = LANE_STATUS_MEASURED
    entry["metrics"] = {
        "crates": crates,
        "aggregation": RUST_AGGREGATION,
        "aggregate": {
            axis: {
                "count": values["count"],
                "covered": values["covered"],
                "percent": _percent(values["covered"], values["count"]),
            }
            for axis, values in aggregate.items()
        },
    }
    return entry


def run_python_lane(lane: Lane, root: Path) -> dict:
    """Measure the repository pytest suite with coverage.py."""
    entry = _lane_entry(lane)

    version = _tool_version((sys.executable, "-m", "coverage", "--version"), root)
    if version is None:
        return _not_measured(
            entry,
            "tool_missing: coverage.py is not importable by this "
            "interpreter (see requirements-dev.txt)",
        )
    entry["tool_version"] = version.splitlines()[0]

    with tempfile.TemporaryDirectory(prefix="opsy-coverage-py-") as scratch:
        data_file = Path(scratch) / "coverage.sqlite"
        report_file = Path(scratch) / "coverage.json"
        run_argv = (
            sys.executable,
            "-m",
            "coverage",
            "run",
            f"--data-file={data_file}",
            f"--source={PYTHON_COVERAGE_SOURCE}",
            f"--omit={PYTHON_COVERAGE_OMIT}",
            "-m",
            "pytest",
            "-q",
            "tests",
        )
        print(f"[coverage-telemetry] {lane.lane_id}: {' '.join(run_argv)}", flush=True)
        started = time.monotonic()
        exit_code = _run_streaming(run_argv, root)
        _record_command(entry, run_argv, exit_code, started)
        if exit_code != 0:
            return _not_measured(
                entry,
                "run_failed: coverage-instrumented pytest run exited "
                f"{exit_code}; the lane reverts to not_measured per DEC-060",
            )

        report_argv = (
            sys.executable,
            "-m",
            "coverage",
            "json",
            f"--data-file={data_file}",
            "-o",
            str(report_file),
        )
        started = time.monotonic()
        exit_code = _run_streaming(report_argv, root)
        _record_command(entry, report_argv, exit_code, started)
        if exit_code != 0:
            return _not_measured(
                entry, f"run_failed: coverage json report exited {exit_code}"
            )

        try:
            totals = json.loads(report_file.read_text(encoding="utf-8"))["totals"]
        except (OSError, json.JSONDecodeError, KeyError):
            return _not_measured(
                entry, "run_failed: coverage json report was unreadable"
            )

    statements = int(totals.get("num_statements", 0))
    covered = int(totals.get("covered_lines", 0))
    entry["status"] = LANE_STATUS_MEASURED
    entry["metrics"] = {
        "statements": {
            "count": statements,
            "covered": covered,
            "percent": _percent(covered, statements),
        }
    }
    return entry


def run_desktop_lane(lane: Lane, root: Path) -> dict:
    """Measure the desktop Vitest unit suite with the v8 coverage provider."""
    entry = _lane_entry(lane)

    for tool in ("npm", "npx"):
        if shutil.which(tool) is None:
            return _not_measured(entry, f"tool_missing: {tool} is not on PATH")
    provider_manifests = (
        root / "node_modules" / "@vitest" / "coverage-v8" / "package.json",
        root
        / "apps"
        / "desktop"
        / "node_modules"
        / "@vitest"
        / "coverage-v8"
        / "package.json",
    )
    provider_manifest = next(
        (path for path in provider_manifests if path.exists()), None
    )
    if provider_manifest is None:
        return _not_measured(
            entry,
            "tool_missing: @vitest/coverage-v8 is not installed (run npm "
            "install; see apps/desktop devDependencies)",
        )
    try:
        entry["tool_version"] = json.loads(
            provider_manifest.read_text(encoding="utf-8")
        ).get("version")
    except (OSError, json.JSONDecodeError):
        entry["tool_version"] = None

    wasm_argv = ("npm", "run", "build:wasm:desktop")
    print(f"[coverage-telemetry] {lane.lane_id}: {' '.join(wasm_argv)}", flush=True)
    started = time.monotonic()
    exit_code = _run_streaming(wasm_argv, root)
    _record_command(entry, wasm_argv, exit_code, started)
    if exit_code != 0:
        return _not_measured(
            entry, f"run_failed: wasm engine build exited {exit_code}"
        )

    with tempfile.TemporaryDirectory(prefix="opsy-coverage-ts-") as scratch:
        vitest_argv = (
            "npx",
            "--no-install",
            "vitest",
            "run",
            "--coverage.enabled=true",
            "--coverage.provider=v8",
            "--coverage.reporter=json-summary",
            f"--coverage.reportsDirectory={scratch}",
        )
        print(
            f"[coverage-telemetry] {lane.lane_id}: {' '.join(vitest_argv)} "
            "(cwd apps/desktop)",
            flush=True,
        )
        started = time.monotonic()
        exit_code = _run_streaming(vitest_argv, root / "apps" / "desktop")
        _record_command(entry, vitest_argv, exit_code, started)
        if exit_code != 0:
            return _not_measured(
                entry,
                f"run_failed: vitest coverage run exited {exit_code}; the "
                "lane reverts to not_measured per DEC-060",
            )

        summary_path = Path(scratch) / "coverage-summary.json"
        try:
            totals = json.loads(summary_path.read_text(encoding="utf-8"))["total"]
        except (OSError, json.JSONDecodeError, KeyError):
            return _not_measured(
                entry, "run_failed: vitest coverage-summary.json was unreadable"
            )

    metrics: dict = {}
    for axis in ("lines", "statements", "functions", "branches"):
        axis_totals = totals.get(axis, {})
        count = int(axis_totals.get("total", 0))
        covered = int(axis_totals.get("covered", 0))
        metrics[axis] = {
            "count": count,
            "covered": covered,
            "percent": _percent(covered, count),
        }
    entry["status"] = LANE_STATUS_MEASURED
    entry["metrics"] = metrics
    return entry


LANE_RUNNERS = {
    "cargo_llvm_cov": run_rust_lane,
    "python_coverage": run_python_lane,
    "desktop_vitest_coverage": run_desktop_lane,
}


def run_telemetry(lanes: list[Lane], root: Path, lane_runners=None) -> dict:
    """Run every selected lane (never fail-fast) and return the summary body.

    Unlike the DEC-025 sweep, lanes are independent telemetry: a lane that
    cannot run is recorded as `not_measured` and the remaining lanes still
    run, because no lane blocks anything.
    """
    if lane_runners is None:
        lane_runners = LANE_RUNNERS
    sweep = _sweep_module()
    started = datetime.now(timezone.utc)

    results = [lane_runners[lane.lane_id](lane, root) for lane in lanes]

    finished = datetime.now(timezone.utc)
    statuses = [entry["status"] for entry in results]
    return {
        "artifact": ARTIFACT_KIND,
        "schema_version": SCHEMA_VERSION,
        "decision_basis": DECISION_BASIS,
        "boundary_note": BOUNDARY_NOTE,
        "git": sweep.collect_git_state(root),
        "runtime": sweep.collect_runtime_versions(root),
        "started_utc": started.isoformat(timespec="seconds"),
        "finished_utc": finished.isoformat(timespec="seconds"),
        "duration_seconds": round((finished - started).total_seconds(), 3),
        "lanes": results,
        "lanes_measured": statuses.count(LANE_STATUS_MEASURED),
        "lanes_not_measured": statuses.count(LANE_STATUS_NOT_MEASURED),
    }


def summary_filename(summary: dict) -> str:
    commit = (summary["git"]["commit_hash"] or "nocommit")[:12]
    if _sweep_module().git_state_unverified(summary["git"]):
        dirty = "-gitunverified"
    elif summary["git"]["working_tree_dirty"]:
        dirty = "-dirty"
    else:
        dirty = ""
    stamp = summary["started_utc"].replace("-", "").replace(":", "")
    stamp = stamp.split("+")[0] + "Z"
    return f"COVERAGE_{stamp}_{commit}{dirty}.json"


def schema_path() -> Path:
    return Path(__file__).resolve().parent / SCHEMA_FILENAME


def validate_summary(summary: dict) -> list[str]:
    """Validate the summary against the artifact schema.

    Returns a list of human-readable validation errors (empty when valid).
    jsonschema comes from requirements-dev.txt; if it is genuinely absent the
    caller records that as an integrity problem rather than skipping quietly.
    """
    try:
        import jsonschema
    except ImportError:
        return [
            "jsonschema is not importable (see requirements-dev.txt); "
            "cannot validate the telemetry artifact"
        ]
    schema = json.loads(schema_path().read_text(encoding="utf-8"))
    validator = jsonschema.Draft202012Validator(schema)
    return [
        f"{'/'.join(str(part) for part in error.absolute_path) or '<root>'}: "
        f"{error.message}"
        for error in validator.iter_errors(summary)
    ]


def write_summary(summary: dict, output_dir: Path) -> Path:
    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / summary_filename(summary)
    output_path.write_text(
        json.dumps(summary, indent=2, sort_keys=False) + "\n", encoding="utf-8"
    )
    return output_path


def print_plan(lanes: list[Lane], root: Path, execute: bool) -> None:
    mode = "execute" if execute else "dry-run"
    print(
        f"OpenPipeStress coverage telemetry ({mode}) — {DECISION_BASIS} "
        "(recorded, never blocking)"
    )
    print(f"repo: {root}")
    print("")
    print(f"lanes: {len(lanes)}")
    for order, lane in enumerate(lanes, start=1):
        print(f"{order}. {lane.lane_id} [{lane.tool}]")
        print(f"   {lane.description}")


def parse_args(argv: list[str]) -> argparse.Namespace:
    lane_ids = [lane.lane_id for lane in build_lane_plan()]
    parser = argparse.ArgumentParser(
        description=(
            "Run the DEC-060 opt-in coverage telemetry lanes and write a "
            "commit-bound machine-readable artifact. Recorded, never "
            "blocking; not part of the DEC-025 five-surface merge gate."
        )
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Run the telemetry lanes. Without this flag, only print the plan.",
    )
    parser.add_argument(
        "--lanes",
        default=",".join(lane_ids),
        help=(
            "Comma-separated lane ids to run "
            f"(default: all of {','.join(lane_ids)})."
        ),
    )
    parser.add_argument(
        "--output-dir",
        default=DEFAULT_OUTPUT_DIR,
        help=f"Telemetry artifact directory relative to the repo root "
        f"(default: {DEFAULT_OUTPUT_DIR}).",
    )
    parser.add_argument(
        "--repo-root",
        default=str(ROOT),
        help="Repository root. Defaults to the root containing this script.",
    )
    return parser.parse_args(argv)


def select_lanes(lane_arg: str) -> list[Lane] | None:
    plan = build_lane_plan()
    requested = [token.strip() for token in lane_arg.split(",") if token.strip()]
    known = {lane.lane_id for lane in plan}
    if not requested or any(token not in known for token in requested):
        return None
    return [lane for lane in plan if lane.lane_id in set(requested)]


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.repo_root).resolve()
    if not root.exists():
        print(f"missing repository root: {root}", file=sys.stderr)
        return 2

    lanes = select_lanes(args.lanes)
    if lanes is None:
        known = ",".join(lane.lane_id for lane in build_lane_plan())
        print(f"unknown lane selection {args.lanes!r}; known: {known}", file=sys.stderr)
        return 2

    print_plan(lanes, root, args.execute)
    sys.stdout.flush()

    if not args.execute:
        return 0

    summary = run_telemetry(lanes, root)

    validation_errors = validate_summary(summary)
    if validation_errors:
        print(
            "[coverage-telemetry] artifact failed its own schema validation:",
            file=sys.stderr,
        )
        for error in validation_errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    output_path = write_summary(summary, root / args.output_dir)

    print("")
    for entry in summary["lanes"]:
        line = f"[coverage-telemetry] {entry['lane_id']}: {entry['status']}"
        if entry["reason"]:
            line += f" ({entry['reason']})"
        print(line)
    print(f"[coverage-telemetry] commit: {summary['git']['commit_hash']}")
    if summary["git"]["working_tree_dirty"]:
        print(
            "[coverage-telemetry] working tree dirty — the DEC-060 "
            "floor-promotion evidence base counts clean-head artifacts only"
        )
    try:
        summary_ref = output_path.relative_to(root).as_posix()
    except ValueError:
        summary_ref = str(output_path)
    print(f"[coverage-telemetry] artifact: {summary_ref}")

    if _sweep_module().git_state_unverified(summary["git"]):
        print(
            "[coverage-telemetry] git state could not be verified — the "
            "artifact is not commit-bound",
            file=sys.stderr,
        )
        return 1
    # Recorded, never blocking: lane outcomes do not affect the exit code.
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
