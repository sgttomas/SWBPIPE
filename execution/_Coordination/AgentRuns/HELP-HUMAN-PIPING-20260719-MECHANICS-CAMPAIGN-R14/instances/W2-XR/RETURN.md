# W2-XR RETURN — R14-W2-XREPAIR-01 (DEL-07-02 wind schema-mirror test repair)

- **Node:** W2-XR (bounded Agent 2 executor, non-delegating)
- **Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`
- **Disposition executed:** `R14-W2-XREPAIR-01`
  (`../../W2_CROSS_PACKAGE_REPAIR_DISPOSITION.md`)
- **Date:** 2026-07-20
- **Result:** PASS

## Step 1 — Schema precondition (read-only)

Live `schemas/model.schema.yaml` `WindEquivalentStaticInput` confirmed:
`required` is exactly `["pressure", "shape_factor", "direction"]`; an
`anyOf` requires at least one of `exposed_element_refs` / `exposed_spans`;
both exposure properties exist under `properties`. No contradiction — the
repair proceeded.

## Step 2 — Exact diff summary (sole code edit)

One file, one assertion cluster:
`apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx`, inside
the "covers the canonical seismic/wind generation parameter vocabulary
one-to-one" test (post-edit lines 380–391). 5 lines removed, 9 added; no
other hunk in the file, no other file edited by hand.

```diff
-    expect(wind.required).toEqual(["pressure", "shape_factor", "direction", "exposed_element_refs"]);
+    expect(wind.required).toEqual(["pressure", "shape_factor", "direction"]);
+    const windProps = wind.properties as JsonRecord;
+    expect(windProps).toHaveProperty("exposed_element_refs");
+    expect(windProps).toHaveProperty("exposed_spans");
     // Preview-surface keys: pressure/shape_factor/direction are shared;
-    // exposed_pipe_refs ↔ exposed_element_refs (pipe spans are the
-    // preview's element collection).
+    // exposed_pipe_refs ↔ exposed_element_refs (the canonical whole-element
+    // exposure surface); exposed_spans is the sub-span alternative exposure
+    // surface (landed by R14-W2 T4) with no preview-surface counterpart.
     const previewWindKeys = ["pressure", "shape_factor", "direction", "exposed_pipe_refs"];
-    expect(previewWindKeys.slice(0, 3)).toEqual((wind.required as string[]).slice(0, 3));
+    expect(previewWindKeys.slice(0, 3)).toEqual(wind.required as string[]);
```

Evidence artifacts written alongside (per the disposition action):

- DEL-07-02 `MEMORY.md` — one newest-first entry citing
  `R14-W2-XREPAIR-01` and T4 landing `a854d43a1`.
- DEL-07-02
  `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14W2_XREPAIR01.md` — new
  run record (what/why/checks; GUI-emit follow-on noted as out of scope
  and slated).
- DEL-07-02 `_STATUS.md` — untouched (verified: not in the change set).

## Step 3–5 — Check results

| Check | Command (cwd) | Result | Exit |
| --- | --- | --- | --- |
| Focused desktop vitest | `npx vitest run src/features/model-tree/schemaSlotEmission.test.tsx` (`apps/desktop`, local node_modules, offline) | 1 file passed, 10/10 tests | 0 |
| Registered evidence sweep | `run_registered_checks.py projects/chirality-piping/software-workflow.json --check evidence-sweep --output instances/W2-XR/CHECK_evidence-sweep.json` (repo root) | PASS (persisted JSON: `CHECK_evidence-sweep.json`; sweep exit 0, 144.1 s) | 0 |
| Claims language | `validate_claims_language.py --repo-root .` (repo root) | VALID, 262 files scanned, DEC-081 taxonomy satisfied | 0 |
| Path anchors | `validate_path_anchors.py . --text` (repo root) | PASS, 679 surfaces, no literal home-dir absolute paths | 0 |
| Whitespace | `git diff --check` (repo root) | clean | 0 |
| Containment | `validate_change_scope.py {REPO_ROOT} --base HEAD --allowed …` (persisted JSON: `CHECK_change-scope.json`) | PASS, violations `[]` | 0 |

### Sweep delta

Directory snapshots `sweeps_before.txt` (288 entries) /
`sweeps_after.txt` (289 entries) in this folder. Exactly one new file:

- `validation/evidence/sweeps/SWEEP_20260720T055048Z_27110b28074a-dirty.json`

### Containment fence (as passed to `--allowed`, all repo-root-relative)

1. `projects/chirality-piping/apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx`
2. `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/MEMORY.md`
3. `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/_run_records/WORKING_ITEMS_RUN_2026-07-20_R14W2_XREPAIR01.md`
4. `projects/chirality-piping/validation/evidence/sweeps/SWEEP_20260720T055048Z_27110b28074a-dirty.json`
5. `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2-XR` (this instance dir)
6. `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/W2_CROSS_PACKAGE_REPAIR_DISPOSITION.md` (pre-existing untracked, authored by HELP_HUMAN)

## Refutable claims

1. **Equal-or-stronger assertion.** The edited cluster asserts the exact
   landed `required` triple AND the existence of both
   `exposed_element_refs` and `exposed_spans` in `wind.properties`; the
   prior cluster asserted only the old four-key set. The
   preview-triple comparison now covers the full `required` array rather
   than a `slice(0, 3)`. Refute by diffing the cluster against
   `schemas/model.schema.yaml` `WindEquivalentStaticInput`.
2. **Focused test passes.** `schemaSlotEmission.test.tsx` runs 10/10 green
   under the local vitest, exit 0. Refute by re-running the command above.
3. **Sweep PASS with one-file delta.** The registered `evidence-sweep`
   check reports PASS (exit 0) and exactly one new
   `validation/evidence/sweeps/SWEEP_*.json`
   (`SWEEP_20260720T055048Z_27110b28074a-dirty.json`). Refute via
   `CHECK_evidence-sweep.json` and the before/after snapshots here.
4. **Fence containment.** Every changed or untracked path is contained in
   the six allowed roots; `CHECK_change-scope.json` records PASS with
   empty violations. Refute by re-running `validate_change_scope.py` with
   the fence above.
5. **No `_STATUS.md` touch.** DEL-07-02 `_STATUS.md` does not appear in
   `git status`/diff output and its Remaining section is unmodified.
   Refute via `git diff --stat` and the containment paths list.

No commit, stage, push, PR, merge, receipt, or network action was taken by
this executor.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
