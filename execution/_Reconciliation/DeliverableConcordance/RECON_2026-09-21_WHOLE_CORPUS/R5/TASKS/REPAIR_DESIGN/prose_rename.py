#!/usr/bin/env python3
"""Bounded A4 active prose rename, preserving code spans and actual identifiers."""
from pathlib import Path
import importlib.util,json,re,sys
sys.dont_write_bytecode=True
import repair
ROOT,PROJECT,RUN,OUT,sha=repair.ROOT,repair.PROJECT,repair.RUN,repair.OUT,repair.sha
sp=importlib.util.spec_from_file_location('ex',RUN/'tools/extract_claims_v2.py');ex=importlib.util.module_from_spec(sp);sys.modules['ex']=ex;sp.loader.exec_module(ex)
files=[]
for p in sorted(PROJECT.glob('execution/PKG-*/1_Working/DEL-*/ScopeOfWork.md')):
 if p.parent.name.startswith('DEL-01-01_'):continue
 before=p.read_text();units=ex.markdown_units(p.parent.name[:9],'SOW',before,True);edits=[];offset=0
 for n,line in enumerate(before.splitlines(keepends=True),1):
  new=line
  if not re.match(r'^(?:>\s*)?#+',line) and not any(w in line.lower() for w in ['http://','https://','historical','ruling recorded','owner said','verbatim','commit ', 'generated:']):
   parts=re.split(r'(`[^`]*`)',line)
   for i in range(0,len(parts),2):parts[i]=re.sub(r'(?<![\w/:.\-])OpenPipeStress(?![\w.\-])','SWBPIPE',parts[i])
   new=''.join(parts)
  if new!=line:
   candidates=[u for u in units if u.start<=n<=u.end];u=min(candidates,key=lambda u:(u.end-u.start,0 if u.kind=='ITEM' else 1))
   edits.append({'start':n,'end':n,'old':line,'new':new,'keys':list(dict.fromkeys([u.key,p.parent.name[:9]+':SOW'])),'classes':['A4'],'posture':'b','reason':'Previously authorized current product-prose rename; all inline code, URLs, explicit history and actual identifiers preserved.','old_start_offset':offset,'old_end_offset':offset+len(line),'old_sha256':sha(line),'new_sha256':sha(new)})
  offset+=len(line)
 if not edits:continue
 after=before
 for e in reversed(edits):after=after[:e['old_start_offset']]+e['new']+after[e['old_end_offset']:]
 for e in edits:
  e['new_start_offset']=e['old_start_offset']+sum(len(q['new'])-len(q['old']) for q in edits if q['old_end_offset']<=e['old_start_offset']);e['new_end_offset']=e['new_start_offset']+len(e['new'])
 files.append({'path':str(p.relative_to(ROOT)),'before_sha256':sha(before),'after_sha256':sha(after),'edits':edits})
manifest=OUT/'PHYSICAL_EDITS_STAGE3.json'
if manifest.exists():raise SystemExit('Stage3 already materialized; use check_all.py')
manifest.write_text(json.dumps({'source_basis':'after PHYSICAL_EDITS_STAGE2.json; preserved chain','files':files},indent=2)+'\n')
for f in files:
 p=ROOT/f['path'];text=p.read_text();assert sha(text)==f['before_sha256']
 for e in reversed(f['edits']):text=text[:e['old_start_offset']]+e['new']+text[e['old_end_offset']:]
 assert sha(text)==f['after_sha256'];p.write_text(text)
print(json.dumps({'stage3_files':len(files),'patches':sum(len(f['edits']) for f in files)}))
