#!/usr/bin/env python3
"""Notarize and staple the signed SWBPIPE `.app` (D-06b, owner, 2026-10-23).

The owner decided on 2026-10-23: "Proceed with signing and notarizing." The
Tauri build signs the bundle with the keychain's Developer ID Application
identity and the hardened runtime (`docs/BUILD_AND_RELEASE.md` §6.1). This
script then:

1. zips the bundle with `ditto -c -k --keepParent` for submission;
2. submits it with `xcrun notarytool submit --keychain-profile <name> --wait`;
3. on `Accepted`, staples the ticket to the bundle and validates the staple.

Credentials stay in the login keychain under a notarytool profile the owner
created with `xcrun notarytool store-credentials`. This script takes only the
profile name, from `--keychain-profile` or `SWBPIPE_NOTARY_PROFILE`, and never
reads, prints or writes a credential. Tool output can name the account or team,
so it is never echoed: the script prints fixed outcome categories, exit codes
and a submission id only when it is a well-formed UUID. For a rejected
submission, `xcrun notarytool log <id> --keychain-profile <name>` gives
Apple's reasons on the owner's terminal.

Local packaging mechanics only: no publication, upload of the release
artifact, version tag or release act. Notarization sends the bundle to Apple's
notary service, which the owner's decision authorizes. A notarized build is
not a release-readiness claim.
"""

from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from pathlib import Path


PROFILE_ENV = "SWBPIPE_NOTARY_PROFILE"
DEFAULT_APP = Path("apps/desktop/src-tauri/target/release/bundle/macos/SWBPIPE.app")
ROOT = Path(__file__).resolve().parents[2]
SUBMISSION_ID = re.compile(r"[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}")
STATUSES = frozenset({"Accepted", "Invalid", "Rejected", "In Progress"})


def run(argv: list[str]) -> subprocess.CompletedProcess:
    """Run a tool with output captured, never echoed; 127 when it is absent."""
    print("$ " + " ".join(argv), flush=True)
    try:
        return subprocess.run(argv, capture_output=True, text=True, check=False)
    except OSError:
        return subprocess.CompletedProcess(argv, 127, "", "")


def submission_outcome(stdout: str) -> tuple[str | None, str | None]:
    """(id, status) from `notarytool submit --output-format json`.

    Only a well-formed UUID id and a known status are returned; any other
    text (the JSON `message`, an account name) is discarded.
    """
    try:
        body = json.loads(stdout)
    except ValueError:
        return None, None
    if not isinstance(body, dict):
        return None, None
    raw_id, raw_status = body.get("id"), body.get("status")
    submission_id = raw_id if isinstance(raw_id, str) and SUBMISSION_ID.fullmatch(raw_id) else None
    status = raw_status if raw_status in STATUSES else ("unrecognized" if raw_status is not None else None)
    return submission_id, status


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--app-path", default=str(ROOT / DEFAULT_APP))
    parser.add_argument(
        "--keychain-profile",
        default=os.environ.get(PROFILE_ENV),
        help=f"notarytool keychain profile name (default: ${PROFILE_ENV}).",
    )
    parser.add_argument("--execute", action="store_true", help="Submit and staple.")
    args = parser.parse_args(sys.argv[1:] if argv is None else argv)

    app = Path(args.app_path).resolve()
    print(f"app bundle:       {app}")
    print(f"keychain profile: {args.keychain_profile or '(none)'}")
    if not args.execute:
        print("dry run: pass --execute to submit to Apple's notary service and staple")
        return 0
    if not app.is_dir():
        print(f"missing .app bundle: {app}", file=sys.stderr)
        return 2
    if not args.keychain_profile:
        print(
            f"no notarytool keychain profile: pass --keychain-profile or set {PROFILE_ENV}; "
            "the owner creates it with `xcrun notarytool store-credentials <name> ...`",
            file=sys.stderr,
        )
        return 2

    with tempfile.TemporaryDirectory(prefix="swbpipe-notarize-") as scratch:
        submission = Path(scratch) / f"{app.stem}.zip"
        zipped = run(["ditto", "-c", "-k", "--keepParent", str(app), str(submission)])
        if zipped.returncode != 0:
            print(f"submission zip failed: ditto exit {zipped.returncode}", file=sys.stderr)
            return 1
        submitted = run([
            "xcrun", "notarytool", "submit", str(submission),
            "--keychain-profile", args.keychain_profile,
            "--wait", "--output-format", "json",
        ])
    submission_id, status = submission_outcome(submitted.stdout)
    print(f"notarization submission: {submission_id or '(unknown)'}; status: {status or '(unknown)'}")
    if status != "Accepted":
        print(f"notarization not accepted: notarytool exit {submitted.returncode}", file=sys.stderr)
        if submission_id:
            print(
                f"for Apple's reasons: xcrun notarytool log {submission_id} "
                f"--keychain-profile {args.keychain_profile}",
                file=sys.stderr,
            )
        return 1

    stapled = run(["xcrun", "stapler", "staple", str(app)])
    validated = run(["xcrun", "stapler", "validate", str(app)])
    print(f"staple: exit {stapled.returncode}; validate: exit {validated.returncode}")
    return 0 if stapled.returncode == 0 and validated.returncode == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
