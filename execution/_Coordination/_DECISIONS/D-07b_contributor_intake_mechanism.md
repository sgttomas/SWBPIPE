# D-07b - Contributor Intake Mechanism (DEC-027 Deferred Follow-Up)

**Date prepared:** 2026-07-04
**Prepared by:** bridge work loop agent at owner direction (Ryan Tufts,
2026-07-04) preparing all eight open register rows.
**Epistemic status:** PROPOSAL (non-governing). Only the human project
authority rules. Nothing here changes lifecycle state, issues deliverables,
creates release readiness, professional approval, certification, sealing,
authentication, code-compliance acceptance, or asserts that any PRD milestone
is met until the human records the ruling.

**Legal-instrument disclaimer (stated plainly, up front):** this packet
describes contributor legal mechanisms (DCO sign-off, CLA variants, copyright
assignment) neutrally as options with trade-offs. It is not legal advice and
cannot be. Adopting any legal instrument — including choosing among the
instruments named here — is the owner's act, taken on legal advice this
packet cannot give.

---

## 1. Decision Statement And Scope

Decide the disposition of register row `D-07b`
(`execution/_Coordination/_DECISIONS/_REGISTER.md:45`): the contributor
intake mechanism — legal instrument, review/quorum process, protected-content
screening, and activation trigger — deferred from the direct `D-07` ruling
`DEC-027` "if and when external contributions open."

Because `DEC-027` closed external contributions and made `D-07b` the gate on
any future intake, the primary structural question is **when and how much to
rule now**: rule the full mechanism today so intake could open at any time
(O-A), rule the trigger structure and the R5 disposition record now while
leaving instrument adoption to a future owner act (O-B), or leave everything
untouched until a concrete contribution request exists (O-C).

Out of scope, whatever is ruled:

- **No intake opens under this packet.** External contributions remain closed
  per `DEC-027` unless and until the owner separately acts.
- **No legal instrument is adopted by this packet.** See the disclaimer above.
- `DEC-027` itself is ruled history and is not reopened or edited.
- No lifecycle, release-readiness, professional-approval, certification,
  sealing, authentication, or code-compliance claim is created, and no PRD
  milestone (including the R5 "IP contribution process" deliverable) is
  asserted as met.

## 2. Verified Facts (Checked Cold, 2026-07-04)

| Check | Result |
|---|---|
| `DEC-027` ruling text | `execution/_Decomposition/SOFTWARE_DECOMP.md:605`: "Rule D-07 directly (no packet): maintainer quorum is one — the sole human project authority is the sole developer, maintainer, and release authority; external contributions are not accepted at this time, so no contributor legal mechanism (CLA/DCO or equivalent) is stood up; any future opening of external contribution intake is a new human-gated decision (registered as D-07b) that must resolve the contributor legal mechanism with appropriate review before intake opens; how this closed-contribution posture maps onto the PRD R5 'IP contribution process' milestone wording remains to be dispositioned at the R5 gate alongside D-12." Basis column records the direct ruling of 2026-06-11: "as the sole developer and maintainer, I have no intention of allowing contributions from external parties at this time." |
| Register rows | `D-07` RULED with no packet (`_REGISTER.md:34`); `D-07b` NOT_PREPARED, blocking "Opening external contribution intake; R5 IP contribution process disposition" (`_REGISTER.md:45`); `D-12` (FR-024/FR-025 disposition, named by `DEC-027` as the R5-gate companion) NOT_PREPARED (`_REGISTER.md:39`); `D-20` (protected-content scan ownership, RGAP-005) NOT_PREPARED (`_REGISTER.md:53`) and in preparation in this same eight-row tranche. |
| Current target stage | R5, advanced by `DEC-054` (`_REGISTER.md:60`) — the R5 gate `DEC-027` pointed the wording disposition at is the current stage, so this packet is timely, not premature. |
| Project license file exists | `LICENSE.md` (chirality-piping project root), lines 1–13: "SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0"; points to the official PolyForm terms URL; self-describes as "a governance notice only... not legal advice." |
| License selection provenance | Selected by the human project authority on 2026-06-03, recorded in the `SOW-001` and `SOW-048` disposition notes (`execution/_Decomposition/SOFTWARE_DECOMP.md:99`, `:146`). |
| Monorepo root license (distinct surface) | The chirality monorepo root also carries a `LICENSE.md` ("MIT License + Professional Engineering Clause", "Copyright (c) 2026 Ryan Tufts"). The piping project's PolyForm notice is project-local. How the two surfaces legally interact for a nested project is **not resolved by any repo file read for this packet** — see Unresolved notes. |
| `CONTRIBUTING.md` exists (draft) | chirality-piping `CONTRIBUTING.md` (doc_id `OPS-CONTRIBUTING`, `status: draft`): defines contribution scope and public-data exclusions (:51–70), a required contributor certification (:72–94), maintainer review routing (:96–115), and rejection/quarantine rules (:117–131). It states twice that the final legal mechanism "such as DCO, CLA, or another instrument, remains `TBD`" (:44–49, :92–94). |
| Certification template exists | `governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md` (draft): interim per-contribution record with source/rights fields, redistribution status enum, a contributor statement ("I certify that... I have the right to submit the covered contribution for OpenPipeStress repository review..."), and an explicit note that the final DCO/CLA/other mechanism remains `TBD`. |
| Review checklist exists | `governance/CONTRIBUTION_REVIEW_CHECKLIST.md` (draft): reviewer-side intake record mirroring the certification fields. |
| Maintainer policy state | `governance/MAINTAINERS.md` (draft): contributor mechanism "final DCO/CLA/other mechanism remains `TBD`" (:34); maintainer roster, release authority, legal-review authority, security contact all `TBD` (:35–42); open questions `GOV-TBD-002` (DCO vs CLA vs other) and `GOV-TBD-003` (maintainers/quorum) at :127–128. Note: this draft predates `DEC-027`; `DEC-027` answered the quorum substance (quorum of one) without the draft being updated. |
| PRD R5 requirement | `docs/PRD.md:1264`: "IP contribution process." is a deliverable of "### 22.6 Release R5: Engineering Beta" (`docs/PRD.md:1258`). The R5 exit criteria (:1271–1272) are "External engineers can reproduce validation examples." and "Public repository contains no known protected standards data." — neither exit criterion requires that intake be **open**. |
| Protected-content escalation rule | `docs/IP_AND_DATA_BOUNDARY.md:73–76`: material appearing to derive from a protected standard or proprietary source must be quarantined and marked `protected_suspected`, with human/legal review required. |
| Copyright-header convention | A grep for "Copyright" across `core/` and `apps/` `*.rs`/`*.ts` sources returned no matches: no per-file copyright-header convention is in evidence in the checked surfaces. Copyright notice lives at the license-file level, not per file. (Scope of check noted; not an exhaustive repo sweep.) |

**Unresolved (explicitly not asserted):**

1. The legal relationship between the monorepo root `LICENSE.md` (MIT +
   Professional Engineering Clause) and the nested chirality-piping
   `LICENSE.md` (PolyForm-Noncommercial-1.0.0) for code inside
   `projects/chirality-piping/`. No repo file read for this packet resolves
   the nesting question. This is a question for the owner's counsel; any
   intake instrument must name which license inbound contributions are
   received under, so this ambiguity should be resolved **before** intake
   ever opens.
2. The actual terms of PolyForm-Noncommercial-1.0.0 beyond what
   `LICENSE.md` itself states (an SPDX identifier and a URL). This packet
   does not characterize the license's contribution or relicensing
   implications; instrument trade-offs in §4 are stated generically.
3. Whether any external party has ever requested to contribute. No such
   request is recorded in any file read for this packet.

## 3. The Trigger Structure: What `DEC-027` Actually Deferred

`DEC-027` did three separable things (all from
`execution/_Decomposition/SOFTWARE_DECOMP.md:605`):

1. **Ruled now:** quorum is one; the sole human project authority is sole
   developer, maintainer, and release authority; contributions are closed.
2. **Deferred with a gate:** any future opening of intake is a new
   human-gated decision — this row — which "must resolve the contributor
   legal mechanism with appropriate review before intake opens."
3. **Deferred to the R5 gate:** how the closed posture maps onto the PRD R5
   "IP contribution process" wording, "alongside D-12."

So `D-07b` is conditioned on "if and when external contributions open" — an
event that has not occurred and that only the owner can cause. The honest
reading of the R5 deliverable follows the same shape: `docs/PRD.md:1264`
requires an IP contribution **process**, and the R5 exit criteria (:1271–1272)
require reproducibility and a clean public repo — not open intake. A recorded
disposition of "process defined; intake closed; opening gated on an owner
act under this packet's skeleton" is therefore a candidate satisfaction of
the R5 deliverable wording, subject to the owner's ruling at the R5 gate
(where `DEC-027` also placed `D-12`).

The draft process surfaces already exist: `CONTRIBUTING.md` (scope,
certification, routing, rejection/quarantine), the certification template,
and the review checklist. What they all mark `TBD` — deliberately, per
`DEC-027` — is the **legal instrument**.

## 4. Decision Content A: Legal Intake Instrument (Neutral Description Only)

Described as options with trade-offs, per the up-front disclaimer. None is
adopted by this packet; adoption is the owner's act on legal advice this
packet cannot give.

| Instrument | What it is | Trade-offs (generic, not legal advice) |
|---|---|---|
| **DCO-style sign-off** | Contributor certifies origin/rights per commit (e.g., a `Signed-off-by` line referencing a Developer Certificate of Origin text). | Lowest friction; no counterparty document to manage; widely recognized. Rights received are essentially "inbound under the project's outbound license" — the project's ability to later change license terms for contributed code is not enlarged. Under a noncommercial project license, whether that constrains any future relicensing or dual-licensing intent is exactly the kind of question requiring counsel. |
| **Lightweight CLA (license grant)** | Contributor signs a one-time agreement granting the project a broad license to the contribution (not ownership transfer). | Moderate friction (signature workflow, records retention, entity-vs-individual variants). Typically gives the project wider latitude than DCO for future license evolution while contributor keeps copyright. Terms must be drafted/reviewed by counsel; a "lightweight" CLA is still a legal contract. |
| **Copyright assignment** | Contributor transfers copyright in the contribution to the project owner. | Maximum consolidation of rights in the sole owner; maximum friction and the strongest deterrent to casual contributors; enforceability and moral-rights treatment vary by jurisdiction. Requires counsel to draft and administer. |

Interaction with the existing project surfaces, which is verifiable and
instrument-neutral: whatever instrument is chosen, the per-contribution
**certification record** (`governance/CONTRIBUTOR_CERTIFICATION_TEMPLATE.md`)
remains the project's provenance/protected-content evidence layer — it
records source and redistribution basis and "is separate from the selected
project license" (template §2). The instrument answers "under what legal
terms does the project receive this"; the certification answers "where did
this come from and may it be public." Both are required by the existing
draft `CONTRIBUTING.md` design; neither substitutes for the other.

## 5. Decision Content B: Review And Quorum Process Under A Sole Maintainer

`DEC-027` fixed quorum at one. A sole-maintainer intake process therefore
cannot rely on independent second review; the compensating controls already
drafted in the repo are record-keeping, not additional reviewers:

- **Routing table:** `CONTRIBUTING.md:96–115` routes each contribution by
  evidence state (complete → maintainer review; missing evidence → pending or
  reject; `unknown` redistribution → not acceptable as public data;
  `private_only` → kept out of public surfaces; `protected_suspected` → stop,
  quarantine, escalate for human/legal review).
- **Reviewer-side record:** `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`
  preserves the same evidence fields on the maintainer side, so acceptance
  leaves an auditable two-sided record (contributor certification + review
  checklist) even with a single reviewer.
- **Boundary checks:** product/report claims route through
  `docs/PROFESSIONAL_BOUNDARY.md` before merge (`CONTRIBUTING.md:111`), and
  maintainer acceptance is expressly "repository governance only"
  (`CONTRIBUTING.md:113–115`).

If intake ever opens while quorum remains one, the review process is these
records plus the owner's own review — a fact any activation ruling should
acknowledge rather than paper over. Expanding the roster is a separate owner
decision (`governance/MAINTAINERS.md:35` roster `TBD`; `GOV-TBD-003`).

## 6. Decision Content C: Protected-Content Screening Of Inbound Contributions

This project bars protected standards data from the public repository
(`CONTRIBUTING.md:57–63`; `docs/IP_AND_DATA_BOUNDARY.md:73–76`), and the R5
exit criterion "Public repository contains no known protected standards
data" (`docs/PRD.md:1272`) makes inbound screening load-bearing for R5.

The intake-side screen already exists in draft: the certification template's
protected-content and private-data screen fields, plus the routing rule that
`protected_suspected` stops review and quarantines metadata without copying
suspected content (`CONTRIBUTING.md:110`, `:129–131`).

Cross-reference: register row `D-20` (`_REGISTER.md:53`) — release-artifact
protected-content **scan ownership and procedure** (RGAP-005) — is being
prepared in this same tranche. The two screens are complementary and should
be ruled consistently: `D-20` governs the release-time artifact scan;
`D-07b` intake screening governs contribution-time entry. Neither packet
asserts the other's outcome; whichever is ruled second should cite the
first's ruling.

## 7. Decision Content D: The R5 "IP Contribution Process" Disposition

The register row names "R5 IP contribution process disposition" as blocked
by this packet (`_REGISTER.md:45`). The candidate disposition record, if the
owner selects O-B (or as a component of O-A):

> The PRD R5 deliverable "IP contribution process" (`docs/PRD.md:1264`) is
> dispositioned as: **process defined, intake closed.** The process consists
> of the draft `CONTRIBUTING.md` workflow, the contributor certification
> template, and the contribution review checklist, with the legal instrument
> deliberately unadopted while contributions remain closed per `DEC-027`.
> External intake remains closed; any opening is a future owner act that
> must first adopt a legal instrument on legal advice, resolve the
> license-surface question (§2 Unresolved note 1), and record the activation
> as a new `DEC` entry citing this packet.

Whether that disposition satisfies the R5 deliverable wording is itself the
owner's call at the R5 gate, alongside `D-12` as `DEC-027` directed. This
packet proposes the record; it does not assert the milestone is met.

## 8. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Rule the full mechanism now:** owner selects a legal instrument (§4, on counsel's advice obtained outside this packet), confirms the §5 sole-maintainer review process and §6 screening as the operative intake procedure, and resolves the license-surface question — so that intake could be opened at any future moment by a single recorded owner act. | Intake becomes open-able on demand; `GOV-TBD-002` closes; the `TBD`s in `CONTRIBUTING.md`/`MAINTAINERS.md`/templates get resolved values. Cost: forces a legal-instrument adoption now, for a project with no recorded contribution request (§2 Unresolved note 3), and the instrument choice cannot responsibly be made from this packet alone — it requires the owner to obtain legal advice first, which sequences this option behind an external act anyway. |
| **O-B** | **Rule the trigger and the disposition record now; defer instrument adoption to activation:** adopt §3's trigger structure (intake stays closed; opening = future owner act), adopt §5 + §6 as the pre-agreed mechanism skeleton that any activation must use or supersede, record the §7 R5 disposition ("process defined, intake closed"), and leave the §4 instrument choice explicitly open until an activation decision taken on legal advice. | The register row moves to RULED without inventing legal posture; R5 gets an honest disposition record for the owner to accept or reject at the R5 gate; the existing draft surfaces keep their deliberate `TBD`s; a future activation is a new `D-XX` row citing this packet, per the residual-work convention (`_REGISTER.md:17–24`). Cost: intake cannot open quickly — activation still requires instrument adoption and the license-surface resolution first. |
| **O-C** | **Take no action; leave the row unruled until a concrete external contribution request exists.** | Zero present cost. But the row then blocks the R5 "IP contribution process" disposition indefinitely (`_REGISTER.md:45`), `DEC-027`'s "dispositioned at the R5 gate" instruction is left undischarged during the current R5 stage, and a future request would arrive with no pre-agreed skeleton — the mechanism, screening posture, and license-surface question would all have to be resolved reactively. |

## 9. Recommended Disposition (PROPOSAL)

Recommend **O-B**, non-bindingly.

Rationale: O-B matches the actual shape of `DEC-027`'s deferral — the ruling
already decided posture (closed) and gate (this row); what remains decidable
today without legal advice is the trigger structure, the review/screening
skeleton (which already exists in draft and needs only adoption as the
skeleton), and the R5 disposition record. The one thing this packet cannot
responsibly resolve is the legal instrument, because instrument selection is
a legal act requiring counsel (§4 disclaimer) and is further complicated by
the unresolved dual-license-surface question (§2 Unresolved note 1). O-A
would either front-run that advice or reduce, in practice, to O-B plus a
promissory note; O-C leaves a `DEC-027` instruction undischarged in the very
stage it was pointed at. O-B discharges what is dischargeable, keeps every
legal decision with the owner, and leaves activation exactly one well-defined
owner act away.

If O-B is accepted, the operative rule proposed is:

> External contribution intake remains **closed** (`DEC-027` posture
> unchanged). The pre-agreed mechanism skeleton for any future intake is:
> the `CONTRIBUTING.md` workflow + contributor certification + review
> checklist (§5), with contribution-time protected-content screening per §6
> coordinated with the `D-20` release-scan ruling. Activation of intake
> requires, in order: (1) owner obtains legal advice and adopts a legal
> instrument (§4 options or another instrument counsel recommends);
> (2) owner resolves the monorepo-root/project license-surface question;
> (3) owner records the activation as a new register row + `DEC` entry
> citing this packet. The R5 "IP contribution process" deliverable is
> dispositioned per §7, subject to owner acceptance at the R5 gate alongside
> `D-12`.

## 10. Human Ruling And Disposition

**Ruling recorded:** _Awaiting owner ruling._

## 11. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; the `D-07b` register row then moves from
`AWAITING_RULING` to `RULED` with the decision pointer. No edit to `DEC-027`,
to any draft governance surface (`CONTRIBUTING.md`, `governance/*`), or to
either `LICENSE.md` occurs under this packet; if O-B is accepted, the
resolution of the drafts' `TBD` fields happens only at a future activation
act, not now.
