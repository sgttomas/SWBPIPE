# I23 VK-S1 survivor diagnosis21

**High-confidence cause: the frozen S1 filter selects a family with no springs.** Every one of the six RF-FINITE inputs has model.springs=[], needs_directional_spring=false and zero spring-target rows. The adapter starts empty SourceParts and fills directional_springs only from that array. form_directional may be called, but its loop at assemble.rs492 is empty: the S1 replacement at510 is unreachable for P33.

This explains the sealed reproduction: exact rf_finite under explicitVK-S1 passed1 test,748 rows,6Selected,6/6 controls and original record equality. Child9-payload seal28c18fe1… and manager5-payload seal77747ee8… were verified; the current original binary/fingerprint still match. P33 remains a required survivor with zero credit. P34–P53 remain unrun; no return control inferred.

Unit-vector normalization, unloaded directional effects and certificate withholding are not the P33 cause: no directional vector exists in this family, and all cases were selected. The fault registry/env/feature evidence is intact. This is a scoped test-mapping coverage defect; it does not establish a production algorithm defect. The6/6 synthetic value controls cover NC-SIGN/NC-SUBTRACT-ROUNDED and do not establish S1 reach.

The smallest source-warranted option is a ROOT-reviewed additive S1 mapping correction to the existing exact rf_skew test, preserving all original inputs/oracles and P33 history. RF-SKEW-T-PIN-AX-345-r1e-04 already has a positive rotational spring with unchanged n=(3,4,0), n·n=25, nonzero torque projection, free root rotations and explicit th.N0.RX=6e-5 / th.N0.RY=8e-5 references. The adapter/source validation does not normalize it. Omitting division changes the ideal spring block by a factor25. No generated fixture or new algorithm is needed to obtain source reach.

That option is not a predicted kill: whether the reached fault publishes a wrong value or is stopped earlier remains unknown. A separately granted baseline/fault/returned-control regression must retain the original value criterion and show an explicit named directional numerical predicate failure. Generic Ceiling, certificate refusal, record/work drift or class-only failure cannot replace it. No new fixture/test/mapping/runtime was prepared or changed; any later repair and review belong to ROOT.

Detailed causal alternatives, affected records/sources, exact six-case reach proof, candidate source/oracle facts, skill origin/hash and reproduction provenance are under `_run_records`. The explicitly invoked software-defect-diagnosis skill was read from its supplied92ea origin (SHA2567e423dfd24132c33d3aa8fe6994bf17a1def966c396723f6bd72ac8d2442112b). Existing P33 evidence fulfills the reproduction boundary; no rerun occurred.

Actual start21:43:26 UTC; boundary21:53:26 UTC. No compiler/Rust/runtime/probe, generator/model/solver, maintained/source/test/fixture edit, Git/index, new tooling/library work or delegation. This is diagnosis and a bounded proposal only, not full V-K/A1/Emax acceptance.

Completed:2026-10-01T21:50:17Z. Final source/input/binary/fingerprint/skill hashes and both reproduction seals verified unchanged.
