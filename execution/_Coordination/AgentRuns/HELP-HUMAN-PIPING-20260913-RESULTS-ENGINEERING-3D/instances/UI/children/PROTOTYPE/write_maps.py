from pathlib import Path
import json,csv,hashlib
root=Path('/Users/ryan/.codex/worktrees/8728/chirality-3d-design-20260913')
p=root/'projects/chirality-piping'
u=p/'execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260913-RESULTS-ENGINEERING-3D/instances/UI'
out=u/'children/PROTOTYPE'
coverage=p/'execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working/DEL-07-09_Interactive operation vocabulary and tool palette contract/Vocabulary_Coverage.csv'
names=['Route pipe','Restraint / anchor','Materials / temperature table','Temperature / pressure','Force / moment / displacement','Wind / exposure spans','Seismic static-g','Combinations','Typed selection / model search','Undo / redo','Agent proposal / pending review','Spring hanger','Nonlinear support','Expansion joint','Components','Named support families','Sections / assignment','Guarded removal','Split pipe / insert node','Copy / rotate / mirror','Equipment / nozzle boundary','Generate self-weight','Imported hanger selection','Display units','Node renumbering','Snubbers','Cold spring / cut-short']
groups=['Build','Supports','Properties','Loads','Loads','Loads','Loads','Loads','Select and View','Edit','Review','Supports','Supports','Build','Build','Supports','Properties','Edit','Build','Edit','Supports','Loads','Supports','Select and View','Edit','Supports','Build']
journeys={1:'route',2:'support',9:'select',10:'history',11:'review',12:'support',13:'support',14:'component',15:'component',16:'support',19:'split',20:'transform',21:'support',24:'units'}
rows=[]
for i,r in enumerate(csv.DictReader(coverage.open())):
    key=r['VocabularyRow']; number=int(key) if key.isdigit() else key
    rows.append({'row':number,'label':names[i],'group':groups[i],'class':r['AnnexClass'],'mapping':r['CurrentOperationLinkage'],'sourceRoute':r['CurrentUIRoute'],'sourceResidual':r['CurrentResidual'],'sourceAcceptance':r['CurrentAcceptance'],'sourceLanding':r['ImplementationLandsIn'],'prototypeSurface':journeys.get(number,'mapped-inspector'),'prototypeStatus':'ROADMAP — no executable semantics' if not key.isdigit() else 'DESIGN SIMULATION — not production evidence'})
(out/'capability-map.json').write_text(json.dumps({'source':'Vocabulary_Coverage.csv at source HEAD 8f27fa3','claimFence':'F-PIP-2 / DEC-081; prototype simulation only','rows':rows},indent=2)+'\n')
(out/'prototype/capabilities.js').write_text('const CAPABILITIES = '+json.dumps(rows,indent=2)+';\n')
inputs=[root/'AGENTS.md',root/'agents/AGENT_TASK.md',p/'AGENTS.md',u/'PROTOTYPE_BRIEF_V1.md',u/'CONCEPT_V1.md',coverage,p/'execution/_ScopeChange/SCA-009_2026-08-20_0000/Vocabulary_Annex.md',p/'execution/_ScopeChange/SCA-009_2026-08-20_0000/ACCEPTANCE_RECORD.md']
for rel in ['services/operationService.ts','services/operationBatchService.ts','types.ts','features/viewport/PipeViewport.tsx','features/viewport/routeDraft.ts','features/toolkit/capabilityCatalog.ts']:
    inputs.append(p/'apps/desktop/src'/rel)
for folder in ['DEL-07-01_3D viewport and centerline editor','DEL-07-05_Results viewer','DEL-07-09_Interactive operation vocabulary and tool palette contract']:
    inputs.append(p/'execution/PKG-07_Graphical User Interface and Engineering Workflow/1_Working'/folder/'_CONTEXT.md')
records=[]
for n,f in enumerate(inputs):
    b=f.read_bytes(); raw=out/'_run_records/context'/f'{n:02d}-{f.name}'
    raw.parent.mkdir(parents=True,exist_ok=True); raw.write_bytes(b)
    records.append({'origin':str(f.relative_to(root)),'sha256':hashlib.sha256(b).hexdigest(),'bytes':len(b),'preservedRawCopy':str(raw.relative_to(out))})
(out/'_run_records/INPUT_MANIFEST.json').write_text(json.dumps({'role':'TASK Type 2','parent':'/root/next_ui_design','mechanism':'delegated-harness-native','sourceHEAD':'8f27fa3d8ec5e128e61fd3ac4076e74d7955f355','context':records},indent=2)+'\n')
