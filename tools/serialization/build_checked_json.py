#!/usr/bin/env python3
"""Explicitly build the private checked-JSON adapter for development/tests."""
from __future__ import annotations
import argparse
from pathlib import Path
import subprocess

PROJECT = Path(__file__).resolve().parents[2]
CRATE = PROJECT / "core" / "serialization" / "canonical_json"
DEFAULT_TARGET = CRATE / "target" / "checked-json"


BINARIES = {
    "openpipestress_jcs_ijson_v1": "openpipestress_jcs_ijson",
    "openpipestress_jcs_binary64_v1": "openpipestress_jcs_binary64",
}


def build(target_dir: Path = DEFAULT_TARGET, *, profile: str = "openpipestress_jcs_ijson_v1") -> Path:
    binary = BINARIES[profile]
    executable = target_dir.resolve() / "release" / binary
    subprocess.run([
        "cargo", "build", "--locked", "--release", "--features", "checked-cli",
        "--bin", binary, "--target-dir", str(target_dir.resolve()),
    ], cwd=CRATE, check=True)
    return executable

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--target-dir", type=Path, default=DEFAULT_TARGET)
    parser.add_argument("--profile", choices=BINARIES, default="openpipestress_jcs_ijson_v1")
    args = parser.parse_args()
    print(build(args.target_dir, profile=args.profile))


if __name__ == "__main__":
    main()
