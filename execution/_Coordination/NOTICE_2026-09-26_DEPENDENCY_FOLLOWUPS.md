# Dependency follow-ups to D-GOV-46 and D-GOV-49

Owner-directed Root tranche `ROOT-DEPENDENCY-FOLLOWUPS-20260926` (manifest `docs/governance_harness/tranche_manifests/ROOT-DEPENDENCY-FOLLOWUPS-20260926.yaml`) fixes the follow-ups left open by D-GOV-46 and applies the owner's decision of 2026-09-26 on how they sit with D-GOV-49 (one accepted DAG per project). It is integrated after D-GOV-49. It adds no new mode or heading, and the `Dependencies.csv` v3.1 schema is unchanged.

1. **Where blockers come from** (`docs/SPEC.md` §5.3–§5.4).
   - A project **without** an accepted project DAG reads blockers from each deliverable's recorded register: the union of the entries in the declared sections of `_DEPENDENCIES.md` and the rows of `Dependencies.csv`. A declared entry and a row for the same direction and target are one edge, counted once. Where they disagree, the declaration governs and the disagreement is reported. A reader never relies on the CSV alone.
   - A project **with** an accepted project DAG reads blockers from the accepted current version named by `_DAG/_LATEST.md`. A deliverable whose local evidence departs from it is `DAG pending` and gets no verdict (D-GOV-49).
   - `project-setup` (contract glossary and Validity, method Phases 1.3 and 3.1) and `review` Gate 2 item 5 state both cases. `project-setup` Phase 3.2 reports `DAG pending` deliverables in their own advisory group, alongside the incremental-setup (D-GOV-50) Phase 5.6 handoff to `project-dag`. Item 5 also names `FULL_GRAPH` and reads legacy `TRACKED` as `FULL_GRAPH`. It checks a declaration without a CSV row by comparing its required maturity with the upstream `_STATUS.md` state.
2. **Declarations reach `Dependencies.csv`.**
   - `dependency-extract` mirrors each declared entry into one `Origin=DECLARED` row marked `mirrored_from=_DEPENDENCIES.md`. Mirroring is idempotent in every `MODE`. It copies only the stated fields, and a mirror whose declaration was removed is retired.
   - The section heading supplies the row's `DependencyType`: `PREREQUISITE` for Declared Upstream, `ENABLES` for Declared Downstream.
3. **`tools/coordination/materialize_local_dependencies.py`**
   - `--refresh-pointers` refreshes `_DEPENDENCIES.md` in place under SPEC §5.2. It writes only the agent-owned Extracted Dependency Register and Lifecycle Summary, fills a placeholder Run Notes body, and appends Run History.
   - Human-owned, legacy-headed and unrecognized sections stay byte-for-byte.
   - A missing human-owned section is added as a `TBD` placeholder (mode `TBD`, declared lists `TBD`) and never filled. A missing file gets the §5.2 skeleton.
   - Rewriting `Dependencies.csv` from the aggregate DAG now keeps the local `Origin=DECLARED` rows byte-for-byte. A `DependencyID` collision is reported.
   - The register section's authority statement now gives the D-GOV-49 reading: the local files are the evidence, and an accepted version governs only while it is current.
4. **Blocker and closure tools.**
   - `tools/coordination/analyze_dep_closure.py` reads the union by default; `--include-declared false` gives the CSV-only reading. With `_DAG/_LATEST.md`, it reports `DAG pending` deliverables in `accepted_dag` and `dag_pending.csv`.
   - `tools/coordination/build_dev001_blocker_queue.py` is unchanged for its DAG-file inputs. Its new `--execution-root` mode computes project blockers by the rule in item 1.
   - Both tools use a new library, `tools/coordination/dependency_evidence.py`. Its departure check compares arcs and inventory only; the `project-dag` currency audit governs where the two differ.
   - `tools/coordination/audit_dag.py`'s markdown report names the edge file it audited and gives the D-GOV-49 reading, in place of the fixed `DAG-001` "synchronized mirrors" text.
5. The `_DEPENDENCIES.md` template's **Register** line (SPEC §5.2 and the `preparation` scaffold) now names the declared sections together with `Dependencies.csv`, in place of "Dependencies.csv when present; otherwise the declared sections". Existing files are not rewritten.

This loop has an accepted DAG pointer (`execution/_DAG/_LATEST.md`, naming `DAG-011`). The tools read its `Latest DAG artifact:` form.
- `build_dev001_blocker_queue.py` keeps its DEV-001 behaviour for `--dag-dir`, `--edges` and `--nodes`. Only `--execution-root` switches to the D-GOV-49 reading.
- In that mode, a `DOWNSTREAM` row is reversed into a consumer-to-supplier arc. The DAG-file mode keeps reading every row as From-consumes-Target.
- At the tranche's basis, the arc check found no departure between the local evidence and `DAG-011`. This is a check on the tool, not a currency audit.
- A rerun of `materialize_local_dependencies.py` keeps local `Origin=DECLARED` rows and writes the new authority statement in the register section.
- One file still carries the earlier generated pointer format and has no human-owned sections: `execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-09_Interactive operation vocabulary and tool palette contract/_DEPENDENCIES.md`. A refresh would turn it into the §5.2 register section and add the mode and declared lists as `TBD` placeholders, never filled.

Recording the mode and declarations, rerunning any tool, and changing this loop's "synchronized mirrors" and "dependency authority" wording (named in the D-GOV-49 notice) are this loop's decisions.

No existing `_DEPENDENCIES.md`, `Dependencies.csv`, DAG version or pointer was changed or regenerated by this tranche, and historical runs and generated reports are not rewritten. This loop decides its own adoption; this source tranche grants no release.
