#!/usr/bin/env python3
"""DEC-057 packaging path: zip the Tauri `.app`, checksum it, record it (§8).

Implements the ruled v0.1 artifact shape from the D-06 O-A ruling (`DEC-057`,
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12; packet
`execution/_Coordination/_DECISIONS/D-06_release_matrix_installers_publication.md`):

- release matrix: macOS Apple Silicon (``aarch64-apple-darwin``) only;
- installer format: the Tauri ``.app`` bundle distributed as a zip archive
  with a published SHA-256 checksum;
- signing/notarization: none for v0.1 (re-decided at ``D-06b``); artifact
  authenticity is carried by the checksum + the commit-bound ``DEC-025``
  evidence-sweep artifact + the ``docs/BUILD_AND_RELEASE.md`` §8 release
  artifact record emitted here, plus the unsigned-install caveat.

The zip is deterministic where feasible: entries are sorted, timestamps are
fixed to the bound commit's committer time (UTC), unix permissions and
symlinks are preserved, and no platform extra fields are written. Byte-exact
reproducibility across hosts is limited only by the host zlib's compressed
stream; the decoded member bytes are exact.

This script is packaging mechanics only. It performs no release act: no
publication, no upload, no version tag, no signing, no notarization, and it
creates no release-readiness claim. Release publication additionally
requires the D-20 scan record, gate records, and the human release
authority's acceptance (`docs/BUILD_AND_RELEASE.md` §6/§8).
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import plistlib
import stat
import struct
import subprocess
import sys
import time
import zipfile
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 1
ARTIFACT_KIND = "openpipestress.release_artifact_record"
DECISION_BASIS = "DEC-057"
SIGNING_RE_DECISION = "D-06b"

RELEASE_TARGET_TRIPLE = "aarch64-apple-darwin"
INSTALLER_FORMAT = "tauri_app_bundle_zip"
RULED_BUILD_COMMAND = "npm run tauri -- build --bundles app"

TAURI_CONF_RELPATH = Path("apps/desktop/src-tauri/tauri.conf.json")
DEFAULT_BUNDLE_DIR = Path("apps/desktop/src-tauri/target/release/bundle/macos")
DEFAULT_OUTPUT_DIR = "dist/release"
DEFAULT_RECORDS_DIR = "validation/evidence/release_artifacts"

# Mach-O constants for the ruled-matrix architecture check.
MH_MAGIC_64_LE = b"\xcf\xfa\xed\xfe"
CPU_TYPE_ARM64 = 0x0100000C

UNSIGNED_INSTALL_CAVEAT = (
    "This build is not code-signed or notarized (DEC-057: v0.1 ships "
    "unsigned; signing/notarization is re-decided at D-06b). macOS "
    "Gatekeeper will quarantine the downloaded app. Before opening it, "
    "verify the artifact's SHA-256 checksum against the published .sha256 "
    "file and the commit-bound DEC-025 evidence-sweep artifact referenced "
    "in the release artifact record; then open the app explicitly "
    "(Control-click > Open, or remove the quarantine attribute). "
    "Authenticity is carried by the checksum + the commit-bound sweep "
    "artifact + the release artifact record, not by an OS code signature."
)

BOUNDARY_NOTE = (
    "Packaging mechanics only. Not a release act, publication, version tag, "
    "signing, notarization, release-readiness claim, professional approval, "
    "certification, sealing, authentication, or code-compliance "
    "determination."
)

PUBLICATION_POSTURE = (
    "not_published: DEC-057 rules GitHub Releases on the prospective public "
    "sanitized-export OpenPipeStress repository (the D-05b surface); until "
    "that repository exists, artifacts are recorded locally per "
    "docs/BUILD_AND_RELEASE.md §8 and distributed directly by the owner."
)

CHAIN_VERIFIED = "verified"
CHAIN_DIRTY = "working_tree_dirty"
CHAIN_GIT_UNVERIFIED = "git_state_unverified"
CHAIN_NO_SWEEP = "missing_sweep_artifact"
CHAIN_SWEEP_COMMIT_MISMATCH = "sweep_commit_mismatch"
CHAIN_SWEEP_NOT_PASS = "sweep_not_pass"
CHAIN_SWEEP_DIRTY = "sweep_working_tree_dirty"
CHAIN_SWEEP_UNREADABLE = "sweep_artifact_unreadable"


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


def load_app_identity(root: Path) -> dict:
    """Product name/version from the Tauri config (single naming source)."""
    conf = json.loads((root / TAURI_CONF_RELPATH).read_text(encoding="utf-8"))
    return {
        "product_name": conf["productName"],
        "version": conf["version"],
        "identifier": conf["identifier"],
        "bundle_active": bool(conf.get("bundle", {}).get("active")),
        "bundle_targets": conf.get("bundle", {}).get("targets"),
    }


def artifact_basename(product_name: str, version: str) -> str:
    slug = product_name.replace(" ", "-")
    return f"{slug}_{version}_{RELEASE_TARGET_TRIPLE}.app"


def macho_is_arm64(executable: Path) -> bool:
    """True when the Mach-O header is a thin little-endian arm64 image.

    The ruled v0.1 matrix is aarch64-apple-darwin only; fat/universal or
    non-arm64 images are outside the ruled artifact shape.
    """
    try:
        header = executable.read_bytes()[:8]
    except OSError:
        return False
    if len(header) < 8 or header[:4] != MH_MAGIC_64_LE:
        return False
    cputype = struct.unpack("<I", header[4:8])[0]
    return cputype == CPU_TYPE_ARM64


def app_main_executable(app_path: Path) -> Path:
    """The bundle's main executable, named by Info.plist CFBundleExecutable."""
    executable_name = app_path.stem
    info_plist = app_path / "Contents" / "Info.plist"
    try:
        with info_plist.open("rb") as handle:
            executable_name = plistlib.load(handle).get(
                "CFBundleExecutable", executable_name
            )
    except (OSError, plistlib.InvalidFileException):
        pass
    return app_path / "Contents" / "MacOS" / executable_name


def commit_timestamp(root: Path) -> int | None:
    """Committer time of HEAD, the deterministic zip timestamp source."""
    try:
        completed = subprocess.run(
            ("git", "log", "-1", "--format=%ct"),
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    try:
        return int(completed.stdout.strip())
    except ValueError:
        return None


def zip_date_time(epoch: int | None) -> tuple[int, int, int, int, int, int]:
    """UTC zip timestamp from an epoch, clamped to the zip 1980 floor."""
    if epoch is None:
        return (1980, 1, 1, 0, 0, 0)
    parts = time.gmtime(max(epoch, 315532800))
    if parts.tm_year < 1980:
        return (1980, 1, 1, 0, 0, 0)
    return (
        parts.tm_year,
        parts.tm_mon,
        parts.tm_mday,
        parts.tm_hour,
        parts.tm_min,
        parts.tm_sec,
    )


def _zip_entries(app_path: Path) -> list[tuple[str, Path]]:
    """Sorted (arcname, path) pairs for the bundle, root dir included."""
    prefix = app_path.name
    entries: list[tuple[str, Path]] = [(f"{prefix}/", app_path)]
    for path in sorted(app_path.rglob("*"), key=lambda p: str(p)):
        rel = path.relative_to(app_path).as_posix()
        arcname = f"{prefix}/{rel}"
        if path.is_dir() and not path.is_symlink():
            arcname += "/"
        entries.append((arcname, path))
    return entries


def build_deterministic_zip(
    app_path: Path,
    zip_path: Path,
    date_time: tuple[int, int, int, int, int, int],
) -> None:
    """Write the .app to a deterministic zip.

    Determinism levers: sorted entries, a fixed commit-derived UTC
    timestamp, unix `external_attr` permissions (symlinks preserved as
    S_IFLNK entries), `create_system=3`, no extra fields, deflate level 9.
    """
    zip_path.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(zip_path, "w") as archive:
        for arcname, path in _zip_entries(app_path):
            st = path.lstat()
            info = zipfile.ZipInfo(filename=arcname, date_time=date_time)
            info.create_system = 3
            mode = st.st_mode
            if stat.S_ISLNK(mode):
                info.external_attr = (stat.S_IFLNK | 0o755) << 16
                info.compress_type = zipfile.ZIP_STORED
                archive.writestr(info, str(path.readlink()))
            elif stat.S_ISDIR(mode):
                info.external_attr = ((mode & 0xFFFF) << 16) | 0x10
                info.compress_type = zipfile.ZIP_STORED
                archive.writestr(info, b"")
            else:
                info.external_attr = (mode & 0xFFFF) << 16
                info.compress_type = zipfile.ZIP_DEFLATED
                with path.open("rb") as source, archive.open(
                    info, "w", force_zip64=True
                ) as target:
                    while True:
                        block = source.read(1 << 20)
                        if not block:
                            break
                        target.write(block)


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def checksum_line(digest: str, zip_name: str) -> str:
    """`shasum -a 256`-compatible checksum file content."""
    return f"{digest}  {zip_name}\n"


def _evidence_only_delta(
    sweep_commit: str, record_commit: str, root: Path
) -> tuple[bool, list[str]]:
    """True when the two commits differ only under ``validation/evidence/``.

    The DEC-025 gate pattern appends sweep summaries as evidence-only
    closeout commits, so the record's HEAD may sit one or more evidence
    commits after the sweep's bound commit with an identical code state
    (the `apps/desktop/SMOKE.md` TP-MAC-141 binary-provenance precedent:
    "the sweep commit added evidence only").
    """
    try:
        completed = subprocess.run(
            # --relative: the project may live inside a larger repository;
            # the evidence-path check below is project-root-relative.
            (
                "git",
                "diff",
                "--name-only",
                "--relative",
                sweep_commit,
                record_commit,
            ),
            cwd=root,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError:
        return False, []
    if completed.returncode != 0:
        return False, []
    paths = [line for line in completed.stdout.splitlines() if line]
    return all(path.startswith("validation/evidence/") for path in paths), paths


def evaluate_chain(
    record_commit: str | None,
    git_state: dict,
    sweep_path: Path | None,
    root: Path,
) -> dict:
    """DEC-057 authenticity chain: checksum + commit-bound sweep + record."""
    sweep_module = _sweep_module()
    chain: dict = {
        "sweep_artifact": None,
        "sweep_commit": None,
        "sweep_overall_status": None,
        "record_commit": record_commit,
        "evidence_only_delta_paths": [],
        "status": CHAIN_NO_SWEEP,
    }
    if sweep_path is not None:
        try:
            chain["sweep_artifact"] = sweep_path.resolve().relative_to(
                root.resolve()
            ).as_posix()
        except ValueError:
            chain["sweep_artifact"] = str(sweep_path)
        try:
            sweep = json.loads(sweep_path.read_text(encoding="utf-8"))
        except (OSError, ValueError):
            chain["status"] = CHAIN_SWEEP_UNREADABLE
            return chain
        chain["sweep_commit"] = (sweep.get("git") or {}).get("commit_hash")
        chain["sweep_overall_status"] = sweep.get("overall_status")
    if sweep_module.git_state_unverified(git_state):
        chain["status"] = CHAIN_GIT_UNVERIFIED
        return chain
    if git_state.get("working_tree_dirty"):
        chain["status"] = CHAIN_DIRTY
        return chain
    if sweep_path is None:
        return chain
    if (sweep.get("git") or {}).get("working_tree_dirty"):
        chain["status"] = CHAIN_SWEEP_DIRTY
        return chain
    if chain["sweep_commit"] != record_commit:
        if not chain["sweep_commit"] or not record_commit:
            chain["status"] = CHAIN_SWEEP_COMMIT_MISMATCH
            return chain
        evidence_only, delta_paths = _evidence_only_delta(
            chain["sweep_commit"], record_commit, root
        )
        if not evidence_only:
            chain["status"] = CHAIN_SWEEP_COMMIT_MISMATCH
            return chain
        chain["evidence_only_delta_paths"] = delta_paths
    if chain["sweep_overall_status"] != "pass":
        chain["status"] = CHAIN_SWEEP_NOT_PASS
        return chain
    chain["status"] = CHAIN_VERIFIED
    return chain


def build_record(
    *,
    identity: dict,
    git_state: dict,
    runtime: dict,
    zip_relpath: str,
    checksum_relpath: str,
    digest: str,
    zip_size: int,
    chain: dict,
    started_utc: str,
) -> dict:
    """The docs/BUILD_AND_RELEASE.md §8 release-artifact record body."""
    return {
        "artifact": ARTIFACT_KIND,
        "schema_version": SCHEMA_VERSION,
        "decision_basis": DECISION_BASIS,
        "boundary_note": BOUNDARY_NOTE,
        "recorded_utc": started_utc,
        "git": git_state,
        "runtime": runtime,
        "evidence_profile": "DEC-025 five-surface sweep (commit-bound)",
        "application": {
            "product_name": identity["product_name"],
            "version": identity["version"],
            "identifier": identity["identifier"],
        },
        "release_matrix": {
            "target_triple": RELEASE_TARGET_TRIPLE,
            "platform": "macos",
            "basis": (
                "DEC-057 O-A: macOS Apple Silicon only; Windows/Linux enter "
                "only through the evidence-gated matrix-expansion rider"
            ),
        },
        "installer_format": INSTALLER_FORMAT,
        "build_command": RULED_BUILD_COMMAND,
        "artifacts": [
            {
                "kind": "app_bundle_zip",
                "path": zip_relpath,
                "size_bytes": zip_size,
                "sha256": digest,
                "checksum_file": checksum_relpath,
            }
        ],
        "signing": {
            "state": "unsigned",
            "notarization": "none",
            "basis": DECISION_BASIS,
            "re_decision": SIGNING_RE_DECISION,
        },
        "publication": {"state": PUBLICATION_POSTURE},
        "authenticity_chain": chain,
        "unsigned_install_caveat": UNSIGNED_INSTALL_CAVEAT,
        "validation_status": chain["sweep_overall_status"],
        "known_limitations": [
            "Unsigned and unnotarized (DEC-057; re-decided at D-06b).",
            "No publication target exists yet; local record + direct "
            "distribution only (DEC-057/DEC-059).",
            "A green sweep is development evidence, not a release claim; "
            "release publication requires the D-20 scan record, gate "
            "records, and human release-authority acceptance.",
        ],
        "human_review": {
            "state": "none_recorded",
            "note": (
                "Release publication requires the human release authority's "
                "acceptance (docs/BUILD_AND_RELEASE.md §6/§8); this record "
                "is packaging evidence only."
            ),
        },
    }


def record_filename(record: dict) -> str:
    sweep_module = _sweep_module()
    commit = (record["git"]["commit_hash"] or "nocommit")[:12]
    if sweep_module.git_state_unverified(record["git"]):
        dirty = "-gitunverified"
    elif record["git"]["working_tree_dirty"]:
        dirty = "-dirty"
    else:
        dirty = ""
    stamp = record["recorded_utc"].replace("-", "").replace(":", "")
    stamp = stamp.split("+")[0] + "Z"
    return f"RELEASE_ARTIFACT_{stamp}_{commit}{dirty}.json"


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Package the built Tauri .app as a deterministic zip with a "
            "SHA-256 checksum and emit the §8 release-artifact record "
            "(DEC-057 shape). Packaging mechanics only; no release act."
        )
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Package and record. Without this flag, only print the plan.",
    )
    parser.add_argument(
        "--app-path",
        default=None,
        help=(
            "Path to the built .app bundle (default: the Tauri bundle "
            f"output under {DEFAULT_BUNDLE_DIR})."
        ),
    )
    parser.add_argument(
        "--sweep-artifact",
        default=None,
        help=(
            "Path to the commit-bound DEC-025 sweep summary JSON that "
            "anchors the authenticity chain."
        ),
    )
    parser.add_argument(
        "--output-dir",
        default=DEFAULT_OUTPUT_DIR,
        help=f"Zip/checksum output dir, repo-relative (default: {DEFAULT_OUTPUT_DIR}).",
    )
    parser.add_argument(
        "--records-dir",
        default=DEFAULT_RECORDS_DIR,
        help=f"Record output dir, repo-relative (default: {DEFAULT_RECORDS_DIR}).",
    )
    parser.add_argument(
        "--repo-root",
        default=str(ROOT),
        help="Repository root. Defaults to the root containing this script.",
    )
    parser.add_argument(
        "--skip-arch-check",
        action="store_true",
        help=(
            "Skip the Mach-O arm64 check (testing only; the ruled matrix "
            "is aarch64-apple-darwin)."
        ),
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = Path(args.repo_root).resolve()
    if not root.exists():
        print(f"missing repository root: {root}", file=sys.stderr)
        return 2

    identity = load_app_identity(root)
    basename = artifact_basename(identity["product_name"], identity["version"])
    app_path = (
        Path(args.app_path).resolve()
        if args.app_path
        else root / DEFAULT_BUNDLE_DIR / f"{identity['product_name']}.app"
    )
    output_dir = root / args.output_dir
    zip_path = output_dir / f"{basename}.zip"
    checksum_path = output_dir / f"{basename}.zip.sha256"

    mode = "execute" if args.execute else "dry-run"
    print(f"OpenPipeStress release-artifact packaging ({mode}) — {DECISION_BASIS}")
    print(f"repo:            {root}")
    print(f"app bundle:      {app_path}")
    print(f"target triple:   {RELEASE_TARGET_TRIPLE} (ruled matrix)")
    print(f"installer form:  {INSTALLER_FORMAT}")
    print(f"zip:             {zip_path}")
    print(f"checksum:        {checksum_path}")
    print(f"records dir:     {root / args.records_dir}")
    print(f"sweep artifact:  {args.sweep_artifact or '(none provided)'}")
    print(f"boundary:        {BOUNDARY_NOTE}")
    if not args.execute:
        return 0

    if not identity["bundle_active"]:
        print("tauri.conf.json bundle.active is not enabled", file=sys.stderr)
        return 2
    if not app_path.is_dir():
        print(
            f"missing .app bundle: {app_path}\n"
            f"build it first: cd apps/desktop && {RULED_BUILD_COMMAND}",
            file=sys.stderr,
        )
        return 2
    executable = app_main_executable(app_path)
    if not args.skip_arch_check and not macho_is_arm64(executable):
        print(
            f"main executable is not a thin arm64 Mach-O image: {executable}\n"
            f"the ruled v0.1 matrix is {RELEASE_TARGET_TRIPLE} only (DEC-057)",
            file=sys.stderr,
        )
        return 2

    sweep_module = _sweep_module()
    started = datetime.now(timezone.utc).isoformat(timespec="seconds")
    git_state = sweep_module.collect_git_state(root)
    runtime = sweep_module.collect_runtime_versions(root)

    date_time = zip_date_time(commit_timestamp(root))
    build_deterministic_zip(app_path, zip_path, date_time)
    digest = sha256_file(zip_path)
    checksum_path.write_text(checksum_line(digest, zip_path.name), encoding="utf-8")

    sweep_path = Path(args.sweep_artifact) if args.sweep_artifact else None
    if sweep_path is not None and not sweep_path.is_absolute():
        sweep_path = root / sweep_path
    chain = evaluate_chain(
        git_state.get("commit_hash"), git_state, sweep_path, root
    )
    chain["checksum_sha256"] = digest

    def _rel(path: Path) -> str:
        try:
            return path.resolve().relative_to(root).as_posix()
        except ValueError:
            return str(path)

    record = build_record(
        identity=identity,
        git_state=git_state,
        runtime=runtime,
        zip_relpath=_rel(zip_path),
        checksum_relpath=_rel(checksum_path),
        digest=digest,
        zip_size=zip_path.stat().st_size,
        chain=chain,
        started_utc=started,
    )
    records_dir = root / args.records_dir
    records_dir.mkdir(parents=True, exist_ok=True)
    record_path = records_dir / record_filename(record)
    record_path.write_text(
        json.dumps(record, indent=2, sort_keys=False) + "\n", encoding="utf-8"
    )

    print("")
    print(f"[package-release-artifact] zip:      {_rel(zip_path)}")
    print(f"[package-release-artifact] sha256:   {digest}")
    print(f"[package-release-artifact] checksum: {_rel(checksum_path)}")
    print(f"[package-release-artifact] record:   {_rel(record_path)}")
    print(f"[package-release-artifact] chain:    {chain['status']}")
    if chain["status"] != CHAIN_VERIFIED:
        print(
            "[package-release-artifact] authenticity chain is not verified — "
            "the record states why; a publishable artifact requires a clean "
            "committed HEAD and a matching passing sweep artifact",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
