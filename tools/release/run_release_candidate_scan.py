#!/usr/bin/env python3
"""DEC-058 release-candidate protected-content scan runner and record emitter.

Implements the tooling half of the D-20/DEC-058 ruling
(`execution/_Coordination/_DECISIONS/D-20_protected_content_scan.md` section 9;
codified in `execution/_Decomposition/SOFTWARE_DECOMP.md` section 12): the
artifact-class inventory walk (AC-1..AC-6, `not_applicable` recorded rather
than silently omitted), the DEL-08-05 protected-content lint over public
text/JSON members (check (b), via the engine's `protected_content_lint_cli`
binary), the `docs/IP_AND_DATA_BOUNDARY.md` section 4 provenance-manifest
check (check (c)), the `check_release_readiness.py --profile security` pass
(check (d)), quarantine hygiene (check (e)), and the machine-readable
per-candidate record
`validation/evidence/releases/SCAN_<candidate>_<utc>_<commit12>.json`.

Boundary (DEC-058/DEC-027, F-PIP-2): this tool documents checks; it does not
clear, sign, gate, or publish anything. The scan act, per-finding
dispositions, and the legal/protected-data release-gate signature are the
owner's human acts — the owner (sole maintainer and release authority) is the
sole signatory. Every record this tool emits is UNSIGNED with the owner
sign-off block explicitly pending. A record with no findings is not a legal
clearance, release claim, release-readiness claim, professional approval,
certification, sealing, authentication, or code-compliance determination.
Local-only: no network services, no signing, no publication credentials.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import platform
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]

SCHEMA_VERSION = 1
ARTIFACT_KIND = "openpipestress.release_protected_content_scan_record"
DECISION_BASIS = "DEC-058"
DEFAULT_OUTPUT_DIR = "validation/evidence/releases"

ARTIFACT_CLASSES = {
    "AC-1": "Native project package / public transport container members",
    "AC-2": "Canonical calculation-report artifact and report templates",
    "AC-3": "Installers / platform packages and embedded resources",
    "AC-4": "Public repository / sanitized public export tree",
    "AC-5": "Public examples, fixtures, validation benchmarks, corpora",
    "AC-6": "Release notes, gate records, and release-evidence documents",
}

LINTABLE_SUFFIXES = {
    ".txt",
    ".md",
    ".json",
    ".html",
    ".htm",
    ".csv",
    ".tsv",
    ".yaml",
    ".yml",
    ".toml",
    ".xml",
    ".svg",
}

REQUIRED_PROVENANCE_FIELDS = (
    "source_name",
    "source_location",
    "source_license",
    "contributor",
    "contributor_certification",
    "redistribution_status",
    "review_status",
)

QUARANTINE_PATH_FRAGMENT = "quarantine/protected-content"

LINTER_MANIFEST = "core/reporting/protected_content_linter/Cargo.toml"
LINT_CLI_BIN = "protected_content_lint_cli"

BOUNDARY_NOTE = (
    "Software-quality and data-boundary evidence only. This record is "
    "UNSIGNED tooling output: the scan disposition and the "
    "legal/protected-data release gate are the owner's human acts (DEC-058, "
    "DEC-027). Not a legal clearance, release claim, release-readiness "
    "claim, professional approval, certification, sealing, authentication, "
    "or code-compliance determination. A clean scan is not a clearance."
)

KNOWN_LIMITATIONS = [
    "Standards-table signature detection is heuristic (DEC-058 / D-20 "
    "packet U-2): false negatives are possible; the failure direction is "
    "UnknownProvenanceReviewRequired routed to human review, never silent "
    "pass.",
    "The DEL-08-05 engine is 'heuristic plus review; cannot be sole legal "
    "control' (SOFTWARE_DECOMP.md DEL-08-05 caveat); pattern matching "
    "cannot determine redistribution rights.",
    "Markup- or binary-embedded tables may evade the dense-row heuristic; "
    "binary members are inventoried and checksummed but not linted.",
    "AC-3 installer payload-walk mechanics finalize under the D-06 ruling "
    "(D-20 packet U-3); until then AC-3 walks the paths the caller names.",
    "No cleanliness claim is made about any artifact not named in the "
    "inventory walk.",
]


def utc_stamp(moment: datetime) -> str:
    return moment.strftime("%Y%m%dT%H%M%SZ")


def run_command(command: list[str], cwd: Path) -> subprocess.CompletedProcess:
    return subprocess.run(
        command, cwd=cwd, capture_output=True, text=True, check=False
    )


def capture_version(command: list[str], cwd: Path) -> str | None:
    try:
        completed = run_command(command, cwd)
    except OSError:
        return None
    if completed.returncode != 0:
        return None
    return completed.stdout.strip() or None


def git_state(root: Path, commit_override: str | None) -> dict:
    if commit_override is not None:
        return {
            "commit_hash": commit_override,
            "commit_source": "caller_override",
            "working_tree_dirty": False,
            "dirty_paths": [],
        }
    commit = capture_version(["git", "rev-parse", "HEAD"], root)
    status = capture_version(["git", "status", "--porcelain"], root)
    dirty_paths = sorted(
        line[3:] for line in (status or "").splitlines() if line.strip()
    )
    return {
        "commit_hash": commit,
        "commit_source": "git",
        "working_tree_dirty": bool(dirty_paths),
        "dirty_paths": dirty_paths,
    }


def runtime_versions(root: Path) -> dict:
    return {
        "platform": platform.platform(),
        "python": platform.python_version(),
        "cargo": capture_version(["cargo", "--version"], root),
    }


def sha256_of(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def walk_artifact_root(class_root: Path) -> list[Path]:
    if class_root.is_file():
        return [class_root]
    return sorted(
        candidate for candidate in class_root.rglob("*") if candidate.is_file()
    )


def inventory_class(class_root: Path) -> list[dict]:
    entries = []
    for artifact in walk_artifact_root(class_root):
        relative = (
            artifact.relative_to(class_root).as_posix()
            if class_root.is_dir()
            else artifact.name
        )
        entries.append(
            {
                "path": relative,
                "size_bytes": artifact.stat().st_size,
                "sha256": sha256_of(artifact),
                "lintable": artifact.suffix.lower() in LINTABLE_SUFFIXES,
            }
        )
    return entries


def run_engine_lint(paths: list[Path], root: Path) -> dict:
    """Check (b): DEL-08-05 protected-content lint via the engine CLI."""
    if not paths:
        return {
            "status": "pass",
            "command": None,
            "finding_count": 0,
            "findings": [],
            "note": "No lintable text/JSON members in the scanned classes.",
        }
    command = [
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
    ] + [str(path) for path in paths]
    try:
        completed = run_command(command, root)
    except OSError as error:
        return {
            "status": "not_run",
            "command": " ".join(command[:8]) + " ...",
            "finding_count": 0,
            "findings": [],
            "note": f"Engine CLI unavailable: {error}. Fails toward review.",
        }
    if completed.returncode != 0:
        return {
            "status": "not_run",
            "command": " ".join(command[:8]) + " ...",
            "finding_count": 0,
            "findings": [],
            "note": (
                "Engine CLI exited "
                f"{completed.returncode}: {completed.stderr.strip()[:400]}. "
                "Fails toward review."
            ),
        }
    payload = json.loads(completed.stdout)
    findings = payload.get("findings", [])
    return {
        "status": "findings_present" if findings else "pass",
        "command": " ".join(command[:8]) + " ...",
        "engine_summary": payload.get("summary"),
        "finding_count": len(findings),
        "findings": findings,
        "note": None,
    }


def iter_provenance_records(document, json_path="$"):
    if isinstance(document, dict):
        if "redistribution_status" in document:
            yield json_path, document
        for key, value in sorted(document.items()):
            yield from iter_provenance_records(value, f"{json_path}.{key}")
    elif isinstance(document, list):
        for index, item in enumerate(document):
            yield from iter_provenance_records(item, f"{json_path}[{index}]")


def provenance_record_violations(json_path: str, record: dict) -> list[str]:
    violations = []
    for field in REQUIRED_PROVENANCE_FIELDS:
        value = record.get(field)
        if not isinstance(value, str) or not value.strip():
            violations.append(f"{json_path}: missing or empty '{field}'")
    redistribution = record.get("redistribution_status")
    if redistribution != "public_permissive":
        violations.append(
            f"{json_path}: redistribution_status is "
            f"'{redistribution}', not 'public_permissive'"
        )
    if record.get("review_status") != "accepted":
        violations.append(
            f"{json_path}: review_status is "
            f"'{record.get('review_status')}', not 'accepted'"
        )
    if record.get("source_license", "").strip().upper() == "TBD":
        violations.append(f"{json_path}: source_license is 'TBD'")
    return violations


def run_provenance_check(json_paths: list[tuple[str, Path]]) -> dict:
    """Check (c): IP_AND_DATA_BOUNDARY section 4 provenance-manifest check."""
    records_checked = 0
    violations = []
    unparseable = []
    for member_label, path in json_paths:
        try:
            document = json.loads(path.read_text(encoding="utf-8"))
        except (OSError, json.JSONDecodeError) as error:
            unparseable.append(f"{member_label}: {error}")
            continue
        for json_path, record in iter_provenance_records(document):
            records_checked += 1
            for violation in provenance_record_violations(json_path, record):
                violations.append(f"{member_label} {violation}")
    status = "findings_present" if (violations or unparseable) else "pass"
    return {
        "status": status,
        "json_members_examined": len(json_paths),
        "provenance_records_checked": records_checked,
        "violations": sorted(violations),
        "unparseable_members": sorted(unparseable),
    }


def run_security_profile(root: Path, mode: str) -> dict:
    """Check (d): private-data and real-secret checks via the security profile."""
    command = [
        sys.executable,
        "tools/release/check_release_readiness.py",
        "--profile",
        "security",
        "--execute",
    ]
    if mode != "run":
        return {
            "status": "not_run",
            "command": " ".join(command[1:]),
            "note": (
                "Security profile skipped by caller request; the record "
                "fails toward owner review."
            ),
        }
    completed = run_command(command, root)
    return {
        "status": "pass" if completed.returncode == 0 else "findings_present",
        "command": " ".join(command[1:]),
        "exit_code": completed.returncode,
        "note": None if completed.returncode == 0 else completed.stdout[-1000:],
    }


def run_quarantine_hygiene(root: Path, classes: dict) -> dict:
    """Check (e): no quarantine material inside any scanned artifact class."""
    offending = []
    for class_id, entry in sorted(classes.items()):
        if entry["status"] != "scanned":
            continue
        if QUARANTINE_PATH_FRAGMENT in entry["root"].replace("\\", "/"):
            offending.append(f"{class_id}: class root is a quarantine path")
        for artifact in entry["artifacts"]:
            if QUARANTINE_PATH_FRAGMENT in artifact["path"]:
                offending.append(f"{class_id}: {artifact['path']}")
    quarantine_dir = root / "quarantine" / "protected-content"
    return {
        "status": "findings_present" if offending else "pass",
        "quarantine_dir_exists": quarantine_dir.exists(),
        "artifacts_under_quarantine_path": sorted(offending),
    }


def normalized_findings(checks: dict) -> list[dict]:
    findings = []
    for lint_finding in checks["protected_content_lint"]["findings"]:
        findings.append(
            {
                "check": "protected_content_lint",
                "code": lint_finding["code"],
                "severity": lint_finding["severity"],
                "class": lint_finding["class"],
                "matched_policy": lint_finding["matched_policy"],
                "review_route": lint_finding["review_route"],
                "path": lint_finding["path"],
                "line": lint_finding["line"],
                "column": lint_finding["column"],
                "excerpt": lint_finding["excerpt"],
                "message": lint_finding["message"],
                "remediation": lint_finding["remediation"],
                "disposition": "pending_owner_review",
            }
        )
    for violation in checks["provenance_manifest_check"]["violations"]:
        findings.append(
            {
                "check": "provenance_manifest_check",
                "code": "UNKNOWN_PROVENANCE_REVIEW_REQUIRED",
                "severity": "BLOCKING",
                "class": "PROVENANCE_WARNING",
                "matched_policy": "IP_AND_DATA_BOUNDARY-section-4",
                "review_route": "MAINTAINER_PROVENANCE_REVIEW",
                "message": violation,
                "remediation": (
                    "Record acceptable provenance fields or keep the record "
                    "private/quarantined; unknown/TBD provenance is not "
                    "acceptable public data."
                ),
                "disposition": "pending_owner_review",
            }
        )
    for offender in checks["quarantine_hygiene"][
        "artifacts_under_quarantine_path"
    ]:
        findings.append(
            {
                "check": "quarantine_hygiene",
                "code": "PROTECTED_CONTENT_SYNTHETIC_MARKER",
                "severity": "BLOCKING",
                "class": "IP_BOUNDARY_WARNING",
                "matched_policy": "IP_AND_DATA_BOUNDARY-section-5",
                "review_route": "HUMAN_IP_REVIEW",
                "message": f"Quarantined material inside artifact class: {offender}",
                "remediation": (
                    "Remove quarantined material from the candidate; no "
                    "waiver may authorize protected-content copying."
                ),
                "disposition": "pending_owner_review",
            }
        )
    return findings


def scan_disposition(checks: dict, findings: list[dict]) -> str:
    if findings:
        return "findings_require_owner_review"
    if any(check["status"] != "pass" for check in checks.values()):
        return "incomplete_checks_require_owner_review"
    return "no_findings_recorded_pending_owner_review"


def pending_owner_sign_off() -> dict:
    return {
        "signed": False,
        "status": "pending_owner_review",
        "signatory": None,
        "signatory_role": (
            "Owner: sole maintainer and release authority (DEC-027); sole "
            "signatory of the legal/protected-data release gate (DEC-058)."
        ),
        "signed_utc": None,
        "per_finding_dispositions_recorded": False,
        "acceptance_statement": None,
        "note": (
            "This block is completed only by the owner as a human act. "
            "Tooling never signs. An unsigned record satisfies no "
            "publication precondition; a Blocking finding halts publication; "
            "no waiver may authorize protected-content copying "
            "(docs/RELEASE_QUALITY_GATES.md)."
        ),
    }


def parse_class_arguments(
    class_specs: list[str], not_applicable_specs: list[str], root: Path
) -> tuple[dict, list[str]]:
    errors = []
    classes: dict[str, dict] = {}

    def split_spec(spec: str, flag: str) -> tuple[str, str] | None:
        if "=" not in spec:
            errors.append(f"{flag} '{spec}': expected AC-n=<value>")
            return None
        class_id, _, value = spec.partition("=")
        class_id = class_id.strip()
        if class_id not in ARTIFACT_CLASSES:
            errors.append(f"{flag} '{spec}': unknown class '{class_id}'")
            return None
        if class_id in classes:
            errors.append(f"class '{class_id}' specified more than once")
            return None
        if not value.strip():
            errors.append(f"{flag} '{spec}': empty value")
            return None
        return class_id, value.strip()

    for spec in class_specs:
        parsed = split_spec(spec, "--artifact-class")
        if parsed is None:
            continue
        class_id, raw_path = parsed
        class_root = (root / raw_path).resolve() if not Path(
            raw_path
        ).is_absolute() else Path(raw_path)
        if not class_root.exists():
            errors.append(
                f"--artifact-class {class_id}: path does not exist: {raw_path}"
            )
            continue
        classes[class_id] = {
            "description": ARTIFACT_CLASSES[class_id],
            "status": "scanned",
            "root": raw_path,
            "resolved_root": str(class_root),
            "artifacts": inventory_class(class_root),
        }

    for spec in not_applicable_specs:
        parsed = split_spec(spec, "--not-applicable")
        if parsed is None:
            continue
        class_id, reason = parsed
        classes[class_id] = {
            "description": ARTIFACT_CLASSES[class_id],
            "status": "not_applicable",
            "reason": reason,
            "artifacts": [],
        }

    missing = sorted(set(ARTIFACT_CLASSES) - set(classes))
    if missing:
        errors.append(
            "every artifact class must be scanned or explicitly "
            f"not_applicable (DEC-058: never silently omitted); missing: "
            f"{', '.join(missing)}"
        )
    return classes, errors


def build_record(
    candidate: str,
    classes: dict,
    checks: dict,
    commands_run: list[dict],
    git: dict,
    now: datetime,
    root: Path,
) -> dict:
    findings = normalized_findings(checks)
    scanned = [
        class_id
        for class_id, entry in sorted(classes.items())
        if entry["status"] == "scanned"
    ]
    not_applicable = [
        class_id
        for class_id, entry in sorted(classes.items())
        if entry["status"] == "not_applicable"
    ]
    return {
        "artifact": ARTIFACT_KIND,
        "schema_version": SCHEMA_VERSION,
        "decision_basis": DECISION_BASIS,
        "boundary_note": BOUNDARY_NOTE,
        "candidate": candidate,
        "generated_utc": now.isoformat(),
        "git": git,
        "runtime": runtime_versions(root),
        "commands_run": commands_run,
        "artifact_classes": {
            class_id: classes[class_id] for class_id in sorted(classes)
        },
        "classes_scanned": scanned,
        "classes_not_applicable": not_applicable,
        "checks": checks,
        "findings": findings,
        "finding_count": len(findings),
        "blocking_finding_count": sum(
            1 for finding in findings if finding["severity"] == "BLOCKING"
        ),
        "scan_disposition": scan_disposition(checks, findings),
        "publication_gate": "blocked_pending_owner_sign_off",
        "clean_scan_is_clearance": False,
        "known_limitations": KNOWN_LIMITATIONS,
        "owner_sign_off": pending_owner_sign_off(),
    }


def record_filename(candidate: str, now: datetime, git: dict) -> str:
    commit = git.get("commit_hash") or "nocommit"
    commit12 = commit[:12]
    if git.get("working_tree_dirty"):
        commit12 += "-dirty"
    safe_candidate = "".join(
        character if character.isalnum() or character in "._-" else "-"
        for character in candidate
    )
    return f"SCAN_{safe_candidate}_{utc_stamp(now)}_{commit12}.json"


def parse_args(argv: list[str] | None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    parser.add_argument(
        "--candidate",
        required=True,
        help="Release-candidate identifier for the record filename.",
    )
    parser.add_argument(
        "--artifact-class",
        action="append",
        default=[],
        metavar="AC-n=PATH",
        help="Scan this artifact class rooted at PATH (repeatable).",
    )
    parser.add_argument(
        "--not-applicable",
        action="append",
        default=[],
        metavar="AC-n=REASON",
        help="Record this class as not shipped, with the reason (repeatable).",
    )
    parser.add_argument(
        "--security-profile",
        choices=("run", "skip"),
        default="run",
        help="Run check (d) via check_release_readiness.py, or record it "
        "not_run (fails toward owner review).",
    )
    parser.add_argument(
        "--lint-mode",
        choices=("engine", "skip"),
        default="engine",
        help="Run check (b) via the DEL-08-05 engine CLI, or record it "
        "not_run (fails toward owner review).",
    )
    parser.add_argument(
        "--output-dir",
        default=DEFAULT_OUTPUT_DIR,
        help="Directory for the scan record (default: "
        f"{DEFAULT_OUTPUT_DIR}; created on demand).",
    )
    parser.add_argument(
        "--root",
        default=str(ROOT),
        help="Project root (default: repository layout of this script).",
    )
    parser.add_argument(
        "--now-utc",
        default=None,
        help="Override the record timestamp (ISO-8601 UTC) for "
        "deterministic testing.",
    )
    parser.add_argument(
        "--commit",
        default=None,
        help="Override the bound commit hash for deterministic testing.",
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Actually run checks and write the record. Without this flag "
        "the tool prints the plan only.",
    )
    return parser.parse_args(argv)


def print_plan(args: argparse.Namespace) -> None:
    print("DEC-058 release-candidate protected-content scan (plan only)")
    print(f"  candidate:        {args.candidate}")
    print(f"  artifact classes: {args.artifact_class or '(none)'}")
    print(f"  not applicable:   {args.not_applicable or '(none)'}")
    print(f"  lint mode:        {args.lint_mode}")
    print(f"  security profile: {args.security_profile}")
    print(f"  output dir:       {args.output_dir}")
    print(
        "  note: the emitted record is UNSIGNED; the scan act and the "
        "release-gate signature are owner-only (DEC-058/DEC-027)."
    )
    print("Run again with --execute to run checks and write the record.")


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    root = Path(args.root).resolve()

    if not args.execute:
        print_plan(args)
        return 0

    classes, errors = parse_class_arguments(
        args.artifact_class, args.not_applicable, root
    )
    if errors:
        for error in errors:
            print(f"error: {error}", file=sys.stderr)
        return 2

    if args.now_utc:
        now = datetime.fromisoformat(args.now_utc)
        if now.tzinfo is None:
            now = now.replace(tzinfo=timezone.utc)
    else:
        now = datetime.now(timezone.utc)

    git = git_state(root, args.commit)

    lintable_paths: list[Path] = []
    json_members: list[tuple[str, Path]] = []
    for class_id, entry in sorted(classes.items()):
        if entry["status"] != "scanned":
            continue
        class_root = Path(entry["resolved_root"])
        for artifact in entry["artifacts"]:
            absolute = (
                class_root / artifact["path"]
                if class_root.is_dir()
                else class_root
            )
            if artifact["lintable"]:
                lintable_paths.append(absolute)
            if absolute.suffix.lower() == ".json":
                json_members.append((f"{class_id}:{artifact['path']}", absolute))

    commands_run: list[dict] = []

    inventory_check = {
        "status": "pass",
        "classes_walked": [
            class_id
            for class_id, entry in sorted(classes.items())
            if entry["status"] == "scanned"
        ],
        "artifact_count": sum(
            len(entry["artifacts"]) for entry in classes.values()
        ),
    }

    if args.lint_mode == "engine":
        lint_check = run_engine_lint(sorted(lintable_paths), root)
    else:
        lint_check = {
            "status": "not_run",
            "command": None,
            "finding_count": 0,
            "findings": [],
            "note": (
                "Lint skipped by caller request; the record fails toward "
                "owner review."
            ),
        }
    if lint_check.get("command"):
        commands_run.append(
            {"check": "protected_content_lint", "command": lint_check["command"]}
        )

    provenance_check = run_provenance_check(sorted(json_members))
    security_check = run_security_profile(root, args.security_profile)
    if security_check.get("command"):
        commands_run.append(
            {"check": "security_profile", "command": security_check["command"]}
        )
    quarantine_check = run_quarantine_hygiene(root, classes)

    checks = {
        "artifact_inventory_walk": inventory_check,
        "protected_content_lint": lint_check,
        "provenance_manifest_check": provenance_check,
        "security_profile": security_check,
        "quarantine_hygiene": quarantine_check,
    }

    record = build_record(
        args.candidate, classes, checks, commands_run, git, now, root
    )

    output_dir = Path(args.output_dir)
    if not output_dir.is_absolute():
        output_dir = root / output_dir
    output_dir.mkdir(parents=True, exist_ok=True)
    output_path = output_dir / record_filename(args.candidate, now, git)
    output_path.write_text(
        json.dumps(record, indent=2, sort_keys=False) + "\n", encoding="utf-8"
    )

    print(f"scan record written (UNSIGNED, owner sign-off pending): {output_path}")
    print(f"scan_disposition: {record['scan_disposition']}")
    print(f"findings: {record['finding_count']} "
          f"(blocking: {record['blocking_finding_count']})")
    print(BOUNDARY_NOTE)
    return 0


if __name__ == "__main__":
    sys.exit(main())
