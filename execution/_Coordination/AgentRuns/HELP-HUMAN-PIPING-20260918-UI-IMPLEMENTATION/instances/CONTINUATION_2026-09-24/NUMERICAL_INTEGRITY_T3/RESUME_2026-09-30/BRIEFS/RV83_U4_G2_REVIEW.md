# RV83: independent re-derivation review of U4 grant 2 (domain, build identity, residuals, stack plan)

TASK (Type 2), an independent reviewer dispatched directly by ROOT (HELP_HUMAN, Agent 0) as a background subagent. ROOT is your return path. You do not delegate. **You did not write this packet. Re-derive; do not re-read.** The authors' scripts and outputs are leads, not oracles.

## The candidate

- **The packet:** `R/I65/u4_g2_01/`, committed at NUM. Your dispatch prompt gives the exact revision. It holds DOMAIN.md, BUILD.md, RESIDUALS.md, D4_RECONCILIATION.md, STACK_PLAN.md, API.md and RETURN.md, plus `_run_records/` and SHA256SUMS.
- **The author** was I65, under `BRIEFS/I65_U4_MEMORY_PROFILE.md` and its plan `R/I65/u4_plan_01/PLAN.md`.
- **The rulings that bind it,** in `T3/ROOT_RULINGS_V1.md`:
  - "U4 plan: decisions, and two questions for the owner" (D-1, D-2, D-5, D-7, D-8, D-9, design-to-budget);
  - "The owner decides D-3 and D-6" (S1; the fail-closed build check);
  - "Checked work custody in U1";
  - "U4 G2 verified; D-4, D-4b and S-1 ruled".

## Read first

- `REPO_ROOT/AGENTS.md`, `agents/AGENT_TASK.md`, `projects/chirality-piping/AGENTS.md`, and this brief.
- The plan's term table (PLAN.md §3, T01–T24) and §7, so you know what G2 owed.
- I54's accepted bound and residuals, and I51's COMPOSITION, as the packet cites them.

## Review, in priority order

1. **The D1 predicate (DOMAIN.md §1, §3).**
   - For each clause, confirm against source that it excludes what it says it excludes.
   - Confirm that an in-domain request can reach only paths the packet treats as priced: the legacy source-blocks namespace, nodal loads, rigid and scalar-spring supports, straight members, one case.
   - **Look for a field the predicate does not constrain** that changes the ordinary or W1 path. Candidates: schema-version defaulting, section properties, optional provenance Values, material lists in both places, case-level options, and solver-mode inputs.
   - Check the refusal map against the schema's `unavailable_precondition` kinds.
   - Confirm the milestone's counts.
2. **Cap arithmetic.** With your own stdlib Python, independently of `caps_arithmetic.py`, re-derive:
   - the derived quantities (N, Q, P_final, C, Z);
   - T02's raw backing;
   - T07's deep legacy-exact total.
   
   State the monotonicity of each formula you substitute caps into. Mark every figure that rests on an ASSUMED layout.
3. **T06 (heap-free H_formation128) and T22 (scalar admission).** Read FK/wide.rs and FKS/formation_check.rs yourself for any allocation on the H_formation128 path. For each T22 site, confirm that the check exists at the cited line and that its value at the caps fits its width.
4. **T03's owner roster.** Enumerate the `LinearStaticPreviewRequest` type tree yourself and compare it with the packet's roster. Report every nested owner that is missing.
5. **T08's text inventory.**
   - Re-extract the template sites with your own method, at least for PP lib.rs, and compare the count with 707.
   - Check the spelling maxima against the installed `core::fmt` source, including the 327-byte f64 Display claim and the 24-byte Debug/LowerExp claim.
   - Confirm that the packet treats the 332-site pre-filter as a lead only.
6. **BUILD.md: the D-6 design and T01/T10.**
   - Every identity read failure must give unavailable. A mismatch or a false witness must give `Stale`, which refuses the permit, and there must be no path to a compile error or to a permit on a mismatch.
   - Check that the recorded identity covers every fact a stride or law depends on.
   - Re-derive the BTree leaf and internal upper bounds (the 640/736 claims against the observed 632/728) and the hashbrown law from the installed sources.
   - Assess the consumer-lock residual.
7. **STACK_PLAN.md under S1.**
   - Look for any recursion class the argument misses: derive-generated recursion on recursive types, trait-object recursion, Value Drop, sort recursion and the reader's `$ref` following.
   - Check the thread-local scan, and the panic and spawn-failure handling.
   - Say whether R = 64 MiB with a witness at R/16 is a defensible proposal on the stated observation. Name what G3 must add.
8. **D4_RECONCILIATION.md.** Check its quotations and citations against C1 §2, the RR lines and I34. Say whether the reading applies C1 §2's own alternative, or changes the contract. A challenge with citations is a finding.
9. **API.md.** Check consistency with D-5, decision 7 (no test permit in maintained code) and design-to-budget.

## Host and method

- **Records review only.** Use stdlib Python for arithmetic, under `NUM/R/REVIEW_RV83/u4_g2_01/_run_records/`. Reading the installed std and cached dependency sources is allowed.
- **Never:** Cargo builds, Git writes, index operations, installs, new tooling, or solver, DEC-025 or native jobs. Git reads use `GIT_OPTIONAL_LOCKS=0`.
- **The memory guard** must be running (`pgrep -fl memguard`). Other TASKs are working: I61 on U1 in `WT/f2a-serializer`, and I65 on U4 G3 under `R/I65/u4_g3_01/`. Don't touch their files.

## Output

- **The report:** `NUM/R/REVIEW_RV83/u4_g2_01/REVIEW.md`, containing:
  - a verdict, PASS or FAIL;
  - counts of BLOCKING, SHOULD-FIX and NOTE findings;
  - a findings table with path:line, evidence and remedy;
  - a short section per review item.
  
  Add your arithmetic and enumerations as files, with a SHA256SUMS. Use placeholder paths only.
- **Time box:** 3 hours from your first tool call. Report anything unfinished.
- **End your turn** with a concise status for ROOT: the verdict, the counts with one line per finding, the report's sha256, and anything ROOT must rule on.
