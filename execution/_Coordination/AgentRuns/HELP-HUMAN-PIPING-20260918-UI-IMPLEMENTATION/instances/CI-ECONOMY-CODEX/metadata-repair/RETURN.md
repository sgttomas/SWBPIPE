# Metadata-trigger repair

ROOT's actionable review finding was accepted: subscribing to body/readiness/label
metadata events could replace a meaningful pending source validation through PR
concurrency. TASK `/root/ci_strategy` repaired only the workflow, router, tests and
ROOT-authorized CI strategy documentation. Existing ROOT checkout `blob:none`
changes were preserved. No Git mutation, CI rerun, browser execution or protected
E2E/config write occurred.

PR events are now exactly opened, synchronize and reopened. Ignored metadata
creates no workflow, concurrency participant, product execution, or competing
successful/skipped stable gate. Label-based full selection is retired. Explicit
manual dispatch always selects full; its optional target_base input accepts an
immutable commit SHA and requires it to be an ancestor of the exact dispatched
head. Hosted dispatch head/target identity is checked against its event. Plans now
use schema version 3, rejecting stale prior artifacts.

A base retarget alone does not automatically invalidate or rerun GitHub checks.
The documentation requires deliberate current-target full proof when no new head
is supplied, or a meaningful update/reopen that validates the current PR event
base. ROOT must revalidate the live target before reliance/merge.

Validation: all 26 focused tests pass, including ignored-event contracts, valid
manual target proof, unresolved/ref/stale targets, and dropped-target/changed-head
negative controls. Workflow YAML parses and manual-input/concurrency/stable-gate
wiring passes. `git diff --check` passes. The two handed-back E2E files and source
config hashes are unchanged from the original economy freeze. No collection or
browser behavior changed, so earlier dirty-source collection remains historical
support, not a new candidate test pass.

`MANIFEST.json` binds the corrected maintained candidate; `working-diff.patch.txt`
is its complete nine-file diff against 88c6c746, including preserved earlier economy
work and ROOT checkout edits. Original evidence remains unchanged. No resources
are held. ROOT owns independent backcheck, integration and validation.
