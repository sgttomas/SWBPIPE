#!/usr/bin/env python3
"""Provider-neutral local release-readiness checks for SWBPIPE."""

from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable


ROOT = Path(__file__).resolve().parents[2]

REQUIRED_PATHS = (
    "docs/BUILD_AND_RELEASE.md",
    "docs/RELEASE_NOTES_TEMPLATE.md",
    "docs/RELEASE_QUALITY_GATES.md",
    "docs/VALIDATION_STRATEGY.md",
    "docs/IP_AND_DATA_BOUNDARY.md",
    "docs/PROFESSIONAL_BOUNDARY.md",
    "execution/_DAG/_LATEST.md",
)
# The monorepo's approved dependency graph. The public export
# (the historical sanitized export, now retired) leaves out `loop/` and
# `execution/`, so there neither the path nor its schema check applies.
DAG_POINTER = "execution/_DAG/_LATEST.md"

CARGO_SEARCH_ROOTS = (
    "core",
    "validation/benchmarks",
)


@dataclass(frozen=True)
class CheckStep:
    name: str
    command: tuple[str, ...]
    description: str


def is_public_export(root: Path = ROOT) -> bool:
    """True in the staged public export: outside a monorepo checkout (where the
    project sits at `projects/<name>` under a repository root holding `.git` or
    `AGENTS.md`, the project having no `.git` of its own), and with neither
    `loop/` nor `execution/`. A sparse monorepo clone is not it; the export,
    a repository of its own, is it even inside another repository."""
    root = root.resolve()
    repository = root.parent.parent
    in_monorepo = not (root / ".git").exists() and root.parent.name == "projects" and (
        (repository / ".git").exists() or (repository / "AGENTS.md").is_file())
    return not in_monorepo and not (root / "loop").is_dir() and not (root / "execution").is_dir()


def check_required_paths(root: Path = ROOT) -> list[str]:
    export = is_public_export(root)
    return [path for path in REQUIRED_PATHS
            if not (export and path == DAG_POINTER) and not (root / path).exists()]


def latest_dag_dependency_edges(root: Path = ROOT) -> Path:
    latest_path = root / "execution" / "_DAG" / "_LATEST.md"
    try:
        lines = latest_path.read_text(encoding="utf-8").splitlines()
    except FileNotFoundError as exc:
        raise FileNotFoundError(f"missing latest DAG pointer: {latest_path}") from exc

    dag_dir: str | None = None
    for line in lines:
        if line.startswith("- Approved graph authority:"):
            _, value = line.split(":", maxsplit=1)
            dag_dir = value.strip().strip("`")
            break

    if dag_dir is None:
        raise ValueError(f"approved graph authority not found in {latest_path}")

    dependency_edges = Path(dag_dir) / "DependencyEdges.csv"
    if not (root / dependency_edges).exists():
        raise FileNotFoundError(
            f"approved graph dependency file not found: {dependency_edges}"
        )
    return dependency_edges


def discover_cargo_manifests(root: Path = ROOT) -> list[Path]:
    manifests: list[Path] = []
    for search_root in CARGO_SEARCH_ROOTS:
        base = root / search_root
        if not base.exists():
            continue
        for manifest in sorted(base.rglob("Cargo.toml")):
            relative = manifest.relative_to(root)
            if "target" in relative.parts:
                continue
            manifests.append(relative)
    return manifests


# b7-plan item 5a: the retained W1 path's stack witnesses at the R/64 = 1 MiB
# headroom leg (core/product_physics retained_memory_witness_tests.rs). An overflow
# aborts the test process, so each runs alone. In the registered build identity on
# the qualification host they are VER-004's record; elsewhere they are an early
# warning (DEL-04-08 Design, stack-headroom-repair.md).
STACK_WITNESS_MANIFEST = Path("core/product_physics/Cargo.toml")
STACK_WITNESSES = (
    "witness_headroom_w1_at_one_mebibyte",
    "witness_w2_deep_milestone_publishes",
    "witness_w_c2_publishes",
)
STACK_WITNESS_MODES = ("sparse", "dense")


def stack_witness_commands() -> list[tuple[str, ...]]:
    return [
        (
            "cargo",
            "test",
            "--offline",
            "--manifest-path",
            STACK_WITNESS_MANIFEST.as_posix(),
            "--lib",
            "--",
            f"retained_memory::witness_tests::{witness}::{mode}",
            "--exact",
            "--ignored",
            "--test-threads=1",
        )
        for witness in STACK_WITNESSES
        for mode in STACK_WITNESS_MODES
    ]


def python_cmd(*args: str) -> tuple[str, ...]:
    return (sys.executable, *args)


def build_plan(profile: str, root: Path = ROOT) -> list[CheckStep]:
    steps: list[CheckStep] = []

    if profile in {"skeleton", "python", "all"}:
        if not is_public_export(root):
            dag_dependency_edges = latest_dag_dependency_edges(root)
            steps.append(
                CheckStep(
                    name="dag dependency schema",
                    command=python_cmd(
                        "tools/validation/validate_dependencies_schema.py",
                        dag_dependency_edges.as_posix(),
                    ),
                    description="Validate the approved latest DAG dependency register schema.",
                )
            )
        steps.append(
            CheckStep(
                name="release readiness script tests",
                command=python_cmd(
                    "-m",
                    "pytest",
                    "-q",
                    "tests/test_release_readiness_script.py",
                ),
                description="Run focused tests for the provider-neutral readiness script.",
            )
        )

    if profile in {"python", "all"}:
        steps.append(
            CheckStep(
                name="python contract tests",
                command=python_cmd("-m", "pytest", "-q", "tests"),
                description="Run repository Python contract tests.",
            )
        )
        steps.append(
            CheckStep(
                name="coordination tool tests",
                command=python_cmd(
                    "-m",
                    "pytest",
                    "-q",
                    "tests/test_coordination_maintenance.py",
                ),
                description="Run coordination tool regression tests.",
            )
        )

    if profile in {"security", "all"}:
        steps.append(
            CheckStep(
                name="security and privacy tests",
                command=python_cmd("-m", "pytest", "-q", "tests/security"),
                description="Run local-first storage, telemetry, and redaction checks.",
            )
        )

    if profile in {"cargo", "all"}:
        for manifest in discover_cargo_manifests(root):
            steps.append(
                CheckStep(
                    name=f"cargo test {manifest.parent.as_posix()}",
                    command=(
                        "cargo",
                        "test",
                        "--offline",
                        "--manifest-path",
                        manifest.as_posix(),
                    ),
                    description="Run crate-local Rust tests without assuming a root workspace.",
                )
            )

    if profile in {"stack-witnesses", "all"}:
        for command in stack_witness_commands():
            steps.append(
                CheckStep(
                    name=f"stack witness {command[7].rsplit('::', 2)[-2]}::{command[7].rsplit('::', 1)[-1]}",
                    command=command,
                    description="Run one retained W1 stack witness at the R/64 headroom leg in its own process.",
                )
            )

    return steps


def missing_tools(steps: Iterable[CheckStep]) -> list[str]:
    missing: list[str] = []
    for step in steps:
        executable = step.command[0]
        if Path(executable).exists():
            continue
        if shutil.which(executable) is None and executable not in missing:
            missing.append(executable)
    return missing


def print_plan(steps: list[CheckStep], root: Path, execute: bool) -> None:
    mode = "execute" if execute else "dry-run"
    print(f"SWBPIPE release readiness profile ({mode})")
    print(f"repo: {root}")
    if is_public_export(root):
        print("public export: the DAG dependency schema check reads the monorepo's execution/_DAG and is not planned")
    print("")

    manifests = discover_cargo_manifests(root)
    print(f"discovered cargo manifests: {len(manifests)}")
    for manifest in manifests:
        print(f"  - {manifest.as_posix()}")
    print("")

    print(f"planned checks: {len(steps)}")
    for index, step in enumerate(steps, start=1):
        print(f"{index}. {step.name}")
        print(f"   {step.description}")
        print(f"   command: {' '.join(step.command)}")


def run_steps(steps: list[CheckStep], root: Path) -> int:
    env = os.environ.copy()
    env["CARGO_NET_OFFLINE"] = "true"
    for step in steps:
        print(f"running: {step.name}", flush=True)
        completed = subprocess.run(step.command, cwd=root, env=env, check=False)
        if completed.returncode != 0:
            print(f"failed: {step.name} exited {completed.returncode}", file=sys.stderr)
            return completed.returncode
    return 0


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Run or dry-run provider-neutral SWBPIPE release checks."
    )
    parser.add_argument(
        "--profile",
        choices=("skeleton", "python", "security", "cargo", "stack-witnesses", "all"),
        default="skeleton",
        help="Local check profile to plan or execute.",
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Run the selected local checks. Without this flag, only dry-run.",
    )
    parser.add_argument(
        "--list-cargo-manifests",
        action="store_true",
        help="Print discovered Cargo manifests and exit.",
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

    if args.list_cargo_manifests:
        for manifest in discover_cargo_manifests(root):
            print(manifest.as_posix())
        return 0

    missing_paths = check_required_paths(root)
    if missing_paths:
        print("missing required release paths:", file=sys.stderr)
        for path in missing_paths:
            print(f"  - {path}", file=sys.stderr)
        return 1

    steps = build_plan(args.profile, root)
    print_plan(steps, root, args.execute)
    sys.stdout.flush()

    if not args.execute:
        return 0

    tools = missing_tools(steps)
    if tools:
        print("missing required local tools:", file=sys.stderr)
        for tool in tools:
            print(f"  - {tool}", file=sys.stderr)
        return 1

    return run_steps(steps, root)


if __name__ == "__main__":
    raise SystemExit(main())
