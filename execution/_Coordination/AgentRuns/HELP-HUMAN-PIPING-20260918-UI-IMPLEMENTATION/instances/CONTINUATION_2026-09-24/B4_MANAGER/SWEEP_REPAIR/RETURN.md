# Clean-sweep repair — manager integration return

WORKING_ITEMS /root/b4_manager returns a minimal three-file correction for ROOT freeze, fresh independent review and new clean DEC025. Retained TASK /root/b4_manager/compact_impl independently reproduced and separated both causes before repair; details/source hashes are in TASK/DIAGNOSIS_RETURN.md and its raw evidence.

| Failure | Diagnosis and minimal correction |
|---|---|
| Q1 queued-message assertion | Compact status already contained exact truthful queued text, but lacked the marker carried by its mutually exclusive noncompact branch. Added entity-grid-queued-message to that existing role=status span. Preserved App test text, both operation IDs and unit payload assertions unchanged. |
| P1 ten unhandled popover selector errors | Closed Info/Details cleanup on ordinary jsdom blur queried unsupported :popover-open although hidePopover was absent. Each helper now returns false if its target lacks a callable hidePopover before querying native state. Supported selector/hide/focus/event ordering stays unchanged; no catch, global selector override, setup shim, fallback UI or error suppression was added. |

The previous closed-selector test spy was removed. Focused tests now use actual absent-API jsdom and assert no state query, exercise supported closed/open branches with per-element doubles, and assert a deliberately injected supported-branch failure remains observable. The latter test captures only its exact expected sentinel after it escapes the product handler, cleaning its listener afterward; unrelated errors remain failures. These branch tests do not claim a Popover polyfill or native-support witness.

Postrepair unchanged queue/payload App test PASS; all5 reproducing App.projectHandlers cases PASS with no unhandled errors;68 focused components PASS; TypeScript PASS. Four existing real-browser Details/Info journeys PASS(34.3s), including Model/narrowBoth traversed Escape and opposite-density classic-scrollbar feedback/pan behavior. Prior reproduction failures remain raw and unmodified. The failed clean c57 sweep remains failed; this focused result is not a new full101-file or full-sweep pass.

Maintained change is only ModelTree.tsx, EngineeringTable.tsx and EngineeringTable.test.tsx. App tests/setup, controllers, native/Cargo and all other primary product files remain unchanged. ROOT's intervening upstream-only PEC document merge changes Git HEAD without changing the diagnosed Piping product bytes. No protected criterion, test expectation, native bundle or OS preference changed. No CUA/PID9925 action or Git mutation occurred.

TASK/RETURN.md and CANDIDATE.json supply exact final commands/check timing/source hashes and canonical evidence. Raw outputs are beneath TASK/_run_records; ROOT separately owns the immutable failed-sweep copy under continuation/_run_records/CANDIDATE_C57. The4-case browser run launched just before manager's reservation-hold message arrived under the existing grant; ROOT was notified immediately and confirmed the lane remained exclusive. No competing browser/native lane was used.

Source digest/resource release are recorded at finalization below. Independent repaired-candidate review and ROOT's clean sweep/native gates remain outstanding. No B4-FIT completion, future-family activation, product acceptance or release follows from these repairs.

## Final freeze and handback

Frozen3-file digest `244767aeb8a29cb924aa0d5b085987cadd7a2149296baf83c3f07ccf7f2d8b11` in TASK/CANDIDATE.json. Manager independently recomputed every file hash (match), inspected the complete minimal diff and ran scoped whitespace check (PASS); MANAGER_CHECK.json records this integration verification, not independent review. TASK additionally reran the3 new/updated branch tests on final bytes.

TASK stopped owned Vite and verified no listener on5174/5175; all focused runs completed. Source and focused JS/browser resources are released to ROOT. No outstanding child source writes/tests/build/UI work remains. ROOT may commit/freeze for independent backcheck and schedule the new clean sweep; prior c57 failure stays historical.
