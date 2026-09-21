**Bounded four-path PASS. No actionable finding.** Reviewed `7b900eaad764603a3c58dd643ebaa34ea569a1d9..a0649cbc9a434a28144d010a3a19d68111ace412`; the correction is suitable for the rebuilt native follow-up.

The repair captures `initialSelection: "all" | "end"` at edit start and initializes selection once per edit token. Character-start places the caret after the replacement; Enter/double-click selects existing text. Refocusing preserves subsequent caret placement. Direct/review, numeric/text, asynchronous ownership and Apply/Keep/Cancel paths retain their existing contracts. No timer, global-event workaround or unauthorized production change was introduced.

All four maintained paths match the frozen hashes:

- `EngineeringTable.tsx`
- `tableState.ts`
- `EngineeringTable.test.tsx`
- `e2e/b4-table-editing.spec.ts`

Existing regression assertions remain unchanged; new tests use separate key events. **R1 remains closed:** both repaired passive-read test files are byte-identical to `97c03e3`.

Evidence reviewed at committed tail `ad70730a6207dcf7643124aaed160d48cd11eca4`, whose desktop diff against `a064` is empty:

- Fail-before: separate `q`, `r` events expected `qr`, observed `r`. Production hashes match the unchanged baseline; the failing test source preserved inside the trace matches its recorded hash.
- Pass-after: **10/10 browser cases**, covering both profiles, all four direct/review × Label/X combinations, and the existing connected text journey.
- **51 core tests passed** across four files; frozen TypeScript exit 0.
- All **27 manifest-bound payloads** match their committed sizes and hashes.

Consulted identity hashes:

| Record | SHA-256 |
|---|---|
| Backcheck launch | `0c608f56d66958ca6faafb2870723d16035b7d9b72f58f52b2770db8657967f2` |
| Repair disposition | `6aa3d0af59934119154d9b3eb37b587098d22fecc013886f83400133b373b21d` |
| `SOURCE_FREEZE.json` | `ce0d0b6724bd32f123cd618c96715db2fb3173f2512b90e3a3befeb14eee80d7` |
| `MANAGER_SOURCE_HANDOFF.md` | `cdf3ebadf3d187c1128fa42a89b5aef38779a7f56dd568dcfb16b498325c7c0f` |
| `CHECK_COMMANDS.json` | `022c567f90b1557fc33c21a27a070a55e70f76860640bd5e6236855b56f8ffc3` |
| Fail-before log | `310cb618a62876829e94d6a466578d7ff9e39645b92f7d9a0dbd7a23e86e657f` |
| Pass-after log | `31ebb1ab4e10ebd230338319cc21b88c409f41ec28499c06ed6af75c38a02307` |
| `ARTIFACT_HASHES.json` | `3d5b0778fd4ee969ba4d1900efeeb2bcfbd83856ea38c7e92895bac57fe476f7` |

This was read-only source/evidence review; I ran no tests, builds or UI actions. Rebuilt-native character/caret/text-Undo/Cancel verification, remaining native exposure checks, the clean sweep and actual-head CI remain pending. Earlier native failures retain their original standing.
