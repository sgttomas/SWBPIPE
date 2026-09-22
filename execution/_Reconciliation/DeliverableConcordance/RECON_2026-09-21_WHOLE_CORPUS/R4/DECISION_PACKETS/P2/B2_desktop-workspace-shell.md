# B2 — Owner of the desktop workspace shell, session state and operations UI

Packet for the R4 gate of run HELP-HUMAN-PIPING-20260921-RECONCILIATION.
Writer: TASK P2. This is a proposal only. The path conventions are as in B1.

## 1. Decision

Who owns the desktop workspace shell? It covers layout, stages, menus, the
native window and menu bridge, toolbar, status bar, drawers, section hosting,
host wiring and project-management UI. The packet also decides where central
session state (CAP-WSUI-011) and the operations UI (apply panel, batch and
draft queue) land.

**Holder: OWNER** on the shape. WORKING_ITEMS (workflow: scope-change)
executes it.

## 2. Background

**Earlier decisions.**
- **DEC-009** (`SD:600`). Adopts the Tauri 2 desktop shell,
  TypeScript/React/Vite GUI and Three.js viewport as the runtime and UI
  *baseline*. It does not make the shell a deliverable. Exact component and
  state libraries remain TBD.
- **SOW-020** (`SD:128`). The note says the GUI baseline is the Tauri shell.
- **PKG-07 scope** (`SD:223`). Lists the modeler, editors, warning UX,
  solve-execution UX and results views, carried by SOW-020/021/022/023/036/055,
  SOW-076 and SOW-077. It does not name shell chrome.
- **DEC-094** (`SD:685`). Makes DEL-07-09 the palette-surface owner, and says
  every palette command routes through PKG-16. DEL-07-09 is a contract and
  coverage slice only (`_CONTEXT.md:37`).

**What the code does now.**
- The shell exists and ships. T2 records 26–44 reverse answers per shell
  capability, and none claims ownership (T2-G1).
- Known defects have no owner. Examples: the dead `listenToNativeMenu`
  listener with a mismatched event name (CAP-SHELL-004 surface note), and a
  null CSP at `apps/desktop/src-tauri/tauri.conf.json:23` (T3-G4; line
  checked).
- DEL-07-08 answered COVERS against its own history for session work. In its
  notes ("Reverse pass") it asks whether this was filed under the wrong
  deliverable (T2-G2).

## 3. Options

**Sub-question (i): the shell (T2-G1, T3-G4)**

| Option | Consequences |
|---|---|
| (a) CREATE a PKG-07 workspace-shell UX_UI_SLICE deliverable | Scope change: SOW, DAG node, keys. 19 T2-G1 and 12 T3-G4 capabilities get one owner. The PKG-07 package scope line gains shell chrome. |
| (b) Extend DEL-07-01 or DEL-07-02 as the SOW-020 carriers | SOW amendment. The chosen deliverable's envelope grows substantially and needs an envelope re-check. |
| (c) Extend DEL-07-08, under which session extraction was already filed | Aligns with the existing history. DEL-07-08's SOW-076 scope (analysis workflow) must be widened. |
| (d) Treat host wiring as non-deliverable infrastructure (T3-G4 third option) | No scope change for the native-host part. Packaging identity, CSP, job service and layout persistence stay without an accountable owner (T3-G4 risk). Could be combined with (a)–(c) for the chrome. |

**Sub-question (ii): CAP-WSUI-011 session state (T2-G2)**
- (a) Goes with the shell owner from (i).
- (b) DEL-07-08 scope growth.

T2 proposes that the other three T2-G2 capabilities were filed under the wrong
deliverable: CAP-WSUI-013 → DEL-02-05, CAP-WSUI-040 → DEL-14-02, CAP-WSUI-012
→ the shell owner. Those are H1 items and not decided here.

**Sub-question (iii): operations UI, CAP-WSUI-027/028 (T2-G4)**
- (a) DEL-07-08. Its SOW CLM-004 lists "operation diff review", and
  CAP-WSUI-029 (operation ledger) is OWNED_SHARED DEL-16-03/DEL-07-08
  (T3-G10).
- (b) DEL-07-09. DEL-00-05 names it, but its envelope excludes implementation
  (`_CONTEXT.md:37`), so this would need an envelope amendment. Compare B4
  option (a).
- (c) The new shell deliverable, if (i-a).

## 4. Evidence and reliability

| Source | Class | Status |
|---|---|---|
| `SD:600`, `SD:128`, `SD:223`, `SD:685` | GOVERNING | Read at the freeze |
| `tauri.conf.json:23` | EVIDENCE (freeze) | Checked by this writer |
| 1,956 reverse answers for the 57 T2 capabilities (sealed `DEL-*_reverse.csv` via `ROUTING_SAMPLE/SAMPLE_MANIFEST.csv`) | R2 sealed ledgers | Aggregated by T2; not re-read here |
| `DEL-07-08_notes.md` "Reverse pass" | R2 worker note | CONTEXT for why the work was filed there |
| T2-G1, G2, G4; T3-G4 | R3 PROPOSAL | Classifications only |

Verified: governing text and the CSP line. From worker notes only: the
`listenToNativeMenu` mismatch (`IMPLEMENTATION_SURFACES.csv` note, R1).

## 5. Affected claims

No class in `CLASS_INDEX.csv` is in B2. The portion is capability-level:
- **T2-G1 (19, `T2_UNMAPPED.csv`, PRODUCT_UNOWNED):** CAP-SHELL-001, 002,
  004, 005, 017, 018, 033, 051; CAP-WSUI-001, 002, 003, 004, 005, 007, 009,
  010, 012, 020, 043.
- **T2-G2 decision item (1):** CAP-WSUI-011 (PRODUCT_UNOWNED;
  OWNER_DECISION).
- **T2-G4 (2):** CAP-WSUI-027, CAP-WSUI-028 (PRODUCT_UNOWNED).
- **T3-G4 (12, `T3_OWNERSHIP.csv`, PARTIAL_UNOWNED_REMAINDER):**
  CAP-COREB-035; CAP-SHELL-003, 006, 010, 052, 053, 058, 061; CAP-VIEW-034;
  CAP-WSUI-008, 019, 044.
- **Total: 34 capabilities.** Deliverables touched: DEL-07-01, 07-02, 07-06,
  07-07, 07-08, 07-09, DEL-00-05, DEL-10-04 (bundler), DEL-16-04 (WSUI-044),
  DEL-02-05.
- **Claim rows.** No divergent claim row carries the shell gap. The relation
  rows that keep it visible are ALIGNED: DEL-07-06 COVERS SHELL-018/WSUI-020;
  DEL-07-07 COVERS WSUI-002/004 ("shell-owned"); DEL-07-09 COVERS WSUI-043.
- **OtherCorrections.** None apply.

## 6. Risks

- **Undecided.** The largest body of user-facing desktop code sits outside
  every acceptance and verification path. Known defects (native-menu listener,
  null CSP, layout persistence) have no owner to route them to. DEL-07-08's
  history records work its SOW cannot accept.
- **(i-a).** A new DAG node. A large first SOW needs a context-envelope
  decision.
- **(i-b)/(i-c).** Envelope overrun of an existing slice. Mixes concerns
  (viewport or workflow with chrome).
- **(i-d).** Security-relevant host settings (CSP) stay unowned.
- **(iii-b).** Contradicts DEL-07-09's accepted envelope unless amended. Same
  tension as B4.

## 7. Recommended routing

No recommendation; owner's call. One constraint follows from the evidence:
option (iii-b) needs a DEL-07-09 envelope amendment, whatever the choice. That
choice should be consistent with B4 option (a).

## 8. On-ruling mechanism

- **(i-a)–(i-c), (ii), (iii).** A scope-change handoff (WORKING_ITEMS,
  workflow: scope-change). It adds or amends the SOW, adds the DAG node or
  edges, and issues keys. The run is re-routed for these capabilities only
  (T2-G1 mechanism). R5 record repair of the reverse answers follows under a
  separate authorization.
- **(i-d).** An owner ruling recorded as a DEC (a non-deliverable-infrastructure
  classification like DEC-074 O3). R5 records it.
- **Defects** (native menu listener, CSP). CODE_FIX_CANDIDATE briefs (H2),
  BlockedOnPacket B2 until an owner exists.

## 9. Dependencies

- **Blocks.**
  - H1 items for the 34 capabilities, and the T2-G2 H1 reassignments
    (CAP-WSUI-012 needs the shell owner).
  - The CAP-SHELL-048 H1 item where the project container is needed
    (crosswalk: H1 with BlockedOnPacket B2).
- **Related.**
  - B4 (DEL-07-09 envelope).
  - B3 (the operation applier behind the operations UI).
  - B12 D10 (the DEC-028 compatibility window for project open and save).
- **Depends on.** None.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
packet makes no certification, code-compliance, professional-approval or
engineering-acceptance claim.
