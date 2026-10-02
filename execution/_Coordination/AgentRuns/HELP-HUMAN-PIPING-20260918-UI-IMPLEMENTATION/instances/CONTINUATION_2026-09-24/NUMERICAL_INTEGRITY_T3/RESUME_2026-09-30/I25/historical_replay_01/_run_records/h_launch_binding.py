from inspect import *
import re,copy
WT=pathlib.Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3')
raw=WT/'scratch/i16/b3'
s=read(raw/'run_slot.sh');build=read(raw/'build_release.sh')
(REC/'H_ORIGINAL_run_slot.sh.txt').write_text(s);(REC/'H_ORIGINAL_build_release.sh.txt').write_text(build)
binary,counts,records=re.search(r'--binary (\S+) --counts (\S+) --records (\S+)',s).groups()
assert str(raw/'records')==records
meta=js(raw/'records/metadata.json');commeta=js(T3/'IMPLEMENTATION/K6B/_run_records/b3/records/metadata.json');assert meta==commeta
assert meta['binary_sha256']=='20b6777675a1ddd46c7c4d82cc3b45ca864008bb12f031cc0d4fe56bacf6fdee'
assert meta['source_commit']=='4eeb206c09b67b9c5f83ca89f5e66d188ad5a02b'
rawrows=[json.loads(x)for x in read(raw/'records/records.jsonl').splitlines()];repo=[json.loads(x)for x in read(T3/'IMPLEMENTATION/K6B/_run_records/b3/records/records.jsonl').splitlines()];assert rawrows==repo
num=js(R/'I21/h_numeric_19/PHASES.json')['rows']; hs={v['run_id']:v for x in num for v in x['launch_variants']}
rows=[]
for r in repo:
 if r.get('mode')!='w1a':continue
 a=r['argv'];rid=r['run_id'];v=hs[rid]
 assert a[a.index('--counts-file')+1]==counts
 assert r['repeats']==5
 fullargs=copy.deepcopy(a);fullargs[0]=binary
 if '--dump-published'in a:
  j=a.index('--dump-published')+1;assert a[j]==rid+'.rows';fullargs[j]=records+'/'+a[j]
 stored=[]
 for k in ['--model','--model-file','--counts-file','--dump-solution','--dump-pattern','--dump-published']:
  if k in fullargs:stored.append(fullargs[fullargs.index(k)+1])
 actual=sum(len(t.encode())for t in stored);delta=actual-v['persistent_argument_bytes']
 assert ('--w1-prefixes'in a)==v['prefix']
 rows.append({'run_id':rid,'model':r['model'],'pass':r['pass'],'argv_reconstructed_from_original_script_and_record':fullargs,'retained_string_utf8_bytes':actual,'reference_string_utf8_bytes':v['persistent_argument_bytes'],'actual_minus_reference_bytes':delta,'source_window_delta':delta,'solve_window_delta':delta,'prefix_window_delta':delta if v['prefix']else 0,'fixed_delta':delta,'qualification':'path arithmetic bound by original unredacted script plus matching original record/metadata; reference ordinary-production profile still conditional'})
save('H_ACTUAL_LAUNCH_BINDING.json',{'historical_metadata':meta,'raw_records_equal_committed':True,'recipe_origin':str(raw/'run_slot.sh'),'rows':rows,'max_delta':max(x['actual_minus_reference_bytes']for x in rows),'min_delta':min(x['actual_minus_reference_bytes']for x in rows)})
print(json.dumps({'rows':len(rows),'deltas':sorted(set(x['actual_minus_reference_bytes']for x in rows)),'example':rows[0]},indent=2))
