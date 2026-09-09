# P5 child implementation amendment V3 — pressure eligibility

RequestedBy: root HELP_HUMAN, relayed by WORKING_ITEMS Agent 1 `/root/endpoint_repair_manager`
AppliesTo: `P5_CHILD_IMPLEMENTATION_BRIEF_V2.md` SHA-256 `9d654abc649631e5827c7f7c858160a6e82a3150489797f65bf1a31b76166900`
Status: `FROZEN ADDITIVE AMENDMENT`
SourceExpansion: none; remains the same `core/product_physics/src/lib.rs` fence
FixtureAuthority: unchanged and conditional

## Accepted added basis

Read and accept `instances/PREVIEW/design/PRESSURE_ELIGIBILITY_REPAIR_ADDENDUM_V1.md`, SHA-256 `dc443b7dea76802072a7564fb8047b43221b37c143bd40ce4ccf32306956d3f9`.

## Added objective

Within the existing authorized Rust file, make pressure stress eligibility identical to pressure-thrust eligibility: `category == "pressure"`, `dimension == "pressure"`, and the same resolved element target. Prefer one private predicate consumed by both producer paths so they cannot drift. Keep the existing sum of all eligible genuine pressure records.

## Added focused acceptance oracles

Add these under the already authorized `endpoint_section_cut` focused test filter:

1. A supported non-pressure `hydrotest` record with `dimension="pressure"` reaches the solve but creates neither pressure-thrust displacement/reaction nor any pressure hoop/longitudinal stress row.
2. A genuine `category="pressure", dimension="pressure"` record preserves existing genuine-pressure result leaves.
3. Two unequal genuine pressure records on one pipe sum once in thrust and stress recovery, killing last-wins and duplicate-count mutations.
4. One genuine pressure plus one supported non-pressure pressure-dimension record uses only the genuine record.

If category/dimension validation now blocks the mixed record, replace oracle 1 with its explicit boundary diagnostic; do not invent a new diagnostic in this repair.

## Preserved limits

No schema, public quantity, topology inference, Poisson behavior, result kind, diagnostic, new source path, fixture hand edit, or raw endpoint-action change. All original V2 acceptance and exclusions remain binding.

## Existing failed attempt

The first focused Cargo attempt finished with a compile failure in newly authored test code before any test ran: the raw-row enumerate loop omitted `slot`, and one station metadata assertion referenced `station` outside its loop. Preserve this actual failure in the child return and run record. Repairing those colocated tests is authorized. The next focused run must validate the complete amended test set.
