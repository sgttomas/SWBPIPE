# W3 (rolling queue) assessment and R2 close-out (Agent 0)

**Inputs.**
- The W3 package verification reports, `WAVES/W3/PKG-NN/PKG-NN_VERIFICATION.md`, and the two rerun reports, `PKG-13_RERUN1_VERIFICATION.md` and `PKG-15_RERUN1_VERIFICATION.md`.
- The cross-package report, `CROSS_PACKAGE/W3_CROSS_PACKAGE_VERIFICATION.md`, and the cross-wave report, `../CROSS_WAVE/CORPUS_CROSS_WAVE_VERIFICATION.md`.
- The adopted resolutions: `RESOLUTIONS.csv` (425 rows) and `../CROSS_WAVE/RESOLUTIONS.csv` (5 rows).
- Agent 0's rechecks and batch runs, recorded in `RUN_STATE.jsonl`.

W3 ran as the post-gate rolling queue (`WAVE_PLAN.md`, "Later waves"): 58 deliverables across 11 packages. After the gate, a package is judged on its verifier's verdict and on the rerun rule. The 5% firm false-alignment limit was the scale-out condition, met at W2. W3 rates are reported below as quality signals, not gate conditions.

## Result: all 58 W3 deliverables accepted. R2 is complete for all 102 deliverables.

| Package | Deliverables | Verdict | Firm false alignment (sampled ALIGNED normative) |
|---|---|---|---|
| PKG-05 | 5 | ACCEPT WITH CONTESTED ROWS | 2.5% (1/40) |
| PKG-06 | 5 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/56); 1.8% counting one out-of-sample row |
| PKG-08 | 6 | ACCEPT WITH CONTESTED ROWS | 4.4% (2/45) |
| PKG-09 | 5 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/29) |
| PKG-10 | 5 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/21) |
| PKG-11 | 5 | ACCEPT WITH CONTESTED ROWS | 2.0% (1/50); 3.9% counting the out-of-sample DEL-11-03 CLM-004.r01 |
| PKG-12 | 5 | ACCEPT WITH CONTESTED ROWS | **9.5% (4/42)**; three more out-of-sample rows follow the same pattern |
| PKG-13 | 4 | RERUN DEL-13-02, then ACCEPT WITH CONTESTED ROWS | 2.7% (1/37) first pass; **5.3%** counting one confirmed out-of-sample row; rerun slice 0.0% (0/13) |
| PKG-14 | 5 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/43) |
| PKG-15 | 4 | RERUN DEL-15-02, then ACCEPT WITH CONTESTED ROWS | 3.7% (1/27) first pass; rerun slice 0.0% on the deterministic sample, but **one firm false alignment outside the sample** (DEL-15-02 CLM-020, the same defect class that caused the rerun; 9.1% counting the verifier's five targeted, deliberately biased checks) |
| PKG-17 | 9 | ACCEPT WITH CONTESTED ROWS | 0.0% (0/85) |

- **Pooled first-pass rate:** 10 of 475, or 2.1%. W2 was 1.9% and W1 4.4%.
- **PKG-12 is the outlier.** All four of its errors share one pattern: principle and guidance rows marked ALIGNED while the gap is carried on another row, or while the claim holds only because the behaviour does not exist (F1, CP-11, F7). They are recorded in `RESOLUTIONS.csv`, and the pattern goes to R3.
- **Reruns.** Two reruns ran, one cycle each, under `WAVE_PLAN.md` "Verifier reruns", and both re-verified slices were accepted. The first-run files are kept unchanged in `superseded_1/`. No deliverable went to the owner under the rerun rule. A rerun does not guarantee the defect class is gone: the DEL-15-02 rerun verifier found one more row of that class (CLM-020) outside its sample. It is recorded as FIRM in `RESOLUTIONS.csv` and does not trigger a second cycle.
- **Out-of-sample errors.** Sampled rates understate the true error rate where verifiers found confirmed errors outside the sample: PKG-06, PKG-11, PKG-12, PKG-13 and PKG-15. Every such row is in `RESOLUTIONS.csv`.

## Consistency

- **Per package.** Batch mode with `RESOLUTIONS.csv` passes with 0 findings.
- **Whole wave (58 ledgers).** Batch mode with `RESOLUTIONS.csv` passes with 0 findings. Before the cross-package verifier ran, 7 cross-package findings remained. No package verifier could see them, because each ran batch mode on its own package only. The cross-package verifier returned 3 LEGITIMATE and 4 CORRECT, recorded as 6 RESOLVED_PAIR rows and 1 FIRM row.
- **Corpus (102 ledgers).** Batch mode with the W1, W2, W3 and cross-wave resolutions combined (`../CROSS_WAVE/ALL_WAVES_RESOLUTIONS_COMBINED.csv`, 599 rows) passes with 0 findings. It had shown 5 cross-wave findings. The cross-wave verifier returned 4 LEGITIMATE and 1 CORRECT, recorded in `../CROSS_WAVE/RESOLUTIONS.csv` so that no row values in the W1 or W2 files changed. (Separately, commit 0aec23cea relabelled 30 W1/W2 rows from AGENT_READING to OWNER_CONFIRMED under Direction 8. It changed the Class column only; see Departures item 8.)
- **Post-review correction.** The independent review (R2-W3-CLOSE) found a column shift in two adopted rows, `DEL-09-04:CONTEXT` and `DEL-09-05:CONTEXT`: `NONE` was in BaselineClass and DivergenceLayers was empty. Both rows now follow the sealed ALIGNED convention their OtherCorrections cite (BaselineClass empty, DivergenceLayers `NONE`), and both batch runs still pass.
- **Duplicates.** Two keys appear twice in `RESOLUTIONS.csv`: `DEL-12-04:SOW#CLM-011.r01` (FIELD and OBSERVED) and `DEL-11-04:SOW#CLM-011/R-DEL-11-04-002` (CONTESTED and FIELD). All four rows have empty value columns, so the validator is unaffected; R3 reads both rows of each pair.
- **Common reading from both verifiers.** CP-11 and CP-03 fix at most the disposition. The CauseTag is the row's own gap cause. Tier, baseline class and layers follow C3, C5 and F8 per row. A difference from a pattern majority is therefore not an error in itself.
- **Blind spot, for R3 as a method note.** Batch mode does not compare minted `.sNN` sub-claims across ledgers, because they have no body hash. Four verifiers found splits there by hand: the architecture-basis rows in PKG-11, PKG-14 and PKG-17, and SR-1.

## Corpus-wide clusters left contested (for a single R3 resolution, with R4 routing where the owner decides)

| Cluster | What is split | Where |
|---|---|---|
| SR-1 | Cause of the "PKG-00 at SEMANTIC_READY" split (RECORD_DRIFT or SCOPE_REDIRECTED_BY_RULING). The disposition is owner-confirmed (Direction 8). | all waves |
| DEC-009 | Python or TypeScript engines where DEC-009 adopts a Rust core: cause (POSSIBLE_DEFECT, AUTHORITY_UNCLEAR or PARTIAL_SLICE), tier and routing | PKG-13, 14, 15, 17 |
| Unit vocabulary | Tier, disposition and AuthorityNeeded of the unit-vocabulary defect (`force_per_length` missing; bare-string quantities) | DEL-13-01/02, DEL-03-07, DEL-15-03 |
| F1 on CONTEXT | Whether F1 reaches CONTEXT-typed purpose rows that record an unmet element | PKG-14 |
| Export plan | Routing of rows that cite the deleted `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` (OWNER or stale pointer) | PKG-17 |
| Tier of in-scope REQs | LOCAL_DESIGN or PROJECT_BASELINE for a REQ that traces to an in-scope item | DEL-07-05, DEL-17-0x, DEL-14-02 |
| Acceptance workflow cause | DEFERRED_BY_RULING (DEC-081) or PARTIAL_SLICE | DEL-15-04, DEL-05-04 |

## Departures and decisions disclosed in `RUN_STATE.jsonl`

1. **API overload.** Sustained 529 errors forced two pauses: 15 and 30 minutes. Agents were resumed by message, which keeps their context and sealed files, not relaunched. Concurrency ran below 16 during the overload (Direction 6 exception: service overload).
2. **PKG-11 G1.** Agent 0 resumed the original worker instead of the manager's recorded rerun. That worker's sealed forwards were valid; the rerun worker had written nothing and is abandoned. No rerun cycle was consumed.
3. **PKG-11 G2 and PKG-14 G2.** Agent 0 launched these workers directly (AGENTS.md allows HELP_HUMAN to dispatch Type 2), using the managers' own sealed LAUNCH_G2.md text plus a dispatch note. Both managers kept stopping on 529s before they could launch.
4. **PKG-12 at 9.5%.** Accepted on its verdict (post-gate rule), with no mid-queue brief change. The remaining workers were already launched, and any brief change needs review first.
5. **DEL-15-02 rerun launch.** The launch message stated the first verifier's tier conclusion before the worker sealed. As a result, the AuthorityTier of the DEL-15-02 CLM-005 Units, Missing-values and Provenance rows was not independently derived. Their dispositions and other fields are the worker's own. The rerun verifier confirmed the underlying defects independently by reading the code. It had also read the worker's `LAUNCH_G1.md`, which carries the hint, so its agreement on the INVARIANT tier is not fully independent either. R3 treats that tier as contested evidence.
6. **PKG-17 WEAK ratings.** The verifier's WEAK rating of the DEL-17-04/05/06 aligned architecture-basis readings stands. Agent 0 does not upgrade a verifier's rating to trigger a rerun. One related transcription is disclosed in the part 2C return: the PKG-17 W-1c rows (DEL-17-07/08/09), which the verifier rated FIELD, were recorded as WEAK with the CP-02 values the verifier stated for the main row, so they change a disposition.
7. **Shared scratchpad.** Two workers' same-named scratch files collided in the session scratchpad. All seals were re-verified and no ledger was affected. Scratch files have since been kept inside deliverable folders.
8. **Convention change during W3 (Direction 8).** While W3 was running, the owner confirmed the SEMANTIC_READY reading. Commit 0aec23cea then added the Direction 8 clarification to CONVENTIONS F3 and relabelled the 30 W1/W2 SR rows as OWNER_CONFIRMED, changing the Class column only. W3 workers launched before the change applied F3's origin test literally to those sub-claims; the resolutions record the owner-confirmed disposition for every W3 SEMANTIC_READY sub-claim (OWNER_CONFIRMED rows). Both are disclosed in `RUN_STATE.jsonl`.

## Owner items added in W3 (for the R4 packet, alongside the W2 list)

- **Product and code:**
  - The DEL-15-03 exporter redacts fields that the SOW requires it to preserve (PR #307).
  - The DEL-15-02 mapping builder fills in silent defaults, including one that skips the unit check.
  - The "JCS-compatible" hash labels are not RFC 8785 (PKG-14, PKG-17).
  - Analysis-run records always write empty rule-pack and library references.
  - PCF writes 0 for a missing coordinate.
  - The desktop panel treats any section unit other than m as millimetres (worker-raised; not confirmed by a verifier).
  - `unit_system_disclosure.json` is missing from PCF packages.
  - The MBF loss-report diagnostic can never fire.
  - The native JSON token is `TBD`, where the contract uses `tbd`.
  - The desktop Knowledge panel reads an invented preview file that is not in schema shape and is not validated against the schema (PKG-13; INVARIANT, OWNER).
  - The desktop shows constraint-validation status that no engine produces, and nothing owns it (PKG-13).
  - No product path emits schema-compliant handoff packages, and the desktop prover packet adds a field its strict schema forbids (PKG-15).
  - GUI panels contradict the SOW GUI exclusions (DEL-17-04, DEL-17-05).
- **Authority:**
  - The DEC-009 Python engines.
  - Values that code settled with no ruling: the desktop store root, the unmatched-classification list, and REXC-CON-002 (worker-raised; not confirmed by a verifier).
  - DEC-051 provider disclosure.
  - The DEL-11-01 user-guide exception scope.
  - The DEL-11-04 professional-boundary list against the DEC-081/DEC-107 prohibition-list instruction.
  - CF-001 and CF-002, pending an owner ruling (PKG-12).
  - The PDU-031 hold and the frozen-contract rename `openpipestress_jcs_ijson_v1` (PKG-17).
- **Evidence and IP:**
  - Six protected-content review records were not located (PKG-14).
  - The PCF fixture's dimensions match a published table, and the PCF and glTF fixtures carry no provenance.
  - Engineering validation holds: DEL-14-04/05, DEL-17-05 and DEL-13-04.
- **Method:**
  - The rerun-rule reading, raised by PKG-08 and PKG-15.
  - Whether a post-gate package well over 5% (PKG-12 at 9.5%, with three more out-of-sample rows) should go to the owner rather than being accepted on its verdict alone. WAVE_PLAN sets no post-gate rate condition.
  - The `.sNN` batch blind spot.
  - The rerun-launch hint (item 5 above).
  - No DivergenceLayers value fits a pure product gap: the claimed code exists but no product path calls it, and workers used RECORD (PKG-15).

## Next

1. Independent review of this assessment, the adopted W3 and cross-wave resolutions, and the departures above.
2. R2 close-out, then R3 synthesis.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
