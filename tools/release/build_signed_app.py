#!/usr/bin/env python3
"""Build the Developer ID signed SWBPIPE `.app` (D-06b, owner, 2026-10-23).

Runs `npm run tauri -- build --bundles app` in `apps/desktop` with
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

Local packaging mechanics only; not a release act.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DESKTOP = ROOT / "apps" / "desktop"
IDENTITY_KIND = "Developer ID Application"
BUILD_COMMAND = ("npm", "run", "tauri", "--", "build", "--bundles", "app")
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


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--execute", action="store_true", help="Run the signed build.")
    args = parser.parse_args(sys.argv[1:] if argv is None else argv)

    print(f"build: (apps/desktop) APPLE_SIGNING_IDENTITY=\"{IDENTITY_KIND}\" {' '.join(BUILD_COMMAND)}")
    if not args.execute:
        print("dry run: pass --execute to build and sign")
        return 0
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
        process = subprocess.Popen(
            BUILD_COMMAND, cwd=DESKTOP, env=build_environment(dict(os.environ)),
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, errors="replace", bufsize=1,
        )
    except OSError as error:
        print(f"build could not start: {type(error).__name__}", file=sys.stderr)
        return 2
    assert process.stdout is not None
    for line in process.stdout:
        sys.stdout.write(redact(line))
        sys.stdout.flush()
    return process.wait()


if __name__ == "__main__":
    raise SystemExit(main())
