# Sealed addendum 2 to brief B-CANVAS: ROOT's decisions on proposals P1, P2 and P3, and the lane's next order

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19T02:06Z, while the lane's manager is stopped at the return of slice C1's first part. It is part of the lane's brief and binds as the brief does. Where it differs from the brief or from addendum 1, this addendum governs. Path placeholders as in the brief.

Every decision below is ROOT's, made inside the owner's rulings, and the owner may reopen any of them. Where a matter is the owner's, this addendum says so and leaves it undecided.

## 0. What ROOT read

- Your return, retained at `{RUN}/lanes/B-CANVAS/returns/C1A_RETURN.md` (SHA-256 `170ab03db6b0533c54087d60a3f456ec647433fdf3ebe255b01b2a4b9bfc05b1`), and `{RUN}/instances/B-CANVAS/slices/C1_PART_A.md` (`6c11123c930873ccb4af6d8164a16055f52564f4dbd7b86460408cfdc2785441`).
- The three proposals under `{RUN}/instances/B-CANVAS/proposals/`, each in full: `P1_SECOND_PROFILE.md` (`cd4650cb4a8c5d65b80bed8555b2349c1aa222938bb987aa09d1e696d7fd4626`), `P2_EDGE_LINE_SLICE.md` (`abb5d7dbc681684bc27cc713ddf977ca05e2f041e1cbc20bc85811bc45ef0a61`) and `P3_STYLESHEET_REQUEST_C1.md` (`0ecab4d2a6a04c9199ffd96fbbe76ba9e1b3991a67b90ebfe09fea5a19a14318`).
- Three of your screenshots, opened by ROOT: `before_light_model.png`, `light_model.png` and `dark_model.png` under `screenshots/C1A/`.

## 1. P1: the instrument's second profile

A correction of ROOT's own words comes first. Addendum 1 asked how the second profile "sits beside the first in `fixture-manifest.json`", and the brief's C4 says "with its own manifest entries". Both phrases were ROOT's. Neither is in the owner's D-72 ruling or its addendum, which fix the values and say nothing of where the profile's files live. Your inventory shows that the manifest is bound by its own hash and cannot gain a key. ROOT withdraws both phrases: a sibling directory meets the ruling.

| Ask | ROOT's decision |
|---|---|
| ASK-1 | Accepted. The second profile is the sibling directory `e2e/ui-foundation/profiles/d72`; the first profile's files stay byte-identical; nothing is added to `fixture-manifest.json`. The seal file and its recomputation in `d72-profile.spec.ts` are accepted as P1 §2 proposes. If the seal proves to cost more than a small hermetic spec, return it as a script for ROOT's reviewer to run, and say so; do not drop it. |
| ASK-2 | Accepted: the bound values of P1 §3, each copied from the D-72 ruling and its addendum, with the canvas sizes checked by exact equality. |
| ASK-3 | Accepted as the form to build against. It is not yet a frozen rule. The freeze of the second profile (descriptor, tokens, primitives, witness and plans, by hash) goes to the owner as one concrete package, with its independent review as instrument code, before any timed run uses it. ROOT prepares that package and you supply its parts. |
| ASK-4 | Not decided. A casing changes the adopted presentation, so it belongs to the design and to the owner. C2's probe reports, per fixture and per theme, how many of the 200 samples pass the pair rule with no casing. ROOT takes those numbers and the options to the owner. Until then C2 draws no casing, and the lane neither lowers 3:1 nor drops the pair check, as P1 §4.5 already says. |
| ASK-5 | Not decided, and your recommendation is not accepted yet. A stencil buffer changes how the persistent renderer is constructed. Handoff constraint 5 names the persistent renderer as foundation that is kept "unless separately justified and authorized", and that authorization is the owner's. C2 brings the options with their measured costs, among them any form that needs no change to the renderer's construction, and the justification for the one you recommend. You may measure a stencil candidate in a probe that is removed before the slice returns. No change to the renderer's construction is in a returned slice without the owner's word through ROOT. |
| ASK-6 | Option A. `viewportSelectionPresentation.ts` stays on disk, byte-identical, and C2 builds the halo in new files. Option B belongs with the maintenance of ASK-9. |
| ASK-7 | Accepted, with the condition you state: if the provisioned Chromium on the reference host has moved, return that fact before the freeze. |
| ASK-8 | Assigned as you propose. The canvas lane writes the profile and supplies the canvas and HUD test ids. The shell lane supplies the pane, view-switch, stage-switch and table-filter test ids and the additive optional field `labels.mode` in `src/features/workspace/uiDiagnostics.ts`. ROOT sends the shell lane a sealed addendum for that when C4 nears. With C3's return, tell ROOT by name what C4 will need from the shell lane. Until the shell lane lands the field, you do not edit `uiDiagnostics.ts`. |
| ASK-9 | Agreed: the four defects are named and none is fixed in this lane. ROOT reports them to the owner as defects of the first profile that predate this lane, with your correction of the inventory's file name as P1 §8 records it. |
| ASK-10 | Yes. The brief's write-scope sentence covers creating `profiles/d72` with its witness, in C4's slot, at `status: DRAFT_UNFROZEN` until the freeze. |
| ASK-11 | Accepted. C1b moves the two held families to `canvas.bg` and `canvas.selection`. From that commit the first profile's freeze no longer describes the lane's product, by design. Name that in C1b's return as a consequence, in those words or better ones. |

## 2. P2: the edge line

| Ask | ROOT's decision |
|---|---|
| ASK-1 | Slice C1E is added. The line of design system §6.2 is part of the adopted design, three accepted records assume it, and the gap in the brief's slicing was ROOT's. |
| ASK-2 | C1E is next, then C1b, then C2. C1E carries the lane's largest unmeasured cost and needs nothing from anyone, so its finding should come first. C1b is small, and by the time you reach it the stylesheet block it touches may have settled (§3 below). If you have a reason to take C1b first, say so in one line in your return and do it; what matters is §4's candidate. |
| ASK-3 | Agreed: mechanism A, then B, then C, and A with B combined if that serves. Mechanism D is not tried. It changes the persistent renderer's frame, which is foundation under constraint 5, and ROOT will not seek the owner's authorization for it unless A to C all fail. |
| ASK-4 | Agreed as you recommend: the rings are in C1E if the chosen mechanism carries them at no measurable cost, and otherwise they come back as a follow-on with the numbers. `src/features/workspace/modelIndex.ts` is in your write scope for the two flags per pipe end. Keep the addition derived, read-only and additive: no existing field changes, and no picking input changes. Name it in the return. |
| ASK-5 | No. The lane's branch does not reach `main` until the edge line is in it. ROOT compared `before_light_model.png` with `light_model.png`: the light figure is paler than today's product and has no line, and the design intends the fill and the line together. If no mechanism holds D-72's limits, return the numbers; the fallback is a design question and ROOT takes it to the owner. The lane does not ship the pale figure alone, thin the line, drop it at a pipe count or touch a limit, as P2 §5 already says. |

Two notes on P2 §5's measurement, which ROOT otherwise accepts as written. This host is shared: two other branches run unit suites, builds and reviews while you measure, and only browser tests are behind the lock. So take each comparison as interleaved pairs (edge off, edge on, off, on), record the host's load average beside each run, keep your own builds clear of your timed runs, and repeat a thin margin when the host is quiet. Your numbers guide the lane. Whether D-72's limits hold is decided only by ROOT's qualification runs.

## 3. P3: the canvas furniture in `styles.css`

`styles.css` stays the shell lane's file. For this one change ROOT grants a scoped exception, because the rules are read by the viewport alone and routing them would make C1b wait on the shell lane's slice order.

In C1b, and nowhere else, you may edit `src/styles.css` for exactly this:

1. the seven rows of P3's "Requested now" table, each by its selector, property and requested value;
2. the two mirrors, `--ui-canvas: var(--canvas-bg);` and `--ui-viewport-selection-geometry: var(--canvas-selection);`, with the dark block's two overrides removed if the token itself carries the theme;
3. the sentence of that block's comment that describes those two variables, so that the comment stays true.

Nothing else in the file changes: no other rule, no other variable, no reformatting. The shell lane is told to leave the `.viewport-*` rules and those two variables alone. `src/design/tokens.json` and `src/tokens.css` remain the shell lane's; you read them and never edit them. Add a test inside your own scope that fails when one of those three rules regains a colour literal or when a mirror stops being its token; a unit test that reads `styles.css`, in the manner of `viewportColourLiterals.test.ts`, would do.

One collision to expect. The design system's V1.4 (`tokens.json` 1.3: a control-boundary token and a lifted disabled ink) is in review on its own branch. After it merges, a small shell-side slice adopts 1.3 in the product and edits `--ui-border`, `--ui-disabled-text` and the comment of the same block. Whichever of the two pull requests reaches `main` second merges `main` first and keeps both changes. No `canvas.*` value differs between 1.2 and 1.3; ROOT compared the two token files by script. When ROOT tells you 1.3 has merged, merge `main`; your palette's values do not move.

## 4. The lane's first pull request, and its reviews

The first pull request of this lane is C1's first part, C1E and C1b together, as one frozen candidate. Its independent code review covers 100 % of that combined diff, and its design-fidelity review by screenshot reads the combined figure in both themes. C2 starts after that candidate is returned.

ROOT dispatches an early read-only code review of C1's first part now, on candidate `beb69d603d74fea326f64ab663e076bc2cd8ba9d`, because C1E's first mechanism is built inside the figure material and a defect there is cheaper to repair before more stands on it. The reviewer works in another worktree at that commit, so your worktree is yours while it reads. Its findings come to you when it returns; route them to the owning child as the brief says. If a finding lands in code C1E is changing, finish the step you are in and then take the finding.

## 5. Your four other items

- **The empty `layer.add`.** Authorized as a named fix: the guard `if (objects.length > 0)` in `replaceLayer`, with a unit test that replaces a layer with an empty list and expects no console error and an empty layer. It changes no ownership, no disposal and no scheduling. Put it in C1E's commit series as its own commit and name it in the return.
- **`canvas.draft` equals `canvas.selection`.** That is the design's intent, and ROOT checked it. Design system §2.1 gives one blue to both roles, and specification §12 row 12 says a draft has "its own token and form": the token exists so that the two can part later, and today form carries the difference. Nothing to change.
- **Ground grid cadence.** Keep today's cadence. The design states no major interval; ROOT carries it as an open design item.
- **Tube shading.** Following design system §6.2 over the frames' untokenized flank band is correct: the document governs, and the frames illustrate it.

## 6. Two facts about this host

- A nested child's completion comes to ROOT when its manager has stopped, and comes to the manager directly when the manager is mid-turn, as C1A-PALETTE's did. Either way ROOT relays what reaches it, verbatim by file with its SHA-256; ignore a duplicate.
- `origin/main` is unchanged since your merge at `9e2fe2826`: it is still `8e4c5df6ec84928ca343224e5244ccdae5771cd4`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
