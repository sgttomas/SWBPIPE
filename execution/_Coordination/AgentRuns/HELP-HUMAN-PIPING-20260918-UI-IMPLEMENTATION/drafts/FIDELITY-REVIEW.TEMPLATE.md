# Sealed brief — @ID@: design-fidelity review of slice @SLICE@ by screenshot

Sealed by ROOT (HELP_HUMAN, Agent 0) on @DATE@ before launch. Role: TASK (Type 2), read-only on the repository, fresh context, working alone; Type 2 does not delegate. Model requested: Claude Fable 5.1, never the implementer of this slice. Mechanism: Claude Code `Agent` tool, general-purpose type, background. Your return is your final message, which ROOT retains at `{RUN}/lanes/@LANE@/reviews/@ID@_RETURN.md`. Your only write target is a scratch folder outside the repository that ROOT's launch message names, for screenshots.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the worktree you are started in; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`. Write no absolute machine path in your return.

## What you judge

Whether the running product at commit `@SHA@` (branch `@BRANCH@`) matches the design for the surfaces this slice draws: @SURFACES@. You judge by looking: you run the product, take screenshots in light and dark, and compare them with the design. You do not review code; a separate reviewer does.

## The design you judge against

In this order of authority. The owner's rulings: `DEC-100` to `DEC-109` in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12, and D-72. The specification V1.2: `{DESIGN}/instances/UX-SPEC/UX_SPEC_V1.md` (@SPEC@). The design system V1.3: `{DESIGN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` (@DS@) with its `specimen.html`. The frames: `{DESIGN}/instances/MOCKS/frames/` (@FRAMES@), which are HTML you can open in the same browser and screenshot beside the product; `MOCKS_V4.md` says what each frame shows. Where the frames and the specification differ, the specification governs and you report the difference. A frame is a drawing of intent, not a pixel contract: judge roles, hierarchy, colour by token, spacing rhythm, states and copy, not pixel equality.

What the slice was asked to do, and told not to do, is in the lane's sealed brief `{RUN}/briefs/@BRIEF@` and its addendum `{RUN}/lanes/@LANE@/briefs/@LANE@_addendum_1.md`. A control whose engine or typed-interface gap is outside this tranche is **absent, or disabled with its reason shown**: that is correct and is not a finding. Read the manager's return (`{RUN}/lanes/@LANE@/returns/@SLICE@_RETURN.md`) last, only to learn what it says it left out, and check those claims by looking.

## How to look

- Build nothing that is already built; the worktree has its dependencies and engines. Start the product with `sh {RUN}/tools/with_e2e_lock.sh npm run dev` from `{WORKING_ROOT}/apps/desktop`, or drive it with a Playwright script of your own under the same lock; every server and every browser run goes through that lock, with `PLAYWRIGHT_WORKERS=1`, because other work shares this host and its ports. Stop what you start. Hold the lock only while you are capturing; do not sit on it while you think or write.
- Window 1440 x 900 unless a frame states another size. Capture each surface in light and in dark, at rest and in each state the slice draws (hover, focus, selected, pressed, disabled, empty, error). Capture the matching frame at the same size.
- Check by measurement where the design gives a number: a token's colour (read the computed style, compare with the token file), a size, a contrast ratio for text and for a control's boundary, a target size. WCAG 2.2 AA applies to every control the slice touches (D-68).
- Copy: the product is SWBPIPE; status labels carry their domain; "Accept", never "Approve"; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control; no maturity sentence, no acceptance sentence, no other vendor's product named; Canadian spelling; tooltips in the design system's §7.6 form.

## Return

Verdict **PASS** (no actionable finding) or **FINDINGS**. For each finding: severity (blocking, major, minor, trivial), whether it is actionable before merge, the surface and state, what the design says (document and section, or frame), what the product shows (screenshot file name and the measured value where there is one), and the smallest correction. Separately list: differences between a frame and the specification that you met; anything that looked wrong but belongs to a later slice or the other lane; what you did not look at; the screenshots you took; which model you are.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). You claim no usability acceptance and no accessibility conformance; PDU-045 and PDU-046 remain holds.
