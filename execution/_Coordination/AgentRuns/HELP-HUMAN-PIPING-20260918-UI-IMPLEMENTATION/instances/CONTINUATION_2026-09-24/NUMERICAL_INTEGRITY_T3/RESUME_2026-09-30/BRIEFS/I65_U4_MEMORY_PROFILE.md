# I65: U4, the memory profile and capture permit (step 4's critical path)

A TASK (Type 2), dispatched directly by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. You do not delegate. You own U4 through its grants; you return at the end of each grant, and at any stop.

## Purpose

The first public milestone (RF-SKEW-T-CANT-OFF-122-r1e-04 through the actual captured facade, both modes) needs a **constructible capture permit**. Today `PP/retained_memory.rs` says there is "no registered production profile or constructible capture permit". U4 delivers a registered production profile for a **bounded first admission domain**: one load case, no combinations, the preview family, no pressure, with counts capped so that the milestone and the post-milestone witnesses fit. Every term is priced at the cap. Then M is selected for that domain, and a permit becomes constructible. Out-of-domain and unknown premises are refused to the unchanged ordinary path.

## The basis (read what each grant needs)

- **The plan:** `R/I61/step4_plan_01/PLAN.md`, §1 (settled items; do not reopen) and §2 U4.
- **The handoff:** `T3/HANDOFF_2026-10-03_TO_NEXT_ROOT.md`, the memory/profile/M items. Note: "Do not enable a permit on symbolic or partially priced terms"; and "Do not commission a new serializer, generic exact-sum engine, guard, probe framework or availability exception merely to bypass a known boundary."
- **The rulings** in `T3/ROOT_RULINGS_V1.md`:
  - the accepted partial-scope ordinary-memory formulas and their residual gaps (around RR:7313; RV75's review);
  - the admission architecture (I51 public_producer_admission_05; RR:7059, 7090);
  - W1 policy and the provisional 3.75 GiB target (RR:4938);
  - the step-4 ruling "Step 4 planned".
- **The earlier work:** native context, tail and backing (I53, RV70); C1 §2's no-wrap premise (I29 f2a_no_wrap_bound_03, partial); the seven `MissingAdmissionTerm`s in `retained_memory.rs`.
- **The native code** at NUM's head.

## Grant 1: the derivation plan (read-only)

Produce `R/I65/u4_plan_01/PLAN.md`:
1. **Every term** that must be priced for the bounded domain, each with:
   - its current state (priced, partial, symbolic or missing), with source;
   - the argument that will close it (a source-bound derivation, a count cap, or a domain restriction);
   - the evidence it needs.
   
   Include each residual gap RR:7313 names, the native context, tail and backing overlap, the seven missing terms, stack, and the no-wrap premise.
2. **For native context, tail and backing in particular:** can they close by source/profile argument within the bounded domain, without new host tooling? If not, stop and say exactly what is missing, so ROOT can take the choice to the owner.
3. **The cap set and the bounded domain definition,** and how the milestone case and the U8 witnesses fit within it.
4. **The grant sequence** for the derivations and the implementation: write fences, deliverables, the independent derivation reviews, and estimates.
5. **The decisions** needed from ROOT or the owner, especially any machine or ceiling interpretation of M (owner-held).

## Limits

- **Never:** new host tooling, guards, probe frameworks or availability exceptions; Git writes or index operations; installs; native or solver jobs at scale; or DEC-025 jobs. Grant 1 runs no code.
- **The memory guard** must be running.
- **Stop and report if:**
  - a term cannot close by source argument within the bounded domain;
  - the plan would need a host tool;
  - a settled item would have to reopen.

## Output

`R/I65/u4_plan_01/` holds `PLAN.md` and `SHA256SUMS`, with placeholder paths only (`WT`, `P`, `PP`, `FK`, `R`).

**Time box:** 90 minutes for grant 1. End with a concise status: the term table summary, the closing route for native context and tail, the grant sequence and estimate, and the decisions needed.
