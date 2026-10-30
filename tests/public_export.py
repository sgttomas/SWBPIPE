"""Tell the public SWBPIPE export's tree from the monorepo's.

`tools/release/export_public_openpipestress.py` stages this project without the
monorepo's `loop/` and `execution/` (its `EXCLUDED_TOP_LEVEL`) and without the
repository root around it. A test that reads a surface the export leaves out
runs in the monorepo and, in the export, skips with the surface named
(DEL-10-04:1, a self-verifying export).

The tree counts as the export only outside a monorepo checkout, and only with
both directories absent. In a monorepo checkout this project sits at
`projects/<name>` under the repository root, which holds `.git` (a sparse clone
or worktree keeps it) or `AGENTS.md`, and this directory has no `.git` of its
own. The export is a repository of its own, so a `.git` here marks the export
even where it is checked out inside another repository. A sparse monorepo clone
that drops the directories, or hosted CI's checkout (which drops `execution/`
only), is not the export.
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
