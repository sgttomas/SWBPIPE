#!/usr/bin/env python3
"""Explicitly build the private checked-JSON adapter for development/tests."""
from __future__ import annotations
import argparse
from pathlib import Path
import subprocess

PROJECT = Path(__file__).resolve().parents[2]
CRATE = PROJECT / "core" / "serialization" / "canonical_json"
DEFAULT_TARGET = CRATE / "target" / "checked-json"


def build(target_dir: Path = DEFAULT_TARGET) -> Path:
    executable = target_dir.resolve() / "release" / "openpipestress_jcs_ijson"
    subprocess.run([
        "cargo", "build", "--locked", "--release", "--features", "checked-cli",
        "--bin", "openpipestress_jcs_ijson", "--target-dir", str(target_dir.resolve()),
    ], cwd=CRATE, check=True)
    return executable

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-dir", type=Path, default=DEFAULT_TARGET)
    args = parser.parse_args()
    print(build(args.target_dir))


if __name__ == "__main__":
    main()
