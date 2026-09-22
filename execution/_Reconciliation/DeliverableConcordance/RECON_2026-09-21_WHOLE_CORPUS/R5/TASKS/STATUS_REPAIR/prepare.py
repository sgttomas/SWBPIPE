"""Prepare exact, bounded status repairs; refuses to replace an existing manifest."""
from pathlib import Path
import csv, hashlib, json, re

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / "agents/AGENT_TASK.md").exists())
PROJECT = ROOT / "projects/chirality-piping"
RUN = HERE.parents[2]
BASE = "379df923927d157be3ebb51d8a1dcf783d970112"
DATE = "2026-09-22"
def sha(s): return hashlib.sha256(s.encode()).hexdigest()
def path_for(d):
    paths = list((PROJECT / "execution").glob(f"PKG-*/*/{d}_*/_STATUS.md"))
    assert len(paths) == 1, (d, paths)
    return paths[0]

N7 = "execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-UI-TOOLKIT-PARITY"
D72 = "Performance acceptance, settled-frame and owned-resource obligations remain open on the redesigned product under [D-72](../../../_Coordination/_DECISIONS/D-72_RULING_2026-09-18.md) and its [final addendum](../../../_Coordination/_DECISIONS/D-72_RULING_ADDENDUM_2026-09-18.md). All six criteria are ruled and frozen: item 5 uses S-1/S-2, with S-3 not accepted. The addendum removes the fresh former-interface baseline cohort; the original D-70 failures and successor demonstration retain their distinct attribution. Independent-usability holds remain. Qualification requires the ruled candidate evidence; no performance acceptance or lifecycle promotion follows from the criteria ruling."
PR789 = "No open pre-merge action remains for PR #789: it merged on 2026-09-17 at `8468a33c`. The [final ROOT acceptance record](../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/FINAL_ACCEPTANCE.json) records bounded functional technical acceptance and its source/verification bindings. Its resource evidence retains the recorded applicability limits; no final-binary resource cohort is claimed. Performance qualification, independent usability/security validation and broader functional residuals remain with their owning records."
INTAKE = f"Historical review evidence is available at `{N7}/instances/N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md` (PASS) and `{N7}/PRECOMMIT_PARENT_FAN_IN_V1.md` (bounded parent fan-in). The formerly cited `HISTORICAL_N7_ACCEPTANCE_INTAKE_V1.json` is unavailable in the inspected project tree; existence elsewhere and its asserted v11 revalidation remain unknown. Locate the intake record or obtain a bounded replacement verification before relying on that revalidation. Historical review snapshots, coverage cells and lifecycle state are unchanged; these records do not accept a later UI candidate."

replacements = {
"DEL-01-03/R01": "External contribution intake remains closed under DEC-027 and the ruled D-07b/DEC-079. Opening it requires a future owner act recorded in a new decision-register row, with legal-instrument adoption on legal advice; the D-07b packet §§5–6 supplies the adopted review/screening skeleton. No intake activation or legal-instrument choice is made here.",
"DEL-05-02/R01": "No DEC-092 product-implementation or Receipt-87 closeout action is identified by this item. The bounded implementation evidence is `_run_records/WORKING_ITEMS_RUN_2026-08-02_DEC092_TEMPERATURE_G_IMPLEMENTATION.md`; `loop/LOOP_RECEIPTS.md` Receipt 87 records the closeout and commit-bound `validation/evidence/sweeps/SWEEP_20260803T194132Z_c394365ca72b.json`. The separate DEL-09-04 derivative regeneration below remains deferred. No suite was rerun for this record repair.",
"DEL-05-04/R01": "Resolve the held stale human-acceptance reuse negative (FG-DEL-05-04-01; ScopeOfWork.md REQ-05-04-008/014 and CLM-008/023) against an authorized implementation and verification basis, or obtain a scope disposition. The landed operation-applier stale-model-hash check does not establish invalidation of external human-acceptance records; external acceptance ownership/storage/presentation remain held. This record does not declare warranted-empty Remaining.",
"DEL-07-01/R03": D72,
"DEL-07-02/R02": D72,
"DEL-07-06/R02": D72,
"DEL-07-09/R02": D72,
"DEL-07-01/R04": PR789,
"DEL-07-02/R03": PR789,
"DEL-07-06/R03": PR789,
"DEL-07-09/R03": PR789,
"DEL-07-01/R06": "Complete broader viewport/editor UX beyond the landed component creation, persistent workspace, bounded route capture, D-68 selection/visibility/measurement foundation and SWBPIPE canvas slices. Evidence references: `apps/desktop/e2e/ui-foundation.spec.ts`, `apps/desktop/e2e/c3-viewport-visibility.spec.ts` and the bounded ROOT acceptance record above. Vocabulary row 15 component-symbol authoring is complete; the specific richer-routing, transform and finite-component residuals below remain open. Work selection follows the owner-steered work graph (CONVENTIONS C9); no lifecycle promotion is inferred.",
"DEL-07-02/R08": "Complete broader canvas/model-tree/property-editor UX beyond the landed D-68 selection/virtualized-workspace foundation and table-editing slices. Evidence references: `apps/desktop/e2e/ui-foundation.spec.ts`, `apps/desktop/src/features/model-tree/ModelTree.test.tsx` and `apps/desktop/src/features/workspace/table/ModelTree.table.test.tsx`. PRD FR-003/FR-013/FR-014 residuals still need bounded work-graph assignments and claim-specific acceptance evidence; the landed slices do not establish full UX closure.",
"DEL-07-02/R11": INTAKE,
"DEL-07-09/R09": INTAKE,
"DEL-07-03/R03": f"No N7 rereview action remains for the reviewed SCA-009 material/section/load-case/hanger/self-weight slice: `{N7}/instances/N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md` records PASS and `{N7}/PRECOMMIT_PARENT_FAN_IN_V1.md` records bounded parent fan-in. R-005/R-006 and historical PDU-041 attribution remain with those records. Browser library import remains unavailable; hanger selection remains native-only/manual without sizing. Independent usability/security validation and broader scope remain open; no lifecycle closure is inferred.",
"DEL-07-03/R04": f"No final N7 rereview remains pending for the shared-support repairs: `{N7}/instances/N7_FINAL_REVIEW/V2_BACKCHECK/REVIEW_RETURN_V2.md` records PASS on its exact V2 source. The accepted family-token, duplicate-stiffness and provenance requirements retain that evidence binding. Broader mixed-payload compatibility remains unresolved; native-only manual hanger selection and no-sizing limits remain. Historical test counts stay in the review evidence; no suite was rerun here and no lifecycle closure follows.",
"DEL-07-05/R02": "Preserve ratio producer/rule-pack sufficiency as upstream responsibilities and never synthesize missing criteria. The current result-semantic contract provides no eligible governing-ratio family, so the viewer reports governing ratio unavailable rather than treating a supplied unrecognized row as an eligible ratio. Evidence: `fixtures/results/semantic_contract_v0_2.json`, `apps/desktop/src/features/results/ResultsPanel.test.tsx` and `apps/desktop/src/App.test.tsx` (PR #787 result-integrity basis). A future eligible producer/semantic basis remains separately governed.",
"DEL-08-06/R01": "Complete remaining producer bindings outside the landed desktop state/run and comparison report-package seam, including solver/rule-check, handoff/adapter, rendered-report/export and external or non-JSON payload partitioning work. The bounded state/run and comparison binding has implementation evidence in `apps/desktop/src/features/report/reportPackageRequest.ts`, `stateComparisonHandoffSections.ts` beside it, and `core/reporting/report_package/src/lib.rs`. It does not close the other producer or partitioning residuals.",
"DEL-09-01/R02": "Complete RQ-004 project-grain unit-system binding and rerun the mechanics benchmarks against that basis. DEC-018 accepted the canonical unit catalog and conversion constants on 2026-06-10; the current fixture-local evidence alone does not establish project-grain acceptance. Preserve upstream DEL-02-02 binding holds and existing tolerance/validation authority. Evidence: `core/units/README.md`, `validation/benchmarks/mechanics/src/lib.rs::FIXTURE_UNIT_BASIS` and `_run_records/WORKING_ITEMS_RUN_2026-07-12_D41-R5-T2B-PDU013.md`.",
"DEL-09-02/R01": "Keep final stress tolerance, release-threshold, publication and professional-reliance choices open. DEC-018 already rules the canonical conversion basis; it does not establish benchmark binding or acceptance. DEC-025 rules the commit-bound evidence-sweep merge-gate posture, with later surface-specific amendments in the decision register; it does not settle stress release thresholds, release authority or publication. PDU-039's result-envelope witness and `validation/benchmarks/stress/src/lib.rs::STRESS_BENCHMARK_READINESS_BOUNDARY` remain verification evidence only.",
"DEL-09-04/R01": "Complete owner/maintainer review and MAINTAINER_REVIEWED promotion of the validation-manual case pages. Runner benchmark/regression and export-results report-package payload bindings are landed evidence; export-results is not a remaining runner stub (see `core/runner/headless/src/bin/openpipestress-runner.rs` and `validation/witness/generated/del1005_export_results_*.json`). DEC-080 establishes the evidence-bundle home `validation/evidence/reproduction/<run-id>/`; binding witnesses do not promote case-page review or engineering validation.",
"DEL-09-05/R03": "No issuance-wave packet preparation is presently actionable under this item: D-40/DEC-072 returned the only CHECKING deliverables to IN_PROGRESS, and the reconciliation census has no CHECKING candidate. Any future candidate requires its own declared checking basis, evidence and owner lifecycle act; DEC-062's issuance evidence bar remains applicable. DEL-01-01 remains the untouched ISSUED baseline.",
"DEL-10-02/R03": "Close the FR-023 GUI import/export round-trip residual through the v0.2 R6 handoff work and its owning deliverables (including DEL-17-03..08). D-12/DEC-078 dispositioned FR-024 and FR-025 only; it neither closes FR-023 nor remains an outstanding ruling gate for it. Scope and acceptance stay with the authorized handoff work graph.",
"DEL-10-03/R01": "Implement the distinct FR-025 local FEA submodel export post-beta in the Phase H (v0.2 R6) lane, as ruled by D-12/DEC-078, with DEL-10-03 as contract authority and implementing scope established by the Phase H lead-up decomposition. FR-025 is distinct from FR-HAND-*; a future re-deferral requires a new owner decision. This ruling settles the disposition, not implementation or external FEA execution.",
"DEL-10-04/R02": "Carry the DEC-089 policy-only future Apple Developer ID signing/notarization target through its separately applicable gates. D-06b is already ruled; DEC-057's current unsigned posture continues with checksum, commit-bound sweep, release record and unsigned-install caveat until signing/notarization requirements are satisfied and accepted. The registered App ID (subsequently amended by DEC-106) is not evidence of signing/notarization or a release. Historical v0.1 PRD §22.6 remains an R6-entry release-machinery residual under D-21/DEC-056; no release action is authorized here.",
"DEL-11-02/R01": "Defer contributor legal-instrument adoption to a future owner intake-activation decision taken on legal advice, as ruled by DEC-079. Intake remains closed under DEC-027; the adopted D-07b review/screening skeleton defines the process without selecting CLA/DCO/equivalent. The project-license selection does not settle contributor attestations or agreement workflow; no current intake activation is authorized.",
"DEL-12-01/R01": "Complete LFSP-REQ-011 runtime private-path resolution and obtain a bounded review mapping its required test families to the existing storage round-trip/migration and report/export evidence. Candidate evidence exists at `apps/desktop/src-tauri/src/lib.rs::saved_edited_load_model_round_trips_and_solves_from_restored_payload`, `::store_migration_ledger_reconciles_legacy_store_and_preserves_rows`, and `apps/desktop/src/services/reportPackageSaveService.test.ts` (missing/blocked local-first evidence rejection). This corrects the claim that all storage families are absent; it does not establish that these tests satisfy LFSP-REQ-011. PDU-036's adjacent trace-gap fixture remains no substitute, and RF-001/RF-002 below remain human-held.",
"DEL-13-02/R01": "Obtain the owning human disposition for `PKG13-DEL-13-02-PKG02-001`; the review record remains `TECHNICALLY_ADDRESSED_PENDING_HUMAN` with `HumanDisposition=TBD`. Reassess its technical premise: `schemas/units.schema.yaml` includes `force_per_length`, while the constraint dimension enum in `schemas/constraint.schema.json` does not. Current vocabulary evidence therefore does not establish enum parity. No reviewer finding or human gate is closed by this factual correction (PDU-060).",
}

assert not (HERE / "operations.json").exists(), "Manifest exists; use apply_check.py"
rows = [r for r in csv.DictReader((RUN / "R3/TASKS/T9_LIFECYCLE.csv").open()) if "STATUS#remaining" in r["Item"]]
assert len(rows) == 29 and len(replacements) == 29
date_rows = [r for r in csv.DictReader((RUN / "R3/TASKS/T4B_CLASSES.csv").open()) if r["ClassID"] == "T4B-C06" and r["ClaimKey"].endswith(":STATUS") and r["DeliverableID"] != "DEL-01-01"]
ids = sorted({r["DeliverableID"] for r in rows + date_rows} | {"DEL-12-01"})
files, operations = [], []
for d in ids:
    path = path_for(d); before = path.read_text()
    assert "**Current State:** ISSUED" not in before
    edits = []
    def add(start, end, new, key, reason, evidence, kind):
        old = before[start:end]
        assert old != new
        edits.append(dict(source_key=key, path=str(path.relative_to(ROOT)), source_start=start, source_end=end,
            before_start_line=before.count("\n",0,start)+1, before_end_line=before.count("\n",0,end)+1,
            before_text=old, after_text=new, before_body_sha256=sha(old), after_body_sha256=sha(new),
            rationale=reason, authority="Current user direction / Agent0 R5 record-repair brief; Rev2 §3.1; " + evidence,
            execution="b" if kind in ("remaining", "declaration") else "RECORD_METADATA", operation_kind=kind))
    rem = re.search(r"(?ms)^## Remaining\n(.*?)(?=^## |\Z)", before)
    starts = list(re.finditer(r"(?m)^- ", rem[1]))
    for row in (r for r in rows if r["DeliverableID"] == d):
        idx = int(row["Item"].rsplit("R",1)[1]) - 1
        a=rem.start(1)+starts[idx].start(); b=rem.start(1)+(starts[idx+1].start() if idx+1<len(starts) else len(rem[1]))
        b=a+len(before[a:b].rstrip())
        tag=d+"/R"+str(idx+1).zfill(2)
        evidence=row["Evidence"]
        if tag in ("DEL-07-01/R03","DEL-07-02/R02","DEL-07-06/R02","DEL-07-09/R02"):
            evidence += "; D-72_RULING_ADDENDUM_2026-09-18.md supersedes incomplete R3 premise that item 5 is unruled"
        add(a,b,"- "+replacements[tag],row["Item"],"Correct the stated record; preserve current residuals and exact ruling reach. " + row["Finding"].split(" RemainingWork:")[0],evidence,"remaining")
    if d == "DEL-12-01":
        marker="## D-41 R5 T7 PDU-055 current declaration\n"
        pos=before.index(marker)+len(marker)
        add(pos,pos,"\n> Historical declaration, superseded as current on 2026-09-22 by the authorized R5 record repair under concordance method Rev2 §3.1. The following D-41 text is retained verbatim as history, including its former revision/DAG pins. Current authority comes from the accepted decomposition and decision register; this old blanket declaration does not supersede sibling claims. Lifecycle and closure fences in the preserved text continue to apply.\n",d+":STATUS#d-41-r5-t7-pdu-055-current-declaration","A9 lift: retain prior bytes as history and remove current force of stale blanket declaration.","D-GOV-44 / method Rev2 §3.1; CONVENTIONS C1 history rule","declaration")
    m=re.search(r"(?m)^\*\*Last Updated:\*\* [^\n]+",before)
    assert m
    add(m.start(),m.end(),"**Last Updated:** "+DATE,d+":STATUS","Date of this actual amendment; no lifecycle transition.","T4B-C06 for listed stale dates; otherwise truthful amendment metadata","date")
    hist = re.search(r"(?ms)^## History\n(.*?)(?=^## |\Z)",before)
    assert hist
    pos=hist.start(1)+len(hist[1].rstrip())
    text="\n- 2026-09-22 - R5 concordance record repair applied under current owner direction and Agent 0's bounded brief; corrected declared-state/Remaining facts or amendment metadata against the recorded basis. Exact before/after operations and evidence are in `execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/R5/TASKS/STATUS_REPAIR/operations.json`. Lifecycle state, human/reviewer holds and release/engineering-acceptance boundaries are unchanged."
    add(pos,pos,text,d+":STATUS#history","Append truthful record of current repair without rewriting historical entries.","Current R5 task brief","history")
    edits.sort(key=lambda x:(x["source_start"],x["source_end"]))
    assert all(a["source_end"] <= b["source_start"] for a,b in zip(edits,edits[1:]))
    after=before
    for e in reversed(edits): after=after[:e["source_start"]]+e["after_text"]+after[e["source_end"]:]
    state=re.search(r"(?m)^\*\*Current State:\*\* .+$",before)[0]
    assert re.search(r"(?m)^\*\*Current State:\*\* .+$",after)[0]==state
    snapshot=HERE/"before"/(d+".md"); snapshot.parent.mkdir(exist_ok=True);snapshot.write_text(before)
    files.append(dict(deliverable_id=d,path=str(path.relative_to(ROOT)),before_file_sha256=sha(before),after_file_sha256=sha(after),lifecycle=state,before_snapshot=str(snapshot.relative_to(HERE))))
    operations.extend(edits)
for i,e in enumerate(operations,1):e["operation_id"]="STATUS-"+str(i).zfill(3)
manifest=dict(task="STATUS_REPAIR",source_base_commit=BASE,amendment_date=DATE,files=files,operations=operations)
(HERE/"operations.json").write_text(json.dumps(manifest,indent=2,ensure_ascii=False)+"\n")
with (HERE/"operations.csv").open("w",newline="") as f:
    w=csv.DictWriter(f,fieldnames=list(operations[0]));w.writeheader();w.writerows(operations)
print(json.dumps({"files":len(files),"remaining":len(rows),"stale_date_rows":len(date_rows),"operations":len(operations)},indent=2))
