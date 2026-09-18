# REVIEW-03 — return, retained by ROOT

Retained 2026-09-18 by ROOT from the child's final message (ROOT's transcription; the child had no write target). The findings are given in full as returned. The "what passed" and "what I did not check" sections are condensed; the one machine path in the message is replaced by `{REPO_ROOT}`. Brief: `../../briefs/REVIEW-03_revision_pass_review.md`, sealed `9b9d0c2067293a1ad2d6efafc67733916f9700c9a5213c61ec400184e4bee8b3`, verified by the child before starting. Candidate `3e4cb3a1eeac12791498aec05fd912735d8f841b` against `origin/main`. Model requested: Claude Opus 5. The child wrote no file and ran no build, test, dev server or state-changing git command.

## Verdict: FINDINGS — no blocking, no major; five minor, five trivial

### Minor

**M-1. The generalized pointer rule is attributed to the owner; the D-71 record says the generalization is ROOT's.** `instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md:681`, `instances/UX-SPEC/UX_SPEC_V1.md:152`, `instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md:44`. The owner's words concern Q-20; `D-71_RULING_2026-09-18.md:65` labels the general rule as ROOT's reading. Three records drop that label. The substance is faithful; the attribution is not.

**M-2. "Twelve uncertainties" is thirteen.** The handoff line 96 and `briefs/_INDEX.md:257`; `instances/UX-SPEC/RETURN.md` §7 lists thirteen.

**M-3. A superseded frontier was replaced, not preserved.** `WORK_GRAPH.json`: the frontier at `origin/main` was dropped and not retired into `frontier_history`.

**M-4. The handoff's validator-coverage statement understates what is unvalidated.** Tokens 1.2 changes no colour value of 1.1 but adds two, `canvas.edgeAlt` light and dark, which the validator never assessed.

**M-5. The handoff never labels ROOT's two settlements as ROOT's.** The index, the handoff state, the specification and the design system do; the handoff's §3 constraint 3 carries them unattributed in a list headed as the owner's constraints.

### Trivial

**T-1.** The handoff's §4 has six gap classes where the operations map has five; G-34 is split out. **T-2.** Constraint 10 omits "coordinate shared solver and result contracts" (carried in §4's Engine row). **T-3.** `current_frontier` changed type from a string to a list. **T-4.** The handoff groups Q-25 and Q-26 as window-width problems; Q-26 concerns the 737 px Both pane. **T-5.** The design system says the Checked mark's words are "exactly these and no others" and then uses derived control names; the specification attributes the expanded list to `DEC-104`.

## What passed (condensed)

Scope: all 72 changed paths under the run directory. Rulings: the reviewer's own case-insensitive searches over the three instances found none of the retired strings, the acceptance sentence or its registered variants; the other vendor's name only inside cited source paths and identifiers; only the eight registered label forms, with domains; the Checked mark's words as the addendum gives them. ROOT's factual basis confirmed in the product source: `commitModelAfterSolveInvalidation` declared at `apps/desktop/src/App.tsx:276` and called at `:853`, with the solve proof and model hash cleared at `:857` and `:858`, and `clearComputedModelState` at `:1695`; the specification's §12 item 4 cites these correctly. Handoff: all eighteen hashes recomputed and identical; it starts and authorizes no implementation; the ten constraints map one to one onto the owner's with none weakened; the D-72 summary matches the ruling and addendum; §8 asks exactly one decision. Records: all fifteen sealed-brief hashes match; the frames' 21 hashes match; the work graph parses and agrees with the index; history preserved; no machine path; fence lines present. Citations: 20 of 284 sampled at random, all resolving at `HEAD`; gap accounting reproduced (313 rows, 3 retired, 34 entries). Copy rules: clean. Frames self-contained; one screenshot read by eye.

## What the reviewer did not check (condensed)

The children's tools were not run. One of nineteen screenshots was read. `shots/report.json` was hash-checked only; contrast numbers were not recomputed. 264 of 284 citations were not sampled. The substance of the contradictions, questions and gaps was not judged. Which model ran each child was not verified. SCA-010, the packets' appended sections and the notices are outside the candidate.

## ROOT's dispositions

| Finding | Disposition |
|---|---|
| M-1 | Accepted. Handoff corrected by ROOT; the design system and the specification corrected by their owning children (DESIGN-SYSTEM-03 correction 2, UX-SPEC-02 correction 1) |
| M-2 | Accepted. Handoff corrected; the index carries a dated correction note |
| M-3, T-3 | Accepted. The dropped frontier is retired into `frontier_history`; `current_frontier` is a string again |
| M-4 | Accepted. Handoff §7 now says the two added values were never assessed |
| M-5 | Accepted. Handoff §3 constraint 3 now separates the owner's constraint from ROOT's decisions for the pass |
| T-1, T-2, T-4 | Accepted. Handoff corrected |
| T-5 | Accepted. Corrected by the owning children with M-1 |

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
