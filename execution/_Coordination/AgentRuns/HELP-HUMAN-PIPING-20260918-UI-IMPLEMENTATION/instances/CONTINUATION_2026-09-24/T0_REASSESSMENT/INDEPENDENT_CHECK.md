# Independent check of the T0 reassessment

HELP_HUMAN (ROOT) records this check. A fresh-context TASK that did not write the reassessment checked the first version of [RETURN.md](RETURN.md), read-only, on main `eb56e1083`. The checker wrote nothing in the repository, so ROOT records its result here.

**Verdict: CLEAR.** It found no blocking finding and raised four should-fix items and three notes. The assessor applied all seven in place, as recorded in RETURN.md's header:

- SF-1: the default route's severity was understated. The Current, rule, report and export eligibility facts were added, with a route-decision paragraph.
- SF-2: the VP-SCOPE ancestry was wrong. Only engine `22452ecd` is a main ancestor; the other checkpoints are superseded by PR905 and PR901.
- SF-3: the toolchain. The finding's premise was mistaken. The Cargo tests ran under the project's 1.97.1 directory override, and the probe ran on the default 1.94.1. It was resolved by correcting `toolchain.txt` to record both.
- SF-4: the M05 wording was changed to "no support reaction moment component", noting that the member torsion row carries −500 N·m.
- N-5: the M14 attribution was corrected to the same abs-sum, computed through the sign-mask maxima, and the ratio to "up to √2 high".
- N-6: the N05/N06 accuracy item is cited as the graph's T3 item, and the SUP-17 wording alone keeps M03 open.
- N-7: the M01 and M08 splits are marked as proposals recorded at tranche activation. Only the precision-1 fixture pair belongs to T6.

The dispositions did not change: M01, M03, M05, M08, M14, M33 and M34 are all OPEN under the closure rule, and none of them can be closed.

## What the checker confirmed independently

- **The default route (the checker found the return understated it).** A fresh desktop model is authored at schema 0.2.0. The native solve takes the ordinary route and publishes `precision-1` with `checks_passed`.
  - That output is Current-eligible and eligible for rule checks.
  - It is the only route the legacy report package accepts, and stress-neutral and result export accept it too.
  - It feeds the Report, NativePackage, Handoff and LocalFeaHandoff headlines.
  - Nothing discloses the defects below. The only guards are the refusal of nonzero legacy pressure, the professional human-review boundary and the absence of user rule packs.
- **The probe.** The checker reran it; its output matches `_run_records/t0_probe_output.log` exactly. The desktop's actual entrypoint gives the same values in both sparse and dense modes. The checker's own hand statics use a section of OD 0.12 m and wall 0.01 m, so Z = π(D⁴−d⁴)/(32D) = 8.78337e-5 m³:

| Group | Hand statics | Ordinary route |
|---|---|---|
| M33 | Case B governs: 3000/Z = 34.155 MPa | Headline 22.770 MPa, from case A (the first case) |
| M14 | Case A resultant √2·1000/Z = 16.101 MPa | 22.770 MPa (abs-sum, exactly √2 high for equal My = Mz); the exact route gives 16.101 |
| M05 | Pure 500 N·m torque: anchor F = 0, Mx = −500 N·m | Reaction force norm 0 N with no support moment component; the member torsion row carries −500 N·m |
| M08 | Equal and opposite cases combine to zero; each review row is 11.385 × 1.15 × 1.08 = 14.140 MPa | Every signed combination stress is 0, but the combination SIF row reads 28.281 MPa |

- **The tests.** product_physics library 268 passed and 1 ignored; its 10 integration binaries 62 passed. Both counts match the logs. Many file:line citations were spot-checked and match the source.
- **Machine paths.** None appear in RETURN.md.

## ROOT consequence

The route disposition cuts across M05, M14, M33 and M08. It becomes tranche **T0R** in the work graph: the default route must not publish silently wrong results. A HELPS_HUMANS design is under way in `instances/CONTINUATION_2026-09-24/DEFAULT_ROUTE_DESIGN/` (being written; not yet committed). A narrow second review of the records commit was also CLEAR; its corrections are applied. Containment alone does not close a group.
