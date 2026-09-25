# Independent executable numerical reference review

2026-09-24. TASK `/root/solver_manager/numerical_reference_review`, delegated-harness-native child of `/root/solver_manager`; dispatch gpt-6-astra xhigh as reported by parent. No descendants. This is an independent reference/applicability review and source observer backcheck, not review of an unwritten kernel, professional acceptance, merge approval or release.

**Disposition: the frozen analytical references are ready for bounded implementation reliance under the selected M03-INTEGRITY-v1 policy.** N01–N09/R01–R07 expressions and classifications, NP-A represented-versus-intended arithmetic, NP-B positive banded family, and NP-C/D explicitly synthetic controls pass independent checks. No analytic blocker remains in those subsets. Product/gate execution and unsupported family coverage remain separate obligations; their absence does not impose a whole-programme hold.

## Evidence and freeze

The source candidate was `63b9a56cbcc40fd2ba7e1a3c2f2555214aa6d8d9` in the new numerical worktree. Exact runtime origins, source-qualified aliases, hashes, parentage and enforcement limits are in [_run_records/SOURCES.json](_run_records/SOURCES.json). Design/policy documents were read from the **primary** worktree, where they actually exist; no absent copy in the new worktree was implied. Full Root/TASK/Piping/LOOP_INIT instructions were read from the new worktree. No skill or workflow body was selected.

[REVIEWED_FREEZE.json](REVIEWED_FREEZE.json) binds all seven maintained reference files. Key hashes:

- `fixtures.json`: `c061d73481d2ad137f7cef988475721681131789ec222e3e93c5ed3836b2910e`.
- `generate.py`: `b491529a4bff8ed0e25de1f85c2488192a8e55677413571da6c260932b1a71a2`.
- Final `src/main.rs`: `f61f115b7c1e22490c204660ad505e2219bf67684add9c24511b4419b3a38037`.

The new [_run_records/independent_check.py](_run_records/independent_check.py) imports neither production nor the submitted generator. It uses exact Fraction substitution/rank and 140-digit Decimal with **Gauss–Legendre pi**, independent of the submitted Machin series. Seven grouped checks pass, covering every frozen NP-B entry/load/order and NP-A exact hex matrix/solution. Six deliberate in-memory corruptions—annular inertia, reaction sign, original load, stored spring, band entry, internal mode—fail as intended. [Raw independent results](_run_records/independent_check.json) and [mutation exits](_run_records/mutation_checks.json) are retained. The submitted eight self-checks were inspected but their reported passes were not used as independent evidence.

Only lightweight Python and read-only source/Git/web operations ran here. No Cargo/build/npm/native/browser or production solver was executed by this reviewer. The parent separately executed the baseline; this review read that resulting JSONL, with its hash and a derived summary retained in [_run_records/parent_baseline_summary.json](_run_records/parent_baseline_summary.json). Same host and model family do not provide OS/model diversity.

## Findings, corrections and necessary limits

| ID | Finding | Disposition |
|---|---|---|
| REF-01 | Observer `3bc57d5a…` used unsupported `moment`/`force` category tokens and omitted required entity provenance. Parent identified these before execution. | Author repaired as `b86b6cfc…`; static backcheck confirmed supported categories, schema/status and final provenance population. Expected physics unchanged. |
| REF-02 | Added N03 rotational restraint inherited omitted-family inference to Guide, which rejects RX/RZ. Parent execution exposed this after my first static readiness assessment missed the inference. | Final `f61f115b…` explicitly selects Anchor on the N03 root only. Source confirms Anchor accepts the **authored restraint list**, adding no implicit DOFs. Targeted backcheck complete; parent rerun reaches mechanics. Earlier readiness was not an execution pass. |
| REF-03 | NP-A's `K_stored_exact` is exact for its explicitly frozen `a=0x1.07c49b6ac7e22p+21`, not automatically the product section's differently rounded coefficient. | Parent notified. Compare the isolated NP-A seam directly; capture actual product coefficients before calling a product matrix an exact stored-matrix oracle. N05 increment/N06 loss still follow at this binade. |
| REF-04 | `COVERAGE.md` predates final observer additions: N01/N03/N09 bending and NP-A matrix observations are now source-prepared and parent-observed. | Harmless underclaim, not a false pass. Manager should synchronize observation status when integrating baseline evidence. All claims of missing **new-gate** coverage remain valid. |
| REF-05 | No executable typed structural gate, immutable-original mutation hooks, full transport chain or selected-contact witness exists in this reference freeze. | These are implementation/integration gates, explicitly mapped below, not analytical-reference defects or reasons to withhold ready subsets. |

No tolerance was changed. The audit's very small Decimal comparison budgets test the generated reference arithmetic; they are not proposed product limits. No new universal rank, symmetry, conditioning or physical-accuracy threshold is installed. Preserve the selected policy names/count bases, existing analytical 1e-9 comparisons and separate DEC-050/053 observations.

## Applicable subsets and remaining checks

| Subset ready now | Evidence and scope | Required downstream evidence |
|---|---|---|
| A/I/J; N01/N09 bending | Polar integration and EB tip-force integration, signs independently balanced; passive constant-section small-displacement EB. N01 L/D=10 is model verification, not negligible-shear validation. | Full-precision quantity comparison under protected criteria; transformations/units and actual product assembly. |
| N02/N03/N04 | Exact rational rigid-restraint rank, common-rotation null, rotational spectrum and disjoint component counting. N03 RX removes the null, RZ does not. | Geometry witness/entity mapping and shared mode/fallback rejection; spatial rotation/origin/unit variants. |
| N05/N06 and NP-A | Positive intended spring proves exact-model stability. Exact stored equations separately prove rounding loss; ULP tie behavior checked. | Contribution-preserving/reformulated or higher-precision repair; intended answer and physical spring action at required accuracy. A warning/rejection alone does not complete this repair. |
| N07; R01–R07 | Negative energy despite zero residual; independently substituted original equations and constraint reactions; R01–R05 all fail known residual controls. | New typed gate and all selected dense/sparse/fallback paths; R04 kernel now, product prescribed-motion witness depends on M10. |
| N08/N09 torsion | Signed nonzero rotations/reactions; old six-decimal quantization erases them. | Exact same-unit transport, native bridge/state/save/reopen/export/rule consumers and producer/canonical compatibility. |
| NP-B | All exact banded entries/loads/order maps checked; dimension-independent condition proof; optional-certificate pessimism proven. | Actual factor/residual/named-condition observations across permutations, with no false mechanism verdict; representative chain/grid protected observations. |
| NP-C/D | Explicit synthetic connector internal mode, near-collinear exact rank, skew, duplicate loss, range and two-mode contact data. | Supported family-specific energy/null contracts; mutation hooks/range accounting; actual nonlinear selected-state lift-off. Synthetic fixtures do not prove product support. |

## Baseline meaning

The repaired observer calls `run_linear_static_preview_with_mode` in **both** modes using authored inputs; it does not manufacture expectations through production code. Its output is observation, not an acceptance assertion. Parent raw corrected baseline has SHA-256 `b226d7e4568b89194db89fb51fc28a44284a110a908c1a6c89881d728d3de94b`.

The observed old behavior matches these diagnostic expectations: N02 loaded publishes enormous rotations and N02 zero-load publishes solved; N03 RZ retains the same mechanism while N03 RX reaches solved; N04/N06 block without the new physical-versus-numerical classification evidence; N08/N09 torsion publishes zero instead of the analytical nonzero rotation. N05 publishes 0.0001 after rounding, hiding its stored-matrix/physical-answer discrepancy; that envelope alone cannot establish 1e-9 physical accuracy. N01/N09 bending also retain quantized output. NP-A's direct matrix calls expose unrounded stored-system results independently of product rounding.

Generic `solve_dense` legitimately solves the nonsymmetric or indefinite algebra controls. Its successful N07/NP-D result is **not** an API defect. N07 must fail the future passive structural contract; skew must be audited before a symmetric-only path uses one triangle. Neither finite algebra output nor sparse nonpositive-pivot counts provide structural acceptance.

Detailed proofs, modern-source qualification and bounded rerun instructions are in [_run_records/DERIVATIONS.md](_run_records/DERIVATIONS.md). Final implementation validation, fresh complete-diff review, protected checks and native witnesses remain with the manager.
