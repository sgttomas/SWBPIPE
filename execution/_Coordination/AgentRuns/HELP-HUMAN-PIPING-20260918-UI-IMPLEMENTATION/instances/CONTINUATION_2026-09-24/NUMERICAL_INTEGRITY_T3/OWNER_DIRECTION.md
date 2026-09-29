# Owner direction for T3 (as relayed by ROOT)

WORKING_ITEMS manager for tranche T3, 2026-09-26. HELP_HUMAN (ROOT) activated this manager and relayed two owner directions in the activation brief. They are quoted as ROOT relayed them; the ellipsis in the second quotation is ROOT's.

**Activation (2026-09-26):**

> start the T3 manager now

**Earlier standing direction:**

> prioritize solver correctness and the validation programme ... Don't populate material or component libraries or code rules. Preserve evidence and completed gates.

## How T3 applies it

- T3 is on the current route in the work graph ([T3 row](../../../../../WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md#current-route-solver-engine-and-validation)). Its closure rule is the graph's rule: a group closes only when it is fixed, verified and merged, and the tranche is complete only when its paired validation evidence (VP-ORACLES, VP-ROBUST) passes on the merged candidate.
- Invented inputs only. No agent adds material, component, catalogue or code-rule data. Every reference case uses stated synthetic properties.
- Evidence and completed gates are preserved. Historical fixtures, hashes, frozen references (N01–N09, R01–R07, NP-A–NP-D, the T0R references) and protected criteria (the analytical 1e-9 relative criterion, DEC-050/053 observations) are not edited. A mismatch returns a repair to the implementer; it never changes a reference, fixture, tolerance or qualified scope.
- ROOT holds delegated correctness authority. T3 brings each checkpoint to ROOT for selection before implementation. It asks for an owner decision only where the design needs a product-semantics choice that no accepted instrument covers, and then it brings concrete options with a recommendation.
- Concurrency with T1 is governed by ROOT's activation brief: design and reference work only in files T1 touches until T1 merges to main; after that, main is merged into the T3 branch before any code change to a shared file.

Other authorities that continue to apply: [CORRECTNESS_ACTIVATION.md](../CORRECTNESS_ACTIVATION.md), [OWNER_PHYSICS_AUTHORITY_2026-09-25.md](../OWNER_PHYSICS_AUTHORITY_2026-09-25.md), [OWNER_ROUTE_DIRECTION_2026-09-25.md](../OWNER_ROUTE_DIRECTION_2026-09-25.md) and [OWNER_RESUME_2026-09-26.md](../OWNER_RESUME_2026-09-26.md).

## Owner decision (2026-09-28): DEC-025 on the Mac

- **The question ROOT asked (verbatim):** "The DEC-025 sweep's cargo surface fails on this Mac for main itself (3 tests compare committed Linux bytes). How should K1 satisfy the 'clean DEC-025 sweep' merge gate?"
- **The owner's answer:** "Mac sweep + Linux CI (Recommended)". The option read: "Run the sandboxed sweep on the Mac for K1 (running now). The gate is met if the only failures are the 3 known platform tests, identical to Mac main, and pytest, vitest and the build pass. Hosted Linux CI's numerical cargo job (all 39 manifests) supplies the clean Linux cargo run. The deviation is recorded in the merge record."
- **How K1 applied it:** `IMPLEMENTATION/K1_MERGE/RECORD.md`, "The DEC-025 sandboxed sweep".
  - The sweep tool is fail-fast, and stops at the cargo surface's first platform failure.
  - So the cargo surface's full evidence is a `--no-fail-fast` run of all 39 manifests against the Mac baseline of main.
  - Surfaces 2, 3 and 5 ran with the sweep's own commands.
- **ROOT's application to later slices (not the owner's wording):**
  - The same rule applies to every T3 slice run on the Mac: K2b, the skew M03 pin, and those after them.
  - It lapses if the owner directs otherwise, or once the three committed-Linux-byte tests stop failing on the Mac.
  - A Linux DEC-025 sweep, where one is available, remains the stricter alternative.
