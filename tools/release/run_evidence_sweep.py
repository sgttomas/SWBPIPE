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

The tool performs no network lookup: surface 4 either runs through the existing
local host path or consumes a caller-supplied, commit-bound record of the
successful DEC-093 Actions run. It uses no signing or publication credentials.
A green sweep is development evidence, not a release claim, professional
approval, certification, sealing, authentication, or code-compliance
determination.
"""

from __future__ import annotations

import argparse
import json
import os
import platform
import re
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

SCHEMA_VERSION = 3
ARTIFACT_KIND = "openpipestress.evidence_sweep_summary"
DECISION_BASIS = ["DEC-025", "DEC-093"]
DEFAULT_OUTPUT_DIR = "validation/evidence/sweeps"
DIRTY_DEFAULT_OUTPUT_DIR_NAME = "openpipestress-dirty-sweeps"
PINNED_WASM_BINDGEN_VERSION = "0.2.123"
MINIMUM_PYTHON_VERSION = (3, 11)
REQUIRED_NODE_BINS = ("playwright", "tsc", "vite", "vitest")
EXECUTION_CAPABILITIES = frozenset({"sandboxed", "host", "ci"})
CI_WORKFLOW_PATH = ".github/workflows/piping-desktop-e2e.yml"
CI_VIEWPORT_PROJECTS = ("chromium-desktop", "chromium-compact")
CI_BINDING_FIELDS = frozenset(
    {
        "workflow_path",
        "run_id",
        "run_attempt",
        "head_sha",
        "conclusion",
        "registered_e2e_specs_executed",
        "viewport_projects",
    }
)
SURFACE_IDS = (
    "cargo_crate_sweep",
    "python_pytest",
    "desktop_vitest",
    "desktop_playwright_e2e",
    "desktop_production_build",
)
SANDBOXED_SURFACE_IDS = tuple(
    surface_id
    for surface_id in SURFACE_IDS
    if surface_id != "desktop_playwright_e2e"
)

BOUNDARY_NOTE = (
    "Commit-bound development evidence; the tool performs no network lookup. "
    "Not a release claim, professional "
    "approval, certification, sealing, authentication, or code-compliance "
    "determination."
)


@dataclass(frozen=True)
class Surface:
    """One evidence surface: an ordered list of commands run sequentially."""

    surface_id: str
    description: str
    execution_capability: Literal["sandboxed", "host", "ci"]
    commands: tuple[tuple[str, ...], ...]
    ci_binding: dict | None = None

    def __post_init__(self) -> None:
        if self.execution_capability not in EXECUTION_CAPABILITIES:
            raise ValueError(
                f"invalid execution capability {self.execution_capability!r}; "
                f"expected one of: {', '.join(sorted(EXECUTION_CAPABILITIES))}"
            )
        if self.execution_capability == "ci":
            errors = validate_ci_binding(self.ci_binding)
            if errors:
                raise ValueError("invalid CI surface binding: " + "; ".join(errors))
            if self.commands:
                raise ValueError("a CI-bound surface cannot declare local commands")
        elif self.ci_binding is not None:
            raise ValueError("only a CI-bound surface may carry ci_binding")


def validate_ci_binding(
    binding: object, *, commit_hash: str | None = None
) -> list[str]:
    """Validate the exact DEC-093 surface-4 CI evidence binding."""
    if not isinstance(binding, dict):
        return ["ci_binding must be an object"]

    errors: list[str] = []
    fields = set(binding)
    missing = sorted(CI_BINDING_FIELDS - fields)
    extra = sorted(fields - CI_BINDING_FIELDS)
    if missing:
        errors.append("missing fields: " + ", ".join(missing))
    if extra:
        errors.append("unexpected fields: " + ", ".join(extra))

    if binding.get("workflow_path") != CI_WORKFLOW_PATH:
        errors.append(f"workflow_path must equal {CI_WORKFLOW_PATH!r}")
    for field in ("run_id", "run_attempt"):
        value = binding.get(field)
        if type(value) is not int or value < 1:
            errors.append(f"{field} must be a positive integer")

    head_sha = binding.get("head_sha")
    if not isinstance(head_sha, str) or re.fullmatch(r"[0-9a-f]{40}", head_sha) is None:
        errors.append("head_sha must be a full lowercase 40-hex Git SHA")
    if commit_hash is not None and head_sha != commit_hash:
        errors.append(
            f"head_sha {head_sha!r} does not equal sweep commit_hash {commit_hash!r}"
        )
    if binding.get("conclusion") != "success":
        errors.append("conclusion must equal 'success'; failed specs are not accepted")
    if binding.get("registered_e2e_specs_executed") is not True:
        errors.append("registered_e2e_specs_executed must be true")
    if binding.get("viewport_projects") != list(CI_VIEWPORT_PROJECTS):
        errors.append(
            "viewport_projects must exactly equal the registered dual-viewport "
            f"projects {list(CI_VIEWPORT_PROJECTS)!r}"
        )
    return errors


def load_ci_binding(path: Path) -> dict:
    """Load and validate a caller-supplied DEC-093 CI binding JSON object."""
    try:
        body = json.loads(path.read_text(encoding="utf-8"))
    except OSError as error:
        raise ValueError(f"unable to read CI binding {path}: {error}") from error
    except json.JSONDecodeError as error:
        raise ValueError(f"invalid CI binding JSON {path}: {error}") from error
    errors = validate_ci_binding(body)
    if errors:
        raise ValueError("invalid CI surface binding: " + "; ".join(errors))
    return body


def build_sweep_plan(ci_binding: dict | None = None) -> list[Surface]:
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
        (
            Surface(
                surface_id="desktop_playwright_e2e",
                description=(
                    "Registered source-mode Playwright e2e specs across both "
                    "viewport projects, bound to the successful DEC-093 Actions run."
                ),
                execution_capability="ci",
                commands=(),
                ci_binding=dict(ci_binding),
            )
            if ci_binding is not None
            else Surface(
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
            )
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
        if surface.ci_binding is not None:
            entry["ci_binding"] = dict(surface.ci_binding)
        if failed:
            results.append(entry)
            continue

        surface_failed = False
        if surface.execution_capability == "ci":
            print(
                f"[evidence-sweep] {surface.surface_id}: bound CI run "
                f"{surface.ci_binding['run_id']} attempt "
                f"{surface.ci_binding['run_attempt']}",
                flush=True,
            )
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


def _validate_git_state(git_state: object) -> tuple[list[str], str | None]:
    if not isinstance(git_state, dict):
        return ["git must be an object"], None
    errors: list[str] = []
    commit_hash = git_state.get("commit_hash")
    capture_failed = git_state.get("status_capture_failed")
    working_tree_dirty = git_state.get("working_tree_dirty")
    dirty_paths = git_state.get("dirty_paths")
    if commit_hash is not None and (
        not isinstance(commit_hash, str)
        or re.fullmatch(r"[0-9a-f]{40}", commit_hash) is None
    ):
        errors.append("git.commit_hash must be null or a full lowercase 40-hex Git SHA")
    if type(capture_failed) is not bool:
        errors.append("git.status_capture_failed must be boolean")
    if capture_failed is True:
        if working_tree_dirty is not None:
            errors.append("git.working_tree_dirty must be null when status capture failed")
    elif type(working_tree_dirty) is not bool:
        errors.append("git.working_tree_dirty must be boolean when status capture succeeds")
    if not isinstance(dirty_paths, list) or not all(
        isinstance(path, str) for path in dirty_paths
    ):
        errors.append("git.dirty_paths must be an array of strings")
    elif capture_failed is True and dirty_paths:
        errors.append("git.dirty_paths must be empty when status capture failed")
    elif (
        capture_failed is False
        and type(working_tree_dirty) is bool
        and working_tree_dirty != bool(dirty_paths)
    ):
        errors.append("git.working_tree_dirty must equal whether dirty_paths is nonempty")
    if capture_failed is False and commit_hash is None:
        errors.append("git.commit_hash cannot be null when status capture succeeds")
    return errors, commit_hash if isinstance(commit_hash, str) else None


def _validate_utc_timestamp(value: object, field_name: str) -> list[str]:
    if not isinstance(value, str) or re.fullmatch(
        r"\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|\+00:00)", value
    ) is None:
        return [f"{field_name} must be an ISO-8601 UTC timestamp string"]
    try:
        datetime.fromisoformat(value.replace("Z", "+00:00"))
    except ValueError:
        return [f"{field_name} must be an ISO-8601 UTC timestamp string"]
    return []


def _validate_command(command: object, surface_id: object) -> list[str]:
    if not isinstance(command, dict):
        return [f"surface {surface_id!r} commands must contain objects"]
    errors: list[str] = []
    argv = command.get("argv")
    if not isinstance(argv, list) or not argv or not all(
        isinstance(argument, str) and argument for argument in argv
    ):
        errors.append(f"surface {surface_id!r} command argv must be nonempty strings")
    if type(command.get("exit_code")) is not int:
        errors.append(f"surface {surface_id!r} command exit_code must be an integer")
    duration = command.get("duration_seconds")
    if type(duration) not in {int, float} or duration < 0:
        errors.append(
            f"surface {surface_id!r} command duration_seconds must be nonnegative"
        )
    return errors


def _expected_ids_for_selection(selection: object) -> tuple[list[str], list[str]]:
    if selection is None:
        return list(SURFACE_IDS), []
    if not isinstance(selection, dict):
        return [], ["surface_selection must be an object when present"]
    if set(selection) != {"only_capability", "complete_dec025_sweep"}:
        return [], ["surface_selection must contain only the two registered fields"]
    capability = selection.get("only_capability")
    if capability not in EXECUTION_CAPABILITIES:
        return [], ["surface_selection.only_capability is invalid"]
    if selection.get("complete_dec025_sweep") is not False:
        return [], ["a capability-selected summary must record complete_dec025_sweep false"]
    if capability == "sandboxed":
        return list(SANDBOXED_SURFACE_IDS), []
    return ["desktop_playwright_e2e"], []


def validate_summary(summary: object) -> list[str]:
    """Validate historical v2 and current v3 sweep summary contracts."""
    if not isinstance(summary, dict):
        return ["summary must be an object"]
    errors: list[str] = []
    if summary.get("artifact") != ARTIFACT_KIND:
        errors.append(f"artifact must equal {ARTIFACT_KIND!r}")

    schema_version = summary.get("schema_version")
    if schema_version not in {2, SCHEMA_VERSION}:
        errors.append(f"schema_version must equal 2 or {SCHEMA_VERSION}")
        return errors
    expected_basis = "DEC-025" if schema_version == 2 else DECISION_BASIS
    if summary.get("decision_basis") != expected_basis:
        errors.append(f"decision_basis must equal {expected_basis!r}")
    errors.extend(_validate_utc_timestamp(summary.get("started_utc"), "started_utc"))

    git_errors, commit_hash = _validate_git_state(summary.get("git"))
    errors.extend(git_errors)
    surfaces = summary.get("surfaces")
    if not isinstance(surfaces, list):
        return errors + ["surfaces must be an array"]
    selection = summary.get("surface_selection")
    expected_ids, selection_errors = _expected_ids_for_selection(selection)
    errors.extend(selection_errors)
    selected_capability = (
        selection.get("only_capability") if isinstance(selection, dict) else None
    )

    surface_ids: list[str] = []
    failed_seen = False
    for index, surface in enumerate(surfaces, start=1):
        if not isinstance(surface, dict):
            errors.append("each surface must be an object")
            continue
        surface_id = surface.get("surface_id")
        surface_ids.append(surface_id if isinstance(surface_id, str) else "")
        if surface.get("order") != index:
            errors.append(f"surface {surface_id!r} order must equal {index}")
        if not isinstance(surface.get("description"), str):
            errors.append(f"surface {surface_id!r} description must be a string")

        capability = surface.get("execution_capability")
        if schema_version == 2 and capability is None:
            capability = (
                "host"
                if surface_id == "desktop_playwright_e2e"
                else "sandboxed"
            )
        allowed_capabilities = {"sandboxed", "host"}
        if schema_version == SCHEMA_VERSION:
            allowed_capabilities.add("ci")
        if capability not in allowed_capabilities:
            errors.append(
                f"surface {surface_id!r} has invalid execution_capability {capability!r}"
            )
        if selected_capability is not None and capability != selected_capability:
            errors.append(
                f"surface {surface_id!r} capability does not match "
                f"surface_selection {selected_capability!r}"
            )
        if surface_id in SANDBOXED_SURFACE_IDS and capability != "sandboxed":
            errors.append(f"surface {surface_id!r} must be sandboxed")

        commands = surface.get("commands")
        if not isinstance(commands, list):
            errors.append(f"surface {surface_id!r} commands must be an array")
            commands = []
        for command in commands:
            errors.extend(_validate_command(command, surface_id))
        command_dicts = [command for command in commands if isinstance(command, dict)]
        status = surface.get("status")
        if status not in {"pass", "fail", "not_run"}:
            errors.append(f"surface {surface_id!r} status is invalid")
        elif status == "not_run":
            if commands:
                errors.append(f"surface {surface_id!r} not_run must have no commands")
            if not failed_seen:
                errors.append(f"surface {surface_id!r} cannot be not_run before a failure")
        elif failed_seen:
            errors.append(f"surface {surface_id!r} must be not_run after a failure")
        elif status == "fail":
            failed_seen = True
            if not command_dicts or not any(
                command.get("exit_code") for command in command_dicts
            ):
                errors.append(f"surface {surface_id!r} fail requires a nonzero command")
        elif capability != "ci" and (
            len(command_dicts) != len(commands)
            or not command_dicts
            or any(command.get("exit_code") != 0 for command in command_dicts)
        ):
            errors.append(f"surface {surface_id!r} pass requires successful commands")

        if surface_id != "desktop_playwright_e2e":
            if "ci_binding" in surface:
                errors.append(f"surface {surface_id!r} cannot carry ci_binding")
            continue
        if capability == "ci":
            binding_errors = validate_ci_binding(
                surface.get("ci_binding"), commit_hash=commit_hash
            )
            errors.extend(
                f"desktop_playwright_e2e.ci_binding: {error}"
                for error in binding_errors
            )
            if commands:
                errors.append("a CI-bound desktop_playwright_e2e surface cannot record commands")
        elif capability == "host":
            if "ci_binding" in surface:
                errors.append("a host desktop_playwright_e2e surface cannot carry ci_binding")
        else:
            errors.append(
                "desktop_playwright_e2e execution_capability must be 'host' or 'ci'"
            )

    if surface_ids != expected_ids:
        errors.append(f"surfaces must equal the selected DEC-025 order {expected_ids!r}")
    if len(surface_ids) != len(set(surface_ids)):
        errors.append("surface_id values must be unique")
    expected_overall = "fail" if failed_seen else "pass"
    if summary.get("overall_status") != expected_overall:
        errors.append(f"overall_status must equal {expected_overall!r}")
    return errors


def is_complete_sweep_summary(summary: object) -> bool:
    """True only for a valid, unselected five-surface DEC-025 summary."""
    return (
        isinstance(summary, dict)
        and "surface_selection" not in summary
        and validate_summary(summary) == []
    )


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
    validation_errors = validate_summary(summary)
    if validation_errors:
        raise ValueError("invalid evidence-sweep summary: " + "; ".join(validation_errors))
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
    print(
        f"OpenPipeStress {plan_scope} evidence sweep ({mode}) — "
        f"{' + '.join(DECISION_BASIS)}"
    )
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

    A host context retains its existing ability to run host and sandboxed
    surfaces. A sandboxed or CI context cannot run surfaces that need host OS
    services. A CI-bound surface records already-completed Actions evidence and
    does not itself require local host services.
    """
    if available_capability not in EXECUTION_CAPABILITIES:
        return [
            f"invalid execution capability {available_capability!r}; expected one of: "
            + ", ".join(sorted(EXECUTION_CAPABILITIES))
        ]

    errors = [
        (
            f"surface {surface.surface_id} requires host execution capability; "
            f"the declared context is {available_capability}"
        )
        for surface in surfaces
        if surface.execution_capability == "host"
        and available_capability != "host"
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
        "--surface4-ci-binding",
        default=None,
        help=(
            "JSON file containing the exact DEC-093 Actions binding for surface 4. "
            "When omitted, the unchanged host Playwright path is used."
        ),
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
            "Declare the invoker's execution context. 'sandboxed' or 'ci' "
            "fails before surface 1 when the plan contains a host-only surface; "
            "'host' retains the existing local execution semantics."
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

    ci_binding = None
    if args.surface4_ci_binding:
        binding_path = Path(args.surface4_ci_binding)
        if not binding_path.is_absolute():
            binding_path = root / binding_path
        try:
            ci_binding = load_ci_binding(binding_path.resolve())
        except ValueError as error:
            print(f"evidence-sweep CI binding rejected: {error}", file=sys.stderr)
            print("no evidence surface ran", file=sys.stderr)
            return 1

        initial_git_state = collect_git_state(root)
        binding_errors = validate_ci_binding(
            ci_binding, commit_hash=initial_git_state.get("commit_hash")
        )
        if binding_errors:
            print(
                "evidence-sweep CI binding rejected before surface 1:",
                file=sys.stderr,
            )
            for error in binding_errors:
                print(f"  - {error}", file=sys.stderr)
            print("no evidence surface ran", file=sys.stderr)
            return 1

    surfaces = select_surfaces(build_sweep_plan(ci_binding), args.only_capability)
    if not surfaces:
        print(
            "evidence-sweep surface selection is empty for this plan",
            file=sys.stderr,
        )
        return 1
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
    validation_errors = validate_summary(summary)
    if validation_errors:
        print("evidence-sweep summary validation failed:", file=sys.stderr)
        for error in validation_errors:
            print(f"  - {error}", file=sys.stderr)
        print("no summary was written", file=sys.stderr)
        return 1
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
