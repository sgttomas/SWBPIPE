# Boundary trusted-context contract review

Verdict: REVISE for BC1, an association-staleness integration omission. The private token/replay architecture itself is sound at design level.

## BC1 — Associated-support single edits still need the promised association precondition

N6 BOUNDARY_WIRE_CONTRACT_FROZEN_V1.md explicitly excludes boundary_association from configuration before/after and relies on a full model revision/hash precondition to guard changed association. Existing operation_applier::check_claimed_model_hash returns immediately for None/null, retaining only field-level before-state checks. PRIVATE_BATCH_CONTEXT_API.md leaves public single calls unchanged and forwards context only into create_support; neither candidate adds the required guard for edits of associated supports.

Counterexample: a reviewed support configuration was authored while association.equipment_reference was A. Current model has reference B but identical family/restraints/stiffness/hanger/nonlinear/provenance configuration. A direct update_support(configuration) with no claimed hash passes the unchanged before projection and changes the differently associated support. The readonly association UI does not protect direct single native/Wasm callers or an imported model replacement.

Minimum amendment: require complete valid authoritative model hash for any update/scalar edit/deletion of a support carrying boundary_association, before resolution/application, while leaving legacy unassociated single behavior unchanged. Apply the same check in validate and apply, native and Wasm. Batch replay already supplies complete evolving hash evidence. Alternatively introduce a concrete association-aware precondition without mutating the frozen configuration projection, but specify it explicitly. Tests: absent/null/malformed/stale hash rejects associated-support edits/deletion unchanged; correct current hash allows ordinary member edits and preserves association; legacy unassociated optional-hash behavior stays compatible.

## Trust context and atomicity findings

- Public single functions hardcode no context; crate-private token has private fields, no Deserialize/public constructor or caller wire. Fake JSON flags cannot select trusted batch behavior.
- preflight_batch evaluates group newness against original base, so deleting old members earlier in a batch cannot recreate their group under a fresh interpretation.
- validate_create in PRIVATE_BATCH_CONTEXT_API checks exact canonical payload membership and requires all same-group current records to be earlier allowed token payloads. A changed node/association/stiffness payload or injected noncanonical prefix cannot borrow the token to pass. The implementation must preserve this exact-string/payload evidence rather than compare only boundary_id.
- The common batch implementation checks initial claim, constructs token once from that same base/batch, and replays those submitted intents. It must not expose a way to substitute a token, base, batch, or reconstructed intent from JSON. Stored base/batch hashes provide evidence binding; the internal trusted sequence is the only holder.
- Metadata stays in normal create_support projection after validation. No remove-before-replay/add-after-apply shortcut is permitted. This preserves one mutation implementation and preserves ordinary reference/unit/ID/before-state guards.
- Both validate and apply may use private replay; B4 V3's simulated-step projection removes all intermediate accepted/model claims. Only successful apply publishes final model and one checkpoint, with the N1 async generation/hash commit guard intact.
- Single creation of one canonical member is permitted for a new group and any subsequent direct append is rejected. Composite creation is one batch. Configuration/scalar editing must preserve association; unknown configuration association keys reject. The schema/native persistence inclusion remains required integration work.
- Disjoint C1/C3 modules are appropriate only after this shared API is accepted; manager-owned routing/schema/wrapper edits remain serial. No new physics, equipment entity collection, solver coupling, provider integration or D58 change is introduced by this design.

## Handoff

Return BC1 to HELP_HUMAN for N2 amendment and focused recheck. No source writes, tests or delegation. This is derivative design evidence against captured source hashes, not a frozen-code review, implementation acceptance or lifecycle closure. Exact inherited model identifier unavailable. Rerun on amended API/wire or source drift. Parent owns eventual integrated fresh code review and deterministic tests.
