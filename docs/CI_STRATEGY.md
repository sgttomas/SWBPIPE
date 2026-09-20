# Piping desktop CI strategy

Current direction: owner resume of 2026-09-20, retained in the UI implementation
run, `instances/ROOT/CONTINUATION_2026-09-19_CODEX/OWNER_RESUME_2026-09-20.md`.
This is a bounded development verification strategy, not release qualification.

## Why keep hosted checks

Local verification and hosted CI answer overlapping but different questions.
CI builds a clean checkout with locked dependencies and pinned Playwright Chromium
on Linux. Local browser configuration can select installed macOS Chrome. PR825
passed locally but its full hosted run found an actual tooltip hit-testing defect:
the focused Agent reason blocked the page Close pointer target. The failure is
retained and repaired; repeating a failed test without fixing it is not evidence.

The observed run35496887667 took35m55s, including32m22s in source browser tests.
Two180-second failures contributed to that duration. The complete original run
had408passes,20existing skips and the same failure in both viewport profiles.
The four instrument specs accounted for about69seconds of listed test durations;
removing their duplicated viewport entries would not address the main cost.
These timings describe that run, not a future runtime guarantee.

## Selection and execution

- Run the complete B3 accessibility spec first in both existing viewport profiles.
  A failure prevents the expensive remainder from starting.
- Broad product, shared configuration, dependency, core, schema, fixture and
  unknown changes retain the full source suite. Manual dispatch also requests full.
  Run the remainder on four isolated runners, each with one worker. Preserve
  per-file execution semantics and all test entries; do not enable fullyParallel.
- For changes confined to source specs, run changed specs plus the accessibility
  barrier. Instrument helpers and fixtures retain full coverage: some feed ordinary
  UI-foundation and workspace-layout journeys outside their directory. Source/dist
  are separate; dist specs and unclassified dependencies do not qualify for the
  source-spec-only reduction. Rename/deletion/selection uncertainty falls back full.
- Route from the complete PR diff, never only its last commit. Execute the exact
  PR head and require the event target-base commit to be integrated into it; an
  absent or unintegrated base blocks execution. Retain target base separately from
  merge base. ROOT revalidates live main before merge. Record selection basis,
  head, changed files, selected/omitted test IDs and reasons. Collect actual tests
  before execution: every required title/profile must exist exactly once, each
  selected file must be nonempty in both profiles, and full mode must prove the
  barrier/remainder partition equals the complete source inventory exactly once.
  Unknown paths or invalid selection cannot produce a silently empty green run.
- The stable aggregate check passes only when every job required by the selected
  plan succeeds. A failed or cancelled shard is incomplete coverage and cannot
  establish a full pass. Superseded PR runs are cancelled.

The selector and workflow are maintained Piping CI inputs. Their tests cover
routing boundaries, missing/invalid bases, unknown changes, renamed/deleted specs,
argument safety and full-suite inventory equality. Changes to routing/configuration
receive independent review. The first subsequent broad product slice exercises
full hosted orchestration; until observed, local plan/inventory checks are not
reported as hosted qualification.

## PR825 focused repair exception

The owner explicitly requested a reduced rerun for the failed tooltip repair.
It applies only to PR825, anchored to the already fully exercised candidate
`002dff0f244976f98b36517d920b3761f6f88704`. The selector verifies ancestry and the
intervening delta. Only the named reason-tooltip CSS/component and accessibility
regression files, plus this CI strategy and its records, qualify. Other executable
changes restore full mode.

The owner subsequently approved parity of one dist wheel-target assertion with
its already accepted source predicate and PR825-only bounded reruns. This single
dist file qualifies only at approved SHA256
`f6def858ca800f0f65fa338a92847e6ad8e0b11052dc6063f56d31d809cca4f3`;
any other edit restores full mode. The original failed local sweep is retained;
its unchanged passing surfaces plus corrected complete dist/build checks form
combined delta verification under OWNER_DIST_PARITY_APPROVAL.md, not a new complete
sweep. Changed CI policy receives its affected checks separately. General DEC025
remains unchanged.

Run all `b3-accessibility.spec.ts`, `workspace-layout.spec.ts` and
`gui-workflow-validation.spec.ts` in both profiles, plus these existing
`ui-foundation.spec.ts` cases in both profiles:

- keyboard splitters stay named and bounded; narrow drawers restore opener focus and hide inactive controls
- decorative viewport overlays pass real canvas gestures while view controls stay interactive
- workspace Escape event ownership consumed palette
- workspace Escape event ownership consumed drawer

This is a focused repair pass supported by prior full-run evidence. It is not a
full-suite pass on the repaired candidate, cannot supply DEC-093's
`registered_e2e_specs_executed: true`, and does not erase the failed run. The actual
repair must remain bounded; shared event/layout changes require reassessment.

## Local and merge evidence

Keep the clean-candidate full DEC-025 sweep required by the Piping loop before
product merges. Avoid running separate complete source and dist suites immediately
before that same sweep without a new failure or concern: use focused repair checks,
independent review, then the registered sweep once on the frozen candidate.
Native-dependent changes still receive an affected actual-Tauri witness. Browser
success is not native-menu/window proof, and source success is not dist proof.

Keep independent review, harness checks, raw failure evidence, source/binary
bindings, existing skips, protected geometry/physics oracles and timeouts. There
are no branch-protection changes, recurring schedules, broader scope, acceptance,
release or lifecycle promotions. Full hosted verification remains available for
explicit checkpoints and mandatory for broad or unclassified changes.

Official orchestration references: [Playwright sharding](https://playwright.dev/docs/test-sharding)
and [GitHub workflow syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
