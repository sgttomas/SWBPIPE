# N5 Return — Fresh implementation verification

**Verdict:** `BLOCK`

## Material findings

1. **Native-package desktop route uses the wrong fixed context.**
   `ControlledExportLink.tsx` recognizes `native-package-export-link`, while
   `NativePackagePanel.tsx` uses `native-package-link`. The route falls back to
   `DOTH-JSON-001/local_private`, and tests authorize local-private exposure,
   instead of required `DOTH-HANDOFF-002/downstream_tool`.
2. **TypeScript infers a global public basis from two false flags.**
   `redactionExportControls.ts` treats
   `private_payload_included:false` plus `protected_content_included:false` as
   sufficient to classify otherwise unmetadataed leaves
   `public_metadata/public_permissive`. Missing metadata must remain unknown.
3. **Rust private-path classification precedes unknownness.**
   `redaction_binding.rs` classifies null/TBD private-named values as known
   private, producing `LOCAL_PRIVATE_INTENT_REQUIRED` blockers rather than the
   unchanged local-private `warning_only` result. A focused binary execution
   reproduced this for runner fields.
4. **Renderer-blocked HTML reaches an iframe.**
   `RenderedReportPanel.tsx` hides save/print on renderer `export_blocked`, but
   the preview iframe remains unconditional. The current test is masked by an
   earlier pre-IPC redaction block and does not reach the renderer-blocked
   outcome.
5. **Required subprocess-level runner evidence is absent.**
   Current Python checks inspect source ordering and Cargo tests exercise
   internal functions, but neither invokes both binaries across active verbs,
   stdout, `--output`, and exits 0/1/2 as candidate §9 requires.

## Positive checks

- Candidate §6 containment: zero violations.
- All 31 routes/dispositions present.
- Protected/release tools and parity corpus unchanged.
- All six recorded checks pass; exactly one sweep exists.
- No premature deliverable state, receipt, or Git closeout.

No repair or state effect was performed. W3 must remain held. Remediation
requires a new bounded N4 attempt and a fresh N5 review.

