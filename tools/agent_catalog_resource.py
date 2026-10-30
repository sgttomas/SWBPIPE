#!/usr/bin/env python3
"""Copy the authoritative Design catalog into its generated build resource.

The check establishes byte equality only; Rust validates catalog semantics.
No supplier, network or private input is consumed.
"""
import argparse
from pathlib import Path

PROJECT = Path(__file__).resolve().parents[1]
SOURCE = Path("execution/PKG-16/DEL-16-06/Design/agent-capability-catalog.json")
RESOURCE = Path("apps/desktop/src-tauri/resources/agent-capability-catalog.json")
MAX_BYTES = 256 * 1024


def synchronize(project: Path, *, check: bool) -> bool:
    source, resource = project / SOURCE, project / RESOURCE
    # Neither a tracked link nor an absent authoritative input may establish
    # equality. Filesystem mutation by another same-uid process is out of scope.
    if source.is_symlink() or resource.is_symlink():
        raise ValueError("catalog links are not admitted")
    if not source.resolve().is_relative_to(project.resolve()) or not resource.resolve().is_relative_to(project.resolve()):
        raise ValueError("catalog paths escape the project")
    with source.open("rb") as stream:
        data = stream.read(MAX_BYTES + 1)
    if not data or len(data) > MAX_BYTES:
        raise ValueError("catalog resource size is invalid")
    if check:
        with resource.open("rb") as stream:
            return stream.read(MAX_BYTES + 1) == data
    resource.parent.mkdir(parents=True, exist_ok=True)
    resource.write_bytes(data)
    return True


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="fail on an absent or differing resource")
    args = parser.parse_args()
    try:
        if not synchronize(PROJECT, check=args.check):
            print("Catalog resource differs; run python3 tools/agent_catalog_resource.py from the project.")
            return 1
    except (OSError, ValueError):
        print("Catalog resource generation/check failed; both regular inputs are required for a check.")
        return 2
    print("Catalog resource matches the authoritative Design bytes." if args.check else "Catalog resource generated from Design bytes.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
