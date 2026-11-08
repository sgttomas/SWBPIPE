"""The execution/ boundary guard (tools/validation/execution_boundary.py): the real tree, the 22 accepted
SCA-011 fidelity exceptions, and adversarial probes from its threat model in temporary repositories."""
import csv
import hashlib
import importlib.util
import os
from pathlib import Path
import subprocess

import pytest

ROOT = Path(__file__).resolve().parents[2]
spec = importlib.util.spec_from_file_location("execution_boundary", ROOT / "tools/validation/execution_boundary.py")
guard = importlib.util.module_from_spec(spec)
spec.loader.exec_module(guard)
SCA011 = ROOT / guard.SCA011


def test_the_repository_keeps_execution_declarative():
    assert guard.check(ROOT) == []


def test_fidelity_exceptions_are_exactly_the_accepted_helper_sources():
    tracked = subprocess.check_output(["git", "-C", str(ROOT), "ls-files", "-s", "--", "execution"], text=True)
    python = {line.split("\t", 1)[1]: line.split()[0] for line in tracked.splitlines() if line.endswith(".py")}
    assert set(python) == set(guard.FIDELITY) and len(python) == 22
    assert set(python.values()) == {"100644"}
    with (SCA011 / "ACCEPTED_MANIFEST.csv").open(newline="") as stream:
        accepted = {row["Path"].removeprefix("projects/chirality-piping/"): row["SHA256"] for row in csv.DictReader(stream)}
    for path, digest in guard.FIDELITY.items():
        assert accepted[path] == digest == hashlib.sha256((ROOT / path).read_bytes()).hexdigest(), path


class Repo:
    def __init__(self, root: Path):
        self.root = root
        subprocess.run(["git", "init", "-q", str(root)], check=True)
        self.write("execution/PKG-01/DEL-01-01/ScopeOfWork.md", "# Scope\n")

    def write(self, path, text, mode=None):
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text)
        subprocess.run(["git", "-C", str(self.root), "add", "--", path], check=True)
        if mode:
            subprocess.run(["git", "-C", str(self.root), "update-index", f"--chmod={mode}", "--", path], check=True)

    def link(self, path, target):
        (self.root / path).parent.mkdir(parents=True, exist_ok=True)
        os.symlink(target, self.root / path)
        subprocess.run(["git", "-C", str(self.root), "add", "--", path], check=True)

    def findings(self, fidelity=None):
        return {(kind, path) for kind, path, _, _ in guard.check(self.root, fidelity or {}, {})}


EXEC_PROBES = {
    "disguised source read then compiled": 'from pathlib import Path\nROOT = Path(__file__).parents[1]\n'
        'source = (ROOT / "execution/PKG-01/x.txt").read_text()\nexec(compile(source, "x", "exec"))\n',
    "component-built path, runpy": 'import runpy\nfrom pathlib import Path\nbase = Path(__file__).parents[1]\n'
        'runpy.run_path(str(base / ("exec" + "ution") / "PKG-01" / "a.json"))\n',
    "sys.path": 'import sys\nfrom pathlib import Path\nsys.path.insert(0, str(Path(__file__).parents[1] / "execution" / "PKG-01"))\n',
    "sys.path slice": 'import sys\nsys.path[:] = ["execution/PKG-01"]\n',
    "PYTHONPATH": 'import os, subprocess\nsubprocess.run(["true"], env={**os.environ, "PYTHONPATH": "execution/PKG-01"})\n',
    "f-string loader": 'import importlib.util\nROOT = "."\nname = "x"\n'
        'spec = importlib.util.spec_from_file_location(name, f"{ROOT}/execution/{name}.md")\n',
    "interpreter argv": 'import subprocess, sys\nsubprocess.run([sys.executable, "execution/PKG-01/x.md"])\n',
    "shell string": 'import os\nos.system("python3 execution/PKG-01/x.txt --apply")\n',
    "package import": 'import execution.PKG_01\n',
    "dotted import_module": 'import importlib\nimportlib.import_module("execution.PKG_01.x")\n',
    "accepted helper interdependency": 'import runpy\nfrom pathlib import Path\nROOT = Path(__file__).parents[1]\n'
        'runpy.run_path(ROOT / "execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/application/dependencies/validate_staged.py")\n',
    "through a same-file helper": 'import runpy\n\ndef load(path):\n    return runpy.run_path(path)\n\n'
        'def main():\n    load("execution/PKG-01/x.json")\n',
    "pytest collection": 'import pytest\npytest.main(["execution/PKG-01"])\n',
}


@pytest.mark.parametrize("name", sorted(EXEC_PROBES))
def test_python_execution_edges_are_rejected_even_in_document_readers(tmp_path, name):
    repo = Repo(tmp_path)
    repo.write("tools/probe.py", EXEC_PROBES[name])
    repo.write("tools/validation/test_execution_dag.py", EXEC_PROBES[name])  # a declared reader gains nothing
    found = repo.findings()
    assert {("exec", "tools/probe.py"), ("exec", "tools/validation/test_execution_dag.py")} <= found
    assert {kind for kind, _ in found} <= {"exec", "read"}


def test_imported_module_constant_reaching_a_loader_is_rejected(tmp_path):
    repo = Repo(tmp_path)
    repo.write("tools/spec_paths.py", 'SPEC = "execution/PKG-01/x.md"\n')
    repo.write("tools/use.py", 'import runpy\nfrom tools.spec_paths import SPEC\nrunpy.run_path(SPEC)\n')
    repo.write("tools/use2.py", 'import runpy\nimport tools.spec_paths as paths\nrunpy.run_path(paths.SPEC)\n')
    repo.write("tools/use3.py", 'import runpy\ndef run():\n'
               '    from tools.spec_paths import SPEC as local_spec\n    runpy.run_path(local_spec)\n')
    assert repo.findings() == {("exec", "tools/use.py"), ("exec", "tools/use2.py"), ("exec", "tools/use3.py")}


def test_product_reads_need_a_declared_document_reader_and_unknown_uses_are_exposed(tmp_path):
    repo = Repo(tmp_path)
    read = 'from pathlib import Path\nSPEC = Path("execution/PKG-01/x.md")\ntext = SPEC.read_text()\n'
    repo.write("tools/product.py", read)
    repo.write("tools/validation/test_reference_system_ledger.py", read)
    repo.write("tools/git_reader.py", 'import subprocess\nsubprocess.run(["git", "show", "HEAD:execution/PKG-01/x.md"])\n')
    repo.write("tools/unknown.py", 'import helper\nhelper.consume("execution/PKG-01/x.md")\n')
    assert repo.findings() == {("read", "tools/product.py"), ("read", "tools/git_reader.py"),
                               ("unclassified", "tools/unknown.py")}


def test_provenance_comments_exclusions_and_temporary_fixtures_are_not_edges(tmp_path):
    repo = Repo(tmp_path)
    repo.write("tools/provenance.py", '"""Design: execution/PKG-01/x.md section 3."""\n'
               '# python3 execution/PKG-01/x.py\nimport json, subprocess\n'
               'RECORD = {"specification": "execution/PKG-01/x.md", "basis": "execution/PKG-01/x.md §3 (S2)"}\n'
               'print(json.dumps(RECORD))\n'
               'subprocess.run(["git", "diff", "--", ":!execution/"])\n'
               'SKIP = ("target", "execution")\n'
               'def keep(name):\n    return name not in SKIP and not name.startswith("execution/")\n')
    repo.write("tests/test_fixture.py", 'def test_x(tmp_path):\n    p = tmp_path / "execution/_DAG/_LATEST.md"\n'
               '    p.parent.mkdir(parents=True)\n    p.write_text("x")\n')
    repo.write(".github/workflows/ci.yml", "jobs:\n  a:\n    steps:\n      - uses: actions/checkout@v4\n"
               "        with:\n          sparse-checkout: |\n            /*\n            !/execution/\n")
    repo.write("core/a/src/lib.rs", '// execution/PKG-01/x.md\nconst SKIP: [&str; 1] = ["execution"];\n'
               'fn f(n: &str) -> bool { n.starts_with("execution/") }\n')
    repo.write("apps/ui/Panel.tsx", 'export const SOURCE = "execution/PKG-01/ScopeOfWork.md";\n')
    assert repo.findings() == set()


@pytest.mark.parametrize("path,text", [
    ("core/a/src/lib.rs", 'const S: &str = include_str!("../../../execution/PKG-01/x.json");\n'),
    ("core/a/src/mod.rs", '#[path = "../../../execution/PKG-01/x.rs"]\nmod x;\n'),
    ("apps/ui/load.ts", 'import data from "../../execution/PKG-01/x.json";\n'),
    ("apps/ui/load2.mjs", 'const p = path.join(root, "execution", "PKG-01");\n'),
    ("apps/ui/vite.config.ts", 'export default { test: { include: ["execution/PKG-01/**"] } };\n'),
    ("package.json", '{"scripts": {"x": "node execution/PKG-01/x.json"}}\n'),
    ("core/a/Cargo.toml", '[package]\nname = "a"\nbuild = "../../execution/PKG-01/build.rs"\n'),
    ("tools/run.sh", "#!/bin/sh\npython3 execution/PKG-01/x.md\n"),
    (".github/workflows/ci.yml", "jobs:\n  a:\n    steps:\n      - run: python3 execution/PKG-01/x.md\n"),
    ("software-workflow.json", '{"checks": {"x": {"cwd": ".", "command": ["python3", "execution/PKG-01/x.md"]}}}\n'),
    ("tools/site.pth", "execution/PKG-01\n"),
])
def test_other_language_and_entrypoint_edges_are_rejected(tmp_path, path, text):
    repo = Repo(tmp_path)
    repo.write(path, text)
    assert ("exec", path) in repo.findings()


def test_execution_holds_only_declarative_regular_files(tmp_path):
    repo = Repo(tmp_path)
    repo.write("execution/PKG-01/run.py", "print(1)\n")
    repo.write("execution/PKG-01/tool.sh", "echo\n")
    repo.write("execution/PKG-01/notes.md", "# notes\n", mode="+x")
    repo.write("execution/PKG-01/payload.txt", "#!/usr/bin/env python3\nprint(1)\n")
    repo.write("execution/PKG-01/package.json", "{}\n")
    repo.write("execution/PKG-01/.gitattributes", "*.md filter=lfs\n")
    repo.link("execution/PKG-01/link.md", "notes.md")
    repo.link("tools/shortcut.py", "../execution/PKG-01/notes.md")
    repo.write("execution/PKG-01/ok.yaml", "a: 1\n")
    assert repo.findings() == {("tree", f"execution/PKG-01/{name}") for name in (
        "run.py", "tool.sh", "notes.md", "payload.txt", "package.json", ".gitattributes", "link.md")} | {
        ("exec", "tools/shortcut.py")}


def test_fidelity_exceptions_fail_on_any_byte_mode_or_presence_change(tmp_path):
    repo = Repo(tmp_path)
    source = "print('accepted')\n"
    path = "execution/_ScopeChange/SCA-011_2026-09-22_OWNERSHIP/dependencies/validate_candidate.py"
    fidelity = {path: hashlib.sha256(source.encode()).hexdigest(),
                path.replace("validate_candidate", "build_candidate"): hashlib.sha256(b"x").hexdigest()}
    repo.write(path, source)
    assert repo.findings(fidelity) == {("tree", fidelity and path.replace("validate_candidate", "build_candidate"))}
    repo.write(path.replace("validate_candidate", "build_candidate"), "x")
    assert repo.findings(fidelity) == set()
    repo.write(path, source + "# edited\n")
    assert repo.findings(fidelity) == {("tree", path)}
    repo.write(path, source, mode="+x")
    assert ("tree", path) in repo.findings(fidelity)
