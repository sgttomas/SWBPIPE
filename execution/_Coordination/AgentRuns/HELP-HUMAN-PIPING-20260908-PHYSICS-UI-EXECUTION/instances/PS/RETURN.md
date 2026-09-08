# PROJECT_SETUP dependency-currency return

Run: `HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION`
Instance: `/root/dependency_currency`
Role: PROJECT_SETUP Agent 1
Source: `779dedb8670625b36af07b89fc5557470e47c50e`
Verdict: `CANDIDATE_COMPLETE_OWNER_DECISION_REQUIRED`

## Result

Revalidated Q1's exact 33 unsatisfied rows without replaying the 74 accepted rows or the full archive. All 33 require `SEMANTIC_READY`; target `IN_PROGRESS` lifecycle alone is not a maturity failure. The package distinguishes current consumed contracts, stale/chronological evidence, formal human holds, and nonactivated rows.

The selected F and U-A work activates 19 rows: four DEL-04-04 rows, eight DEL-07-01 rows, and seven DEL-07-02 rows. Current source evidence supports bounded factual checks for F E006-E008 and UI E0478-E0481. F E009 retains the PDU-035 finding and human disposition. UI E0482-E0485 are not consumed by the selected straight slice but remain formal rows. All seven inspector rows are activated by the new inline Apply behavior; their R23 target maturity passed, while edge-specific consumer integration was not established. DEL-07-02's local review is PASS and its `Review_Findings.csv` is empty, so this is missing acceptance/proof chronology rather than an identified substantive safety or correctness defect. Post-change tests and independent product-diff review can supply that proof.

The selected independent DEL-09-01 reference does not consume DEL-04-01/04-02 output or bundled expected constants and uses fixture-local SI. E0532/E0533 and the unit row remain unchanged. The three DEL-07-05 and eight DEL-13-04 rows remain unchanged because U-B and B are not selected.

PS1 independently returned factual `PASS` for E006-E008 and E0478-E0481 at the bound HEAD, with no source drift or missing producer behavior. F4's six focused command results support the three friction rows. U7's four rows have current focused test source but no frozen pre-change execution transcript; that is a run-evidence gap, not a producer-contract defect. U7's post-change focused and registered tests remain mandatory before integration acceptance. PS validated the child return. Its original pre-format SHA-256 was `9ff61400d195cc23412123dadbe71f5e4877e7838b2c04b40a93c34f3804c302`; the whitespace-normalized successor is `93e9fa72f5cf46b1d82e79f114502c3b3a76b1004f0bf3df5fd059aa5729881c`. The exact original bytes are preserved in `CORRECTIONS/V1/SERIALIZED_ORIGINALS.json`.

## Exact decision returned

Option A applies the standing all-row path. It can review current factual evidence now, but it cannot close F E009 without the existing PDU-035 independent basis and human finding disposition. The new inspector consumer proof may also require the implementation that Step 1 presently prevents.

Option B is the recommended exact execution release: an Owner-adopted, one-time Step 1 exception for twelve rows only—F E009; UI E0482-E0485; and all seven DEL-07-02 rows—while all rows, findings, DAG-010, pointer, and local registers remain unchanged. Root first verifies E006-E008 and E0478-E0481 against the frozen source/test evidence. F4 and U7 then implement only their frozen source fences. Focused tests and fresh independent product-diff review must pass before integration acceptance. The exception expires at slice closeout.

The exact non-binding ruling text is in `PHYSICS_UI_EXECUTION_20260908_CANDIDATE/PROPOSED_OWNER_ACT.md`. No current direction already grants this exception: D-66 expressly preserves the rows and conditions source writes on prerequisite disposition.

## Outputs

- `projects/chirality-piping/execution/_DAG/PHYSICS_UI_EXECUTION_20260908_CANDIDATE/README.md`
- `projects/chirality-piping/execution/_DAG/PHYSICS_UI_EXECUTION_20260908_CANDIDATE/ROW_DISPOSITIONS.csv`
- `projects/chirality-piping/execution/_DAG/PHYSICS_UI_EXECUTION_20260908_CANDIDATE/OWNER_DECISION_INTERFACE.md`
- `projects/chirality-piping/execution/_DAG/PHYSICS_UI_EXECUTION_20260908_CANDIDATE/PROPOSED_OWNER_ACT.md`
- `projects/chirality-piping/execution/_DAG/PHYSICS_UI_EXECUTION_20260908_CANDIDATE/EVIDENCE_BINDINGS.csv`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/PS/PS1/RETURN.md`
- `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260908-PHYSICS-UI-EXECUTION/instances/PS/VALIDATION.md`

No authoritative dependency, pointer, local row, lifecycle, review finding, decision register, receipt, or source file was changed by PS. N7's accepted `112/112` review and accepted parent handoff are preserved; any prose saying N7 final review remains pending is stale.
