from pathlib import Path,PurePosixPath
import json,hashlib,tarfile,subprocess,os,collections,datetime,re,struct
W=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3');A=W/'a1';R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30');O=A/R/'source_review_RV29/vk_f01_runtime_28';Q=O/'_run_records';H=lambda b:hashlib.sha256(b).hexdigest();J=lambda p:json.loads(p.read_text());ph=lambda s:Path(s.replace('<wt>',str(W)));dt=lambda s:datetime.datetime.fromisoformat(s.replace('Z','+00:00'));t=tarfile.open(Q/'committed_packets.tar');fs={str(PurePosixPath(m.name)):t.extractfile(m).read() for m in t if m.isfile()};P=str(R/'I22/vk_f01_diagnostic_01');get=lambda n:fs[P+'/'+n];ready=json.loads(get('READINESS.json'));final=json.loads(get('FINAL_CHECK.json'));proposal=J(A/R/'I23/vk_f01_diagnosis_15/_run_records/COMMAND_PROPOSAL.json');bind=J(A/R/'I23/vk_f01_diagnosis_15/_run_records/SOURCE_BINDING.json');root=ph(bind['archive_path']);mf=J(A/R/'I23/vk_prep_05/SOURCE_MANIFEST.json');env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');vr=Path('projects/chirality-piping/validation/benchmarks/numerical_robustness');cases=[json.loads(l) for l in (root/vr/'cases/rf_chain.jsonl').read_text().splitlines()];frozen=J(root/vr/'observations/kernel_lane/rf_chain.json');ids=[c['id'] for c in cases];assert len(ids)==len(set(ids))==30
assert len(mf['files'])==189 and {str(f.relative_to(root)) for f in root.rglob('*') if f.is_file()}=={r['path'] for r in mf['files']}
for r in mf['files']:
 f=root/r['path'];b=f.read_bytes();assert H(b)==r['sha256'] and len(b)==r['size'] and (f.stat().st_mode&0o777)==int(r['mode'],8);assert not f.is_symlink() and f.stat().st_nlink==1
art=bind['normal_release_artifact'];assert art==ready['binary']==final['binary'];assert H(ph(art['path']).read_bytes())==art['sha256'];assert H(ph(art['fingerprint_path']).read_bytes())==art['fingerprint_sha256'];assert art['profile']['test']==False and art['profile']['opt_level']=='3';assert art['cargo_features']==['seeded-faults']
for r in final['dependency_fingerprints']:
 b=ph(r['path']).read_bytes();assert H(b)==r['sha256'];meta=json.loads(b);assert meta['features']==r['features'] and meta['rustflags']==[]
def same(a,b,path=''):
 assert type(a) is type(b),(path,type(a),type(b))
 if isinstance(a,dict):
  assert a.keys()==b.keys(),path
  for k in a:same(a[k],b[k],path+'/'+k)
 elif isinstance(a,list):
  assert len(a)==len(b),path
  for i,(x,y) in enumerate(zip(a,b)):same(x,y,path+'/'+str(i))
 else:assert a==b,(path,a,b)
def decode(text):
 decoder=json.JSONDecoder();records=[];i=0
 while True:
  while i<len(text) and text[i].isspace():i+=1
  if i>=len(text) or text[i]!='{':return records,text[i:]
  obj,i=decoder.raw_decode(text,i);records.append(obj)
records={};execution=[];last=dt(ready['start_utc'])
for plan in proposal['processes_in_proposed_order']:
 name=plan['proposal_id'];e=json.loads(get(name+'/EVIDENCE.json'));assert e['executed'] and e['fresh_process_required'];assert e['argv']==plan['argv'] and e['environment']==plan['environment'] and e['unset_environment']==plan['unset_environment'] and e['cwd']==plan['cwd'];assert e['argv']==[art['path'],'RF-CHAIN']+['--show='+id for id in ids];assert '--write' not in e['argv'];assert e['tool_initial']['exit_code']==0 and 'session_id' not in e['tool_initial'];assert '5387' in e['preflight'] and 'memguard.sh' in e['preflight'];assert last<dt(e['launch_utc'])<dt(ready['end_utc']);last=dt(e['launch_utc'])
 streams={}
 for channel in ['stdout','stderr']:
  raw=ph(e['log']+'.'+channel).read_bytes();assert H(raw)==e['raw_hashes'][channel];assert raw.decode().replace(str(W),'<wt>')==get(name+'/'+channel+'.txt').decode();streams[channel]=raw.decode()
 rr,report=decode(streams['stdout']);assert len(rr)==30 and [r['id'] for r in rr]==ids;same(rr,json.loads(get(name+'/RECORDS.json')));assert report.strip()==get(name+'/REPORT.txt').decode().strip();assert all(type(r['case_limit']) is int and r['case_limit']==2**64-1 for r in rr)
 if name!='D02_F01':
  same(rr,frozen);assert 'rows 2760: passes 2700, absolute-range passes 0, not covered 0 | structural zeros 60, expected unresolved 0, failures 0' in report;assert 'outcomes {"selected Some(128)": 30}' in report;assert 'controls discriminated 82, non-discriminating 38, undiscriminated [], unexpectedly failing []' in report;assert 'FAILURE' not in report
 else:assert report.count('not selected: Unresolved Ceiling [Restrained]')==30 and 'rows 2760: passes 0' in report
 times=re.findall(r'^\s*([0-9.]+) real\s+([0-9.]+) user\s+([0-9.]+) sys',streams['stderr'],re.M);assert len(times)==1 and float(times[0][0])<300
 records[name]=rr;execution.append({'id':name,'fault':e['environment']['FK_SEEDED_FAULT'],'launch_utc':e['launch_utc'],'exit':0,'raw_hashes':e['raw_hashes'],'real_seconds':times[0][0],'synchronous_completion':True,'exact_UTC_completion':'not separately recorded; completed tool return precedes next launch/final check','preflight':e['preflight']})
same(records['D01_NONE_BEFORE'],records['D03_NONE_RETURN']);assert dt(final['checked_utc'])<dt(ready['end_utc'])
attrs=json.loads(get('CASE_ATTRIBUTION.json'));counts=collections.Counter();verified=[]
for index,(case,none,fault,back,attr) in enumerate(zip(cases,records['D01_NONE_BEFORE'],records['D02_F01'],records['D03_NONE_RETURN'],attrs)):
 id=case['id'];assert none['id']==fault['id']==back['id']==attr['id']==id;assert none['schema']==fault['schema']==back['schema']=='vk-case-record-v1';assert none['family']==fault['family']==back['family']=='RF-CHAIN';assert none['k4src_sha256']==fault['k4src_sha256']==back['k4src_sha256']==case['k4src_sha256']==attr['source_sha256'];assert attr['original_source_line']==index+1
 ns=[[a['precision'],a['role'],a['outcome']] for a in none['attempts']];assert ns==[[128,'Candidate','Accepted'],[256,'Verification','Verified']];assert none['selected_precision']==128 and none['verification_precision']==256
 torsion='RF-CHAIN-T-' in id;special=id in ['RF-CHAIN-A-n03-r1e-04','RF-CHAIN-A-n03-r1e-06'];component=3 if torsion else 0;reference='th.N0.RX' if torsion else 'u.N0.UX';kind='Rotation' if torsion else 'Translation';compname='Rx' if torsion else 'Ux'
 reason='Rejected(VerificationEstimate { quantity: EndAction { member: 1, end: I, component: Ux }, body: 0, kind: Force })' if special else 'Rejected(StopRule { quantity: Displacement(Dof { node: 0, component: '+compname+' }), body: 0, kind: '+kind+' })'
 seq=[[a['precision'],a['role'],a['outcome']] for a in fault['attempts']];expected=[[128,'Candidate',reason],[256,'VerificationThenCandidate',reason],[512,'VerificationThenCandidate',reason],[1024,'Verification','Solved']];assert seq==expected==attr['F01_sequence'] and ns==attr['NONE_sequence'];assert fault['outcome']=='Unresolved Ceiling [Restrained]' and 'selected_precision' not in fault and 'verification_precision' not in fault;assert [a['residual_basis'] for a in fault['attempts']]==[192,320,576,1024]
 assert all(not any(x in a['outcome'] for x in ['Publication','VerificationFailed','Budget','Failed(Stop','Source','Structure']) for a in fault['attempts']);assert [0,component] not in case['model']['constraints'];spring=next(x for x in case['model']['springs'] if x[0]=='S.N0.0');assert spring==attr['root_spring'] and spring[1:5]==[1,0,'r' if torsion else 't',component] and spring[5]==['3ff0000000000000','0000000000000000','0000000000000000'];assert struct.unpack('>d',bytes.fromhex(spring[6]))[0]>0;assert 'S.N0.0' not in case['model']['omitted_springs']
 ref=next(r for r in case['rows'] if r[0]==reference);assert ref==attr['root_reference_row'] and ref[2]==('rotation' if torsion else 'translation') and ref[1]=='1.000000000000000000000000000000000000000e-4';member=next(m for m in case['model']['members'] if m[1]==1);assert member[:4]==attr['root_incident_member']==['M1',1,0,1]
 # Original chain is one connected body, containing root and member1's I node.
 adjacency=collections.defaultdict(set)
 for mm in case['model']['members']:adjacency[mm[2]].add(mm[3]);adjacency[mm[3]].add(mm[2])
 reached={0};todo=[0]
 while todo:
  for n in adjacency[todo.pop()]-reached:reached.add(n);todo.append(n)
 assert len(reached)==len(case['model']['nodes']) and attr['body']==0
 counts['torsion_root_Rx_StopRule' if torsion else 'axial_member1_I_Ux_VerificationEstimate' if special else 'axial_root_Ux_StopRule']+=1
 verified.append({'id':id,'source_sha256':case['k4src_sha256'],'body':0,'reference_row':ref,'root_spring':spring,'root_component_free':True,'root_incident_member':member[:4],'force_reference':next((r for r in case['rows'] if r[0]=='N.M1'),None) if special else None,'NONE_sequence':ns,'F01_sequence':seq,'historical_control_discriminating':next(x[1] for x in case['controls'] if x[0]=='NC-STORED-ASSEMBLY'),'complete_NONE_restoration':True})
assert dict(counts)=={'torsion_root_Rx_StopRule':15,'axial_member1_I_Ux_VerificationEstimate':2,'axial_root_Ux_StopRule':13}
receipt=json.loads(get('CONTROL_ADDENDUM_RECEIPT.json'));ref=receipt['ref'];commit,path=ref.split(':',1);b=subprocess.check_output(['git','-C',str(W/'numerics'),'show',ref],env=env);assert H(b)==receipt['sha256'] and b==get('CONTROL_ADDENDUM.md');assert '18:37:50' in receipt['actual_committed_clarification_received_read_utc'];assert execution[2]['launch_utc']==receipt['original_D03_launch_utc'];assert execution[2]['launch_utc']<'2026-10-01T18:36:52Z'
report={'candidate':'0613cb0f871a29123ca8c57b720b5d9a166f840d','records_from_raw_stdout':90,'exact_type_value_comparisons':True,'u64_MAX_exact_in_all90_records':True,'source_files_modes_checked':189,'same_release_binary':art,'actual_processes':execution,'case_reason_counts':dict(counts),'all_cases':verified,'late_control_clarification':receipt,'authority_assessment':'D03 was already planned in original grant after a normal complete attributable D02; actual D02 has only eligible new persistent R7 reasons. Later clarification is preserved and not used retroactively.','missing_private_values':'No exact failing R7 numeric operands/private radii were printed; do not invent them.','finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};(Q/'INDEPENDENT_CHECKS.json').write_text(json.dumps(report,indent=2)+'\n');print('PASS90rawrecords/30caseattributions/189sourcefiles/3processes');print(dict(counts));print(report['finished_utc'])
