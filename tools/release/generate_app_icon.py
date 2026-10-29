#!/usr/bin/env python3
"""Deterministic SWBPIPE application-icon generator (DEC-057 / E5; RT-V8 P1).

Derives the desktop icons from the SWB mark in
``apps/desktop/SWBPIPE_logo.png`` (owner decision RT-V8 P1, 2026-10-28: the
app icon is the SWB mark the project website uses), then writes:

- ``apps/desktop/src-tauri/icons/icon.png``   (512x512 RGBA)
- ``apps/desktop/src-tauri/icons/icon.icns``  (multi-resolution macOS icon)

These are the two files ``tauri.conf.json`` lists under ``bundle.icon`` for
the macOS-only ``app`` target. The logo is read only in its canonical
encoding (IHDR/IDAT/IEND, 8-bit RGBA, filter 0 on every row; RT-V8 P9), and
anything else is refused: this is not a general PNG decoder. The 1254px mark
is resampled to the 1024px base by exact area averaging on premultiplied
alpha in integer arithmetic, then box-downscaled by integer factors, and the
``.icns`` is assembled in pure Python as PNG-typed members (icp4..ic14, 16px
through 1024px, including the @2x pairs). No host icon tooling is needed and
the committed bytes are reproducible from this script and the logo.

This script produces desktop icon assets only. It is not a release act,
publication, signing, notarization, or release-readiness claim.
"""

from __future__ import annotations

import argparse
import struct
import sys
import zlib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ICON_DIR = ROOT / "apps" / "desktop" / "src-tauri" / "icons"
LOGO = ROOT / "apps" / "desktop" / "SWBPIPE_logo.png"

BASE_SIZE = 1024
PNG_SIGNATURE = b"\x89PNG\r\n\x1a\n"

# icns member types that carry raw PNG data, per size.
ICNS_PNG_TYPES: tuple[tuple[bytes, int], ...] = (
    (b"icp4", 16),
    (b"icp5", 32),
    (b"icp6", 64),
    (b"ic07", 128),
    (b"ic08", 256),
    (b"ic09", 512),
    (b"ic10", 1024),  # 512@2x
    (b"ic11", 32),  # 16@2x
    (b"ic12", 64),  # 32@2x
    (b"ic13", 256),  # 128@2x
    (b"ic14", 512),  # 256@2x
)


def read_logo(path: Path = LOGO) -> tuple[int, bytes]:
    """The logo's side and straight-alpha RGBA bytes, from its canonical
    encoding only (IHDR, IDAT..., IEND; square 8-bit RGBA; filter 0)."""
    data = path.read_bytes()
    if not data.startswith(PNG_SIGNATURE):
        raise ValueError("logo is not a PNG")
    offset, header, idat = len(PNG_SIGNATURE), None, []
    while offset < len(data):
        length, kind = struct.unpack(">I4s", data[offset : offset + 8])
        body = data[offset + 8 : offset + 8 + length]
        if kind == b"IHDR":
            header = struct.unpack(">IIBBBBB", body)
        elif kind == b"IDAT":
            idat.append(body)
        elif kind != b"IEND":
            raise ValueError("logo must be the canonical IHDR/IDAT/IEND encoding")
        offset += 12 + length
        if kind == b"IEND":
            break
    if header is None or header[0] != header[1] or header[2:] != (8, 6, 0, 0, 0):
        raise ValueError("logo must be square, 8-bit RGBA and not interlaced")
    side, stride = header[0], header[0] * 4
    raw = zlib.decompress(b"".join(idat))
    if len(raw) != side * (stride + 1) or any(raw[y * (stride + 1)] for y in range(side)):
        raise ValueError("logo rows must all use filter 0")
    pixels = b"".join(raw[y * (stride + 1) + 1 : (y + 1) * (stride + 1)] for y in range(side))
    return side, pixels


def _area_taps(source: int, target: int) -> list[list[tuple[int, int]]]:
    """Per output index, (source index, overlap) pairs in integer units where
    a source pixel spans `target` and an output pixel spans `source`."""
    taps = []
    for x in range(target):
        lo, hi = x * source, (x + 1) * source
        taps.append([
            (s, min(hi, (s + 1) * target) - max(lo, s * target))
            for s in range(lo // target, (hi - 1) // target + 1)
        ])
    return taps


def _round_div(numerator: int, denominator: int) -> int:
    return (2 * numerator + denominator) // (2 * denominator)


def render_base(logo: Path = LOGO) -> list[list[tuple[int, int, int, int]]]:
    """The 1024x1024 RGBA base (straight alpha), area-resampled from the logo."""
    side, pixels = read_logo(logo)
    taps = _area_taps(side, BASE_SIZE)
    stride = side * 4
    # Horizontal pass over premultiplied channels (integer sums).
    rows = []
    for y in range(side):
        line = pixels[y * stride : (y + 1) * stride]
        out = []
        for column in taps:
            r = g = b = a = 0
            for s, weight in column:
                alpha = line[4 * s + 3] * weight
                r += line[4 * s] * alpha
                g += line[4 * s + 1] * alpha
                b += line[4 * s + 2] * alpha
                a += alpha
            out.append((r, g, b, a))
        rows.append(out)
    # Vertical pass, then back to straight alpha.
    total = side * side
    base = []
    for column in taps:
        out_row = []
        for x in range(BASE_SIZE):
            r = g = b = a = 0
            for s, weight in column:
                pr, pg, pb, pa = rows[s][x]
                r += pr * weight
                g += pg * weight
                b += pb * weight
                a += pa * weight
            if a == 0:
                out_row.append((0, 0, 0, 0))
                continue
            out_row.append((_round_div(r, a), _round_div(g, a), _round_div(b, a), _round_div(a, total)))
        base.append(out_row)
    return base


def downscale(
    pixels: list[list[tuple[int, int, int, int]]], size: int
) -> list[list[tuple[int, int, int, int]]]:
    """Integer-factor box downscale on premultiplied alpha (deterministic)."""
    factor = BASE_SIZE // size
    if factor * size != BASE_SIZE:
        raise ValueError(f"size {size} is not an integer factor of {BASE_SIZE}")
    if factor == 1:
        return pixels
    area = factor * factor
    out: list[list[tuple[int, int, int, int]]] = []
    for by in range(size):
        row: list[tuple[int, int, int, int]] = []
        for bx in range(size):
            acc_r = acc_g = acc_b = acc_a = 0
            for sy in range(by * factor, (by + 1) * factor):
                src = pixels[sy]
                for sx in range(bx * factor, (bx + 1) * factor):
                    r, g, b, a = src[sx]
                    acc_r += r * a
                    acc_g += g * a
                    acc_b += b * a
                    acc_a += a
            if acc_a == 0:
                row.append((0, 0, 0, 0))
                continue
            row.append(
                (
                    round(acc_r / acc_a),
                    round(acc_g / acc_a),
                    round(acc_b / acc_a),
                    round(acc_a / area),
                )
            )
        out.append(row)
    return out


def encode_png(pixels: list[list[tuple[int, int, int, int]]]) -> bytes:
    """Encode RGBA rows as a minimal PNG (IHDR/IDAT/IEND only, filter 0)."""
    height = len(pixels)
    width = len(pixels[0])
    raw = bytearray()
    for row in pixels:
        raw.append(0)  # filter type 0
        for r, g, b, a in row:
            raw.extend((r, g, b, a))

    def chunk(kind: bytes, payload: bytes) -> bytes:
        body = kind + payload
        return struct.pack(">I", len(payload)) + body + struct.pack(
            ">I", zlib.crc32(body) & 0xFFFFFFFF
        )

    ihdr = struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0)
    idat = zlib.compress(bytes(raw), 9)
    return (
        b"\x89PNG\r\n\x1a\n"
        + chunk(b"IHDR", ihdr)
        + chunk(b"IDAT", idat)
        + chunk(b"IEND", b"")
    )


def encode_icns(pngs_by_size: dict[int, bytes]) -> bytes:
    """Assemble a PNG-member .icns from the rendered sizes."""
    members = bytearray()
    for kind, size in ICNS_PNG_TYPES:
        payload = pngs_by_size[size]
        members += kind + struct.pack(">I", len(payload) + 8) + payload
    return b"icns" + struct.pack(">I", len(members) + 8) + bytes(members)


def generate(icon_dir: Path) -> list[Path]:
    base = render_base()
    sizes = sorted({size for _, size in ICNS_PNG_TYPES})
    pngs_by_size = {size: encode_png(downscale(base, size)) for size in sizes}
    icon_dir.mkdir(parents=True, exist_ok=True)
    png_path = icon_dir / "icon.png"
    icns_path = icon_dir / "icon.icns"
    png_path.write_bytes(pngs_by_size[512])
    icns_path.write_bytes(encode_icns(pngs_by_size))
    return [png_path, icns_path]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description=(
            "Derive the desktop icon assets (icon.png 512px + "
            "multi-resolution icon.icns) from the SWB mark logo."
        )
    )
    parser.add_argument(
        "--icon-dir",
        default=str(ICON_DIR),
        help=f"Output icon directory (default: {ICON_DIR}).",
    )
    args = parser.parse_args(sys.argv[1:] if argv is None else argv)
    for path in generate(Path(args.icon_dir).resolve()):
        print(f"[generate-app-icon] wrote {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
