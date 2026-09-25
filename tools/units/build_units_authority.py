#!/usr/bin/env python3
"""Explicit development/test build of the existing-unit-authority CLI bridge."""
from __future__ import annotations
import argparse
from pathlib import Path
import subprocess

PROJECT = Path(__file__).resolve().parents[2]
CRATE = PROJECT / "core" / "units"
DEFAULT_TARGET = CRATE / "target" / "units-authority"

def build(target_dir: Path = DEFAULT_TARGET) -> Path:
    target = target_dir.resolve()
    subprocess.run(["cargo", "build", "--locked", "--release", "--features", "cli",
                    "--bin", "openpipestress_units", "--target-dir", str(target)], cwd=CRATE, check=True)
    return target / "release" / "openpipestress_units"

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-dir", type=Path, default=DEFAULT_TARGET)
    print(build(parser.parse_args().target_dir))

if __name__ == "__main__":
    main()
