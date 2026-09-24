# Independent compact-drawer complete-candidate review

Reviewer `/root/source_review`, TASK Type 2, fresh delegated-harness-native child of `/root`, independent of implementation. Parent supplied allocation `gpt-6-astra/xhigh`; this is allocation provenance, not a separate backend attestation or model diversity. No child was delegated. Root/project instructions, canonical TASK, current LOOP_INIT and project `software-code-review` skill were read. No other role was activated.

Frozen candidate: `1c8f04039b5f41812ebd879fc80d810392123072`; complete comparison base: `06aff05a412b9e25ad022e5fc629c77ee51efbc6`. The six maintained source/test hashes independently match `TASK_COMPACT/CANDIDATE.json`, including recomputed aggregate SHA-256 `5b83325516e87361da145a0d27807f8cd9a24b0d27b20e8c7e525ec7689836f1` (compact JSON encoding of its ordered files array). Candidate source was read from Git objects, not mutable working files. The parent expressly clarified that read-only Git inspection is permitted; no Git mutation, network, tests, builds, browser/native/CUA, application process, or product write occurred. Only this return was written. Local viewing of the two saved evidence PNGs followed byte equality checks against the frozen blobs.

## Result

**HOLD for two bounded source repairs and independent backcheck before treating the candidate as ready for the planned native/sweep stage.** The findings concern new control accessibility and Escape ownership. No engine, controller, mutation-route, budget or VirtualList change was found in the complete diff. After repair, rerun affected focused scenarios and bind subsequent native/broad checks to the repaired candidate. This is not merge acceptance, performance qualification, practitioner usability acceptance, or product/release acceptance.

## Actionable findings

### SR-1 — P2 — Details loses Escape ownership after keyboard focus leaves it

- Location: `apps/desktop/src/features/model-tree/ModelTree.tsx:865–868`, especially the popover-local `onKeyDown` at line 868. Relevant unchanged caller: `apps/desktop/src/features/workspace/workspaceSession.ts:2399–2411`, attached by `App.tsx:381`.
- Trigger: in narrow Both below 1280px, open Details, then Tab or Shift+Tab to a control outside the popover and press Escape while Details remains open. Details is a nonmodal `popover="auto"`; it has no tabbable child or focus-exit dismissal. Keyboard traversal can therefore leave its focused div without dismissing it.
- Impact: Escape no longer reaches the only new popover handler. It bubbles to the narrow-drawer handler, which prevents the default action and closes the table drawer. The user is taken out of the editing surface instead of dismissing Details. This is the same ownership class as the initial recorded failure, with the surviving outside-focus path.
- Evidence: static event/DOM ownership trace above; the retained initial browser log corroborates that drawer Escape handling can supersede the popover. The new test at `e2e/b4-table-editing.spec.ts:696–697` checks Escape only immediately after asserting the popover itself is focused. This reviewer did not execute the new trigger.
- Repair direction: explicitly end Details when focus leaves it, or own Escape at an appropriate scoped ancestor while that popover is open, before the drawer handler. Preserve normal drawer Escape once Details is closed, avoid swallowing editor/enum Escape, and restore focus appropriately. Add connected keyboard coverage for open → forward/backward Tab → Escape, plus immediate Escape and subsequent ordinary drawer Escape.

### SR-2 — P2 — Family's accessible name does not contain its visible label

- Location: `apps/desktop/src/features/model-tree/ModelTree.tsx:857–858`.
- Trigger: use the new compact family selector through a speech or accessibility interface by its visible label, “Family”.
- Impact: `aria-label="Grid entity type"` overrides the enclosing visible label. The accessibility name contains no “Family”, so visible-label speech targeting is inconsistent with the interface and the touched control fails the label-in-name requirement of the adopted WCAG 2.2 AA basis. Deferred contrast/fidelity work does not defer this new control semantics issue.
- Evidence: the frozen JSX establishes the override, and both saved final screenshots visibly display “Family”. The new source and dist locators explicitly target the mismatching name, so their passes do not verify visible-label agreement.
- Repair direction: derive the accessible name from the visible label, or use a name that contains “Family” (for example “Grid family”). Update affected role locators and add a focused accessible-name assertion. Preserve the native select's keyboard and pointer operation.

## Evidence and authority assessment

All 59 changed paths are covered below: six maintained files by complete diff and relevant caller/style inspection; manager/root documents by content and their relationships; inventories/manifests/recovery and raw geometry/report data by parsed structure/identity comparisons; logs by recorded outcomes; both PNGs by local inspection. This is proportionate complete-candidate review, not a new audit of every historical source cited by those documents.

- The presentation props now bound the grid in every shell mode. The compact toolbar is portalled separately from the retained direct/review table/editor ancestors. Family selection, draft storage, Queue/Clear intent routes and direct Apply's owned outcome carrier retain their existing semantics. Compact feedback preserves its alert/status roles and input error linkage. Noncompact/inactive/legacy-family paths were traced; no additional confirmed control leak was found.
- The preserved focused logs support 65 component tests, a successful TypeScript exit, 32 B4/Sections browser cases, and the final 12-case reporter pass. The reporter has 12 expected, zero skipped/unexpected/flaky and no errors. Each decoded geometry attachment exactly matches its entry in `GEOMETRY_FINAL.json`. All 60 recorded measurements have host scroll height equal to client height; minimum/default host heights are 127/227, with 36/136 body slots. Expanded observations retain the reported density/view caps. These are recorded Chromium observations, not a reviewer rerun or native proof.
- The exploratory baseline/candidate scripts and their 36-state outputs are explicitly historical/exploratory; the baseline's 1280px Both is not narrow Both. Final maintained cases correctly use 1024px for narrow Both. Initial absent-WASM and interrupted/failed browser attempts remain recorded, with later successful checks distinguished.
- The changed dist selector is not claimed as executed. Complete desktop/build, source/dist coverage, clean DEC-025, required hosted checks and native interaction remain explicit outstanding gates. Existing Both+Inspector-open Provenance pointer exposure remains open in B4-FAMILIES; this slice does not close it.
- Geometry checks do not establish all long feedback is reachable by sighted keyboard users: they hit Apply/Cancel and inspect alert text, but do not exercise long errors/statuses through actual horizontal scrolling or keyboard traversal. Carry that as a targeted pending native/connected witness, not a confirmed additional defect. The enum popup and persistent editor use existing clipping/ownership logic; compact minimum, horizontal scrolling and responsive transitions still warrant the planned native check. Native narrow Both must be reported unavailable if the 1280px minimum prevents it, never inferred from Chromium.
- `GRAPH_REVIEW/RETURN.md` and `BACKCHECK.md` are retained prior independent review evidence. Their reviewed graph/recovery content was compared with the current candidate and the later delta was read. GR-1's two inherited follow-ups remain explicit; the later opening prose correctly separates UI and activated LIVE milestones and their common final closeout. Owner rulings supersede the earlier proposals only where stated, preserving the D-72 population conflict for its existing profile freeze. No instruction amendment, changed protected budget, completed B4, accepted future schema, live implementation, or release is claimed.
- SCA-011/DAG-011 current ownership clauses preserve generic table/support versus specialized family semantics, DEL-07-11 hosting/focus/history and DEL-07-08/DEL-16 interaction/execution boundaries. Current ownership is not dependency satisfaction. Live wire/brief/test records remain preparation for the separately reviewed activated scope; no live/C4 product code is in this candidate. This review does not substitute for the independent wire/source-fit review or writer acknowledgements.
- Source manifests were checked for recoverable identity. All 42 live sources and the execution/B4 instruction entries match candidate bytes; design App identity matches its explicitly earlier product basis, and noted graph/coordination hashes remain historical concurrent observations. The later source manifest binds the compact changes. No current-candidate coverage is inferred from historical hashes. Peer exchanges and owner answers are reviewed as attributed supplied records, not independently replayed transport custody.

## Complete changed-file coverage

Path anchors below are repository-relative: `D = projects/chirality-piping/apps/desktop/`; `C = projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/`; `G = projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`. SHA-256 identifies each entire frozen blob.

| Path | SHA-256 |
|---|---|
| `D/e2e/b4-table-editing.spec.ts` | `569e15560a2a6b1bfe36fde70155439b0e410c90d426e8ff492a8750f0b9efa6` |
| `D/e2e/ui-foundation-dist.spec.ts` | `d5b67fae02e77cc5f97d5475fb93aa6358f314030546cab819529af662b8cfea` |
| `D/src/App.tsx` | `5f900ec96cc7e1d58f13f53a653bf845b448594c0a3d9bcb7b20e10df387e86e` |
| `D/src/features/model-tree/ModelTree.tsx` | `9168a12772f3d21911eaafadefaf83818ce7d8aa0c8c461ddd42bcb1fe0fd6c0` |
| `D/src/features/workspace/table/EngineeringTable.tsx` | `2c4bedcacb145287c132069ab50bb05d74ad74518ea45a35571b5d825a590326` |
| `D/src/styles.css` | `4096d44dad9b84a0a7edff534290cbd4001a81cc477b8783a0a5a16d733df3b0` |
| `C/B4_MANAGER/INTEGRATION_NOTES.md` | `8cae854bce0c429b717a3f070928b20f20a07baf60d4ca05d8e99502f41ad4f8` |
| `C/B4_MANAGER/MANAGER_CHECK.json` | `e28f4ebd6c51ddbad66aa496b176ea69c6baeb21f4c32b58a529b9e64dd2653c` |
| `C/B4_MANAGER/NATIVE_PLAN_DRAFT.md` | `f113b13d44c3c409878f97e1f543e5b462c0b220453453f70fe69ab0d0f05e30` |
| `C/B4_MANAGER/PIPES_PREPARATION/PREPARATION_RETURN.md` | `94844ef7d83fed3ed6561cf29b71c882176ef4baad5c3551ebe16096b03e46a2` |
| `C/B4_MANAGER/PROVENANCE.json` | `4bcd643df998d02879ba4cb668610bed97dbf87ffeca8928d41b337d6bb960da` |
| `C/B4_MANAGER/RETURN.md` | `05c5c97ad8117f51bfef6db105618772dde7815fb2a293ba11f1888ae203c3be` |
| `C/B4_MANAGER/TASK_COMPACT/CANDIDATE.json` | `7ae0100b3749caf3d53b8ca23eb6051fc41695afd6c6683a40f444c3c801b32f` |
| `C/B4_MANAGER/TASK_COMPACT/CHECK_EXITS.json` | `5aeed644a0a32f0c4b1e4e6b79a966e88ac14eafd6030224a1355712f32b9ac5` |
| `C/B4_MANAGER/TASK_COMPACT/COMMANDS.md` | `c6c9bc39a809da1b915f5e8fc221a5cf19889143808480b5dfb4f2f2b144ed5e` |
| `C/B4_MANAGER/TASK_COMPACT/GEOMETRY_FINAL.json` | `5bf7ee33e115afbebf4301bf2c6263bd5767598ccdc658eb698178bddc8cd7cd` |
| `C/B4_MANAGER/TASK_COMPACT/INPUTS.json` | `f94a4f2cf244ae902a204fa90f156d0d72e629403250e1497231b2e0f6fed7f1` |
| `C/B4_MANAGER/TASK_COMPACT/PROVENANCE.json` | `d183e97b64a823587c77afcf8f33783ede85d9e16404c3c35feeffb0b07c3bc6` |
| `C/B4_MANAGER/TASK_COMPACT/RETURN.md` | `a4b46127a6f64da6c736afdb3e76c524a20972e4c9a3c79470465f95f90c8ecf` |
| `C/B4_MANAGER/TASK_COMPACT/baseline-geometry.json` | `1f89d189fac0a606be5b5ca25dc817e7b071a112839229da8258c47c4cd1ac8a` |
| `C/B4_MANAGER/TASK_COMPACT/baseline-measurement.mjs` | `3227e5449ada47583d2dfdcdd5f60bf7d5227ff635ea410d0b575e85a4b2a3aa` |
| `C/B4_MANAGER/TASK_COMPACT/browser-r2.log` | `5062659408cce3a735f9c4496d1d34c5377b62c8aed40807c16ca5070c0d5059` |
| `C/B4_MANAGER/TASK_COMPACT/browser-r3.log` | `0ded2e2e555494564370cd0a98d96744ab950bdffe90e9f2b2d834b5b1ce674c` |
| `C/B4_MANAGER/TASK_COMPACT/browser-r4-report.json` | `410789fc326e3b97a7842165d32fd88bd0bc39ea77df12e691dcb40e11746e02` |
| `C/B4_MANAGER/TASK_COMPACT/browser-r4.log` | `9abe0356165082a1abd28338e31a98be71cf6e0a4a84fce3ee5cc1fc5c37cb27` |
| `C/B4_MANAGER/TASK_COMPACT/browser-r4/b4-table-editing-B4-compac-38279-mpact-180-explicit-viewport-chromium-desktop/compact-drawer.png` | `95f47fa3d18877c4cc5eb5feb0b3a4598f49faa353fd1315c3b6cf181f98c734` |
| `C/B4_MANAGER/TASK_COMPACT/browser-r4/b4-table-editing-B4-compac-3abeb-table-180-explicit-viewport-chromium-desktop/compact-drawer.png` | `4e668e56d4d1bb6fa858d87569cdc15ebf4d77144c32841aad74fe2c7e460bc4` |
| `C/B4_MANAGER/TASK_COMPACT/browser.log` | `38c124086740ac28a68793a9058c18debba1d86cdd34c8c32e23aaceed46a642` |
| `C/B4_MANAGER/TASK_COMPACT/candidate-geometry.json` | `aa950ff3bff2aad6339b1631f55043025f60732bb2dddd0b0b864f97563b148b` |
| `C/B4_MANAGER/TASK_COMPACT/candidate-measurement.mjs` | `b637de36498811f6b1b5682bc1448641d1628b088f4f53a53204b411dcd5cca3` |
| `C/B4_MANAGER/TASK_COMPACT/typecheck-final.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `C/B4_MANAGER/TASK_COMPACT/unit-after-wasm.log` | `9ace96f2003c32c32264f810137e219567f9d264709ec96444487dc1bea9f911` |
| `C/B4_MANAGER/TASK_COMPACT/unit-final.log` | `55456b390dab888f788fcc68cbe0da1aadaeb4567d8272c659985ac38901e1e0` |
| `C/B4_MANAGER/TASK_COMPACT/unit.log` | `8e6113978947abdf6c20c52d228ec0e2a1e52bb3f91b81425eaec798fd6cc2bf` |
| `C/B4_MANAGER/TASK_COMPACT/wasm.log` | `44c6b8161b303b14d470734e1a5fa14722dc0842473ac23a49c3c8a3c38eb178` |
| `C/COORDINATION.md` | `7ce763b1c631d1fe02e5f447ae8f37734d07a91d010d50028226041a31ba9321` |
| `C/DESIGN_MANAGER/OWNER_RULING_APPLICATION.md` | `e3f76a85aa4370f697c669ad0fc89bb724028f54e551feba1ae87cb6d14acc4a` |
| `C/DESIGN_MANAGER/OWNER_RULING_SOURCES.json` | `83100dace36f6a61e02813052a41c127f169ed94b36c4d42f95b33aa32e3c557` |
| `C/DESIGN_MANAGER/RETURN.md` | `74be013b6405f8987155453a8e70829d59d463678c57dffdd271679551ba965a` |
| `C/DESIGN_MANAGER/SOURCES.json` | `9aa7e9d1b24b7c45b4df671cd1913cf6673f8fd2cc0c4a1896619bd416fc24ec` |
| `C/DESIGN_MANAGER/SOURCE_DELTA.txt` | `c2968e9e7e001f87e43634e221510cf8fd191ee01faaa8849bd206d6544e7015` |
| `C/DESIGN_MANAGER_BRIEF.md` | `4882d53f41ad2b2d2251f1ea891dbef19f2bc0e2b8aa74cc8005e396f34ae024` |
| `C/EXECUTION_BASIS.json` | `660f9e6248561ca8aa916819f5774f2019ff81878f1e816875f5ed2b8a9fce66` |
| `C/GRAPH_REVIEW/BACKCHECK.md` | `0f11f1276208539db6b3321c4ccfd50c07e3fbd342a111ffe6652bb8c5264aae` |
| `C/GRAPH_REVIEW/RETURN.md` | `8fde62cb53482937c267ded42757786d7f76cca63af839fa9586d18668f2baf6` |
| `C/GRAPH_REVIEW_BRIEF.md` | `730699e0a03942b5ec2e2afe381be94644a261ba136440d7cea95c7de0720159` |
| `C/IMPLEMENTATION_MANAGER_BRIEF.md` | `190383d57b9788904ba6c9dbb066daa8abe7b114852c398302949c254091e5e3` |
| `C/LIVE_MANAGER/IMPLEMENTATION_BRIEFS.md` | `5a3116ca2a6999204a3d6f8dcd65d9cd4a176fa9404940f68b631fbb38938915` |
| `C/LIVE_MANAGER/PREPARATION_RETURN.md` | `e9a07e804c67414c0d562a0e781ddbbd6eb1239a40f8c93d261592b79c910865` |
| `C/LIVE_MANAGER/SOURCES.json` | `a918d07a3a38e51008baff04f1c0edcd0d91976794f2391085913e874031c7e5` |
| `C/LIVE_MANAGER/TEST_PLAN.md` | `a68a1ca42c9a44fdd2aaa01b63813a4045fd69b64757d34892fe1278ab2e5bb7` |
| `C/LIVE_MANAGER/WIRE_PROPOSAL.md` | `91f4910026bcceffc98c669a4d2487539ad63cc878822061e6cc1b40c98c40ce` |
| `C/LIVE_MANAGER_BRIEF.md` | `fd3a24e13e6be0839907719d537aa4590cad5d1296ddeca51e0d34b1d39a0d3b` |
| `C/OWNER_DECISIONS.md` | `7f244d1cbdfc6060d77de371e141b082adc936bccbfd4eb36e2a3b4308434b2e` |
| `C/OWNER_STEER.md` | `bec19591b50d4132cb4487971353f2594041603c157dee5c8221fd982e7bbe10` |
| `C/RECOVERY.json` | `4471132b830139cd21749335c8fc689a01f10279943fb2e80ef7b37e86f0b66f` |
| `C/SOURCE_REVIEW_BRIEF.md` | `4dba69cd6c340cb30af7da890767022a040b5de45fc37e0c8171383b55575084` |
| `C/WIRE_REVIEW_BRIEF.md` | `3ff8a1e7243c8903340df92162245ab6c0749b513e077680e4098925804990d5` |
| `G` | `bd510c737fc25b6d6abd10d957f0fd234bdbcaa03e2a2adc74438286bfb671ac` |

## Consulted instruction and additional source origins

The initial Root/project/TASK/loop/skill/brief reads used on-disk canonical files; their recorded hashes below match the frozen candidate bytes. Other listed source files were read as frozen Git objects at the candidate (bounded relevant excerpts for long carriers). Historical owner control-layer/contrast records were located through their retained ROOT source records; they were not activated as new instructions. No `chirality-change` or other role body was selected by this reviewer.

| Origin at candidate unless noted | SHA-256 |
|---|---|
| `AGENTS.md` | `c8ce87ef342902cb081bc659b26fc9a4edda1b6dba513814e5cb1e14e0b1dffd` |
| `agents/AGENT_TASK.md` | `1a13a5b00b3ce01ff8519efe6b46bcbe0cd6a5b7985e24282fa7efa2c57c8fb7` |
| `.agents/skills/software-code-review/SKILL.md` | `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` |
| `projects/chirality-piping/AGENTS.md` | `d9481951912ceffdd5bc47dbb6549bf0044f5d92f9fe6a9b11ba5969bfebc792` |
| `projects/chirality-piping/loop/LOOP_INIT.md` | `c712b6487faa3fab461e181aad1a6c16a2cb0eb734c510744e5037289105df1b` |
| `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOURCE_REVIEW_BRIEF.md` | `4dba69cd6c340cb30af7da890767022a040b5de45fc37e0c8171383b55575084` |
| `projects/chirality-piping/apps/desktop/src/features/workspace/workspaceSession.ts` | `fe0f9eea5a991d965dc097553e73e8c4c23c7968ed1743a8709af94f02dab20d` |
| `projects/chirality-piping/apps/desktop/src/features/workspace/table/useEnumEditor.tsx` | `9e409953160722b30dd7c5388723b5406255e722714627f7a5f1a4119e88af4d` |
| `projects/chirality-piping/execution/_DAG/_LATEST.md` | `6335bc5cb1b0d1c13c41181b0fbd3e3ab4547fa355b6c9ddc49513307b721422` |
| `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-02_Model tree and property inspector/ScopeOfWork.md` | `88d542aed787a0126dd6ba9427a9fe8fa35f0ea38d0496f219b39f5a3a8403e9` |
| `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-03_Material, component, and rule-pack editors/ScopeOfWork.md` | `d46cb51e59d53758f373ba910c3615de106264b586375763ae936aa8c7094525` |
| `projects/chirality-piping/execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-11_Desktop workspace shell and host integration/ScopeOfWork.md` | `ba2c60ffde8af56a1dc9b64f7e7170780fa9888407c241ffc49966a25190eb27` |
| `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md` | `0711b0e35f8d02f404dcae9a2b440bcf4506d5f0f9b76dc748195952bf7ab9a7` |
| `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX/B4_NEXT_PREPARATION_BRIEF.md` | `0bce9d60a09fb54ba2cac4e80a64b71ee4bcabd9ed458097be2d9c7626c04699` |
| `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX/_run_records/B4_NEXT_PREPARATION_RETURN.md` | `c6d8ece0b35b36db2d9fd71a7e35a084bef15df0e3663544883d926f401e26f8` |
| `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/OWNER_DIRECTION_2026-09-19_CONTROL_LAYER_FIRST.md` | `bc41f82680cfcb69a193ee3820cb0861f36fbe872fae970df1be1729f9a23268` |
| `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/OWNER_DIRECTION_2026-09-19_SECTION7_DECISIONS.md` | `c9049e543dd1a46b1fdcf3d7f23cd5551c89b24185c66c70d2d03eb77c2fee2e` |
