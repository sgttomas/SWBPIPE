# Affected-spec integration repair — P2

Retained TASK `/root/ci_strategy`, parent `/root` HELP_HUMAN, Astra/low through
native Codex delegation; no child agents. Source candidate
`52142dbdaec190f3a7fdb47a4f937c3e23720824`, worktree swbpipe-wt2. Instructional
scope limits remained distinct from unrestricted host access.

The router now adds complete `c3-viewport-visibility.spec.ts` to both authoring
and layout selections, and complete `b3b-project-persistence.spec.ts` to
results/persistence selection. The maintained coverage table matches those
additions. No fallback, barrier, lean title, test assertion or collection rule
changed.

Three new product-only policy regressions initially failed with the dedicated
spec absent from each selection (`failure-before.txt`). The final fixtures assert
an actual M-only production diff, with dedicated specs present before the diff,
and require the full dedicated spec, accessibility barrier and every lean title.
They also validate the generated plan. Final focused policy run:

`/Users/ryan/.local/share/mise/installs/python/3.13/bin/python3 -m pytest --noconftest -q projects/chirality-piping/tests/test_ci_e2e_plan.py`

Result: **29 passed, 7 subtests passed**. `git diff --check` passed. Runtime details
and raw focused outputs are retained beside this return.

Execution boundary: the initial focused pytest command omitted `--noconftest`.
The repository's pytest_sessionstart hook invoked its cached Cargo release build
and reported completion in 0.07s before the three expected failures. This was an
unintended hook side effect, reported promptly to ROOT and preserved verbatim;
ROOT acknowledged no timed qualification/native build overlap. Subsequent pure
policy checks disabled conftest. No browser/UI, network, collection, native witness,
full suite, explicit build command, real-worktree Git mutation or delegation was
performed. Disposable Git fixture commits are confined to the policy tests.

`MANIFEST.json` binds the sealed brief, exact outputs, diff and evidence. The
three-file working patch is frozen for ROOT commit and bounded reviewer backcheck.
E2E/config/product/instruction files remain untouched. No new collection counts,
hosted success, qualification or release readiness are claimed. ROOT owns final
combined-candidate collection and integration. No process, browser, lock or shared
slot remains held; CI scope is handed back.
