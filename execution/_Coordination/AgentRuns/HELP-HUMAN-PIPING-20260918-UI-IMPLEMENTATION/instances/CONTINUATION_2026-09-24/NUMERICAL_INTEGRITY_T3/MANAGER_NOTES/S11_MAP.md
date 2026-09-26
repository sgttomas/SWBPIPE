# V1-S11 — cancelled loads published as Passed: map

T3 WORKING_ITEMS manager, 2026-09-26, on ROOT's direction: T3 owns this at top priority. The map comes from a source reading at `c61a540ea` and a standard-library arithmetic demonstration. No product build or run was done (host hold). A product reproduction is part of P1 and of the containment slice.

## Mechanism

1. `primitive_loads::LoadApplication::global_load_vector` (`P/core/loads/primitive_loads/src/lib.rs:1401-1409`) folds every nodal load contribution into its DOF with binary64 `+=`, in contribution order.
2. The facade starts the case force vector from that fold (`PP:1810`), then adds more contributions into the same binary64 entries:
   - element uniform-load equivalents (`PP:7759`, `:7797`);
   - thermal and pressure-thrust axial pairs (`PP:8000-8001`, `:8035-8040`, `:8068-8069`);
   - other producers (`PP:8090`, `:9808`).
3. M03's intended-action audit (`FK/structural.rs:510-532`) sees only the folded `system.force[i]`. Stiffness contributions are audited exactly (`contribution_sums`), but load contributions are not.
4. So when contributions on one DOF cancel after one has been absorbed, the net load is wrong. The solve, residual and audit are all consistent with the wrong load, and the case publishes as Passed, which makes it Current-eligible.

## Arithmetic (binary64 left-to-right fold against the exact sum of the same binary64 inputs)

| Contributions on one DOF (N) | binary64 fold | exact | relative error of the net load |
|---|---|---|---|
| 1e80, 1e-8, −1e80 (V1 check L) | 0 | 1e-8 | 1.0 (the whole response lost) |
| 1e8, 0.3, −1e8 | 0.29999999701976776 | 0.3 | 9.9e-9 |
| 1e7, 0.3, −1e7 | 0.30000000074505806 | 0.3 | 2.5e-9 |
| 1e6, 0.3, −1e6 | 0.30000000004656613 | 0.3 | 1.6e-10 |
| 1e5, 0.3, −1e5 | 0.3000000000029104 | 0.3 | 9.7e-12 |
| 1e8, −1e8, 0.3 (order swapped) | 0.3 | 0.3 | 0 |

The error depends on order, and it grows with the ratio of the gross to the net load. Against the analytical 1e-9 criterion on the response to the net load, the criterion is breached once the cancelled gross load exceeds the net by about 10^7 (a few MN against a fraction of a newton). The extreme case loses the net load entirely. Whether such a response counts as wrong depends on the quantity's scale. If the scale is the gross load, the realistic rows are within budget; if it is the net response, they are not. The containment design must state the scale explicitly, as D1's stop-rule floor (V1-S8) must.

## Reach

- **Affected routes:** the ordinary route (`preview-physics-1`), the exact route (`physics-1`), and T1's `load-reference-1`. These build their force vector the same way. Every producer that adds into `force` in `PP` can take part, not only nodal loads.
- **Retained-source methods (source-blocks-1, physics-source-1):** these carry each nodal load as a separate exact term (`source_recovery.rs:554-586`, `exact::ForceContribution`). Their answer is therefore likely correct. D1 must confirm this from the exact context. Note that `source_recovery.rs:589-595` also re-folds the loads in binary64 and requires the result to be bit-equal to the actual `force`. A change to `global_load_vector` alone would break that check.
- **Standing:** a Passed ordinary case is Current-, rule- and export-eligible (subject to T0R's gates). Sensitive and rejected cases are already withheld.

## Containment options (for D1 to design; manager's reading)

| Option | Files | T1 overlap | Notes |
|---|---|---|---|
| C1: exact per-DOF summation of nodal loads, rounded once, in `global_load_vector` | `primitive_loads` | disjoint | Removes nodal absorption without a threshold. But `source_recovery.rs:589-595` compares its own binary64 fold bit-for-bit, and `source_receipt.rs:154` recomputes the vector, so cases whose fold differs would lose retained-source recovery unless those files change (T1-touched). It may change bytes of committed fixtures whose nodal fold rounds; that needs a scan. It does not cover element, thrust or thermal contributions. |
| C2: a `primitive_loads` finding that blocks the load application when the nodal fold differs from the exact sum beyond a stated bound | `primitive_loads` | disjoint | It fails closed through the existing blocked-application path. It needs a bound (method policy), and the same caveat about nodal-only coverage. |
| C3: collect every force contribution in `PP`, sum exactly and round once, and pass the contributions to M03's intended-action audit (`FK`), so an absorbed contribution makes the case not Passed | `PP`, `FK`, `SA` | `PP` T1-touched | This is the real repair and audit, with the same machinery as V1-B1. It waits for T1's merge. |

The manager's expectation is C1 (or C2) before T1's merge for nodal loads, if D1 confirms the source-recovery interaction is safe or fails closed per case, then C3 as the first facade slice after the merge. D1 decides with evidence.

## References

R1 needs an RF-CANCEL family: cancelling contributions on one DOF, at realistic and extreme ratios, from nodal loads and from element-load equivalents at a shared node. Each case states the exact response, a stated scale, and the binary64-fold answers (in authored order and reversed) as negative controls.
