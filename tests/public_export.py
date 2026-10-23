"""Tell the public SWBPIPE export's tree from the monorepo's.

`tools/release/export_public_openpipestress.py` stages this project without the
monorepo's `loop/` and `execution/` (its `EXCLUDED_TOP_LEVEL`) and without the
repository root around it. A test that reads a surface the export leaves out
runs in the monorepo and, in the export, skips with the surface named
(DEL-10-04:1, a self-verifying export).

The tree counts as the export only outside a monorepo checkout, where this
project sits at `projects/<name>` under a root carrying `AGENTS.md`, and only
with both directories absent. A sparse monorepo clone that drops them, or
hosted CI's checkout (which drops `execution/` only), is not the export.
"""
from pathlib import Path

import pytest

PROJECT = Path(__file__).resolve().parents[1]
IN_MONOREPO = PROJECT.parent.name == "projects" and (PROJECT.parents[1] / "AGENTS.md").is_file()
PUBLIC_EXPORT = not IN_MONOREPO and not (PROJECT / "loop").is_dir() and not (PROJECT / "execution").is_dir()


def monorepo_only(surface: str):
    """Skip in the public export, which does not carry `surface`."""
    return pytest.mark.skipif(PUBLIC_EXPORT, reason=f"public export: reads {surface}, which the export leaves out")
