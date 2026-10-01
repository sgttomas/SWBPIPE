from pathlib import Path
import subprocess,os,json,hashlib,sys
ROOT=Path(sys.argv[1]).resolve();WT=ROOT.parent;ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0',PYTHONDONTWRITEBYTECODE='1')
T3=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3');RV=WT/'numerics'/T3/'REVIEW/RESUME_RECORDS_RV27';OUT=RV/'mergecheck_ea2aaee7'; PK=ROOT/T3/'RESUME_2026-09-30/_run_records/aud_t3_04'
BASE='d01ad98a754698631f927709d08284c272de85e8';PRIOR='de9f8d83bdaab01b2bb988b78791f608adda61e6';MAIN='a38617d08bfbff6ea0a15b1cb129a16cbc82b2fc';HEAD='ea2aaee702624cd612eee23a7e1969fd3c670d3f'
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,env=ENV)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def seal(p):
 fail=[];lines=p.read_text().splitlines()
 for line in lines:
  h,r=line.split(None,1);r=r.strip()
  if sha(p.parent/r)!=h:fail.append(r)
 return {'sha256':sha(p),'entries':len(lines),'failures':fail}
def tree(ref):
 entries={}
 for b in git('ls-tree','-r','-z','--full-tree',ref).split(b'\0'):
  if b:
   metadata,p=b.split(b'\t',1);entries[p]=metadata
 return entries
def difference(a,b):return sorted(k for k in a.keys()|b.keys() if a.get(k)!=b.get(k))
def patchhash(a,b):
 p=subprocess.Popen(['git','diff','--binary','--no-ext-diff','--no-renames',a,b],cwd=ROOT,env=ENV,stdout=subprocess.PIPE);h=hashlib.sha256();n=0
 for block in iter(lambda:p.stdout.read(1048576),b''):h.update(block);n+=len(block)
 assert p.wait()==0
 return {'bytes':n,'sha256':h.hexdigest()}
b,o,t,m=[tree(x) for x in [BASE,PRIOR,MAIN,HEAD]];ours=difference(b,o);theirs=difference(b,t);expected=dict(b)
for source,paths in [(o,ours),(t,theirs)]:
 for path in paths:
  if path in source:expected[path]=source[path]
  else:expected.pop(path,None)
oldraw=git('diff','--raw','--abbrev=40','--no-renames',BASE,PRIOR);newraw=git('diff','--raw','--abbrev=40','--no-renames',MAIN,HEAD)
(OUT/'evidence/final_pr_raw_diff.txt').write_bytes(newraw);(OUT/'evidence/upstream_changed_paths.txt').write_text('\n'.join(p.decode() for p in theirs)+'\n')
result={'head':git('rev-parse','HEAD').decode().strip(),'expected_head':HEAD,'parents':git('show','-s','--format=%P',HEAD).decode().strip().split(),'expected_parents':[PRIOR,MAIN],'status':git('status','--porcelain=v1').decode(),'merge_base':git('merge-base',PRIOR,MAIN).decode().strip(),'reviewed_change_paths':len(ours),'upstream_change_paths':len(theirs),'overlapping_paths':[x.decode() for x in sorted(set(ours)&set(theirs))],'upstream_outside_app_v4_execution':[x.decode() for x in theirs if not x.startswith(b'projects/chirality-app-v4/execution/')],'full_tree_union_mismatches':[x.decode() for x in difference(expected,m)],'combined_diff':git('diff-tree','--cc','--no-commit-id','--name-status','-r',HEAD).decode(),'complete_pr_raw_diff_identical':oldraw==newraw,'old_pr_patch':patchhash(BASE,PRIOR),'new_pr_patch':patchhash(MAIN,HEAD),'relevant_identities':{},'seals':{'initial_review':seal(RV/'SHA256SUMS'),'prior_backcheck':seal(RV/'backcheck_de9f8d83/SHA256SUMS'),'preservation':seal(PK/'SHA256SUMS')},'python':sys.version}
for path in ['projects/chirality-piping','tools','.github','AGENTS.md','agents','.agents/skills','workflows']:
 result['relevant_identities'][path]={ref:git('rev-parse',ref+':'+path).decode().strip() for ref in [PRIOR,HEAD]}
(OUT/'evidence/CHECKS.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result,indent=2))
