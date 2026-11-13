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
project's own files are only read.

A third review found that an index flag hides an edit from `git status`: with a linked source
marked assume-unchanged and then edited, the status printed nothing and the identity said clean.
The build script now also lists the index's flags (`git ls-files -v`), and a clean status with an
entry flagged assume-unchanged or skip-worktree gives "unknown", never false;
`test_an_index_flag_that_hides_an_edit_makes_the_flag_unknown` sets each flag in the copy only.

No call here can reach another repository. A second review found that git obeys an inherited
GIT_DIR and GIT_WORK_TREE, so that the copy's `git init`, `add` and `commit`, and the build
script's queries, could act on whatever repository those named. Every git call here and in
`build.rs` therefore runs with every GIT_* variable removed, with no system or global
configuration, bound to an explicit path (`-C`, and `--git-dir`/`--work-tree` once the
repository exists), and the top level git reports is checked against the expected folder before
any answer is used. `test_an_inherited_git_dir_reaches_no_other_repository` sets GIT_DIR and
GIT_WORK_TREE to a second disposable repository and shows it unchanged, with the copy's
identity still correct.

The build tests run Cargo themselves (the first build from scratch), so they are marked
`runs_cargo`: hosted CI runs them, and a local session with OPENPIPESTRESS_NO_CARGO_BUILD=1 skips
them. Behaviour only: nothing here is a timing.

Every build here is `--offline`. On main these builds once inherited whatever the process
environment held, and reached the registry when it let them: an earlier test on the same worker
(the evidence sweep's, which sets CARGO_NET_OFFLINE in its own process) left them offline, and
they then passed or failed by whether some other build of the same job had already cached the
driver's locked dependencies. Now they never reach the registry, whatever the environment says:
the hosted runner fetches the locked dependencies before pytest starts (`tools/ci/python_ci.py`
`OFFLINE_BUILDS`), and where they are missing the failure names the command that fetches them.
`test_the_builds_never_reach_the_registry` builds against an empty Cargo home and requires a
refusal and no downloaded crate.
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
RUNS_CARGO = pytest.mark.runs_cargo("cargo builds of the release timing driver in a copy of its crates")
# What provides the driver's locked dependencies to these offline builds (the hosted runner runs it before pytest).
FETCH = f"cargo fetch --locked --manifest-path {(CRATE / 'Cargo.toml').as_posix()}"


def isolated_env(ceiling: Path) -> dict[str, str]:
    """The environment of every git call here: no inherited GIT_* variable (GIT_DIR and
    GIT_WORK_TREE among them), no system or global configuration, and git's search for a
    repository stopped at `ceiling`."""
    env = {name: value for name, value in os.environ.items() if not name.startswith("GIT_")}
    env.update(GIT_CONFIG_NOSYSTEM="1", GIT_CONFIG_GLOBAL=os.devnull, GIT_CEILING_DIRECTORIES=str(ceiling),
               HOME=str(ceiling / "no-home"), XDG_CONFIG_HOME=str(ceiling / "no-home"))
    return env


def run_git(root: Path, *args: str, bound: bool = True) -> str:
    """One isolated git call on the repository at `root`, bound to it explicitly: `-C root`, and
    once it exists `--git-dir root/.git --work-tree root`."""
    binding = ["--git-dir", str(root / ".git"), "--work-tree", str(root)] if bound else []
    done = subprocess.run(["git", "--no-optional-locks", "-C", str(root), *binding,
                           "-c", "user.name=release-timing-test", "-c", "user.email=release-timing-test@invalid",
                           "-c", "commit.gpgsign=false", "-c", "init.defaultBranch=main", "-c", "core.hooksPath=/dev/null",
                           *args], capture_output=True, text=True, check=True, env=isolated_env(root.parent))
    return done.stdout.strip()


def verified_root(root: Path) -> Path:
    """`root`, once git, bound to it, reports it as the top level and its own `.git` as the git
    directory; anything else stops the test before any answer is used."""
    top = Path(run_git(root, "rev-parse", "--show-toplevel")).resolve()
    git_dir = Path(run_git(root, "rev-parse", "--absolute-git-dir")).resolve()
    assert (top, git_dir) == (root.resolve(), (root / ".git").resolve()), (top, git_dir)
    return root


def new_repository(root: Path, message: str) -> str:
    """A fresh repository at `root` holding its files, committed; returns the commit, read from
    `root/.git` itself, where the commit must have landed."""
    run_git(root, "init", "-q", str(root), bound=False)
    verified_root(root)
    run_git(root, "add", "-A")
    run_git(root, "commit", "-q", "-m", message)
    commit = run_git(root, "rev-parse", "--verify", "HEAD")
    landed = (root / ".git" / "refs" / "heads" / "main").read_text().strip()
    assert landed == commit, "the commit landed in the copy's own .git"
    return commit


def copy_workspace(destination: Path) -> None:
    """The tracked files under COPIED, read (never written) from the project's own repository,
    whose top level is checked to be the project first (it may be a linked worktree, so its git
    directory is not assumed to be `.git`)."""
    project_git = ["git", "--no-optional-locks", "-C", str(PROJECT)]
    env = isolated_env(PROJECT.parent)
    top = subprocess.run([*project_git, "rev-parse", "--show-toplevel"], capture_output=True, text=True, check=True,
                         env=env).stdout.strip()
    assert Path(top).resolve() == PROJECT.resolve(), top
    listed = subprocess.run([*project_git, "ls-files", "-z", "--", *COPIED], capture_output=True, check=True,
                            env=env).stdout
    for name in filter(None, listed.decode().split("\0")):
        source = PROJECT / name
        if not source.is_file() or source.is_symlink():
            continue  # outside a sparse checkout, or not a regular file
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)  # a copy, never a link: editing it cannot reach the project


def build_identity(copy: Path, target: Path, env: dict[str, str] | None = None) -> dict:
    """Builds the copy's driver, offline, and prints its identity. `env` is the environment cargo (and
    so the build script) inherits: by default this process's, GIT_* variables and all."""
    env = dict(os.environ if env is None else env, CARGO_TARGET_DIR=str(target))
    built = subprocess.run(["cargo", "build", "--quiet", "--locked", "--offline", "--bin", "release_timing"],
                           cwd=copy / CRATE, capture_output=True, text=True, check=False, env=env)
    assert built.returncode == 0, (f"the offline build failed; if a dependency is missing from Cargo's cache, run "
                                   f"`{FETCH}` from the project first.\n{built.stderr[-4000:]}")
    shown = subprocess.run([str(target / "debug" / "release_timing"), "--build-identity"], capture_output=True,
                           text=True, check=True)
    return json.loads(shown.stdout)


def snapshot(repository: Path) -> tuple:
    """Everything a stray git call could change in `repository`: its refs, its index and its
    working files, read directly from the file system (no git call)."""
    files = sorted((p.relative_to(repository).as_posix(), p.read_bytes()) for p in repository.rglob("*") if p.is_file())
    return tuple(files)


@pytest.fixture
def hostile_repository(tmp_path, monkeypatch):
    """A second, disposable repository that inherited GIT_DIR and GIT_WORK_TREE point at."""
    other = tmp_path / "other"
    other.mkdir()
    (other / "README").write_text("an unrelated repository\n")
    new_repository(other, "the unrelated repository")
    monkeypatch.setenv("GIT_DIR", str(other / ".git"))
    monkeypatch.setenv("GIT_WORK_TREE", str(other))
    return other


def test_the_copys_git_calls_reach_no_other_repository(tmp_path, hostile_repository):
    """No build: with GIT_DIR and GIT_WORK_TREE set to another repository, making the copy a
    repository and committing it leaves that repository unchanged."""
    before = snapshot(hostile_repository)
    copy = tmp_path / "workspace"
    (copy / "core").mkdir(parents=True)
    (copy / "core" / "x.rs").write_text("// a file of the copy\n")
    commit = new_repository(copy, "the copy, clean")
    assert run_git(copy, "status", "--porcelain", "--untracked-files=all") == ""
    assert snapshot(hostile_repository) == before, "the other repository was touched"
    assert run_git(hostile_repository, "rev-parse", "--verify", "HEAD") != commit


@RUNS_CARGO
def test_an_unstaged_edit_of_a_linked_product_source_makes_the_identity_dirty(tmp_path):
    original = (PROJECT / EDITED).read_bytes()
    copy, target = tmp_path / "workspace", tmp_path / "target"
    copy_workspace(copy)
    assert (copy / CRATE / "Cargo.toml").is_file() and (copy / EDITED).is_file()
    commit = new_repository(copy, "the copy, clean")
    # Cargo keeps HOME (rustup and cargo find their toolchain there); no GIT_* variable reaches it.
    # The build script isolates its own git calls in any case (the next test sets GIT_DIR).
    env = {name: value for name, value in os.environ.items() if not name.startswith("GIT_")}

    clean = build_identity(copy, target, env)
    assert (clean["source_commit"], clean["source_dirty"]) == (commit, False), clean

    # An unstaged edit of a linked product source: cargo rebuilds that crate and relinks the driver.
    with open(copy / EDITED, "ab") as edited:
        edited.write(b"\n// release timing build identity regression: an unstaged edit in the copy\n")
    assert run_git(copy, "status", "--porcelain", "--untracked-files=all").split() == ["M", EDITED.as_posix()]
    dirty = build_identity(copy, target, env)
    assert (dirty["source_commit"], dirty["source_dirty"]) == (commit, True), dirty

    # No repository: git cannot establish the state, and the flag says so instead of false.
    shutil.rmtree(copy / ".git")
    unknown = build_identity(copy, target, env)
    assert (unknown["source_commit"], unknown["source_dirty"]) == ("unknown", "unknown"), unknown

    assert (PROJECT / EDITED).read_bytes() == original, "the project's own source was not touched"


@RUNS_CARGO
def test_an_inherited_git_dir_reaches_no_other_repository(tmp_path, hostile_repository):
    """GIT_DIR and GIT_WORK_TREE name a second repository for the whole test, the build included
    (cargo passes them to the build script): that repository is unchanged, and the identity is
    still the copy's, not the other repository's."""
    before = snapshot(hostile_repository)
    other_commit = run_git(hostile_repository, "rev-parse", "--verify", "HEAD")
    copy, target = tmp_path / "workspace", tmp_path / "target"
    copy_workspace(copy)
    commit = new_repository(copy, "the copy, clean")
    assert os.environ["GIT_DIR"] == str(hostile_repository / ".git")
    identity = build_identity(copy, target)  # this process's environment, GIT_DIR and GIT_WORK_TREE set
    assert (identity["source_commit"], identity["source_dirty"]) == (commit, False), identity
    assert identity["source_commit"] != other_commit
    assert snapshot(hostile_repository) == before, "the other repository was touched"


@RUNS_CARGO
def test_the_builds_never_reach_the_registry(tmp_path):
    """With an empty Cargo home, so that no crate is cached, and with CARGO_NET_OFFLINE removed, so that
    only the build's own flag holds it offline, the copy's build is refused and downloads nothing."""
    copy, target, cargo_home = tmp_path / "workspace", tmp_path / "target", tmp_path / "cargo-home"
    copy_workspace(copy)
    new_repository(copy, "the copy, clean")
    cargo_home.mkdir()
    env = {name: value for name, value in os.environ.items()
           if not name.startswith("GIT_") and name != "CARGO_NET_OFFLINE"}
    env["CARGO_HOME"] = str(cargo_home)
    with pytest.raises(AssertionError) as refused:
        build_identity(copy, target, env)
    assert "offline" in str(refused.value) and FETCH in str(refused.value), str(refused.value)[-2000:]
    assert not list(cargo_home.rglob("*.crate")), "a crate was downloaded"
    assert not (target / "debug" / "release_timing").exists()


def project_index_flags() -> str:
    """The project's own index entry for EDITED with its flag tag, read (never written) through the
    same isolated, bound call as `copy_workspace`."""
    project_git = ["git", "--no-optional-locks", "-C", str(PROJECT)]
    return subprocess.run([*project_git, "ls-files", "-v", "--", EDITED.as_posix()], capture_output=True, text=True,
                          check=True, env=isolated_env(PROJECT.parent)).stdout


@RUNS_CARGO
def test_an_index_flag_that_hides_an_edit_makes_the_flag_unknown(tmp_path):
    original, flags = (PROJECT / EDITED).read_bytes(), project_index_flags()
    copy, target = tmp_path / "workspace", tmp_path / "target"
    copy_workspace(copy)
    commit = new_repository(copy, "the copy, clean")
    env = {name: value for name, value in os.environ.items() if not name.startswith("GIT_")}
    clean = build_identity(copy, target, env)
    assert (clean["source_commit"], clean["source_dirty"]) == (commit, False), clean

    source = copy / EDITED
    before = source.read_bytes()
    for flag in ("assume-unchanged", "skip-worktree"):
        # In the copy only: the flag on a linked source, then an edit that git status no longer shows.
        run_git(copy, "update-index", "--" + flag, EDITED.as_posix())
        source.write_bytes(before + b"\n// release timing build identity regression: an edit hidden by " + flag.encode()
                           + b"\n")
        assert run_git(copy, "status", "--porcelain", "--untracked-files=all") == "", "git status hides the edit"
        hidden = build_identity(copy, target, env)
        assert (hidden["source_commit"], hidden["source_dirty"]) == (commit, "unknown"), (flag, hidden)
        source.write_bytes(before)
        run_git(copy, "update-index", "--no-" + flag, EDITED.as_posix())

    assert (PROJECT / EDITED).read_bytes() == original, "the project's own source was not touched"
    assert project_index_flags() == flags, "the project's own index was not touched"
