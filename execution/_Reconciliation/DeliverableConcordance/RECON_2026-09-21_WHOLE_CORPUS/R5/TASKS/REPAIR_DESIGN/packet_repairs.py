#!/usr/bin/env python3
"""Apply exact A3/A10 claim repairs selected by parent Agent0, after snapshot stage."""
from pathlib import Path
import hashlib,json,re,sys
sys.dont_write_bytecode=True
import repair
ROOT,PROJECT,OUT=repair.ROOT,repair.PROJECT,repair.OUT
sha=repair.sha
proposal=(OUT.parent/'PACKET_ANALYSIS.md').read_text().split('## A10 exact record-repair proposals (supplement)')[1]
code=re.findall(r'```text\n(.*?)\n```',proposal,re.S)
assert len(code)==10
spec=[]
def target(did,name):return next(PROJECT.glob('execution/PKG-*/1_Working/'+did+'*/'+name))
def add(did,name,keys,old,new,packet):spec.append((target(did,name),keys,old,new,packet))
# A3 exact local choices; normative/acceptance questions remain registered.
p=target('DEL-00-01','ArchitectureBasis.md');old=next(l for l in p.read_text().splitlines() if l.startswith('- Exact dependency versions,'))
new='- Implementation-level choices are governed by DEC-012 and may be resolved in a sealed brief or later human ruling. Current dependency versions, local GUI component/state choices and repository tooling are recorded in their implementation and accepted change evidence. Changes to the DEC-009 runtime/UI baseline, public interfaces, dimensional semantics, privacy boundaries or registered acceptance criteria retain their own decision paths. The current state-hook realization is evidenced by the adopted UI program and `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B-SHELL/briefs/B2-STATE.md`; lockfile-bound installation is recorded in the adopted `execution/_Coordination/CANDIDATE_BRIEF_2026-07-15_DEL-10-04_CI_BROWSER_MAPPING.md` T1. These references settle only their scoped implementation choices; unresolved rule-expression, public-transport, CI and coverage-policy portions retain their respective decision records.'
add('DEL-00-01','ArchitectureBasis.md',['DEL-00-01:AB#open-holds-and-routed-questions.s01'],old,new,'A3')
p=target('DEL-00-02','ArchitectureBasis.md');old=next(l for l in p.read_text().splitlines() if l.startswith('- **Still-open TBDs:**'))
new='- Repository/package-manager and lint-tool choices are implementation-level choices under DEC-012; a sealed brief or later human ruling may resolve them. Lockfile-bound `npm ci` use is recorded in the adopted `execution/_Coordination/CANDIDATE_BRIEF_2026-07-15_DEL-10-04_CI_BROWSER_MAPPING.md` T1; this is bounded implementation evidence, not an exclusive package-manager mandate. Language-specific module syntax follows the DEC-009 stack; it requires no additional architecture ruling unless it changes a governed boundary. Lint-policy portions not resolved by an accepted brief remain open; the separate PDU-007 formal-review hold above is unchanged.'
add('DEL-00-02','ArchitectureBasis.md',['DEL-00-02:AB#open-holds-and-routed-questions.s03'],old,new,'A3')
p=target('DEL-00-05','ArchitectureBasis.md');old=next(l for l in p.read_text().splitlines() if l.startswith('- OPEN: GUI component/state-management-library'))
new='- The GUI follows the DEC-009 TypeScript/React/Vite and Three.js baseline. Component and state-management mechanisms are implementation choices under DEC-012, resolved through accepted bounded briefs/change evidence. The adopted `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/B-SHELL/briefs/B2-STATE.md` records the current React-hook extraction with no new dependency and names byte-preservation, build, unit and browser checks. This is evidence of the current local mechanism, not a permanent library mandate or a claim that all GUI behavior is accepted; governed state, operation and interface contracts remain binding.'
add('DEL-00-05','ArchitectureBasis.md',['DEL-00-05:AB#open-holds-and-routed-questions.s01'],old,new,'A3')
# A10 mixed undo/accessibility bullet: retain actual operation/history obligations and named witness.
undo='- Undo/redo must preserve the governed operation route, model history, busy-state and result-invalidation contracts. Current focus-aware shortcut ownership and its native witness requirement are recorded in `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/ROOT/CONTINUATION_2026-09-19_CODEX/B3_UNDO_OWNERSHIP_DECISION.md`; this does not assert that every undo/persistence behavior passed. Accessibility criteria for touched controls are WCAG 2.2 AA under D-68 (`execution/_Coordination/_DECISIONS/D-68_RULING_2026-09-15.md`); these are candidate checks, not certification or an independent-usability result. Independent usability/security validation remains held under its owning criteria and workflow.'
add('DEL-00-05','ArchitectureBasis.md',['DEL-00-05:AB#open-holds-and-routed-questions.s02','DEL-00-05:AB#open-holds-and-routed-questions.s03'],code[0],undo,'A3;A10')
for i,key in [(2,'DEL-08-05:SOW#CLM-033/OQ-08-05-003'),(4,'DEL-08-05:SOW#CLM-033/OQ-08-05-005'),(6,'DEL-12-05:SOW#CLM-011.r02'),(8,'DEL-12-05:SOW#CLM-033.r02')]:add(key[:9],'ScopeOfWork.md',[key],code[i],code[i+1],'A10')
files=[]
for p in sorted({s[0] for s in spec}):
 before=p.read_text();after=before;edits=[]
 for _,keys,old,new,packet in [s for s in spec if s[0]==p]:
  assert before.count(old)==1,(p,old)
  start=before.index(old);end=start+len(old)
  edits.append({'start':before[:start].count('\n')+1,'end':before[:end].count('\n')+1,'old':old,'new':new,'keys':keys,'classes':packet.split(';'),'posture':'b','reason':'Apply named existing decisions and sealed-brief authority; preserve unrelated holds and acceptance limits.','old_start_offset':start,'old_end_offset':end,'old_sha256':sha(old),'new_sha256':sha(new)})
 for e in sorted(edits,key=lambda e:e['old_start_offset'],reverse=True):after=after[:e['old_start_offset']]+e['new']+after[e['old_end_offset']:]
 for e in edits:
  e['new_start_offset']=e['old_start_offset']+sum(len(q['new'])-len(q['old']) for q in edits if q['old_end_offset']<=e['old_start_offset']);e['new_end_offset']=e['new_start_offset']+len(e['new'])
 files.append({'path':str(p.relative_to(ROOT)),'before_sha256':sha(before),'after_sha256':sha(after),'edits':sorted(edits,key=lambda e:e['old_start_offset'])})
manifest=OUT/'PHYSICAL_EDITS_STAGE2.json'
if manifest.exists():raise SystemExit('Stage2 already materialized; use check_all.py for replay/inverse verification')
manifest.write_text(json.dumps({'source_basis':'after PHYSICAL_EDITS.json; preserved chain','files':files},indent=2)+'\n')
for f in files:
 p=ROOT/f['path'];text=p.read_text();assert sha(text)==f['before_sha256']
 for e in reversed(f['edits']):text=text[:e['old_start_offset']]+e['new']+text[e['old_end_offset']:]
 assert sha(text)==f['after_sha256'];p.write_text(text)
print(json.dumps({'stage2_files':len(files),'patches':len(spec),'source_keys':sum(len(s[1]) for s in spec)}))
