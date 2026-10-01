from pathlib import Path
import subprocess,os,json,hashlib,sys
ROOT=Path(sys.argv[1]).resolve(); WT=ROOT.parent; ENV=dict(os.environ,GIT_OPTIONAL_LOCKS='0',PYTHONDONTWRITEBYTECODE='1')
T3=Path('projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3'); R=T3/'RESUME_2026-09-30'; REVIEW=WT/'numerics'/T3/'REVIEW/RESUME_RECORDS_RV27'; OUT=REVIEW/'backcheck_de9f8d83';PK=ROOT/R/'_run_records/aud_t3_04'
BASE='90b6bcbbf64b13975211038bf3f33bb87273e646'; HEAD='de9f8d83bdaab01b2bb988b78791f608adda61e6'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def git(*args):return subprocess.check_output(['git',*args],cwd=ROOT,env=ENV)
def seal(p):
 fails=[];lines=p.read_text().splitlines()
 for line in lines:
  h,r=line.split(None,1);r=r.strip()
  if sha(p.parent/r)!=h:fails.append(r)
 return {'entries':len(lines),'failures':fails}
paths=git('diff','--name-only',BASE,HEAD).decode().splitlines();allowed=[str(T3/'ROOT_RULINGS_V1.md'),'projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md']
result={'head':git('rev-parse','HEAD').decode().strip(),'expected_head':HEAD,'base':BASE,'status':git('status','--porcelain=v1').decode(),'changed_paths':paths,'scope_exact':sorted(paths)==sorted(allowed),'initial_review_sha256':sha(REVIEW/'REVIEW.md'),'initial_seal_sha256':sha(REVIEW/'SHA256SUMS'),'initial_seal':seal(REVIEW/'SHA256SUMS'),'preservation_seal_sha256':sha(PK/'SHA256SUMS'),'preservation_seal':seal(PK/'SHA256SUMS'),'source_evidence_unchanged':not bool(git('diff','--name-only',BASE,HEAD,'--',str(R),str(T3/'AUDIT'),'projects/chirality-piping/core','projects/chirality-piping/validation','tools','.agents','.github')),'current_file_hashes':{p:sha(ROOT/p) for p in allowed},'python':sys.version}
result['initial_review_expected_match']=result['initial_review_sha256']=='8bb6773bfb27f2e11a3b4f2a2dd1e740cc92fcb8f1a74fcc4ca0792a619658f6' and result['initial_seal_sha256']=='6d46d3170ca77f3015e0cad5ece55757d5070f566cf59a37aae5a07e91ff8715'
result['all_old_manifest_blobs_unchanged']=not bool(git('diff','--name-only',BASE,HEAD,'--','**/SHA256SUMS'))
(OUT/'evidence/CHECKS.json').write_text(json.dumps(result,indent=2)+'\n');(OUT/'evidence/delta.patch').write_bytes(git('diff','--no-ext-diff',BASE,HEAD));print(json.dumps(result,indent=2))
