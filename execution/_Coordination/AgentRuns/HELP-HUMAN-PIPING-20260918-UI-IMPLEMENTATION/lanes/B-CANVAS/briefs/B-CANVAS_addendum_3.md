# Sealed addendum 3 to brief B-CANVAS: the early code review of slice C1's first part, and ROOT's disposition of its findings

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19T03:46Z, while the lane's manager is stopped. It is part of the lane's brief and binds as the brief does. Path placeholders as in the brief.

## The review

Read the retained return first: `{RUN}/lanes/B-CANVAS/reviews/C1-REVIEW_RETURN.md` (SHA-256 `5f87c173e0a9f6373de5991eb31bfbedffcffdaea0ee6ba4752e8af7b7b3e08f`). The reviewer was Claude Opus 5, read-only, in a fresh context, in another worktree at your commit `cc17cda6e`, whose tree under `apps/desktop` is the candidate `beb69d603`. Verdict: FINDINGS, two minor and two trivial, none blocking. It found picking byte-identical and green, the two held colour families held exactly, nothing under `e2e/**` touched, every `canvas.*` binding checked against design system §6, the repaint path correct in both themes with nothing created or disposed, the eleven removed builders and the two lights confirmed dead, and the figure shader correct against three 0.181.2 as installed for instancing, per-instance colour, the view-space vectors, the output colour space and the transparent form. What it could not check, because it ran no browser: that the shader compiles and links on a real context, and any drawn pixel.

## ROOT's disposition

| Finding | ROOT's decision |
|---|---|
| F1, minor: `deformedShape` is bound to `canvas.vector`; design system §6.8 draws the deformed shape "solid with the result colour or the pipe neutral" and gives `canvas.deformGhost` to the dashed undeformed outline; with result colour on, deformed tubes and load vectors would share one ink. | **Keep the binding for this lane, as a named provisional one.** This is ROOT's decision and the owner may reopen it. §6.8 swaps which shape is solid: the deformed shape becomes the solid one and the undeformed one becomes a 1 px dashed ghost. Today's product draws both solid at once, so the pipe neutral cannot tell them apart, and the ghost needs the edge-line mechanism and result colour, neither of which this tranche has. When §6.8 is built the role disappears, so the collision the reviewer describes is never reached. What you change now: the comment at `viewportPalette.ts` near 121 and the slice record's §6.4 say that the design's pipe neutral was weighed, why it cannot be used while both shapes draw solid, and that the binding is provisional until §6.8 is built. No role string changes. ROOT carries §6.8's deformation view as an open item for the results work. |
| F2, minor: the figure material's `color` property is defined non-configurable on the instance, so it does not survive `clone()`, while `userData` does; a cloned figure material would pass `isFigureMaterial` and silently never repaint. | **Accepted. Fix it before C1E builds inside the material.** Prefer the durable form the reviewer names, a `FigureMaterial` class that extends `THREE.ShaderMaterial` and defines `color` in its constructor so that `clone()` rebuilds it; the descriptor change is the fallback if the class disturbs anything. Test first: a clone satisfies `isFigureMaterial`, exposes a `color` that is its own `tint` uniform's value, and repaints with the theme. Its own commit. |
| F3, trivial: the drawn alpha comes from the `opacity` uniform while `material.opacity` stays 1, so a later slice that sets `material.opacity` (C3's dimming is next) would change nothing and raise nothing. | **Accepted. Same commit series as F2.** The two can never disagree: either mirror the value or expose the uniform through an `opacity` accessor as `color` is exposed. Test first: setting the material's opacity changes what the shader reads. |
| F4, trivial and older than the lane: `registerSelectionPresentation` is imported into `PipeViewport.tsx` and never called. | Drop the import line when C1E next touches the file; keep the export. |

## Three of its observations become instructions

- **A texture in a uniform is invisible to the ownership ledger.** `countOwnedObjects` and `disposeObjectChildren` find textures by scanning a material's own values, not its uniforms. If any mechanism of C1E, or any later slice, gives a figure material a texture uniform, the same slice extends the ledger's rule and adds the test that counts and disposes it. Mechanism A as P2 describes it needs none.
- **The silhouette shade of roles other than the tube is a derived colour.** One ratio, `canvas.pipeShade` over `canvas.pipe` in linear terms, shades every figure material, so a support's or a node's silhouette draws a colour that is in no token. Say so in C1E's return, with the measured contrast of the worst such silhouette against the ground in both themes, so that the design-fidelity review can judge it.
- **Roles the design does not name** (`node`, `routeGridAxis`, `routeGridLine`) are your judgement, not the design's word. Nothing to change. ROOT carries them as open design items.

Its note on the first profile's rim band (the light major grid line is 49 off white on the red channel, where the oracle's band is 48) needs no action: after C1b the first profile is the recorded demonstration and no longer describes the lane's product.

## Backcheck

Name the commits that answer F1 to F3 in your C1E return. The same reviewer backchecks them then, and the combined candidate of the lane's first pull request still gets its own complete review.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
