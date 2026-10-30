# D-06b — Signing/notarization re-decision for the DEC-057 macOS artifact

**Date prepared:** 2026-07-25
**Prepared by:** HELPS_HUMANS under the owner's bounded R19 preparation
direction.
**Epistemic status:** `PROPOSAL / AWAITING_RULING`.

Only the human project authority may select an option. This packet does not
record or imply a ruling.

## 1. Decision statement and exact scope

Decide the future signing/notarization posture for the one release-artifact
shape already selected by D-06/`DEC-057`:

- matrix: macOS Apple Silicon (`aarch64-apple-darwin`) only;
- package: the Tauri `.app` bundle distributed as a zip archive;
- present posture: unsigned, with SHA-256 checksum, commit-bound `DEC-025`
  sweep evidence, the `docs/BUILD_AND_RELEASE.md` §8 release-artifact record,
  and the unsigned-install caveat.

D-06b does not reopen the release matrix, package format, publication target,
container naming, or evidence-gated matrix-expansion rider selected by
`DEC-057`. It does not decide signing for Windows, Linux, or another artifact
shape.

## 2. Corrected authority and traceability basis

All paths are relative to `projects/chirality-piping/`.

| Surface | Verified fact |
|---|---|
| Archived PRD v0.1 | `docs/_history/PRD_v0.1.md` §22.6, “Release R5: Engineering Beta,” listed “Signed releases.” This is historical text, not the current PRD R5. |
| D-21 / `DEC-056` | D-21 Annex A identified v0.1 release-machinery items that had no v0.2 milestone-list home. `DEC-056` carried signed releases, issue templates, redaction workflow, and IP contribution process as explicit **R6-entry residuals**. |
| Current PRD | `docs/PRD.md` R5 is “Piping Components and Nonlinear Supports.” Current R6 is “Design Knowledge and Handoff Beta” and does not itself list signed releases. D-06b must not describe the historical v0.1 token as a current R5 requirement. |
| D-06 / `DEC-057` | D-06 Option O-A selected the exact macOS Apple-Silicon `.app`-zip posture above, expressly unsigned for v0.1 and re-decided at D-06b. D-06b is the residual vehicle for that exact signing posture. |
| Build/release guide | `docs/BUILD_AND_RELEASE.md` records the ruled unsigned posture, authenticity chain, caveat, and D-06b re-decision. It is guidance and evidence structure, not release authorization. |
| DEL-10-04 | Its `_STATUS.md` Remaining section correctly re-homes the historical v0.1 §22.6 token through D-21 Annex A to the `DEC-056` R6-entry residual and gates the signing/notarization disposition on D-06b. |

Bound preparation-basis hashes:

- D-06 packet:
  `17bfaa1de4c90cbb8d031eb375826c915446240c2f10e32d0aec9a9ba1524956`;
- D-21 packet:
  `029b25fd16b3f680eb49d55b1de108ffe44531fd78df0dc0270385436103f0a4`;
- `SOFTWARE_DECOMP.md`:
  `6536db3aa86ad0eae22ede93ceedb6e52f0ce33264b135812593b14c92045349`;
- current PRD:
  `9c3bccd8d2eb8e68c10e05d50bdd29619892196c98069a2587ba3a1ff4880793`;
- archived PRD v0.1:
  `d165efc02bff002e629d5734490bb51014d0f7336cdf38b7b52f41f9c89d509b`;
- DEL-10-04 status:
  `097ea5ecc389cdadfb8ada41da6c0066cc4d9ce447430150171ce476f1efca04`;
- `BUILD_AND_RELEASE.md`:
  `b0ea8de3a3338d8f0c767f898fbf2ab00d146eb6e9cf27c221430ea354a46066`.

## 3. Decision constraints

1. The decision is limited to the existing `DEC-057` macOS Apple-Silicon
   `.app`-zip residual. A disposition here does not silently govern another
   platform or artifact.
2. The current checksum, sweep-artifact, release-record, and unsigned-caveat
   requirements remain until a later owner ruling and bounded implementation
   change them.
3. Apple Developer enrollment, spending, legal-account acceptance, signing
   identities, credential generation/storage, network/notary use,
   implementation, publication, and release are distinct owner/operational
   gates. Selecting a policy does not perform or authorize those acts.
4. A future documentation/status edit is a separately prepared, bounded
   candidate after ruling. No current guide or deliverable-status byte changes.
5. The historical v0.1 “Signed releases” token remains readable only through
   the D-21/`DEC-056` carry-forward. No option rewrites historical PRD or ruled
   history.

## 4. Options

| ID | Option | Exact disposition | Consequence |
|---|---|---|---|
| **O-A** | **Adopt a standing unsigned deviation for the existing DEC-057 artifact.** | The macOS Apple-Silicon `.app` zip remains unsigned and unnotarized. Its required authenticity posture remains checksum + commit-bound `DEC-025` sweep artifact + §8 release-artifact record + unsigned-install caveat. | Intrinsically closes only the existing `DEC-057` signing/notarization residual as a standing unsigned deviation unless a later owner ruling changes it. It does not close other `DEC-056` R6-entry release-machinery residuals, govern another platform, or create a release/publication claim. |
| **O-B** | **Adopt policy-only future Apple Developer ID signing and notarization.** | A future bounded implementation may target Developer ID signing and Apple notarization for the existing `.app` zip. | Policy selection only. Enrollment, spend, agreement acceptance, credential handling, notary/network access, implementation, evidence, publication, and release each remain separately gated. The unsigned posture continues until those later gates are explicitly satisfied and accepted. |
| **O-C** | **Defer with an exact trigger and successor row.** | D-06b becomes `RULED` as a deferral. At that eventual ruling, create one new, next-free rechecked `D-XX` row in `NOT_PREPARED`, bound to the earlier of: (T1) before an owner-authorized first external/public distribution of the `DEC-057` macOS `.app` zip, or (T2) before any Apple Developer Program enrollment, signing-identity purchase, or credential-acquisition act. | No signing policy is selected now. The successor row, not D-06b, carries the future trigger. Reaching either trigger stops the proposed act until that successor is prepared and ruled. |

## 5. Recommendation

Recommend **O-A**, non-bindingly.

O-A is the smallest disposition consistent with the selected artifact and
current operating posture: `DEC-057` already defines an explicit evidence
chain for an unsigned `.app` zip, while no enrollment, credentials, signing
implementation, notary workflow, publication surface, or release act is
authorized here. It closes the exact residual without pretending that a
policy choice performed operational work.

O-B is viable if the owner wants the future policy despite its separate cost,
credential, network, implementation, and release gates. O-C is viable if the
owner prefers to postpone the policy choice; its trigger-bound successor
prevents the deferral from becoming silent debt.

## 6. Owner decision

**Status:** `OWNER-PENDING`.

- Selected option: —
- Conditions or riders: —
- Ruling record: —
- `DEC` codification: —

Preparation and validation are evidence only. They do not select an option.

## 7. Future ruling and downstream mechanism

If the owner later rules:

1. Create a separate D-06b ruling record that binds the selected option and
   any conditions.
2. Recheck the live end of `execution/_Decomposition/SOFTWARE_DECOMP.md` §12
   and append the next free `DEC-XXX` codification. Do not assume the current
   next number remains free.
3. Transition only the D-06b register row from `AWAITING_RULING` to `RULED`
   with packet, ruling, and rechecked `DEC-XXX` pointers.
4. If O-C is selected, allocate the next free live `D-XX` identifier and add
   exactly one `NOT_PREPARED` successor row containing the T1/T2 trigger in
   §4. D-06b still becomes `RULED`; it does not remain awaiting.
5. If the selected option warrants guide/status alignment, prepare a separate
   bounded candidate for `docs/BUILD_AND_RELEASE.md` and DEL-10-04
   `_STATUS.md`. A ruling does not itself authorize those edits.
6. Any enrollment, spend, credential, network/notary, implementation,
   publication, or release action requires its own applicable authority and
   evidence gate.

None of these future steps is authorized by this preparation packet.

## 8. Preparation exclusions

This R19 preparation does not rule D-06b; sign, notarize, build, package,
publish, or release an artifact; enroll in a program; incur spend; accept an
external agreement; create, read, or store signing credentials; use a notary
or network service; edit PRD, decomposition, D-06, D-21, DEL-10-04,
`BUILD_AND_RELEASE.md`, product, code, tests, lifecycle, DAG, or release state;
or perform Git closeout.

Standard claim fence applies (F-PIP-2; DEC-081 claims taxonomy).
