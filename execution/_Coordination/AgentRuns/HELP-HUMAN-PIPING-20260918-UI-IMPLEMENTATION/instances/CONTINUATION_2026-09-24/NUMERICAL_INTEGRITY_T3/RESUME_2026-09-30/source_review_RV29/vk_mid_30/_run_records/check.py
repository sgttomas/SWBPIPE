from pathlib import Path,PurePosixPath
import json,hashlib,tarfile,re,datetime,subprocess,os,collections
W=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3');A=W/'a1';R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30');O=A/R/'source_review_RV29/vk_mid_30';Q=O/'_run_records';t=tarfile.open(Q/'committed_packets.tar');fs={str(PurePosixPath(m.name)):t.extractfile(m).read() for m in t if m.isfile()};p=str(R/'I22/vk_runtime_02');p3=str(R/'I22/vk_f03_diagnostic_01');get=lambda p,n:fs[p+'/'+n];J=lambda p,n:json.loads(get(p,n));H=lambda b:hashlib.sha256(b).hexdigest();ph=lambda s:Path(s.replace('<wt>',str(W)));dt=lambda s:datetime.datetime.fromisoformat(s.replace('Z','+00:00'));root=W/'scratch/i23/vk_prep/archive';vr=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness');mf=json.loads((A/R/'I23/vk_prep_05/SOURCE_MANIFEST.json').read_text());env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
assert len(mf['files'])==189 and {str(f.relative_to(root)) for f in root.rglob('*') if f.is_file()}=={x['path'] for x in mf['files']}
for r in mf['files']:
 f=root/r['path'];b=f.read_bytes();assert H(b)==r['sha256'] and len(b)==r['size'] and (f.stat().st_mode&0o777)==int(r['mode'],8);assert not f.is_symlink() and f.stat().st_nlink==1
cases={};frozen=[]
for family in ['rf_chain','rf_skew']:
 cc=[json.loads(l) for l in (root/vr/'cases'/f'{family}.jsonl').read_text().splitlines()]
 for c in cc:cases[c['id']]=c
 frozen.extend(json.loads((root/vr/'observations/kernel_lane'/f'{family}.json').read_text()))
# P10/P11/P12 exact original-process evidence, no P13 attribution inferred.
processes=[];binaries={};last=dt('2026-10-01T18:48:13Z');failure_records=[]
for id,seed,exit in [('P10','NONE',0),('P11','VK-F02',101),('P12','NONE',0)]:
 e=J(p,id+'/EVIDENCE.json');out=get(p,id+'/stdout.txt');err=get(p,id+'/stderr.txt');assert e['environment']['FK_SEEDED_FAULT']==seed and e['fresh_process'];assert e['argv'][1:]==['--exact','rf_chain','--test-threads=2','--nocapture'];assert e['cwd']==str(root.relative_to(W)).join(['<wt>/','/'+str(vr)]);assert '5387' in e['guard_before'] and 'memguard.sh' in e['guard_before'];assert e['tool_completion']['exit_code']==exit;assert dt(e['runtime_authority']['at'])<dt(e['launch_utc']);assert last<dt(e['launch_utc'])<=dt(e['completed_utc'])<dt(e['hard_end']);last=dt(e['completed_utc'])
 for ch in ['stdout','stderr']:
  raw=ph(e[ch+'_path']).read_bytes();assert H(raw)==e['raw_hashes'][ch];assert raw.decode().replace(str(W),'<wt>')==get(p,id+'/'+ch+'.txt').decode()
 b=e['binary'];assert H(ph(b['path']).read_bytes())==b['sha256'];fp=ph(b['fingerprint_path']).read_bytes();assert H(fp)==b['fingerprint_sha256'];assert json.loads(fp)['features']=='["seeded-faults"]' and json.loads(fp)['rustflags']==[];assert b['profile']['test'] and b['profile']['opt_level']=='0';binaries[b['path']]=b
 for flag in ['RUSTUP_TOOLCHAIN=1.97.1','RUSTUP_AUTO_INSTALL=0','CARGO_INCREMENTAL=0','RUST_TEST_THREADS=2','CARGO_NET_OFFLINE=true','FK_SEEDED_FAULT='+seed,'-u RUSTFLAGS','-u CARGO_ENCODED_RUSTFLAGS','-u RUSTC_WRAPPER','-u RUSTC_WORKSPACE_WRAPPER']:assert flag in e['cmd']
 text=out.decode();assert 'running 1 test' in text and 'RF-CHAIN: outcomes {"selected": 30}' in text
 if exit==0:
  assert '1 passed; 0 failed;' in text and 'rows 2760: passes 2700, absolute-range passes 0, not covered 0 | structural zeros 60, expected unresolved 0, failures 0' in text;assert 'FAILURE' not in text and 'controls discriminated 82, non-discriminating 38' in text
 else:
  assert '0 passed; 1 failed;' in text and 'tests/lane.rs:75:' in err.decode();lines=[l for l in text.splitlines() if l.startswith('RF-CHAIN: FAILURE ')];assert len(lines)==176
  for line in lines:
   m=re.fullmatch(r'RF-CHAIN: FAILURE ([^:]+): ([^ ]+) \(([^)]+)\) predicate',line);assert m,line;cid,key,reference=m.groups();row=next(x for x in cases[cid]['rows'] if x[0]==key);assert row[1]==reference;failure_records.append({'case':cid,'row':key,'reference':reference,'predicate':'existing exact judge'})
 processes.append({'id':id,'seed':seed,'exit':exit,'launch':e['launch_utc'],'completion':e['completed_utc'],'raw_hashes':e['raw_hashes'],'binary_sha256':b['sha256'],'fingerprint_sha256':b['fingerprint_sha256']})
# All four requested type families at r1e-10 AND r1e-12 are concrete protected witnesses.
required=[]
for suffix in ['r1e-10','r1e-12']:
 for kind,keys in [('T',['T.M1','tw.M1']),('A',['N.M1','ext.M1'])]:
  cid='RF-CHAIN-'+kind+'-n03-'+suffix
  for key in keys:assert any(x['case']==cid and x['row']==key for x in failure_records);required.append({'case':cid,'row':key})
(Q/'F02_FAILURES.json').write_text(json.dumps({'count':176,'all_references_match_original_corpus':True,'required_examples':required,'failures':failure_records},indent=2)+'\n')
# Decode both interleaved family segments from original release stdout, preserving exact ints.
def same(a,b,path=''):
 assert type(a) is type(b),(path,type(a),type(b))
 if isinstance(a,dict):
  assert a.keys()==b.keys(),path
  for k in a:same(a[k],b[k],path+'/'+k)
 elif isinstance(a,list):
  assert len(a)==len(b),path
  for i,(x,y) in enumerate(zip(a,b)):same(x,y,path+'/'+str(i))
 else:assert a==b,(path,a,b)
def decode(s):
 dec=json.JSONDecoder();rr=[];ends=[]
 for m in re.finditer(r'^\{',s,re.M):
  obj,end=dec.raw_decode(s,m.start());rr.append(obj);ends.append(end)
 reports={family:[line for line in s.splitlines() if line.startswith(family+':')] for family in ['RF-CHAIN','RF-SKEW']}
 return rr,reports
pro=json.loads((A/R/'verification/vk_f03_plan_01/_run_records/COMMAND_PROPOSAL.json').read_text());ready=J(p3,'READINESS.json');final=J(p3,'FINAL_CHECK.json');art=ready['binary'];assert art==final['binary'];assert H(ph(art['path']).read_bytes())==art['sha256'];assert H(ph(art['fingerprint_path']).read_bytes())==art['fingerprint_sha256'];assert art['profile']['test']==False and art['profile']['opt_level']=='3'
for fp in final['dependency_fingerprints']:
 raw=ph(fp['path']).read_bytes();assert H(raw)==fp['sha256'];meta=json.loads(raw);assert meta['features']==fp['features'] and meta['rustflags']==[]
recs={};obs=[];last=dt(ready['start_utc'])
for plan in pro['processes_in_proposed_order']:
 id=plan['proposal_id'];e=J(p3,id+'/EVIDENCE.json');assert e['argv']==plan['argv'] and e['environment']==plan['environment'] and e['unset_environment']==plan['unset_environment'] and e['cwd']==plan['cwd'];assert e['executed'] and e['fresh_process_required'] and e['tool_initial']['exit_code']==0 and 'session_id' not in e['tool_initial'];assert last<dt(e['launch_utc'])<dt(ready['end_utc']);last=dt(e['launch_utc']);assert '5387' in e['preflight'] and 'memguard.sh' in e['preflight']
 streams={}
 for ch in ['stdout','stderr']:
  raw=ph(e['log']+'.'+ch).read_bytes();assert H(raw)==e['raw_hashes'][ch];assert raw.decode().replace(str(W),'<wt>')==get(p3,id+'/'+ch+'.txt').decode();streams[ch]=raw.decode()
 rr,reports=decode(streams['stdout']);assert len(rr)==66 and [x['id'] for x in rr]==list(cases);same(rr,J(p3,id+'/RECORDS.json'));same(reports,J(p3,id+'/REPORTS.json'));assert all(type(x['case_limit']) is int and x['case_limit']==2**64-1 for x in rr)
 if id!='D02_F03':
  same(rr,frozen)
  for family,pin in pro['control_pins'].items():
   group=[x for x in rr if x['family']==family];assert len(group)==pin['records'];assert {k:sum(x['report'][k] for x in group) for k in group[0]['report']}==pin['report'];assert not any('FAILURE' in l for l in reports[family]);assert any('controls discriminated '+str(pin['controls_discriminated'])+', non-discriminating '+str(pin['controls_non_discriminating'])+', undiscriminated [], unexpectedly failing []' in l for l in reports[family])
 elapsed=re.findall(r'^\s*([0-9.]+) real\s+',streams['stderr'],re.M);assert len(elapsed)==1 and float(elapsed[0])<300
 recs[id]=rr;obs.append({'id':id,'launch':e['launch_utc'],'exit':0,'raw_hashes':e['raw_hashes'],'real_seconds':elapsed[0],'synchronous_tool_completion':True,'exact_completion_UTC':'not individually recorded; bounded by next command/final check'})
same(recs['D01_NONE_BEFORE'],recs['D03_NONE_RETURN']);assert dt(final['checked_utc'])<dt(ready['end_utc'])
links=J(p3,'CASE_LINKS.json');counts=collections.Counter();verified=[];components=['Ux','Uy','Uz','Rx','Ry','Rz']
for i,(case,base,fault,link) in enumerate(zip(cases.values(),recs['D01_NONE_BEFORE'],recs['D02_F03'],links)):
 id=case['id'];assert base['id']==fault['id']==link['id']==id;assert base['k4src_sha256']==fault['k4src_sha256']==link['source_sha256']==case['k4src_sha256'];assert base['family']==fault['family']==link['family']==case['family'];assert fault['schema']==base['schema']=='vk-case-record-v1';assert fault['outcome']=='Unresolved Ceiling '+base['geometry'];assert 'selected_precision' not in fault
 bs=[[a['precision'],a['role'],a['outcome']] for a in base['attempts']];assert bs==[[128,'Candidate','Accepted'],[256,'Verification','Verified']]==link['NONE_sequence'];seq=[[a['precision'],a['role'],a['outcome']] for a in fault['attempts']];same(seq,link['F03_sequence']);assert link['F03_top_outcome']==fault['outcome']
 is_pivot=seq[0][2].startswith('Failed(Stop(Pivot')
 if is_pivot:
  if id.startswith('RF-CHAIN'):g={3:14,5:26,10:59}[len(case['model']['members'])];group='CHAIN_Pivot_'+str(g)
  elif '-T-PIN-OFF-' in id:g=3;group='SKEW_T_PIN_OFF_Pivot3'
  elif '-T-CANT-OFF-' in id:g=5;group='SKEW_T_CANT_OFF_Pivot5'
  elif '-A-CANT-OFF-345-' in id:g=1;group='SKEW_A_CANT_OFF_345_Pivot1'
  elif '-A-CANT-OFF-122-' in id:g=2;group='SKEW_A_CANT_OFF_122_Pivot2'
  else:raise AssertionError(id)
  reason='Failed(Stop(Pivot { global_dof: '+str(g)+' }))';assert seq==[[p,'Candidate',reason] for p in [128,256,512]];assert len(fault['attempts'])==3 and link['reported_body'] is None
 else:
  assert id.startswith('RF-SKEW-') and '-AX-' in id;rot=id.startswith('RF-SKEW-T-');g=3 if rot else 0;group='SKEW_AX_root_'+('Rx' if rot else 'Ux')+'_StopRule';reason='Rejected(StopRule { quantity: Displacement(Dof { node: 0, component: '+('Rx' if rot else 'Ux')+' }), body: 0, kind: '+('Rotation' if rot else 'Translation')+' })';assert seq==[[128,'Candidate',reason],[256,'VerificationThenCandidate',reason],[512,'VerificationThenCandidate',reason],[1024,'Verification','Solved']];assert link['reported_body']==0
 counts[group]+=1;node,comp=divmod(g,6);assert [node,comp] not in case['model']['constraints'];assert link['observed_global_dof']==g and link['node']==node and link['component']==components[comp];incident=[m[:4] for m in case['model']['members'] if node in m[2:4]];springs=[s for s in case['model']['springs'] if s[2]==node];reference=('u.N'+str(node)+'.'+components[comp].upper()) if comp<3 else ('th.N'+str(node)+'.'+components[comp].upper());rows=[r for r in case['rows'] if r[0]==reference];assert link['source_incident_members_label_id_nodes']==incident and link['source_springs_at_reported_node']==springs and link['original_reference_rows']==rows;assert link['historical_NC_LOST_SOFT_controls']==[c for c in case['controls'] if c[0]=='NC-LOST-SOFT'];assert not any('Publication' in a[2] or 'Budget' in a[2] or 'VerificationFailed' in a[2] for a in seq)
 verified.append({'case':id,'family':case['family'],'source_sha256':case['k4src_sha256'],'complete_NONE_restores':True,'F03_sequence':seq,'observed_global_dof':g,'node':node,'component':components[comp],'reference_rows':rows,'incident_members':incident,'springs_at_reported_node':springs,'claim_limit':'Observed pivot/R7 location, not deleted contribution identity.'})
assert sum(v for k,v in counts.items() if 'Pivot' in k)==48 and sum(v for k,v in counts.items() if 'StopRule' in k)==18
report={'candidate':'2e1adc4015e833780a354bc0ee8ba0946b17cce6','source_files_checked':189,'F02':{'processes':processes,'all176_reference_strings_verified':True,'required_type_ratio_examples':required,'status':'QUALIFIED_VALUE_WITNESS_WITH_RETURNED_CONTROL','P10':'original debug F01 returned control PASS; does not supply P09 reasons'},'F03':{'processes':obs,'raw_decoded_records':198,'both66record_NONE_sets_exact_match_frozen':True,'type_sensitive_integer_compare':True,'reason_counts':dict(counts),'cases':verified,'status':'PROVENANCE_CONTROLS_AND_COMPLETE_NUMERICAL_SEQUENCES_VERIFIED; source attribution consequence pending separate independent review','no_universal_private_term_requirement':'No requirement to reconstruct every deleted term is imposed by this packet.'},'source_and_artifacts':{'debug_lane_binary':binaries,'release_binary':art},'P09_P13':'remain unqualified; release observations do not supply unprinted debug reasons','runtime_by_reviewer':'NONE','finished_checks_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};(Q/'CHECKS.json').write_text(json.dumps(report,indent=2)+'\n');print('CHECKED F02 176exactrefs/P10P12; F03 198rawrecords+66sequences',dict(counts));print(report['finished_checks_utc'])
