VERDICT: ACCEPT WITH CONTESTED ROWS

# PKG-12 verification: wave W3, STANDARD sampling

- Verifier: a fresh, evidence-only TASK (Type 2). Parent: HELP_HUMAN Agent 0, run
  `HELP-HUMAN-PIPING-20260921-RECONCILIATION`. I am independent of the PKG-12
  manager and its workers G1 and G2.
- Brief: `briefs/R2-VERIFIER_brief.md`. Its SHA-256
  `47fb3c5225ac0a3c532d99a53ee3211129998f887dc9187b40ab898bb2dd5b00` was
  checked and matches.
- Evidence: the read-only freeze at `00115c71931bcae79909602d653740d3bb72dfa1`.
  I checked it with `git rev-parse`.
- Rules applied: `CONVENTIONS.md` (Parts A–F, Part F included),
  `CANONICAL_SITUATIONS.md`, `AUTHORITY_AND_SOURCE_RELIABILITY_MAP.md`, and
  `WAVES/W1/RESOLUTIONS.csv`, used for precedent only.
- Seals: for all five forward ledgers, the recomputed SHA-256 equals the SEAL
  record (DEL-12-01 `e2019839…`, 02 `b24c5042…`, 03 `b7869d18…`, 04 `85e41a75…`,
  05 `fc28af0d…`).
- The dispositions below are agent judgments, not owner rulings.

## 1. Verdict basis

- No deliverable's firm error rate on sampled rows exceeds 10%. The highest is
  3.4%, on DEL-12-03.
- No firm error falls in a 100%-sampled class, so no firm error in such a class
  changes a tier or owner routing. The rerun rule is therefore not triggered.
- **The package-level firm false-alignment rate is 4/42 = 9.5%.** That is above
  the 5% scale-out gate (see §3 and §7).
- All four firm errors are the same kind of mistake: an `ALIGNED` principle or
  requirement row whose substance holds only by absence, or is already carried
  as a gap on a sibling row. That is a Part F1 / CP-11 / F7 application
  failure. It is not an evidence failure.

## 2. Per-deliverable tables

Sample classes, following the brief's sampling table:

- **A100:** `INVARIANT`, `UNKNOWN`, `ACCEPTED_DIVERGENCE`, `AUTHORITY_CONFLICT`
  or `LIFECYCLE_REASSESSMENT_REQUIRED` rows, and rows carrying
  `PROTECTED_CHECK` or `FROZEN_CONTRACT`. No deliverable is ISSUED.
- **SH100:** the unit has `SharedTextCount > 1`. A `.sNN` row inherits its
  parent's count.
- **F2F4:** an `ALIGNED` row with `GAP_WORDING_CHECKED:` or `OPEN_ACTION:`.
- **PC25:** an `ALIGNED` row with `PRODUCT_CALLER: NONE`.
- **NA25:** other non-aligned rows.
- **N20:** other `ALIGNED` normative rows.
- **S10:** structural rows and inherited canonical rows.

Selection method: within each deliverable and class, candidate keys were
sorted by SHA-256 of the full `ClaimKey`. I took the lowest ceil(rate × n).
The list is deterministic and was not weighted by confidence. The brief's
sort rule governs over its "weighted toward" wording.

"False-align rate" means firm false alignments among the sampled `ALIGNED`
normative rows (`REQUIREMENT`, `ACCEPTANCE` or `EXCLUSION`) in any class.

| DEL | Rows | Sampled (A100/SH100/F2F4/PC25/NA25/N20/S10) | Firm | Weak | Field | Sampled ALIGNED normative | Firm false-align rate |
|---|---|---|---|---|---|---|---|
| DEL-12-01 | 138 | 36 (2/10/2/0/10/7/5) | 1 | 0 | 0 | 9 | 1/9 = 11.1% |
| DEL-12-02 | 147 | 40 (3/12/1/0/11/7/6) | 1 | 1 | 0 | 9 | 1/9 = 11.1% |
| DEL-12-03 | 98 | 29 (3/8/3/0/5/6/4) | 1 | 0 | 1 | 6 | 1/6 = 16.7% |
| DEL-12-04 | 113 | 39 (8/8/2/7/9/2/3) | 1 | 0 | 4 | 7 | 1/7 = 14.3% |
| DEL-12-05 | 158 | 41 (0/17/0/0/7/11/6) | 0 | 1 | 0 | 11 | 0/11 = 0% |
| **PKG-12** | 654 | 185 | 4 | 2 | 5 | 42 | **4/42 = 9.5%** |

Firm error rate on all sampled rows:

| DEL | Firm error rate |
|---|---|
| DEL-12-01 | 2.8% |
| DEL-12-02 | 2.5% |
| DEL-12-03 | 3.4% |
| DEL-12-04 | 2.6% |
| DEL-12-05 | 0% |

The cross-deliverable cause conflict on `CONTEXT#architecture-basis-injection.s01`
is reported once, in §5. It is not counted per row.

## 3. Package-level firm false-alignment rate

**9.5%** (4 of 42 sampled `ALIGNED` normative rows). The scale-out gate is 5%
or less, so this package fails it.

The errors do not come from misread evidence. In every case the worker's own
Notes name the unmet element, or point to a sibling row that carries it. §4
lists out-of-sample rows with the same pattern.

## 4. Disagreements

### Firm

**F-1: DEL-12-01:SOW#CLM-024.r06** (N20). The "No-bypass adapters" principle.

- **Row says:** `ALIGNED`. Notes: "Guidance principle; its runtime coverage
  is assessed on LFSP-REQ-010."
- **Found:** the principle (SOW L357) has the same substance as LFSP-REQ-010
  (SOW L160): import/export, plugins and private-library adapters pass through
  the validation, unit, provenance, diagnostic and boundary controls.
  - The ledger disposes LFSP-REQ-010 as `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE ·
    INVARIANT` because the plugin clause holds only by absence (CP-11) and the
    no-bypass tests are future work.
  - F1: "even when the same gap is also recorded on another row", a row that
    records an unmet element is not `ALIGNED`.
  - This follows the W1/W3 precedent for deferral to a sibling row
    (`RESOLUTIONS_DRAFT_PART1.csv`, DEL-05-01 VER-001).
- **Right values:** `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
  IP_DATA`. It shares LFSP-REQ-010's gap, with AuthorityNeeded `OWNER` (plugin
  runtime not authorized).

**F-2: DEL-12-02:SOW#CLM-027** (N20). The principles block, assessed whole.

- **Row says:** `ALIGNED`. Notes: "the no-bypass principle's runtime coverage
  is assessed on REXC-REQ-012."
- **Found:** the "No-bypass exports" principle (SOW L421) covers GUI, CLI,
  adapters, plugins, public APIs and downstream handoffs, the same subject as
  REXC-REQ-012 (SOW L195).
  - The ledger disposes REXC-REQ-012 as `PARTIALLY_IMPLEMENTED · INVARIANT`
    because the plugin clause holds only by absence (CP-11).
  - Because the block was not split, the gap part decides the whole row. The
    same worker handled DEL-12-02 CLM-004 exactly this way ("same gap as
    REXC-REQ-012 (F1)").
- **Right values:** `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
  IP_DATA`, AuthorityNeeded `OWNER`.

**F-3: DEL-12-03:SOW#CLM-011/TEL-REQ-006** (N20). "Turning telemetry on shall
require an affirmative user action distinct from ... terms, install, open,
solve".

- **Row says:** `ALIGNED`. Notes: "No path turns telemetry on."
- **Found:** the SOW row itself (L156) records that the PDU-042 action "does
  not turn telemetry on, grant consent, approve an allowlist, mutate product
  config". The panel `apps/desktop/src/features/telemetry/TelemetryBoundaryPanel.tsx`
  (L171–173, `request_telemetry_enablement_review`) only records a request.
  - The governed behaviour, enabling telemetry, does not exist. The claim is
    satisfied only by that absence, so C6(a) and CP-11 apply.
  - A distinct request control exists, so this is a bounded slice. It is not
    zero.
- **Right values:** `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE`.
  - Tier: under F8 the gap touches the telemetry privacy subject (OPS-K-PRIV-2),
    which gives `INVARIANT · SECURITY`, consistent with TEL-REQ-009/-010.
    `LOCAL_DESIGN · RECORD` is the defensible alternative.
  - AuthorityNeeded `OWNER`, because enablement is not authorized.

**F-4: DEL-12-04:SOW#CLM-024** (PC25). Principles, including "Fail closed at
boundaries: imports, exports, report generation, bug reports, telemetry, plugin
access and public contribution flows should block, redact, warn or quarantine"
(SOW L383) and "No-bypass architecture" (L385).

- **Row says:** `ALIGNED`, `PRODUCT_CALLER: NONE`. Notes: "Principles are
  design guidance; delivery at product boundaries is assessed on the CLM-011
  requirement rows."
- **Found:** these principles are claims about product boundaries, not about
  the helper.
  - F7: a helper with no product caller does not satisfy a claim about
    app behaviour. The worker confirms the missing caller:
    `core/security/secret_private_library/controls.py` is referenced by the
    desktop panels only as strings.
  - The bug-report path does not exist.
  - The CLM-011 rows the Notes defer to (R1, R4–R8, R10) are all
    `PARTIALLY_IMPLEMENTED` (F1).
- **Right values:** `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · NONE ·
  IP_DATA`. AuthorityNeeded `OWNER`, because the PDU-034 quarantine policy and
  the plugin grant model are held.

### Weak

**W-1: DEL-12-02:SOW#CLM-037/REXC-OI-002** (NA25). The F3 origin.

- **Row says:** `STALE_SETUP_SPECIFICATION` ("Setup-era text (7bee9ae41)").
- **Found:** the item title "Export context UI and override flow" is present at
  `7bee9ae41`.
- However, the TBD statement being assessed ("Exact GUI controls, CLI prompts,
  user confirmation, and approval workflow are not selected here") first appears
  at `586d3f10d` (2026-06-07), per `git log -S` on the freeze.
- Under a text-level reading of F3 the class is `STALE_REVIEW_OR_EVIDENCE`.
  F3 does not say whether origin is judged by the item's identity or by the
  assessed sentence. The conventions should settle that.

**W-2: DEL-12-05:SOW#CLM-033.r02** (NA25). The explicit-disclosure principle
against DEC-051.

- **Row says:** `STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING ·
  PROJECT_BASELINE`.
- **Found:** DEC-051 amended `docs/CONTRACT.md` OPS-K-PRIV-1 (L41) and added
  `IP_AND_DATA_BOUNDARY.md` §6.1 (L94). The ruling redirected the governing
  sources, so this is not an `AUTHORITY_CONFLICT`, and the disposition and
  cause are right.
- The gap is between the SOW's principle and a contract invariant on privacy.
  F8 therefore points to `INVARIANT` (layer `SECURITY`).
- `PROJECT_BASELINE`, "conflicts with a ruled decision", is also a literal fit.
  The conventions do not rank the two when a ruling amends an invariant.

### Field (the disposition stands; a field is wrong)

**D-1: DEL-12-04:SOW#CLM-011.r01** (A100). R1: private libraries and rule packs
are stored outside public example directories by default and marked private.

- **Wrong in the row:** the Notes say "No product feature stores private
  libraries or rule packs". The RemainingWork asks for product storage.
- **What exists:**
  - `apps/desktop/src-tauri/src/lib.rs::save_local_library` (L3676) stores
    accepted library imports in the app-local SQLite store, validated with
    `intended_visibility` "private".
  - `save_local_rule_pack` (L3270) stores rule packs in the same store.
- **Why the disposition stands:** the store does not force private
  classification on a rule pack. Rule packs keep the document's own
  `/classification/privacy_class`, and drafts with findings remain saveable.
  So "marked private in metadata" is not enforced for rule packs, and
  `PARTIALLY_IMPLEMENTED` still holds, for that reason.
- **Correct:** ImplementationEvidence (add `lib.rs::save_local_library` and
  `lib.rs::save_local_rule_pack`), VerificationEvidence (add
  `lib.rs::local_library_store_round_trips_accepted_import_per_project`),
  Notes and RemainingWork. The worker disclosed this error itself in
  `DEL-12-04_notes.md` L105–119 and in its return. The same Notes error also
  sits on the out-of-sample rows CLM-004.r01 and CLM-027.

**D-2: DEL-12-04:SOW#CLM-011.r06** (A100). R6: quarantine of suspected
protected content.

- **Right:** `PARTIALLY_IMPLEMENTED · INVARIANT` is correct, because routing to
  human review is absent and PDU-034 is held.
- **Missing from the evidence:** the product ingestion stop. The ledger omits
  `lib.rs::save_local_library`, which refuses QUARANTINE outcomes (L3689–3706),
  and its test `local_library_store_gate_admits_accepted_and_refuses_quarantined_imports`.

**D-3: DEL-12-04:SOW#CLM-011.r05** (NA25). R5: export, report, bug-report and
shared-model paths warn and redact private values by default.

- **Row says:** tier `PROJECT_BASELINE`, layer `SECURITY`.
- **Should be:** `INVARIANT`, layer `IP_DATA` (C3 covers "security and privacy",
  and under F8 the gap touches private-data exposure).
- In the same package, DEL-12-02 REXC-REQ-010/-012 carry the same export-privacy
  subject at `INVARIANT · IP_DATA`.

**D-4: DEL-12-04:SOW#CLM-011.r07** (NA25). R7: plugins and adapters are denied
private-library, filesystem and network access without a grant.

- **Row says:** tier `PROJECT_BASELINE`.
- **Should be:** `INVARIANT` (sandbox and privacy subject, CP-11 gap). This
  matches DEL-12-01 LFSP-REQ-010 and DEL-12-03 TEL-REQ-009, both `INVARIANT`.

**D-5: DEL-12-03:SOW#CLM-013/TEL-TEST-005** (N20). The disposition `ALIGNED` is
right: the claim concerns a test of the policy engine. But the only
implementation evidence is `core/security/telemetry_policy/controls.py`, which
has no product caller. The desktop panels name it only as a string (for
example `TelemetryBoundaryPanel.tsx` L134, L194). F7 therefore requires
`PRODUCT_CALLER: NONE` in Notes, and it is missing.

Out-of-sample rows with the same omission:

- DEL-12-03 CLM-005.r02
- DEL-12-03 TEL-TEST-001
- DEL-12-03 CLM-021.r03
- DEL-12-03 CLM-021.r08

### Out-of-sample rows Agent 0 should see (same pattern as the firm errors, not counted)

- **DEL-12-02:SOW#CLM-033:** a guidance principle `ALIGNED`, with Notes
  "Runtime plugin coverage is assessed on REXC-REQ-012". This is the F-2
  pattern.
- **DEL-12-03:SOW#CLM-026:** `ALIGNED`, with Notes "Runtime consumer routing is
  assessed on TEL-REQ-009". This is the F-1 pattern if the clause concerns
  runtime routing.
- **DEL-12-03:SOW#CLM-011/TEL-REQ-003:** "explicit human/security approval of
  the event allowlist before collection" is `ALIGNED`, but no collection exists.
  This is the F-3 pattern (CP-11).

### Rows checked and agreed (selected notes)

- **CP-10 root finding:** `app_store_path` resolves `app_local_data_dir` and
  creates it (`lib.rs` L548–553), and `docs/SPEC.md` §4.4 still has OS roots
  as TBD. The FG-DEL-12-01-01 rows (LFSP-REQ-005, CLM-004.r03) are right as
  `IMPLEMENTED_DIFFERENTLY · AUTHORITY_UNCLEAR · PROJECT_BASELINE · OWNER`.
- **DEL-12-01 CLM-005:** the policy and guard use the eleven-class vocabulary
  (`controls.py` L51–60), so `IMPLEMENTED_DIFFERENTLY · DOC_BEHIND_CODE` is
  correct.
- **CP-09 rows (VER-001 in 01, 03 and 04):** all have AnyPassMatchesFrozen=NO
  and are correctly `EVIDENCE_OVERTAKEN`.
- **DEL-12-02 CLM-019:** the `CANONICAL_DEPARTURE` (F3 origin class for a
  setup-era file pointer) is valid. F3 exempts only keyed CS rows and revision
  pins.
- **DEL-12-04 CF-001 and CF-002:** `UNKNOWN · AUTHORITY_UNCLEAR` with an owner
  next step is correct. PRD open question 11 asks the question; no ruling
  answers it.
- **DEL-12-03 TEL-REQ-009 and -010 and TEL-TEST-006:** `INVARIANT` with the
  CP-11 plugin reasoning is correct.
- **F2 Remaining rows:** DEL-12-03 R01 and R02 with `OPEN_ACTION` pointing to a
  PARTIAL governing row are correct. DEL-12-03 R03 takes its own gap
  disposition, also correct. DEL-12-04 R01's `OPEN_ACTION` points to
  CLM-011.r06, which is PARTIAL, so it is correct.
- **GAP_WORDING_CHECKED rows:** the clauses on DEL-12-01 LFSP-REQ-006 and
  CLM-003.r08, DEL-12-02 REXC-REQ-002, DEL-12-03 CLM-029 and DEL-12-04
  CLM-013.r06 each justify their wording.
- **DEL-12-05 threat rows (STM-T-002, -003, -010, STM-REQ-006):** they match
  `docs/security/threat_model.md` L92–100.

## 5. Batch consistency and shared situations

- **Validator:** `validate_ledger_v2.py`, run by me against the freeze.
- **Single mode:** run with `--reverse`, `--inventory ROUTING/PKG-12_capabilities.csv`
  and `--notes-gap`. All five pass with 0 findings (138, 147, 98, 113 and 158
  rows).
- **Batch mode (5 ledgers):** `PASS batch of 5 ledgers: 0 consistency
  findings`. No `WAVES/W3/RESOLUTIONS.csv` existed.

**Shared-situation conflict (not seen by batch mode, because `.sNN` keys carry
no BodySHA256):**

- **Key:** `CONTEXT#architecture-basis-injection.s01` ("PKG-00 at SEMANTIC_READY
  supplies architecture-basis constraints").
- **Causes:** DEL-12-01, -02 and -03 (G1) use `SCOPE_REDIRECTED_BY_RULING`, with
  FindingGroups. DEL-12-04 and -05 (G2) use `RECORD_DRIFT`, with no
  FindingGroup.
- **Everything else agrees:** disposition `STALE_REVIEW_OR_EVIDENCE`, tier
  `LOCAL_DESIGN` and layer `RECORD` are the same on all five.
- **Precedent:** this is the corpus-wide contested cluster SR-1 already recorded
  in `WAVES/W1/RESOLUTIONS.csv` (DEL-07-xx rows, OWNER_CONFIRMED on the
  disposition, cause left for one R3 resolution). It is weak, and needs no
  per-row resolution beyond joining SR-1.
- `.s02` agrees across all five.

## 6. Reverse pass

- **Coverage:** checked 100% of the `CLAIMED_BY`, `PARTIAL`, `UNKEYED` and
  `CONSTRAINS` answers (19 `CLAIMED_BY`, 5 `PARTIAL`, 1 `UNKEYED`, 0
  `CONSTRAINS`) and 186 NOT_MINE rows (10% per deliverable, hash-selected).
- **F5:** checked on 24 NOT_MINE rows, 20% of each deliverable's NOT_MINE rows
  whose EntryPoints hit a path the ledger cites (DEL-12-01 9 of 42, DEL-12-02 9
  of 43, DEL-12-03 2 of 7, DEL-12-04 2 of 8, DEL-12-05 2 of 9). Every checked
  reason names the capability and says why the shared path is cited. The G1
  reasons share a stem but quote the capability text each time. That meets F5.
- **Manifest:** all 381 routed IDs resolve through
  `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv` (352 AREA, 29 SAMPLE).

**Capabilities claimed by more than one of the package's deliverables:**

- **RC-12-0265 (CAP-FEATB-019)**, `enforceLocalFirstRoute`/`controlRouteExport`
  in `redactionExportControls.ts`, is `PARTIAL` for DEL-12-01 (local-first
  admission) and for DEL-12-02 (route allow-list and lossy-redaction
  withholding). The split is complementary and plausible, but R3 should record
  the boundary.
- All other overlaps pair a single `CLAIMED_BY` with other deliverables'
  `COVERS`, which is consistent:
  - RC-12-0060, -0080, -0152, -0182 and -0192: DEL-12-04 and DEL-12-05 cite
    sibling code without claiming it.
  - RC-12-0062: `PARTIAL` DEL-12-01, `COVERS` DEL-12-02.

**Suspected missed claims:**

- **RC-12-0362 (CAP-SHELL-038, AREA; the local rule-pack store,
  `lib.rs::save_local_rule_pack` and siblings):** DEL-12-04 answered NOT_MINE,
  on the ground that it is outside its scope. DEL-12-04 R1 names "rule packs",
  and the worker answered `COVERS` for the parallel library store RC-12-0177. A
  `COVERS DEL-12-04:SOW#CLM-011.r01` is missing. This is coverage, not
  ownership.
- **RC-12-0379 (SecretPrivateLibraryPanel, `deliverable_id` DEL-12-04):**
  `UNKEYED` against CLM-010 is right. No issued key names the desktop surface.
  The panel hard-codes its references and decisions. R3 should decide whether
  it is a DEL-12-04 product surface.
- No product owner for the private-library and rule-pack stores appears in this
  package. They are library-import and store work (DEL-03-07 / DEL-02-05 areas),
  so this is not an ownership gap here.

**Answer distribution, sampled (SAMPLE) rows against area (AREA) rows:**

| DEL | AREA non-NOT_MINE | SAMPLE non-NOT_MINE |
|---|---|---|
| DEL-12-01 | 13/352 | 0/29 |
| DEL-12-02 | 22/352 | 0/29 |
| DEL-12-03 | 7/352 | 0/29 |
| DEL-12-04 | 14/352 | 0/29 |
| DEL-12-05 | 3/352 | 0/29 |

- The 29 SAMPLE rows come from VIEW, PHYS, SOLVER, WSUI and COREB. None
  concerns storage, privacy, telemetry, secrets or threat modelling, so 0/29 is
  the expected result.
- Nine SAMPLE rows overlap package-cited paths, but only through generic files
  (`App.test.tsx`, `lib.rs`), not a declared deliverable path. I found no
  anchored answer.
- One minor inconsistency: RC-12-0277 (WSUI local project UI) is NOT_MINE for
  DEL-12-01, while the AREA row RC-12-0004 (the Rust commands for the same
  store) is `COVERS`. A `COVERS` omission does not affect ownership.

## 7. What the owner must see

1. **Package false alignment is 9.5%, above the 5% scale-out gate, although no
   deliverable needs a rerun.**
   - All four firm errors are the same class: principle or gating rows marked
     `ALIGNED` while the worker's own Notes send the unmet element (plugin
     no-bypass by absence, telemetry enablement that does not exist, helper
     with no product caller) to a sibling `PARTIALLY_IMPLEMENTED` row.
   - Three more out-of-sample rows follow the pattern (§4).
   - Recommendation: record F-1 to F-4 in `WAVES/W3/RESOLUTIONS.csv` (F6) and
     state explicitly in the next worker briefs that principle and guidance
     rows follow F1, CP-11 and F7 like requirement rows.
2. **The sealed DEL-12-04 Notes are factually wrong on three rows.** They say
   no product private-library or rule-pack store exists; `save_local_library`
   and `save_local_rule_pack` exist. The worker disclosed this. The
   dispositions still stand (D-1), but R3 must read those rows with this
   correction.
3. **Items already routed to R4 by the workers, which I confirm:**
   - The desktop project-store root was settled in code with no ruling
     (FG-DEL-12-01-01, CP-10).
   - DEC-051's model-provider channel against the SOW's explicit-disclosure
     principle (FG-DEL-12-05-02). Governing sources were amended by DEC-051, so
     this is ruling-redirected text and not an authority conflict. The tier is
     contested (W-2).
   - CF-001 and CF-002 (secret provider and encrypted-storage default) are
     `UNKNOWN` pending an owner ruling.
4. **Boundary disclosure:** the manager's return records that G1 wrote scratch
   files to the shared session scratchpad before the scratch-path clause
   existed, and that a PKG-15 file of the same name overwrote one of them. I
   found no scratch residue in `WAVES/W3/PKG-12/`. This verification read the
   freeze only and wrote only this file and its scratch folder, which I then
   deleted.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
