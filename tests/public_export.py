"""Compatibility detector for historical reduced-source exports.

Those trees omitted loop/execution and Root tools. The active standalone SWBPIPE
repository carries its development localities and local verification tools, so
these legacy skip markers must remain false there. No current publication
pipeline trims development or CI files from this repository.
"""
from pathlib import Path

import pytest

PROJECT = Path(__file__).resolve().parents[1]
IN_MONOREPO = not (PROJECT / ".git").exists() and PROJECT.parent.name == "projects" and (
    (PROJECT.parents[1] / ".git").exists() or (PROJECT.parents[1] / "AGENTS.md").is_file())
PUBLIC_EXPORT = not IN_MONOREPO and not (PROJECT / "loop").is_dir() and not (PROJECT / "execution").is_dir()


def monorepo_only(surface: str):
    """Skip in the public export, which does not carry `surface`."""
    return pytest.mark.skipif(PUBLIC_EXPORT, reason=f"public export: reads {surface}, which the export leaves out")
