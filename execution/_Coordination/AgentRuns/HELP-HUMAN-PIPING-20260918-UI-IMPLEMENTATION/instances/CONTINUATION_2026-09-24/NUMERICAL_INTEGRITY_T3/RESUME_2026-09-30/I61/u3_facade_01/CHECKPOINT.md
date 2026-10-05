# I61 U3 grant 1: design checkpoint

**Two of the four items need a ruling before U3 grant 2 makes the permitted path reachable:**
- item 1, the public surface of the successor carrier;
- item 2, the bytes of the public `RETAINED_PRECISION_UNAVAILABLE` notice.

Neither blocks grant 1. In grant 1 the carrier is crate-private, the fallback returns exactly the ordinary bytes, and no permit exists. Items 3 and 4 are implementation choices within decisions 5 and 7, and are recorded here for review.

**Basis:**
- the brief `BRIEFS/I61_U3_FACADE_CAPTURE.md` (NUM `92750f1b6e`);
- the code at `b54caba7ab`.

Abbreviations: PLAN = `R/I61/step4_plan_01/PLAN.md`; API, DOMAIN and STACK = `R/I65/u4_g2_01/{API,DOMAIN,STACK_PLAN}.md`; COMP = `R/I51/public_producer_admission_05/COMPOSITION.md`; ROUTING = `R/I30/f2a_routing_02/ROUTING.md`; C1 = `R/I32/f2a_wire_c1/WIRE_CONTRACT.md`; D1 = `T3/DESIGN_NUMERICS/DESIGN.md`.

## 1. The successor carrier (D-a): its type and visibility. A ruling is needed for grant 2.

**Implemented in grant 1:**
- A crate-private field `RetainedPreviewOutput.retained: Option<Result<RetainedSuccessor, W1Fallback>>`.
  - `RetainedSuccessor` wraps the staged, serialized and precommit-validated successor `serde_json::Value`.
  - `W1Fallback` is the private cause of a fallback.
- `envelope()`, `admission()` and `into_parts()` are unchanged, so **no public type changes**.
- On success, `envelope()` still returns the untouched ordinary base (the fallback copy, D-b), and the successor sits beside it.

**Why a ruling is needed:** `MechanicsEnvelope` cannot carry the successor. It has no `retained_precision` member, no `recovery_method` on its rows, and none of the successor identity (experiment 03, D-a). A caller of the Direct entry therefore needs a public way to receive the successor, and that is a public API change.

**Proposed (A), additive, with no existing signature changed:**
- `pub fn successor(&self) -> Option<&serde_json::Value>` on `RetainedPreviewOutput`;
- `pub fn into_publication(self) -> RetainedPublication`, where `pub enum RetainedPublication { Ordinary(MechanicsEnvelope), Successor(serde_json::Value) }` names exactly one publication;
- `envelope()` is documented as "the ordinary base: the publication unless `successor()` is present".

The alternative (B) changes what `envelope()` returns, which breaks `into_parts` callers in runner/headless (outside the fence, and Direct-only under D-2). Not recommended.

**Memory note for G4:** under (A) the ordinary owner and the successor coexist until return, which is COMP §3's "public staging" plus the fallback copy. Under `into_publication`, the ordinary owner drops as soon as a caller takes the successor.

## 2. The public notice. A ruling is needed (the likeliest one, as the brief says).

**The question.** A permitted invocation can end W1-unavailable after W1 work actually ran: a preparation, native, proof or facade refusal, a serializer refusal, or a precommit-validation refusal. Should its published ordinary base carry a `RETAINED_PRECISION_UNAVAILABLE` notice, and with which bytes? And what of a permit refusal or a gate refusal, where no W1 work ran?

**What the record says:**
- **No notice where no W1 work ran:**
  - ROUTING:96 — admission fails before W1: "Drop new owners; return the complete ordinary base".
  - ROUTING:98 — "In a later-exact/admission-denied path no such work ran, so exact fallback bytes remain mandatory".
  - DOMAIN §3 — "A refusal never changes an ordinary byte".
  - These cover G-A (`admit` refused), G-B and G-C refusals, a spawn failure (StackReservation), the Domain guard and coexistence.
- **A notice where W1 work actually ran:**
  - ROUTING:98 — "Only required W1 unavailable diagnostics may be appended after that prefix for work that actually ran, with case refs only and no invented rows".
  - C1:68 — the receipt-encoding fallback "return[s] the preserved base ordinary publication with `RETAINED_PRECISION_UNAVAILABLE`, reason `receipt_encoding`, detail …".
  - D1:573 — "One info diagnostic per case … The case keeps its ordinary rows and standing".
  - COMP:66 — "Refusal drops unpublished state before rendering its already-reserved unavailable notice".
- **The tension:** these documents favour a notice. Today a refused retained entry returns exactly the ordinary bytes, so a notice is a **published-byte change** for permitted invocations that fall back after real W1 work. The brief reserves that to a ruling.

**Grant 1 implements (N0): exactly the ordinary bytes on every fallback.** This is the status quo and changes nothing, since no permit exists. The cause is kept privately in `W1Fallback`.

**Proposed for the ruling (N1):**
- **When:** only for fallbacks after W1 work ran: Preparation, Native, Candidate, Serializer and Precommit.
- **What:** append exactly one diagnostic after the ordinary diagnostic prefix:
  - `id` `diagnostic:retained-precision:{case}:unavailable`, `code` `RETAINED_PRECISION_UNAVAILABLE`, `severity` `info`;
  - `source` `core/product_physics`, `affected_refs` `[case]`;
  - fixed product text, with **no** receipt reference, because the base publication has no receipt. C1:68's "reason receipt_encoding, detail work_counter_range|work_counter_inconsistent|saturation_not_excluded" would be stated in the text only for a receipt-encoding fallback. That would be a typed mapping from `ReceiptFailure::check.wire()`, never Debug text.
- **Every other refusal:** exact ordinary bytes (N0).
- **Space:** the notice is reserved before W1 starts (COMP:66), so rendering it cannot fail.

**Open fact for ROOT (unverified by me):** whether the base `preview-physics-1` readers and carriers accept a `RETAINED_PRECISION_UNAVAILABLE` diagnostic on a base-identity publication. The successor readers' G4 rules cover only successor identities.

## 3. Precommit validation: its position and failure route. No ruling needed (decision 5).

**Implemented:**
- `retained_w1` runs on the reserved-stack thread and calls `result_export::retained_precision::validate(&successor, Some(&invocation))` after serialization and before the transfer. `invocation` is `{request: <the captured raw request>, solver_mode}`.
- An `Err` is `W1Fallback::Precommit{gate, code}` with the reader's first failure, and returns the untouched ordinary bytes (with the item 2 ruling's notice).
- Eligibility stays off. Success means G0–G8 passed with standing `needs_recompute`.
- `result_export` is now a runtime path dependency (decision 5). `Cargo.lock` is unchanged, because it already held the package as a dev-dependency.

**Cost for G4:** building the invocation `Value` deep-copies the raw request, and the reader's temporaries follow. Both are priced under design-to-budget (COMP §3 "Hash/validate").

## 4. The test strategy under decision 7. No ruling needed.

- **No permit in maintained code.** The permitted branch of the dispatch exists and calls the API.md interface: `admit` → reserved-stack thread → observed run with the permit-bound observer → G-B inside the capture → G-C → `retained_w1`. It is statically unreachable, because `RegisteredProfile` is uninhabited.
- **Committed tests (PP)** call `retained_w1`, the exact body the permitted dispatch runs after G-C. They use an observer installed in the actual single ordinary run, as U1's private driver does:
  - the pinned U1 successor bytes in both modes, with the returned ordinary envelope byte-identical to the plain route;
  - fault controls at preparation, native, proof (Maxima and ValuesCompletion), serializer, precommit and coexistence. Each falls back to the preserved ordinary bytes;
  - the no-permit Direct entry is the ordinary route, with no W1 result;
  - the reserved-stack runner: a 3 MiB frame on an 8 MiB reservation, borrowing without a clone, panic re-raised with its original payload, and a spawn failure that does not run the work.
- **Test-only fault seams:** `#[cfg(test)]` thread-local hooks in `lib.rs::retained_tests_hooks` for the native stage (the source is withdrawn) and for precommit (the receipt hash is corrupted). There is no test permit.
- **Unreachable without a permit:** G-B, G-C, the dispatch's permit branch and `permitted_run`. These are exercised once end to end in a disposable archive behind a stub (evidence, not code), and are committed in grant 2 after U4 G5.

## Coordination note for ROOT and I65 (D-5)

U3's dispatch needs the API.md §2 names to compile. I added a minimal shim in `retained_memory.rs`, marked "U3 consumer shim (D-5; API.md §2); U4 G5 replaces these bodies and fixes the fact fields (G3)":
- `admit`, with today's census and `admission` refusal; `assess` now wraps `admit`, with unchanged results;
- `#[derive(Clone, Copy)]` on `CapturePermit`, so the observer can hold it for G-B;
- `CapturePermit::{reserved_stack_bytes, check_late, check_complete}`, with uninhabited-match bodies;
- `LateFacts` (borrowed model, built, materials, case, restrained and springs), `CompleteFacts` (the borrowed ordinary envelope), `PhaseGate` and `PhaseRefusal{gate}`.

No profile or permit is constructed. I65's G5 replaces the bodies and may extend the fact records; this is the merge point.

## Addendum after the checkpoint: a third ruling (R-3)

The control runs found one more decision outside the four items. Details are in RETURN.md, finding F-1.

**The issue.** Decision 5 makes `result_export` a runtime dependency of PP. That adds an edge to every maintained `Cargo.lock` that builds PP. Six such files are outside the write fence:
- runner/headless and apps/desktop/src-tauri gain one line each;
- self_weight_wasm, operation_applier, and the numerical_integrity and physics_audit_regression benchmarks each gain the dependency line and the `open_pipe_stress_result_export` package block.

**What does not change.** No registry package or version changes. `serde_json`'s `float_roundtrip` feature was already on in all six, because PP itself enables it.

**Why it blocks.** CI's numerical cargo suite runs `cargo fetch --locked` and `cargo test --locked` for every discovered manifest. Without these edits the branch cannot pass CI.

**Proposed.** Extend the fence to exactly these six lock deltas (`_run_records/downstream_lock_deltas.diff`). The alternative is to revisit decision 5's runtime dependency.
