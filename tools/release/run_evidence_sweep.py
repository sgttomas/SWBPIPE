#!/usr/bin/env python3
"""DEC-025 five-surface evidence sweep — the deterministic local merge gate.

Runs the five evidence surfaces sequentially in F-4-safe order (cargo crate
sweep, Python pytest, desktop Vitest with the wasm engine built first,
Playwright e2e, desktop production build) and writes a machine-readable
summary artifact bound to the current commit hash. This sweep is the required
pre-push/fan-in evidence for every parallel agent development branch
(`DEC-025`, recorded in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12;
basis `execution/_Coordination/_DECISIONS/D-05_ci_provider_workflow.md`
Option D).

The sweep is local-only: no network services, no signing, no publication
credentials. A green sweep is development evidence, not a release claim,
professional approval, certification, sealing, authentication, or
code-compliance determination.
"""

from __future__ import annotations

import argparse
import json
import platform
import shutil
import subprocess
import sys
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 1
ARTIFACT_KIND = "openpipestress.evidence_sweep_summary"
DECISION_BASIS = "DEC-025"
DEFAULT_OUTPUT_DIR = "validation/evidence/sweeps"

BOUNDARY_NOTE = (
    "Local-only development evidence. Not a release claim, professional "
    "approval, certification, sealing, authentication, or code-compliance "
    "determination."
)


@dataclass(frozen=True)
class Surface:
    """One evidence surface: an ordered list of commands run sequentially."""

    surface_id: str
    description: str
    commands: tuple[tuple[str, ...], ...]


def build_sweep_plan() -> list[Surface]:
    """The five surfaces in the DEC-025 / F-4-safe sequential order.

    The wasm engine build precedes Vitest because the Vitest setup pre-warms
    the engine and fails loudly when the generated artifact is absent. The
    Playwright root script rebuilds the wasm artifact itself
    (`test:e2e` = `build:wasm && playwright test`), which is safe because the
    sweep is strictly sequential and the build swaps atomically (F-4).
    """
    return [
        Surface(
            surface_id="cargo_crate_sweep",
            description="Rust crate tests across all discovered crate manifests.",
            commands=(
                (
                    sys.executable,
                    "tools/release/check_release_readiness.py",
                    "--profile",
                    "cargo",
                    "--execute",
                ),
            ),
        ),
        Surface(
            surface_id="python_pytest",
            description="Repository Python contract, governance, and validation tests.",
            commands=((sys.executable, "-m", "pytest", "-q", "tests"),),
        ),
        Surface(
            surface_id="desktop_vitest",
            description="Desktop unit/contract tests; wasm engine built first.",
            commands=(
                ("npm", "run", "build:wasm:desktop"),
                ("npm", "run", "test:desktop"),
            ),
        ),
        Surface(
            surface_id="desktop_playwright_e2e",
            description="Playwright end-to-end smoke in a real Chrome browser.",
            commands=(("npm", "run", "test:e2e:desktop"),),
        ),
        Surface(
            surface_id="desktop_production_build",
            description="Desktop production build (tsc -b && vite build).",
            commands=(("npm", "run", "build:desktop"),),
        ),
    ]


def _capture(command: tuple[str, ...], root: Path) -> str | None:
    try:
        completed = subprocess.run(
            command, cwd=root, capture_output=True, text=True, check=False
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout.strip()


def collect_git_state(root: Path = ROOT) -> dict:
    """Bind the summary to the commit hash and record working-tree deltas."""
    commit = _capture(("git", "rev-parse", "HEAD"), root)
    branch = _capture(("git", "rev-parse", "--abbrev-ref", "HEAD"), root)
    porcelain = _capture(("git", "status", "--porcelain"), root)
    dirty_paths = sorted(
        line[3:] for line in (porcelain or "").splitlines() if line.strip()
    )
    return {
        "commit_hash": commit,
        "branch": branch,
        "working_tree_dirty": bool(dirty_paths),
        "dirty_paths": dirty_paths,
    }


def collect_runtime_versions(root: Path = ROOT) -> dict:
    versions = {"platform": platform.platform(), "python": platform.python_version()}
    for name, command in (
        ("node", ("node", "--version")),
        ("npm", ("npm", "--version")),
        ("cargo", ("cargo", "--version")),
        ("rustc", ("rustc", "--version")),
    ):
        versions[name] = _capture(command, root)
    return versions


def run_command(command: tuple[str, ...], root: Path) -> int:
    """Run one evidence command streaming output to the console."""
    completed = subprocess.run(command, cwd=root, check=False)
    return completed.returncode


def run_sweep(surfaces: list[Surface], root: Path, runner=None) -> dict:
    """Execute surfaces sequentially, fail-fast, and return the summary body.

    A failing command fails its surface; later surfaces are recorded as
    `not_run` so the artifact states exactly which evidence exists for the
    bound commit.
    """
    if runner is None:
        runner = run_command
    started = datetime.now(timezone.utc)
    results: list[dict] = []
    failed = False

    for order, surface in enumerate(surfaces, start=1):
        entry: dict = {
            "order": order,
            "surface_id": surface.surface_id,
            "description": surface.description,
            "commands": [],
            "status": "not_run",
        }
        if failed:
            results.append(entry)
            continue

        surface_failed = False
        for command in surface.commands:
            print(f"[evidence-sweep] {surface.surface_id}: {' '.join(command)}", flush=True)
            start = time.monotonic()
            exit_code = runner(command, root)
            entry["commands"].append(
                {
                    "argv": list(command),
                    "exit_code": exit_code,
                    "duration_seconds": round(time.monotonic() - start, 3),
                }
            )
            if exit_code != 0:
                surface_failed = True
                break

        entry["status"] = "fail" if surface_failed else "pass"
        if surface_failed:
            failed = True
            print(
                f"[evidence-sweep] FAILED at surface {surface.surface_id}; "
                "remaining surfaces recorded as not_run",
                file=sys.stderr,
                flush=True,
            )
        results.append(entry)

    finished = datetime.now(timezone.utc)
    return {
        "artifact": ARTIFACT_KIND,
        "schema_version": SCHEMA_VERSION,
        "decision_basis": DECISION_BASIS,
        "boundary_note": BOUNDARY_NOTE,
        "git": collect_git_state(root),
        "runtime": collect_runtime_versions(root),
        "started_utc": started.isoformat(timespec="seconds"),
        "finished_utc": finished.isoformat(timespec="seconds"),
        "duration_seconds": round((finished - started).total_seconds(), 3),
        "surfaces": results,
        "overall_status": "fail" if failed else "pass",
    }


def summary_filename(summary: dict) -> str:
    commit = (summary["git"]["commit_hash"] or "nocommit")[:12]
    dirty = "-dirty" if summary["git"]["working_tree_dirty"] else ""
    stamp = summary["started_utc"].replace("-", "").replace(":", "")
    stamp = stamp.split("+")[0] + "Z"
    return f"SWEEP_{stamp}_{commit}{dirty}.json"


def write_summary(summary: dict, output_dir: Path) -> Path:
    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / summary_filename(summary)
    output_path.write_text(
        json.dumps(summary, indent=2, sort_keys=False) + "\n", encoding="utf-8"
    )
    return output_path


def print_plan(surfaces: list[Surface], root: Path, execute: bool) -> None:
    mode = "execute" if execute else "dry-run"
    print(f"OpenPipeStress five-surface evidence sweep ({mode}) — {DECISION_BASIS}")
    print(f"repo: {root}")
    print("")
    print(f"surfaces (sequential, F-4-safe order): {len(surfaces)}")
    for order, surface in enumerate(surfaces, start=1):
        print(f"{order}. {surface.surface_id}")
        print(f"   {surface.description}")
        for command in surface.commands:
            print(f"   command: {' '.join(command)}")


def missing_tools(surfaces: list[Surface]) -> list[str]:
    missing: list[str] = []
    for surface in surfaces:
        for command in surface.commands:
            executable = command[0]
            if Path(executable).exists():
                continue
            if shutil.which(executable) is None and executable not in missing:
                missing.append(executable)
    return missing


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Run the DEC-025 five-surface evidence sweep and write a "
            "commit-bound machine-readable summary."
        )
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Run the sweep. Without this flag, only print the plan.",
    )
    parser.add_argument(
        "--output-dir",
        default=DEFAULT_OUTPUT_DIR,
        help=f"Summary artifact directory relative to the repo root "
        f"(default: {DEFAULT_OUTPUT_DIR}).",
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

    surfaces = build_sweep_plan()
    print_plan(surfaces, root, args.execute)
    sys.stdout.flush()

    if not args.execute:
        return 0

    tools = missing_tools(surfaces)
    if tools:
        print("missing required local tools:", file=sys.stderr)
        for tool in tools:
            print(f"  - {tool}", file=sys.stderr)
        return 1

    summary = run_sweep(surfaces, root)
    output_path = write_summary(summary, root / args.output_dir)

    print("")
    print(f"[evidence-sweep] overall: {summary['overall_status']}")
    print(f"[evidence-sweep] commit: {summary['git']['commit_hash']}")
    if summary["git"]["working_tree_dirty"]:
        print(
            "[evidence-sweep] working tree dirty — summary records the deltas; "
            "the merge gate binds to a clean committed HEAD"
        )
    try:
        summary_ref = output_path.relative_to(root).as_posix()
    except ValueError:
        summary_ref = str(output_path)
    print(f"[evidence-sweep] summary: {summary_ref}")
    return 0 if summary["overall_status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
