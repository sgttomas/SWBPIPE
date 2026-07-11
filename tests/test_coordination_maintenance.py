from __future__ import annotations

import importlib.util
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "coordination" / "list_deliverable_status.py"


def load_module():
    spec = importlib.util.spec_from_file_location("list_deliverable_status", MODULE_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def write_status(
    root: Path, package: str, folder: str, state: str, remaining: list[str] | None = None
) -> Path:
    deliverable_dir = root / "execution" / package / "1_Working" / folder
    deliverable_dir.mkdir(parents=True)
    status_path = deliverable_dir / "_STATUS.md"
    deliverable_id = folder.split("_", maxsplit=1)[0]
    remaining_lines = (
        ["", "## Remaining", *[f"- {item}" for item in remaining]] if remaining else []
    )
    status_path.write_text(
        "\n".join(
            [
                f"# Status: {deliverable_id}",
                "",
                f"**Current State:** {state}",
                "**Last Updated:** 2026-06-03",
                *remaining_lines,
                "",
                "## History",
                f"- 2026-06-03 - Fixture state set to {state}.",
            ]
        )
        + "\n",
        encoding="utf-8",
    )
    return status_path


def write_fixture_repo(root: Path) -> None:
    (root / "execution/_DAG/DAG-006").mkdir(parents=True)
    (root / "execution/_DAG/_LATEST.md").write_text(
        "- Approved graph authority: `execution/_DAG/DAG-006/`\n",
        encoding="utf-8",
    )
    (root / "execution/_DAG/DAG-006/DeliverableNodes.csv").write_text(
        "\n".join(
            [
                "NodeID,PackageID,DeliverableID,DeliverableName,DeliverableType,ScopeItems,Objectives,ContextEnvelope,ExecutionPath,ContextPath,DependenciesPath,HasFourDocumentKit,HasSemanticMatrix,HasSemanticLensing,HasReview,SourceRegister,Notes",
                "DEL-00-01,PKG-00,DEL-00-01,Architecture baseline,DOC,SOW,OBJ,M,execution/PKG-00_Runway/1_Working/DEL-00-01_Architecture baseline,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
                "DEL-01-01,PKG-01,DEL-01-01,Governance,DOC,SOW,OBJ,M,execution/PKG-01_Governance/1_Working/DEL-01-01_Governance,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
                "DEL-02-01,PKG-02,DEL-02-01,Domain schema,DOC,SOW,OBJ,M,execution/PKG-02_Domain/1_Working/DEL-02-01_Domain schema,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
            ]
        )
        + "\n",
        encoding="utf-8",
    )
    write_status(root, "PKG-00_Runway", "DEL-00-01_Architecture baseline", "SEMANTIC_READY")
    write_status(
        root,
        "PKG-01_Governance",
        "DEL-01-01_Governance",
        "IN_PROGRESS",
        remaining=["Open item one (gated: D-99)", "Open item two"],
    )
    write_status(root, "PKG-02_Domain", "DEL-02-01_Domain schema", "CHECKING")


def test_discovers_local_status_and_dag_context(tmp_path):
    tool = load_module()
    write_fixture_repo(tmp_path)

    discovered = tool.discover_statuses(tmp_path)
    output_rows = tool.rows(tmp_path, "DAG-006", discovered)

    by_id = {row["DeliverableID"]: row for row in output_rows}
    assert set(by_id) == {"DEL-00-01", "DEL-01-01", "DEL-02-01"}
    assert by_id["DEL-01-01"]["LocalStatus"] == "IN_PROGRESS"
    assert by_id["DEL-02-01"]["DAGNodePresent"] == "TRUE"
    assert by_id["DEL-00-01"]["StatusVocabulary"] == "NONSTANDARD_TOLERATED"
    assert by_id["DEL-01-01"]["RemainingItems"] == "2"
    assert by_id["DEL-02-01"]["RemainingItems"] == "0"


def test_filters_statuses_without_hiding_inventory_by_default(tmp_path):
    tool = load_module()
    write_fixture_repo(tmp_path)

    output_rows = tool.rows(tmp_path, "DAG-006", tool.discover_statuses(tmp_path))
    filtered = tool.filter_rows(output_rows, {"IN_PROGRESS"}, exclude_issued=False)

    assert len(output_rows) == 3
    assert len(filtered) == 1
    assert filtered[0]["DeliverableID"] == "DEL-01-01"


def test_live_status_discovery_command_passes():
    completed = subprocess.run(
        [
            sys.executable,
            "tools/coordination/list_deliverable_status.py",
            "--dag",
            "DAG-006",
            "--format",
            "csv",
            "--summary",
        ],
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )

    assert completed.returncode == 0, completed.stderr
    assert "DeliverableID,PackageID,DeliverableName,LocalStatus" in completed.stdout
    assert "Rows:" in completed.stderr
