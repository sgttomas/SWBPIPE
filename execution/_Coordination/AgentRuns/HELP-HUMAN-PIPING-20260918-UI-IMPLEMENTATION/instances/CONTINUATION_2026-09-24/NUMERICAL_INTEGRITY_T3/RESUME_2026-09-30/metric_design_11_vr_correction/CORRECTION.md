# Additive VR caller correction for RV28-VRNUM-1/-2

Status: correction candidate for RV28 same-reviewer backcheck. The sealed
metric_design_10_vr_numbers candidate and vr_numbers_06 review are preserved.
Only the two incomplete named addends are replaced, across all 24 original
RF-LARGE cases and both metrics (96 exact replacements). No kernel value is
joined and no new proof programme is opened.

## Exact correction

Let E be expected_list_initialization_while_outcome_alive and N be
nonselected_diagnostic_while_outcome_alive. For each case and each metric:

    PriorFailures = G(24,4) + 4*F(Dfail) = 96 + 4*max(8,2*Dfail)
    E_corrected = E_original + PriorFailures
    N_corrected = N_original + 78

Dfail is the existing reviewed diagnostic String-length bound, unchanged.
Its actual fixed-case descriptors and all nine candidate expressions are in
_run_records/DESCRIPTORS.json. Substitution gives PriorFailures from 1,592 to
1,608 bytes. This is retained Vec/String ownership in both metrics, with no
additional active-old term. Existing expectedH preserves all read/path/raw
Value/parser scratch/typed-list alternatives. Existing single-active-growth
terms remain unchanged.

The supplied review anchors failures at immutable lane.rs:192 and prior appends
at :230–234 and :243–245, before expected_unresolved() at :250. Those retained
owners are distinct from ctrldiag. This correction uses the same conservative
four nonrow failure slots already prepaid by solve_max; it does not add them
twice inside any one phase.

LIST78 initializes at :250 and persists through nonselected diagnostics at
:251–256 and :314–322. It is separate from cut and diag. The added 78 is typed
LIST retention only: parser scratch80/120 has dropped and is not added here.
The source-refusal branch returning before LIST remains unchanged.

Example RF-LARGE-CHAIN-n00010-AX requested: E 43,656 -> 45,256 (+1,600);
N 221,456 -> 221,534 (+78). Moving: E 43,897 -> 45,497 (+1,600);
N 221,644 -> 221,722 (+78).

## Aggregate consequence and reuse

Define M_out as the maximum of comparison, record1, E and N addends. Direct
integer checks on all 48 case/metric rows establish M_out_corrected =
M_out_original. The smallest old-M_out minus corrected-E margin is 272,896
bytes; the corresponding corrected-N margin is 99,241 bytes. Comparison alone
dominates corrected E with minimum margin 228,723 bytes. Every old/new value,
delta and per-row maximum is recorded in OVERLAY.json; independently calculated
margins and the full match to the sealed reviewer table are in DOMINANCE_CHECKS.

All four arms add the same Kernel_outcome_retained. Therefore their maximum
remains K_outcome + M_out for every nonnegative supplied common kernel upper.
The separate solve_max arm and all caller_only_phases are unchanged. Thus the
complete displayed join expression is unchanged; no numerical aggregate
underbound or actual memory excess has been demonstrated. The original two
named addends were nonetheless incomplete and must not be reused unchanged.

Reuse the original sealed table together with OVERLAY.json. Validate both
seals and the original table hash. Match the complete unique 24-ID roster,
then replace the two exact old values with their new values for each metric.
Do not accumulate deltas. A previously authenticated application may recognize
the exact new value as already applied; reject other values or identities.
Preserve every other field and all original qualifications. This is an additive
evidence overlay, not permission to overwrite either sealed packet.

## Unchanged qualification

Original source/artifact/public-type/profile restrictions, final ordinary
production transfer, fixed stable input and launch correspondence remain.
Successful direct entry, valid parsing, normal/handled I/O with successfully
constructed error owners and scoped returned errors retain their reviewed
conditions; foreign startup, fatal allocator/backend failures and unexpected
general panic/unwind are not assigned zero. No metric, window, domain,
prelaunch admission rule or sparse-profile policy changes.

Kernel/H joining, checked estimator implementation, final artifact binding,
chronological replay and required measurements remain outside this correction.
Caller-only maxima remain 29,394,825,566 requested and 43,795,065,566 moving;
these are not full E_max. The existing triangular-profile deferral implications
are unchanged. This packet supplies no checkpoint0, admission, implementation,
measurement or production acceptance. Return to ROOT for RV28 backcheck.
