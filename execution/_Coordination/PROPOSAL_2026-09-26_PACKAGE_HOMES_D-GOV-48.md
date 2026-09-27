# PROPOSAL — Package homes for five Piping scope items (D-GOV-48), with a DAG wording note (D-GOV-49)

Status: PROPOSAL. Analysis only. No register, ledger or project record is edited
by this file. The human picks.
Date: 2026-09-26. Basis: `origin/main` `dfb089b8a`.
Prepared by: a Claude Code session, at the owner's direction of 2026-09-26:
"D4 yes proceed that way".

## 1. The finding, confirmed

The validator reads `<root>/_Decomposition/ScopeLedger.csv`. Piping keeps its
registers in `docs/_Registers/`, so a run against `projects/chirality-piping/execution`
skips the XRG family. To reach them, stage a scratch root whose `_Decomposition`
points at Piping's register folder:

```
S=<scratch>/piping_xrg; mkdir -p "$S"
ln -s "$PWD/projects/chirality-piping/docs/_Registers" "$S/_Decomposition"
python3 tools/validation/validate_decomposition_registers.py "$S" --families XRG
```

Result: exit 1, with `XRG-014` ×5 (SOW-030, SOW-046, SOW-052, SOW-074 and
SOW-075, each naming two Packages). The same run also reports `XRG-005` ×29
errors (objectives not propagated to the covering deliverable) and `XRG-008`
×106 warnings (blank `PhaseHint`). Those were already there and are separate
matters, not covered here.

## 2. The pick

| Item | `PackageID` now | Recommended home | Alternative | Confidence |
|---|---|---|---|---|
| SOW-030 | `PKG-10, PKG-17` | **PKG-10** | PKG-17 | High |
| SOW-046 | `PKG-08, PKG-17` | **PKG-08** | PKG-17 | Medium: closest call |
| SOW-052 | `PKG-05, PKG-04` | **PKG-05** | PKG-04 | High |
| SOW-074 | `PKG-15, PKG-17` | **PKG-15** | PKG-17 | High |
| SOW-075 | `PKG-15, PKG-17` | **PKG-15** | PKG-17 | High |

In every case the recommendation is the item's home before a later amendment
added a second Package. SCA-004 added PKG-17 on 2026-05-18, "under existing
export/handoff scope boundaries" (SCA-004 `Decision_Log.md`), with "existing
scope boundaries preserved" (`Amendment_Actions.csv` A011). SCA-011 added
PKG-04 to SOW-052 on 2026-09-22. No Deliverable mapping changes.

### Candidate Packages

`Deliverables.csv` has no Package-name column. The names and purposes below come
from the package register, `execution/_Decomposition/SOFTWARE_DECOMP.md` §6
(lines 224–237).

| Package | Name | Purpose (from §6, shortened) |
|---|---|---|
| PKG-04 | Solver Core and Numerical Methods | Global 3D centerline/frame mechanics, supports, nonlinear support logic, diagnostics |
| PKG-05 | Loads, Load Cases, and Stress Recovery | Primitive loads, concentrated/distributed user loads, load-case algebra, stress recovery |
| PKG-08 | Reporting, Audit, and Reproducibility | Calculation reports, audit manifests, hashes, report guardrails, "and exports" |
| PKG-10 | Build, Packaging, API, and Interoperability | Public API/plugin boundaries, import/export adapters, headless execution, packaging |
| PKG-15 | Handoff and External Prover Workflow | Canonical handoff package, manifests, target mapping, unsupported-target flags, external-prover metadata |
| PKG-17 | Export Format Interoperability | Deterministic export-format contracts, target profiles, stable ID maps, loss reports, harness boundaries, adapter SDK |

## 3. Per item

**SOW-030.** "The architecture shall support public APIs, plugins, and
import/export adapters for integration with external tools." Source: PRD §19.3,
which is "Public API" in the archived v0.1 (`docs/_history/PRD_v0.1.md`).
- Deliverables: PKG-10 has DEL-10-01 (public API and plugin boundary) and
  DEL-10-02 (import/export adapter framework). PKG-17 has DEL-17-01, -02, -03,
  -04, -05, -07, -08 and -09.
- Evidence: PKG-10's §6 purpose restates the item almost word for word.
  `docs/architecture/plugin_boundary.md` carries `scope_item: SOW-030`. In DAG-011,
  DEL-17-09 depends on DEL-10-01 (constraint) and DEL-10-02 (interface).
- Recommendation: **PKG-10.** It owns the architectural API, plugin and adapter
  obligation. PKG-17 builds target formats on top of that obligation.

**SOW-046.** "The product shall support result exports suitable for review,
regression comparison, and downstream tooling." Source: PRD §15.1, §19.3
(v0.1 "Report Requirements" and "Public API").
- Deliverables: PKG-08 has DEL-08-04 (result export format, an API contract).
  PKG-17 has DEL-17-05 (CAEPIPE harness and CSV parser) and DEL-17-06
  (stress-neutral CSV/JSON package).
- Evidence: PKG-08's purpose includes "exports". DEL-08-04 is the only
  deliverable that covers SOW-046 alone. In DAG-011, DEL-17-06 depends on
  DEL-08-04 (interface). The ledger note calls the baseline "schema-first JSON
  result envelopes for review/regression" and describes PKG-17's work as
  additions ("SCA-004 adds").
- Recommendation: **PKG-08.** It owns the baseline result-export contract, and
  PKG-17 adds formats to it. This is the closest call: PKG-17 holds two of the
  three Deliverables. If the owner reads "downstream tooling" as the main
  obligation, PKG-17 is a defensible home.

**SOW-052.** "The load engine shall support concentrated forces, concentrated
moments, and distributed user loads in addition to core primitive piping load
categories." Source: PRD §10 FR-007 and §11.5 (v0.1 "Load Types").
- Deliverables: PKG-05 has DEL-05-05 (concentrated and distributed user load
  application). PKG-04 has DEL-04-07 (product solve integration and nonlinear
  orchestration).
- Evidence: the item's home was PKG-05 only until SCA-011 (see
  `SCA-011_2026-09-22_OWNERSHIP/AMENDMENT.patch` lines 97–99). The ledger note
  added by SCA-011 reads: "DEL-04-07 owns product integration only; primitive
  obligations remain." SCA-011 gave SOW-005, -011, -012 and -053 one home each.
  SOW-052 has two only because DEL-04-07 sits in another Package.
- Recommendation: **PKG-05.** It is the load engine, and SCA-011 says the load
  obligation stays there.

**SOW-074.** "The product shall generate schema-compliant handoff packages with
model hash, units manifest, entity IDs, library/rule references, unresolved
assumptions, warnings, target mapping metadata, and unsupported-target flags."
Source: PRD v0.2 §16.1, FR-HAND-001 through FR-HAND-004.
- Deliverables: PKG-15 has DEL-15-01 (canonical handoff schema and manifest),
  DEL-15-02 (target mapping and unsupported behavior) and DEL-15-03 (downstream
  export workflow). PKG-17 has DEL-17-01, -02, -03, -04, -06, -07, -08 and -09.
- Evidence: SCA-002 created PKG-15 for this item and SOW-075, and PKG-15's §6
  row lists only those two. The SCA-002 review handoff records "`SOW-074`
  (`PKG-15`)". In DAG-011, DEL-17-07 depends on DEL-15-02 (prerequisite) and
  DEL-17-08 depends on DEL-15-01 (interface).
- Recommendation: **PKG-15.** The item's statement is the PKG-15 contract (the
  canonical package, its manifest and target mapping). PKG-17 holds more of the
  Deliverables, but they are format-specific realizations of it.

**SOW-075.** "The product shall support external-prover workflow metadata
without forcing a formal prover-status lifecycle, automatic professional
acceptance record, or comprehensive commercial-tool result ingestion in the
MVP." Source: PRD v0.2 §4.2, §16.2–§16.4, §23.
- Deliverables: PKG-15 has DEL-15-04 (external prover boundary metadata).
  PKG-17 has DEL-17-01, -04, -05 and -09.
- Evidence: the same SCA-002 origin, and the SCA-002 handoff records
  "`SOW-075` (`PKG-15`)". The ledger note says that SCA-004 permits CAEPIPE
  harness records "only as non-authoritative external-run evidence", which is a
  bounded addition.
- Recommendation: **PKG-15.** It owns the prover-boundary metadata. PKG-17
  supplies one target's harness within that boundary.

## 4. Applying the pick

**Ledger edit** (`projects/chirality-piping/docs/_Registers/ScopeLedger.csv`,
`PackageID` cell only; every other cell unchanged):

| Line | Row | `PackageID` before | after |
|---|---|---|---|
| 31 | SOW-030 | `"PKG-10, PKG-17"` | `PKG-10` |
| 47 | SOW-046 | `"PKG-08, PKG-17"` | `PKG-08` |
| 53 | SOW-052 | `"PKG-05, PKG-04"` | `PKG-05` |
| 75 | SOW-074 | `"PKG-15, PKG-17"` | `PKG-15` |
| 76 | SOW-075 | `"PKG-15, PKG-17"` | `PKG-15` |

**No change** is needed to `Deliverables.csv`, `ContextBudgetQA.csv`, any
Deliverable's mapping, or any DAG version. Counts and `UnassignedScopeItems = 0`
are unchanged.

**Matching edits outside the CSV.** Some other records repeat the two-Package
cells, so the pick is not quite zero-touch:
- `SOFTWARE_DECOMP.md` §9 repeats the ledger's `PackageID` at lines 507, 523,
  529, 551 and 552. Apply the same five edits there.
- The §6 column "Assigned Scope Items" also lists every Package an item touches.
  Line 224 lists SOW-052 under PKG-04, and line 237 lists all four other items
  under PKG-17. To match the pick, remove SOW-052 from PKG-04. PKG-17 would then
  home no scope item, which no rule forbids. Its cell could read "none homed;
  Deliverables serve SOW-030, SOW-046, SOW-074 and SOW-075 (homed in PKG-10,
  PKG-08 and PKG-15)".
- If §6 changes, 15 deliverable `_CONTEXT.md` files copy its "Package Assigned
  Scope Items" line: six in PKG-04 and nine in PKG-17. They can be refreshed in
  the amendment's propagation, or at each deliverable's next context refresh.
  The human picks.
- Update §12 and the revision header to 0.14.

**Validator consequence (important).** I simulated the five edits on a scratch
copy and reran XRG. `XRG-014` clears, but **23 `XRG-004` errors** appear:
"PackageID disagrees … for a linked deliverable". One appears for each
Deliverable in the non-home Package. Any choice of homes gives this result,
because every one of these items has Deliverables in both Packages. `XRG-004`
therefore conflicts with the rule the owner restored. Management manual v7
(line 1081) says: "Several Deliverables may contribute to an included
obligation, including supporting contributions from another Package." Proposed
Root follow-up, outside Piping's authority: narrow `XRG-004` to "the home
Package holds at least one of the item's Deliverables". All five
recommendations meet that test. Until then, Piping would adopt with 23 known
`XRG-004` findings recorded against this ruling.

**Instrument.** Add a new row, **D-77**, to
`execution/_Coordination/_DECISIONS/_REGISTER.md`: an owner ruling that picks the
five homes and adopts D-GOV-48 for Piping. Apply it as a scope-neutral
amendment, **SCA-012**, following the SCA-006 and SCA-008 precedent. It would
carry **DEC-115** in `SOFTWARE_DECOMP.md` §12 and update
`execution/_ScopeChange/_LATEST.md`. These are the next free numbers at
`dfb089b8a`.

## 5. Separate matter: Piping's DAG wording under D-GOV-49

Some Piping records call the DAG the "dependency graph authority" and call local
files "synchronized mirrors". D-GOV-49 (SPEC §5.4) says neither side authorizes
itself. An accepted version governs only through its acceptance and only while
it is current with the local evidence.

**Proposed standard sentence** (the materializer direction):

> Dependency rows are materialized from the accepted DAG version named by
> `execution/_DAG/_LATEST.md` into deliverable-local `Dependencies.csv` files.
> The local declarations and extractions remain the dependency evidence. A local
> departure from the accepted version (an added or removed edge, or a cycle)
> triggers a currency audit and a human decision to accept a successor version
> or reject the change. Until the human decides, affected deliverables are
> `DAG pending`. The accepted version governs only while it is current
> (D-GOV-49; SPEC §5.4).

| # | File : line | Now | Proposed |
|---|---|---|---|
| A1 | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md:52` | "Active dependency graph authority and approved edge context." | "The accepted current project DAG version and its edge context. It governs blockers only while current with local dependency evidence; departures are audited and decided by the human (D-GOV-49)." Keep the Boundary cell. |
| A2 | `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md:54` (Boundary cell) | "Local records do not supersede decomposition, DAG, review, or human approval authority." | "Local records do not supersede decomposition, review, or human approval authority. Local dependency files are the dependency evidence; a departure from the accepted DAG triggers a currency audit and a human decision, not a silent override either way." |
| A3 | same file, `:390-391` (optional) | "…decomposition truth, DAG authority, lifecycle state…" | "…decomposition truth, the accepted DAG version, lifecycle state…" |
| B1 | `execution/_DAG/_LATEST.md`, new line after `:8` | none | "- Authority reading: D-GOV-49. The accepted version governs only while current with local `_DEPENDENCIES.md`/`Dependencies.csv` evidence; departures go to a currency audit (`_Evaluation/DAGCurrency/`) and a human decision." |
| B2 | `execution/_DAG/_LATEST.md:12` | "…adopts DAG-011 dependency authority." | "…accepts DAG-011 as the current project DAG version." |
| B3 | `execution/_DAG/_LATEST.md:13` | "…`DAG-010/` dependency authority" | "…`DAG-010/` (accepted version, superseded)" |
| C | 85 deliverable `_DEPENDENCIES.md` files, human-owned `## Coordination Mode` bullet | "**Graph Authority:** `execution/_DAG/DAG-007/` is the current approved canonical graph authority." (stale: the current version is DAG-011) | "**Accepted project DAG:** the version named by `execution/_DAG/_LATEST.md`; it governs this deliverable's blockers only while current with this file and `Dependencies.csv` (D-GOV-49)." |
| D | `PKG-07_…/DEL-07-09_…/_DEPENDENCIES.md:4-5, 10-12` | "SYNCHRONIZED_FROM_DAG_010"; "synchronized mirror/evidence surface, not an independent graph authority" | Refresh with `materialize_local_dependencies.py --refresh-pointers`, which writes the D-GOV-49 authority statement. Per `NOTICE_2026-09-26_DEPENDENCY_FOLLOWUPS.md`, it also adds `TBD` mode and declaration placeholders. |

Leave unchanged:
- `:6` and `:7` of `_LATEST.md` (`approved_active_graph_authority`;
  `- Approved graph authority:`). Tools parse the `Approved graph authority:`
  key: `tools/practitioner_harness/adapter_project.py:242`, and Piping's
  `tools/coordination/list_deliverable_status.py:63` and
  `tools/release/check_release_readiness.py:54`. Renaming it is a separate tool
  change.
- `_LATEST.md:15-35`, which describe historical versions.

Files for row C, by line: line 4 in DEL-15-01 to DEL-15-04. Line 46 in DEL-07-08,
DEL-08-04 and DEL-16-03. Line 47 in DEL-07-01 and DEL-07-04. Line 48 in
DEL-07-03. Line 5 in the other 75 files. To list them:
`grep -n 'Graph Authority:\*\* `execution/_DAG/DAG-007/` is the current approved canonical graph authority' execution/PKG-*/1_Working/*/_DEPENDENCIES.md`.

Optional: project `AGENTS.md:45-47` ("the approved snapshot named by
`execution/_DAG/_LATEST.md`") already resolves through the pointer and is
acceptable. Changing it is an instruction change, which needs its own scope and
tranche manifest.

**Kept as written:** the DAG audits (`_DAG/DAG-005`, `-006`, `-007`
`DAG_Audit.md:13`; `DAG-010/DAG_Audit.md:15`), the DAG-001 to DAG-011
records, the D-56 packet and ruling, the SCA-011 snapshots and
`_PostAcceptanceValidation`, reconciliation records, earlier `loop/WORKPLAN_*`
files, `LOOP_RECEIPTS.md`, and the run notes and run history inside local
`_DEPENDENCIES.md` files.

**Instrument.** Add a new row, **D-78**, to the Piping decision register as the
D-56 successor. The register's residual-work convention keeps the D-56 row
immutable. The D-56 ruling itself says: "Any wording beyond the exact O-B
replacement or any wider instruction cleanup requires separate authority." D-78
would rule rows A to D, and optionally A3 and `AGENTS.md`. It can be ruled in
the same sitting as D-77 but kept as its own row, because it changes authority
wording, not decomposition. It changes no DAG version, pointer target, edge,
lifecycle or scope.
