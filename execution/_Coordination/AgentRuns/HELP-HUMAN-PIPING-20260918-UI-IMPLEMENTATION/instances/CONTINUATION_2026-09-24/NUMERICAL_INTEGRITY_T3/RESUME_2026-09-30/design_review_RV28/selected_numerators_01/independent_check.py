"""RV28 source-only numerator review: integers/Fractions, no model or solver run.
Usage: python -B independent_check.py K6C NUM OUT APP
Reads immutable source and existing fixture/descriptor bytes. Generates only
proof evidence, not models, reference outcomes, maintained tools or expectations.
"""
from fractions import Fraction as F
from pathlib import Path
import hashlib,json,os,platform,subprocess,sys
K6C,NUM,OUT,APP=map(Path,sys.argv[1:])
REV='40129a225d73860ac2a53da9a2fa73869df668f3'
BRIEF_REV='5293da04d3987e1fa9dd9b6d54ff156a1bf0c39c'
T='projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'
R=T+'/RESUME_2026-09-30'
FK='projects/chirality-piping/core/solver/frame_kernel'
VR='projects/chirality-piping/validation/benchmarks/numerical_robustness'
env=dict(os.environ,GIT_OPTIONAL_LOCKS='0')
records=[];commands=[];checks=[]
def sha(b):return hashlib.sha256(b).hexdigest()
def two(k):return F(2)**k
def check(name,condition):
    assert condition,name
    checks.append(name)
def git(root,rev,path):
    p=subprocess.run(['git','show',rev+':'+path],cwd=root,env=env,capture_output=True)
    commands.append({'cwd':'<NUM>' if root==NUM else '<K6C>','argv':['git','show',rev+':'+path],'GIT_OPTIONAL_LOCKS':'0','exit':p.returncode,'stdout_sha256':sha(p.stdout),'stderr':p.stderr.decode()})
    assert p.returncode==0,p.stderr
    records.append({'origin':'Git object','revision':rev,'path':path,'sha256':sha(p.stdout),'bytes':len(p.stdout),'current_checkout_matches': (root/path).exists() and p.stdout==(root/path).read_bytes()})
    return p.stdout
def local(path):
    b=(K6C/path).read_bytes();records.append({'origin':'<K6C>','path':path,'sha256':sha(b),'bytes':len(b)});return b
brief=git(NUM,BRIEF_REV,R+'/BRIEFS/RV28_K6C_SELECTED_NUMERATORS.md')
for path in ['AGENTS.md','agents/AGENT_TASK.md','projects/chirality-piping/AGENTS.md','.agents/skills/software-code-review/SKILL.md']:
    b=(APP/path).read_bytes()
    records.append({'origin':'<APP_WORKTREE> active instruction/skill','path':path,'sha256':sha(b),'bytes':len(b)})
for path in [FK+'/src/structural/retained/'+s for s in ['adaptive.rs','assemble.rs','recover.rs','verify.rs','source.rs','wide/multi.rs']]+[VR+'/src/'+s for s in ['lane.rs','floor.rs','cases.rs']]+[T+'/DESIGN_NUMERICS/REV_5A3_CANDIDATE/D1_REV_5A3_SSTAR_RESOLUTION_R7.md']:
    git(K6C,REV,path)
seals=[]
for rel,expected in [(R+'/metric_design_05_selected_numerators','8de7420579e705b7930c8e22cb0322616f718e0379dcd907937f2b077b91982a'),(R+'/I21/source_14',None),(R+'/source_review_RV30/finite_callers_12',None)]:
    seal=local(rel+'/SHA256SUMS')
    if expected:check('designer seal hash',sha(seal)==expected)
    count=0
    for line in seal.decode().splitlines():
        digest,path=line.split(maxsplit=1);p=K6C/rel/path.lstrip('*')
        if not p.exists():p=K6C/path.lstrip('*')
        check('sealed file '+str(p.relative_to(K6C)),sha(p.read_bytes())==digest);count+=1
    seals.append({'path':rel+'/SHA256SUMS','sha256':sha(seal),'entries':count})
proposal=local(R+'/metric_design_05_selected_numerators/DERIVATION.md')
check('derivation hash',sha(proposal)=='a54c0e88f09c59a6a728a8067805b6894e574ddd4ba69379f939059ceb48b069')
proposed=json.loads(local(R+'/metric_design_05_selected_numerators/DERIVED_BOUNDS.json'))
desc=json.loads(local(R+'/I21/source_14/FIXED_INEQUALITIES_ALL.json'))
local(R+'/source_review_RV30/finite_callers_12/RETURN.md')
local(R+'/source_review_RV30/finite_callers_12/FINDINGS.json')
def decode(s):
    b=int(s,16);sign=-1 if b>>63 else 1;e=(b>>52)&2047;f=b&((1<<52)-1)
    assert e!=2047
    return sign*F(f if e==0 else (1<<52)+f)*two(-1074 if e==0 else e-1075)
def lowlog(x):
    assert x>0
    e=x.numerator.bit_length()-x.denominator.bit_length()
    return e-1 if x<two(e) else e
def highlog(x):
    e=lowlog(x);return e if x==two(e) else e+1
def cross(a,b):return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
fixtures={}
for file in ['rf_weak.jsonl','rf_range.jsonl']:
    path=VR+'/cases/'+file;b=git(K6C,REV,path)
    for line_no,line in enumerate(b.decode().splitlines(),1):
        if line.strip():
            c=json.loads(line);fixtures[c['id']]=(c,path,line_no)
open_cases={c['id']:c for c in desc['cases'] if c['observed_division_gaps']}
check('open case roster equals proposal',set(open_cases)=={c['id'] for c in proposed['cases']})
check('open case count',len(open_cases)==18)
proposed_cases={c['id']:c for c in proposed['cases']}
out=[];total_rows=0
for name,dc in open_cases.items():
    c,file,line=fixtures[name];m=c['model'];pc=proposed_cases[name]
    check(name+' SI source units',c['units']=='m')
    xyz=[[decode(s) for s in n[1:]] for n in m['nodes']]
    check(name+' node/member bound',len(xyz)<=11 and len(m['members'])<=10)
    check(name+' no omitted springs',m['omitted_springs']==[])
    check(name+' known primitive shape',set(m)=={'nodes','members','springs','constraints','loads','stations','omitted_springs'})
    parents=list(range(len(xyz)))
    def root(i):
        while parents[i]!=i:i=parents[i]
        return i
    members={};terms=[];edges=[]
    for row in m['members']:
        nm,ident,i,j,*_=row
        check(name+'/'+nm+' unique ids and names',nm not in members and all(v['id']!=ident for v in members.values()))
        check(name+'/'+nm+' acyclic',root(i)!=root(j))
        parents[root(j)]=root(i)
        delta=[xyz[j][a]-xyz[i][a] for a in range(3)];ds=max(map(abs,delta));d=lowlog(ds)
        yr=list(map(decode,row[10:13]));check(name+'/'+nm+' valid exact frame',any(cross(delta,yr)))
        E,G,A,Iy,Iz,J=map(decode,row[4:10]);check(name+'/'+nm+' positive properties',all(z>0 for z in [E,G,A,Iy,Iz,J]))
        Lhi=two(d+2);check(name+'/'+nm+' exact length enclosure',two(2*d)<=sum(z*z for z in delta)<=Lhi*Lhi)
        axis=(sum(z!=0 for z in delta)==1 and ds==two(d))
        # Bounds suffice for all source binary64 norm operations: dominant square
        # normal and the loose squared-sum upper normal; no norm is evaluated.
        check(name+'/'+nm+' normal norm bounding powers',-1022<=2*d and 2*d+4<=1023)
        members[nm]={'id':ident,'i':i,'j':j,'d':d,'axis':axis,'E':E,'G':G,'A':A,'Iy':Iy,'Iz':Iz,'J':J,'Lhi':Lhi}
        edges.append((i,j,Lhi))
    check(name+' connected rooted tree',len({root(i) for i in range(len(xyz))})==1 and len(members)==len(xyz)-1)
    D=highlog(sum(v['Lhi'] for v in members.values()));ell=two(D)
    constraints={tuple(x) for x in m['constraints']}
    check(name+' unique valid constraints',len(constraints)==len(m['constraints']) and all(0<=i<len(xyz) and 0<=a<6 for i,a in constraints))
    check(name+' fixed root translations',all((0,a) in constraints for a in range(3)))
    spring_terms=[]
    for s in m['springs']:
        check(name+' positive spring '+s[0],decode(s[6])>0)
        check(name+' spring endpoint '+s[0],0<=s[2]<len(xyz))
    for a in range(3,6):
        if (0,a) in constraints:continue
        choices=[s for s in m['springs'] if s[2]==0 and s[3]=='r' and s[4]==a and decode(s[6])>0]
        check(name+' grounded root rotation '+str(a),bool(choices))
        # Take first actual spring only. Additional positive springs can be released.
        spring_terms.append(1/decode(choices[0][6]))
    C=sum(v['Lhi']*(1/(v['E']*v['A']*ell*ell)+1/(v['G']*v['J'])+1/(v['E']*v['Iy'])+1/(v['E']*v['Iz'])) for v in members.values())+sum(spring_terms)
    load_l1=F(0);load_terms=[]
    for n,a,bits,src in m['loads']:
        check(name+' valid load '+src,0<=n<len(xyz) and 0<=a<6)
        term=abs(decode(bits))*(ell if a<3 else 1);load_l1+=term;load_terms.append({'source':src,'node':n,'component':a,'normalized_magnitude':str(term)})
    truth=C*load_l1;u=highlog(truth)
    body_span=max(max(z[a] for z in xyz)-min(z[a] for z in xyz) for a in range(3));db=lowlog(body_span)
    check(name+' normal body norm bounding powers',-1022<=2*db and 2*db+4<=1023)
    check(name+' operational extent bounded by proof scale',db+2<=D)
    a=max(2,2+D-db);alpha=two(a-64)
    check(name+' contraction',a<=20 and alpha<1/2)
    check(name+' retained candidate factor',(1+alpha)/(1-alpha)<4)
    check(name+' verification magnitude two-round factor',F(3)*(1+two(-256))**3<4)
    check(name+' compliance equality to proposal',C==F(pc['exact_compliance_upper']))
    check(name+' load equality to proposal',load_l1==F(pc['exact_normalized_load_l1_upper']))
    check(name+' truth enclosure equality',truth==F(pc['exact_truth_normalized_displacement_upper']))
    check(name+' derived exponents equality',(D,db,a,u)==(pc['length_scale_pow2'],pc['body_span_lower_pow2'],pc['coupling_bound_pow2'],pc['exact_truth_normalized_displacement_upper_pow2']))
    check(name+' source file/line identity',(file,line)==(pc['file'],pc['line']))
    row_map={(r['row_index'],r['key']):r for r in pc['rows']}
    gaps={(r['row_index'],r['key']):r for r in dc['observed_division_gaps']}
    independent_gaps={};row_results=[]
    descriptor_members={x['name']:g['facts'] for g in dc['member_groups'] for x in g['members']}
    for row_index,row in enumerate(c['rows']):
        key=row[0];parts=key.split('.')
        if parts[0] not in ['tw','ext']:continue
        check(name+'/'+key+' key mapping',len(parts)==2 and parts[1] in members)
        v=members[parts[1]];twist=parts[0]=='tw';prop_a=v['G'] if twist else v['E'];prop_b=v['J'] if twist else v['A'];ea,eb=lowlog(prop_a),lowlog(prop_b)
        d=v['d'];lower=ea+eb-d-(0 if v['axis'] else 2);upper=ea+eb-d+2
        # Independently check both fast/fallback coefficient envelope premises.
        check(name+'/'+key+' positive normal factors',-1022<=ea<=1023 and -1022<=eb<=1023)
        check(name+'/'+key+' scale2 path endpoints normal',-1022<=-ea<=1023)
        check(name+'/'+key+' normalized product envelope',1<=prop_a*(prop_b*two(-ea-eb))<4)
        check(name+'/'+key+' normalized quotient bounding powers normal',-1022<=(-d if v['axis'] else -d-2) and 2-d<=1023)
        check(name+'/'+key+' final denominator normal bounds',-1022<=lower<=upper<=1023)
        df=descriptor_members[parts[1]]['kt' if twist else 'ka']
        check(name+'/'+key+' source14 coefficient equality',(lower,upper)==(df['lower_pow2'],df['upper_pow2']))
        if lower>=0:continue
        ident=(row_index,key);independent_gaps[ident]=True
        check(name+'/'+key+' explicit open row',ident in gaps and ident in row_map)
        row_bound=row_map[ident];gb=gaps[ident]
        K=u+2+(0 if twist else D);N=upper+K+4;Q=N-lower
        check(name+'/'+key+' all exponent identities',(lower,upper,N,Q)==(row_bound['lane_coefficient_lower_pow2'],row_bound['retained_coefficient_upper_pow2'],row_bound['retained_and_published_numerator_absolute_upper_pow2'],row_bound['quotient_absolute_upper_pow2']))
        check(name+'/'+key+' component identity',row_bound['j_end_component']==('RX' if twist else 'UX') and row_bound['member']==parts[1] and gb['member']==parts[1])
        check(name+'/'+key+' finite bounding publication',-1022<=N<=1023)
        check(name+'/'+key+' below actual overflow threshold',two(Q)<two(1024)-two(970) and Q<=1023)
        row_results.append({'index':row_index,'key':key,'member_id':v['id'],'component':'RX' if twist else 'UX','lower_k_exponent':lower,'retained_k_upper_exponent':upper,'candidate_component_upper_exponent':K,'numerator_upper_exponent':N,'quotient_upper_exponent':Q})
    check(name+' exact complete row roster',set(independent_gaps)==set(gaps)==set(row_map))
    total_rows+=len(row_results)
    out.append({'id':name,'fixture':file,'line':line,'nodes':len(xyz),'members':len(members),'D':D,'d_body':db,'a':a,'compliance':str(C),'load_l1':str(load_l1),'truth_upper':str(truth),'u':u,'load_terms':load_terms,'rows':row_results})
check('138 rows',total_rows==138)
worst=max(r['quotient_upper_exponent'] for c in out for r in c['rows'])
check('full calculated worst matches proposal',worst==proposed['summary']['worst_quotient_upper_pow2'])
# Symbolic bending block inverse, no numerical physical-model solve.
check('bending inverse endpoints',max((a*a-a*b+b*b)/F(3) for a in [-1,1] for b in [-1,1])==1)
OUT.mkdir(parents=True,exist_ok=True)
(OUT/'INDEPENDENT_BOUNDS.json').write_text(json.dumps({'status':'PASS','cases':out,'summary':{'cases':len(out),'rows':total_rows,'worst_quotient_exponent':worst},'method':'Exact integer bit decode and Fraction compliance/exponent proof; no solver, source emulation, norms, reference values or designer-script import.'},indent=2)+'\n')
(OUT/'CHECKS.json').write_text(json.dumps({'status':'PASS','count':len(checks),'checks':checks},indent=2)+'\n')
(OUT/'BASIS.json').write_text(json.dumps({'agent':'/root/rv28_a1_design','parent':'/root','role':'TASK','mechanism':'collaboration.followup_task','actual_start_utc':'2026-10-01 17:27:21 UTC','deadline_utc':'2026-10-01 17:52:21 UTC','source_records':records,'seals':seals,'python_version':sys.version,'system':platform.system(),'machine':platform.machine(),'executable_sha256':sha(Path(sys.executable).read_bytes())},indent=2)+'\n')
(OUT/'COMMANDS.json').write_text(json.dumps(commands,indent=2)+'\n')
print(json.dumps({'status':'PASS','checks':len(checks),'cases':len(out),'rows':total_rows,'worst_quotient_exponent':worst,'source_records':len(records),'seals':len(seals)}))
