"""Member classification shared by the public-export pipeline and the
DEC-058 release-candidate scan runner.

DEC-058 check (b) lints every public text surface. Rather than a
hand-maintained suffix list, a member is linted when its bytes decode as
UTF-8, whatever its extension or its case. A member that does not decode
cannot be linted, and its format is never parsed: it ships only when its
exact path, size and SHA-256 are on the committed list of reviewed binaries
(`reviewed_binaries.json`), whatever its suffix. Every other binary is
blocked.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


def suffix_of(path: Path) -> str:
    """The lower-case suffix, so `.PEM` and `.pem` classify alike."""
    return path.suffix.lower()


def is_text_member(path: Path) -> bool:
    """True when the file's bytes decode as UTF-8 (lint and sanitize it)."""
    try:
        path.read_bytes().decode("utf-8")
    except (OSError, UnicodeDecodeError):
        return False
    return True


REVIEWED_BINARIES_NAME = "reviewed_binaries.json"
# The list's location inside the project tree (and so inside the export).
REVIEWED_BINARIES_PATH = "tools/release/" + REVIEWED_BINARIES_NAME


def load_reviewed_binaries(path: Path) -> dict[str, dict]:
    """The committed reviewed-binary list, keyed by project-relative path.
    A missing list is an empty one: every binary is then blocked."""
    if not path.is_file():
        return {}
    document = json.loads(path.read_text(encoding="utf-8"))
    entries: dict[str, dict] = {}
    for entry in document["binaries"]:
        if not all(isinstance(entry.get(key), str) and entry[key] for key in ("path", "sha256", "reason")) \
                or not isinstance(entry.get("size_bytes"), int):
            raise ValueError("reviewed binary entries need path, sha256, size_bytes and reason")
        entries[entry["path"]] = entry
    return entries


def binary_admission(relative_path: str, data: bytes, reviewed: dict[str, dict]) -> str | None:
    """None when this exact binary is on the reviewed list, else the finding."""
    entry = reviewed.get(relative_path)
    if entry is None:
        return "unreviewed_binary"
    if entry["size_bytes"] != len(data) or entry["sha256"] != hashlib.sha256(data).hexdigest():
        return "reviewed_binary_hash_mismatch"
    return None
