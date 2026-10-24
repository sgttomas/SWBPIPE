#!/usr/bin/env python3
"""Verdict parity of the two handoff package verifiers (W-6b review round 2,
should-fix 5).

`fixtures/handoff/verifier_parity_cases.json` holds a maintained differential
over the invented witness package (`fixtures/handoff/invented_witness.swbhandoff`):

- every schema-valid replacement of one manifest leaf (or list), each resealed
  as a forger would (export hash and checksum inventory recomputed) and left
  unsealed;
- coordinated, self-consistent rewrites of one subject (model, basis state,
  library, rule pack, their digests, the units block);
- a substituted model member with its hashes repaired;
- an empty basis-state id;
- 500 models carrying one number each (edge values, then random binary64
  values from a fixed seed), with every hash and identity repaired.

Each case is a list of edits, so both twins rebuild the same package from it:
this test with the Python verifier (`tests/verifiers/handoff_package.py`), and
the Rust test `verifier_parity_cases_match_the_python_twin`
(`apps/desktop/src-tauri/src/handoff_package.rs`) with `verify_handoff_package`.
The fixture pins the first 16 hex digits of the SHA-256 of the built bytes
(`<model member>\n<manifest member>`), so both twins judge the same bytes, and
both must give every case the verdict the fixture records. The cases are
regenerated here and must equal the fixture's; set
`SWBPIPE_WRITE_HANDOFF_PARITY_CASES=1` to rewrite it (and re-run the Rust test).

Edit forms, applied in order to the package instance (`"model"` edits apply to
the model member first):

- `[pointer, "set", value]`;
- `[pointer, "append_x"]` (a string with `x` appended);
- `[pointer, "negate"]` (a boolean);
- `[pointer, "double"]` (a list followed by itself);
- `[pointer, "model_hex"]` (the model member's SHA-256 hex);
- `[pointer, "units_hash"]` (the SHA-256 hex of the JCS text of
  `/handoff_package_manifest/units_manifest` without `hash_refs`);
- `[pointer, "handoff_id"]` / `[pointer, "manifest_id"]` (`handoff:` /
  `manifest:` and the package token of the model member's project id and hash).

A case is `[name, edits, model_edits, seal, bytes, accepted]`. Nothing here is an
acceptance; the witness and its mutations are invented.
"""

from __future__ import annotations

import copy
from hashlib import sha256
import json
import math
import os
from pathlib import Path
import random
import struct
import sys
from typing import Any

from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tests.verifiers.canonical_json import canonical_json_unchecked as canonical_json  # noqa: E402
from tests.verifiers.handoff_package import (  # noqa: E402
    HandoffVerificationError,
    checksum_inventory,
    export_hash,
    package_token,
    verify_handoff_package,
)

WITNESS = ROOT / "fixtures" / "handoff" / "invented_witness.swbhandoff"
CASES = ROOT / "fixtures" / "handoff" / "verifier_parity_cases.json"
HANDOFF_SCHEMA = ROOT / "schemas" / "handoff_package.schema.json"
STRING_REPLACEMENTS = ["TBD", "unrelated_model", "private_only", "public_permissive", "quarantined", "0" * 64]


def _token(key: Any) -> str:
    return str(key).replace("~", "~0").replace("/", "~1")


def _pointer(path: tuple) -> str:
    return "".join(f"/{_token(key)}" for key in path)


def _resolve(document: Any, pointer: str) -> tuple[Any, Any]:
    tokens = [token.replace("~1", "/").replace("~0", "~") for token in pointer.split("/")[1:]]
    parent = document
    for token in tokens[:-1]:
        parent = parent[int(token)] if isinstance(parent, list) else parent[token]
    last = tokens[-1]
    return parent, int(last) if isinstance(parent, list) else last


def apply_edits(document: Any, edits: list, model_hex: str = "", project_id: str = "") -> None:
    for edit in edits:
        pointer, op = edit[0], edit[1]
        parent, key = _resolve(document, pointer)
        if op == "set":
            parent[key] = copy.deepcopy(edit[2])
        elif op == "append_x":
            parent[key] = parent[key] + "x"
        elif op == "negate":
            parent[key] = not parent[key]
        elif op == "double":
            parent[key] = parent[key] + copy.deepcopy(parent[key])
        elif op == "model_hex":
            parent[key] = model_hex
        elif op == "handoff_id":
            parent[key] = f"handoff:{package_token(project_id, model_hex)}"
        elif op == "manifest_id":
            parent[key] = f"manifest:{package_token(project_id, model_hex)}"
        elif op == "units_hash":
            units = document["handoff_package_manifest"]["units_manifest"]
            block = {name: value for name, value in units.items() if name != "hash_refs"}
            parent[key] = sha256(canonical_json(block).encode("utf-8")).hexdigest()
        else:
            raise ValueError(f"unknown edit {edit!r}")


def build_bytes(case: list) -> tuple[str, str]:
    """The case's model and manifest member texts."""
    _, edits, model_edits, seal = case[:4]
    package = json.loads((WITNESS / "handoff_package.json").read_text(encoding="utf-8"))
    model = json.loads((WITNESS / "model" / "model.json").read_text(encoding="utf-8"))
    apply_edits(model, model_edits)
    model_text = canonical_json(model)
    model_hex = sha256(model_text.encode("utf-8")).hexdigest()
    apply_edits(package, edits, model_hex, model["project"]["id"])
    manifest = package["handoff_package_manifest"]
    if seal:
        manifest["checksums"] = checksum_inventory(
            manifest["model_hash"].get("payload_ref"), manifest["package_identity"]["handoff_package_id"], model_hex, export_hash(manifest)
        )
    return model_text, canonical_json(package)


def bytes_digest(model_text: str, manifest_text: str) -> str:
    return sha256(f"{model_text}\n{manifest_text}".encode("utf-8")).hexdigest()[:16]


def build(case: list, directory: Path) -> tuple[Path, str]:
    """The case's package, written under `directory`, and its bytes digest."""
    model_text, manifest_text = build_bytes(case)
    target = directory / f"{case[0]}.swbhandoff"
    (target / "model").mkdir(parents=True)
    (target / "model" / "model.json").write_text(model_text, encoding="utf-8")
    (target / "handoff_package.json").write_text(manifest_text, encoding="utf-8")
    return target, bytes_digest(model_text, manifest_text)


def _leaves(value: Any, path: tuple = ()):
    if isinstance(value, dict):
        for key, child in value.items():
            yield from _leaves(child, path + (key,))
    elif isinstance(value, list):
        yield path, value
        for index, child in enumerate(value):
            yield from _leaves(child, path + (index,))
    else:
        yield path, value


def _replacements(value: Any) -> list[list]:
    if isinstance(value, str):
        return [["append_x"]] + [["set", text] for text in STRING_REPLACEMENTS]
    if isinstance(value, bool):
        return [["negate"]]
    if isinstance(value, list):
        return [["set", []], ["double"]]
    return [["set", 0], ["set", 1]]


def generate_cases() -> list[list]:
    """The differential's cases as `[name, edits, model_edits, seal]`."""
    package = json.loads((WITNESS / "handoff_package.json").read_text(encoding="utf-8"))
    schema = Draft202012Validator(json.loads(HANDOFF_SCHEMA.read_text(encoding="utf-8")))
    cases: list[list] = [["original", [], [], True]]
    serial = 0
    for path, value in _leaves(package):
        if path[-1:] == ("checksums",):
            continue
        for replacement in _replacements(value):
            edits = [[_pointer(path), *replacement]]
            mutated = copy.deepcopy(package)
            apply_edits(mutated, edits)
            if not schema.is_valid(mutated) or not isinstance(mutated["handoff_package_manifest"]["model_hash"], dict):
                continue
            for seal in (True, False):
                serial += 1
                cases.append([f"leaf-{serial:04d}", edits, [], seal])
    manifest = "/handoff_package_manifest"
    model_hash_refs = [f"{manifest}/model_hash", f"{manifest}/model_basis/hash_refs/0", f"{manifest}/model_state_refs/0/hash_refs/0"]
    state_refs = [f"{manifest}/model_basis/basis_state_ref", f"{manifest}/model_state_refs/0/model_state_ref", f"{manifest}/model_state_refs/0/hash_refs/1/payload_ref"]
    coordinated = {
        "model": [[f"{ref}/payload_ref/ref", "set", "project:other"] for ref in model_hash_refs] + [[f"{manifest}/model_basis/model_ref/ref", "set", "project:other"]],
        "state": [[f"{ref}/ref", "set", "state:unrelated"] for ref in state_refs] + [[f"{manifest}/entity_ids/model_state_ids", "set", ["state:unrelated"]]],
        "library": [[f"{manifest}/library_refs/0/library_ref/ref", "set", "library:unrelated"], [f"{manifest}/library_refs/0/checksum/payload_ref/ref", "set", "library:unrelated"]],
        "rule-pack": [[f"{manifest}/rule_pack_refs/0/rule_pack_id", "set", "pack:unrelated"], [f"{manifest}/rule_pack_refs/0/checksum/payload_ref/ref", "set", "pack:unrelated"]],
        "state-digest": [[f"{manifest}/model_state_refs/0/hash_refs/1/value", "set", "0" * 64]],
        "library-digest": [[f"{manifest}/library_refs/0/checksum/value", "set", "0" * 64]],
        "rule-pack-digest": [[f"{manifest}/rule_pack_refs/0/checksum/value", "set", "0" * 64]],
        "units": [[f"{manifest}/units_manifest/coordinate_unit", "set", "unit:foot"], [f"{manifest}/units_manifest/hash_refs/0/value", "units_hash"]],
        "empty-state-id": [[f"{ref}/ref", "set", ""] for ref in state_refs] + [[f"{manifest}/entity_ids/model_state_ids", "set", [""]]],
    }
    for tag, edits in coordinated.items():
        cases.append([f"coordinated-{tag}", edits, [], True])
    cases.append([
        "substituted-model",
        [[f"{ref}/value", "model_hex"] for ref in model_hash_refs],
        [["/project/id", "set", "project:other"], ["/nodes/0/id", "set", "node:other"]],
        True,
    ])
    # Numbers in the model member, every hash and identity repaired: the two
    # canonicalizers must write the same text (edge values, then binary64).
    numbers: list[Any] = [-0.0, 5e-324, 1e-7, 1e20, 1e21, -(2**63), 2**64 - 1, 2**64, -(2**63) - 1, 9007199254740993]
    generator = random.Random(1600)
    while len(numbers) < 500:
        value = struct.unpack(">d", generator.randbytes(8))[0]
        if math.isfinite(value):
            numbers.append(value)
    repaired = [[f"{ref}/value", "model_hex"] for ref in model_hash_refs] + [
        [f"{manifest}/package_identity/handoff_package_id", "handoff_id"],
        [f"{manifest}/package_identity/manifest_id", "manifest_id"],
    ]
    for index, value in enumerate(numbers):
        cases.append([f"number-{index:04d}", repaired, [["/review_number", "set", {chr(0x1F600): value, chr(0xE000): "invented"}]], True])
    return cases


def python_verdict(case: list, directory: Path) -> tuple[str, bool]:
    """The built bytes' digest and the Python verifier's verdict."""
    target, digest = build(case, directory)
    try:
        verify_handoff_package(target)
        return digest, True
    except HandoffVerificationError:
        return digest, False


def _write_fixture(cases: list[list], directory: Path) -> None:
    rows = [[*case[:4], *python_verdict(case, directory)] for case in cases]
    body = ",\n".join(json.dumps(row, separators=(",", ":"), ensure_ascii=False) for row in rows)
    CASES.write_text(
        '{"comment":"W-6b verifier parity cases; see tests/test_handoff_verifier_parity.py. Invented; not an acceptance.",\n"cases":[\n'
        + body
        + "\n]}\n",
        encoding="utf-8",
    )


def test_the_parity_cases_are_the_generators_and_the_python_verdicts_are_the_fixtures(tmp_path):
    generated = generate_cases()
    if os.environ.get("SWBPIPE_WRITE_HANDOFF_PARITY_CASES") == "1":
        _write_fixture(generated, tmp_path / "write")
    pinned = json.loads(CASES.read_text(encoding="utf-8"))["cases"]
    assert [case[:4] for case in pinned] == [case[:4] for case in generated], "the fixture's cases are not the generator's; regenerate"
    assert len(pinned) > 2900
    names = {case[0]: case for case in pinned}
    assert names["original"][5] is True and names["number-0000"][5] is True
    for refused in ("coordinated-model", "coordinated-empty-state-id", "substituted-model"):
        assert names[refused][5] is False, refused
    mismatched = [case[0] for case in pinned if python_verdict(case, tmp_path) != (case[4], case[5])]
    assert mismatched == [], f"the Python verifier disagrees with the pinned verdicts: {mismatched[:20]}"
