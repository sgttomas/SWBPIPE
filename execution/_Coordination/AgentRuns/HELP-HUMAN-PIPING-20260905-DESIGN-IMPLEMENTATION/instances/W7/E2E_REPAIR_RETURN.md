# W7 browser repair supplement — accepted technical freeze

Status: PASS for the bounded implementation and preflight. Source and build ownership released to root; no W7 process remains. This supplements the immutable prior FINAL_SNAPSHOT, SOURCE_V2 and local-state supplement, without rewriting their historical bindings.

The clean committed sweep on e5ec1d4 exposed stale browser paths after the workspace redesign. U1 audited both entire source specs and retained selection, explicit units/payloads, validation/application, save/list/reopen, solve/results/report and browser-boundary assertions while using visible current disclosures, Toolkit destinations and Review tabs. Arbitrary inspector width became actual control actionability/containment checks. Two old synthetic clicks became ordinary pointer actions.

That stronger diagnostic click exposed a real Issues drawer defect: height-limited flex layout compressed direct child panels, allowing adjacent content to intercept clicks. U1 V8 added only scoped `.issues-drawer > .panel { flex-shrink: 0; }`. The outer drawer retains its bounds and scrolling. No product handler, model, engine, schema, native configuration or engineering default changed in this supplement.

## Frozen source and review

Accepted three-file candidate: `snapshots/E2E_REPAIR_V2/MANIFEST.json`, SHA256 `cf2cc3df5746b76652974df700a78ea0f876ca232ab9835ecf019ec0d5d18daf`. Exact diff against e5ec1d4 SHA256 `1a85cb5e1336ad073009ad70645bbae0ad48f23672ca04f84799865bbba937ca`. Two source specs plus styles are the complete source delta. All live hashes rechecked unchanged.

Fresh independent R2 reviewed 100% of the combined diff and final execution evidence: `children/R2/RETURN_FINAL_V3.json`, PASS, no actionable findings. Earlier static PASS and actionability HOLD evidence remain separately preserved. Author returns V7/V8 and sealed amendments record exact ownership; no child delegation. Actual model and token/context occupancy unavailable; native role/non-delegation instruction+config asserted.

## Verification

- Full source browser: 28/28 PASS, 4.7 minutes.
- Production-dist browser: 3/3 PASS, 9.5 seconds, including both real Wasm artifacts and atomic self-weight application/one batch undo; both Wasm builds and TypeScript/Vite production build PASS. Existing chunk-size warning remains.
- Desktop tests: 748 tests / 45 files PASS, 134.72 seconds.
- Targeted diagnostic ordinary-click regression: 2/2 PASS at desktop and compact sizes.
- Exact 1024x768 browser witness: ordinary diagnostic selection, linked MPa result context, required-input scrolling and Close PASS. Drawer bounds (246,62), 760x648; no horizontal overflow or page errors. Screenshot/script/result are under `_run_records/`.
- Diff whitespace and live source binding PASS. Browser servers 5174/5175/5178 absent at release; witness browser closed and temporary Vite stopped.

All new raw logs/diff bytes are encoded in JSON. The first W7 preflight was stopped after 15 passes, one real failure, one interruption and 11 unexecuted tests; bounded error/frame evidence is retained and the large ignored trace was not copied into Git. Iterative trace disabling was explicitly authorized and does not modify the final registered DEC command. Helper startup retry for an absent Chrome executable and telemetry output-path repair are explicitly recorded.

## Handoff and limits

Root owns refreshed native bundle/validation, new source commit, complete clean committed DEC-025 and publication decisions. This preflight is not the full sweep. Browser fixture mechanics/in-memory storage and browser screenshots are not native-backend or native-GUI acceptance. Owner usability and all prior engineering/lifecycle holds remain open.

No lifecycle promotion or new scope is claimed. MAP031 direct inline human Apply, full typed widgets, broader geometry/editor work, canonical analytical bridge, durable history, live-provider D58, and 07-06 PDU045/046 remain as previously recorded. Existing queue → review → apply semantics are preserved; batch undo remains covered by the unchanged dist spec.
