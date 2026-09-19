# Procedure: a probe-only build copy that isolates one cue

Written by B-CANVAS on 2026-09-19 after slice C2, at ROOT's request, so that the second profile's preparation can repeat it. Placeholders as in the lane brief.

**Why.** A pixel rule counts whatever satisfies it. In C2 the product's halo build passed the pair rule 199 to 200 of 200 only because the first profile's diamond cue (Selection-coloured, with a `canvas.bg` rim since C1b) satisfies the rule alone. To read one cue, every other source of the counted colour must be absent from the picture, in a build that is otherwise the product's.

**Steps.**
1. Start from a clean tree at a committed head. Record `shasum -a 256` of each file you will touch.
2. Make the smallest edit that removes the other cue **at the point where the product last sets it**, and mark the line `// PROBE-ONLY COPY`. For the diamond that point is the end of `ViewportResource.updateSelectionCue()` (`if (this.selectionPresentation) this.selectionPresentation.group.visible = false;`), because the cue's own `update()` sets `group.visible` on every call; hiding the group where it is mounted does nothing.
3. Build through the lock (`sh {RUN}/tools/with_e2e_lock.sh npm run build:desktop` from `{WORKING_ROOT}`), copy `{DESKTOP}/dist` outside the repository, record the copy's `index.html` SHA-256.
4. Restore the tree (`git checkout -- <file>`), confirm the recorded hashes, confirm `git status --short` is clean. Nothing of the copy is ever committed, and no switch enters the product.
5. **Prove the edit took effect before spending a full run**: search the minified bundle for the statement, and run ten samples against the product's build. If the numbers do not move when the subject is removed, the probe is measuring something else. (C2's first copy failed this test: its four full runs equalled the product's to the pixel, and were deleted.)
6. Label every output with `--note` as a probe-only copy that is not the product, and name the copy's hash in the record.

**What the second profile should take from it.** Its witness needs a negative control of this kind built in: the halo rule must fail on a picture that holds the diamond and no halo, and the capture protocol must say whether the first profile's diamond is still drawn when the second profile measures (it is today; `viewportSelectionPresentation.ts` is byte-identical by ROOT's decision on P1 ASK-6).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
