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

**Amendment, 2026-09-18** — registry acts executing the owner's D-71 rulings
codified as `DEC-100`, `DEC-101` (ii), `DEC-102` and `DEC-105`
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12), in the owner-authorized
implementation tranche A1 (the visible rename), with the lint and the product
text changed in the same tranche: `BS-MATURITY` is retired (`DEC-105`;
`DEC-099` was superseded the same day and never executed); the `BS-ACCEPT`
placement clause is retired for product surfaces (`DEC-100`); the `BS-IP`
canonical text takes the product name SWBPIPE (`DEC-101` (ii)); §2 gains the
display-form table and its three rules (`DEC-102`). `DEC-081` is otherwise
unchanged. Ruled history keeps its wording (§5).

**Amendment, 2026-09-18 (`DEC-107`)** — registry act executing the owner's
direction extending `DEC-100`, codified as `DEC-107`
(`execution/_Decomposition/SOFTWARE_DECOMP.md` §12), in the owner-authorized
follow-up to tranche A1, with the emitted notices, the documents and the tests
that pinned the text changed together: the `BS-ACCEPT` canonical text and
every listed short variant are withdrawn from every emitted notice and from
the project's user-, contributor- and agent-facing documents, in addition to
`DEC-100`'s product surfaces, and nothing is put in their place; the texts
stay registered for the live `ScopeOfWork.md` files that already carry them;
§4 places `BS-ACCEPT` on no new artifact. The lint is not edited: it requires
the sentence nowhere. `DEC-081` is otherwise unchanged. Ruled history keeps
its wording (§5).

## 1. Boundary statements

### BS-IP — content/IP boundary

**Canonical:** "SWBPIPE ships no protected standards content. All
code-specific values, tables, allowables, and factors are supplied by the
user or user-controlled private sources, with provenance recorded."

**Short variants:** "no protected standards content; code-specific data is
user-supplied" · "user-supplied data with recorded provenance; no protected
standards content"

**Belongs on:** import/library/contribution UI, redaction/export surfaces,
contribution docs. **Not on:** results views (unless the surface genuinely
handles imported content).

**Former canonical text (2026-07-16 to 2026-09-18):** the same sentence with
the product's former name, "OpenPipeStress", as its first word; replaced by
`DEC-101` (ii). It is no longer a registered text. The short variants carry no
product name and are unchanged.

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

**Belongs on:** the live `ScopeOfWork.md` files that already carry it, as
existing placements only; there the canonical text and the listed short
variants remain registered statements. The PRD, the anchor document
`docs/PROFESSIONAL_BOUNDARY.md`, `docs/CONTRACT.md` and `docs/SPEC.md` state
the boundary in their own words, carry no registered text, and are untouched.

**Not on:** any product surface, any emitted notice, or the project's user-,
contributor- and agent-facing documents; and no new artifact (§4). The former
placement clause ("results, rule-check, comparison, solve, report-preview
surfaces; any place a computed outcome is shown") is retired for product
surfaces by `DEC-100`: no product surface (results, rule-check, comparison,
solve, report-preview, Review page, report body, inspector, canvas, agent
panel) carries the canonical text or any listed short variant, and the lint
reports one found in product source (`RETIRED_ACCEPTANCE_SENTENCE`).
`DEC-107` extends the withdrawal to the emitted notices — the rule-check run
notice, the rule-pack document validation notice and the report package's
first boundary note, which keep their first clause and the human-review
sentence — and to the project documents, where the rest of each sentence or
paragraph stays and nothing is put in the text's place. The lint is not edited
by `DEC-107`: it reports the texts in product source only and requires them
nowhere. On product surfaces the PRD §5.9 distinction is carried by the §2
display forms with their authority domains and by the PRD §19.3 report notice.

**Retires (unchanged, still enforced):** "not authoritative"-family phrasing
and multi-noun prohibition litanies on product surfaces.

**Untouched by `DEC-100`:** the PRD §19.3 required report notice; PRD §5.9,
§21.2 and §21.3; `docs/PROFESSIONAL_BOUNDARY.md`; `docs/CONTRACT.md`; the
control word "Accept" for a proposed edit.

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

### BS-MATURITY — release maturity (RETIRED 2026-09-18)

**Status:** retired by `DEC-105`, the owner's superseding ruling on D-71
item 1 (`DEC-099`, placement in the information popover and About, was ruled
earlier the same day, superseded, and never executed). No live surface carries
the sentence: not the product banner, footer, status bar, information popover
or About; not packaging or build-readiness surfaces; not user-facing
documentation; not the current design documents and frames. The lint no
longer requires it in the app shell source (the `MISSING_MATURITY_BANNER`
anchor is removed) and reports it when it appears on a scanned surface
(`RETIRED_MATURITY_SENTENCE`). The retirement is a display decision: the ruled
stage record from which the sentence was derived is unchanged, and no release,
lifecycle promotion or maturity statement follows from it. Ruled history that
contains the sentence is not rewritten (§5).

**Former canonical text (2026-07-16 to 2026-09-18), recorded here only:**
"Technical preview — not a released product."

**Former placement:** the app shell banner/footer; packaging/build-readiness
surfaces could reuse it.

**Belongs on:** nothing. There is no replacement statement.

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

### 2.1 Display forms (`DEC-102`)

The six automatic analysis statuses (`docs/TYPES.md` §4) and the two emitted
evidence-status labels take their displayed form from this table and from
nowhere else. Authority domains are those of `docs/SPEC.md` §4.3.

| Token | Display form | Authority domain (`docs/SPEC.md` §4.3) |
|---|---|---|
| `MODEL_INCOMPLETE` | Model incomplete | mechanics solve authority |
| `MECHANICS_SOLVED` | Mechanics solved | mechanics solve authority |
| `RULE_INPUTS_INCOMPLETE` | Rule inputs incomplete | user-rule-check authority |
| `USER_RULE_CHECKED` | User-rule checked | user-rule-check authority |
| `USER_RULE_FAILED` | User-rule failed | user-rule-check authority |
| `HUMAN_REVIEW_REQUIRED` | Human review required | human acceptance authority |
| `INTERNALLY_VERIFIED` | Internally verified | evidence status (this section); not one of the three §4.3 domains |
| `PROVER_CORRELATED` | Prover correlated | evidence status (this section); not one of the three §4.3 domains |

Rules:

1. A label is never shown without its token reachable in place.
2. A label is always shown with its authority domain.
3. No label exists outside the table: there is no seventh status, and
   `ENGINEER_ACCEPTED` stays reserved and has no display form.

The registered forms replace the product's earlier curated labels "Review
required" and "Inputs needed". The `docs/TYPES.md` §4 tokens and the
`human_review_required: const true` data obligation are unchanged. The short
word a surface uses to show a domain is a design-system matter, not registry
vocabulary.

`non-authoritative` (evidence-standing qualifier; owner-adopted by the
D-49 ruling, `DEC-082`) — marks external-run, parsed, or invented-fixture
evidence as admissible bounded technical context (regression, handoff
review) that does not close source-basis, code-compliance, or acceptance
questions (PRD §21.2). This is a source-basis standing label on governance
surfaces, distinct from the retired product-surface hedge; it never
appears on product surfaces.

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
`BS-ACCEPT` is placed on no new artifact (`DEC-107`): it stays registered only
for the live `ScopeOfWork.md` files that already carry it, and where it has
been withdrawn nothing is written in its place.
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
