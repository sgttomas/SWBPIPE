# D-71 — second ruling addendum: the maturity sentence is removed

Status: RULED — explicit in-session owner direction, 2026-09-18. It supersedes item 1 of the [D-71 ruling record](D-71_RULING_2026-09-18.md) (option B) and corrects ROOT's reading under item 3 of that record. Precedent and form: the [first addendum](D-71_RULING_ADDENDUM_2026-09-18.md). Earlier records are not rewritten; this addendum governs where they differ.

## Owner act

The ruling record read the owner's "Do not call it a \"Technical Preview\"" as being about the product's name, kept the maturity sentence in the information popover and About under item 1 B, and said so to the owner so the reading could be corrected. The owner corrected it (stored transcript timestamp 2026-09-18T14:01:23.493Z, 82 bytes of UTF-8, SHA-256 `5c01428f3a49a383e95b1a5a6871fd5f8ad05c031f148e057f886b11101d0f78`; an in-session extraction from the host's stored transcript, not original transport bytes):

> No, remove "Technical preview — not a released product." in all instances of it.

## Adopted bounded effect

1. **The maturity sentence is removed from every live surface.** No product surface carries "Technical preview — not a released product." or a variant: not a banner, footer, status bar, information popover or About; not packaging or build-readiness surfaces, which the registry allowed to reuse it; not the user-facing documentation. **Registry act:** `BS-MATURITY` is retired as a boundary statement with a placement. Item 1's option B, ruled earlier the same day and codified as `DEC-099`, is superseded before it was ever executed. Codified as `DEC-105`.
2. **What the removal is not.** It is a decision about what the product displays. It does not change the ruled stage record, from which the sentence was derived, and it is not a release, a lifecycle promotion or a statement about the product's maturity. ROOT records this boundary as its reading, stated so it can be corrected.
3. **Ruled history keeps its words.** Run records, decision packets, earlier mock frames, evidence logs and retained screenshots that contain the sentence are historical and are not rewritten. "All instances" is read as all live instances: product, packaging, registry, user-facing documentation and the current design documents and frames.
4. **Obligation carried to the implementation tranche.** The claims lint requires the sentence to be present in the app shell source (`MISSING_MATURITY_BANNER`) and embeds it in its registered-texts list (`tools/validation/validate_claims_language.py` at the repository root, lines 86 and 104 to 106 at `0d0021db71022a615f6ac162f273c03695c7beab`). Removing the sentence from the product without changing the lint fails the lint. The lint, the registry and the product therefore change in the same tranche, and the root tool edit needs its own instruction-surface authorization.

## Design consequence

Design system V1.2, the UX specification V1.1 and the regenerated frames carry no maturity sentence; disclosure M-01's homes are removed. The running design-system child was told by message, recorded in the run's brief index.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
