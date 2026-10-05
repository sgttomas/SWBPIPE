# API.md amendments (RV84 S-6; RR "RV84 on U4 G3")

This amends `R/I65/u4_g2_01/API.md` §2 and G3 COMPOSITION.md §4. Each row names the line it replaces.

**Basis:** U3 grant 1 at NUM `b1f80234dc`. It already declares `LateFacts`, `CompleteFacts`, `PhaseGate` and `PhaseRefusal { gate }` in `retained_memory.rs:312–335`, with uninhabited bodies.

**Who changes the code.**
- The `PP/lib.rs` and `retained_product.rs` hook changes below are **I61's** (integration owner, D-5), to this specification, in U3 grant 1b or grant 2, or at the G5 integration point.
- `retained_memory.rs` is U4's (G5).

## 1. Signatures (replace API.md §2's code block)

```rust
// G-A: census + D1 predicate (D1.0-D1.11) + build status (D-6) + the cap-priced admission bound
// (COMPOSITION_G4.md: the maximum over both branches of every phase through completion,
// <= M, with the 0.9 M margin checked at G4/G5 registration). Allocation-free. Refuses Headless
// under D-2 (U3 F-5).
pub(super) fn admit(capture: &CapturedInvocation, request: &LinearStaticPreviewRequest,
                    entry: Entry<'_>) -> Result<CapturePermit, RetainedAdmissionReport>;

impl CapturePermit {
    pub(super) fn reserved_stack_bytes(&self) -> usize;                       // R (STACK_PLAN.md)
    pub(super) fn check_late(&self, facts: &LateFacts<'_>) -> Result<(), PhaseRefusal>;
    pub(super) fn check_complete(&self, facts: &CompleteFacts<'_>) -> Result<(), PhaseRefusal>;
    pub(super) fn budgets(&self) -> &'static PhaseBudgets;                    // TRANSFER_COMPLETION.md §3
}

/// G-B, at `ProductCapture::prepared_case_source` (retained_product.rs:3222; PP lib.rs:5269),
/// immediately before the late old-source capture, only when `source_selected == false`.
pub(super) struct LateFacts<'a> {
    pub(super) model: &'a PreviewModel,
    pub(super) built: &'a BuiltModel,
    pub(super) materials: &'a [MaterialInput],
    pub(super) case: &'a PreviewLoadCase,
    pub(super) restrained: &'a [usize],
    pub(super) springs: &'a [SpringEntry],
    pub(super) capture: &'a ProductCapture,          // NEW (S-6(c)): the observer's own capacities so far
}

/// G-C, after the complete ordinary owner returns (PP lib.rs:2938).
pub(super) struct CompleteFacts<'a> {
    pub(super) ordinary: &'a MechanicsEnvelope,
    pub(super) capture: &'a ProductCapture,          // NEW (S-6(c)): includes the late capture, measured after it is made
}

pub(super) struct PhaseRefusal { pub(super) gate: PhaseGate, pub(super) fact: PhaseFact, pub(super) observed: u64, pub(super) cap: u64 }
```

## 2. The facts each gate reads (replaces COMPOSITION.md §4's two tables)

### G-B: `LateFacts`

The hook receives `source_selected, model, built, materials, case, restrained, springs, application, thermal, pressure` (RV84 S-6(a)). **It receives no result rows, no diagnostics and no retained errors,** so G-B reads only these:

| Fact | Read from | Must be ≤ |
|---|---|---|
| `built_nodes`, `built_members`, `built_supports` | `built` lengths | n, m, g (a re-check of the census) |
| `case_loads` | `case.primitive_loads.len()` | l |
| `restrained`, `restrained_capacity` | `restrained.len()` / `.capacity()` | k = min(N, r) |
| `springs`, `springs_capacity` | `springs` | s |
| `materials` | `materials.len()` | 8 |
| `observation_bytes` | the capacity arrays in `capture` so far (adapter, observation, support, prepared) | T11's bound without the late capture |
| `legacy_selected` | `source_selected` | false: G-B is reached only on branch W |

**Moved to G-C, because they cannot be read at G-B:** `case_rows`, `case_row_text_bytes`, `diagnostics`, `diagnostic_text_bytes` and `retained_error_text_bytes`.

### G-C: `CompleteFacts`

| Fact | Read from | Must be ≤ |
|---|---|---|
| `envelope_results`, `…_capacity` | `ordinary.results` | P_final = 2,115 |
| `envelope_result_text_bytes` | Σ of each row's String capacities | 2·P_final·Text(row) = 48,535,020 |
| `envelope_diagnostics`, `…_capacity` | `ordinary.diagnostics` | D_env = 11,408 (G4 atom) |
| `envelope_diagnostic_text_bytes` | Σ of each diagnostic's String capacities and refs | **2·Text(diag_env)** = 157,720,102 at the G4 atoms (S-6(d); G3 printed "2·Text(diag_total)" beside 2·Text(diag_env)'s value) |
| `contract_evidence_census` | the borrowed Value census of `ordinary.contract_evidence` | the PREVIEW facts (ordinary_caps.py) |
| `source_block_recovery` | `ordinary.source_block_recovery.is_some()` | false (coexistence is checked by U3 first) |
| `observation_bytes` | `capture` capacity arrays | T11 |
| **`late_capture_bytes`** (NEW, S-6(c)) | the late old-source capture's capacities, `capture.P1_old_source` | T11.P1 |
| `ordinary_seed_bytes` | `capture.ordinary` capacities | T11.4 |
| `retained_error_text_bytes` (moved from G-B) | the maximum and record error Strings in `capture` | (3m + 1)·Text(err) = 1,589,248 |

**Dropped (S-6(b)):** `source_cases` and `source_case_row_json_census`. The local at PP lib.rs:2679 (`b1f80234dc`) is consumed by the finalization (:2860–2862) or dropped before the ordinary owner returns. The envelope has no such field.

**What the gates are for.** Each fact is a borrowed length or capacity read, allocation-free where its owner is in scope. Each bound is cap-priced. A fact above its bound means the derivation missed something, and the gate refuses to the ordinary path (N1: ordinary bytes, because no W1 work has run yet).

**What the gates are not.** They are a cross-check of the derivation, not the memory bound itself. TAV cannot be read at a gate.

## 3. The hook change I61 owns (D-5)

1. **`prepared_case_source`** passes `&self` (the observer's `ProductCapture`) into `LateFacts`. It already holds it as `self` (retained_product.rs:3222–3233); this is a struct-field addition at the call into `permit.check_late`.
2. **`permitted_run`** builds `CompleteFacts { ordinary: &ordinary, capture: &observer }` (lib.rs:2938). The observer is in scope there; it moves into `retained_w1` afterwards.
3. **`PhaseRefusal` gains `fact`, `observed` and `cap`.** U4 G5 defines the `PhaseFact` enum.

None of these changes a published byte.

**At `a634ac8b53`** (grants 1b and 1c, merged during G4):
- G-C is `observer.permit().map(|permit| permit.check_complete(&CompleteFacts { ordinary: &ordinary }))` (lib.rs:2999).
- G-B is at retained_product.rs:3244–3246.
- The permit is linear and owned by the observer.

Item 1's `capture: &*self` and item 2's `capture: &observer` are shared borrows beside those reads, so §3 applies unchanged (TRANSFER_COMPLETION.md §7).
