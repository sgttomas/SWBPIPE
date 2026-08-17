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
import os
import platform
import shutil
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Literal


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 2
ARTIFACT_KIND = "openpipestress.evidence_sweep_summary"
DECISION_BASIS = "DEC-025"
DEFAULT_OUTPUT_DIR = "validation/evidence/sweeps"
DIRTY_DEFAULT_OUTPUT_DIR_NAME = "openpipestress-dirty-sweeps"
PINNED_WASM_BINDGEN_VERSION = "0.2.123"
MINIMUM_PYTHON_VERSION = (3, 11)
REQUIRED_NODE_BINS = ("playwright", "tsc", "vite", "vitest")
EXECUTION_CAPABILITIES = frozenset({"sandboxed", "host"})

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
    execution_capability: Literal["sandboxed", "host"]
    commands: tuple[tuple[str, ...], ...]

    def __post_init__(self) -> None:
        if self.execution_capability not in EXECUTION_CAPABILITIES:
            raise ValueError(
                f"invalid execution capability {self.execution_capability!r}; "
                f"expected one of: {', '.join(sorted(EXECUTION_CAPABILITIES))}"
            )


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
            execution_capability="sandboxed",
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
            execution_capability="sandboxed",
            commands=((sys.executable, "-m", "pytest", "-q", "tests"),),
        ),
        Surface(
            surface_id="desktop_vitest",
            description="Desktop unit/contract tests; wasm engine built first.",
            execution_capability="sandboxed",
            commands=(
                ("npm", "run", "build:wasm:desktop"),
                ("npm", "run", "test:desktop"),
            ),
        ),
        Surface(
            surface_id="desktop_playwright_e2e",
            description=(
                "Playwright end-to-end smoke in a real Chrome browser: dev-server "
                "lane, then production-dist lane via vite preview "
                "(TP-APP-R2-WASMPKG-001)."
            ),
            execution_capability="host",
            commands=(
                ("npm", "run", "test:e2e:desktop"),
                ("npm", "run", "test:e2e:dist:desktop"),
            ),
        ),
        Surface(
            surface_id="desktop_production_build",
            description="Desktop production build (tsc -b && vite build).",
            execution_capability="sandboxed",
            commands=(("npm", "run", "build:desktop"),),
        ),
    ]


def _capture(
    command: tuple[str, ...], root: Path, *, strip: bool = True
) -> str | None:
    try:
        completed = subprocess.run(
            command, cwd=root, capture_output=True, text=True, check=False
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout.strip() if strip else completed.stdout


def parse_porcelain_status(porcelain: str) -> list[str]:
    """Parse `git status --porcelain -z` output into the affected paths.

    Records are `XY <path>` terminated by NUL; rename/copy records carry the
    original path as a second NUL-terminated token. The X status character is
    a significant leading space for worktree-only changes (` M <path>`), so
    the captured output must never be whitespace-stripped before parsing.
    """
    paths: list[str] = []
    tokens = porcelain.split("\0")
    index = 0
    while index < len(tokens):
        record = tokens[index]
        index += 1
        if not record:
            continue
        status = record[:2]
        paths.append(record[3:])
        if "R" in status or "C" in status:
            if index < len(tokens) and tokens[index]:
                paths.append(tokens[index])
            index += 1
    return sorted(paths)


def collect_git_state(root: Path = ROOT) -> dict:
    """Bind the summary to the commit hash and record working-tree deltas.

    A failed git capture is recorded explicitly and must never read as a
    clean working tree: the DEC-025 gate binds evidence to the commit hash,
    so a summary that cannot verify its git state has to say so out loud
    (`working_tree_dirty` becomes null, never false).
    """
    commit = _capture(("git", "rev-parse", "HEAD"), root)
    branch = _capture(("git", "rev-parse", "--abbrev-ref", "HEAD"), root)
    porcelain = _capture(
        ("git", "status", "--porcelain", "-z"), root, strip=False
    )
    status_capture_failed = porcelain is None
    dirty_paths = parse_porcelain_status(porcelain or "")
    return {
        "commit_hash": commit,
        "branch": branch,
        "status_capture_failed": status_capture_failed,
        "working_tree_dirty": None if status_capture_failed else bool(dirty_paths),
        "dirty_paths": dirty_paths,
    }


def git_state_unverified(git_state: dict) -> bool:
    """True when the summary cannot honestly bind to a verified git state."""
    return bool(git_state.get("status_capture_failed")) or not git_state.get(
        "commit_hash"
    )


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


def current_python_version() -> tuple[int, int, int]:
    """Return the running interpreter version as a comparison-safe tuple."""
    return sys.version_info.major, sys.version_info.minor, sys.version_info.micro


def python_runtime_preflight_error(
    version: tuple[int, int, int] | None = None,
) -> str | None:
    """Reject interpreters below the project's Python 3.11 runtime floor."""
    current = current_python_version() if version is None else version
    if current[:2] >= MINIMUM_PYTHON_VERSION:
        return None
    current_label = ".".join(str(part) for part in current)
    required_label = ".".join(str(part) for part in MINIMUM_PYTHON_VERSION)
    return (
        f"current Python is {current_label}; required Python is "
        f">={required_label}"
    )


def run_command(command: tuple[str, ...], root: Path) -> int:
    """Run one evidence command streaming output to the console."""
    env = os.environ.copy()
    env["CARGO_NET_OFFLINE"] = "true"
    completed = subprocess.run(command, cwd=root, env=env, check=False)
    return completed.returncode


@dataclass(frozen=True)
class CargoCacheProbe:
    """One offline Cargo cache check and its source manifest identity."""

    source_manifest: Path
    cwd: Path
    command: tuple[str, ...]


def discover_cargo_manifests(root: Path) -> list[Path]:
    """Return project-relative Cargo manifests covered by the sweep."""
    return sorted(
        path.relative_to(root)
        for search_root in (root / "core", root / "validation" / "benchmarks")
        if search_root.exists()
        for path in search_root.rglob("Cargo.toml")
        if "target" not in path.relative_to(root).parts
    )


def tracked_cargo_locks(root: Path) -> set[Path] | None:
    """Return project-relative Cargo.lock paths owned by Git.

    File existence is deliberately insufficient: library crates may have an
    ignored Cargo.lock left by a local Cargo invocation. Such a lock must not
    change a nominally lockless preflight into a ``--locked`` probe.
    """
    try:
        completed = subprocess.run(
            (
                "git",
                "ls-files",
                "-z",
                "--",
                "core",
                "validation/benchmarks",
            ),
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )
    except FileNotFoundError:
        return None
    if completed.returncode != 0:
        return None
    return {
        Path(path)
        for path in completed.stdout.split("\0")
        if path.endswith("Cargo.lock")
    }


def build_cargo_cache_probes(
    manifests: list[Path],
    root: Path,
    projected_root: Path,
    tracked_locks: set[Path],
) -> list[CargoCacheProbe]:
    """Bind tracked locks to the checkout and lockless crates to a projection."""
    probes: list[CargoCacheProbe] = []
    for manifest in manifests:
        has_tracked_lock = manifest.with_name("Cargo.lock") in tracked_locks
        command = ["cargo", "fetch"]
        if has_tracked_lock:
            command.append("--locked")
        command.extend(("--offline", "--manifest-path", manifest.as_posix()))
        probes.append(
            CargoCacheProbe(
                source_manifest=manifest,
                cwd=root if has_tracked_lock else projected_root,
                command=tuple(command),
            )
        )
    return probes


def project_cargo_sources(root: Path, projected_root: Path) -> None:
    """Copy Cargo source roots without lockfiles or build output."""

    def ignore_generated(_directory: str, names: list[str]) -> set[str]:
        return {name for name in names if name == "Cargo.lock" or name == "target"}

    for relative_root in (Path("core"), Path("validation") / "benchmarks"):
        source = root / relative_root
        if source.exists():
            shutil.copytree(
                source,
                projected_root / relative_root,
                ignore=ignore_generated,
            )


def cargo_cache_preflight_errors(root: Path, env: dict[str, str]) -> list[str]:
    """Probe tracked and lockless Cargo manifests without mutating sources."""
    manifests = discover_cargo_manifests(root)
    tracked_locks = tracked_cargo_locks(root)
    if tracked_locks is None:
        return [
            "unable to identify Git-tracked Cargo.lock files for offline Cargo "
            "cache preflight"
        ]

    errors: list[str] = []
    with tempfile.TemporaryDirectory(prefix="openpipestress-cargo-preflight-") as temp:
        projected_root = Path(temp) / "project"
        project_cargo_sources(root, projected_root)
        for probe in build_cargo_cache_probes(
            manifests, root, projected_root, tracked_locks
        ):
            try:
                cache_probe = subprocess.run(
                    probe.command,
                    cwd=probe.cwd,
                    env=env,
                    capture_output=True,
                    text=True,
                    check=False,
                )
            except FileNotFoundError:
                errors.append("missing executable: cargo")
                break
            if cache_probe.returncode != 0:
                detail = cache_probe.stderr.strip().splitlines()
                suffix = f": {detail[-1]}" if detail else ""
                errors.append(
                    "offline Cargo cache incomplete for "
                    f"{probe.source_manifest}{suffix}"
                )
    return errors


def preflight_prerequisites(
    root: Path, surfaces: list[Surface] | None = None
) -> list[str]:
    """Validate the complete local/offline execution environment.

    The sweep never installs or downloads prerequisites.  Every check runs
    before the first evidence surface, and every Cargo cache probe is forced
    offline both in argv and environment. Direct callers that omit ``surfaces``
    retain the complete five-surface preflight; selected plans probe only the
    external services needed by their surfaces.
    """
    if surfaces is None:
        surfaces = build_sweep_plan()
    needs_host_services = any(
        surface.execution_capability == "host" for surface in surfaces
    )
    errors: list[str] = []
    env = os.environ.copy()
    env["CARGO_NET_OFFLINE"] = "true"

    for executable in ("cargo", "node", "npm", "rustup", "wasm-bindgen"):
        if shutil.which(executable) is None:
            errors.append(f"missing executable: {executable}")

    node_bin = root / "node_modules" / ".bin"
    required_node_bins = tuple(
        executable
        for executable in REQUIRED_NODE_BINS
        if executable != "playwright" or needs_host_services
    )
    for executable in required_node_bins:
        if not (node_bin / executable).is_file():
            errors.append(
                f"missing local Node prerequisite: node_modules/.bin/{executable} "
                "(provision from package-lock.json before the sweep)"
            )

    if shutil.which("rustup") is not None:
        try:
            target_probe = subprocess.run(
                ("rustup", "target", "list", "--installed"),
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
        except FileNotFoundError:
            # shutil.which can be out of step with the spawn path; report the
            # tool through the normal missing-dependency channel instead of
            # crashing the preflight.
            errors.append("missing executable: rustup")
        else:
            installed = {
                line.strip() for line in target_probe.stdout.splitlines()
            }
            if (
                target_probe.returncode != 0
                or "wasm32-unknown-unknown" not in installed
            ):
                errors.append("missing Rust target: wasm32-unknown-unknown")

    if shutil.which("wasm-bindgen") is not None:
        try:
            bindgen_probe = subprocess.run(
                ("wasm-bindgen", "--version"),
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
        except FileNotFoundError:
            errors.append("missing executable: wasm-bindgen")
        else:
            version_fields = bindgen_probe.stdout.strip().split()
            version = version_fields[1] if len(version_fields) > 1 else None
            if (
                bindgen_probe.returncode != 0
                or version != PINNED_WASM_BINDGEN_VERSION
            ):
                errors.append(
                    "wasm-bindgen version mismatch: expected "
                    f"{PINNED_WASM_BINDGEN_VERSION}, found "
                    f"{version or 'unavailable'}"
                )

    if (
        needs_host_services
        and shutil.which("node") is not None
        and (node_bin / "playwright").is_file()
    ):
        try:
            browser_probe = subprocess.run(
                (
                    "node",
                    "-e",
                    "const fs=require('node:fs');"
                    "const {chromium}=require('@playwright/test');"
                    "const p=chromium.executablePath();"
                    "if(!fs.existsSync(p)){console.error(p);process.exit(1)}",
                ),
                cwd=root,
                env=env,
                capture_output=True,
                text=True,
                check=False,
            )
        except FileNotFoundError:
            errors.append("missing executable: node")
        else:
            if browser_probe.returncode != 0:
                errors.append(
                    "missing local Playwright Chromium executable "
                    "(provision it before the sweep)"
                )

    if shutil.which("cargo") is not None:
        errors.extend(cargo_cache_preflight_errors(root, env))

    return errors


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
            "execution_capability": surface.execution_capability,
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
    if git_state_unverified(summary["git"]):
        dirty = "-gitunverified"
    elif summary["git"]["working_tree_dirty"]:
        dirty = "-dirty"
    else:
        dirty = ""
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


class SummaryOutputError(ValueError):
    """The requested summary destination violates evidence-output policy."""


def resolve_summary_output_dir(
    *,
    root: Path,
    requested_output_dir: str | None,
    git_state: dict,
    allow_dirty_canonical_output: bool,
) -> tuple[Path, str]:
    """Resolve a summary destination without polluting canonical evidence.

    Clean runs retain the canonical default. Dirty runs with no explicit
    destination are routed to the system temporary directory. An explicit
    non-canonical ``--output-dir`` is always honored, while an intentional
    dirty write to the canonical directory requires the dedicated opt-in.
    Unverified Git state can never use the canonical directory because it
    cannot establish that the worktree is safe.
    """
    canonical_dir = (root / DEFAULT_OUTPUT_DIR).resolve()
    explicit_output = requested_output_dir is not None
    if explicit_output:
        requested = Path(requested_output_dir)
        output_dir = (
            requested if requested.is_absolute() else root / requested
        ).resolve()
    else:
        output_dir = canonical_dir

    targets_canonical = output_dir == canonical_dir
    unverified = git_state_unverified(git_state)
    dirty = git_state.get("working_tree_dirty") is True

    if not dirty and not unverified:
        return output_dir, "clean-default" if not explicit_output else "explicit"

    if explicit_output and not targets_canonical:
        return output_dir, "explicit-noncanonical"

    if unverified:
        if explicit_output:
            raise SummaryOutputError(
                "Git state is unverified; canonical evidence output is prohibited. "
                "Choose a non-canonical --output-dir."
            )
        return (
            Path(tempfile.gettempdir()) / DIRTY_DEFAULT_OUTPUT_DIR_NAME,
            "unverified-temporary",
        )

    if allow_dirty_canonical_output:
        return canonical_dir, "dirty-canonical-opt-in"

    if explicit_output:
        raise SummaryOutputError(
            "the working tree is dirty and the requested --output-dir is the "
            "canonical validation/evidence/sweeps directory. Use "
            "--allow-dirty-canonical-output only when that write is intentional, "
            "or choose a non-canonical --output-dir."
        )

    return (
        Path(tempfile.gettempdir()) / DIRTY_DEFAULT_OUTPUT_DIR_NAME,
        "dirty-temporary",
    )


def print_plan(surfaces: list[Surface], root: Path, execute: bool) -> None:
    mode = "execute" if execute else "dry-run"
    plan_scope = (
        "five-surface" if len(surfaces) == 5 else f"{len(surfaces)}-surface partial"
    )
    print(f"OpenPipeStress {plan_scope} evidence sweep ({mode}) — {DECISION_BASIS}")
    print(f"repo: {root}")
    print("")
    print(f"surfaces (sequential, F-4-safe order): {len(surfaces)}")
    for order, surface in enumerate(surfaces, start=1):
        print(f"{order}. {surface.surface_id}")
        print(f"   execution capability: {surface.execution_capability}")
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


def capability_preflight_errors(
    surfaces: list[Surface], available_capability: str
) -> list[str]:
    """Return plan incompatibilities for the invoker's execution context.

    A host context can run both capability classes. A sandboxed context cannot
    run surfaces that need host OS services (currently Playwright/Chromium).
    """
    if available_capability not in EXECUTION_CAPABILITIES:
        return [
            f"invalid execution capability {available_capability!r}; expected one of: "
            + ", ".join(sorted(EXECUTION_CAPABILITIES))
        ]

    errors = [
        (
            f"surface {surface.surface_id} requires host execution capability; "
            "the declared context is sandboxed"
        )
        for surface in surfaces
        if surface.execution_capability == "host"
        and available_capability == "sandboxed"
    ]
    errors.extend(
        f"surface {surface.surface_id} declares invalid execution capability "
        f"{surface.execution_capability!r}"
        for surface in surfaces
        if surface.execution_capability not in EXECUTION_CAPABILITIES
    )
    return errors


def select_surfaces(
    surfaces: list[Surface], only_capability: str | None
) -> list[Surface]:
    """Select an explicitly capability-scoped subset of the sweep plan."""
    if only_capability is None:
        return surfaces
    return [
        surface
        for surface in surfaces
        if surface.execution_capability == only_capability
    ]


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
        default=None,
        help=(
            "Summary artifact directory relative to the repo root. Clean runs "
            f"default to {DEFAULT_OUTPUT_DIR}; dirty or Git-unverified runs "
            "default to a system temporary directory."
        ),
    )
    parser.add_argument(
        "--allow-dirty-canonical-output",
        action="store_true",
        help=(
            "Intentionally allow a dirty-worktree summary in the canonical "
            f"{DEFAULT_OUTPUT_DIR} directory. Without this opt-in, dirty "
            "default output is routed to a system temporary directory and an "
            "explicit canonical --output-dir is rejected."
        ),
    )
    parser.add_argument(
        "--repo-root",
        default=str(ROOT),
        help="Repository root. Defaults to the root containing this script.",
    )
    parser.add_argument(
        "--require-capability",
        choices=sorted(EXECUTION_CAPABILITIES),
        help=(
            "Declare the invoker's execution context. 'sandboxed' fails "
            "before surface 1 when the plan contains a host-only surface; "
            "'host' permits both surface capability classes."
        ),
    )
    parser.add_argument(
        "--only-capability",
        choices=sorted(EXECUTION_CAPABILITIES),
        help=(
            "Run only surfaces declaring this capability. The summary records "
            "the selection and is not a complete five-surface DEC-025 sweep."
        ),
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.repo_root).resolve()
    if not root.exists():
        print(f"missing repository root: {root}", file=sys.stderr)
        return 2

    python_error = python_runtime_preflight_error()
    if python_error:
        print(
            "evidence-sweep Python runtime preflight failed before "
            "prerequisite probing or surface 1:",
            file=sys.stderr,
        )
        print(f"  - {python_error}", file=sys.stderr)
        print(
            "no prerequisite probe or evidence surface ran",
            file=sys.stderr,
        )
        return 1

    surfaces = select_surfaces(build_sweep_plan(), args.only_capability)
    print_plan(surfaces, root, args.execute)
    if args.only_capability:
        print(
            "surface selection: only "
            f"{args.only_capability} (partial DEC-025 sweep)"
        )
    sys.stdout.flush()

    if args.require_capability:
        capability_errors = capability_preflight_errors(
            surfaces, args.require_capability
        )
        if capability_errors:
            print(
                "evidence-sweep execution capability preflight failed before "
                "surface 1:",
                file=sys.stderr,
            )
            for error in capability_errors:
                print(f"  - {error}", file=sys.stderr)
            print("no evidence surface ran", file=sys.stderr)
            return 1
        print(
            "[evidence-sweep] execution capability preflight: PASS "
            f"({args.require_capability})"
        )
        sys.stdout.flush()

    if not args.execute:
        return 0

    tools = missing_tools(surfaces)
    if tools:
        print("missing required local tools:", file=sys.stderr)
        for tool in tools:
            print(f"  - {tool}", file=sys.stderr)
        return 1

    os.environ["CARGO_NET_OFFLINE"] = "true"
    prerequisite_errors = preflight_prerequisites(root, surfaces)
    if prerequisite_errors:
        print(
            "evidence-sweep prerequisite preflight failed before execution:",
            file=sys.stderr,
        )
        for error in prerequisite_errors:
            print(f"  - {error}", file=sys.stderr)
        print(
            "no prerequisite was installed or downloaded; no evidence surface ran",
            file=sys.stderr,
        )
        return 1
    print("[evidence-sweep] prerequisite preflight: PASS (local/offline)")

    # The Playwright surface runs last, after the cargo + wasm + vitest surfaces
    # have loaded the machine. At Playwright's default worker count (~half the
    # cores) the extra Chromium workers are slow to reap under that load and
    # trip the internal 300s worker-stop grace ("worker process did not exit
    # within 300000ms after stop, force-killed it"), failing an otherwise
    # all-green run. Run the e2e workers serially for the gate; the configs read
    # PLAYWRIGHT_WORKERS and `setdefault` lets an explicit caller value win.
    os.environ.setdefault("PLAYWRIGHT_WORKERS", "1")

    summary = run_sweep(surfaces, root)
    if args.only_capability:
        summary["surface_selection"] = {
            "only_capability": args.only_capability,
            "complete_dec025_sweep": False,
        }
    try:
        output_dir, output_disposition = resolve_summary_output_dir(
            root=root,
            requested_output_dir=args.output_dir,
            git_state=summary["git"],
            allow_dirty_canonical_output=args.allow_dirty_canonical_output,
        )
    except SummaryOutputError as error:
        print(
            f"evidence-sweep summary output rejected: {error}",
            file=sys.stderr,
        )
        print("no summary was written", file=sys.stderr)
        return 1

    if output_disposition in {"dirty-temporary", "unverified-temporary"}:
        reason = (
            "working tree is dirty"
            if output_disposition == "dirty-temporary"
            else "Git state is unverified"
        )
        print(
            f"[evidence-sweep] {reason}; canonical evidence output is disabled "
            f"and the summary will be written under {output_dir}"
        )
    elif output_disposition == "dirty-canonical-opt-in":
        print(
            "[evidence-sweep] dirty canonical output explicitly enabled by "
            "--allow-dirty-canonical-output"
        )
    output_path = write_summary(summary, output_dir)

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
    if git_state_unverified(summary["git"]):
        print(
            "[evidence-sweep] git state could not be verified — the summary "
            "is not commit-bound and does not satisfy the DEC-025 gate",
            file=sys.stderr,
        )
        return 1
    return 0 if summary["overall_status"] == "pass" else 1


if __name__ == "__main__":
    raise SystemExit(main())
