# B3 continuation: decision package and proposed strategy

Prepared by HELP_HUMAN, Agent 0, 2026-09-19. Status: PROPOSED; awaiting the
owner's strategy agreement and three product dispositions. No agents launched,
no product changes or test changes made, no new runtime pass claimed. This is
preparation within the existing implementation run, not a new reusable workflow.

`{P}` = `projects/chirality-piping`; `{RUN}` = this file's owning implementation
run; `{DESIGN}` = sibling `HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.
Source hashes and reproducible Git observations are in `DISCOVERY.json` beside
this file. The active chat's owner steering is retained in `OWNER_STEER.md`.

## Orientation and next bounded work

Fresh fetch: `origin/main` is `c459a0fa15e8d33613b6b95f529b5d369c707874`.
The entry checkout was `1e3fdcf8bd4d198e4dce0b7d3b2b17b081e2b5da`.
There is no Piping diff between these revisions. The newer Root AGENTS section
explains four views of project information and explicitly requires no DAG rebuild.
Preparation now lives on `codex/swbpipe-continuation-20260919` from live main.

| Preserved work | Live result |
|---|---|
| Shell, `codex/swbpipe-b-shell-20260918`, `swbpipe-wt3` | `a9be4aaefa432e1eed56e87482391e90958cb087`, clean, local matches tracking branch. B3 remains unfinished. 37 desktop files differ from its merge base with main. |
| Canvas, `codex/swbpipe-b-canvas-20260918`, `swbpipe-wt4` | `f6c0bab8e8893237f83f57656b98039d68efb2b4`, clean, local matches tracking branch. Only I1's seven instrument files remain beyond main in desktop scope. |
| Design system, `codex/swbpipe-design-system-v14-20260918` | `4dddd44128e052cfe518f84c0b718e6125cb2b69`, clean; stays parked. |

All graph-recorded merge commits checked are ancestors of live main, including
B2F/B2G, C1 and C2. No open PR was returned. No listener was found on 5174,
5175, 5183 or 5184; no shared browser-test lock exists. This is not a census
of unrelated processes. Both working lane directories and node_modules are
accessible. No takeover or replacement checkout is needed. Their dependencies
and WASM outputs still need fresh build verification before runtime reliance.

Historical evidence: B3's last product checkpoint is `46db29e92`; the retained
lane log reports 85 unit files / 1,354 tests at an earlier merge, 19 dist geometry
failures and one compact workflow failure, and no complete final Playwright runs.
These are not freshly reproduced results. The actual source still contains the
loosened geometry helper, the armed-tool-only table-lending rule, and the strip
foot inset. B3-FLOORS is sealed but unlaunched; its blanket permission to apply
padding and general inspector lending is superseded by the continuation steer.
Do not launch that brief unchanged.

I1 at `240a246f3` is existing work to inspect and complete, not recreate. Its
manager did not finish reading the complete diff or rerun both full lanes.
Missing protocol-history bytes and historical hash bindings remain explicit
limits; B3 does not depend on closing these instrument questions.

Recommended next bounded outcome: finish and merge B3, including its native
witness, before resuming canvas implementation. Reuse the shell branch, merge
current main at a clean point, preserve C2's viewport changes, and reseal B3's
remaining brief against the owner's dispositions. Control behaviour and layout
come first. Colour, contrast, washes and fine spacing wait for the closing pass;
C1's completed colour/edge-line exception is not restarted.

## One product decision package

All three dispositions below are pending. A recommendation is not an owner
ruling. Historical ROOT decisions quoted in B3-FLOORS do not change that.

### D1 — drawer strip: retain the adopted overlay and repair its safe area

Evidence: owner direction `instances/ROOT/OWNER_DIRECTION_2026-09-19_EIGHT_UX_ITEMS.md`
item 5 adopted the overlay with scale and triad lifted clear. Shell lane
`instances/B-SHELL/returns/B3-DIST_RETURN.md` section 3 reports a 28 px strip
covering the measurement readout and the painted orientation frame at 1024.
Current `styles.css` lifts two DOM elements, while `viewportResource.ts` draws
the orientation frame separately. Moving the DOM marker alone cannot move that
painted frame. The helper `ui-foundation-workflows.ts` samples its old position.

The prior ROOT proposal was padding: keep the outer pane and deduct the strip
height from the viewport. That would address occlusion simply, but reduce the
available drawing height by 28 px wherever applied. D-72's accepted item 3 and
its packet's item 3 rationale specify canvas dimensions AND drawn pixel areas
(499,284 and 828,000). Calling the unchanged outer pane the canvas is not enough
to establish conformity. Even today's nested toolbar/readout consumes some pane
height; neither design is qualified by this discovery.

**Recommendation:** retain the overlay. Give all bottom furniture, including
the measurement readout and the actually painted orientation frame, one explicit
strip clearance. Reserve the strip's hit region; controls and readouts must not
be hidden beneath it. Preserve the expanded narrow drawer's proposed in-flow
behaviour, visible splitter and 180–600 px reachability. That is a separate
expanded state, not an instruction to overlay a 600 px drawer.

Requested bounded disposition includes the orientation witness's placement:
where the overlay is present, its expected bottom clearance becomes the strip
height plus the existing clearance. Retain its crop size, visibility, axis
semantics, passive pointer ownership and picking coverage. This is a named
change to a placement assertion, not permission to weaken oracle expectations,
Box16 endpoints, performance limits, or unrelated frozen checks. Supply measured
rectangles and before/after witnesses. A scoped viewport presentation edit is
needed in addition to the already planned routing portal; renderer construction,
resource ownership and picking stay unchanged.

Alternative: explicitly adopt padding, with both pane and drawn dimensions
recorded, and explicitly resolve its D-72 workload consequence before the
second profile is frozen. Do not silently relabel a smaller drawing as 828 px.
Reversal is local to layout/furniture placement and its named geometry tests;
there is no persistence or model change. Risk of the recommendation: more careful
coordination of DOM and rendered furniture than padding. Advantage: it preserves
the adopted mechanism without adding a new reduction in rendering area.

### D2 — inspector: approve table-first width lending in Both view

Evidence: UX specification §§2.1, 6.1 and 10.9 say that the table never reflows
and the inspector consumes canvas width. B3-FLOORS item 3 proposes the opposite.
The source implements lending only while the viewport authoring panel is active.
The existing frozen helper requires drawn width over 35% of the window and
canvas area greater than inspector area. A 231 px canvas at 1280 cannot meet
those defaults beside a 300 px inspector even after the routing block moves.

**Recommendation:** in Both view lend the docked inspector its 300 px from the
table first, down to a 320 px lending floor; only then consume canvas width,
respecting its 220 px minimum and the applicable slide-over rules. The 320 px
floor is a new lending constraint, not an already accepted specification value.
Keep the stored split as the inspector-closed split, restoring the table on close.

| Window width, default split, agent strip | Inspector closed: table / canvas | Specification: table / canvas / inspector | Proposed: table / canvas / inspector |
|---|---|---|---|
| 1440 | 737 / 603 | 737 / 303 / 300 | 437 / 603 / 300 |
| 1280 | 649 / 531 | 649 / 231 / 300 | 349 / 531 / 300 |

These are specified/derived pane widths, not fresh browser or Tauri measurements.
The canvas retains its size at these defaults but moves horizontally: its screen
rectangle is NOT invariant. Check camera and pointer mapping after the move;
record both position and dimensions at benchmark boundaries. Do not claim every
user-adjusted split can retain that width. Full narrow-case transitions remain
B5 work, with B3's supported cases and deferrals explicitly checked.

Tradeoff: table columns reflow or scroll when the inspector opens. Benefit:
selection and routing have usable space and opening their editor does not squeeze
the model projection to 231 px. This explicitly supersedes the fixed-table rule
for this state. Alternative: retain the specification and bring the conflicting
protected geometry requirements back for an explicit replacement; do not loosen
them inside implementation. Reversal: shell arithmetic/CSS and named layout tests,
without changing engineering state or stored project schemas.

### D3 — Both-view assertion: replace only the obsolete dominance comparison

Evidence: approved 55/45 Both split and UX §10.9 give 737 px table / 603 px
canvas at 1440, at the same pane height. Thus a canvas-larger-than-table rule
cannot hold in that state. The current branch also weakens other assertions:
it makes the width floor conditional and compares pane area instead of the
actual drawing; those changes are unaccepted.

**Recommendation:** restore the original drawn-canvas measurement, unconditional
35% width and height checks in the existing regression scenarios, canvas-larger-
than-inspector check, and overflow checks. In Both view only, replace the
canvas-larger-than-table comparison with the explicit approved split/allocation
geometry for the tested state. Keep the dominance comparison in the existing
Model/drawer and narrow scenarios where it applies. Do not apply a default-state
35% assertion indiscriminately to every user-adjusted split or expanded drawer.

This is an explicit design-driven assertion amendment, with the contradictory
737/603 example retained as its reason. No other floor, oracle, tolerance or
benchmark limit is waived. This disposition depends on D2 for the inspector-open
default geometry. If D2 is rejected, D3 alone does not resolve that failure.

## Delegation and model strategy — proposed, not launched

Allocation is based on this host's exposed model IDs. Official OpenAI guidance
supports assigning reasoning by task and using high effort for tracing logic and
edge cases; this table is Agent 0's judgment, not a published comparative score.
Source: https://learn.chatgpt.com/docs/agent-configuration/subagents

| Responsibility | Role / parent | Model and effort | Scope and return |
|---|---|---|---|
| Owner continuity, rulings, graph, cross-scope contracts, final integration and Git closeout | HELP_HUMAN Type 0 | GPT-6 Astra, high or greater | Agent 0 owns shared run records, briefs for managers/reviewers, decision custody, final candidate, own app inspection, sweep/CI/merge. |
| Finish B3, manage repairs and local fan-in | WORKING_ITEMS Type 1 / Agent 0 | GPT-6 Astra, high | Reuses shell lane; seals and launches its own Type 2 executors; owns App.tsx/styles.css integration and consolidated candidate/evidence. |
| B3 layout, routing portal, geometry regressions | TASK Type 2 / B3 manager | GPT-5.6 Sol, high | Shell source and ordinary e2e tests; precisely lent viewport presentation/portal paths; no instrument, schema or core changes. Escalate a demonstrated hard diagnosis to Astra/high within this same scope. |
| B3 native contract inspection, bounded repairs and verification preparation | TASK Type 2 / B3 manager | GPT-5.6 Sol, high | src-tauri config/menu/title paths and native test plan; frontend bridge edits go to the integration owner. Existing host deferrals are assessed explicitly. No persistence schema change. |
| Complete frozen-diff review and finding backchecks | fresh TASK Type 2 / Agent 0 | GPT-6 Astra, xhigh | Read-only, independent of authors/manager; all inherited B3 bytes and new changes, requirements, protected tests, actual candidate evidence. |
| B3 structure-only fidelity review | separate fresh TASK Type 2 / Agent 0 | GPT-6 Astra, high | Screenshots and actual interaction against named frames and ruled departures; no appearance tuning. |
| Later read-only inventories or literal check execution | TASK Type 2 / owning manager | GPT-5.6 Terra, medium | Only explicit repeatable inventories/checks, not protected-test disposition or result-integrity judgment. |

No HELPS_HUMANS manager is needed for the current bounded package; Agent 0
handles these three owner decisions. The parked design work resumes only at the
closing visual pass, when its design responsibility warrants that role.

Concurrency: after agreement, shell and native work can proceed on disjoint
writes. The manager integrates shared frontend bridge changes. No canvas product
writer runs during B3; this prevents overlapping changes to PipeViewport and
viewport presentation. Native/browser sessions, dev ports and full Playwright
runs are serialized by one resource owner and the existing lock. Reviewers may
inspect a frozen candidate concurrently, with one scheduled UI witness. Agent 0
performs coordination and review integration, not a third implementation stream.

After B3 merges, retain the shell manager for B3A, B3B, B4, B5, B6, G-11 and B7
in the recorded order. Activate a second WORKING_ITEMS manager (Astra/high) for
I1 completion then C3–C5, using Sol/high implementers, with the existing disjoint
lane scopes. Shared styles, diagnostics and routing interfaces remain shell-owned
and go through Agent 0. G-17 is authorised but scheduled explicitly with its
compatibility evidence. The second profile/casing stays an owner package before
timed qualification. Closing visual pass, D-72 qualification, then A2 remain the
sequence; A2 receives its own plan. Material strategy changes return to the owner.

Actual mechanism: Codex delegated-harness-native descendants, not the historical
Claude launch/relay tools. A manager launches its own children and receives their
returns. Type 2 does not delegate. Fresh reviewers receive bounded context, not
the implementer's entire conversation. Retain sealed brief bytes/hashes, actual
model/effort, parentage and verbatim returns. Isolation is by worktree and written
scope; this host's unrestricted filesystem does not enforce per-agent boundaries.
Same-model review is context/author independence, not model diversity. The Sol
implementation/Astra review pair uses different named models without claiming
measured blind-spot independence.

## Verification and native scope

B3's addendum 4 explicitly deferred native menu changes and the window minimum
by the previous ROOT's decision, not by an owner ruling. Current source still
has `minWidth: 1024`, `minHeight: 768` and the static title SWBPIPE. New shell
stage/view commands are deliberately absent from NATIVE_MENU_COMMAND_IDS. The
accepted UX spec requires 1280 × 800. The current owner requires native evidence.

Proposed B3 native executor assesses and repairs these bounded shell seams where
needed against the adopted requirements: native menu commands/state and keyboard
routes, minimum size, title bar/project identity. Native busy-project gating and
the unsaved-title marker remain B3B and B3A respectively unless a demonstrated
B3 dependency requires a separately named integration decision. Verification alone
does not silently pull those later features into B3.

Native witness: launch the actual Tauri candidate, record build/environment and
inner/outer window sizes; exercise menu actions and accelerators through real UI;
resize to the native minimum; inspect the title bar and project identity through
open/new transitions. Browser CSS classes named native are not native evidence.
No fresh native witness was attempted during this discovery.

Connected scenarios begin in B3: open/create a disposable project; switch stages
and views; select and inspect; arm/cancel routing; enter/review/apply an operation;
undo/redo; solve/cancel/fail as operable; inspect Current/Historical designation;
save/reopen. Include keyboard and pointer paths, invalid input/recovery, window
sizes, active measurement and drawer/split transitions. Verify resulting model,
history and standing, not just screenshots. Match typed-operation routes where
applicable. Maintain and extend reusable scenarios through later slices and
repeat connected journeys after the visual pass. Explicitly list unavailable
states. Practitioner-usability holds remain.

Per merge: picking regression first/last for the viewport edits; registered unit,
build and harness checks; both source/dist Playwright lanes; independent complete
frozen-diff review and backchecks; B3's structure-only fidelity review; Agent 0's
own look; DEC-025 sweep on a clean candidate; required CI on the actual final
candidate. Reassess affected checks after any change. Native unavailability is
an outstanding witness, not a browser or CI pass. Contrast deferrals list touched
controls; all other D-68 obligations remain per slice.

## Continuity and decision interface

The work graph records this live audit and awaiting-agreement state. Its existing
candidate-to-deliverable bindings stay provisional, with a trace to approved
DAG-010 and the relevant source records. No deliverable reconciliation, lifecycle
change, DAG rebuild, product acceptance or release is performed.

The owner can agree to the strategy and rule D1, D2 and D3 separately. Recommended
package: D1 overlay with complete clearance and the named placement witness;
D2 table-first lending; D3 Both-only replacement of the dominance comparison.
Strategy agreement alone starts unaffected authorised work; unresolved dependent
product changes and B3 merge remain pending. Preparation records are uncommitted
and not independently reviewed; no publication is claimed.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
