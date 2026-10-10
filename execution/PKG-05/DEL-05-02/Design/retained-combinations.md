# Retained combination contract for J1

B2-C is final for J1: CONTRACT with REVISION_01 and REVISION_02. Its reviewed inputs are SCHEMA `abf3225c…`, PTABLE r2 `b2b4a54d…`, DEF-C r2 `3cebce55…` (H `d3fde142…`). Names remain reserved. The following corrections belong to implementation, not a new contract revision.

For B2-K's exact norm formation, derive midpoints from y₀'s actual binary64 neighbours averaged exactly, never a binade rule. At 2⁻¹⁰²² both gaps are 2⁻¹⁰⁷⁴. At MAX, the upper midpoint is MAX + 2⁹⁷⁰ and S at/above its square refuses. An overflowing estimate uses the same exact comparison, not an estimate-based refusal. At most one correction step is permitted; a second, or a zero/underflowing estimate with S > 0, is an invariant failure. Add the prescribed near-MIN_POSITIVE vectors to implementation tests without rewriting sealed reference vectors.

Ordinary and retained combination magnitudes may differ by about two ulps while both meet G7. SC2's TS `consistentNorm` negative test uses p·(1 + 2⁻⁴⁰) only for p ≥ MIN_POSITIVE; below that, including +0, use p + 256·2⁻¹⁰⁷⁴. D6b remains a case rule, not a rule refusing ordinarily checks-passed retained combinations.

## Forward constraints on B2 continuation

Before any lane admits material selectors at c ≥ 2, N-2 requires the modulus-basis custody fields in `retained_product.rs` (`selections`, `basis_record`, `basis_record_calls`, `basis_expected`) to become per case, rather than per invocation. This binds lane A's admission widening, lane P and SQ2. Existing refusals remain until that condition is met.

N-1 records that readers admitted negative or zero `global_upper_bound_pa` and `certified_gap_pa`. PR-B2 adds the negative-value refusal with one shared shape: in all three readers, the existing "extrema numbers" demand also requires each value to be at least zero (`-0` and `+0` pass), so a negative value fails at the same gate, code and detail as a non-number at that site. There is no zero-value refusal.

SQ2 priced the combination route (C_eq = c + z ≤ 3) and the exact route on the real code, each with its own TEXT graph, and the registered profile takes the maximum over routes. M stays 10.5 GiB (11,274,289,152 bytes): E_mov,max + R is 0.8705 M dense and 0.8653 M sparse, with text-error budgets above 5 % in both modes; 10.25 GiB fails the 5 % rule. SQ2 could have selected up to 10.75 GiB within the 12 GiB ceiling and did not need to. The exact route at large member counts still falls back before W1 at G-C's contract-evidence bounds, which are the preview route's per-case figures; that is a fail-safe availability limit, not a pricing gap.

These constraints come from the frozen NUM rulings below: RV125's forward N-1/N-2, R6a's SQ2 carry, and R6b's B1 disposition. N-2 still binds: D1.5 refuses material selectors, so the per-invocation custody fields are not yet reached at c ≥ 2.

The retained contract and revisions remain retrievable at the frozen source commit. PR-B2 implements B2 in the producer, the kernel and all three readers, appends corpus 07o (19 bases, 69 mutations, 19 must-pass entries) and pins the three readers to it and to a shared B2 parity probe set. Two 07o hook bases (`b2_operand_preparation_failure`, `b2_operand_source_unavailable`) state a bound G8 `RETAINED_PRECISION_PREPARATION_MISMATCH` outcome, because their hooks forge the refused member's old diameter as +0.

Source: [NUM frozen rulings: B2-C revision 02 final for J1](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/ROOT_RULINGS_V1.md). Contract inputs: [B2-C contract and revisions](https://github.com/sgttomas/chirality/blob/e85d383b64/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/R/I97/b2_c_01/).
