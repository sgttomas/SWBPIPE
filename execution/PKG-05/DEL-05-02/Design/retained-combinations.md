# Retained combination contract for J1

B2-C is final for J1: CONTRACT with REVISION_01 and REVISION_02. Its reviewed inputs are SCHEMA `abf3225c…`, PTABLE r2 `b2b4a54d…`, DEF-C r2 `3cebce55…` (H `d3fde142…`). Names remain reserved. The following corrections belong to implementation, not a new contract revision.

For B2-K's exact norm formation, derive midpoints from y₀'s actual binary64 neighbours averaged exactly, never a binade rule. At 2⁻¹⁰²² both gaps are 2⁻¹⁰⁷⁴. At MAX, the upper midpoint is MAX + 2⁹⁷⁰ and S at/above its square refuses. An overflowing estimate uses the same exact comparison, not an estimate-based refusal. At most one correction step is permitted; a second, or a zero/underflowing estimate with S > 0, is an invariant failure. Add the prescribed near-MIN_POSITIVE vectors to implementation tests without rewriting sealed reference vectors.

Ordinary and retained combination magnitudes may differ by about two ulps while both meet G7. SC2's TS `consistentNorm` negative test uses p·(1 + 2⁻⁴⁰) only for p ≥ MIN_POSITIVE; below that, including +0, use p + 256·2⁻¹⁰⁷⁴. D6b remains a case rule, not a rule refusing ordinarily checks-passed retained combinations.

## Forward constraints on B2 continuation

Before any lane admits material selectors at c ≥ 2, N-2 requires the modulus-basis custody fields in `retained_product.rs` (`selections`, `basis_record`, `basis_record_calls`, `basis_expected`) to become per case, rather than per invocation. This binds lane A's admission widening, lane P and SQ2. Existing refusals remain until that condition is met.

N-1 records that readers admit negative or zero `global_upper_bound_pa` and `certified_gap_pa`. Its prescribed B2 reader-round repair is a negative-value refusal with a shared shape; PR-B1 needs no action. The source does not separately prescribe a zero-value refusal, so migration does not introduce one.

For SQ2 (B2/B3), price the exact route on B1's real profile: 10.5 GiB is not a final B2/B3 budget; SQ2 may select 10.75 GiB within the 12 GiB ceiling. B3b-A stays provisional. Separately, R6b finalized B1's M = 10.5 GiB (11,274,289,152 bytes) on `ddc8eaaf54`; do not reopen that settled B1 value by carrying forward R6a's provisional label.

These constraints come from the frozen NUM rulings below: RV125's forward N-1/N-2, R6a's SQ2 carry, and R6b's B1 disposition.

The retained contract and revisions remain retrievable at the frozen source commit; migration does not merge the NUM branch or declare B2 implemented.

Source: [NUM frozen rulings: B2-C revision 02 final for J1](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md). Contract inputs: [B2-C contract and revisions](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/R/I97/b2_c_01/).
