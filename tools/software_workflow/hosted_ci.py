#!/usr/bin/env python3
"""Route hosted jobs using the existing registered affected-check selector."""
from __future__ import annotations

import argparse
import functools
import json
import importlib.util
import os
from pathlib import Path, PurePosixPath
import re
import subprocess
import tomllib

from select_affected_checks import select_checks
from software_workflow_common import load_profile, matches

ROOT = Path(__file__).resolve().parents[2]
PROFILE = ROOT / "tools/hosted-ci-routing.json"
SUITES = ("piping", "piping-desktop", "piping-desktop-rust", "piping-numerical", "piping-python")
PIPING = ""
# Repository-level execution controls are shared inputs even after the
# product directory is promoted to repository root.
SHARED_INPUTS = ('.github/**', 'tools/software_workflow/**', 'tools/ci/**',
                 'tools/hosted-ci-routing.json', 'tools/tools-test-routing.json')


def shared_input(path):
    return matches(path, SHARED_INPUTS)

# The crates `apps/desktop/scripts/build-wasm-engine.mjs` compiles into the
# engines the desktop suite executes.
WASM_ENGINE_CRATES = ("core/model_operations/operation_applier/Cargo.toml", "core/loads/self_weight_wasm/Cargo.toml")
DESKTOP_SOURCES = "apps/desktop/src"
# The desktop shell crate, whose Cargo tests the desktop Rust job runs.
DESKTOP_RUST_CRATE = "apps/desktop/src-tauri"
# A project-relative path written in desktop source, cut at a template,
# glob, query or fragment; `core/units catalog` names `core/units`.
PROJECT_PATH = re.compile(r"(?<![\w./-])((?:api|apps|core|docs|examples|fixtures|governance|provenance|schemas|tests|tools|validation)/[^\s'\"`$*?#]*)")
RELATIVE_PATH = re.compile(r"['\"`]((?:\.\./)+[^\s'\"`$*?#]+)")
# The selection job passes these to Playwright on its command line.
BROWSER_PROJECT = re.compile(r"[a-z0-9-]+")
BROWSER_SPEC = re.compile(r"[A-Za-z0-9_.-]+\.spec\.ts")
# Desktop test and test-support sources: a path they name may be read.
DESKTOP_TEST_SOURCE = re.compile(r"(^|/)(__tests__|__mocks__|test-support)/|\.(test|spec)\.[cm]?[jt]sx?$")
# A source line that may load the path it names rather than display it.
DESKTOP_READ_LINE = re.compile(r"\b(import|require|fetch|readFile\w*|open\w*)\b|\?(raw|url)\b")
# Always-on repository regressions that read the real tree's Rust,
# TypeScript, manifest and routing inputs and dominate the step's time. A
# 2026-10-30 trace of every file the regression step opens found that they
# read no Markdown, so a Markdown-only change cannot alter their result.
SOURCE_REACHABILITY_TESTS = (
    "tests/test_ci_numerical.py::SourceReadTests::test_repository_cross_crate_examples_select_readers",
    "tests/test_ci_numerical.py::ReachabilitySafetyTests::test_actual_complete_desktop_pr_diffs_retain_every_reader_and_controls_select_all",
    "tests/test_ci_numerical.py::NumericalTests::test_real_rust_literal_includes_require_numerical",
    "tests/test_ci_numerical.py::NumericalTests::test_real_rust_runtime_resource_literals_require_numerical",
    "tests/test_ci_numerical.py::ReachabilitySafetyTests::test_python_and_browser_checks_are_independent_of_cargo",
    "tools/software_workflow/test_hosted_ci.py::HostedCITests::test_desktop_resource_table_cannot_hide_new_unknown_fixture",
    "tools/software_workflow/test_hosted_ci.py::HostedCITests::test_desktop_rust_selection_fails_toward_running_the_tests",
)
# Markdown here is tool, test or workflow material, so it keeps every regression.
MARKDOWN_ONLY_EXCLUDED = (".github/**", "tools/**", "tests/**")


def git(root: Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(root), *args], stderr=subprocess.PIPE).decode("utf-8")


def numerical_module(root: Path):
    path = root / "tools/ci/numerical_ci.py"
    if not path.is_file():
        raise ValueError("Numerical selector unavailable")
    spec = importlib.util.spec_from_file_location("hosted_numerical_selection", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def numerical_plan(root: Path, paths: list[str] | None, profile: dict) -> dict:
    manifests = []
    try:
        module = numerical_module(root)
        project = root / "."
        manifests = module.readiness.discover_cargo_manifests(project)
        ignored = []
        for path in paths or []:
            rules = [rule for rule in profile["path_rules"] if matches(path, rule["paths"])]
            if rules and not any("piping-numerical" in rule["checks"] for rule in rules):
                ignored.append(path)
        desktop = profile.get("piping_desktop_resources", {})
        if desktop:
            if desktop.get("pointer") != "cases.fixture":
                raise ValueError("Unresolved desktop resource declaration")
            source = (project / desktop["source"]).resolve()
            source.relative_to(project.resolve())
            cases = json.loads(source.read_text())["cases"]
            actual = {case["fixture"] for case in cases}
            if actual != set(desktop.get("files", [])):
                raise ValueError("Stale desktop resource declaration")
            resources = {"" + path for path in actual}
            for path in resources:
                target = (root / path).resolve()
                target.relative_to(project.resolve())
                if not target.is_file():
                    raise ValueError("Unresolved desktop fixture: " + path)
            ignored.extend(p for p in paths or [] if p in resources)
        return module.numerical_selection(project, manifests, paths,
                                          profile.get("piping_runtime_reads", {}), ignored)
    except (ValueError, OSError, KeyError, TypeError, AttributeError, ImportError) as exc:
        return {"version": 2, "classification": "full-fallback", "manifests": [m.as_posix() for m in manifests],
                "reasons": [], "fallback_reasons": [str(exc)]}


# One selection run reads one checkout; scan it once.
@functools.lru_cache(maxsize=None)
def wasm_engine_inputs(root: Path) -> tuple[frozenset[str], frozenset[str]]:
    """Crate directories and exact files the wasm engines compile from.

    Follows normal and build path dependencies; dev dependencies are not
    compiled into the engines.
    """
    return compiled_crate_inputs(root, WASM_ENGINE_CRATES, False, "wasm engine")


@functools.lru_cache(maxsize=None)
def compiled_crate_inputs(root: Path, roots: tuple[str, ...], root_dev: bool,
                          label: str) -> tuple[frozenset[str], frozenset[str]]:
    """Crate directories and exact files that `roots` compile from.

    Follows normal and build path dependencies, and the roots' own dev
    dependencies when `root_dev` (their tests compile them). Compile reads are
    the literal `include*!` and `#[path]` targets in each crate's `src/` and
    `build.rs`, including test modules there (a superset). A read that is not
    a literal fails selection.
    """
    module = numerical_module(root)
    project = (root / PIPING).resolve()
    closure: set[Path] = set()
    pending = [Path(m) for m in roots]
    while pending:
        manifest = pending.pop()
        if manifest in closure:
            continue
        if manifest.parts[:1] == ("..",) or not (project / manifest).is_file():
            raise ValueError(f"Unresolved {label} crate: " + manifest.as_posix())
        closure.add(manifest)
        data = tomllib.loads((project / manifest).read_text())
        keys = ("dependencies", "build-dependencies")
        if root_dev and manifest.as_posix() in roots:
            keys += ("dev-dependencies",)
        tables = [data.get(key, {}) for key in keys]
        tables += [target.get(key, {}) for target in data.get("target", {}).values() for key in keys]
        for table in tables:
            for name, spec in table.items():
                if isinstance(spec, dict) and spec.get("workspace") is True:
                    raise ValueError(f"Unresolved inherited {label} dependency: " + name)
                if isinstance(spec, dict) and "path" in spec:
                    target = (project / manifest.parent / spec["path"] / "Cargo.toml").resolve()
                    pending.append(Path(os.path.relpath(target, project)))
    reads = set()
    for manifest in closure:
        crate = project / manifest.parent
        sources = sorted((crate / "src").rglob("*.rs")) + [p for p in [crate / "build.rs"] if p.is_file()]
        seen = set()
        while sources:
            source = sources.pop()
            if source in seen:
                continue
            seen.add(source)
            code = module.rust_source(source.read_text())
            syntax = module.RUST_STRING.sub(lambda m: " " * len(m.group()), code)
            for match in re.finditer(r"\binclude(?:_str|_bytes)?\s*!\s*\(|#\s*\[\s*path\s*=", syntax):
                start = match.end()
                while start < len(code) and code[start].isspace():
                    start += 1
                literal = module.RUST_STRING.match(code, start)
                if not literal:
                    raise ValueError(f"Unresolved {label} compile read in " + source.relative_to(project).as_posix())
                target = (source.parent / module.rust_literal(literal.group())).resolve()
                if not target.is_relative_to(project) or not target.is_file():
                    raise ValueError(f"Unresolved {label} compile read: " + str(target))
                reads.add(target.relative_to(project).as_posix())
                if not match.group().startswith("include"):
                    sources.append(target)
    return frozenset(m.parent.as_posix() for m in closure), frozenset(reads)


@functools.lru_cache(maxsize=None)
def desktop_reference_sites(root: Path) -> tuple[frozenset[str], frozenset[str]]:
    """Project paths, or directory prefixes, that desktop source names, and
    the subset a test, a relative path or a loading line may read."""
    project = (root / PIPING).resolve()
    sources = project / DESKTOP_SOURCES
    if not sources.is_dir():
        raise ValueError("Desktop sources unavailable")
    references, reads = set(), set()
    for source in sorted(sources.rglob("*")):
        if source.suffix not in {".ts", ".tsx", ".js", ".mjs"} or not source.is_file():
            continue
        test_source = bool(DESKTOP_TEST_SOURCE.search(source.relative_to(project).as_posix()))
        for line in source.read_text().splitlines():
            # Prose punctuation after a named path is not part of it.
            named = {match.rstrip("/.,;:)") for match in PROJECT_PATH.findall(line)}
            references.update(named)
            if test_source or DESKTOP_READ_LINE.search(line):
                reads.update(named)
            for match in RELATIVE_PATH.findall(line):
                target = Path(os.path.normpath(source.parent / match))
                if target.is_relative_to(project):
                    references.add(target.relative_to(project).as_posix())
                    reads.add(target.relative_to(project).as_posix())
    drop = {"", "."}
    return frozenset(references - drop), frozenset(reads - drop)


def desktop_references(root: Path) -> frozenset[str]:
    """Project paths, or directory prefixes, that desktop source names."""
    return desktop_reference_sites(root)[0]


def named_only_document(root: Path, relative: str) -> bool:
    """A Markdown file desktop source names only as text, outside tests and
    loading lines and under no named directory: editing its content cannot
    change a desktop test, though adding, deleting or renaming it can."""
    references, reads = desktop_reference_sites(root)
    return (relative.endswith(".md") and relative in references and relative not in reads
            and not any(ref != relative and relative.startswith(ref + "/") for ref in references))


def desktop_plan(root: Path, paths: list[str], selection: dict, profile: dict,
                 content_edits: frozenset[str] = frozenset()) -> dict:
    """Select the full desktop suite unless each changed input is Python or
    Rust code that the suite neither names nor compiles into its engines, or
    an in-place edit of a document desktop source only names."""
    piping_paths = set(selection["reasons"].get("piping", []))
    candidates = sorted(piping_paths | {p for p in paths if p.startswith(PIPING)})
    if not candidates:
        return {"classification": "not-applicable", "reasons": [], "fallback_reasons": []}
    try:
        crates, compile_reads = wasm_engine_inputs(root)
        references = desktop_references(root)
    except (ValueError, OSError, KeyError, TypeError, AttributeError, ImportError, tomllib.TOMLDecodeError) as exc:
        return {"classification": "full-fallback", "reasons": [], "fallback_reasons": [str(exc)]}
    reasons, named_documents = [], []
    for path in candidates:
        relative = path[len(PIPING):]
        named_document = path in content_edits and named_only_document(root, relative)
        if shared_input(path):
            kind = "shared-input"
        elif not named_document and any(relative == ref or relative.startswith(ref + "/") for ref in references):
            kind = "desktop-source-reference"
        elif path not in piping_paths:
            if named_document:
                named_documents.append(path)
            continue
        elif path in selection.get("unmatched_paths", []):
            kind = "unmatched-input"
        elif relative.startswith("apps/desktop/") or PurePosixPath(relative).suffix not in {".py", ".rs"}:
            kind = "desktop-input"
        elif relative in compile_reads:
            kind = "wasm-engine-compile-read"
        elif any(relative.startswith(crate + "/") for crate in crates):
            kind = "wasm-engine-source"
        else:
            continue
        reasons.append({"path": path, "kind": kind})
    plan = {"classification": "reachability" if reasons else "not-applicable", "reasons": reasons, "fallback_reasons": []}
    if named_documents:
        plan["named_document_edits"] = named_documents
    return plan


@functools.lru_cache(maxsize=None)
def desktop_rust_references(root: Path) -> frozenset[str]:
    """Project files and directories that desktop shell Rust names by a
    path-like string literal, resolved from the project, the crate or the
    source directory. Its tests read fixtures at run time through such
    literals; a format template names the directory before its first slot.
    As for desktop source, a bare word or a whole top-level directory is not
    a reference."""
    module = numerical_module(root)
    project = (root / PIPING).resolve()
    crate = project / DESKTOP_RUST_CRATE
    if not (crate / "src").is_dir():
        raise ValueError("Desktop Rust sources unavailable")
    references = set()
    for source in sorted(crate.rglob("*.rs")):
        if {"target", ".git"} & set(source.relative_to(crate).parts):
            continue
        for match in module.RUST_STRING.finditer(module.rust_source(source.read_text())):
            literal = module.rust_literal(match.group())
            if "{" in literal:
                # A format template names the directory before its first slot.
                literal = literal[:literal.index("{")].rpartition("/")[0]
            if "/" not in literal or "\0" in literal or "\n" in literal or len(literal) > 1024:
                continue
            for base in (project, crate, source.parent):
                target = Path(os.path.normpath(base / literal))
                try:
                    if not target.is_relative_to(project) or len(target.relative_to(project).parts) < 2:
                        continue
                    # A named file this diff deletes still names its directory.
                    if target.exists() or (target.suffix and target.parent != project and target.parent.is_dir()
                                           and len(target.parent.relative_to(project).parts) > 1):
                        references.add(target.relative_to(project).as_posix())
                except (OSError, ValueError):
                    continue
    return frozenset(references)


def desktop_rust_dependency_reads(root: Path, crates: frozenset[str], profile: dict) -> frozenset[str]:
    """Declared runtime reads of the path dependencies' `src/` and `build.rs`.

    The numerical selector's own check validates the declarations first; a
    stale or missing declaration fails selection.
    """
    return declared_compiled_reads(root, crates, json.dumps(profile.get("piping_runtime_reads", {}), sort_keys=True))


@functools.lru_cache(maxsize=None)
def declared_compiled_reads(root: Path, crates: frozenset[str], declarations_json: str) -> frozenset[str]:
    module = numerical_module(root)
    project = (root / PIPING).resolve()
    declarations = json.loads(declarations_json)
    module.source_read_dependencies(project, module.readiness.discover_cargo_manifests(project), declarations)
    reads = set()
    for source, declaration in declarations["reads"].items():
        if any(source == crate + "/build.rs" or source.startswith(crate + "/src/") for crate in crates):
            reads.update(name for entry in declaration["inputs"] for name in entry["files"])
    return frozenset(reads)


def desktop_rust_cargo_control(relative: str) -> bool:
    """A Cargo control the desktop job's `cargo test` reads: `.cargo/` configuration
    in its working directory (the project) or a directory above the crate, a
    `rust-toolchain` file there, or a project-level Cargo workspace file. Cargo
    reads configuration from the working directory and its ancestors, never from
    a dependency's own `.cargo/`; repository-root files are shared inputs."""
    parts = PurePosixPath(relative).parts
    ancestors = {PurePosixPath(DESKTOP_RUST_CRATE).parts[:i] for i in range(len(PurePosixPath(DESKTOP_RUST_CRATE).parts))}
    if ".cargo" in parts:
        return parts[:parts.index(".cargo")] in ancestors
    if parts and parts[-1] in {"rust-toolchain", "rust-toolchain.toml"}:
        return parts[:-1] in ancestors
    return parts in {("Cargo.toml",), ("Cargo.lock",)}


def desktop_rust_plan(root: Path, paths: list[str], selection: dict, profile: dict) -> dict:
    """Select the desktop shell's Cargo tests when a changed input is that
    crate, a path dependency it compiles, a file either compiles or reads, or
    a shared or unmatched input."""
    piping_paths = set(selection["reasons"].get("piping", []))
    shared = set(selection["reasons"].get("piping-desktop-rust", []))
    candidates = sorted(piping_paths | shared | {p for p in paths if p.startswith(PIPING)})
    if not candidates:
        return {"classification": "not-applicable", "reasons": [], "fallback_reasons": []}
    try:
        crates, compile_reads = compiled_crate_inputs(root, (DESKTOP_RUST_CRATE + "/Cargo.toml",), True, "desktop Rust")
        references = desktop_rust_references(root)
        runtime_reads = desktop_rust_dependency_reads(root, crates, profile)
    except (ValueError, OSError, KeyError, TypeError, AttributeError, ImportError, tomllib.TOMLDecodeError) as exc:
        return {"classification": "full-fallback", "reasons": [], "fallback_reasons": [str(exc)]}
    dependencies = crates - {DESKTOP_RUST_CRATE}
    reasons = []
    for path in candidates:
        relative = path[len(PIPING):]
        if shared_input(path):
            kind = "shared-input"
        elif desktop_rust_cargo_control(relative):
            kind = "cargo-control"
        elif path in selection.get("unmatched_paths", []):
            kind = "unmatched-input"
        elif relative.startswith(DESKTOP_RUST_CRATE + "/"):
            kind = "desktop-rust-source"
        elif relative in compile_reads:
            kind = "desktop-rust-compile-read"
        elif any(relative == ref or relative.startswith(ref + "/") for ref in references):
            kind = "desktop-rust-source-reference"
        elif relative in runtime_reads:
            kind = "path-dependency-runtime-read"
        elif (any(relative.startswith(crate + "/") for crate in dependencies)
              and (PurePosixPath(relative).suffix == ".rs" or PurePosixPath(relative).name in {"Cargo.toml", "Cargo.lock"})):
            kind = "path-dependency-source"
        elif PurePosixPath(relative).name.isascii() and PurePosixPath(relative).name.lower() in {"cargo.toml", "cargo.lock"}:
            # The shell's reach scan reads every project manifest's dependencies (W-18b
            # r2c), finding manifests by name case-insensitively, so any manifest or lock
            # change, in any letter case, can change its result.
            kind = "cargo-manifest"
        else:
            continue
        reasons.append({"path": path, "kind": kind})
    return {"classification": "reachability" if reasons else "not-applicable", "reasons": reasons, "fallback_reasons": []}


def browser_journeys(profile: dict) -> tuple[str, list[str], list[dict]]:
    """The product job's declared Chromium project, core specs and path groups.

    A malformed declaration fails selection: no spec set can be derived from it.
    """
    declaration = profile["piping_browser_journeys"]
    project, core, groups = declaration["project"], declaration["core"], declaration["groups"]
    if not isinstance(project, str) or not BROWSER_PROJECT.fullmatch(project):
        raise ValueError("Invalid browser project declaration")
    if not isinstance(core, list) or not core or not isinstance(groups, list):
        raise ValueError("Invalid browser spec declaration")
    names = set()
    for group in groups:
        if (not isinstance(group.get("name"), str) or group["name"] in names or not isinstance(group.get("reason"), str)
                or not isinstance(group.get("paths"), list) or not group["paths"]
                or not all(isinstance(path, str) and path for path in group["paths"])
                or not isinstance(group.get("specs"), list) or not group["specs"]):
            raise ValueError("Invalid browser spec group declaration")
        names.add(group["name"])
    for spec in core + [spec for group in groups for spec in group["specs"]]:
        if not isinstance(spec, str) or not BROWSER_SPEC.fullmatch(spec):
            raise ValueError("Invalid browser spec name: " + repr(spec))
    return project, list(core), groups


def browser_specs(core: list[str], groups: list[dict], selected: set[str]) -> list[str]:
    """The core specs, then each selected group's specs in declaration order."""
    specs = list(core)
    for group in groups:
        if group["name"] in selected:
            specs.extend(spec for spec in group["specs"] if spec not in specs)
    return specs


def browser_full(profile: dict, reason: str) -> dict:
    project, core, groups = browser_journeys(profile)
    return {"classification": "full-fallback", "project": project,
            "specs": browser_specs(core, groups, {group["name"] for group in groups}),
            "reasons": [], "fallback_reasons": [reason]}


def browser_plan(paths: list[str], selection: dict, profile: dict) -> dict:
    """The specs the product job runs when it runs: the core specs, and each
    group whose declared path the diff changes. A shared CI input or an
    unmatched project input selects every group."""
    project, core, groups = browser_journeys(profile)
    if selection["modes"]["piping"] != "full":
        return {"classification": "not-applicable", "project": project, "specs": [], "reasons": [], "fallback_reasons": []}
    piping_paths = set(selection["reasons"].get("piping", []))
    reasons, selected = [], set()
    for path in sorted(piping_paths | {p for p in paths if p.startswith(PIPING)}):
        if shared_input(path):
            kind, matched = "shared-input", groups
        elif path in selection.get("unmatched_paths", []):
            kind, matched = "unmatched-input", groups
        else:
            kind, matched = "group-path", [group for group in groups if matches(path[len(PIPING):], group["paths"])]
        for group in matched:
            selected.add(group["name"])
            reasons.append({"path": path, "kind": kind, "group": group["name"], "specs": group["specs"], "why": group["reason"]})
    return {"classification": "reachability" if reasons else "core", "project": project,
            "specs": browser_specs(core, groups, selected), "reasons": reasons, "fallback_reasons": []}


def route_paths(paths: list[str], profile: dict) -> dict:
    """The registered path rules' selection, with unmatched inputs given to
    their owning product or to every suite; reachability is not yet applied."""
    selection = select_checks(profile, paths)
    checks = set(selection["checks"])
    unmatched = [p for p in paths if not any(matches(p, rule["paths"]) for rule in profile["path_rules"])]
    for path in unmatched:
        owners = list(SUITES)
        checks.update(owners)
        for owner in owners:
            selection["reasons"].setdefault(owner, []).append(path)
    selection.update(checks=sorted(checks), unmatched_paths=unmatched)
    selection["modes"] = {suite: "full" if suite in checks else "not-applicable" for suite in SUITES}
    return selection


def select_paths(paths: list[str], profile: dict, root: Path = ROOT,
                 content_edits: frozenset[str] = frozenset()) -> dict:
    selection = route_paths(paths, profile)
    checks = set(selection["checks"])
    if "piping-numerical" in checks or any(p.startswith("") for p in paths):
        numerical = numerical_plan(root, paths, profile)
        selected = numerical["classification"] == "full-fallback" or bool(numerical["manifests"])
        selection["modes"]["piping-numerical"] = "full" if selected else "not-applicable"
        if selected:
            checks.add("piping-numerical")
            # A source consumed by Rust may occur in a records/desktop path rule.
            checks.add("piping-python")
            selection["modes"]["piping-python"] = "full"
        else:
            checks.discard("piping-numerical")
        selection["piping_numerical_selection"] = numerical
        selection["checks"] = sorted(checks)
    desktop = desktop_plan(root, paths, selection, profile, content_edits)
    if desktop["classification"] != "not-applicable":
        # The product job runs the desktop suite, so it is selected too.
        checks.update(("piping", "piping-desktop"))
        selection["modes"].update({"piping": "full", "piping-desktop": "full"})
    else:
        checks.discard("piping-desktop")
        selection["modes"]["piping-desktop"] = "not-applicable"
    selection["piping_desktop_selection"] = desktop
    desktop_rust = desktop_rust_plan(root, paths, selection, profile)
    selected = desktop_rust["classification"] != "not-applicable"
    (checks.add if selected else checks.discard)("piping-desktop-rust")
    selection["modes"]["piping-desktop-rust"] = "full" if selected else "not-applicable"
    selection["piping_desktop_rust_selection"] = desktop_rust
    selection["checks"] = sorted(checks)
    selection["piping_browser_selection"] = browser_plan(paths, selection, profile)
    return selection


REGULAR_FILE_MODES = {"100644", "100755"}


def changed_paths(root: Path, merge_base: str, head: str) -> tuple[list[str], frozenset[str]]:
    """Every changed path, and those whose content changed in place: a regular
    file before and after, not added, deleted, renamed, typechanged or a
    symlink, so its existence and kind are unchanged. --no-renames gives both
    sides of a rename; NUL delimiters preserve paths."""
    fields = [f for f in git(root, "diff", "--raw", "--no-renames", "--no-abbrev", "-z", merge_base, head, "--").split("\0") if f]
    if len(fields) % 2:
        raise ValueError("Malformed raw diff")
    paths, edits = set(), set()
    for header, path in zip(fields[0::2], fields[1::2]):
        parts = header.lstrip(":").split()
        if not header.startswith(":") or len(parts) != 5:
            raise ValueError("Malformed raw diff entry")
        old_mode, new_mode, _, _, status = parts
        paths.add(path)
        if status == "M" and old_mode == new_mode and new_mode in REGULAR_FILE_MODES:
            edits.add(path)
    return sorted(paths), frozenset(edits)


def text_at(root: Path, head: str, path: str) -> bool:
    """Whether the candidate's blob is UTF-8 text without NUL; a path absent
    from the candidate (a deletion) counts as text."""
    process = subprocess.run(["git", "-C", str(root), "cat-file", "blob", f"{head}:{path}"], capture_output=True)
    if process.returncode:
        if git(root, "ls-tree", "--name-only", head, "--", path).strip():
            raise ValueError(f"Unreadable candidate blob: {path}")
        return True
    if b"\0" in process.stdout:
        return False
    try:
        process.stdout.decode("utf-8")
    except UnicodeDecodeError:
        return False
    return True


def repository_plan(root: Path, event: str, base: str, head: str) -> dict:
    """Always-on repository regressions: all of them, except that a pull
    request whose complete diff is in-place edits of Markdown text outside
    tool, test and workflow trees omits SOURCE_REACHABILITY_TESTS, which read
    no Markdown. An addition, deletion, rename or typechange keeps them: a
    file's existence is itself an input."""
    resolved_head = git(root, "rev-parse", "--verify", head + "^{commit}").strip()
    plan = {"schema": "chirality-repository-ci/v1", "event": event, "head": resolved_head,
            "paths": [], "mode": "full", "deselect": [], "reason": "Not a pull request; every regression"}
    if event != "pull_request":
        return plan
    try:
        if not base:
            raise ValueError("missing base")
        merge_base = git(root, "merge-base", base, resolved_head).strip()
        paths, edits = changed_paths(root, merge_base, resolved_head)
        plan["paths"] = paths
        if not paths:
            plan["reason"] = "Empty diff; every regression"
            return plan
        for path in paths:
            if not path.endswith(".md") or matches(path, MARKDOWN_ONLY_EXCLUDED):
                plan["reason"] = f"{path} is not Markdown outside tool, test and workflow trees; every regression"
                return plan
            if path not in edits:
                plan["reason"] = f"{path} is added, deleted, renamed or not a regular file; every regression"
                return plan
            if not text_at(root, resolved_head, path):
                plan["reason"] = f"{path} is not UTF-8 text; every regression"
                return plan
    except (subprocess.CalledProcessError, UnicodeError, ValueError):
        plan["reason"] = "Unavailable complete PR diff; every regression"
        return plan
    plan.update(mode="markdown-only", deselect=list(SOURCE_REACHABILITY_TESTS),
                reason="In-place Markdown edits only; source-reachability regressions, which read no Markdown, omitted")
    return plan


def make_plan(root: Path, profile: dict, event: str, base: str, head: str) -> dict:
    resolved_head = git(root, "rev-parse", "--verify", head + "^{commit}").strip()
    plan = {"schema": "chirality-hosted-ci/v1", "event": event,
            "target_base": base, "head": resolved_head, "base": None,
            "paths": [], "checks": list(SUITES), "reasons": {}, "unmatched_paths": [],
            "modes": {suite: "full" for suite in SUITES}, "selection_reason": "Explicit full workflow dispatch",
            "piping_numerical_selection": numerical_plan(root, None, profile),
            "piping_desktop_selection": {"classification": "full-fallback", "reasons": [],
                                         "fallback_reasons": ["Full product coverage"]},
            "piping_desktop_rust_selection": {"classification": "full-fallback", "reasons": [],
                                              "fallback_reasons": ["Full product coverage"]},
            "piping_browser_selection": browser_full(profile, "Full product coverage")}
    if event != "pull_request":
        return plan
    try:
        if not base:
            raise ValueError("missing base")
        merge_base = git(root, "merge-base", base, resolved_head).strip()
        paths, content_edits = changed_paths(root, merge_base, resolved_head)
    except (subprocess.CalledProcessError, UnicodeError, ValueError):
        plan["selection_reason"] = "Unavailable complete PR diff; full product coverage"
        return plan
    plan.update(select_paths(paths, profile, root, content_edits))
    if "piping_numerical_selection" not in plan or not any(p.startswith("") for p in paths) and "piping-numerical" not in plan["checks"]:
        plan["piping_numerical_selection"] = {"version": 2, "classification": "not-applicable", "manifests": [], "reasons": [], "fallback_reasons": []}
    plan.update(schema="chirality-hosted-ci/v1", base=merge_base,
                selection_reason="Complete PR diff and registered path rules; unmatched inputs use full coverage")
    return plan


def validate_plan(root: Path, plan: dict) -> None:
    """Recompute candidate diff, routing, graph and exact manifest identity."""
    if plan.get("schema") != "chirality-hosted-ci/v1":
        raise ValueError("Unknown selection schema")
    if git(root, "rev-parse", "HEAD").strip() != plan.get("head"):
        raise ValueError("Selection does not name the checked-out candidate")
    _, profile = load_profile(root / "tools/hosted-ci-routing.json")
    expected = make_plan(root, profile, plan.get("event", ""), plan.get("target_base", ""), plan["head"])
    if plan != expected:
        raise ValueError("Selection differs from the complete candidate diff or current reachability inputs")


def aggregate(suite: str, mode: str, selection: str, product: str, instructions: str = "skipped") -> bool:
    if selection != "success":
        return False
    if mode == "full":
        return product == "success"
    return mode == "not-applicable" and product == "skipped" and instructions == "skipped"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    plan_parser = commands.add_parser("plan")
    plan_parser.add_argument("--event", required=True)
    plan_parser.add_argument("--base", default="")
    plan_parser.add_argument("--head", default="HEAD")
    plan_parser.add_argument("--suite", choices=SUITES, required=True)
    plan_parser.add_argument("--output", required=True)
    repository_parser = commands.add_parser("repository")
    repository_parser.add_argument("--event", required=True)
    repository_parser.add_argument("--base", default="")
    repository_parser.add_argument("--head", default="HEAD")
    repository_parser.add_argument("--output", required=True)
    result_parser = commands.add_parser("aggregate")
    result_parser.add_argument("--suite", choices=SUITES, required=True)
    result_parser.add_argument("--mode", required=True)
    result_parser.add_argument("--selection", required=True)
    result_parser.add_argument("--product", required=True)
    result_parser.add_argument("--instructions", default="skipped")
    args = parser.parse_args()
    if args.command == "aggregate":
        passed = aggregate(args.suite, args.mode, args.selection, args.product, args.instructions)
        print(f"{args.suite}: mode={args.mode}, selection={args.selection}, product={args.product}, instructions={args.instructions}")
        if passed and args.mode == "not-applicable":
            print(f"No inputs selected for {args.suite}; this suite was not run.")
        elif not passed and args.product in ("skipped", "", "cancelled"):
            print("Selected coverage did not complete; this is not a pass.")
        return 0 if passed else 1
    if args.command == "repository":
        plan = repository_plan(ROOT, args.event, args.base, args.head)
        if git(ROOT, "rev-parse", "HEAD").strip() != plan["head"]:
            raise ValueError("Selection checkout differs from requested candidate")
        Path(args.output).write_text(json.dumps(plan, indent=2, sort_keys=True) + "\n")
        print(json.dumps(plan, indent=2, sort_keys=True))
        if os.getenv("GITHUB_STEP_SUMMARY"):
            with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as stream:
                stream.write(f"### Repository regressions: {plan['mode']}\n\n{plan['reason']}.\n")
        return 0
    _, profile = load_profile(PROFILE)
    plan = make_plan(ROOT, profile, args.event, args.base, args.head)
    if git(ROOT, "rev-parse", "HEAD").strip() != plan["head"]:
        raise ValueError("Selection checkout differs from requested candidate")
    Path(args.output).write_text(json.dumps(plan, indent=2, sort_keys=True) + "\n")
    mode = plan["modes"][args.suite]
    print(json.dumps(plan, indent=2, sort_keys=True))
    if os.getenv("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a") as stream:
            stream.write(f"mode={mode}\n")
    if os.getenv("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a") as stream:
            stream.write(f"### {args.suite} CI selection: {mode}\n\n{plan['selection_reason']}. "
                         f"Candidate `{plan['head']}`; {len(plan['paths'])} changed paths.\n\n"
                         "Selection and reasons are printed in this job’s log.\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
