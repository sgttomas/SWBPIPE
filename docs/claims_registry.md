# Claims Registry — Governed Boundary-Statement Vocabulary

**Authority:** D-48 Option O-A, ruled 2026-07-16 and codified as `DEC-081`
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12); derives from PRD v0.3
§19.3 (required report notice), §21.1/§21.2 (permitted/prohibited claims),
§21.3 (human-acceptance separation), §22.5 (external-prover validation
posture, `DEC-080`), and F-PIP-2. This registry is the single source for
boundary language on product and live governance surfaces. It creates no
lifecycle, release, professional-approval, certification, sealing,
authentication, or code-compliance claim.

**Enforced by:** `tools/validation/validate_claims_language.py` (repo root),
registered as self-check GEN-13; violations block closeout.

## 1. Boundary statements

### BS-IP — content/IP boundary

**Canonical:** "OpenPipeStress ships no protected standards content. All
code-specific values, tables, allowables, and factors are supplied by the
user or user-controlled private sources, with provenance recorded."

**Short variants:** "no protected standards content; code-specific data is
user-supplied" · "user-supplied data with recorded provenance; no protected
standards content"

**Belongs on:** import/library/contribution UI, redaction/export surfaces,
contribution docs. **Not on:** results views (unless the surface genuinely
handles imported content).

### BS-ACCEPT — acceptance/authority boundary

**Canonical:** "Results are engineering decision-support information.
Acceptance, professional judgment, and any certification, sealing, or
code-compliance determination remain with the responsible engineer and
project authority."

**Short variants:** "Acceptance, professional judgment, and any
certification, sealing, or code-compliance determination remain with the
responsible engineer and project authority." (the canonical second sentence,
usable standalone) · "acceptance and professional judgment remain with the
responsible engineer" · "human review remains required; acceptance stays
with the responsible engineer" · "decision-support information for review by
the responsible engineer"

**Belongs on:** results, rule-check, comparison, solve, report-preview
surfaces; any place a computed outcome is shown. **Retires:** "not
authoritative"-family phrasing and multi-noun prohibition litanies on
product surfaces — BS-ACCEPT states who holds judgment instead of demoting
the output.

### BS-VALID — validation posture

**Canonical:** "Candidate designs are validated in the user's accepted
professional tools (external-prover correlation, PRD §22.5). Internal
benchmarks and rule checks are development verification and screening
evidence."

**Short variants:** "validation occurs in the user's accepted professional
tools; this package is screening and handoff evidence" · "handoff evidence
for external validation, not a validation outcome"

**Belongs on:** handoff/export UI (MBF, PCF, stress-neutral, native package,
external-prover surfaces), validation manual, headless-runner evidence
surfaces.

### BS-MATURITY — release maturity

**Canonical:** "Technical preview — not a released product." (One sentence,
derived from the ruled stage record; updated only when the stage record
changes. Never a compound litany.)

**Belongs on:** the app shell banner/footer; packaging/build-readiness
surfaces may reuse it.

### GF-TOKEN — governance fence token (agent-facing artifacts only)

**Canonical:** "Standard claim fence applies (F-PIP-2; claims taxonomy
per DEC-081)."

**Belongs on:** new governance artifacts — scopes of work, packets, status
entries, run records, receipts — wherever an author would previously have
restated the claims litany. The F-PIP-2 fence text itself and OPS-K-AUTH-1
(`docs/CONTRACT.md`) are the governing definitions and are never edited or
replaced by this token. Never used on product surfaces.

## 2. Evidence-status labels

`INTERNALLY_VERIFIED` — internal benchmark/regression/rule-check evidence ·
`PROVER_CORRELATED` — an external-prover comparison recorded with the PRD
§22.5 evidence fields · `ENGINEER_ACCEPTED` — reserved for a future separate
human-acceptance record per §21.3; no acceptance workflow exists in MVP.
Labels attach to individual results, reports, and case pages instead of
global hedging.

## 3. Composite anchors (authoritative, never weakened)

- **PRD §19.3 report notice** — the required composite for generated
  reports; `docs/report_notice_template.md` is its template home; the report
  renderer emits it. Changes require a PRD-level owner act.
- **PRD §21.2 prohibited-claims table** — the claims authority this registry
  implements.
- `docs/PROFESSIONAL_BOUNDARY.md` (BS-ACCEPT anchor doc) and
  `docs/IP_AND_DATA_BOUNDARY.md` (BS-IP anchor doc) — policy depth behind
  the one-line statements.

## 4. Authoring directive (agents)

When an artifact needs boundary language, use the applicable registry
statement (or a listed short variant) verbatim — one surface, one boundary.
Do not compose ad-hoc prohibition lists; do not restate F-PIP-2 or
OPS-K-AUTH-1 as a litany — cite GF-TOKEN. Deliverable-specific exclusions
("does not implement a parser/harness/API...") are scope statements, not
claims language, and stay as ordinary prose. Structured metadata fields
(e.g. `professional_boundary` booleans, `software_makes_certification_claim`)
are data contracts, not prose — they are outside this registry. Receipts may
use GF-TOKEN in boundary lines going forward; the D-44 receipt contract is
unchanged.

## 5. Out of scope / exclusions

Ruled history (run records, reconciliation and scope-change snapshots,
ruled decision packets, `plans/**`, `LOOP_RECEIPTS.md` entries,
`docs/_history/**`) keeps its original wording permanently. The PRD, this
registry, the anchor docs, `docs/CONTRACT.md`, the threat model, and the
loop fence definitions are allow-listed authorities for the lint.
