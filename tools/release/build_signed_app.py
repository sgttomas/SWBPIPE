#!/usr/bin/env python3
"""Build the Developer ID signed SWBPIPE `.app` (D-06b, owner, 2026-10-23).

Runs `npm run tauri -- build --bundles app --target aarch64-apple-darwin` in `apps/desktop` with
`APPLE_SIGNING_IDENTITY="Developer ID Application"`, after confirming that the
keychain holds exactly one valid Developer ID Application identity. That kind
then names one identity, so codesign never meets an ambiguous match, whose
diagnostic would list identity names. Tauri's notarization variables are
removed from the build environment, because notarization runs separately
through a keychain profile (`notarize_app.py`).

The build's output is forwarded line by line through a redactor that masks
signing identity names, team identifiers and certificate SHA-1 hashes. Only
counts and exit codes are reported about the keychain. No identity, credential
or certificate is read into the repository or written to it.

One explicit supplier source is mandatory for execution. A private stock-resource
overlay survives through the build and post-build bundle verification.
Local packaging mechanics only; not a release act.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DESKTOP = ROOT / "apps" / "desktop"
sys.path.insert(0, str(ROOT / "tools"))
import agent_supplier_bundle as supplier

TARGET = "aarch64-apple-darwin"
IDENTITY_KIND = "Developer ID Application"
BUILD_COMMAND = ("npm", "run", "tauri", "--", "build", "--bundles", "app", "--target", TARGET)
# Tauri notarizes when these are set; v0.1 notarizes through a keychain profile instead.
TAURI_NOTARIZATION_VARIABLES = (
    "APPLE_ID", "APPLE_PASSWORD", "APPLE_TEAM_ID",
    "APPLE_API_KEY", "APPLE_API_ISSUER", "APPLE_API_KEY_PATH",
    "APPLE_CERTIFICATE", "APPLE_CERTIFICATE_PASSWORD",
)

_IDENTITY_KINDS = (
    r"Developer ID Application|Developer ID Installer|Apple Development|"
    r"Apple Distribution|Mac Developer|3rd Party Mac Developer Application|"
    r"3rd Party Mac Developer Installer|iPhone Developer|iPhone Distribution"
)
_REDACTIONS = (
    # Everything after an identity kind, to the end of the line: names may hold quotes.
    (re.compile(rf"({_IDENTITY_KINDS}): .*"), r"\1: <redacted>"),
    (re.compile(r"\b[0-9A-Fa-f]{40}\b"), "<redacted-sha1>"),
    (re.compile(r"(TeamIdentifier=)\S+"), r"\1<redacted>"),
    (re.compile(r"\([A-Z0-9]{10}\)"), "(<redacted-team>)"),
)


def redact(line: str) -> str:
    """Mask identity names, team identifiers and certificate hashes."""
    for pattern, replacement in _REDACTIONS:
        line = pattern.sub(replacement, line)
    return line


def count_identities(find_identity_output: str, kind: str = IDENTITY_KIND) -> int:
    """Valid identities of one kind in `security find-identity -v -p codesigning`."""
    return sum(1 for line in find_identity_output.splitlines() if f'"{kind}: ' in line)


def keychain_identity_count() -> int | None:
    try:
        completed = subprocess.run(
            ["security", "find-identity", "-v", "-p", "codesigning"],
            capture_output=True, text=True, check=False,
        )
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return count_identities(completed.stdout)


def build_environment(base: dict[str, str]) -> dict[str, str]:
    env = {key: value for key, value in base.items() if key not in TAURI_NOTARIZATION_VARIABLES}
    env["APPLE_SIGNING_IDENTITY"] = IDENTITY_KIND
    return env


def built_resources(env: dict[str, str]) -> Path:
    """Tauri runs Cargo from src-tauri; relative CARGO_TARGET_DIR is native."""
    cargo_root = DESKTOP / "src-tauri"
    target_dir = Path(env.get("CARGO_TARGET_DIR", "target"))
    if not target_dir.is_absolute():
        target_dir = cargo_root / target_dir
    return target_dir / TARGET / "release/bundle/macos/SWBPIPE.app/Contents/Resources"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--execute", action="store_true", help="Run the signed build.")
    source = parser.add_mutually_exclusive_group()
    source.add_argument("--supplier-archive", type=Path)
    source.add_argument("--supplier-binary", type=Path)
    source.add_argument("--supplier-download", action="store_true")
    args = parser.parse_args(sys.argv[1:] if argv is None else argv)

    print(f'build: (apps/desktop) APPLE_SIGNING_IDENTITY="{IDENTITY_KIND}" {" ".join(BUILD_COMMAND)} --config <private-resource-overlay>')
    selected = args.supplier_archive is not None or args.supplier_binary is not None or args.supplier_download
    if not args.execute:
        print("dry run: pass --execute and one explicit supplier archive/binary/download choice to build and sign")
        return 0
    if not selected:
        print("signed build requires one explicit supplier source; no fallback exists", file=sys.stderr)
        return 2
    count = keychain_identity_count()
    if count != 1:
        found = "unreadable" if count is None else str(count)
        print(
            f"expected exactly one valid {IDENTITY_KIND} identity in the keychain, found {found}; "
            "refusing so codesign cannot meet an ambiguous match",
            file=sys.stderr,
        )
        return 2
    try:
        with tempfile.TemporaryDirectory(prefix="swbpipe-supplier-") as temporary:
            overlay = supplier.prepare(
                Path(temporary).resolve() / "overlay", TARGET, archive=args.supplier_archive,
                binary=args.supplier_binary, download=args.supplier_download,
            )
            env = build_environment(dict(os.environ))
            process = subprocess.Popen(
                (*BUILD_COMMAND, "--config", str(overlay)), cwd=DESKTOP, env=env,
                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, errors="replace", bufsize=1,
            )
            try:
                assert process.stdout is not None
                private_paths = [temporary, str(Path(temporary).resolve()), str(overlay)] + [
                    str(path) for path in (args.supplier_archive, args.supplier_binary) if path is not None
                ]
                for line in process.stdout:
                    for private in sorted(private_paths, key=len, reverse=True):
                        line = line.replace(private, "<private-supplier-path>")
                    try:
                        sys.stdout.write(redact(line))
                        sys.stdout.flush()
                    except BaseException:
                        # A downstream closure must not leave a verbose build
                        # blocked on its full pipe while we wait. Discard the
                        # rest without forwarding; retain staging until exit.
                        while process.stdout.read(64 * 1024):
                            pass
                        raise
            finally:
                # Even output-forwarding failure must not remove resources from
                # beneath the original build while it is still running.
                result = process.wait()
            if result != 0:
                return result
            supplier.verify(built_resources(env), TARGET)
    except Exception:
        # OS, acquisition and verification exceptions may contain private paths
        # or signed URLs; report only a static failure, never a false success.
        print("signed build failed to prepare, build or verify pinned supplier resources", file=sys.stderr)
        return 2
    print("signed build completed; bundled supplier and notice bytes verified")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
