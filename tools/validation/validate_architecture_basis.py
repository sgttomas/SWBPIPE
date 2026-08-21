#!/usr/bin/env python3
"""Validate the reference-only PKG-00 architecture-basis corpus.

PKG-00 is deliberately outside the PROJECT/SOFTWARE production-contract
grammar.  This validator therefore checks its separate D-43 reading contract
without teaching production-format tooling to accept ArchitectureBasis.md.
"""

from __future__ import annotations

import argparse
import hashlib
import re
from pathlib import Path


PACKAGE_NAME = "PKG-00_Software Architecture Runway"
EXPECTED_IDS = tuple(f"DEL-00-{index:02d}" for index in range(1, 9))
FORMER_PRODUCTION_FILES = (
    "Datasheet.md",
    "Specification.md",
    "Procedure.md",
    "Guidance.md",
    "ScopeOfWork.md",
)
CONTEXT_NOTICE = "Preserved setup context — superseded for current consumption."
# Conscious pin update (2026-08-21): SOFTWARE_DECOMP.md advanced 0.11 -> 0.12
# through accepted SCA-009. The owner accepted Gate 5 and advanced the live
# decomposition pointer to revision 0.12; see SCA-009 ACCEPTANCE_RECORD.md and
# Handoff_State.md. The D-43 reading contract itself is unchanged; only the
# accepted-revision pin moves.
EXPECTED_DECOMP_REVISION = "0.12"


def _read(path: Path, errors: list[str]) -> str:
    if not path.is_file():
        errors.append(f"missing required file: {path}")
        return ""
    return path.read_text(encoding="utf-8")


def _member_dirs(working_root: Path) -> dict[str, Path]:
    members: dict[str, Path] = {}
    if not working_root.is_dir():
        return members
    for path in sorted(working_root.iterdir()):
        if not path.is_dir():
            continue
        match = re.match(r"^(DEL-00-\d{2})_", path.name)
        if match:
            members[match.group(1)] = path
    return members


def _manifest_hashes(text: str) -> dict[str, str]:
    pattern = re.compile(
        r"^\| (DEL-00-\d{2}) \| ArchitectureBasis\.md \| `([0-9a-f]{64})` \|$",
        re.MULTILINE,
    )
    return dict(pattern.findall(text))


def validate(project_root: Path) -> list[str]:
    errors: list[str] = []
    execution = project_root / "execution"
    package_root = execution / PACKAGE_NAME
    working_root = package_root / "1_Working"
    manifest_path = package_root / "CONSOLIDATION_MANIFEST.md"
    decomp_path = execution / "_Decomposition" / "SOFTWARE_DECOMP.md"
    decision_path = (
        execution
        / "_Coordination"
        / "_DECISIONS"
        / "D-43_pkg00_architecture_basis_consolidation.md"
    )

    manifest = _read(manifest_path, errors)
    decomp = _read(decomp_path, errors)
    _read(decision_path, errors)
    expected_hashes = _manifest_hashes(manifest)
    members = _member_dirs(working_root)

    if set(members) != set(EXPECTED_IDS):
        errors.append(
            "PKG-00 member census mismatch: "
            f"expected {list(EXPECTED_IDS)}, found {sorted(members)}"
        )

    if f"revision: {EXPECTED_DECOMP_REVISION}" not in decomp:
        errors.append(
            f"SOFTWARE_DECOMP.md is not revision {EXPECTED_DECOMP_REVISION}"
        )

    for deliverable_id in EXPECTED_IDS:
        member = members.get(deliverable_id)
        if member is None:
            continue
        suffix = deliverable_id.removeprefix("DEL-00-")
        ab_id = f"AB-00-{suffix}"

        basis_path = member / "ArchitectureBasis.md"
        basis = _read(basis_path, errors)
        context = _read(member / "_CONTEXT.md", errors)
        dependencies = _read(member / "_DEPENDENCIES.md", errors)
        references = _read(member / "_REFERENCES.md", errors)
        status = _read(member / "_STATUS.md", errors)

        required_basis_fragments = (
            "<!-- chirality-architecture-basis/v1 -->",
            f"# {deliverable_id} ",
            "Classification: `ARCHITECTURE_BASIS_REFERENCE`",
            f"architecture-basis row {ab_id}",
            "Dependency direction: one-way",
            "D-43",
        )
        for fragment in required_basis_fragments:
            if fragment not in basis:
                errors.append(f"{basis_path}: missing {fragment!r}")

        if basis and not basis.startswith("<!-- chirality-architecture-basis/v1 -->\n"):
            errors.append(f"{basis_path}: architecture-basis marker must be first")

        for former_name in FORMER_PRODUCTION_FILES:
            if (member / former_name).exists():
                errors.append(f"{member}: unexpected live production file {former_name}")

        expected_hash = expected_hashes.get(deliverable_id)
        if expected_hash is None:
            errors.append(f"{manifest_path}: missing successor hash for {deliverable_id}")
        elif basis:
            actual_hash = hashlib.sha256(basis.encode("utf-8")).hexdigest()
            if actual_hash != expected_hash:
                errors.append(
                    f"{basis_path}: hash {actual_hash} != manifest {expected_hash}"
                )

        if CONTEXT_NOTICE not in context:
            errors.append(f"{member / '_CONTEXT.md'}: missing supersession notice")
        if "**Accepted Revision:** 0.7" not in context:
            errors.append(f"{member / '_CONTEXT.md'}: setup revision 0.7 not preserved")

        required_dependency_fragments = (
            "DAG-007_ACTIVE_GRAPH",
            "SATISFIED_BY_DAG_007_AUTHORITY",
            "currently records `IN_PROGRESS`",
            "amended through SCA-006",
        )
        for fragment in required_dependency_fragments:
            if fragment not in dependencies:
                errors.append(f"{member / '_DEPENDENCIES.md'}: missing {fragment!r}")
        for stale_fragment in (
            "SATISFIED_BY_DAG_006_AUTHORITY",
            "PKG-00 remains `SEMANTIC_READY`",
        ):
            if stale_fragment in dependencies:
                errors.append(
                    f"{member / '_DEPENDENCIES.md'}: stale current-state claim {stale_fragment!r}"
                )

        for fragment in (
            "## Current Reading Contract",
            "ArchitectureBasis.md",
            "D-43_pkg00_architecture_basis_consolidation.md",
            "CONSOLIDATION_MANIFEST.md",
        ):
            if fragment not in references:
                errors.append(f"{member / '_REFERENCES.md'}: missing {fragment!r}")

        if "**Current State:** IN_PROGRESS" not in status:
            errors.append(f"{member / '_STATUS.md'}: expected IN_PROGRESS lifecycle")
        if f"|{ab_id}|{deliverable_id}|" not in decomp:
            errors.append(f"{decomp_path}: missing {ab_id}/{deliverable_id} row")

    if set(expected_hashes) != set(EXPECTED_IDS):
        errors.append(
            "manifest successor census mismatch: "
            f"expected {list(EXPECTED_IDS)}, found {sorted(expected_hashes)}"
        )
    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--project-root",
        type=Path,
        default=Path(__file__).resolve().parents[2],
        help="chirality-piping project root (defaults from this script)",
    )
    args = parser.parse_args()
    project_root = args.project_root.resolve()
    errors = validate(project_root)
    if errors:
        print(f"PKG-00 architecture-basis validation: FAIL ({len(errors)} findings)")
        for error in errors:
            print(f"- {error}")
        return 1
    print("PKG-00 architecture-basis validation: PASS (8 members)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
