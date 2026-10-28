#!/usr/bin/env python3
"""DEC-059 sanitized public-export pipeline for SWBPIPE (build surface).

Implements the export-pipeline prerequisite named by the D-05b ruling
(`execution/_Coordination/_DECISIONS/D-05b_public_export_ci_activation.md`
section 4.1, guarantees G1-G7; codified as DEC-059 in
`execution/_Decomposition/SOFTWARE_DECOMP.md` section 12): a deterministic,
allowlist-driven staging build of the public SWBPIPE tree with a
`path/size/sha256` manifest, a generated export report, a mechanical
forbidden-path/forbidden-content boundary check, and the DEL-08-05
protected-content lint (via the engine's `protected_content_lint_cli`
binary, the same surface the DEC-058 scan runner consumes) as an in-pipeline
gate. The pipeline fails toward exclusion: anything not allowlisted stays
out, and any unexpected blocking finding fails the run.

Boundary (DEC-059/DEC-058/DEC-027): this tool BUILDS and self-tests the
export mechanism only. It does not create a public repository, push
anything, activate CI, or clear anything. The export act itself remains
owner-gated behind the three DEC-059 prerequisites (pipeline exists meeting
G1-G7; a D-20/DEC-058-ruled green scan of the staged export signed by the
owner before any public push; public-repo location/naming fixed by/with the
D-06 ruling, DEC-057). A green pipeline run is development evidence only:
not a clearance, release claim, release-readiness claim, professional
approval, certification, sealing, authentication, or code-compliance
determination. Local-only: no network services, no signing, no publication
credentials.

Usage:
  python3 tools/release/export_public_openpipestress.py            # plan only
  python3 tools/release/export_public_openpipestress.py \
      --execute --target /path/to/export-out                        # stage
  python3 tools/release/export_public_openpipestress.py \
      --verify /path/to/export-out                                  # re-hash
"""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import platform
import shutil
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path, PureWindowsPath

sys.path.insert(0, str(Path(__file__).resolve().parent))
from release_members import is_text_member, suffix_of  # noqa: E402


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 1
ARTIFACT_KIND = "openpipestress.public_export_record"
DECISION_BASIS = "DEC-059"

STAGING_DIR_NAME = "staging"
MANIFEST_NAME = "export-manifest.csv"
REPORT_NAME = "export-report.md"
RECORD_NAME = "export-record.json"
LINT_RUN_NAME = "lint-run.json"

BOUNDARY_NOTE = (
    "UNSIGNED local tooling output. Building or running this pipeline "
    "creates no public repository, pushes nothing, activates no CI, and "
    "clears nothing: the export act is owner-gated behind the DEC-059 "
    "prerequisites, including a D-20/DEC-058-ruled owner-signed green scan "
    "of the staged export before any public push. A green run is not a "
    "legal clearance, release claim, release-readiness claim, professional "
    "approval, certification, sealing, authentication, or code-compliance "
    "determination."
)

# ---------------------------------------------------------------------------
# Export profile (G1/G4): inclusion allowlist and exclusions.
#
# Fail-toward-exclusion: only ROOT_FILES/ROOT_DIRS enter the staged tree;
# any other top-level entry is excluded and counted. The include/exclude
# set below is the "unresolved design work" the D-05b packet section 4
# assigned to this work item; it mirrors the `exports/chirality-app/`
# discipline the packet named.
# ---------------------------------------------------------------------------

ROOT_FILES = (
    ".gitignore",
    "CONTRIBUTING.md",
    "LICENSE.md",
    "README.md",
    "package-lock.json",
    "package.json",
    "requirements-dev.txt",
)

ROOT_DIRS = (
    "api",
    "apps",
    "core",
    "docs",
    "examples",
    "fixtures",
    "governance",
    "provenance",
    "schemas",
    "tests",
    "tools",
    "validation",
)

# Private working/coordination surfaces deliberately kept out of the public
# tree (documented exclusions; IP_AND_DATA_BOUNDARY sections 3 and 6).
EXCLUDED_TOP_LEVEL = (
    "AGENTS.md",
    "_harness",
    "execution",
    "init",
    "loop",
    "plans",
)

# Allowlisted roots whose subtrees are partially private or generated.
# Relative-path prefix match (posix separators).
EXCLUDED_SUBTREE_PREFIXES = (
    # Local machine-bound evidence artifacts (sweep summaries, scan records).
    "validation/evidence",
    # Coordination maintenance scripts operate on the private execution/ tree.
    "tools/coordination",
    # Generated build surfaces (also gitignored).
    "apps/desktop/src-tauri/gen",
    "apps/desktop/playwright-report",
    "apps/desktop/test-results",
    "apps/desktop/public/wasm-engine",
)

# Directory names never staged wherever they appear. `.github` is listed
# deliberately: no CI workflow ships in the export (DEC-059 hard fence; CI
# activation is a separate owner-gated act).
SKIP_DIR_NAMES = {
    # Deliverable/tranche run records are private development working
    # surfaces wherever they appear (e.g. core/units/_run_records).
    "_run_records",
    ".Archive",
    ".archive",
    ".chirality",
    ".checkpoints",
    ".claude",
    ".git",
    ".github",
    ".mypy_cache",
    ".next",
    ".pytest_cache",
    ".ruff_cache",
    "__pycache__",
    "build",
    "dist",
    "dist-electron",
    "node_modules",
    "out",
    "target",
}

SKIP_FILE_NAMES = {".DS_Store"}

SKIP_SUFFIXES = {
    ".log",
    ".pdf",
    ".pyc",
    ".pyo",
    ".tsbuildinfo",
    ".zip",
}

# Filename patterns that are boundary findings if they survive into staging
# (secret-material posture, G5-adjacent mechanical check).
FORBIDDEN_FILE_SUFFIXES = {".key", ".p12", ".pem", ".pfx"}
FORBIDDEN_FILE_STEM_PREFIXES = ("id_rsa", "id_ed25519", "id_ecdsa", "id_dsa")

# Binary (non-UTF-8) staged files must carry an explicitly allowed suffix;
# anything else is a boundary finding (fail toward exclusion).
ALLOWED_BINARY_SUFFIXES = {".png"}

# Private absolute path prefixes rewritten in staged text files (mirrors the
# `exports/chirality-app/export_public.py` sanitize pass). Longest first.
# The prefixes are assembled by concatenation so this script's own staged
# copy carries no contiguous private-path literal and stays functional after
# the sanitize pass.
_PRIVATE_PROJECTS_PREFIX = "/Users" + "/ryan/ai-env/projects/"
PUBLIC_REPLACEMENTS = (
    (
        _PRIVATE_PROJECTS_PREFIX + "chirality/projects/chirality-piping",
        "<swbpipe-project-root>",
    ),
    (_PRIVATE_PROJECTS_PREFIX + "chirality-piping", "<legacy-chirality-piping-root>"),
    (_PRIVATE_PROJECTS_PREFIX + "chirality", "<chirality-monorepo-root>"),
)

PRIVATE_PATH_RESIDUE_MARKER = "/Users" + "/"
_PRIVATE_KEY_BLOCK_PARTS = ("-----" + "BEGIN", "PRIVATE" + " KEY")

# Reviewed expected boundary findings: exact (staging-relative path, kind)
# pairs the gate treats as known-and-reviewed. Still recorded in the export
# record and report; any boundary finding NOT listed fails the run.
EXPECTED_BOUNDARY_FINDINGS = {
    ("tests/security/test_local_first_storage_policy.py", "private_path_residue"): (
        "Policy-test fixture data: the generic macOS home-directory prefix "
        "is itself the forbidden-prefix assertion payload of the local-first "
        "storage policy test."
    ),
}

# The sanitize pass and the lint gate cover every staged member that decodes
# as UTF-8 text (`release_members.is_text_member`), whatever its extension.

# ---------------------------------------------------------------------------
# Protected-content lint gate (G4, DEC-058 check (b) surface, consumed as-is
# from DEL-08-05 via the engine CLI — same invocation as
# tools/release/run_release_candidate_scan.py).
# ---------------------------------------------------------------------------

LINTER_MANIFEST = "core/reporting/protected_content_linter/Cargo.toml"
LINT_CLI_BIN = "protected_content_lint_cli"
LINT_BATCH_SIZE = 400


# Reviewed expected blocking findings: exact (staging-relative path, code)
# pairs the gate treats as known-and-reviewed. Every entry is still recorded
# in lint-run.json and surfaced in the export report for the owner's
# D-20/DEC-058 scan; nothing here waives protected-content policy (DEC-058:
# no waiver may authorize protected-content copying). Any blocking finding
# NOT listed fails the run.
EXPECTED_BLOCKING_LINT_FINDINGS = {
    ("docs/TYPES.md", "PROHIBITED_PROFESSIONAL_CLAIM"): (
        "Prohibition prose: the Report type row states reports 'must not "
        "claim certification, sealing, or professional approval by the "
        "software'; the phrase match is the policy statement itself."
    ),
    (
        "fixtures/report_lint/invented/synthetic_risk_template.txt",
        "PROTECTED_CONTENT_SYNTHETIC_MARKER",
    ): "Invented DEL-08-05 linter fixture; the marker is the test payload.",
    (
        "fixtures/report_lint/invented/synthetic_risk_template.txt",
        "PROHIBITED_PROFESSIONAL_CLAIM",
    ): "Invented DEL-08-05 linter fixture; the claim line is the test payload.",
    (
        "fixtures/report_lint/invented/lint_run_fixture.json",
        "PROTECTED_CONTENT_SYNTHETIC_MARKER",
    ): "Invented DEL-08-05 schema fixture quoting engine excerpts.",
    (
        "fixtures/report_lint/invented/lint_run_fixture.json",
        "PROHIBITED_PROFESSIONAL_CLAIM",
    ): "Invented DEL-08-05 schema fixture quoting engine excerpts.",
}

GUARANTEES = {
    "G1": (
        "IP_AND_DATA_BOUNDARY section 3 content excluded: private/coordination "
        "surfaces are outside the inclusion allowlist; the boundary check and "
        "the DEL-08-05 protected-content lint gate run over every staged "
        "text member and fail the run on unexpected findings."
    ),
    "G2": (
        "Supported, not claimed: this pipeline produces the staged tree, "
        "manifest, and report the D-20/DEC-058 owner-signed scan consumes. "
        "The recorded green scan before any public push is the owner's "
        "separate human act; no scan or clearance happens here."
    ),
    "G3": (
        "Deterministic, manifested staging: sorted walk, byte-identical "
        "re-runs, every staged file hashed (path/size_bytes/sha256) in "
        "export-manifest.csv, and the export bound to the canonical private "
        "monorepo commit recorded in export-record.json."
    ),
    "G4": (
        "Mechanical in-pipeline checks: forbidden-path/forbidden-content "
        "boundary findings (export_public.py pattern) plus the DEL-08-05 "
        "protected-content lint CLI gate; both fail toward exclusion/review, "
        "not silent pass."
    ),
    "G5": (
        "Evidence-only posture: the pipeline holds no signing secrets or "
        "publishing credentials, performs no network calls, and boundary "
        "findings flag secret-material filename/content patterns in the "
        "staged tree."
    ),
    "G6": (
        "Comparability preserved: the staged tree carries the provider-"
        "neutral BUILD_AND_RELEASE.md section 7 command basis (tools/release "
        "scripts, tests, five-surface sweep entrypoint) unmodified, and "
        "ships no CI workflow of its own (.github is excluded)."
    ),
    "G7": (
        "Drift managed: each export is cut from the named canonical commit "
        "recorded in export-record.json, and --verify re-hashes a staged "
        "tree against its manifest so stale or tampered exports are "
        "mechanically detectable."
    ),
}


def utc_now() -> datetime:
    return datetime.now(timezone.utc)


def run_command(command: list[str], cwd: Path) -> subprocess.CompletedProcess:
    return subprocess.run(command, cwd=cwd, capture_output=True, text=True, check=False)


def capture_output(command: list[str], cwd: Path) -> str | None:
    try:
        completed = run_command(command, cwd)
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout.strip() or None


def collect_git_state(root: Path) -> dict:
    commit = capture_output(["git", "rev-parse", "HEAD"], root)
    porcelain = capture_output(["git", "status", "--porcelain"], root)
    dirty_paths = []
    if porcelain:
        dirty_paths = [line[3:].strip() for line in porcelain.splitlines() if line.strip()]
    # Counts only: dirty and untracked file names can be private, and the
    # export stages HEAD blobs, so the working tree never reaches it.
    return {
        "commit_hash": commit,
        "working_tree_dirty": bool(dirty_paths),
        "dirty_path_count": len(dirty_paths),
    }


def repository_prefix(source_root: Path) -> str:
    """The source root as a repository-relative path (no host path)."""
    prefix = capture_output(["git", "rev-parse", "--show-prefix"], source_root)
    return "<repository>/" + (prefix or "")


def collect_runtime_versions(root: Path) -> dict:
    return {
        "python": platform.python_version(),
        "platform": platform.platform(),
        "cargo": capture_output(["cargo", "--version"], root),
    }


# ---------------------------------------------------------------------------
# Selection (G1: allowlist in, everything else out).
# ---------------------------------------------------------------------------


def is_excluded_subtree(rel_posix: str) -> bool:
    for prefix in EXCLUDED_SUBTREE_PREFIXES:
        # `prefix + "."` also excludes atomic-swap siblings such as
        # apps/desktop/public/wasm-engine.tmp-*/ (DEC-025 F-4 staging dirs).
        if (
            rel_posix == prefix
            or rel_posix.startswith(prefix + "/")
            or rel_posix.startswith(prefix + ".")
        ):
            return True
    return False


def skip_reason(rel: Path) -> str | None:
    """Why a candidate relative path stays out of staging (None = staged)."""
    rel_posix = rel.as_posix()
    for part in rel.parts:
        if part in SKIP_DIR_NAMES:
            return "skip_dir_name"
        if part.lower().startswith(".env"):
            return "env_file"
    if is_excluded_subtree(rel_posix):
        return "excluded_subtree"
    if rel.name in SKIP_FILE_NAMES:
        return "skip_file_name"
    if suffix_of(rel) in SKIP_SUFFIXES:
        return "skip_suffix"
    if "quarantine" in (part.lower() for part in rel.parts):
        return "quarantine_path"
    return None


# Git tree-entry modes (`git ls-tree`).
MODE_SYMLINK = "120000"
MODE_SUBMODULE = "160000"
MODE_EXECUTABLE = "100755"


def head_entries(source_root: Path) -> dict[str, tuple[str, str]] | None:
    """The HEAD tree under `source_root`: posix path relative to it ->
    (mode, object id). None when `source_root` is not inside a git work tree
    or git is unavailable (`select_files` then refuses the export).

    G3 binds the export to HEAD, so selection and staging both read HEAD's
    tree and blobs: working-tree edits, newly indexed files, ignored and
    untracked files cannot enter the export, and an LFS pointer is exported
    as the pointer HEAD holds."""
    probe = capture_output(["git", "rev-parse", "--is-inside-work-tree"], source_root)
    if probe != "true":
        return None
    try:
        prefix = run_command(["git", "rev-parse", "--show-prefix"], source_root)
        listing = run_command(
            ["git", "ls-tree", "-r", "-z", f"HEAD:{prefix.stdout.strip()}"], source_root
        )
    except OSError:
        return None
    if prefix.returncode != 0 or listing.returncode != 0:
        raise SystemExit(
            "export refused (HEAD_TREE_UNREADABLE): git could not list HEAD's "
            "tree for the source root"
        )
    entries: dict[str, tuple[str, str]] = {}
    for record in listing.stdout.split("\0"):
        if not record:
            continue
        meta, path = record.split("\t", 1)
        mode, _kind, oid = meta.split()
        entries[path] = (mode, oid)
    return entries


def tracked_files(source_root: Path) -> set[str] | None:
    """Paths in HEAD's tree under `source_root`, or None outside git."""
    entries = head_entries(source_root)
    return None if entries is None else set(entries)


def select_files(source_root: Path) -> tuple[list[Path], dict, list[str]]:
    """Select from HEAD's tree; return staged relative paths, exclusion
    counts, and the top-level HEAD entries that are not allowlisted
    (fail-toward-exclusion ledger). A source outside a git work tree is
    refused (GIT_PROBE_FAILED)."""
    entries = head_entries(source_root)
    if entries is None:
        # Fail closed: without git the export cannot be bound to a commit, and
        # a filesystem walk would admit ignored and untracked material.
        raise SystemExit(
            "export refused (GIT_PROBE_FAILED): the source root is not inside a "
            "git work tree, or git is unavailable; the export is cut only from "
            "a git commit"
        )

    # No symlink is ever exported. An allowlisted root that HEAD holds as a
    # symlink refuses the export rather than ship its link text, its target's
    # bytes, or an incomplete tree; deeper symlinks are excluded and counted.
    symlinked_roots = sorted(
        name
        for name in (*ROOT_FILES, *ROOT_DIRS)
        if entries.get(name, ("",))[0] == MODE_SYMLINK
    )
    if symlinked_roots:
        raise SystemExit(
            "export refused (SYMLINKED_ROOT): allowlisted roots are symlinks in "
            "HEAD: " + ", ".join(symlinked_roots)
        )

    missing = [name for name in ROOT_FILES if name not in entries] + [
        name for name in ROOT_DIRS if not any(p.startswith(name + "/") for p in entries)
    ]
    if missing:
        raise SystemExit(
            "allowlisted roots missing from HEAD: "
            + ", ".join(sorted(missing))
            + "; update ROOT_FILES/ROOT_DIRS to match the tree"
        )

    selected: list[Path] = [Path(name) for name in ROOT_FILES]
    exclusions: dict[str, int] = {}

    def count(reason: str) -> None:
        exclusions[reason] = exclusions.get(reason, 0) + 1

    root_dirs = set(ROOT_DIRS)
    for path in sorted(entries):
        if path.split("/", 1)[0] not in root_dirs:
            continue
        mode, _oid = entries[path]
        rel = Path(path)
        if mode == MODE_SYMLINK:
            reason = "symlink"
        elif mode == MODE_SUBMODULE:
            reason = "submodule"
        else:
            reason = skip_reason(rel)
        if reason is not None:
            count(reason)
            continue
        selected.append(rel)

    allowlisted = set(ROOT_FILES) | root_dirs
    unlisted_top_level = sorted(
        {
            path.split("/", 1)[0]
            for path in entries
            if path.split("/", 1)[0] not in allowlisted
            and path.split("/", 1)[0] not in SKIP_DIR_NAMES
        }
    )

    return sorted(selected), exclusions, unlisted_top_level


# ---------------------------------------------------------------------------
# Staging, sanitize pass, manifest (G3).
# ---------------------------------------------------------------------------


def stage_files(source_root: Path, staging: Path, selected: list[Path]) -> None:
    """Write each selected path's HEAD blob (never working-tree bytes)."""
    entries = head_entries(source_root)
    if entries is None:
        raise SystemExit("export refused (GIT_PROBE_FAILED): cannot read HEAD blobs")
    process = subprocess.Popen(
        ["git", "cat-file", "--batch"],
        cwd=source_root,
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
    )
    assert process.stdin is not None and process.stdout is not None
    try:
        for rel in selected:
            mode, oid = entries[rel.as_posix()]
            process.stdin.write(oid.encode("ascii") + b"\n")
            process.stdin.flush()
            header = process.stdout.readline().split()
            if len(header) != 3 or header[1] != b"blob":
                raise SystemExit(
                    f"export refused (HEAD_BLOB_UNREADABLE): {rel.as_posix()}"
                )
            data = process.stdout.read(int(header[2]))
            process.stdout.read(1)
            target = staging / rel
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            target.chmod(0o755 if mode == MODE_EXECUTABLE else 0o644)
    finally:
        process.stdin.close()
        process.wait()


def sanitize_text_files(staging: Path) -> int:
    changed = 0
    for path in sorted(staging.rglob("*")):
        if not path.is_file() or path.is_symlink() or not is_text_member(path):
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        updated = text
        for old, new in PUBLIC_REPLACEMENTS:
            updated = updated.replace(old, new)
        if updated != text:
            path.write_text(updated, encoding="utf-8")
            changed += 1
    return changed


def iter_staged_files(staging: Path):
    for path in sorted(staging.rglob("*")):
        if path.is_file():
            yield path


def manifest_rows(staging: Path) -> list[dict]:
    rows = []
    for path in iter_staged_files(staging):
        data = path.read_bytes()
        rows.append(
            {
                "path": path.relative_to(staging).as_posix(),
                "size_bytes": str(len(data)),
                "sha256": hashlib.sha256(data).hexdigest(),
            }
        )
    return rows


def write_manifest(rows: list[dict], output: Path) -> None:
    with output.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=["path", "size_bytes", "sha256"])
        writer.writeheader()
        writer.writerows(rows)


# ---------------------------------------------------------------------------
# Boundary check (G4 mechanical forbidden-path/forbidden-content pass).
# ---------------------------------------------------------------------------


def read_text_if_utf8(path: Path) -> str | None:
    try:
        return path.read_bytes().decode("utf-8")
    except UnicodeDecodeError:
        return None


def boundary_findings(staging: Path) -> list[dict]:
    """Mechanical forbidden-path/forbidden-content findings over the staged
    tree. Each finding is {"kind", "path"}; anything found here should have
    been excluded by selection, so findings are gate failures unless listed
    in EXPECTED_BOUNDARY_FINDINGS."""
    findings: list[dict] = []
    allowlisted = set(ROOT_FILES) | set(ROOT_DIRS)

    def add(kind: str, rel_posix: str) -> None:
        findings.append({"kind": kind, "path": rel_posix})

    for path in sorted(staging.rglob("*")):
        rel = path.relative_to(staging)
        rel_posix = rel.as_posix()

        if rel.parts and rel.parts[0] not in allowlisted:
            add("non_allowlisted_top_level", rel_posix)
        if path.is_symlink():
            add("symlink", rel_posix)
            continue
        for part in rel.parts:
            if part in SKIP_DIR_NAMES or part in EXCLUDED_TOP_LEVEL:
                add("forbidden_path_segment", rel_posix)
                break
        if "quarantine" in (part.lower() for part in rel.parts):
            add("quarantine_path", rel_posix)
        if is_excluded_subtree(rel_posix):
            add("excluded_subtree_staged", rel_posix)

        if not path.is_file():
            continue

        if path.name in SKIP_FILE_NAMES or path.name.lower().startswith(".env"):
            add("forbidden_file_name", rel_posix)
        if suffix_of(path) in SKIP_SUFFIXES:
            add("forbidden_suffix", rel_posix)
        if suffix_of(path) in FORBIDDEN_FILE_SUFFIXES or path.name.lower().startswith(
            FORBIDDEN_FILE_STEM_PREFIXES
        ):
            add("secret_material_filename", rel_posix)

        text = read_text_if_utf8(path)
        if text is None:
            if suffix_of(path) not in ALLOWED_BINARY_SUFFIXES:
                add("non_allowlisted_binary", rel_posix)
            continue

        if PRIVATE_PATH_RESIDUE_MARKER in text:
            add("private_path_residue", rel_posix)
        if all(part in text for part in _PRIVATE_KEY_BLOCK_PARTS):
            add("private_key_block", rel_posix)

    return findings


def classify_boundary_findings(findings: list[dict]) -> dict:
    """Split boundary findings into reviewed-expected and unexpected; any
    unexpected finding fails the gate."""
    expected: list[dict] = []
    unexpected: list[dict] = []
    for finding in findings:
        key = (finding["path"], finding["kind"])
        if key in EXPECTED_BOUNDARY_FINDINGS:
            finding = dict(finding)
            finding["expected_justification"] = EXPECTED_BOUNDARY_FINDINGS[key]
            expected.append(finding)
        else:
            unexpected.append(finding)
    return {
        "gate_pass": not unexpected,
        "expected": expected,
        "unexpected": unexpected,
    }


# ---------------------------------------------------------------------------
# Protected-content lint gate (G4, engine consumed as-is).
# ---------------------------------------------------------------------------


def lintable_staged_files(staging: Path) -> list[Path]:
    return [
        path
        for path in iter_staged_files(staging)
        if is_text_member(path)
    ]


def run_engine_lint(paths: list[Path], engine_root: Path) -> dict:
    """DEL-08-05 protected-content lint over staged text members via the
    engine CLI (same surface as run_release_candidate_scan.py check (b)).
    Fails toward review: an unavailable engine is a failed gate."""
    base_command = [
        "cargo",
        "run",
        "--quiet",
        "--manifest-path",
        LINTER_MANIFEST,
        "--bin",
        LINT_CLI_BIN,
        "--",
        "--provenance-mode",
        "external",
    ]
    if not paths:
        return {
            "status": "pass",
            "command": " ".join(base_command[:8]) + " ...",
            "summary": {"target_count": 0, "finding_count": 0, "blocking_finding_count": 0},
            "findings": [],
            "note": "No lintable text members staged.",
        }

    findings: list[dict] = []
    target_count = 0
    scanned_count = 0
    for start in range(0, len(paths), LINT_BATCH_SIZE):
        batch = paths[start : start + LINT_BATCH_SIZE]
        command = base_command + [str(path) for path in batch]
        try:
            completed = run_command(command, engine_root)
        except OSError as error:
            return {
                "status": "engine_unavailable",
                "command": " ".join(base_command[:8]) + " ...",
                "summary": None,
                "findings": [],
                "note": (
                    f"Engine CLI unavailable ({type(error).__name__}). "
                    "Fails toward review."
                ),
            }
        if completed.returncode != 0:
            # Engine output can carry host paths: it goes to the console only.
            print(completed.stderr.strip()[-2000:], file=sys.stderr)
            return {
                "status": "engine_error",
                "command": " ".join(base_command[:8]) + " ...",
                "summary": None,
                "findings": [],
                "note": (
                    f"Engine CLI exited {completed.returncode}; its output is "
                    "not recorded (it can carry host paths). Fails toward review."
                ),
            }
        payload = json.loads(completed.stdout)
        summary = payload.get("summary", {})
        target_count += summary.get("target_count", len(batch))
        scanned_count += summary.get("scanned_target_count", 0)
        findings.extend(payload.get("findings", []))

    blocking = [f for f in findings if f.get("severity") == "BLOCKING"]
    return {
        "status": "findings_present" if findings else "pass",
        "command": " ".join(base_command[:8]) + " ...",
        "summary": {
            "target_count": target_count,
            "scanned_target_count": scanned_count,
            "finding_count": len(findings),
            "blocking_finding_count": len(blocking),
            "clean_scan_is_clearance": False,
        },
        "findings": findings,
        "note": None,
    }


def relativize_finding_path(finding_path: str, staging: Path) -> str:
    # The Windows CLI uses a stable logical identity for canonical UNC files.
    # Compare that identity with the resolved staging root using Windows path
    # components; treating ``unc://`` as a native Path makes it relative to the
    # current directory and loses reviewed expected-finding matches.
    if finding_path.startswith("unc://"):
        parts = finding_path[len("unc://") :].split("/")
        if len(parts) < 3 or any(part in {"", ".", ".."} or "\\" in part for part in parts):
            return finding_path
        staging_path = str(staging.resolve()).replace("\\", "/")
        if staging_path.casefold().startswith("//?/unc/"):
            staging_path = "//" + staging_path[len("//?/UNC/") :]
        try:
            return PureWindowsPath("//" + "/".join(parts)).relative_to(
                PureWindowsPath(staging_path)
            ).as_posix()
        except ValueError:
            return finding_path
    try:
        return Path(finding_path).resolve().relative_to(staging.resolve()).as_posix()
    except ValueError:
        return finding_path


def classify_lint_gate(lint_result: dict, staging: Path) -> dict:
    """Split blocking findings into reviewed-expected and unexpected; any
    unexpected blocking finding (or an unavailable engine) fails the gate."""
    if lint_result["status"] in {"engine_unavailable", "engine_error"}:
        return {
            "gate_pass": False,
            "expected_blocking": [],
            "unexpected_blocking": [],
            "warning_count": 0,
            "stale_expectations": sorted(
                f"{path} :: {code}" for path, code in EXPECTED_BLOCKING_LINT_FINDINGS
            ),
            "note": lint_result.get("note"),
        }

    expected: list[dict] = []
    unexpected: list[dict] = []
    warning_count = 0
    matched_keys = set()

    for finding in lint_result.get("findings", []):
        rel_path = relativize_finding_path(finding.get("path", ""), staging)
        finding = dict(finding)
        finding["path"] = rel_path
        if finding.get("severity") != "BLOCKING":
            warning_count += 1
            continue
        key = (rel_path, finding.get("code"))
        if key in EXPECTED_BLOCKING_LINT_FINDINGS:
            matched_keys.add(key)
            finding["expected_justification"] = EXPECTED_BLOCKING_LINT_FINDINGS[key]
            expected.append(finding)
        else:
            unexpected.append(finding)

    stale = sorted(
        f"{path} :: {code}"
        for path, code in EXPECTED_BLOCKING_LINT_FINDINGS
        if (path, code) not in matched_keys
    )

    return {
        "gate_pass": not unexpected,
        "expected_blocking": expected,
        "unexpected_blocking": unexpected,
        "warning_count": warning_count,
        "stale_expectations": stale,
        "note": lint_result.get("note"),
    }


# ---------------------------------------------------------------------------
# Report and record.
# ---------------------------------------------------------------------------


def top_level_counts(rows: list[dict]) -> dict[str, int]:
    counts: dict[str, int] = {}
    for row in rows:
        top = row["path"].split("/", 1)[0]
        counts[top] = counts.get(top, 0) + 1
    return counts


def build_record(
    *,
    now: datetime,
    source_root: Path,
    git: dict,
    versions: dict,
    rows: list[dict],
    exclusions: dict,
    unlisted_top_level: list[str],
    sanitized_count: int,
    boundary_gate: dict,
    lint_result: dict,
    lint_gate: dict,
    gate_pass: bool,
) -> dict:
    return {
        "schema_version": SCHEMA_VERSION,
        "artifact_kind": ARTIFACT_KIND,
        "decision_basis": DECISION_BASIS,
        "generated_utc": now.isoformat(),
        "source": {
            "source_root": repository_prefix(source_root),
            "git": git,
            "runtime_versions": versions,
        },
        "profile": {
            "root_files": list(ROOT_FILES),
            "root_dirs": list(ROOT_DIRS),
            "excluded_top_level": list(EXCLUDED_TOP_LEVEL),
            "excluded_subtree_prefixes": list(EXCLUDED_SUBTREE_PREFIXES),
            "allowed_binary_suffixes": sorted(ALLOWED_BINARY_SUFFIXES),
        },
        "counts": {
            "staged_files": len(rows),
            "staged_bytes": sum(int(row["size_bytes"]) for row in rows),
            "sanitized_text_files": sanitized_count,
            "excluded_by_reason": dict(sorted(exclusions.items())),
            "unlisted_top_level_excluded": unlisted_top_level,
            "top_level_file_counts": top_level_counts(rows),
        },
        "boundary_check": {
            "gate_pass": boundary_gate["gate_pass"],
            "expected_finding_count": len(boundary_gate["expected"]),
            "expected_findings": boundary_gate["expected"],
            "unexpected_finding_count": len(boundary_gate["unexpected"]),
            "unexpected_findings": boundary_gate["unexpected"][:500],
        },
        "lint_gate": {
            "engine": "open_pipe_stress_protected_content_linter",
            "engine_surface": LINT_CLI_BIN,
            "status": lint_result["status"],
            "command": lint_result["command"],
            "summary": lint_result.get("summary"),
            "gate_pass": lint_gate["gate_pass"],
            "expected_blocking_count": len(lint_gate["expected_blocking"]),
            "expected_blocking": lint_gate["expected_blocking"],
            "unexpected_blocking_count": len(lint_gate["unexpected_blocking"]),
            "unexpected_blocking": lint_gate["unexpected_blocking"][:100],
            "warning_count": lint_gate["warning_count"],
            "stale_expectations": lint_gate["stale_expectations"],
            "note": lint_gate.get("note"),
            "clean_scan_is_clearance": False,
        },
        "guarantees": GUARANTEES,
        "gate_result": "pass" if gate_pass else "fail",
        "owner_gated_next_steps": [
            "D-20/DEC-058 owner-signed protected-content scan of this staged "
            "tree (green, recorded) before any public push.",
            "Public-repo location/naming per the D-06 ruling (DEC-057).",
            "The publication event itself (owner act; DEC-059).",
        ],
        "boundary_note": BOUNDARY_NOTE,
    }


# DEL-10-04:1: the staged tree verifies itself. This pipeline does not run
# these commands; the report states them for whoever reviews the export.
SELF_VERIFICATION_LINES = [
    "",
    "## Self-Verification",
    "",
    "The staged tree verifies itself without the monorepo, as the git "
    "repository the public export will be. Copy `staging/` to a new directory "
    "(a `.git` inside `staging/` would fail `--verify`) and, there:",
    "",
    "- `git init -q && git add -A -f && git commit -q -m export` (tests that "
    "bind a commit need one; `-f` keeps the committed `Cargo.lock` files that "
    "crate `.gitignore` files list)",
    "- `python3 -m pytest -q tests`",
    "- `python3 tools/release/check_release_readiness.py --profile skeleton --execute`",
    "",
    "Tests that read a surface the export leaves out (the monorepo's "
    "`execution/`, its repository-root tools and CI action, and local "
    "evidence) skip there with that surface named (`tests/public_export.py`). "
    "This pipeline does not run them.",
]


def write_report(record: dict, output: Path) -> None:
    counts = record["counts"]
    lint = record["lint_gate"]
    lines = [
        "# SWBPIPE Sanitized Public-Export Report",
        "",
        "Generated by `tools/release/export_public_openpipestress.py` "
        "(DEC-059 export pipeline build surface).",
        "",
        f"- Canonical commit: `{record['source']['git'].get('commit_hash')}`",
        f"- Working tree dirty: {record['source']['git'].get('working_tree_dirty')}",
        f"- Generated (UTC): {record['generated_utc']}",
        f"- Staged files: {counts['staged_files']}",
        f"- Staged bytes: {counts['staged_bytes']}",
        f"- Text files sanitized for private absolute paths: "
        f"{counts['sanitized_text_files']}",
        f"- Boundary check: "
        f"{'pass' if record['boundary_check']['gate_pass'] else 'FAIL'} "
        f"(expected {record['boundary_check']['expected_finding_count']}, "
        f"unexpected {record['boundary_check']['unexpected_finding_count']})",
        f"- Lint gate: {'pass' if lint['gate_pass'] else 'FAIL'} "
        f"(expected blocking {lint['expected_blocking_count']}, "
        f"unexpected blocking {lint['unexpected_blocking_count']}, "
        f"warnings {lint['warning_count']})",
        f"- Overall gate result: {record['gate_result']}",
        "",
        "## Top-Level File Counts",
        "",
        "| Path | Files |",
        "|---|---:|",
    ]
    for key in sorted(counts["top_level_file_counts"]):
        lines.append(f"| `{key}` | {counts['top_level_file_counts'][key]} |")

    lines.extend(["", "## Exclusions", "", "| Reason | Count |", "|---|---:|"])
    for reason in sorted(counts["excluded_by_reason"]):
        lines.append(f"| {reason} | {counts['excluded_by_reason'][reason]} |")
    lines.append("")
    lines.append(
        "Top-level entries present but not allowlisted (stay out by "
        "construction): "
        + (
            ", ".join(f"`{name}`" for name in counts["unlisted_top_level_excluded"])
            or "none"
        )
    )

    lines.extend(["", "## Boundary Findings", ""])
    boundary = record["boundary_check"]
    if boundary["expected_findings"]:
        lines.append("Reviewed expected findings (recorded, not waived):")
        for finding in boundary["expected_findings"]:
            lines.append(
                f"- `{finding['path']}` {finding['kind']}: "
                f"{finding.get('expected_justification', '')}"
            )
        lines.append("")
    if boundary["unexpected_findings"]:
        lines.append("UNEXPECTED findings (gate failure):")
        for finding in boundary["unexpected_findings"]:
            lines.append(f"- `{finding['path']}` {finding['kind']}")
    if not boundary["expected_findings"] and not boundary["unexpected_findings"]:
        lines.append("No boundary findings.")

    lines.extend(["", "## Protected-Content Lint Gate", ""])
    lines.append(
        f"Engine: `{lint['engine']}` via `{lint['engine_surface']}` "
        f"(status: {lint['status']})."
    )
    if lint["expected_blocking"]:
        lines.append("")
        lines.append("Reviewed expected blocking findings (recorded, not waived):")
        for finding in lint["expected_blocking"]:
            lines.append(
                f"- `{finding['path']}` {finding['code']}: "
                f"{finding.get('expected_justification', '')}"
            )
    if lint["unexpected_blocking"]:
        lines.append("")
        lines.append("UNEXPECTED blocking findings (gate failure):")
        for finding in lint["unexpected_blocking"]:
            lines.append(
                f"- `{finding['path']}` line {finding.get('line')}: "
                f"{finding['code']} — {finding.get('message', '')}"
            )
    if lint["stale_expectations"]:
        lines.append("")
        lines.append(
            "Stale expected-finding entries (listed but not matched — "
            "review and prune):"
        )
        lines.extend(f"- {item}" for item in lint["stale_expectations"])
    lines.append("")
    lines.append(f"Warnings routed to human review: {lint['warning_count']}.")

    lines.extend(["", "## DEC-059 G1-G7 Disposition", ""])
    for key in sorted(record["guarantees"]):
        lines.append(f"- **{key}**: {record['guarantees'][key]}")

    lines.extend(SELF_VERIFICATION_LINES)

    lines.extend(["", "## Boundary", "", record["boundary_note"], ""])
    output.write_text("\n".join(lines), encoding="utf-8")


# ---------------------------------------------------------------------------
# Modes.
# ---------------------------------------------------------------------------


def run_plan(source_root: Path) -> int:
    selected, exclusions, unlisted = select_files(source_root)
    print("DEC-059 export pipeline — plan only (no writes).")
    print(f"Source root: {source_root}")
    print(f"Would stage {len(selected)} files.")
    print("Lint gate: every staged member that decodes as UTF-8 text (counted at --execute).")
    print("Exclusions by reason:")
    for reason in sorted(exclusions):
        print(f"  {reason}: {exclusions[reason]}")
    print(
        "Top-level entries not allowlisted (stay out): "
        + (", ".join(unlisted) or "none")
    )
    print(
        "Boundary check, sanitize pass, manifest, lint gate, report, and "
        "record run under --execute --target <dir>."
    )
    print(BOUNDARY_NOTE)
    return 0


def run_execute(source_root: Path, target: Path, force: bool, engine_root: Path) -> int:
    target = target.resolve()
    if target == source_root.resolve() or source_root.resolve() in target.parents:
        print("error: --target must be outside the source root", file=sys.stderr)
        return 2
    if target.exists() and any(target.iterdir()):
        if not force:
            print(
                f"error: target {target} exists and is not empty (use --force "
                "to replace)",
                file=sys.stderr,
            )
            return 2
        shutil.rmtree(target)
    target.mkdir(parents=True, exist_ok=True)

    now = utc_now()
    git = collect_git_state(source_root)
    versions = collect_runtime_versions(source_root)

    selected, exclusions, unlisted = select_files(source_root)
    staging = target / STAGING_DIR_NAME
    if staging.exists():
        shutil.rmtree(staging)
    stage_files(source_root, staging, selected)
    sanitized_count = sanitize_text_files(staging)

    rows = manifest_rows(staging)
    write_manifest(rows, target / MANIFEST_NAME)

    boundary_gate = classify_boundary_findings(boundary_findings(staging))
    lint_result = run_engine_lint(lintable_staged_files(staging), engine_root)
    # Recorded finding paths are staging-relative, never host paths.
    lint_record = dict(lint_result)
    lint_record["findings"] = [
        dict(finding, path=relativize_finding_path(finding.get("path", ""), staging))
        for finding in lint_result.get("findings", [])
    ]
    (target / LINT_RUN_NAME).write_text(
        json.dumps(lint_record, indent=2) + "\n", encoding="utf-8"
    )
    lint_gate = classify_lint_gate(lint_result, staging)

    gate_pass = boundary_gate["gate_pass"] and lint_gate["gate_pass"]
    record = build_record(
        now=now,
        source_root=source_root,
        git=git,
        versions=versions,
        rows=rows,
        exclusions=exclusions,
        unlisted_top_level=unlisted,
        sanitized_count=sanitized_count,
        boundary_gate=boundary_gate,
        lint_result=lint_result,
        lint_gate=lint_gate,
        gate_pass=gate_pass,
    )
    (target / RECORD_NAME).write_text(
        json.dumps(record, indent=2) + "\n", encoding="utf-8"
    )
    write_report(record, target / REPORT_NAME)

    print(f"Staged {len(rows)} files into {staging}")
    print(
        "Boundary check: "
        f"{'pass' if boundary_gate['gate_pass'] else 'FAIL'} "
        f"(expected {len(boundary_gate['expected'])}, "
        f"unexpected {len(boundary_gate['unexpected'])})"
    )
    summary = lint_result.get("summary") or {}
    print(
        "Lint gate: "
        f"{'pass' if lint_gate['gate_pass'] else 'FAIL'} "
        f"(findings {summary.get('finding_count', 'n/a')}, "
        f"expected blocking {len(lint_gate['expected_blocking'])}, "
        f"unexpected blocking {len(lint_gate['unexpected_blocking'])}, "
        f"warnings {lint_gate['warning_count']})"
    )
    print(f"Gate result: {record['gate_result']}")
    print(BOUNDARY_NOTE)
    return 0 if gate_pass else 1


def run_verify(target: Path) -> int:
    staging = target / STAGING_DIR_NAME
    manifest_path = target / MANIFEST_NAME
    if not staging.is_dir() or not manifest_path.is_file():
        print(
            f"error: {target} does not contain {STAGING_DIR_NAME}/ and "
            f"{MANIFEST_NAME}",
            file=sys.stderr,
        )
        return 2

    with manifest_path.open(encoding="utf-8", newline="") as handle:
        recorded = {row["path"]: row for row in csv.DictReader(handle)}
    actual = {row["path"]: row for row in manifest_rows(staging)}

    added = sorted(set(actual) - set(recorded))
    removed = sorted(set(recorded) - set(actual))
    changed = sorted(
        path
        for path in set(recorded) & set(actual)
        if recorded[path]["sha256"] != actual[path]["sha256"]
        or recorded[path]["size_bytes"] != actual[path]["size_bytes"]
    )

    record_path = target / RECORD_NAME
    if record_path.is_file():
        record = json.loads(record_path.read_text(encoding="utf-8"))
        commit = record.get("source", {}).get("git", {}).get("commit_hash")
        print(f"Export record binds to canonical commit: {commit}")

    if not (added or removed or changed):
        print(f"Verified {len(actual)} staged files against {MANIFEST_NAME}: no drift.")
        return 0

    print("DRIFT DETECTED between staged tree and manifest:")
    for path in added:
        print(f"  added:   {path}")
    for path in removed:
        print(f"  removed: {path}")
    for path in changed:
        print(f"  changed: {path}")
    return 1


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "DEC-059 sanitized public-export pipeline for SWBPIPE. "
            "Plan-only by default; --execute stages into --target; --verify "
            "re-hashes an existing export against its manifest."
        )
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Stage the export (requires --target).",
    )
    parser.add_argument(
        "--target",
        type=Path,
        default=None,
        help="Explicit output directory for --execute (must be outside the "
        "source root).",
    )
    parser.add_argument(
        "--force",
        action="store_true",
        help="Replace a non-empty --target directory.",
    )
    parser.add_argument(
        "--verify",
        type=Path,
        default=None,
        help="Verify an existing export directory against its manifest.",
    )
    parser.add_argument(
        "--source-root",
        type=Path,
        default=ROOT,
        help="Project tree to export (default: this project root; used by "
        "self-tests with synthetic trees).",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    source_root = args.source_root.resolve()

    if args.verify is not None:
        if args.execute:
            print("error: --verify and --execute are mutually exclusive", file=sys.stderr)
            return 2
        return run_verify(args.verify.resolve())

    if args.execute:
        if args.target is None:
            print("error: --execute requires --target <dir>", file=sys.stderr)
            return 2
        return run_execute(source_root, args.target, args.force, ROOT)

    if args.target is not None or args.force:
        print("error: --target/--force require --execute", file=sys.stderr)
        return 2

    return run_plan(source_root)


if __name__ == "__main__":
    sys.exit(main())
