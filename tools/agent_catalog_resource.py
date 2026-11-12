#!/usr/bin/env python3
"""Validate the canonical embedded-agent capability catalog resource.

`apps/desktop/src-tauri/resources/agent-capability-catalog.json` is the one
runtime catalog: Rust compiles it with `include_str!` and validates its
semantics; the witness and capture tools read it. DEL-16-06's Work prose
governs its requirements and is not a second source. This check admits the
resource only as a contained regular file of bounded size, strict UTF-8 JSON,
and the bytes the host-witness pin names, so a catalog edit must move the pin
in the same reviewed change. The witness pin fixture
(`fixtures/embedded_agent/agent-witness-pin.json`) states the same catalog
digest as `catalogSha256`; the check refuses any difference, so the two move
together. No supplier, network or private input is consumed.
"""
import argparse
import hashlib
import json
from pathlib import Path
import sys

PROJECT = Path(__file__).resolve().parents[1]
RESOURCE = Path("apps/desktop/src-tauri/resources/agent-capability-catalog.json")
WITNESS_PIN = Path("fixtures/embedded_agent/agent-witness-pin.json")
MAX_BYTES = 256 * 1024


def _reject_constant(token):
    raise ValueError(f"non-finite JSON number {token}")


def _unique_object(pairs):
    keys = [key for key, _ in pairs]
    if len(keys) != len(set(keys)):
        raise ValueError("duplicate JSON object key")
    return dict(pairs)


def pinned_sha256(project: Path) -> str:
    sys.path.insert(0, str(project))
    try:
        from tools import agent_host_witness
    finally:
        sys.path.pop(0)
    return agent_host_witness.CATALOG_SHA256


def witness_pin_sha256(project: Path) -> str:
    """The catalog digest the witness pin fixture states."""
    value = json.loads((project / WITNESS_PIN).read_text(encoding="utf-8"), object_pairs_hook=_unique_object,
                       parse_constant=_reject_constant)
    digest = value.get("catalogSha256") if isinstance(value, dict) else None
    if not isinstance(digest, str):
        raise ValueError("witness pin states no catalog digest")
    return digest


def validate(project: Path, expected_sha256: str) -> dict:
    """The admitted catalog value; raises ValueError or OSError otherwise."""
    resource = project / RESOURCE
    # A tracked link, a linked parent or an absent resource never establishes
    # the catalog. Filesystem mutation by another same-uid process is out of scope.
    if resource.is_symlink():
        raise ValueError("catalog links are not admitted")
    if not resource.resolve().is_relative_to(project.resolve()):
        raise ValueError("catalog path escapes the project")
    if not resource.is_file():
        raise ValueError("catalog resource is not a regular file")
    with resource.open("rb") as stream:
        data = stream.read(MAX_BYTES + 1)
    if not data or len(data) > MAX_BYTES:
        raise ValueError("catalog resource size is invalid")
    if hashlib.sha256(data).hexdigest() != expected_sha256:
        raise ValueError("catalog bytes differ from the host-witness pin")
    value = json.loads(data.decode("utf-8"), object_pairs_hook=_unique_object, parse_constant=_reject_constant)
    if not isinstance(value, dict) or not isinstance(value.get("tools"), list) or not value["tools"]:
        raise ValueError("catalog is not an object with tools")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    parser.parse_args()
    try:
        pinned = pinned_sha256(PROJECT)
        if witness_pin_sha256(PROJECT) != pinned:
            raise ValueError("the witness pin fixture's catalogSha256 differs from the host-witness pin")
        validate(PROJECT, pinned)
    except (OSError, ValueError) as error:
        print(f"Catalog resource check failed: {error}")
        return 1
    print("Catalog resource is contained, bounded, strict JSON and matches its pin.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
