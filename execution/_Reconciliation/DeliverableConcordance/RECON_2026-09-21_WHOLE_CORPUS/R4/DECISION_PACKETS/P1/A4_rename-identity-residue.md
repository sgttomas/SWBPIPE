# A4 — Rename and identity residue (one ruling)

PROPOSAL for the R4 gate (R3 integration, TASK P1). It decides nothing and changes no row.

Conventions: `F:` is `{FREEZE}/projects/chirality-piping/` at `00115c719`. `RUN/` is the run folder.

## 1. Decision

One ruling on how the former product name OpenPipeStress is removed from deliverable text and from active code identifiers after DEC-101 renamed the product SWBPIPE, variant by variant, including whether the four identifiers kept on 2026-09-18 are renamed or kept.

**Holder: OWNER.** `DEL-01-01:SOW` (the ISSUED text) is **not** decided here; A6 owns its single route.

## 2. Background

- **Ruling.** DEC-101 (`F:execution/_Decomposition/SOFTWARE_DECOMP.md:692`, owner ruling 2026-09-18) names the product SWBPIPE. Its (iv) identity-layer act covers the schema `$id` namespace, `openpipestress.*` document-kind constants, package, crate and binary names. It carries a persistence-compatibility obligation (the product keeps reading documents with the former kinds and namespace), and it says other governance documents take the name at their next amendment and historical records never change. It is executed only by an owner-authorised implementation tranche.
- **Run conventions.** C6(e) and R0 ruling item 3: DEC-101 does not reach deliverable SOW files, so residue there is a finding (RENAME_OR_IDENTITY), never an accepted divergence, clustered for one R4 ruling (`RUN/CONVENTIONS.md` C6(e)). CP-04 (`RUN/CANONICAL_SITUATIONS.md:74`) records it once per surface with AuthorityNeeded OWNER.
- **The four kept identifiers.** The owner kept `.opsproj`, `openpipestress_jcs_ijson_v1`, `openpipestress_result_semantics_v0_2` and `openpipestress-projects.sqlite3` on 2026-09-18 (`ACTIVATION_2026-09-18.md` §2, now context). R0 addendum item 2 records the owner's words that identifiers of active code "need to revise them to be compatible with the current name". Agent 0 read that as covering the four (`RUN/R0_CALIBRATION/R0_RULING.md` addendum item 2). That is an AGENT reading, open to the owner's correction.
- **What the code carries** (read at the freeze): `CONTAINER_EXTENSION = "opsproj"` (`F:core/reporting/report_package/src/lib.rs:56`); `PROJECT_STORE_FILE = "openpipestress-projects.sqlite3"` (`F:apps/desktop/src-tauri/src/lib.rs:32`); the const `openpipestress_jcs_ijson_v1` (`F:schemas/stress_neutral_export.v0.2.schema.json:388`); the const `openpipestress_result_semantics_v0_2` (`F:schemas/analysis_run.v0.2.schema.json:194`). Also crate and binary names such as `open_pipe_stress_*` and `openpipestress-runner`, and schema `$id` URLs on `openpipestress.org` (T4B-C01).

## 3. Options

Per variant, as they stand in the evidence (T4B-C01):

1. **Deliverable text residue** (SOW, AB, STATUS, CONTEXT). One R5 record-repair sweep replacing the former **prose product name**, leaving history, run records and references to past states untouched. R0 item 3 rules out accepted divergence, so this is the only option the rulings leave open for deliverable prose. It does **not** reach text that names an active code identifier: those rows (at least the 9 PROJECT_BASELINE rows, 6 persistence and 3 frozen-contract, and the default-variant rows or passages that name crate, binary, schema `$id` or document-kind identifiers) wait for the option 2/3 ruling and follow it, so the text never names an identifier that does not exist (2a/3a) or is re-read as not residue (2b/3b).
2. **Active code identifiers, default variant** (crate, binary, schema `$id`, document kinds):
   - **2a.** Rename through code-fix briefs, following DEC-101 (iv) (the owner's R0 addendum words point here);
   - **2b.** Record them as retained identifiers.
3. **The four kept identifiers**, each separately:
   - **3a.** Rename: with read-compatibility for existing `.opsproj` files and stores; through a new contract version for the two frozen-contract identifiers;
   - **3b.** Keep: the owner corrects Agent 0's reading, and the 2026-09-18 selection stands; the rows are then re-read as not residue.
4. **The DEC-022 grammar identifier** `open_pipe_stress_declared_expression` (active in `core/rules/expression_evaluator` and `schemas/rule_pack.schema.yaml` per the OBSERVED resolution on `DEL-11-02:SOW`). It has no row of its own; the ruling should name it under 2 or 3.

**Consequences.** Option 1 is a mechanical text tranche across 80 deliverables. 2a and 3a are code changes. 3a on the frozen identifiers is a contract-version change (A2 owns whether content also changes, and the two can share one version bump). 3a on `.opsproj` and the store filename carries DEC-101's persistence-compatibility test obligation. 2b or 3b leaves code and product under two names.

## 4. Evidence and reliability

| Source | Shows | Reliability |
|---|---|---|
| DEC-101 (`SOFTWARE_DECOMP.md:692`) | Rename scope and obligations | Governing source |
| C6(e), CP-04, R0 item 3 and addendum item 2 | Treatment of residue; the AGENT reading on the four | RULED conventions; the four-identifier application is an AGENT reading |
| Code lines above | The identifiers are active | Frozen code, read by this task |
| T4B-C01 (`RUN/R3/TASKS/T4B_CLASSES.md`) | 86 rows, four variants, options | R3 task proposal over verifier-checked ledgers |
| T11 S-06 (`RUN/R3/TASKS/T11_METHOD.csv`) | Rename sub-claims split on tier: LOCAL_DESIGN (DEL-05-04 SOW.s01, DEL-05-05 SOW.s01) against PROJECT_BASELINE (DEL-17-06 SOW.s02 frozen profile, DEL-02-05 STATUS.s01 `.opsproj`) | R3 screen; the difference may be justified by frozen wire identifiers, and the ruling should state it |
| W3 owner list (`RUN/WAVES/W3/W3_ASSESSMENT.md:88`) | The frozen-contract rename `openpipestress_jcs_ijson_v1` (PKG-17) | Agent 0 assessment |

## 5. Affected claims

**This packet's portion: 85 rows** of T4B-C01 (Authority OWNER; 86 rows). Filter: `ClassID == 'T4B-C01' and ClaimKey != 'DEL-01-01:SOW'`. The excluded row is A6's portion. T4B-C01 is not listed as a split class in the topic file; reported to Agent 0.

| Variant | Rows | Keys or filter |
|---|---|---|
| Default text and identifier residue (LOCAL_DESIGN · NONE · RECORD) | 76 | portion filter and `AuthorityTier == 'LOCAL_DESIGN'` |
| Persistence identifiers (PROJECT_BASELINE · NONE · RECORD) | 6 | `DEL-00-01:AB`, `DEL-00-02:AB`, `DEL-00-07:AB`, `DEL-00-08:AB`, `DEL-02-05:STATUS.s01`, `DEL-08-01:STATUS` |
| Frozen-contract identifiers (PROJECT_BASELINE · FROZEN_CONTRACT · RECORD;BASELINE) | 3 | `DEL-14-02:STATUS`, `DEL-17-06:CONTEXT.s01`, `DEL-17-06:SOW.s02` |

Packages (rows): PKG-00 5, 01 3, 02 6, 03 5, 04 1, 05 5, 06 2, 07 5, 08 7, 09 4, 10 5, 11 5, 12 5, 13 4, 14 4, 15 4, 16 4, 17 11. Deliverables: 80 (T4B-C01's 81 less DEL-01-01).

**Exceptions kept visible** (T4B-C01): RESOLVED_PAIR on `DEL-00-01:AB`, `DEL-00-02:AB`, `DEL-00-03:AB`, `DEL-00-07:AB`, `DEL-00-08:AB`, `DEL-02-05:STATUS.s01`; OBSERVED on `DEL-05-01:SOW`, `DEL-05-02:SOW`, `DEL-05-03:SOW`, `DEL-05-05:SOW.s01`, `DEL-08-06:SOW`, `DEL-09-04:SOW`, `DEL-09-05:SOW`, `DEL-11-02:SOW`; FIELD on `DEL-05-04:SOW.s01`, `DEL-10-04:SOW`, `DEL-10-05:SOW`, `DEL-14-05:SOW`.

**Identifiers with no row** (T4B observations 2–3): the store filename (mentioned only in `DEL-02-05:SOW` Notes), `openpipestress_result_semantics_v0_2` (no row mentions it), and `open_pipe_stress_declared_expression` (OBSERVED on `DEL-11-02:SOW`, from `OtherCorrections`). The `DEL-14-05:SOW` FIELD correction (`OtherCorrections`) adds the schema `$id` and titles to its RemainingWork.

**Cross-reference.** `DEL-01-01:SOW` → A6. The contract-version content question for `openpipestress_jcs_ijson_v1` → A2. T9-C12 SRE-7 counts 85 RENAME_OR_IDENTITY rows among overtaken declarations; they are these rows.

## 6. Risks

- **Undecided.** Deliverables and code keep naming a product that no longer exists. Each later rename raises the cost of the frozen-contract and persistence migration. Future briefs sealed from these surfaces inherit the old name.
- **2a / 3a.** Code churn across crates, binaries and schemas; persisted files and stores need a read path for the old identifiers (DEC-101 obligation); frozen contracts need a version change that consumers must follow.
- **2b / 3b.** The identity stays split; DEC-101 (iv) names these identifier kinds, so retaining them needs an explicit owner record to avoid a conflict with that ruling.
- **Option 1 alone.** Text says SWBPIPE while code identifiers say OpenPipeStress; readers may think the rename is complete.

## 7. Recommended routing

Option 1 is the only option the rulings leave open for deliverable prose (R0 item 3 excludes accepted divergence), so the evidence supports authorising it **for prose product-name residue only**. Rows and passages naming active identifiers (at least the 9 PROJECT_BASELINE rows) wait for the 2/3 ruling and are repaired with it. The code-identifier variants (2 and 3) have no recommendation; owner's call, in particular on whether Agent 0's reading of the four kept identifiers is correct.

## 8. On-ruling mechanism

- **Option 1.** Under the owner's separate R5 authorisation (D-73), an R5 record-repair tranche per package through the ordinary change path (`.agents/skills/chirality-change/SKILL.md`), over the prose product-name residue on the 85 rows' surfaces, leaving history untouched. Identifier-naming text (the 9 PROJECT_BASELINE rows and identifier passages in default-variant rows) is repaired only after, and consistent with, the 2/3 ruling (in the same tranche as the code rename under 2a/3a, or as a not-residue re-read under 2b/3b).
- **2a.** CODE_FIX_CANDIDATE briefs (H2) for crate, binary, `$id` and document-kind renames, executed under the owner-authorised DEC-101 implementation tranche, with the persistence-compatibility tests DEC-101 requires.
- **3a.** As 2a; for `openpipestress_jcs_ijson_v1` and `openpipestress_result_semantics_v0_2`, a new contract version through the baseline change path (combinable with A2's outcome); for `.opsproj` and the store filename, a read-compatibility test obligation.
- **2b / 3b.** An owner record naming the retained identifiers, then an R5 repair that re-reads those rows as not residue.

Nothing executes until the owner acts. R5 needs separate authorisation.

## 9. Dependencies

- **Depends on:** none.
- **Interacts with:** A2 (same frozen identifier), A6 (the ISSUED `DEL-01-01:SOW` row; one route).
- **Blocks:** H4 repair of the 85 rows; H2 identifier-rename briefs; any R5 sweep that would otherwise touch these surfaces' product name.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). No certification, code-compliance, professional-approval or engineering-acceptance claim is made.
