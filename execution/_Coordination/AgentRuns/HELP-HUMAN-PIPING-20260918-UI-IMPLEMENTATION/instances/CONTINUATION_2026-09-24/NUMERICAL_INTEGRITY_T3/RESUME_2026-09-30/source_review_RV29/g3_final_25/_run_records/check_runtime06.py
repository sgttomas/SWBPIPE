from pathlib import Path,PurePosixPath
import json,hashlib,subprocess,os,tarfile,io,datetime,re,collections
W=Path('/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3');A=W/'a1';R=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30');O=A/R/'source_review_RV29/g3_final_25';Q=O/'_run_records';rev='9ef9508dea8205297631299b3c384a3cd5185bef';basis='40129a225d73860ac2a53da9a2fa73869df668f3';env=dict(os.environ,GIT_OPTIONAL_LOCKS='0');H=lambda b:hashlib.sha256(b).hexdigest();ph=lambda s:Path(s.replace('<wt>',str(W)));dt=lambda s:datetime.datetime.fromisoformat(s.replace('Z','+00:00'));fk='projects/chirality-piping/core/solver/frame_kernel'
def git(*a):return subprocess.check_output(['git','-C',str(A),*a],env=env)
def blob(p,r=rev):return git('show',r+':'+str(p))
t=tarfile.open(Q/'committed_runtime06.tar');files={str(PurePosixPath(m.name)):t.extractfile(m).read() for m in t if m.isfile()};p=str(R/'I22/protected_g3/runtime_06');m=str(R/'manager/protected_g3/runtime_06');get=lambda n:files[str(PurePosixPath(p)/n)];c=json.loads(get('COMMANDS.json'));manifest=json.loads(blob(R/'verification/g3_preparation_01/MANIFEST.json'));gs={g['runtime_id']:g for g in manifest['entries']};raw=git('archive',basis,'projects/chirality-piping/core');t=tarfile.open(fileobj=io.BytesIO(raw));orig={m.name:t.extractfile(m).read() for m in t if m.isfile()};assert len(orig)==560
seen=set();sources=[]
for id in [f'R{i:02}' for i in range(46,53)]+['BASELINE']:
 root=W/'scratch/i22/mutations_e1/base' if id=='BASELINE' else ph(gs[id]['source_root']);scope=fk if id=='BASELINE' else gs[id]['source_scope'];expected={n:H(b) for n,b in orig.items() if n.startswith(scope+'/')}
 if id!='BASELINE':expected.update({x['path']:x['after_sha256'] for x in gs[id]['files']})
 assert {str(f.relative_to(root)) for f in root.rglob('*') if f.is_file()}==set(expected),(id,'file set')
 for n,h in expected.items():
  f=root/n;assert H(f.read_bytes())==h,(id,n);st=f.stat();assert st.st_nlink==1 and (st.st_dev,st.st_ino) not in seen;seen.add((st.st_dev,st.st_ino));assert not f.is_symlink() and f.resolve().is_relative_to(root)
 sources.append({'id':id,'scope':scope,'files':len(expected),'only_frozen_postimages':True})
assert sum(s['files'] for s in sources)==1372
# Explicit prospective deadline addendum and original grant; read only their pinned bytes.
grants=[]
for key in ['grant','window_addendum']:
 x=c[key];b=blob(x['path'],x['commit']);assert H(b)==x['sha256'];grants.append({'kind':key,'commit':x['commit'],'path':x['path'],'sha256':H(b),'text':b.decode()})
assert dt(c['window_addendum']['ROOT_decision_utc'])<dt(c['window_addendum']['received_read_utc'])<dt(c['original_end_utc'])
releases=json.loads(blob(R/'manager/protected_g3/runtime_04/RELEASES.json'));release={r['runtime_id']:r for r in releases['releases']};helper=blob(R/'I22/protected_g3/batch_prep_04/after.txt');assert H(helper)==c['helper_sha256']=='b6313acb720917b3bfb549a049aacfe9097bf95f224ec7c001afc794164c1029';assert H(blob(R/'I22/protected_g3/batch_prep_04/check_body.txt'))==c['checker_sha256'];assert not c['rebuild_R46'] and not c['repatch_or_helper_change']
bins={};fps={};runs=[];counts=collections.Counter();prev=dt(c['restart_utc']);cut=dt(c['window_addendum']['received_read_utc'])
for r in c['runs']:
 e=json.loads(get(r['evidence']));inf=e['info'];out=get(r['stdout']);err=get(r['stderr']);ro=ph(r['log']+'.stdout').read_bytes();stderr_raw=ph(r['log']+'.stderr').read_bytes();assert ro==out and H(out)==inf['stdout_sha256'];assert H(stderr_raw)==inf['stderr_raw_sha256'];assert stderr_raw.decode().replace(str(W),'<wt>')==err.decode();assert out.decode()==inf['stdout'] and err.decode()==inf['stderr']
 for k in ['kind','id','argv','cmd','filter','workdir','launch_utc']:assert r[k]==e[k]
 for setting in ['RUSTUP_TOOLCHAIN=1.97.1','RUSTUP_AUTO_INSTALL=0','CARGO_INCREMENTAL=0','RUST_TEST_THREADS=2','CARGO_NET_OFFLINE=true','FK_SEEDED_FAULT=NONE','-u RUSTFLAGS','-u CARGO_ENCODED_RUSTFLAGS','-u RUSTC_WRAPPER','-u RUSTC_WORKSPACE_WRAPPER']:assert setting in r['cmd']
 assert r['guard_before']['exit_code']==0 and '5387' in r['guard_before']['output'] and 'memguard.sh' in r['guard_before']['output'] and '5387' in inf['guard_after']
 t0,t1=dt(r['launch_utc']),dt(r['completed_utc']);assert prev<=t0<=t1;prev=t1;window=dt(c['original_end_utc']) if t0<cut else dt(c['window_addendum']['new_end_utc']);assert t1<window;assert ('original c86' in r['window_authority'])==(t0<cut)
 failed=r['kind'] in ['mutant','historical_mutant'];exit=101 if failed else 0;assert r['tool_completion']['exit_code']==exit
 source_id=r['source_id'];root='<wt>/scratch/i22/mutations_e1/base' if source_id=='BASELINE' else gs[source_id]['source_root'];assert r['workdir']==root+'/'+fk
 if r['kind']=='build':
  assert r['id']!='R46';assert r['argv']==release[r['id']]['compile'];assert dt(release[r['id']]['released_at_utc'])<t0;assert 'Finished `test` profile' in err.decode();assert r['binary']['path'].split('/')[-1] in err.decode()
 elif r['kind']=='list':assert '--exact' in r['argv'] and r['filter']+': test' in out.decode() and '1 test, 0 benchmarks' in out.decode()
 else:assert '--exact' in r['argv'] and '--test-threads=2' in r['argv'] and r['filter'] in out.decode() and 'running 1 test' in out.decode();assert ('0 passed; 1 failed;' if failed else '1 passed; 0 failed;') in out.decode()
 if r['id']=='R46':assert dt(c['R46_release']['at'])<t0
 b=r['binary'];assert b==inf['binary'];bp=ph(b['path'])
 if str(bp) not in bins:assert H(bp.read_bytes())==b['sha256'];bins[str(bp)]=b['sha256']
 fp=r['fingerprint'];fb=ph(fp['path']).read_bytes();assert fp==inf['fingerprint'] and fb==get(r['raw_fingerprint']) and H(fb)==fp['sha256'];meta=json.loads(fb);assert meta['features']=='[]' and meta['rustflags']==[];fps[fp['path']]=fp['sha256']
 rr={'id':r['id'],'kind':r['kind'],'index':r.get('index'),'name':r['name'],'filter':r['filter'],'source_id':source_id,'start':r['launch_utc'],'end':r['completed_utc'],'window_authority':r['window_authority'],'exit':exit,'stdout_sha256':H(out),'raw_stderr_sha256':H(stderr_raw),'binary_sha256':b['sha256'],'fingerprint_sha256':fp['sha256']}
 if failed:
  s=err.decode().split('note:')[0].strip();rr['assertion_prefix']=s[:6000];rr['assertion_bytes']=len(s.encode());rr['assertion_sha256']=H(s.encode())
 runs.append(rr);counts[r['kind']]+=1
assert len(runs)==35 and len(bins)==9
for id in [f'R{i:02}' for i in range(47,53)]:
 rs=[r for r in runs if r['id']==id];expected=['build','list','mutant','control']+(['list','mutant','control'] if id=='R52' else []);assert [r['kind'] for r in rs]==expected
 for i in [0,1] if id=='R52' else [0]:
  mu=next(r for r in rs if r['kind']=='mutant' and r['index']==i);co=next(r for r in rs if r['kind']=='control' and r['index']==i);assert mu['filter']==co['filter']==gs[id]['filters'][i] and mu['end']<co['start'];assert co['binary_sha256']==gs[id]['baseline_binding']['sha256']
r46=[r for r in runs if r['id']=='R46'];assert len(r46)==8 and all(r['kind']!='build' for r in r46);assert next(r for r in r46 if r['kind']=='historical_mutant')['binary_sha256']=='ff8dbe80165c3097db94efc4d17dfe58cb04e1ace6ac891b4020146cbab8f337';assert r46[3]['filter']==r46[5]['filter']==r46[7]['filter'];assert r46[1]['binary_sha256']==r46[3]['binary_sha256']==r46[7]['binary_sha256']=='cd5967f30c1ca9d8957d02126d2639e943435b84f86c5d02c0b21c904af8e00a'
# Exact all-row comparison of historical set19, avoiding floating conversion of integer bounds.
s=get('R46/historical_mutant/stderr.txt').decode();assert 'classification_tests.rs:96:' in s and 'failed: set 19' in s
pat=r'InputDerived|RelativeVerified|Unpublishable|AbsoluteVerified \{ bound_bits: \d+ \}'
left=re.findall(pat,next(l for l in s.splitlines() if l.startswith('  left:')));right=re.findall(pat,next(l for l in s.splitlines() if l.startswith(' right:')));assert len(left)==len(right)==23;assert [(i,a,b) for i,(a,b) in enumerate(zip(left,right)) if a!=b]==[(20,'AbsoluteVerified { bound_bits: 2 }','AbsoluteVerified { bound_bits: 3 }')]
lines=orig[fk+'/tests/retained_k4/classification.txt'].decode().splitlines();vectors=[];n=-1
for l in lines:
 if l.startswith('set '):n+=1
 if n==19 and l.startswith('row '):
  token=l.split()[5];vectors.append({'I':'InputDerived','R':'RelativeVerified','U':'Unpublishable'}.get(token,'AbsoluteVerified { bound_bits: '+str(int(token[2:],16))+' }' if token.startswith('A:') else 'INVALID'))
assert right==vectors
# R51 contingency remained untouched: original plus its prepared overlay only, targets/logs empty.
prep=json.loads(blob(R/'I23/r51_fallback_13/COPIES.json'));mf=json.loads(blob(R/'I23/r51_fallback_13/SOURCE_MANIFEST.json'));unrun=[]
for copy in prep['copies']:
 root=ph(copy['source_root']);assert {str(p.relative_to(root)) for p in root.rglob('*') if p.is_file()}=={x['path'] for x in mf}
 for x in mf:assert H((root/x['path']).read_bytes())==x['prepared_sha256']
 for key in ['target','logs']:assert not list(ph(copy[key]).iterdir())
 unrun.append({'name':copy['name'],'source_files':len(mf),'production_fault_unapplied':True,'targets_logs_empty':True})
report={'candidate':rev,'physical_source_files':1372,'source_copies':sources,'counts':dict(counts),'commands':35,'tests':18,'binaries':bins,'fingerprints':fps,'runs':runs,'set19':{'rows':23,'only_difference':20,'actual_bound_bits':2,'expected_bound_bits':3,'exact_right_vector_matches_original_corpus':True},'R51_contingent_copies':unrun,'grant_bindings':grants,'precheck_output_limit':'Per-build checker stdout absent; selected immutable launcher/manager releases/refresh and complete post-execution bindings are retained.','finished_utc':datetime.datetime.now(datetime.timezone.utc).isoformat()};(Q/'RUNTIME06_CHECKS.json').write_text(json.dumps(report,indent=2)+'\n');print('VERIFIED',len(runs),'commands',len(bins),'binaries',1372,'runtime source files; R51contingency232files untouched');print(report['finished_utc'])
