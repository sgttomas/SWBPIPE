# D-48 - Claims-Language Taxonomy and Semantic Alignment

**Status:** PROPOSAL  
**Date prepared:** 2026-07-16  
**Decision ID:** D-48  
**Prepared by:** agent, from the owner's 2026-07-16 in-session direction following the DEC-080 validation-posture amendment  
**Owner act required:** yes

## 1. Decision Statement And Scope

Adopt a governed claims-language taxonomy: a small set of named boundary
statements with canonical texts and per-surface placement rules, replacing
the pervasive undifferentiated disclaimer litany ("not authoritative / no
professional judgment / no code-compliance claim / ...") across product UI,
docs, and live scopes of work — with positive capability framing, compaction
of the governance-internal litany on new artifacts, and a deterministic lint
so the drift cannot recur.

This packet does not: change any solver, rule-pack, or report *behavior*;
weaken the §19.3 report notice or §21 prohibited-claims table (they are
strengthened as anchors); edit ruled history; advance lifecycle or stage;
or create any release-readiness, professional-approval, certification,
sealing, authentication, or code-compliance claim.

## 2. Accepted Basis And Current Evidence

- **Owner direction (2026-07-16, in-session, verbatim):** *"There's a
  pervasive syntactic prevalence of 'not authoritative, not governance, not
  professional judgment, etc.' type of statements in the UI and in the docs
  and probably many scopes of work. But the whole idea is that this is
  supposed to be free of any IP it isn't allowed to have. Yet so much
  remains at its disposal. Other than that, the validation of 'truth' and
  'professional judgment' is done through proper solvers that ingests the
  output of OpenPipeStress as you've now described in the PRD and elsewhere.
  How would you approach better semantic alignment here?"* — followed, after
  the agent's diagnosis and proposed approach, by: *"Prepare the D-48
  packet."*
- **Diagnosis (agent, accepted as the packet's framing):** one blanket
  vocabulary is doing four different jobs — (1) the content/IP boundary
  (ship no protected standards content — the actual hard constraint),
  (2) the acceptance boundary (the software never performs the acceptance
  act), (3) the validation posture (truth-testing via external-prover
  correlation per `DEC-080` / PRD v0.3 §22.5), and (4) governance-internal
  claim calibration (the "no lifecycle advance..." litanies in packets,
  status files, and receipts). Collapsing 1–3 into one hedge makes results
  read as untrustworthy when the design intent is the opposite: real, open,
  classical mechanics; precise fences on content and acceptance only.
- **Measured prevalence (live tree at `dba189d83`):** "code-compliance
  claim" appears in ~1,366 files under `execution/` (≈925+ of them ruled
  history: run records, reconciliation snapshots, decision packets), 45
  live `ScopeOfWork.md` files, 16 live `_STATUS.md` files, 11 files under
  `apps/`, 29 under `core/`, 25 under `docs/`. "certification, sealing"
  shows the same shape (1,324 / 22 / 20 / 90). Representative product-facing
  examples: `apps/desktop/src/App.tsx:1458` ("Technical preview only: no
  production-readiness, release-readiness, certification, sealing,
  code-compliance; no licensed engineering reliance claim.") and
  `apps/desktop/src/features/rule-check/RuleCheckPanel.tsx:99` ("...is not a
  code-compliance, approval, certification, sealing, or professional
  acceptance result.").
- **Existing anchors (authority for the canonical texts):** PRD v0.3 §19.3
  required report notice — already the model: positive capability statement,
  precise non-acceptance sentence, and "validation in accepted professional
  tools" named; §21.1/§21.2 permitted/prohibited claims tables; §21.3 human
  acceptance separation; §15.6 future external result state; F-PIP-2 (loop
  claims fence); `DEC-080` (external-prover correlation as the principal
  validation posture).
- **Enforcement precedent:** the practitioner harness already lints its own
  emitted templates against `FORBIDDEN_CLAIM_WORDS`
  (`tools/practitioner_harness/test_claim_language.py`,
  `harness_common.find_claim_language`) — the product-side lint proposed
  here extends an existing pattern, not a new invention.

## 3. The Proposed Taxonomy (normative content of the ruling)

### 3.1 Named boundary statements and canonical texts

| ID | Boundary | Canonical text (one statement, used verbatim or with ruled variants) |
|---|---|---|
| **BS-IP** | Content/IP | "OpenPipeStress ships no protected standards content. All code-specific values, tables, allowables, and factors are supplied by the user or user-controlled private sources, with provenance recorded." |
| **BS-ACCEPT** | Acceptance/authority | "Results are engineering decision-support information. Acceptance, professional judgment, and any certification, sealing, or code-compliance determination remain with the responsible engineer and project authority." |
| **BS-VALID** | Validation posture | "Candidate designs are validated in the user's accepted professional tools (external-prover correlation, PRD §22.5). Internal benchmarks and rule checks are development verification and screening evidence." |
| **BS-MATURITY** | Release maturity | One sentence derived from the ruled stage/lifecycle record (e.g. "Technical preview — not a released product."), never a compound litany. |
| **GF-TOKEN** | Governance fence (agent-facing artifacts only) | "Standard claim fence applies (F-PIP-2; claims taxonomy per this ruling's `DEC` row)." |

The §19.3 report notice remains the authoritative composite for reports,
unchanged. "Not authoritative"-family phrasing is retired from product
surfaces in favor of BS-ACCEPT (which says *who holds judgment* instead of
vaguely demoting the output).

### 3.2 Evidence-status labels (per-artifact, replacing global hedging)

`INTERNALLY_VERIFIED` (internal benchmark/regression evidence) →
`PROVER_CORRELATED` (external-prover comparison recorded per §22.5 fields) →
`ENGINEER_ACCEPTED` (human acceptance record, separate per §21.3; not
implemented in MVP). Labels attach to results/reports/case pages; they align
with §15.6's future external result state and create no acceptance workflow
now.

### 3.3 Placement rules

| Surface | Carries | Must not carry |
|---|---|---|
| Report notice | §19.3 canonical notice (unchanged) | — |
| Results / rule-check / comparison UI | BS-ACCEPT (+ evidence-status label where results shown) | BS-IP; governance litany; "not authoritative" phrasing |
| Import / library / contribution UI and docs | BS-IP | BS-ACCEPT duplication |
| Handoff / export UI; validation manual | BS-VALID | compound litany |
| App shell banner | BS-MATURITY | compound litany |
| New governance artifacts (SOWs, packets, status entries, receipts) | GF-TOKEN | eleven-noun restatements |
| Ruled history (~925+ files) | untouched | — |

One surface, one boundary (plus the report notice as composite). Capability
is stated affirmatively; each boundary appears where it applies and nowhere
else.

### 3.4 Registry and lint

- A governed registry file `docs/claims_registry.md` holds the canonical
  texts, IDs, placement rules, and permitted short variants, citing PRD
  §19.3/§21 and the `DEC` codification as authority. No PRD text change is
  required — the registry derives from the PRD, and §21 stays the claims
  authority.
- A deterministic validator (new `tools/validation/validate_claims_language.py`,
  patterned on the harness lint) checks: (i) product UI strings and docs for
  ad-hoc disclaimer-family phrasing not matching a registry entry;
  (ii) presence of the required statement per placement rule on the named
  surfaces; (iii) new governance artifacts for litany restatements where
  GF-TOKEN belongs. Wired into repo-wide `self-check` and the DEC-025 sweep
  surface it fits; ruled-history paths excluded.

## 4. Options

### O-A - Adopt the taxonomy and align all live surfaces (recommended)

Adopt §3 in full and execute in two waves:

- **Wave 1 — product surfaces (code-touching):** UI strings (`apps/desktop`,
  ~12 files incl. the App.tsx banner and panel notices), report/notice
  strings in `core/` crates (verify the emitted notice matches §19.3),
  `docs/` user-facing pages (user guide, validation manual, README family).
  Because this touches executable surfaces, the `DEC-025` five-surface sweep
  and the H4 desktop evidence posture (Playwright e2e, Vitest) apply; e2e
  specs asserting current disclaimer text are updated with the canonical
  texts in the same tranche.
- **Wave 2 — live governance surfaces (docs-only):** the 45 live
  `ScopeOfWork.md` and 16 `_STATUS.md` litany occurrences are replaced with
  GF-TOKEN (plus the specific boundary statement where one genuinely
  applies); authoring templates and agent instructions updated so new
  artifacts are born compliant.

An audit inventory precedes each wave (multi-agent sweep classifying every
occurrence keep / substitute-canonical / delete), and the lint lands with
Wave 1 so regressions fail checks immediately.

Pros: fixes the semantics everywhere they are live; enforcement prevents
re-accretion; product text becomes accurate and confident.  
Cons: largest diff; Wave 1 requires the full code-touching evidence battery.

### O-B - Product surfaces only

Adopt §3 but execute only Wave 1 (UI, core report strings, user-facing
docs) plus the lint scoped to product surfaces. Live SOW/STATUS litanies and
governance templates stay as they are.

Pros: fixes what users see with a smaller program.  
Cons: the governance litany keeps regenerating the tone (SOWs seed UI text
via authoring agents), so the alignment erodes; the 45+16 live files stay
semantically stale.

### O-C - Defer

Keep current language everywhere.

Pros: no change risk.  
Cons: the product keeps hedging capability it legitimately has, misstating
the design's actual boundaries, in ~100 live product/doc files and every
new artifact.

## 5. Constraints Carried By Every Option

- PRD §19.3 notice text and §21.2 prohibited-claims table are anchors: the
  alignment may relocate and deduplicate them, never weaken them. Anything
  the threat model (`docs/security/threat_model.md`), PKG-01, or DEL-16-04's
  FR-AGENT-005 professional-boundary hard gate relies on maps to a taxonomy
  entry rather than being deleted.
- F-PIP-1/2 remain in force verbatim; GF-TOKEN references F-PIP-2, it does
  not replace it.
- Ruled history is never rewritten; the lint excludes it by path.
- No acceptance workflow is created (§21.3 MVP posture unchanged);
  `ENGINEER_ACCEPTED` is a reserved label, not a feature.

## 6. Non-Binding Recommendation

Select **O-A**. The owner's stated intent is semantic alignment of the whole
surface, the two-wave split contains the code-touching risk to Wave 1's
evidence battery, and the lint is what makes the fix durable — O-B leaves
the source of the accretion (governance authoring habits) in place.

## 7. On-Ruling Mechanism

- Append the ruling to this packet; flip the D-48 register row; codify as
  the next free `DEC-XXX` row in `execution/_Decomposition/SOFTWARE_DECOMP.md`
  §12 (carrying the §3 taxonomy verbatim or by reference).
- On O-A/O-B: execute as governed loop work (not an SCA — no PRD text or
  scope item changes): first the audit inventory, then Wave 1 as a
  code-touching tranche under the DEC-025 sweep + H4 desktop evidence
  posture, then (O-A) Wave 2 as a docs-only tranche; registry + lint land
  with Wave 1. Multi-agent execution per root `AGENTS.md` with an AgentRuns
  record, mirroring the SCA-007 run shape.
- Deliverable homes: the registry and lint attach to existing deliverables
  (candidate: DEL-16-04 professional-boundary controls for the taxonomy;
  DEL-08-01/DEL-09-04 for report/manual surfaces; per-deliverable `_STATUS.md`
  history lines at execution). No new package or deliverable is created.
- Branch-first + PR per wave; the owner merges unless session merge
  authority is granted with the ruling.
- If any Wave-1 string change alters behavior asserted by tests beyond text
  (unexpected), that item returns to the owner rather than being absorbed.

## 8. Current Gateway

`AWAITING_RULING`. No UI, doc, SOW, template, registry, or lint change is
authorized by this proposal.
