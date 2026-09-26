from pathlib import Path
import subprocess,json,re,hashlib,datetime,urllib.parse
RAW=Path(__file__).parent;OUT=RAW.parent;D=OUT.parent;ROOT=next(p for p in D.parents if (p/'agents/AGENT_TASK.md').is_file());PRIMARY=Path('/Users/ryan/.codex/worktrees/6614/chirality');QUAL=Path('/private/tmp/piping-joined-qualification-20260925');G=Path('projects/chirality-piping/execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md')
def sha(data):return hashlib.sha256(data).hexdigest()
def git(root,*args):return subprocess.check_output(['git',*args],cwd=root)
b=json.loads((D/'_run_records/BASIS.json').read_text());src=git(PRIMARY,'show',b['source_commit']+':'+str(G));candidate=(ROOT/G).read_bytes();assert sha(src)==b['source_graph_sha256']=='b0dda7eec9f9883a7385eda82281d8e59a3135391fff7ae566437f44490f89d6';assert sha(candidate)==b['candidate_graph_sha256']=='8051e6735c7075597c6bf89c294080554abfa3499accf30027cc27542ba9f57f'
assert len(b['link_updates'])==14 and not b['missing_links'];expected=src.decode()
for update in b['link_updates']:
 old='('+update['old']+')';new='('+update['new']+')';assert expected.count(old)==1;expected=expected.replace(old,new)
append=candidate.decode()[len(expected):];assert candidate.decode().startswith(expected) and append.startswith('\n## Connected candidate integration checkpoint\n\n')
assert '379 tests' in append and 'project self-check with no findings' in append and 'remain in progress' in append and 'no solver finding, native witness or programme scope is newly closed' in append
links=[]
for label,target in re.findall(r'\[([^\]]*)\]\(([^)]+)\)',candidate.decode()):
 if target.startswith('https://github.com/sgttomas/chirality/') and any(x in target for x in ['/blob/','/tree/']):
  style='blob' if '/blob/' in target else 'tree';rev,path=target.split('/'+style+'/',1)[1].split('/',1);path=urllib.parse.unquote(path.split('#',1)[0]);kind=git(ROOT,'cat-file','-t',rev+':'+path).decode().strip();assert kind==style
  content=git(ROOT,'show',rev+':'+path) if style=='blob' else git(ROOT,'ls-tree',rev+':'+path)
  links.append({'target':target,'kind':style,'exists_in_local_git':True,'content_or_listing_sha256':sha(content)})
 elif target.startswith(('http:','https:','#','mailto:')):links.append({'target':target,'kind':'existing_external_or_anchor','network_checked':False})
 else:
  p=(ROOT/G.parent/target.split('#',1)[0]).resolve();assert p.exists();links.append({'target':target,'kind':'local','resolved_origin':str(p),'is_file':p.is_file(),'sha256':sha(p.read_bytes()) if p.is_file() else None})
assert sum(x['kind']=='local' for x in links)==23 and sum(x['kind'] in ('blob','tree') for x in links)==23
# Source graph statuses and owner boundaries are copied, not widened.
# Remove only link targets before comparing every existing line: exact prefix equality above is stronger.
assert 'Original finding closure remains4of38 (M04/M09/M24/M35)' in candidate.decode();assert 'one undertaking CLOSE→RECORD→FINAL boundary' in candidate.decode();assert 'user-library/code-rule prohibition' in candidate.decode();assert 'General UI/C4/live work remains deferred' in candidate.decode()
assert git(ROOT,'diff','--name-only','HEAD','--','projects/chirality-piping/apps/desktop').strip()==b''
pract=(QUAL/'practitioner-repaired.log').read_bytes();selfcheck=(QUAL/'self-check-repaired.log').read_bytes();original=(QUAL/'practitioner.log').read_bytes()
assert b'379 passed' in pract and b'failed' not in pract.lower();assert b'Finding severities: none' in selfcheck and b"scope: ['projects/chirality-piping']" in selfcheck;assert b'1 failed, 378 passed' in original
for name,data in [('practitioner-repaired.log',pract),('self-check-repaired.log',selfcheck)]: (RAW/name).write_bytes(data)
import difflib
(RAW/'SOURCE_TO_CANDIDATE.patch').write_text(''.join(difflib.unified_diff(src.decode().splitlines(True),candidate.decode().splitlines(True),fromfile='primary9e8a55/WORK_GRAPH.md',tofile='engine/WORK_GRAPH.md')))
# Record the small exact transformation without invoking the author script.
(RAW/'APPENDED_CHECKPOINT.md').write_text(append)
record={'status':'CLEAR graph integration','checked_at_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'reviewer':'/root/numerical_resume/n05_admission','original_parent':'/root/numerical_resume','cross_manager_assignment':'/root','role':'TASK','delegation':'none','source_commit':b['source_commit'],'engine_head':git(ROOT,'rev-parse','HEAD').decode().strip(),'source_graph_sha256':sha(src),'candidate_graph_sha256':sha(candidate),'exact14links_plus_append_reconstruction':True,'links':links,'desktop_source_diff_empty':True,'check_evidence':[{'origin':str(QUAL/name),'sha256':sha((QUAL/name).read_bytes())} for name in ['practitioner.log','practitioner-repaired.log','self-check-repaired.log','integrate_graph.py']],'author_basis_sha256':sha((D/'_run_records/BASIS.json').read_bytes()),'claims':'Project selfcheck0 findings and practitioner379pass supported; earlier378/379 paragraph is the preserved initial state explicitly superseded by repair now passes. Current rows and latestcheckpoint preserve4of38, onecloseout and ownership/library/physics limits.','scope_limit':'No network target query, source import, relocation backcheck, build/native/test or actualhead qualification rerun.'}
(RAW/'CHECKED_BASIS.json').write_text(json.dumps(record,indent=2)+'\n');assert (ROOT/G).read_bytes()==candidate;print(json.dumps({'status':'CLEAR','graph_sha256':sha(candidate),'links':{'local':23,'pinned_git':23,'existing_other':6}},indent=2))
