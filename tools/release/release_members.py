"""Member classification shared by the public-export pipeline and the
DEC-058 release-candidate scan runner.

DEC-058 check (b) lints every public text surface. Rather than a
hand-maintained suffix list, a member is linted when its bytes decode as
UTF-8, whatever its extension or its case. Binary members cannot be linted;
the export admits them only from its declared binary suffixes, and both
tools inventory and hash them.
"""
from __future__ import annotations

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
