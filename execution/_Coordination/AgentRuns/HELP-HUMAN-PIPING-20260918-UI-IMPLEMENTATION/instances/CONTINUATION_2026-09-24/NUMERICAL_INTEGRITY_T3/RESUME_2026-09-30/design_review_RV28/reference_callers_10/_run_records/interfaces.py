"""Supplemental scope/interface checks; never evaluates or joins kernel bytes."""
from pathlib import Path
import hashlib,json,copy,sys
K,OUT=map(Path,sys.argv[1:]);raw=OUT/'_run_records';WT=K.parent
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3';R=T+'/RESUME_2026-09-30';P=R+'/metric_design_14_reference_callers'
checks=[];records=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def read(p):
 b=(K/p).read_bytes();records.append({'path':p,'sha256':sha(b),'bytes':len(b)});return b
def load(p):return json.loads(read(p))
def ck(n,x):
 assert x,n
 checks.append(n)
for path,expected in [(R+'/metric_design_10_vr_numbers','a5390504ba0198ebe280877b75d8b374a38c266e780a7e3dc0d7a280f1fb9a9c'),(R+'/metric_design_11_vr_correction','21bc713d669f4b4dcfe618a3454a874fef97edba230920b56647f3d63d8b8b14')]:
 s=read(path+'/SHA256SUMS');ck(path+' seal',sha(s)==expected)
 for line in s.decode().splitlines():
  h,f=line.split(maxsplit=1);ck(path+'/'+f,sha((K/path/f).read_bytes())==h)
base=load(R+'/metric_design_10_vr_numbers/VR_CALLER_TABLE.json');overlay=load(R+'/metric_design_11_vr_correction/OVERLAY.json');byid={r['id']:r for r in base['rows']}
for r in overlay['rows']:
 for metric,m in r['metrics'].items():
  for name,v in m['replacements'].items():
   ck(r['id']+metric+name+' original',byid[r['id']][metric]['kernel_phase_addends'][name]==v['old']);byid[r['id']][metric]['kernel_phase_addends'][name]=v['new']
cli=load(P+'/CLI24_CALLERS.json')
for row in cli['rows']:
 v=copy.deepcopy(row)
 for metric in ['requested','moving']:v[metric].pop('result5_components')
 ck(row['id']+' original plus sealed overlay unchanged',v==byid[row['id']])
external=load(R+'/I23/external_inputs_06/INPUTS.json')['entries']
for ent in external:
 p=WT/ent['raw_path'].removeprefix('<wt>/');a=p.stat();b=p.read_bytes();z=p.stat();ck(ent['id']+' stable external hash',sha(b)==ent['expected']['model_sha256'] and (a.st_size,a.st_mtime_ns,a.st_ino)==(z.st_size,z.st_mtime_ns,z.st_ino));records.append({'origin':ent['raw_path'],'sha256':sha(b),'bytes':len(b)})
ref=load(P+'/REFERENCE193_CALLERS.json');join=load(P+'/KERNEL_JOIN_INPUTS.json');kern=load(R+'/I21/kernel_reference_22/RESULT5_KERNEL_INPUTS.json');kb={r['id']:r for r in kern}
ck('193 matching kernel descriptors',len(kb)==193 and set(kb)=={r['id'] for r in ref['rows']})
for r in ref['rows']:
 d=r['shape'];k=kb[r['id']];desc=k['descriptor']
 for name in ['N','m','s','d','u','r','l','t','n','f','z','q','X','B','b']:ck(r['id']+' shape only '+name,d[name]==desc[name])
 ck(r['id']+' exact stored h and IDs',d['h_W1_unused_for_sparse']==desc['h'] and d['load_id_bytes']==desc['source_id_bytes'])
 ck(r['id']+' upper policy binding',k['B_policy']['kind']=='upper' and k['b_policy']['kind']=='upper' and k['B_policy']['value']==d['N'] and k['b_policy']['value']==d['f'])
 ck(r['id']+' kernel interface shape',all(name in k for name in ['K_all','K_128','O_all','O_128','B0','R7']))
 for metric in ['requested','moving']:
  c=r[metric];ck(r['id']+metric+' 15 phases five addends',len(c['caller_only_phases'])==15 and len(c['kernel_phase_addends'])==5)
  ck(r['id']+metric+' caller no missing byte interface',c['unbound_source_addends']=={})
  ck(r['id']+metric+' checked positive u64',all(type(v) is int and 0<=v<2**64 for v in list(c['caller_only_phases'].values())+list(c['kernel_phase_addends'].values())))
  v=c['result5_components'];ck(r['id']+metric+' caller subsets',v['C_cut']>=v['M'] and v['comparison_over_fixed_addend']>=0)
 ck(r['id']+' move dominates caller request',all(r['moving']['caller_only_phases'][k]>=v for k,v in r['requested']['caller_only_phases'].items()))
basepoint='/reference';manifest=basepoint+'/projects/chirality-piping/validation/benchmarks/numerical_robustness';binary=basepoint+'/target/release/examples/vk_scale'
launches=load(P+'/REFERENCE_LAUNCHES.json')['rows'];ck('reference launch193',len(launches)==193)
for l in launches:
 ck(l['id']+' reference root fixed',l['cwd']==basepoint and l['CARGO_MANIFEST_DIR']==manifest and l['normal_argv'][0]==binary)
 ck(l['id']+' actual reference argv bytes',[len(x.encode()) for x in l['normal_argv']]==l['argument_utf8_lengths'] and [len(x.encode()) for x in l['counts_argv']]==l['counts_argument_utf8_lengths'])
 ck(l['id']+' ordinary versus counts flags','--counts-only' not in l['normal_argv'] and '--counts-only' in l['counts_argv'] and '--model-file' not in l['normal_argv'])
(raw/'INTERFACE_CHECKS.json').write_text(json.dumps({'status':'PASS','checks':len(checks),'check_names':checks,'origins':records,'kernel22_scope':'descriptor policy and API shape only; no kernel arithmetic or global join','CLI24':'all original fields equal sealed metric10 plus metric11 overlay'},indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'CLI24_original_values_preserved':24,'external_hashes':12,'reference_kernel_shape_matches':193,'kernel_numeric_joined':False}))
