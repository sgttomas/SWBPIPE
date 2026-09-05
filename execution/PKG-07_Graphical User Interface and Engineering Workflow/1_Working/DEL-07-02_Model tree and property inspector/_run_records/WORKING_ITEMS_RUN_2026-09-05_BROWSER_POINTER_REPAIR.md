# W7 browser path and pointer repair supplement

Selected deliverable: DEL-07-02. Status remains IN_PROGRESS; broader Remaining and all owner/engineering holds are unchanged.

The first committed sweep on e5ec1d4 exposed legacy browser navigation assumptions. Both source workflow specs now use visible current disclosures, Toolkit and Review tabs while retaining engineering payloads, validation, application and save/reopen/result boundaries. Replacing an old synthetic diagnostic click with a real click exposed compressed Issues drawer panels; the scoped non-shrinking child-panel rule restores actual pointer selection and outer scrolling.

Accepted source binding: W7 `snapshots/E2E_REPAIR_V2/MANIFEST.json`, SHA256 cf2cc3df5746b76652974df700a78ea0f876ca232ab9835ecf019ec0d5d18daf; exact three-file diff SHA256 1a85cb5e1336ad073009ad70645bbae0ad48f23672ca04f84799865bbba937ca. Fresh independent R2 final PASS. [Manager return](../../../../_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W7/E2E_REPAIR_RETURN.md) contains evidence paths and limits.

Full source browser28, production-dist3, desktop748/45, targeteddiagnostic2 and exact1024x768 browser witness PASS. Both Wasm and TypeScript/Vite builds PASS. Ordinary selection, linked MPa context, required-input scrolling and Close were verified at1024; no body overflow or page errors. Browser fixtures and browser memory do not establish native GUI/backend or owner usability acceptance.

Prior immutable snapshots and failed/interrupted evidence remain preserved. New source commit, refreshed native validation and complete clean committed DEC025 are root-owned gates. No lifecycle/ScopeOfWork/dependency amendment; Queue → Review → Apply, MAP031, broader editor/typed-widget work, D58 and DEL07-06 PDU045/046 remain as previously recorded. This repair is part of the already-landed bounded presentation slice, so no further Remaining narrowing is warranted.
