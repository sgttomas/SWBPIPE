VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-17 verification — wave W3 (fresh, evidence-only verifier)

- Run: `HELP-HUMAN-PIPING-20260921-RECONCILIATION`. Parent: HELP_HUMAN Agent 0.
- Brief: `briefs/R2-VERIFIER_brief.md`, SHA-256 `47fb3c52…5b00` (matched before starting).
- Sampling: STANDARD. Evidence checkout: `00115c71931bcae79909602d653740d3bb72dfa1` (read-only).
- Rules applied: `CONVENTIONS.md` (Parts A–F, including F1–F8), `CANONICAL_SITUATIONS.md` and `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`.
- I read the manager's validator transcripts and the workers' returns only as pointers. None of it is used here as evidence about a claim.

## 0. Integrity and mechanical checks

- **Seals.** I recomputed each forward ledger's SHA-256. All nine equal their `SEAL.txt` hashes and the hashes the manager reported.
- **Single-mode validation.** I ran `validate_ledger_v2.py` on each of the nine ledgers with `--reverse`, `--inventory ROUTING/PKG-17_capabilities.csv` and `--notes-gap`. All nine PASS with 0 findings (96, 160, 72, 97, 129, 123, 102, 110 and 91 rows).
- **Batch mode.** I ran `--batch` over the nine forward ledgers. It returns FAIL with 9 consistency findings, the same 9 the manager recorded. They are resolved in §5.
- **Keyed canonical rows.** I checked 45 of 45 rows (CS-01 to CS-07) field by field against `CANONICAL_ASSIGNMENTS.csv`. All 45 conform.
- **Test names.** Every `tests/…::name` token in the evidence columns exists in the frozen tree: 215 of 215.
- **Gate record.** `GATE:GATE_EVIDENCE/B4_4_SWEEP_9D55/SUMMARY.json` exists and records a PASS on five surfaces. No suite was rerun (A6).
- **No ISSUED deliverables.** All nine are IN_PROGRESS. No row carries `PROTECTED_CHECK`. Two rows carry `FROZEN_CONTRACT`, both in DEL-17-06.

## 1. Sample selection

I assigned each forward row to the first class that fits, in the order below. Within each (deliverable, class) cell, I sorted candidates by the SHA-256 of the `ClaimKey` and took the lowest `ceil(rate × n)`.

| Class | Definition | Rate |
|---|---|---|
| A100 | `INVARIANT` tier; `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`, `UNKNOWN` or `LIFECYCLE_REASSESSMENT_REQUIRED`; `PROTECTED_CHECK` or `FROZEN_CONTRACT` | 100% |
| SH | The unit has `SharedTextCount > 1` in `CLAIM_KEYS_V2.csv` (a `.sNN` or `.rNN` row uses its base key) | 100% |
| GC | `ALIGNED` with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:` | 100% |
| PC | `ALIGNED` with `PRODUCT_CALLER: NONE` | 25% |
| NA | Other non-aligned rows | 25% |
| AN | `ALIGNED` normative rows (`REQUIREMENT`, `ACCEPTANCE`, `EXCLUSION`) | 20% |
| ST | Structural rows, inherited canonical rows, and other quiet rows | 10% |

- **Weighting for AN.** Half of each cell's quota, rounded up, comes from the weighted pool, taken in SHA order. The weighted pool is rows with `LOW` or `MEDIUM` confidence, `NONE_FOUND` verification or verification class `NONE`. The rest of the quota comes from the unweighted pool. Where one pool runs short, the other fills the gap.
- **Size.** 298 of 980 forward rows were sampled.
- **Reverse sample.**
  - 100% of the non-`NOT_MINE` answers: 45 `COVERS`, 27 `CLAIMED_BY`, 2 `PARTIAL`, and no `UNKEYED` or `CONSTRAINS`.
  - 10% of `NOT_MINE` answers (281 of 2,805), keyed `DEL|RC-id`, in SHA order.
  - For F5, 20% of the 33 `NOT_MINE` answers whose EntryPoints overlap a path the deliverable's own forward ledger cites (7 rows).

## 2. Per-deliverable results

The last column is the firm false-alignment rate. Its denominator is the sampled `ALIGNED` normative rows from every class.

| DEL | Rows | Sampled (A100/SH/GC/PC/NA/AN/ST) | Firm | Weak | Field | Firm false alignment |
|---|---|---|---|---|---|---|
| DEL-17-01 | 96 | 28 (0/7/6/0/5/5/5) | 0 | 0 | 0 | 0/6 = 0% |
| DEL-17-02 | 160 | 42 (0/9/6/0/10/11/6) | 0 | 1 | 0 | 0/16 = 0% |
| DEL-17-03 | 72 | 21 (0/7/0/7/3/1/3) | 0 | 1 | 0 | 0/8 = 0% |
| DEL-17-04 | 97 | 29 (0/7/5/8/4/1/4) | 0 | 1 | 0 | 0/13 = 0% |
| DEL-17-05 | 129 | 39 (1/5/6/6/15/2/4) | 0 | 1 | 0 | 0/7 = 0% |
| DEL-17-06 | 123 | 38 (2/4/9/1/14/4/4) | 0 | 1 | 0 | 0/13 = 0% |
| DEL-17-07 | 102 | 39 (9/9/4/1/8/4/4) | 0 | 2 | 1 | 0/9 = 0% |
| DEL-17-08 | 110 | 33 (1/9/3/1/11/5/3) | 0 | 0 | 1 | 0/7 = 0% |
| DEL-17-09 | 91 | 29 (1/8/3/1/9/4/3) | 0 | 0 | 1 | 0/6 = 0% |
| **PKG-17** | **980** | **298 (14/65/42/25/79/37/36)** | **0** | **7** | **3** | **0/85 = 0%** |

- **Rerun thresholds.** No deliverable has a firm error, so none exceeds 10%, and no firm error falls on a 100%-sampled class.
- **One disclosed judgment.** I classify the DEL-17-04, 17-05 and 17-06 architecture-basis-injection rows as *weak* (§4, W-1a). If Agent 0 or the owner reads them as *firm*, the rerun rule's second clause fires: each is a 100%-sampled row whose correction moves a quiet row to `LOCAL_DESIGN`. That would require reruns of DEL-17-04, 17-05 and 17-06. Each rerun would reduce to the one row, so settling it through `RESOLUTIONS.csv` (F6) would be proportionate.

## 3. Package-level firm false-alignment rate

**0 / 85 = 0.0%** of sampled `ALIGNED` normative rows. The scale-out gate is 5% or less, so it is met.

## 4. Disagreements

### W-1. Architecture Basis Injection block (shared body `e8db53739ad57c63` in 8 deliverables; DEL-17-06 has its own, similar body)

The block `_CONTEXT.md` § Architecture Basis Injection says:
- Scope Changes: "SCA-001 architecture basis as amended by SCA-003 and SCA-004";
- Resolved Baseline: "Rust core/application services where implementation-facing … canonical JSON/JCS-compatible hash basis".

What I found:
- `SOFTWARE_DECOMP.md` L706 says the basis "as amended through SCA-008" is what may be injected. L35 shows that SCA-006 consolidated the PKG-00 kits into `ArchitectureBasis.md`. The injected amendment list is therefore behind the decomposition.
- DEC-009 (L600) adopts Rust core/application services. Every PKG-17 exporter is a Python module under `core/handoff/*`, plus TypeScript desktop panels. No ruling permitting this was found; DEC-060 and DEC-023 do not address it.
- The hash bytes of MBF, PCF, glTF and the external run record are Python `json.dumps(sort_keys, compact, ensure_ascii)`, labelled `JCS_compatible_json_payload_hash`:
  - `caepipe_mbf/package.py` L513 and L743;
  - `pcf_export/package.py` L460 and L680;
  - `review_geometry/package.py` L549 and L805;
  - `caepipe_external/run.py` L605.

  Native JSON, the SDK and stress-neutral 0.1 use the non-JCS label instead.

**W-1a (weak).** DEL-17-04:CONTEXT#architecture-basis-injection, DEL-17-05:CONTEXT#architecture-basis-injection and DEL-17-06:CONTEXT#architecture-basis-injection.
- *Rows:* `ALIGNED`.
- *Worker's reasoning:* later amendments did not change the basis content, and the Python question is "not settled here".
- *My reading:* CP-02 applies to the amendment list. The Notes also record an unsettled baseline question, which F1 disfavours on an `ALIGNED` row.
- *Why weak:* CP-02 names pins, section references and code anchors, not amendment lists without a revision pin, and CS-04 is explicitly scoped to pins. The conventions should settle this.
- *Right values:* `STALE_REVIEW_OR_EVIDENCE · BASIS_POINTER_STALE · LOCAL_DESIGN · NONE · RECORD · NO`. Add a `.sNN` for the DEC-009 element (see W-1b).

**W-1b (weak; the known corpus DEC-009 cluster).**
- *Rows:* DEL-17-03, 17-04, 17-05 and 17-06 record no DEC-009 finding anywhere in their ledgers, although all of them ship Python `core/handoff` code. DEL-17-03 carries the JCS element on AC-001 (`IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · OWNER`), which is correct. DEL-17-01 is documentation-only and DEL-17-02 is contract-only (DEC-076), so the Rust clause does not diverge for either; their CP-02 rows are right.
- *Right values:* a `.sNN` on each code-bearing deliverable's ABI row: `IMPLEMENTED_DIFFERENTLY · PROJECT_BASELINE · NONE · RECORD;BASELINE · OWNER`. The cause (`POSSIBLE_DEFECT` or `AUTHORITY_UNCLEAR`) follows Agent 0's corpus-wide settlement in `W3/RESOLUTIONS_DRAFT_PART2A.csv`.

**W-1c (field).** DEL-17-07, 17-08 and 17-09:CONTEXT#architecture-basis-injection.
- *Rows:* `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · BASELINE;RECORD · OWNER`. The tier and routing for the DEC-009 gap are correct under F8.
- *Field errors:*
  1. The stale amendment list is folded into the same row. C1 says parts that take different dispositions get `.sNN`, and CS-04 practice keeps the main row for the pointer.
  2. The 17-07 and 17-08 Notes say "JCS-labelled hashing hold". That is wrong: the bytes are not RFC 8785, as shown above and in DEL-17-02 REQ-007. The G3 worker flagged this itself.
- *Right values:* the main row takes the CP-02 values. The `.sNN` takes the values the row holds now. Correct the Notes.

### W-2 (weak, cross-group routing split). The missing `plans/EXPORT_FORMAT_INTEROPERABILITY_PLAN.md` (PLAN-EXPORT-INTEROP)

- *Facts:* the file was deleted by `349a2ab33` (2026-06-03). It was present at `7bee9ae41`. No copy exists in the frozen tree.
- *Split treatment:*
  - G1 and G2 apply CP-08: `STALE_* · RECORD_DRIFT · LOCAL_DESIGN · RECORD · OWNER` (DEL-17-01 ×8, DEL-17-02 ×3, DEL-17-06 ×1).
  - G3 uses `STALE_SETUP_SPECIFICATION · BASIS_POINTER_STALE · NO` with no CS. Sampled rows: DEL-17-07:SOW#CLM-008 and DEL-17-07:SOW#CLM-046/DEL-17-07-CF-001. The same treatment appears on further 17-08 and 17-09 rows.
- *The difference:* `AuthorityNeeded` is OWNER on one side and NO on the other. The repair ("restore, re-point or retire the plan") is a choice, which favours OWNER.
- *Why weak:* CP-08 is written for evidence files, not governing sources (G1 disclosed this friction). The conventions should settle whether CP-08 reaches a deleted source.
- *Right values (my reading):* CP-08's cause, tier, layer and OWNER, with the disposition class set by F3 origin.

### W-3 (weak). DEL-17-02:SOW#CLM-017/DEL-17-02-REQ-002

- *Row:* `ALIGNED MEDIUM` ("target behaviour unresolved by the DEL-17-01 TBD register stays TBD").
- *Found:* TBD-17-01-006 (the glTF identity policy) is still open in the DEL-17-01 register. DEL-17-08 selected extras plus an authoritative sidecar (`review_geometry/package.py` L607). The row accepts that selection on the strength of DEC-074 O11/E7, whose option text is in the excluded July `PROPOSED_*` file and was not read. DEC-074's §12 row adopts "O1–O13 option A" without the text.
- *Why weak:* whether an unread adopted option can settle a TBD is for the conventions (DEC-074 O7 flagging precedent).
- *Alternative right values:* `PARTIALLY_IMPLEMENTED · RECORD_DRIFT · LOCAL_DESIGN · RECORD · REVIEW`, or keep the row and flag it `ADOPTED_BY_REFERENCE` as unread.

### Rows checked and upheld (selected)

- **DEL-17-05:SOW#CLM-019** (`VERIFIED_NOT_VALIDATED · VALIDATION_GAP · INVARIANT · VALIDATION · OWNER`). Correct. DEC-080 (L671) owner-gates external-prover activation, and `ValidationEvidence` is `NONE_FOUND`.
- **The DEL-17-07 `INVARIANT · IP_DATA` fixture-provenance rows:** AC-001, CLM-007, CLM-025, CLM-031, CLM-034, VER-001 and REQ-040. Correct under F8: the gap touches the IP boundary's subject, because no fixture provenance or contributor certification record exists.
- **DEL-17-07 REQ-041 and CLM-045** (`UNKNOWN · AUTHORITY_UNCLEAR · INVARIANT · REVIEW`). Upheld; the smallest next check is present (C6(h)).
- **DEL-17-08 VER-001** (`INVARIANT · IP_DATA`). Consistent with DEL-17-07.
- **DEL-17-09:SOW#CLM-018** (`UNKNOWN · AUTHORITY_UNCLEAR`). Upheld. The clause ties the label to the Python serializer; the desktop `hashService.ts` records `rfc8785_jcs` (L121, L148, L165).
- **DEL-17-06 CONTEXT.s01 and SOW.s02** (CP-04 frozen-contract variant for `openpipestress_jcs_ijson_v1`, `package_v0_2.py` L20). Correct fields.
- **Confirmed code facts behind non-aligned rows:**
  - DEL-17-02 CLM-012: native JSON `LOSS_CATEGORIES` uses `"TBD"` (`native_json/package.py` L26–32).
  - DEL-17-04 CLM-023: `_loss_report` synthesises a blocking `tbd` entry when no entries are supplied (L632–647), so `MBF-LOSS-REPORT-MISSING` (L332–340) cannot fire.
  - DEL-17-07 REQ-021: `_coord` defaults missing x, y and z to 0 (`pcf_export/package.py` L724–725).
  - DEL-17-02 CLM-009 and REQ-021/022: no `entity_coverage` field anywhere; `coordinate_policy` is absent from the MBF and stress-neutral profiles.
- **Origin tests (F3).** Sampled against `git log -S`, including DEL-17-01 CLM-015 (origin `7bee9ae41`). They hold.
- **GAP_WORDING_CHECKED and OPEN_ACTION (all 42 sampled).** The clauses are well-founded; "MISSING" appears in them only as part of diagnostic code names. F2 is applied correctly on:
  - DEL-17-03 R01, DEL-17-08 R01 and DEL-17-09 R01 (the gap disposition on the Remaining row);
  - DEL-17-05 R01, DEL-17-08 R02 and DEL-17-09 R02 (OPEN_ACTION).
- **PRODUCT_CALLER: NONE (25 sampled).** In each case the claim is about the builder, profile or schema itself, so F7 is satisfied.
- **No context stands in as authority.** Merged PRs and run records are cited only in `ContextRefs` or as evidence.

## 5. Batch consistency and shared-situation conflicts

- **Body `e8db53739ad57c63`** (5 findings). A real three-way split: G1 CP-02, G2 `ALIGNED`, G3 `IMPLEMENTED_DIFFERENTLY`. No row carries `CANONICAL_DEPARTURE`. Resolution:
  - the main row takes the CP-02 values on all 8;
  - DEL-17-03 to 17-09 add a DEC-009 `.sNN` (W-1).
  - Agent 0 should record all 8 in `RESOLUTIONS.csv`.
- **CP-11 cause findings** (4). These are not conflicts. CP-11 requires the cause of each row's own gap (`DEFERRED_BY_RULING`, `NOT_STARTED` or `PARTIAL_SLICE`). The majority (DEL-17-05) is backed by DEC-080. I checked each flagged row:
  - DEL-17-02 REQ-014 (the timestamp declaration) and REQ-053 (the blocking rule), DEL-17-07 REQ-034 (PCF component mapping) and DEL-17-09 REQ-010 (the contract has no external-execution field): no ruling defers any of these gaps, so `PARTIAL_SLICE` is right.
  - Record them as resolved-consistent.
- **Unflagged shared-situation conflict:** the PLAN-EXPORT-INTEROP routing split (W-2) spans G1/G2 and G3.
- **Other same-titled blocks: no conflicts.** CS-06 rows, CP-09 OUT-001 rows (checked against `AnyPassMatchesFrozen`), CP-03 D-41 declarations (all CP-02 fields, revision 0.8 and DAG-007 against the frozen 0.12 and DAG-010) and CP-01 four-document rows are consistent.

## 6. Reverse pass

- **Capabilities claimed by more than one PKG-17 deliverable:** none.
  - RC-17-0218 (`caepipe_external/run.py`) is `CLAIMED_BY` DEL-17-05 and `COVERS` DEL-17-04, which is the correct binding.
  - DEL-17-01 (16) and DEL-17-02 (29) answer only `COVERS`. That matches a documentation-only source basis and a contract-only deliverable (DEC-076).
- **All claim keys resolve** to the claiming deliverable's own ledger. No `STATUS#remaining` key is used.
- **Suspected missed or inconsistent claims:**
  - **Weak: RC-17-0086 (`NativePackagePanel.tsx`).** DEL-17-03 answers `NOT_MINE` because its SOW excludes GUI. For their own panels, DEL-17-04 (RC-17-0061) and DEL-17-05 (RC-17-0141) answer `PARTIAL`, and DEL-17-07, 17-08 and 17-09 answer `CLAIMED_BY`. The panel cites DEL-17-03 as source basis (L288, L876). DEL-17-03's forward ledger also does not record the GUI-exclusion contradiction that DEL-17-04 records (CLM-003 and CLM-008 `IMPLEMENTED_DIFFERENTLY`). R3 should apply one treatment across the PKG-17 panels.
  - **Correctly unowned: RC-17-0273 (`exportUnitDisclosure.ts`).** DEC-076 classifies it as shared desktop infrastructure with DEL-17-02 as contract owner only. DEL-17-02's `COVERS` is right; ownership belongs to R3's unmapped set.
- **F5 (7 of 33 sampled).** All reasons are specific to the capability (for example the registers, CONTRACT, IP_AND_DATA_BOUNDARY and `model.schema.yaml`). No template reasons.
- **Answer distribution: sampled rows against area rows.**
  - `SAMPLE`-routed rows: 36 per deliverable, all `NOT_MINE` (324 of 324).
  - `AREA` rows: 89.8–98.9% `NOT_MINE` per deliverable. The low end is DEL-17-02, whose 29 `COVERS` answers are contract relations.
  - The `SAMPLE` rows come from SHELL, COREB, SOLVER, PHYS, WSUI and VIEW, which no PKG-17 deliverable plausibly owns. Uniform `NOT_MINE` is therefore expected, and there is no sign of anchoring.
  - In the 10% `NOT_MINE` sample (252 area and 29 sample rows), I found no export or handoff capability wrongly disowned beyond RC-17-0086.
  - Area capabilities that every PKG-17 deliverable disowned (handoff exporter workflow, target mapping, external prover, redaction and adapter framework) belong to PKG-15, PKG-12, PKG-10 and PKG-08 surfaces. That is for R3.
- **Anchored answers:** none detected.

## 7. What the owner must see

1. **DEC-009 departure across all of PKG-17.** Every exporter is a Python `core/handoff` module, plus TypeScript desktop builders. DEC-009 adopts Rust core/application services, and no ruling permits this. Only DEL-17-07, 17-08 and 17-09 record it. The corpus-wide cluster needs one owner decision.
2. **"JCS-compatible" labels on bytes that are not JCS.** The MBF, PCF, glTF and external-run packages label Python sorted-compact ASCII bytes `JCS_compatible_json_payload_hash`. The contract owner row is DEL-17-02 REQ-007 (`PROJECT_BASELINE · POSSIBLE_DEFECT · OWNER`). The desktop hash service separately labels `rfc8785_jcs`. This is an owner and engineering question, not a record catch-up.
3. **IP and data (INVARIANT).**
   - The PCF invented fixture pairs a nominal size with OD and wall values that match a published dimensional-table entry (DEL-17-07 REQ-041 and CLM-045, `UNKNOWN`). A maintainer review is needed.
   - The PCF and glTF fixtures have no fixture-provenance or contributor-certification record.
4. **Validation is held.** DEL-17-05 CLM-019 is `VERIFIED_NOT_VALIDATED`. Live CAEPIPE runs stay owner-gated under DEC-080.
5. **Missing governing plan.** PLAN-EXPORT-INTEROP was deleted on 2026-06-03 and many SOW rows still cite it. The choice is to restore it, re-point the citations or retire it. The workers routed it inconsistently (OWNER against NO).
6. **Code-defect candidates.** None is judged here as a ruling:
   - MBF `MBF-LOSS-REPORT-MISSING` cannot fire;
   - PCF writes 0 for a missing coordinate;
   - the PCF writer omits `unit_system_disclosure.json`, which its manifest lists;
   - native JSON uses the loss token `TBD` where the contract uses `tbd`.
7. **Holds and R4 candidates.**
   - The PDU-031 timestamp and generator policy is an owner hold (DEL-17-08 R01, `OWNER_HOLD`).
   - The `openpipestress_jcs_ijson_v1` frozen-contract rename is an R4 code-change candidate (DEL-17-06).
   - The desktop GUI panels contradict the SOW GUI exclusions (DEL-17-04 and 17-05 recorded; DEL-17-03 not).
8. **The disclosed weak/firm call in §2.** If the three `ALIGNED` ABI rows are read as firm, DEL-17-04, 17-05 and 17-06 hit the rerun rule. I recommend settling them through F6 instead.

Nothing here is a release, approval, compliance or certification statement. Agent dispositions are not owner rulings.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
