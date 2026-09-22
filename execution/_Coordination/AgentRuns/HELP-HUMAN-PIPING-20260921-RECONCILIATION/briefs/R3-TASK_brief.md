# Brief — R3 synthesis task (TASK, Type 2)

You are a TASK agent in R3 (cross-package synthesis) of run
HELP-HUMAN-PIPING-20260921-RECONCILIATION. Your parent is HELP_HUMAN Agent 0.
You do not delegate. Your launch message names your task ID ({TASK}), its
scope and its output files.

## Purpose

R2 produced 102 accepted, verified deliverable ledgers. R3 reads them together
and prepares the R4 owner decision gate. Your task is one bounded slice of
that synthesis. You classify and propose. You never repair, rule or decide.

## Accepted basis (read-only)

`RUN` is `{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

- **`RUN/R3_PLAN.md`.** Your task's row, the partition rules and the class contents.
- **`RUN/R3/CORPUS_CLAIMS.csv`.** One row per claim key, all 102 deliverables.
  - **Effective values.** Where an adopted resolution row sets a Disposition, its five value fields replace the sealed ones: Disposition, CauseTag, AuthorityTier, BaselineClass and DivergenceLayers. `Sealed*` columns keep the sealed values.
  - **Other corrections.** Anything else a resolution corrects is **not** applied: AuthorityNeeded, CanonicalSituation, FindingGroup, RemainingWork, evidence and notes. It is carried word for word in `OtherCorrections`, prefixed by the resolution class. Take it into account wherever it bears on your work. For example, a corrected AuthorityNeeded changes the owning authority you propose.
  - **`ResolutionClasses`** lists every resolution row for the key.
  - **Flags.** `Divergent = YES` marks every row whose effective Disposition is not ALIGNED, NOT_ASSESSED or COVERED_BY_CHILDREN. `ProductCallerNone = YES` marks rows whose Notes carry `PRODUCT_CALLER: NONE` (F7).
  - **Your population** comes from this file, filtered exactly as your launch message states.
- **The other `RUN/R3/` tables.** `CAPABILITY_COVERAGE.csv`, `CLUSTER_MATRIX.csv`, `PACKAGE_SUMMARY.csv` and `REMAINING_CENSUS.csv`. `R3/SYNTHESIS_STATS.md` records their hashes.
- **The sealed ledgers**, `RUN/WAVES/W*/PKG-*/DEL-*/DEL-*_{forward,reverse,notes}.*`. Ignore every `superseded_<n>/` folder. Read them for a row's Notes, evidence and RemainingWork.
- **The adopted resolutions**, `RUN/WAVES/{W1,W2,W3,CROSS_WAVE}/RESOLUTIONS.csv`. Ignore the draft and merged-draft files (`RESOLUTIONS_DRAFT_*.csv` and `CROSS_PACKAGE/RESOLUTIONS_MERGED_DRAFT.csv`). They are superseded working files. The verification reports are `RUN/WAVES/*/PKG-*/*_VERIFICATION.md` and the cross-package and cross-wave reports. The assessments are `RUN/WAVES/W1/W1_GATE_ASSESSMENT.md`, `W1_GATE_RULING.md`, `RUN/WAVES/W2/W2_GATE_ASSESSMENT.md` and `RUN/WAVES/W3/W3_ASSESSMENT.md`.
- **Conventions and vocabulary.** `RUN/CONVENTIONS.md` (Parts A–F), `RUN/CANONICAL_SITUATIONS.md`, `RUN/AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`, `RUN/IMPLEMENTATION_SURFACES.csv` and `RUN/ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`.
- **The frozen source**, `{FREEZE}`. It is a read-only checkout of `00115c71931bcae79909602d653740d3bb72dfa1`. It is evidence, never authority. Code shows what was built, not what was decided.
- **T10 only:** `{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/` (the July concordance). It is a cross-check, not evidence or authority. No other task reads it.

## Rules

1. **Authority.** Effective ledger values are accepted R2 results. Do not
   re-dispose rows. If you think an effective value is wrong, record it as an
   `R3_OBSERVATION` in your notes, with evidence. It is not a correction.
   Owner decisions stay with the owner, engineering decisions with
   engineering, and scope changes with the scope-change workflow. You only
   propose routing.
2. **Exactly once.**
   - T4A–T7: every divergent row in your partition gets exactly one class.
   - T1–T3: every capability in your partition gets exactly one row.
   - Before you return, check your output's coverage against
     `CORPUS_CLAIMS.csv` or `CAPABILITY_COVERAGE.csv` with a script. Report
     the counts.
3. **Visibility.** Never merge away an exception. Contested, field and
   observed resolutions (see `ResolutionClasses`) stay visible. When a class
   contains such rows, name them in the class notes.
4. **Classes (T4A–T7).** Use about 3–8 classes. Each class needs:
   - a stable ID `<TASK>-C<nn>`;
   - a name and a one-paragraph description;
   - its signature (cause, tier and disposition);
   - its population count, packages and deliverables;
   - the owning authority: OWNER, ENGINEERING, REVIEW, SCOPE_CHANGE,
     HELPS_HUMANS, EXTERNAL or NONE;
   - the recommended routing, one route from the route vocabulary below;
   - the exact on-ruling mechanism, meaning what a ruling would authorise and
     through which workflow or change path;
   - the risk if left unrepaired;
   - two or three representative keys, with evidence.

   A class that needs an owner decision must say what the decision is, and
   give the options as they stand in the evidence.
5. **Capabilities (T1–T3).** Classify each capability as one of:
   - PRODUCT_UNOWNED: a product capability no deliverable owns. Name the best
     existing owner if one exists; otherwise propose CREATE.
   - ROUTING_GAP: a deliverable plausibly owns it but was not routed it, or
     answered otherwise. Name the deliverable and cite its scope text.
   - NON_DELIVERABLE: tooling, CI, test infrastructure, demos or scaffolding
     outside any deliverable's scope. Say why.
   - SHARED_OK: the ownership split is complementary.
   - DUPLICATE: two owners conflict. Say which owner is better supported,
     with evidence.
   - UNKEYED_SCOPE_GAP: owned, but no issued key covers it.
   - PARTIAL_UNOWNED_REMAINDER: owned only in part, and the rest is unowned.
     Name the unowned part and its best owner.

   Also give a confidence (HIGH, MEDIUM or LOW) and evidence (file:line at
   the freeze, or scope-text references).
6. **Evidence.** Cite `path:line` at the freeze, ledger keys, or report
   sections. Never run builds or tests. Reading files and running your own
   small read-only scripts is fine. Quote no protected, private or
   third-party copyrighted content. Follow the DEC-043 equation-source
   exclusion: do not quote equation sources.
7. **Claim fence.** Make no claims of certification, code compliance,
   professional approval or engineering acceptance (F-PIP-2, claims taxonomy
   per DEC-081). You describe records and evidence, not engineering adequacy.
8. **Other tasks.** Do not read other tasks' files under `RUN/R3/TASKS/`.
   Your work must be independent. T10's July material is visible to T10
   alone.
9. **Writes.**
   - Write only your named output files, under `RUN/R3/TASKS/`.
   - Scratch files go under `RUN/R3/TASKS/_scratch_{TASK}/`. Delete them
     before you return.
   - Do not use git or the network, or write anywhere else. Never write in
     `{FREEZE}`.
   - Run Python with `PYTHONDONTWRITEBYTECODE=1`.
10. **API overload.** The API returns intermittent 529 errors. Write your
   output incrementally, for example per class or per area, and keep each
   generation short. If you are resumed after an interruption, check what
   is on disk and continue from there; don't start over.

## Route vocabulary (every Route or routing field)

- `R5_RECORD_REPAIR`: a deliverable text or record repair under a later R5 ruling.
- `SCOPE_CHANGE_HANDOFF`: retire, merge, create or reassign deliverables.
- `CODE_FIX_CANDIDATE`: a candidate brief for a code or test change. It is not executed.
- `ENGINEERING_AUTHORITY`: an engineering or validation decision.
- `OWNER_DECISION`: a product, authority or baseline decision.
- `REVIEW`: a review under the review workflow.
- `NO_ACTION`: nothing to change, for example an accepted divergence or a correct record. Say why.

Note that `OwnerKeys` in `CAPABILITY_COVERAGE.csv` separates owners with ` | `. A single owner's keys may themselves contain `;`.

## Output formats

- **CSV files.** Use Python's `csv` module with CRLF line endings. The last
  row is `#END`, with the body count in the last column.
  - T4A–T7 `_CLASSES.csv` columns: `ClaimKey, DeliverableID, ClassID`, one row per divergent row in the partition.
  - T1–T3 CSV columns: `CapabilityID, Area, Status, Classification, ProposedOwner, Confidence, Evidence, Notes`.
  - T8 `_ROWS.csv` columns: `ClaimKey, DeliverableID, Cluster, ProposedReading, Route`.
  - T9 CSV columns: `DeliverableID, Item, Finding, Evidence, Route`.
  - T10 CSV columns: `DeliverableID, Topic, JulyStatus, CurrentStatus, Change, Evidence`.
  - T11 CSV columns: `Check, Subject, Deliverables, Finding, Evidence, Route`. `Check` is one of DEPENDENCY, TERMINOLOGY, REUSED_EVIDENCE, SUBCLAIM_SPLIT or SHARED_SURFACE.
  - T12 CSV columns: `ClaimKey, DeliverableID, Engine, Area, ClusterID, Route`, one row per `ProductCallerNone = YES` row.
- **Markdown files.** Start with a one-paragraph summary, then give the
  classes or findings. End with a `## Coverage` section (the counts and the
  check you ran) and a `## R3 observations` section, which may be empty.

## Return

Return one line:

`R3 {TASK} DONE rows=<n> classes=<n> <file>=<sha256> ...`

Then give at most 12 lines:
- the classes, with their counts;
- the owner-decision candidates;
- any coverage problem;
- any R3 observations.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
