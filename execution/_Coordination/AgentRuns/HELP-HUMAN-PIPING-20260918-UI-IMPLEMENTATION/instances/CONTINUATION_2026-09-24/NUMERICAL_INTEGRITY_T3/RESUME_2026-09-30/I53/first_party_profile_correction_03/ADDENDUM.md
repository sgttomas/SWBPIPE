# I53 correction 03 — caller overlay is not the resident lifetime

This addendum corrects **RV70-P02-1 (P2)** in `I53/first_party_profile_02/RETURN.md:9` and its subsequent W_J/formula uses. Those definitions cannot be used as a complete I51 composition interface. Both earlier seals remain historical and unchanged. ROOT supplied the exact reviewer interface in this corrective assignment; review/backcheck of this addendum remains pending.

## Replacement interface

For each **actual native invocation/job J**, keep the upstream ownership account U_J distinct from the caller overlay. U_J carries the applicable producer/completion ownership and, after publication, the resident result Value/lease R_J until its **actual last owning record/table/lease owner drops**. Moving the result and lease from producer into the record transfers ownership; it neither duplicates the allocation nor releases its reservation. Worker completion, final poll, successful registration, JS completion, cancellation request and UI detachment do not retire R_J.

Use **W_call,J** only for the named included caller-profile domain. Each guarded W1 reply has its own interval from its first covered allocation through JSON-body transfer or actual error/abandoned-wrapper Drop. The response cut closes that reply's ownership window; it does not close U_J. Original related request/context allocations are counted while actually live within W_call,J, including tails carried into a later included caller window. Merely being post-transfer does not erase a surviving request owner, and merely remaining resident does not enlarge this caller profile to all later framework/process work.

For phase metric k and t in W_call,J, define the disjoint overlay:

`A_call,J(k,t) = B_call,J(k,t) + O_call,J(k,t)
                + Σ C_a(k,t) + Σ L_i(k,t) + Σ D_j(k,t)`.

| Term | Allocation ownership and boundary |
|---|---|
| U_J, including R_J | Existing upstream producer/resident composition; actual producer transfer/destruction and record/table/lease last-owner Drop. Keep charging applicable shared lease/control storage while an actual owner remains. |
| B_call,J | Attributed caller shared backing/metadata not already in U_J; actual capacity and moving overlap, counted once. No automatic attribution of all WebView backing or unrelated jobs. |
| C_a | Original request/envelope, arguments, URL/headers, command/resolver/transport context allocations for an included native admission, through their actual Drop where they overlap W_call,J; includes carryover. |
| O_call,J | Related ordinary **response work** included by this profile and not already in C_a or U_J. It does not relabel ordinary/cancellation responses as W1 controls. |
| L_i, D_j | Guarded W1 response allocations only: admitted large reply or small Busy/error/emergency response, through their own transfer/Drop. A separately allocated response job-id copy is distinct from the original request copy in C_a. |

Compose **U_J + A_call,J at their actual overlap**, with each physical allocation charged once. There is **no second R_J** in A_call,J. Where I30's earlier per-request D included request/id/header costs, repartition those existing costs into C_a and retain only W1 response costs in D_j; do not add the full old D again. Outside W_call,J the overlay makes no wider promise and is not asserted to be zero; U_J's resident obligation continues independently. If a detached reply survives table teardown, its remaining copy/guard still follows its own actual ownership window.

Authority: `I30/f2a_routing_02/CALLERS_AND_TESTS.md:14–15,36–45,47–51,99–109`; `I30/f2a_native_reply_03/CONTRACT.md:90–102,113–138`; ROOT's adopted native window/control rulings in `T3/ROOT_RULINGS_V1.md:5078–5087,5124–5131`. Frozen native source **c5a1fe55e4**, `P/apps/desktop/src-tauri/src/lib.rs:1700–1729` moves publication into the record; `:1733–1750` clones without removal; `:1773–1784` refuses terminal cancellation without dropping that record.

## Facts retained and one upstream qualification

The logical s+p≤1, c≤1 facts and 38/50/228-byte poll and 72/145/325-byte cancellation identity/payload/envelope facts remain as qualified in profile 02. They are not native callback/worker counts. In particular, **one logical start does not prove one backend worker**: the fetch→postMessage fallback can submit the same logical start through both routes (`tauri-2.11.1/scripts/ipc-protocol.js:37–68`). Upstream reservations must bind each actual admitted invocation/job, not assume one worker from one UI start. No duplicate execution was observed or asserted, and no deduplication feature is demanded by this correction.

The finite native context/overlap premise and capacity, shared-backing, serializer/emergency and build/layout obligations remain open. Re-express profile 02's conditional context bound over C_a and its reply bound over L_i/D_j, within W_call,J; they do not bound or terminate U_J. No H, δ, K, allowance, availability change, runtime qualification or public activation follows. This is a documentation/interface correction only, with no source, runtime, Git/index/API mutation, vendor research or descendants.
