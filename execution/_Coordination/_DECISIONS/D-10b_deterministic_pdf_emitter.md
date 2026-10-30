# D-10b - Hash-Bound Deterministic PDF Emitter For The Full Report Package

**Date prepared:** 2026-07-04
**Prepared by:** bridge work loop agent at owner direction (Ryan Tufts,
2026-07-04) preparing all eight open register rows.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

---

## 1. Decision Statement And Scope

Decide whether and how to build the **hash-bound deterministic PDF emitter**
for the R5 "full report package" — the follow-up ruling that the `D-10`
Option B ruling (`DEC-021`) explicitly deferred "to a named follow-up ruling
(D-10b) at the R5 full-report-package lead-up"
(`execution/_Decomposition/SOFTWARE_DECOMP.md:599`). Register row `D-10b`
(`execution/_Coordination/_DECISIONS/_REGISTER.md:41`) names this decision as
blocking the R5 full report package gate; completion-plan Phase E row E3
carries the implementation slot ("add the deferred hash-bound deterministic
PDF emitter", `plans/PLAN_2026-06-17_prd_completion.md:201`).

**In scope:** whether a byte-deterministic, hash-bound PDF joins the
canonical rendered-report evidence for R5; if so, where it is emitted
(in-repo Rust crate vs app runtime vs new third-party dependency) and how it
binds into the existing hash-evidence model; if not, what explicit gap record
the R5 report package carries instead.

**Out of scope:** the canonical HTML pipeline itself (ruled by `DEC-021`,
implemented and tested — not reopened here); report content contracts (the
parent packet's §1 out-of-scope carries forward); release packaging and
signing (`D-06`); the assembly of the full report package as a container
(Phase E row E3 implementation work, which follows whatever is ruled here);
and the R5 exit ruling itself — a later human decision. This packet flags the
exit-claim interaction (§5 O-D, §7) and does not rule on it.

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| What `DEC-021` deferred | `execution/_Decomposition/SOFTWARE_DECOMP.md:599`: Option B made deterministic single-file HTML "the canonical hash-bound calculation-report artifact", with "an app-driven webview print-to-PDF convenience whose PDF output is explicitly labeled a derived, non-hash-bound view of the named HTML hash; whether a hash-bound deterministic PDF emitter is required is deferred to a named follow-up ruling (D-10b) at the R5 full-report-package lead-up." |
| Parent-packet framing | `execution/_Coordination/_DECISIONS/D-10_report_rendering_target.md` §5 item 6: the R5 PDF question is "deferred to a named follow-up ruling (suggested D-10b) at the R5 lead-up — not silently assumed either way"; §6 impact row: "the deferred D-10b PDF checkpoint (§5.6) must be scheduled before the R5 gate." |
| Lead-up condition met | Current target stage is PRD R5, advanced 2026-06-23 by `DEC-054` (`execution/_Coordination/_COORDINATION.md:262-271`; `SOFTWARE_DECOMP.md:632`; register row `D-27`, `_REGISTER.md:60`). Phase E is the ordinary in-stage program, and the plan schedules D-10b "At E3 lead-up" (`plans/PLAN_2026-06-17_prd_completion.md:91`). The `DEC-021` lead-up condition is therefore met; this packet is due now. |
| Canonical HTML pipeline exists | `core/reporting/report_renderer/src/lib.rs:1-17`: the crate emits a "single self-contained, scriptless HTML document with a deterministic byte layout"; SHA-256 of the exact bytes is the canonical evidence (`lib.rs:745`); fixed eight-section render order (`lib.rs:28-37`); three protected-content lint gates feeding an `export_blocked` gate (`lib.rs:100-110`). |
| Determinism is tested | `core/reporting/report_renderer/tests/render.rs:122-158` asserts byte-identical re-render, 64-hex hash, scriptlessness, and zero external references; the derived print view is tested to name the canonical hash without touching canonical bytes (`tests/render.rs:232-243`). |
| Current PDF path is derived-only | `derived_print_view` injects a "DERIVED VIEW — not hash-bound evidence" banner naming the canonical hash (`lib.rs:778-795`); the document itself prints "Printed or PDF copies of this document are derived views and are not hash-bound evidence" (`lib.rs:718-723`). The app's "Print / PDF (derived view)" button fires `contentWindow.print()` in a hidden iframe — the OS print dialog, user-driven; no programmatic PDF capture exists in-repo (`apps/desktop/src/features/report/RenderedReportPanel.tsx:146-152, 162-171`). |
| App runtime | Tauri 2 (`apps/desktop/src-tauri/Cargo.toml:26`), not Electron. The renderer runs behind the `render_calculation_report` Tauri command, which appends `derived_print_html` to the outcome payload (`apps/desktop/src-tauri/src/lib.rs:1097-1109`). |
| Hash-evidence model | The document's audit-manifest section states its SHA-256 "is computed over its exact bytes and recorded alongside the referenced envelope hashes" (`lib.rs:510-516`). Hash inputs project-wide use canonical JSON + asset-byte hashing (`core/reporting/audit_manifest/src/lib.rs:1-17`); `DEC-028` binds package evidence "to the canonical members and the manifest's per-member hashes ... not to raw archive container bytes" (`SOFTWARE_DECOMP.md:606`). |
| Evidence entry point | The merge gate is the `DEC-025` five-surface local sweep (`tools/release/run_evidence_sweep.py:1-17`); its `cargo_crate_sweep` surface runs all crate tests (`run_evidence_sweep.py:66-79`), so golden/determinism tests are how renderer guarantees enter standing evidence today. |
| Renderer dependency posture | `core/reporting/report_renderer/Cargo.toml`: dependencies are three in-repo reporting crates plus `serde` and `sha2` — both already used project-wide. `DEC-023` chose a hand-rolled in-repo sparse solver with "zero new numerical dependencies" (`SOFTWARE_DECOMP.md:601`; register row `D-03`, `_REGISTER.md:30`). |
| No package assembly yet | No code assembles the §22.6 "full report package"; the only `report_package` matches in app/core code are unrelated export-adapter field names (e.g. `apps/desktop/src/types.ts:778-779`). E3 remains future Phase E work. |
| PRD demands | R5 deliverable "Full report package" (`docs/PRD.md:1263`); R5 exit criteria are validation-example reproduction and no protected standards data (`docs/PRD.md:1271-1272`) — PDF is not itself an exit criterion. FR-016 requires calculation reports (`docs/PRD.md:368`), currently "Met (deterministic hash-bound HTML render, A7); full report package + PDF in R5" (`plans/PLAN_2026-06-17_prd_completion.md:269`). §23.1 lists "Reproducibility of reports across platforms" as a success metric (`docs/PRD.md:1285`). |

**Unresolved (explicitly not asserted):**

- Whether Tauri 2 exposes a programmatic per-OS print-to-PDF API, and whether
  webview print output is byte-stable across OS/webview versions. Both were
  `TBD` in the parent packet (§3 Q3) and remain unmeasured in-repo; no option
  below may silently assume either answer.
- The effort for a hand-rolled deterministic PDF writer is an estimate
  (ASSUMPTION), not a measured fact; no PDF-emission code exists anywhere
  in-repo to calibrate against.
- Lint-gating a binary PDF: the protected-content linter takes caller-supplied
  text (parent packet §2.1), and in-repo text re-extraction from PDF bytes
  does not exist. Any PDF option therefore gates on the pre-render text model
  and the shared source content, not on the PDF binary itself — the parent
  packet recorded the same limitation for its Option C.

## 3. Context: What The Deferral Preserved, And What Has Changed

`DEC-021` deliberately split the rendering question. Deterministic HTML was
cheap and closed A7/FR-016/R2; PDF determinism was expensive, and only R5's
"full report package" could justify pricing it. The parent packet's Option C
("Rust-side deterministic PDF renderer",
`D-10_report_rendering_target.md` §4 Option C) was named then as the natural
shape of this follow-up: pure-Rust PDF emission, fixed layout, embedded
fonts, fixed metadata/timestamps, byte-determinism by suppressing every
nondeterministic field. Its recorded costs — hand-built typography and
pagination for the ~30 required content classes, exacting nondeterminism
suppression, text-model-only lint gating — are the costs this packet must now
price honestly against R5 reality.

Two things have materially changed since 2026-06-11:

1. **The deterministic section model now exists and is tested.** The renderer
   assembles the eight required sections in fixed order from validated
   envelopes (`lib.rs:28-37`) with escaping, gating, and hashing already
   proven (`tests/render.rs:122-158`). A PDF emitter would consume the same
   assembled section model; none of the content, ordering, or gating logic
   needs rebuilding.
2. **The evidence machinery is settled.** SHA-256-over-exact-bytes with
   determinism tests under the `DEC-025` cargo sweep is the standing pattern.
   A PDF emitter has a ready evidence lane to join rather than a new one to
   invent, and `DEC-028` already establishes how multiple hash-bound members
   coexist inside one package manifest.

What has *not* changed: the PRD still nowhere mandates PDF (parent packet
§2.3 finding, re-checked against the R5 section), and the webview print path
remains unmeasured for byte stability.

## 4. Constraints On Any Option

- **Evidence honesty (`DEC-021`):** only byte-deterministic artifacts may be
  hash-bound. The derived-view labeling discipline for everything else is
  ruled and in force in shipped bytes (`lib.rs:718-723, 778-795`); no option
  may relabel a non-deterministic artifact as evidence without new
  measurement and a new ruling.
- **Dependency posture (`DEC-023` precedent):** new third-party dependencies
  are a cost to be ruled on, never assumed. The renderer today adds nothing
  beyond `serde`/`sha2` (`report_renderer/Cargo.toml`), and the project's
  hardest numerical component was deliberately hand-rolled to keep it that
  way (`SOFTWARE_DECOMP.md:601`).
- **Local-only and boundary rules:** no network fetches; bundled-or-system
  assets only; protected-content lint gates and the six-status vocabulary
  carry forward unchanged from `DEC-021` (`lib.rs:1-17`).
- **No silent R5 claim:** whichever option is ruled, this packet creates no
  R5 exit claim. The R5 exit ruling is a separate future human decision
  (`_COORDINATION.md:262-271`).

## 5. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Pure in-repo minimal deterministic PDF writer** (new crate, e.g. `core/reporting/pdf_emitter`; zero new dependencies) consuming the existing assembled section model. | The only path to a genuinely hash-bound PDF under §4. Highest effort of the live options; output is deliberately plain (§5.1 scope). |
| **O-B** | **Promote the app-runtime print path**: drive webview print-to-PDF and record the resulting PDF hash as evidence. | Cheapest, but not honestly available as *hash-bound* today: byte stability is unmeasured and the API surface unverified (§2 Unresolved); contradicts the ruled derived-view labeling unless a measurement precondition passes first (§5.2). |
| **O-C** | **Adopt a third-party pure-Rust PDF library** as the emitter backend. | Cuts O-A's PDF-plumbing effort; breaks the zero-new-dependencies posture and imports an unaudited determinism surface (§5.3). |
| **O-D** | **Defer PDF again: R5 full report package is HTML-only** with an explicit gap record as a new residual-work register row. | Zero build cost now; the HTML-only-package question is surfaced to the R5 exit ruling where it belongs (§5.4). Conventional-deliverable risk resurfaces at the exit review. |

### 5.1 O-A detail — in-repo minimal PDF writer

Scope proposed (bounding, not binding): text, tables, and banner boxes only;
base-14 standard fonts (no font embedding or subsetting — the largest single
nondeterminism and effort sink in PDF generation); uncompressed or
deterministically-compressed content streams; fixed document metadata; no
timestamps, no document IDs, no producer strings that vary. SHA-256 of the
PDF bytes joins the evidence alongside the HTML hash, recorded the same way
(`lib.rs:510-516` pattern). Byte-golden determinism tests mirror
`tests/render.rs:122-158` and run under the `DEC-025` cargo sweep. Lint gates
operate on the shared pre-render text model (§2 Unresolved, third bullet).
The HTML remains a co-canonical artifact and the app's display surface; the
derived print view remains a convenience, exactly as `DEC-021` labeled it.

- For: full determinism control where `cargo test` can prove it; zero new
  dependencies (matches `DEC-023` posture); both emissions single-sourced
  from one tested section model; satisfies the register row's own words
  ("hash-bound deterministic PDF emitter").
- Against: the largest remaining effort in Phase E reporting work —
  pagination and table layout are real hand-built work even at minimal scope
  (ASSUMPTION on magnitude, §2 Unresolved); visual fidelity is deliberately
  plain, an evidence document rather than a typeset one.

### 5.2 O-B detail — runtime print-to-PDF promoted to evidence

Could only be ruled with a mandatory precondition: an in-repo,
cross-platform, repeated-render byte-stability measurement of webview PDF
output — tooling that does not exist and whose expected result, per the
parent packet's recorded ASSUMPTION (§3 Q3), is negative. Absent that
measurement, ruling O-B would relabel as evidence an artifact that shipped
bytes currently declare "not hash-bound evidence" (`lib.rs:718-723`),
inverting the ruled evidence hierarchy on hope rather than proof.

### 5.3 O-C detail — third-party PDF library

A pure-Rust PDF crate would carry object serialization while repo code drives
layout. But determinism responsibility without determinism control: the
library's object ordering, string escaping, font handling, and version drift
all enter the hashed bytes and must be audited and pinned indefinitely. This
is the shape of dependency the project has previously declined when in-repo
code could carry the burden (`DEC-023`, `SOFTWARE_DECOMP.md:601`). Named for
completeness; not priced further.

### 5.4 O-D detail — defer with explicit gap record

Per the 2026-07-03 residual-row convention (`_REGISTER.md:17-25`), a ruling
for O-D would append a new `D-XX` residual row recording: no hash-bound PDF
exists; the derived print view is the only PDF path; the R5 "full report
package" assembles HTML + envelope evidence only. Whether an HTML-only
package satisfies the R5 deliverable "Full report package"
(`docs/PRD.md:1263`) is a judgment reserved to the human R5 exit ruling —
this packet flags the interaction and does not rule it. The risk is timing:
external reviewers conventionally expect PDF deliverables (ASSUMPTION,
stakeholder convention per parent packet Option C), and under O-D that gap
surfaces at the exit review itself.

## 6. Recommended Disposition (PROPOSAL)

Recommend **O-A** — the pure in-repo minimal deterministic PDF writer at the
deliberately plain §5.1 scope, implemented as ordinary bounded-tranche
Phase E work under completion-plan row E3.

Rationale:

1. O-A is the only option that produces what the register row actually names
   — a *hash-bound deterministic* PDF. O-B cannot promise bytes it does not
   control and would quietly contradict ruled labeling; O-C buys effort
   savings by importing an unaudited determinism surface against the
   `DEC-023` posture; O-D produces no PDF at all.
2. The expensive halves of the parent packet's Option C costing are already
   sunk (§3): content assembly, section ordering, lint gating, hash
   discipline, and the evidence lane all exist and are tested. What remains
   is PDF byte emission and plain pagination — real but bounded work, and
   the minimal scope refuses exactly the typography that made Option C look
   unbounded back in Phase A.
3. The evidence hierarchy stays coherent: HTML and PDF become two
   deterministic emissions of one assembled section model, each hash-recorded
   per the existing convention; the derived print view keeps its ruled role
   as a convenience.

If the human authority judges the O-A effort unaffordable in the R5 window,
**O-D with the explicit gap record** is the honest fallback: it keeps the
evidence chain truthful and routes the HTML-only-package question to the R5
exit ruling where it belongs, rather than papering over it with a
non-deterministic PDF. O-B and O-C are not recommended in any ordering.

This recommendation is a PROPOSAL only. It confers no authority, changes no
state, and makes no claim about the R5 exit.

## 7. Consequence For The R5 Gate (Flag, Not Ruling)

The register marks D-10b as blocking the "R5 full report package gate"
(`_REGISTER.md:41`). Ruling any option unblocks E3 planning; only O-A (or
O-C) yields a hash-bound PDF inside that package; O-D transfers the question
"is HTML-only a full report package?" to the future R5 exit decision. Nothing
in this packet asserts that any R5 deliverable or exit criterion is met.

Interactions with other open rows prepared in this same 2026-07-04 batch
(cross-references only; each rules independently):

- **D-06 (release matrix/packaging):** whichever artifacts D-10b yields
  (HTML-only or HTML+PDF) become members of whatever the release package
  ships; naming/extension decisions stay with D-06 per `DEC-028`
  (`SOFTWARE_DECOMP.md:606`). Neither packet depends on the other's ruling.
- **D-20 (protected-content scan ownership):** a PDF member is a binary
  release surface; the release-time scan procedure D-20 defines should name
  how binary members are covered (via their lint-gated text sources, per §2
  Unresolved). Flagged here so D-20 preparation sees it.
- **D-11 (issuance waves):** the report package is a `CHECKING`-adjacent
  deliverable surface; issuance sequencing is D-11's question, not this one.

## 8. Human Ruling And Disposition

**Ruling recorded:** O-A approved as recommended by owner (Ryan Tufts), 2026-07-04 — in-session blanket slate direction ('proceed as recommended through the "Decision slate"'; verbatim in `_DomainEngines/bridge/LOOP_RECEIPTS.md` Receipt 20): the pure in-repo minimal deterministic PDF writer at the deliberately plain §5.1 scope, implemented as ordinary bounded-tranche Phase E work under completion-plan row E3. Codified as `DEC-061` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12; register row D-10b → RULED. No implementation occurs under this ruling and no R5 exit claim is made (§7's flag stands: the R5 package question remains the R5 exit review's).

## 9. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; register row `D-10b` in
`execution/_Coordination/_DECISIONS/_REGISTER.md` then moves from
`AWAITING_RULING` to `RULED` with the decision pointer. No implementation,
plan-row, or coordination edit occurs under this packet itself.
