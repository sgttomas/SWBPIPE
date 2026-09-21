# B4.1 worker freeze03 addendum

Manager browser new-01 at source87 measured last-cell no-op editing Tab landing on its replacement coordinate button in both profiles. Worker diagnosis and pre-repair regression are retained in TAB_BOUNDARY_DIAGNOSIS.md and tab-boundary-repro-01.txt (3 failed, 2 passed, 14 unrelated skipped). No browser oracle changed.

Bounded repair only in EngineeringTable.tsx and EngineeringTable.test.tsx: editing boundary owns Tab default, captures persistent local footer group (forward) or first row-header button (backward), and transfers focus after successful/no-op Apply only while original focus/generation/editor ownership persists. Invalid/rejected edits retain input/context; external focus wins over pending completion; detached/hidden/inert destinations cannot receive focus. Footer has a clear accessible name and uses existing universal .app-shell :focus-visible styling; no CSS/native/global-listener changes.

`npx vitest run src/features/workspace/table/EngineeringTable.test.tsx --maxWorkers=1`: 19 passed, tab-boundary-repair-01.txt. `npx tsc --noEmit`: exit0, typescript-freeze03.txt. Browser rerun remains manager-owned; worker ran no browser/build/native/full suite or Git mutation.

Source frozen/released: SOURCE_FREEZE_03.json SHA256 a6ae14369cbc0db00dc32192138b523c97ed65751777be7264e192ae183b041b, based on 87faf4b240d1be8b197843cd869b22408e85be3b. Exactly two files differ from freeze02; all remaining frozen source/test bytes, including e2e line35/non-body oracle, are unchanged. Prior freeze/return/source evidence retained as intermediate. Manager reruns the two affected connected-browser cases and repeats boundary behavior in native WebKit verification. No full B4/acceptance/release claim.
