"""Member classification shared by the public-export pipeline and the
DEC-058 release-candidate scan runner.

DEC-058 check (b) lints every public text surface. Rather than a
hand-maintained suffix list, a member is linted when its bytes decode as
UTF-8, whatever its extension or its case. Binary members cannot be linted;
the export admits them only from its declared binary suffixes, and both
tools inventory and hash them.
"""
from __future__ import annotations

import struct
import zlib
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


# ---------------------------------------------------------------------------
# Binary formats. A declared binary suffix admits a file only when its content
# parses as that format; textual metadata inside it is reported, never passed.
# ---------------------------------------------------------------------------

PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"
# Chunk types that carry pixels or colour data only. Every other chunk
# (tEXt, zTXt, iTXt, eXIf, iCCP, sPLT, caBX content credentials, private
# chunks) is metadata that can carry text and is reported.
PNG_PLAIN_CHUNKS = {
    b"IHDR", b"PLTE", b"IDAT", b"IEND", b"tRNS", b"gAMA", b"cHRM",
    b"sRGB", b"pHYs", b"bKGD", b"sBIT", b"hIST", b"tIME",
}
# icns elements that hold text rather than image data.
ICNS_TEXT_ELEMENTS = {b"info", b"name"}


def inspect_png(data: bytes) -> dict:
    """{"well_formed": bool, "metadata": [chunk types beyond plain image]}:
    signature, chunk bounds and CRCs, IHDR first, an IDAT, IEND last and no
    trailing bytes."""
    result = {"well_formed": False, "metadata": []}
    if not data.startswith(PNG_SIGNATURE):
        return result
    offset, types = len(PNG_SIGNATURE), []
    while offset < len(data):
        if offset + 12 > len(data):
            return result
        length = struct.unpack(">I", data[offset : offset + 4])[0]
        kind = data[offset + 4 : offset + 8]
        end = offset + 12 + length
        if not (kind.isascii() and kind.isalpha()) or end > len(data):
            return result
        body = data[offset + 8 : offset + 8 + length]
        crc = struct.unpack(">I", data[end - 4 : end])[0]
        if zlib.crc32(kind + body) & 0xFFFFFFFF != crc:
            return result
        types.append(kind)
        if kind not in PNG_PLAIN_CHUNKS:
            result["metadata"].append(kind.decode("ascii"))
        offset = end
        if kind == b"IEND":
            break
    if (
        offset != len(data)
        or not types
        or types[0] != b"IHDR"
        or types[-1] != b"IEND"
        or b"IDAT" not in types
        or struct.unpack(">I", data[8:12])[0] != 13
    ):
        return result
    result["well_formed"] = True
    return result


def inspect_icns(data: bytes) -> dict:
    """{"well_formed": bool, "metadata": [...]}: the `icns` header and total
    length, an element table that exactly fills the file, a TOC (if present)
    that matches it, and every embedded PNG inspected in turn."""
    result = {"well_formed": False, "metadata": []}
    if len(data) < 8 or data[:4] != b"icns" or struct.unpack(">I", data[4:8])[0] != len(data):
        return result
    offset, elements = 8, []
    while offset < len(data):
        if offset + 8 > len(data):
            return result
        kind = data[offset : offset + 4]
        length = struct.unpack(">I", data[offset + 4 : offset + 8])[0]
        if length < 8 or offset + length > len(data):
            return result
        elements.append((kind, length, data[offset + 8 : offset + length]))
        offset += length
    toc = [body for kind, _, body in elements if kind == b"TOC "]
    if toc:
        listed = [
            (toc[0][i : i + 4], struct.unpack(">I", toc[0][i + 4 : i + 8])[0])
            for i in range(0, len(toc[0]) - 7, 8)
        ]
        if len(toc[0]) % 8 or listed != [(k, n) for k, n, _ in elements if k != b"TOC "]:
            return result
    for kind, _, body in elements:
        label = kind.decode("latin-1")
        if kind in ICNS_TEXT_ELEMENTS:
            result["metadata"].append(label)
        if body.startswith(PNG_SIGNATURE):
            embedded = inspect_png(body)
            if not embedded["well_formed"]:
                return result
            result["metadata"].extend(f"{label}/{chunk}" for chunk in embedded["metadata"])
    result["well_formed"] = True
    return result


# Every binary suffix a caller may allow needs an inspector here.
BINARY_INSPECTORS = {".png": inspect_png, ".icns": inspect_icns}


def inspect_binary(path: Path) -> dict | None:
    """The inspection for a file whose suffix has an inspector, else None."""
    inspector = BINARY_INSPECTORS.get(suffix_of(path))
    if inspector is None:
        return None
    try:
        return inspector(path.read_bytes())
    except OSError:
        return {"well_formed": False, "metadata": []}
