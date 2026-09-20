# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: ui-foundation/full-cohort-controller.spec.ts >> runtime geometry pin checks self-contained manifests and actual disk source
- Location: e2e/ui-foundation/full-cohort-controller.spec.ts:577:1

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: "c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e"
Received: "fdf3eaa49b9685b932bce404421086c45a82fa22e5c8abc3f8146fc09cb846d5"
```

# Test source

```ts
  487 |     components: [{ id: "c", label: "C", geometry: { bend_radius: { value: 1.25, unit: "radiusunit" }, bend_geometry_source_reference: "unique-bend-ref", center_of_gravity: { x: 1, y: 2, z: 3, unit: "cgunit" } }, modifiers: { axial_stiffness_user_value: { value: 12, unit: "stiffnessunit" }, source_reference: "unique-modifier-ref" }, mechanics_interface: { solver_consumption: "unique-solver" }, completeness: [{ diagnostic_code: "unique-finding", status: "pending" }], provenance: "unique-component-origin" }],
  488 |     load_cases: [{ id: "l", label: "L", status: "unique-status", provenance: "unique-load-origin" }],
  489 |     combinations: [{ id: "b", label: "B", basis: "unique-basis", provenance: "unique-combination-origin" }],
  490 |     diagnostics: [{ id: "d", code: "D", message: "unique-message" }] };
  491 |   const cases: [string, string][] = [
  492 |     ["unique-description", "project-p"], ["model", "project-p"], ["unique-material-origin", "material-m"],
  493 |     ["unique-section", "section-s"], ["unique-structured-origin", "section-s"], ["pipe section", "section-s"],
  494 |     ["unique-node-origin", "node-n"], ["unique-material-ref", "pipe-e"], ["pipe segment", "pipe-e"], ["unique-pipe-origin", "pipe-e"],
  495 |     ["UX", "support-h"], ["unique-hanger", "support-h"], ["unique-hanger-ref", "support-h"], ["unique-hanger-consumption", "support-h"], ["unique-support-origin", "support-h"],
  496 |     ["1.25 radiusunit", "component-c"], ["unique-bend-ref", "component-c"], ["1 2 3 cgunit", "component-c"], ["12 stiffnessunit", "component-c"], ["unique-modifier-ref", "component-c"], ["unique-solver", "component-c"], ["unique-finding pending", "component-c"], ["unique-component-origin", "component-c"],
  497 |     ["load case", "load-l"], ["unique-status", "load-l"], ["unique-load-origin", "load-l"], ["unique-basis", "combination-b"], ["unique-combination-origin", "combination-b"], ["unique-message", "diagnostic-d"]];
  498 |   for (const [query, expected] of cases) {
  499 |     const result = frozenTreeExpectation(model, `  ${query.toUpperCase()}  `);
  500 |     expect(result.rows.filter(r => r.testId.startsWith("tree-row-")).map(r => r.testId), query).toEqual([`tree-row-${expected}`]);
  501 |     expect(result.visibleCount).toBe(1);
  502 |     expect(frozenTreeExpectation(model, `${query} absent-suffix`).visibleCount).toBe(0);
  503 |   }
  504 | });
  505 | // Frozen before repair by ROOT: TREE_QUERY_EXPECTATIONS_V30 manifest
  506 | // d61b6dd86718aeafb19e1960b749f646fe9782f3081c96c7050c1d3696053349.
  507 | const frozenQueryMemberships: [number, number, string, number, string][] = [
  508 |   [1000, 1, "UI benchmark pipe 00001", 1, "9a30796bec4ef2d1e8f9c56849c0e316e820b456da3451036ef52d02e13cdfb0"],
  509 |   [1000, 2, "UI benchmark pipe 00100", 1, "bb1c15a91dc23736d0d1615fa7cb284b4a1ea5d0f282465065ddb009529401b2"],
  510 |   [1000, 3, "UI benchmark pipe 00500", 1, "1a503c99ebf41ce0433147ef7738029953ea1e26b01587297dd8f5597dd4eb07"],
  511 |   [1000, 4, "UI benchmark node 00001", 1, "234d27b9429cbb815c3b93fa8f6891c9617e1f5d62d580ee183c5591a8337f1c"],
  512 |   [1000, 5, "UI benchmark node 00100", 1, "de59820ef55b27c11b6b356c41f3a6b32e137d87dba6e8f15c1ba954724d1010"],
  513 |   [1000, 6, "UI benchmark node 00500", 1, "91076fa335b246685ba2c7e01c639b9b1a3a88cbe0b2f268df0b1cb241cc3651"],
  514 |   [1000, 7, "UI benchmark support", 50, "e9e674eed2ff3b21c6be4ff137b1c8792f470e2e22bb25c13526918f385c819a"],
  515 |   [1000, 8, "UI benchmark valve", 10, "91d680fe307aa57f46a5b280b0b5528e3b688bdcdb9cdbfb02cd9d7447cde8bc"],
  516 |   [1000, 9, "section:UIF-OD-060", 1, "7b8212f8ae702e419c883990a6fda051d15740ef36bf06a2407d979ecbc8f085"],
  517 |   [1000, 10, "section:UIF-OD-180", 1, "5e3880d1296952d6e1ff0e41fb0740c896ad67ab66984bc101e7d80c436bbd74"],
  518 |   [1000, 11, "pipe:UIF-00077", 1, "d9929469c43630ca9f2cf9f70b7906e64071e0734e64ec30c5bf76e93ddf4710"],
  519 |   [1000, 12, "node:UIF-00088", 3, "cc39f81f9ba6af5d6db1b3c68b2fe1f9cc63c33d9ef8aeafb418807f8ec1c3e1"],
  520 |   [1000, 13, "support:UIF-00120", 1, "339ea5b93ae77b4d1c7633639201eb34020a939fdaacdb126324dad7b0640c29"],
  521 |   [1000, 14, "component:UIF-00200", 1, "575bd1827dc899128a3745239f3b3bd4c3e0fa4e09fed9b8530317dc52eda85f"],
  522 |   [1000, 15, "generated UI nodal arrow", 1, "c7c9338f4d83c05e9750fc05baba9680c6d70848b61e03f3fd5b1e617968eee7"],
  523 |   [1000, 16, "invented OD 90", 1, "581aa790ca089cdc09961e974f87895743f802e2a06f7434ac364e0287b0470a"],
  524 |   [1000, 17, "invented elastic material", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  525 |   [1000, 18, "no-match-ui-foundation", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  526 |   [1000, 19, "UIF-00", 2058, "b9999a66cfd35261f5d38273c12142c1273abf0538574d54494cc8a7196c054d"],
  527 |   [1000, 20, "generated ui", 2, "e6806950b139f6852346c75aa91fe24f91a0e043730cbe3b2ca762ad87309542"],
  528 |   [10000, 1, "UI benchmark pipe 00001", 1, "9a30796bec4ef2d1e8f9c56849c0e316e820b456da3451036ef52d02e13cdfb0"],
  529 |   [10000, 2, "UI benchmark pipe 00100", 1, "bb1c15a91dc23736d0d1615fa7cb284b4a1ea5d0f282465065ddb009529401b2"],
  530 |   [10000, 3, "UI benchmark pipe 00500", 1, "1a503c99ebf41ce0433147ef7738029953ea1e26b01587297dd8f5597dd4eb07"],
  531 |   [10000, 4, "UI benchmark node 00001", 1, "234d27b9429cbb815c3b93fa8f6891c9617e1f5d62d580ee183c5591a8337f1c"],
  532 |   [10000, 5, "UI benchmark node 00100", 1, "de59820ef55b27c11b6b356c41f3a6b32e137d87dba6e8f15c1ba954724d1010"],
  533 |   [10000, 6, "UI benchmark node 00500", 1, "91076fa335b246685ba2c7e01c639b9b1a3a88cbe0b2f268df0b1cb241cc3651"],
  534 |   [10000, 7, "UI benchmark support", 500, "92a5dc10b1d293c55a571615706023df43848c069e5c778afe9c6871c1836c20"],
  535 |   [10000, 8, "UI benchmark valve", 100, "c2c4ec1db718366604a06b2cc5320d9ddfa6ca1e485ce70cc710c10cf7d2dd7a"],
  536 |   [10000, 9, "section:UIF-OD-060", 1, "7b8212f8ae702e419c883990a6fda051d15740ef36bf06a2407d979ecbc8f085"],
  537 |   [10000, 10, "section:UIF-OD-180", 1, "5e3880d1296952d6e1ff0e41fb0740c896ad67ab66984bc101e7d80c436bbd74"],
  538 |   [10000, 11, "pipe:UIF-00077", 1, "d9929469c43630ca9f2cf9f70b7906e64071e0734e64ec30c5bf76e93ddf4710"],
  539 |   [10000, 12, "node:UIF-00088", 3, "cc39f81f9ba6af5d6db1b3c68b2fe1f9cc63c33d9ef8aeafb418807f8ec1c3e1"],
  540 |   [10000, 13, "support:UIF-00120", 1, "339ea5b93ae77b4d1c7633639201eb34020a939fdaacdb126324dad7b0640c29"],
  541 |   [10000, 14, "component:UIF-00200", 1, "575bd1827dc899128a3745239f3b3bd4c3e0fa4e09fed9b8530317dc52eda85f"],
  542 |   [10000, 15, "generated UI nodal arrow", 1, "c7c9338f4d83c05e9750fc05baba9680c6d70848b61e03f3fd5b1e617968eee7"],
  543 |   [10000, 16, "invented OD 90", 1, "581aa790ca089cdc09961e974f87895743f802e2a06f7434ac364e0287b0470a"],
  544 |   [10000, 17, "invented elastic material", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  545 |   [10000, 18, "no-match-ui-foundation", 0, "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"],
  546 |   [10000, 19, "UIF-00", 2058, "b9999a66cfd35261f5d38273c12142c1273abf0538574d54494cc8a7196c054d"],
  547 |   [10000, 20, "generated ui", 2, "5899202cc9fa8b0b45d327b294b6981c2fd1a5e188f3d84053378fbcdb7ca9bf"]
  548 | ];
  549 | 
  550 | test("tree oracle matches all 40 independently frozen ordered query memberships", async () => {
  551 |   for (const size of [1000, 10000]) {
  552 |     const model = JSON.parse(await readFile(new URL(`./fixtures/ui-foundation-${size}.model.json`, import.meta.url), "utf8"));
  553 |     const samples = JSON.parse(await readFile(new URL(`./samples/ui-foundation-${size}.interactions.json`, import.meta.url), "utf8"));
  554 |     expect(samples.tree_filters).toHaveLength(20);
  555 |     for (const [n, sample, query, count, hash] of frozenQueryMemberships.filter(row => row[0] === size)) {
  556 |       expect(samples.tree_filters.find((q: any) => q.sample === sample).query).toBe(query);
  557 |       const result = frozenTreeExpectation(model, query);
  558 |       const ids = result.rows.filter(r => r.testId.startsWith("tree-row-")).map(r => r.testId);
  559 |       expect(result.visibleCount, `${n}/${sample}/${query}`).toBe(count);
  560 |       expect(createHash("sha256").update(JSON.stringify(ids)).digest("hex"), `${n}/${sample}/${query}`).toBe(hash);
  561 |     }
  562 |   }
  563 | });
  564 | 
  565 | test("runtime geometry pin rejects old and mutated plan sources after valid selfhash rebinding", () => {
  566 |   expect(CUE_GEOMETRY_SOURCE_SHA256).toBe("c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e");
  567 |   for (const hash of ["97b18c9671fc7f98e1cbb94bf6833f737c91e3ef4580d02c711d306c82486aee", "0".repeat(64)]) {
  568 |     const { probe, plan } = syntheticCue(); plan.source.geometrySourceSha256 = hash;
  569 |     const { sha256: ignored, ...body } = plan; plan.sha256 = createHash("sha256").update(JSON.stringify(body)).digest("hex");
  570 |     probe.candidate_runtime.visual_plan = plan;
  571 |     expect(() => validateWinnerCuePlan(plan, probe)).toThrow("independent winner cue plan binding");
  572 |   }
  573 | });
  574 | 
  575 | // Self-contained admission controls: maintained fixtures/source plus test-owned manifests.
  576 | // Split verification checkouts may explicitly set the existing candidate source root.
  577 | test("runtime geometry pin checks self-contained manifests and actual disk source", async ({}, testInfo) => {
  578 |   const methodRoot = new URL("./", import.meta.url);
  579 |   const sourceRoot = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT ?? new URL("../../../../", import.meta.url).pathname;
  580 |   const hash = (bytes: string | Buffer) => createHash("sha256").update(bytes).digest("hex");
  581 |   // Historical oracle preimage; never imported or executed as the current product.
  582 |   const geometryPath = process.env.UI_FOUNDATION_CANDIDATE_SOURCE_ROOT
  583 |     ? `${sourceRoot}/apps/desktop/src/features/viewport/viewportSelection.ts`
  584 |     : new URL("fixtures/frozen-oracle-geometry.ts.txt", methodRoot);
  585 |   const cuePath = `${sourceRoot}/apps/desktop/src/features/viewport/viewportSelectionPresentation.ts`;
  586 |   const geometry = await readFile(geometryPath);
> 587 |   expect(hash(geometry)).toBe(CUE_GEOMETRY_SOURCE_SHA256);
      |                          ^ Error: expect(received).toBe(expected) // Object.is equality
  588 |   expect(hash(await readFile(cuePath))).toBe(CUE_SOURCE_SHA256);
  589 |   const dependencies = await Promise.all(["freeze-candidate-point-oracle.mjs", "point-hit-oracle.mjs", "box-selection-oracle.mjs"]
  590 |     .map(async name => ({ path: name, sha256: hash(await readFile(new URL(name, methodRoot))) })));
  591 |   const priorDir = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR, priorHash = process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256;
  592 |   try {
  593 |     // Read only maintained baseline fixtures; external oracle configuration cannot select inputs.
  594 |     delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR;
  595 |     for (const size of [1000, 10000]) {
  596 |       const fixture = await loadFixture(size);
  597 |       const camera = { fixture: "synthetic-camera" }, canvas = { fixture: "synthetic-canvas" }, preflightSha256 = hash("synthetic-preflight");
  598 |       const { probe, plan } = syntheticCue();
  599 |       const body: any = { ...plan, camera, canvas, source: { ...plan.source, dependencies,
  600 |         modelSha256: hash(fixture.bytes), sampleSha256: hash(await readFile(fixture.samplesPath)), cameraPreflightSha256: preflightSha256 } };
  601 |       delete body.sha256;
  602 |       const checkedPlan = { ...body, sha256: hash(JSON.stringify(body)) };
  603 |       const checkedProbe = { ...probe, candidate_runtime: { ...probe.candidate_runtime, visual_plan: checkedPlan } };
  604 |       const oracle = { probes: [checkedProbe], candidate_preflight: { preflightSha256, camera, rawCanvasReadback: canvas } };
  605 |       const oracleBytes = JSON.stringify(oracle), oracleName = `ui-foundation-${size}.candidate-runtime-point-oracle-v3.json`;
  606 |       for (const mode of ["accepted-copy", "old-declaration", "mutated-disk", "mutated-declaration-and-disk"]) {
  607 |         const dir = testInfo.outputPath(`${size}-${mode}`); await mkdir(dir, { recursive: true });
  608 |         const sourcePath = `${dir}/viewportSelection.ts`;
  609 |         const bytes = mode.includes("mutated") ? Buffer.concat([geometry, Buffer.from("\n// mutation\n")]) : geometry;
  610 |         await writeFile(sourcePath, bytes, { flag: "wx" });
  611 |         const manifest = { status: "PASS_ALL_200_ACTIONABLE_AND_PRODUCT_PROJECTION_CROSSCHECKED",
  612 |           cueSourcePath: cuePath, cueSourceSha256: CUE_SOURCE_SHA256, geometrySourcePath: sourcePath,
  613 |           geometrySourceSha256: mode === "old-declaration" ? "97b18c9671fc7f98e1cbb94bf6833f737c91e3ef4580d02c711d306c82486aee"
  614 |             : mode === "mutated-declaration-and-disk" ? hash(bytes) : CUE_GEOMETRY_SOURCE_SHA256,
  615 |           dependencies, boxSelectionPolicySha256: "8195cd971146d337323dd791992884ce670b766f29bbdde6abd76b82da73b310",
  616 |           files: [{ pipeCount: size, path: oracleName, sha256: hash(oracleBytes), actionableCount: 200, boxSampleCount: 20 }] };
  617 |         const manifestBytes = JSON.stringify(manifest);
  618 |         await writeFile(`${dir}/${oracleName}`, oracleBytes, { flag: "wx" });
  619 |         await writeFile(`${dir}/CANDIDATE_POINT_ORACLE_MANIFEST.json`, manifestBytes, { flag: "wx" });
  620 |         process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR = dir;
  621 |         process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256 = hash(manifestBytes);
  622 |         const input = { ...fixture, pointOracle: oracle, pointOraclePath: `${dir}/${oracleName}`, pointOracleSha256: hash(oracleBytes) };
  623 |         if (mode === "accepted-copy") { await validateCandidateOracleBinding(input, size); validateWinnerCuePlan(checkedPlan, checkedProbe); }
  624 |         else await expect(validateCandidateOracleBinding(input, size)).rejects.toThrow("winner product geometry/cue source drift");
  625 |       }
  626 |       delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR;
  627 |     }
  628 |   } finally {
  629 |     if (priorDir === undefined) delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR; else process.env.UI_FOUNDATION_CANDIDATE_ORACLE_DIR = priorDir;
  630 |     if (priorHash === undefined) delete process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256; else process.env.UI_FOUNDATION_CANDIDATE_ORACLE_MANIFEST_SHA256 = priorHash;
  631 |   }
  632 | });
  633 | 
  634 | test("orbit constructor preserves full qualified sequence and unchanged interval gaps", () => {
  635 |   const actionAt = 1000, times = [2990,2999,3000.062,5000,9000,13000.062,13001,13010];
  636 |   const results = times.map(t => ({ presentationTraceTimestamp: t * 1000,
  637 |     pageToTraceOffsetIntervalMs: { minimum: 0, maximum: 0 },
  638 |     actionToPresentationIntervalMs: { lower: t - .125 - actionAt, upper: t + .125 - actionAt } }));
  639 |   const input = syntheticTraceInput(results, actionAt), basis = bindSameTraceDurationBasis(input);
  640 |   const boundResults = input.extraction.presentations, before = structuredClone(boundResults), evidence = constructOrbitEvidence(boundResults, actionAt, "centerline", basis);
  641 |   expect(evidence.envelope).toEqual({ first: 1, last: 6 });
  642 |   expect(evidence.endpoints.map(p=>p.sourceIndex)).toEqual([0,1,2,3,4,5,6,7]);
  643 |   expect(evidence.endpoints.map(p=>p.reportedTimestamp)).toEqual(times.map(t=>t*1000));
  644 |   expect(evidence.gaps).toHaveLength(5);
  645 |   evidence.gaps.forEach((g,i)=> {
  646 |     expect(g.fromSourceIndex).toBe(i+1);expect(g.toSourceIndex).toBe(i+2);
  647 |     expect(g.durationIntervalMs).toEqual(sameTracePresentedGap(evidence.endpoints[i+1].reportedTimestamp,evidence.endpoints[i+2].reportedTimestamp));
  648 |   });
  649 |   expect(evidence.warmup).toEqual({startMs:1000,endMs:3000});
  650 |   expect(evidence.measured).toEqual({startMs:3000,endMs:13000});expect(boundResults).toEqual(before);
  651 |   expect(()=>constructOrbitEvidence(boundResults.map((r:any)=>({...r,pageToTraceOffsetIntervalMs:{minimum:1,maximum:0}})),actionAt,"centerline",basis)).toThrow("no common source-bound clock mapping");
  652 | });
  653 | 
  654 | function syntheticTraceInput(rows: any[], actionAt=1000) {
  655 |   const token="synthetic-bound-orbit", frame="document-A", events:any[]=[{name:"TimeStamp",ts:1000000,pid:41,tid:7,args:{data:{message:`UIF_CAUSAL_V1:ACTION:${token}`,frame}}}];
  656 |   const feedbackMarkers:any[]=[];
  657 |   const presentations=rows.map((r,i)=>{
  658 |     const markerIdentity=`${token}.${i}`;events.push({name:"TimeStamp",pid:41,tid:7,ts:1000100+i,args:{data:{message:`UIF_CAUSAL_V1:FEEDBACK:${markerIdentity}`,frame}}});
  659 |     const reporterBeginEventIndex=events.length;events.push({name:"PipelineReporter",ph:"b",pid:41,tid:9,ts:r.presentationTraceTimestamp-1,cat:"cc,benchmark",scope:"renderer",id2:{local:"0x8"}});
  660 |     const reporterEndEventIndex=events.length;events.push({name:"PipelineReporter",ph:"e",pid:41,tid:9,ts:r.presentationTraceTimestamp,cat:"cc,benchmark",scope:"renderer",id2:{local:"0x8"}});
  661 |     feedbackMarkers.push({markerIdentity,canvasEpoch:1,contextEpoch:2});
  662 |     return {...r,status:"PASS_EXACT_CAUSAL_CHROMIUM_REPORTED_PRESENTATION",feedbackKind:"orbit",token,markerIdentity,actionTraceTimestamp:1000000,
  663 |       canvasEpoch:1,contextEpoch:2,modelGeneration:3,rendererProcessId:41,rendererMainThreadId:7,rendererCompositorThreadId:9,layerTreeId:23,
  664 |       pageClockSource:{...PAGE_CLOCK_SOURCE,crossOriginIsolated:false},pipelineReporterOccurrence:'[41,9,"id2.local","0x8","cc,benchmark","renderer","PipelineReporter"]',reporterBeginEventIndex,reporterEndEventIndex};
  665 |   });
  666 |   const rawBytes=Buffer.from(JSON.stringify({metadata:{"clock-domain":"MAC_MACH_ABSOLUTE_TIME"},traceEvents:events}));
  667 |   const rawSha256=createHash("sha256").update(rawBytes).digest("hex");
  668 |   return {rawBytes,capture:{events,rawTraceComplete:true,rawTraceSha256:rawSha256},trace:{traceDataLossOccurred:false,rawTraceTransport:{rawCompleteThroughEof:true,rawSha256}},
  669 |     extraction:{status:"PASS_ALL_CAUSAL_PRESENTATIONS_EXACT_AND_UNAMBIGUOUS",sourceBinding:REQUIRED_CHROMIUM_BINDING,presentations},
  670 |     stopped:{active:{token,feedbackKind:"orbit",documentTimeOrigin:10000,evidenceEpoch:0,armedCanvasEpoch:1,armedContextEpoch:2,
  671 |       actionMarker:{token,listenerObservedAt:actionAt,traceClock:{crossOriginIsolated:false}},feedbackMarkers}}};
  672 | }
  673 | // Independently frozen V65 rational/binary64 expectations, copied as literals (no run dependency).
  674 | test("same-trace integer arithmetic matches independent rational bounds and translation",()=>{
  675 |   const cases=[[0, 0, 0.0010000000000000002], [1, 0, 0.0020000000000000005], [16666, 16.664999999999996, 16.667000000000005], [16667, 16.665999999999997, 16.668000000000003], [33332, 33.330999999999996, 33.333000000000006], [33333, 33.331999999999994, 33.33400000000001], [9007199254740989, 9007199254740.986, 9007199254740.992]];
  676 |   for(const[delta,lower,upper]of cases){
  677 |     expect(sameTracePresentedGap(0,delta)).toEqual({lower,upper});
  678 |     expect(sameTracePresentedGap(Number.MAX_SAFE_INTEGER-delta,Number.MAX_SAFE_INTEGER)).toEqual({lower,upper});
  679 |   }
  680 |   for(const pair of [[2,1],[-1,0],[0,.5],[0,Number.MAX_SAFE_INTEGER+1],[NaN,1],[0,Infinity]])expect(()=>sameTracePresentedGap(...pair as [number,number])).toThrow();
  681 |   expect(conservativePresentedGap({lower:100,upper:100.1},{lower:116.5,upper:116.8})).toEqual({lower:4616189618054759/2**48,upper:4728779608739021/2**48});
  682 | });
  683 | test("same-trace boundary rejects mixed raw clock source document action epoch process and lineage",()=>{
  684 |   const make=()=>syntheticTraceInput([1000001,1016668].map(t=>({presentationTraceTimestamp:t}))), good=make();
  685 |   expect(bindSameTraceDurationBasis(good).references).toHaveLength(2);
  686 |   for(const mutate of [
  687 |     (v:any)=>{v.capture.rawTraceSha256="0".repeat(64);},(v:any)=>{v.trace.rawTraceTransport.rawSha256="0".repeat(64);},
```