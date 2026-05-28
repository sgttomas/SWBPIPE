from __future__ import annotations

import importlib.util
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
MODULE_PATH = ROOT / "tools" / "coordination" / "maintain_dev001_coordination.py"


def load_module():
    spec = importlib.util.spec_from_file_location("maintain_dev001_coordination", MODULE_PATH)
    assert spec is not None
    assert spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


def write_fixture_repo(root: Path) -> None:
    (root / "execution/_Coordination").mkdir(parents=True)
    (root / "execution/_DAG/DAG-005").mkdir(parents=True)
    (root / "execution/_DAG/_LATEST.md").write_text(
        "- Approved graph authority: `execution/_DAG/DAG-005/`\n",
        encoding="utf-8",
    )
    (root / "execution/_DAG/DAG-005/DeliverableNodes.csv").write_text(
        "\n".join(
            [
                "NodeID,PackageID,DeliverableID,DeliverableName,DeliverableType,ScopeItems,Objectives,ContextEnvelope,LifecycleState,ExecutionPath,ContextPath,DependenciesPath,HasFourDocumentKit,HasSemanticMatrix,HasSemanticLensing,HasReview,SourceRegister,Notes",
                "DEL-00-01,PKG-00,DEL-00-01,Architecture baseline,DOC,SOW,OBJ,M,SEMANTIC_READY,path,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
                "DEL-01-01,PKG-01,DEL-01-01,Governance,DOC,SOW,OBJ,M,IN_PROGRESS,path,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
                "DEL-02-01,PKG-02,DEL-02-01,Domain schema,DOC,SOW,OBJ,M,IN_PROGRESS,path,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
                "DEL-03-01,PKG-03,DEL-03-01,Consumer,DOC,SOW,OBJ,M,OPEN,path,context,deps,TRUE,TRUE,TRUE,FALSE,register,",
            ]
        )
        + "\n",
        encoding="utf-8",
    )
    (root / "execution/_DAG/DAG-005/DependencyEdges.csv").write_text(
        "\n".join(
            [
                "RegisterSchemaVersion,DependencyID,FromPackageID,FromDeliverableID,FromDeliverableName,DependencyClass,AnchorType,Direction,DependencyType,TargetType,TargetPackageID,TargetDeliverableID,TargetRefID,TargetName,TargetLocation,Statement,EvidenceFile,SourceRef,EvidenceQuote,Explicitness,RequiredMaturity,ProposedMaturity,SatisfactionStatus,Confidence,Origin,FirstSeen,LastSeen,Status,Notes",
                "v3.1,E001,PKG-01,DEL-01-01,Governance,EXECUTION,NOT_APPLICABLE,UPSTREAM,PREREQUISITE,DELIVERABLE,PKG-00,DEL-00-01,DEL-00-01,Architecture,path,statement,file,ref,quote,EXPLICIT,SEMANTIC_READY,SEMANTIC_READY,SATISFIED,HIGH,EXTRACTED,2026-01-01,2026-01-01,ACTIVE,",
                "v3.1,E002,PKG-03,DEL-03-01,Consumer,EXECUTION,NOT_APPLICABLE,UPSTREAM,PREREQUISITE,DELIVERABLE,PKG-02,DEL-02-01,DEL-02-01,Domain,path,statement,file,ref,quote,EXPLICIT,COMMITTED,COMMITTED,SATISFIED,HIGH,EXTRACTED,2026-01-01,2026-01-01,ACTIVE,",
                "v3.1,E003,PKG-03,DEL-03-01,Consumer,EXECUTION,NOT_APPLICABLE,UPSTREAM,PREREQUISITE,DELIVERABLE,PKG-99,DEL-99-01,DEL-99-01,Candidate,path,statement,file,ref,quote,EXPLICIT,COMMITTED,TBD,TBD,HIGH,EXTRACTED,2026-01-01,2026-01-01,CANDIDATE,",
            ]
        )
        + "\n",
        encoding="utf-8",
    )


def test_generates_queue_with_architecture_and_candidate_semantics(tmp_path):
    tool = load_module()
    write_fixture_repo(tmp_path)
    (tmp_path / "execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv").write_text(
        "\n".join(
            [
                "DeliverableID,PackageID,EvidenceState,EvidenceKind,Commit,CommitSubject,CommittedDate,HandoffCommit,Notes",
                "DEL-01-01,PKG-01,COMMITTED,BOUNDED_ITEM_COMMIT,,,,,fixture evidence",
                "DEL-02-01,PKG-02,COMMITTED,BOUNDED_ITEM_COMMIT,,,,,fixture evidence",
            ]
        )
        + "\n",
        encoding="utf-8",
    )

    generated, findings = tool.generate(tmp_path, "DAG-005", "2026-05-28")

    assert findings == []
    by_id = {row["DeliverableID"]: row for row in generated.rows}
    assert by_id["DEL-01-01"]["BlockerState"] == "UNBLOCKED"
    assert by_id["DEL-03-01"]["ActiveUpstreamCount"] == "1"
    assert by_id["DEL-03-01"]["BlockerState"] == "UNBLOCKED"
    assert generated.active_edge_count == 2
    assert generated.candidate_edge_count == 1
    assert generated.architecture_basis_edge_count == 1
    assert "| PKG-00 architecture-basis edges satisfied | 1 |" in generated.md_text


def test_duplicate_evidence_fails_validation(tmp_path):
    tool = load_module()
    write_fixture_repo(tmp_path)
    header = ",".join(tool.EVIDENCE_HEADER)
    rows = [
        "DEL-02-01,PKG-02,COMMITTED,BOUNDED_ITEM_COMMIT,,,,,one",
        "DEL-02-01,PKG-02,COMMITTED,BOUNDED_ITEM_COMMIT,,,,,two",
    ]

    findings = tool.validate_evidence(
        root=tmp_path,
        evidence_header=tool.EVIDENCE_HEADER,
        evidence_rows=[dict(zip(tool.EVIDENCE_HEADER, row.split(","))) for row in rows],
        node_ids={"DEL-02-01"},
        validate_git=False,
    )

    assert "duplicate evidence DeliverableID: DEL-02-01" in findings
    assert header.startswith("DeliverableID")


def test_check_detects_stale_outputs_and_write_repairs_them(tmp_path):
    tool = load_module()
    write_fixture_repo(tmp_path)
    (tmp_path / "execution/_Coordination/DEV-001_IMPLEMENTATION_EVIDENCE.csv").write_text(
        "\n".join(
            [
                "DeliverableID,PackageID,EvidenceState,EvidenceKind,Commit,CommitSubject,CommittedDate,HandoffCommit,Notes",
                "DEL-01-01,PKG-01,COMMITTED,BOUNDED_ITEM_COMMIT,,,,,fixture evidence",
                "DEL-02-01,PKG-02,COMMITTED,BOUNDED_ITEM_COMMIT,,,,,fixture evidence",
            ]
        )
        + "\n",
        encoding="utf-8",
    )
    for rel in [
        "execution/_Coordination/DEV-001_BLOCKER_QUEUE.csv",
        "execution/_Coordination/DEV-001_BLOCKER_QUEUE.md",
        "execution/_DAG/DAG-005/DEV-001_BLOCKER_QUEUE.csv",
        "execution/_DAG/DAG-005/DEV-001_BLOCKER_QUEUE.md",
    ]:
        (tmp_path / rel).write_text("stale\n", encoding="utf-8")

    generated, findings = tool.generate(tmp_path, "DAG-005", "2026-05-28")
    assert findings == []
    assert len(tool.compare_outputs(tmp_path, "DAG-005", generated)) == 4

    tool.write_outputs(tmp_path, "DAG-005", generated)
    assert tool.compare_outputs(tmp_path, "DAG-005", generated) == []


def test_live_coordination_check_passes():
    completed = subprocess.run(
        [
            sys.executable,
            "tools/coordination/maintain_dev001_coordination.py",
            "--dag",
            "DAG-005",
            "--check",
        ],
        cwd=ROOT,
        check=False,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )

    assert completed.returncode == 0, completed.stderr
