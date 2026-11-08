"""The live approved-graph pointer and its dependency register (`execution/_DAG/_LATEST.md`).

Document checks over the declarative graph records: they run on the documentation route and in the
repository checks. `tests/test_release_readiness_script.py` tests the readiness script's own logic on
temporary fixtures, without the current execution records.
"""
import importlib.util
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
MODULE_PATH = ROOT / "tools" / "release" / "check_release_readiness.py"


def load_module():
    spec = importlib.util.spec_from_file_location("check_release_readiness_documents", MODULE_PATH)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module  # its dataclasses resolve their module
    spec.loader.exec_module(module)
    return module


def test_required_release_paths_exist():
    release = load_module()
    assert release.check_required_paths(ROOT) == []


def test_latest_dag_dependency_edges_uses_approved_graph_pointer():
    release = load_module()
    assert (
        release.latest_dag_dependency_edges(ROOT)
        == Path("execution/_DAG/DAG-011/DependencyEdges.csv")
    )


def test_planned_dag_schema_step_validates_the_approved_register():
    release = load_module()
    [step] = [step for step in release.build_plan("skeleton", ROOT) if step.name == "dag dependency schema"]
    assert step.command[1:] == ("tools/validation/validate_dependencies_schema.py",
                                "execution/_DAG/DAG-011/DependencyEdges.csv")
    done = subprocess.run([sys.executable, *step.command[1:]], cwd=ROOT, capture_output=True, text=True, timeout=60)
    assert done.returncode == 0, done.stdout + done.stderr
