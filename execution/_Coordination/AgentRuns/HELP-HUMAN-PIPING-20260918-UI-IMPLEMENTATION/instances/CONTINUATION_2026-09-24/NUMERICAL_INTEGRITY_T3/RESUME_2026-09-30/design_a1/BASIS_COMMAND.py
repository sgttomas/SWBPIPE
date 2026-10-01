import hashlib,json,os,pathlib,platform,subprocess,sys
a1=pathlib.Path(sys.argv[1]); coord=pathlib.Path(sys.argv[2]); app=pathlib.Path(sys.argv[3]); out=pathlib.Path.cwd()
r=sys.argv[4]; t=r.rsplit('/',1)[0]; p='projects/chirality-piping'; fk=p+'/core/solver/frame_kernel'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
sources=[]
def record(root,path,origin,rev=None,consultation='selected source excerpts'):
    data=(root/path).read_bytes()
    item={'origin':origin,'path':path,'sha256':hashlib.sha256(data).hexdigest(),'bytes':len(data),'consultation':consultation}
    if rev:
        pinned=subprocess.run(['git','show',rev+':'+path],cwd=root,env=env,check=True,stdout=subprocess.PIPE).stdout
        item['revision']=rev;item['matches_revision']=pinned==data
        assert pinned==data,(path,rev)
    sources.append(item)
for path in ['AGENTS.md','agents/AGENT_HELPS_HUMANS.md']:
    record(app,path,'<APP_WORKTREE>','', 'full instruction read at entry')
for path in [p+'/AGENTS.md',r+'/PLAN.md',r+'/BRIEFS/A1_DESIGN.md',r+'/BRIEFS/COMMON.md']:
    record(coord,path,'<COORD>','dd677a972799237edfd8f389468aaa13c2ecf330','full instruction/brief/plan read')
for path in ['AGENTS.md','agents/AGENT_HELPS_HUMANS.md']:
    pinned=subprocess.run(['git','show','dd677a972799237edfd8f389468aaa13c2ecf330:'+path],cwd=coord,env=env,check=True,stdout=subprocess.PIPE).stdout
    local=(app/path).read_bytes()
    sources.append({'origin':'<COORD>','path':path,'revision':'dd677a972799237edfd8f389468aaa13c2ecf330','sha256':hashlib.sha256(pinned).hexdigest(),'bytes':len(pinned),'consultation':'read-only byte comparison against full entry instructions already read','matches_entry_bytes':pinned==local})
    assert pinned==local,path
for path in [
t+'/DESIGN_NUMERICS/DESIGN.md',t+'/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md',
t+'/ROOT_RULINGS_V1.md',t+'/ROOT_RULINGS_V2.md',t+'/DESIGN_STANDING/DESIGN.md',
t+'/AUDIT/A1_A2_REVIEW.md',t+'/AUDIT/REPORT.md',
r+'/I22/CHECKPOINT_0.md',r+'/I22/build_01/RETURN.md',r+'/I22/b_01/RETURN.md',r+'/I22/c_01/RETURN.md',
r+'/I22/c_01/COMMANDS.json',r+'/oracle_fresh/CHECKPOINT_0.md',r+'/oracle_fresh/SOURCE_READS.json',
r+'/oracle_fresh/addendum_01/RETURN.md',r+'/oracle_fresh/addendum_02_C17/CONFIRMATION.json']:
    record(a1,path,'<A1_WT>','a6b40d2d036acac556e28f28f5b482a4adb39333')
for path in [fk+'/src/structural/retained/adaptive.rs',fk+'/src/structural/retained/verify.rs',fk+'/src/structural/retained/wide/multi.rs']:
    record(a1,path,'<A1_WT>','d01ad98a754698631f927709d08284c272de85e8')
path=t+'/AUDIT/AUDIT_RESPONSE_SCOPE_HANDOFF_2026-09-30.md'
raw=subprocess.run(['git','show','520d7dfb790bcedabc03e92b9692884ce295be54:'+path],cwd=a1,env=env,check=True,stdout=subprocess.PIPE).stdout
sources.append({'origin':'<A1_WT> git object','path':path,'revision':'520d7dfb790bcedabc03e92b9692884ce295be54','sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'consultation':'full handoff read; no old conditional designs read'})
queries=[]
for args in [['rev-parse','HEAD'],['rev-parse','d01ad98a754698631f927709d08284c272de85e8:'+fk],['rev-parse','3bddc2b05f6106e969c7cf43373b230845c7cc66:'+fk],['diff','--exit-code','d01ad98a754698631f927709d08284c272de85e8','--',fk]]:
    run=subprocess.run(['git',*args],cwd=a1,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    queries.append({'argv':['GIT_OPTIONAL_LOCKS=0','git',*args],'cwd':'<A1_WT>','exit':run.returncode,'stdout':run.stdout.decode(),'stderr':run.stderr.decode()})
    assert run.returncode==0,args
checks=json.loads((out/'EXACT_CHECKS.json').read_text())
result={'schema':'a1-design-basis-v1','agent':'/root/a1_design','parent':'/root','mechanism':'collaboration.spawn_agent','roles_read':['Root AGENTS entry','HELPS_HUMANS full role'],'skills_or_workflows_activated':[],'source_records':sources,'git_queries':queries,'runtime':{'executable':'<VENV>/bin/python','sha256':hashlib.sha256(pathlib.Path(sys.executable).read_bytes()).hexdigest(),'version':sys.version,'system':platform.system(),'machine':platform.machine()},'exact_arithmetic_check_count':len(checks['checks']),'exact_arithmetic_status':checks['status'],'limitations':['initial filename/status discovery output was tool-truncated; no conclusion depends on completeness','some long document outputs were tool-truncated; targeted relevant sections were subsequently read','no source test, solver, native route, Git mutation or host-tool change']}
(out/'BASIS.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps({'status':'PASS','origins':len(sources),'git_queries':len(queries),'arithmetic_checks':len(checks['checks']),'source_unmodified':True}))
