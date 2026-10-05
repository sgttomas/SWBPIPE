# RV65 implementation return

**FINDINGS — RV65-I1 blocks local fan-in.** Three new bitmap resize/fills at
PP/src/retained_product.rs:830–835 have no entered initialization/write accounting.
Reservation/capacity charges do not cover them. Add checked initialization and a
control that exhausts work after the first reserve but before filling, asserting
the exact prefix and no second allocation. REVIEW.md gives the evidence and
bounded remediation.

The `hits` counter is i32, but the sole actual producer supplies duplicate-free
restraints, proving hits<=1. No reachable production overflow is established.
That warrant does not cover arbitrary directly supplied malformed slices;
explicit usize/early-second-hit hardening is a separate non-blocking direction.

RV65-1/2 and dense custody are functionally confirmed. Fresh debug/optimized PP18,
FK17, named formation1, PP-S1111 and FK-S113 all pass. Independent exact checks
cover194 named quantity rows and135 candidate PASS predicates: no false PASS or
unresolved comparison;60 sparse/61 dense real miss rows and5 conservative rows
per mode. Both complete cases still refuse numerically. Old six-case104-record
semantic captures remain identical in both builds.

Seven-file source/ten-file packet/73-file bulk/950 unchanged-file verification
passed. Source is untouched and clean. Compiler lane is reaped/released; guard
PID5387 remains running. No publication, full resource qualification or main
acceptance is implied. Seal and bulk manifests preserve the complete return.
