#!/usr/bin/env python3
"""Deterministic OpenPipeStress application-icon generator (DEC-057 / E5).

Renders the invented OpenPipeStress mark — a pipe cross-section (annulus)
with an amber stressed sector on a dark steel rounded square — procedurally
from this file alone, then writes:

- ``apps/desktop/src-tauri/icons/icon.png``   (512x512 RGBA source mark)
- ``apps/desktop/src-tauri/icons/icon.icns``  (multi-resolution macOS icon)

The ``.icns`` is assembled in pure Python as PNG-typed members (icp4..ic14,
16px through 1024px, including the @2x pairs), so no host icon tooling is
required and the committed bytes are reproducible from this script alone
(modulo the host zlib's compressed-stream bytes: the decoded pixels are
exact; "deterministic where feasible" per the DEC-057 packaging posture).

The mark is original invented content created for this project. It contains
no third-party art, fonts, logos, or trademarked material.

This script produces desktop icon assets only. It is not a release act,
publication, signing, notarization, or release-readiness claim.
"""

from __future__ import annotations

import argparse
import math
import struct
import sys
import zlib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
ICON_DIR = ROOT / "apps" / "desktop" / "src-tauri" / "icons"

BASE_SIZE = 1024

# Palette (invented; flat mark, no third-party art).
BG_TOP = (37, 62, 92)  # steel blue, top of gradient
BG_BOTTOM = (17, 31, 49)  # steel blue, bottom of gradient
PIPE = (240, 244, 247)  # pipe-wall annulus
STRESS = (232, 138, 52)  # amber stressed sector

# Geometry at the 1024 base canvas (scaled linearly for other sizes).
SQUARE_INSET = 100.0
SQUARE_RADIUS = 185.0
ANNULUS_OUTER = 300.0
ANNULUS_INNER = 168.0
# Stressed sector angular span, radians, y-up, 0 at +x (upper-right sector).
SECTOR_START = math.radians(-15.0)
SECTOR_END = math.radians(105.0)

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


def _coverage(distance: float) -> float:
    """Linear one-pixel antialias coverage from a signed distance (px)."""
    if distance <= -0.5:
        return 1.0
    if distance >= 0.5:
        return 0.0
    return 0.5 - distance


def _rounded_square_distance(x: float, y: float) -> float:
    """Signed distance to the background rounded square (negative inside)."""
    half = (BASE_SIZE - 2.0 * SQUARE_INSET) / 2.0
    cx = abs(x - BASE_SIZE / 2.0) - (half - SQUARE_RADIUS)
    cy = abs(y - BASE_SIZE / 2.0) - (half - SQUARE_RADIUS)
    ox = max(cx, 0.0)
    oy = max(cy, 0.0)
    outside = math.hypot(ox, oy)
    inside = min(max(cx, cy), 0.0)
    return outside + inside - SQUARE_RADIUS


def _annulus_distance(x: float, y: float) -> float:
    """Signed distance to the pipe-wall annulus (negative inside the wall)."""
    r = math.hypot(x - BASE_SIZE / 2.0, y - BASE_SIZE / 2.0)
    mid = (ANNULUS_OUTER + ANNULUS_INNER) / 2.0
    halfwidth = (ANNULUS_OUTER - ANNULUS_INNER) / 2.0
    return abs(r - mid) - halfwidth


def _sector_distance(x: float, y: float) -> float:
    """Signed distance to the stressed annulus sector (negative inside)."""
    dx = x - BASE_SIZE / 2.0
    dy = (BASE_SIZE / 2.0) - y  # y-up
    r = math.hypot(dx, dy)
    theta = math.atan2(dy, dx)
    if theta < SECTOR_START:
        angular_inside = (theta - SECTOR_START) * max(r, 1.0)
    elif theta > SECTOR_END:
        angular_inside = (SECTOR_END - theta) * max(r, 1.0)
    else:
        angular_inside = min(theta - SECTOR_START, SECTOR_END - theta) * max(r, 1.0)
    return max(_annulus_distance(x, y), -angular_inside)


def render_base() -> list[list[tuple[int, int, int, int]]]:
    """Render the 1024x1024 RGBA mark, straight (non-premultiplied) alpha."""
    rows: list[list[tuple[int, int, int, int]]] = []
    span = BASE_SIZE - 2.0 * SQUARE_INSET
    for py in range(BASE_SIZE):
        y = py + 0.5
        t = min(max((y - SQUARE_INSET) / span, 0.0), 1.0)
        bg = tuple(
            round(BG_TOP[i] + (BG_BOTTOM[i] - BG_TOP[i]) * t) for i in range(3)
        )
        row: list[tuple[int, int, int, int]] = []
        for px in range(BASE_SIZE):
            x = px + 0.5
            a_bg = _coverage(_rounded_square_distance(x, y))
            if a_bg == 0.0:
                row.append((0, 0, 0, 0))
                continue
            # Composite pipe wall, then stressed sector, over the gradient.
            r, g, b = float(bg[0]), float(bg[1]), float(bg[2])
            a_pipe = _coverage(_annulus_distance(x, y))
            if a_pipe > 0.0:
                r += (PIPE[0] - r) * a_pipe
                g += (PIPE[1] - g) * a_pipe
                b += (PIPE[2] - b) * a_pipe
            a_stress = _coverage(_sector_distance(x, y))
            if a_stress > 0.0:
                r += (STRESS[0] - r) * a_stress
                g += (STRESS[1] - g) * a_stress
                b += (STRESS[2] - b) * a_stress
            row.append(
                (
                    min(255, max(0, round(r))),
                    min(255, max(0, round(g))),
                    min(255, max(0, round(b))),
                    min(255, max(0, round(a_bg * 255.0))),
                )
            )
        rows.append(row)
    return rows


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
            "Render the invented OpenPipeStress mark and write the desktop "
            "icon assets (icon.png 512px + multi-resolution icon.icns)."
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
