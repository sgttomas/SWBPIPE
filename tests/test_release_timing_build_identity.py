#!/usr/bin/env python3
"""The release timing driver's build identity follows the tree it was linked from (DEL-04-05).

An independent review found that an incrementally rebuilt driver could record a dirty product as
clean: its build script watched only its own crate and Git's control files, so an unstaged edit
of a linked product crate rebuilt that crate and relinked the driver without rerunning the
script, and the embedded identity kept `source_dirty: false`. The script now runs on every build,
and where git cannot establish the state the flag says "unknown", never false.

The regression: build the driver in a clean tree, edit a linked product source without staging
it, rebuild, and require the identity (`release_timing --build-identity`) to say dirty; then,
with no repository at all, require "unknown". It runs on a COPY of the driver's crates and of
what they compile in (the tracked files under `core/`, `fixtures/`, `schemas/` and `validation/`),
made into a fresh repository under pytest's temporary folder. Only the copy is edited; the
project's own files are only read, so no failure path, an interruption or a kill included, can
leave a product source modified. Its builds use their own target folder there.

It runs Cargo itself (three builds of the driver and its product crates, the first from scratch),
so it is marked `runs_cargo`: hosted CI runs it, and a local session with
OPENPIPESTRESS_NO_CARGO_BUILD=1 skips it. Behaviour only: nothing here is a timing.
"""
from __future__ import annotations

import json
import os
import shutil
import subprocess
from pathlib import Path

import pytest

PROJECT = Path(__file__).resolve().parents[1]
CRATE = Path("core") / "solver" / "release_timing"
# A product source the driver links (open_pipe_stress_product_physics), edited in the copy only.
EDITED = Path("core") / "product_physics" / "src" / "lib.rs"
# The driver's path-dependency crates are all under core/; they compile in files under these too.
COPIED = ("core", "fixtures", "schemas", "validation")
CI_READS = ("core/**", "fixtures/**", "schemas/**", "validation/**")

pytestmark = pytest.mark.runs_cargo("three cargo builds of the release timing driver in a copy of its crates")


def git_env(ceiling: Path) -> dict[str, str]:
    # The copy's parent bounds git's search, so a folder above it is never taken for its repository.
    return dict(os.environ, GIT_CEILING_DIRECTORIES=str(ceiling), GIT_CONFIG_NOSYSTEM="1")


def copy_workspace(destination: Path) -> None:
    listed = subprocess.run(["git", "-C", str(PROJECT), "ls-files", "-z", "--", "core", "fixtures", "schemas",
                             "validation"], capture_output=True, check=True).stdout
    for name in filter(None, listed.decode().split("\0")):
        source = PROJECT / name
        if not source.is_file() or source.is_symlink():
            continue  # outside a sparse checkout, or not a regular file
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)  # a copy, never a link: editing it cannot reach the project


def git(copy: Path, *args: str) -> str:
    done = subprocess.run(["git", "-C", str(copy), "-c", "user.name=release-timing-test",
                           "-c", "user.email=release-timing-test@invalid", "-c", "commit.gpgsign=false",
                           "-c", "init.defaultBranch=main", *args],
                          capture_output=True, text=True, check=True, env=git_env(copy.parent))
    return done.stdout.strip()


def build_identity(copy: Path, target: Path) -> dict:
    env = dict(git_env(copy.parent), CARGO_TARGET_DIR=str(target))
    built = subprocess.run(["cargo", "build", "--quiet", "--locked", "--bin", "release_timing"], cwd=copy / CRATE,
                           capture_output=True, text=True, check=False, env=env)
    assert built.returncode == 0, built.stderr[-4000:]
    shown = subprocess.run([str(target / "debug" / "release_timing"), "--build-identity"], capture_output=True,
                           text=True, check=True)
    return json.loads(shown.stdout)


def test_an_unstaged_edit_of_a_linked_product_source_makes_the_identity_dirty(tmp_path):
    original = (PROJECT / EDITED).read_bytes()
    copy, target = tmp_path / "workspace", tmp_path / "target"
    copy_workspace(copy)
    assert (copy / CRATE / "Cargo.toml").is_file() and (copy / EDITED).is_file()
    git(copy, "init", "-q")
    git(copy, "add", "-A")
    git(copy, "commit", "-q", "-m", "the copy, clean")
    commit = git(copy, "rev-parse", "HEAD")

    clean = build_identity(copy, target)
    assert (clean["source_commit"], clean["source_dirty"]) == (commit, False), clean

    # An unstaged edit of a linked product source: cargo rebuilds that crate and relinks the driver.
    with open(copy / EDITED, "ab") as edited:
        edited.write(b"\n// release timing build identity regression: an unstaged edit in the copy\n")
    assert git(copy, "status", "--porcelain").split() == ["M", EDITED.as_posix()], "unstaged, and the only change"
    dirty = build_identity(copy, target)
    assert (dirty["source_commit"], dirty["source_dirty"]) == (commit, True), dirty

    # No repository: git cannot establish the state, and the flag says so instead of false.
    shutil.rmtree(copy / ".git")
    unknown = build_identity(copy, target)
    assert (unknown["source_commit"], unknown["source_dirty"]) == ("unknown", "unknown"), unknown

    assert (PROJECT / EDITED).read_bytes() == original, "the project's own source was not touched"
