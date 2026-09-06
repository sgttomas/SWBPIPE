# Independent P1 review

Verdict: PASS for bounded manager fan-in. No actionable finding. This is not final integration, publication, lifecycle, or engineering acceptance.

Source binding: `INPUT_VERIFICATION.json` independently verifies all 67 P1 manifest members and both live source hashes. P1 manifest SHA256 is `30600919d738e858ce002fb402762f3955ccaa4082ffb5cef50239b408e38369`. Product source is `171a63860bb9d105e2336bb76be6a71b26032cca5795d55ba3556f8c08ba3d26`; nonlinear source is `fb02a52273637f844c63ac2e1545bc7fbe8ab79b58d7f5e259829e46be4b1439`. Every changed line in the two frozen patches was reviewed, including kernel overlap and tests; COVERAGE.csv records each patch hunk. Live HEAD-relative diff also checked for overlap context.

## Findings

None requiring remediation within this bounded return.

## Behavioral review

- Selected converged nonlinear displacements replace the preliminary vector before ordinary nodes, support actions, end/station recovery, stress, movement warnings, and maxima. Selected reactions are used with the same vector. Failure/nonconvergence blocks the envelope; outer load-case fan-in rejects blocking recovery diagnostics. Preliminary linear mode/parity rows remain explicitly linear evidence. Contact-only initialization remains held.
- The new spring adapter preserves the prior function and input DTO. Canonical spring diagonal contributions are validated and assembled before every active-set iteration. Ordinary spring reaction uses -k*u; contact/rigid actions use the selected assembled reaction. Translational hypot excludes rotational moments. The internal signed support map remains unrounded for P5. No new public schema/result kind was introduced.
- Ground DOF count is a necessary lower bound, not a rank test. Unique rigid and positive spring DOFs count; duplicate restraints do not inflate it. Six-spring and five-plus-spring controls use actual solver acceptance. This does not guarantee detection of every mechanism under the existing pivot policy.
- Finite guards cover assembly, selected primary state, reaction/work arithmetic, stress components and numeric publication. NaN no longer disappears into residual maxima. Optional overflowing deltas and sparse observations become unavailable; representable large values avoid rounding-only overflow. Existing errors and tolerances are retained.
- The historical no-spring mixed-friction fixture explicitly removes the ground spring and retains prior exact compatibility numbers. It is not presented as an independent physics oracle. Independent spring/gap and selected-state parity checks supplement it. D01 remains open for retained-spring mixed friction numerical adequacy; no current-normal friction assertion or production policy change was smuggled in.

## Evidence and downstream gates

Decoded and SHA-verified final product, bounded product, nonlinear and P9 logs. Product full suite is 129 PASS / one FAIL. The enabled failing fixture assertion is specifically generated N-140 UY zero versus unchanged fixture .332485 at product source line 14201. This matches selected-state correction and does not waive other failures. Final fixture generation and full suite must pass downstream.

Nonlinear suite is 22 PASS. P9 is two PASS/eight FAIL: selected closed stop and inactive-gap spring pass; remaining station, algebra, precision, link and distributed-load assertions are P5 obligations. Pure-moment force assertion passes before the later station-moment failure. Original I1 replay evidence and independent pre-round spring expectation are preserved in the verified P1 packet; reviewer did not rerun builds. Scope validator PASS for the two assigned source files. Registered affected checks are evidence-sweep, piping-pytest and harness-self-check; they remain final root gates, not waived by local evidence.

Applicable reviewed contracts include DEL04-01 assembly/boundary/finite diagnostics, DEL04-04 REQ03/04/05/06/09, DEL04-06 diagnostic/status requirements, DEL05-03 governed mechanics consumption, and OPS-K-UNIT-1/DATA-2/SOLVER-1/SOLVER-2/MECH-2. No new pressure, connector, friction-history, threshold, public-schema or migration choice. Package cross-boundary consumer integration is explicitly granted by the sealed brief.

P5 may consume this exact source after manager/root acceptance. Preserve this snapshot; later P5 source needs fresh review and final P9/global/fixture gates. No user app, network, source mutation, Git mutation or builds by reviewer. Native Agent2 role is instruction/config asserted; model unexposed.
