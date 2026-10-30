#!/usr/bin/env python3
"""Explicit development/test build of `openpipestress-runner`, whose `read`
subcommand gives Python product code the Rust product reader's verdict."""
from __future__ import annotations
import argparse
from pathlib import Path
import subprocess

PROJECT = Path(__file__).resolve().parents[2]
CRATE = PROJECT / "core" / "runner" / "headless"
DEFAULT_TARGET = CRATE / "target" / "reader-runner"

def build(target_dir: Path = DEFAULT_TARGET) -> Path:
    target = target_dir.resolve()
    subprocess.run(["cargo", "build", "--locked", "--release", "--bin", "openpipestress-runner",
                    "--target-dir", str(target)], cwd=CRATE, check=True)
    return target / "release" / "openpipestress-runner"

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-dir", type=Path, default=DEFAULT_TARGET)
    print(build(parser.parse_args().target_dir))

if __name__ == "__main__":
    main()
