import pathlib,json,hashlib,subprocess,os,re,datetime
base=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c");r=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30");q=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/scratch/i28-ordinary-artifacts-02");rr=pathlib.Path("/Users/ryan/dev/chirality/.claude/worktrees/swbpipe-control-layer-8a41be/.claude/t3/k6c/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/NUMERICAL_INTEGRITY_T3/RESUME_2026-09-30/I28/ordinary_artifacts_02")/'_run_records'
sha=lambda p:hashlib.sha256(pathlib.Path(p).read_bytes()).hexdigest()
def meta(p): return {'path':str(p),'sha256':sha(p),'bytes':pathlib.Path(p).stat().st_size}
def dump(n,d):(rr/n).write_text(json.dumps(d,indent=2)+'\n')
rust=pathlib.Path('/Users/ryan/.rustup/toolchains/1.97.1-aarch64-apple-darwin')
docs={}
for f in ['I21/source_12/LIBRARY_SOURCE.json','I21/source_13/LIBRARY_SOURCE.json']:
 for x in json.loads((r/f).read_text())['pages']:
  docs[x['origin'].replace('<RUST>',str(rust))]={'expected':x['html_sha256'],'record':f}
for x in json.loads((r/'I23/vr_nodes_source_16/_run_records/BINDING.json').read_text())['source02_installed_documentation_identity']:
 docs[x['origin'].replace('<TOOLCHAIN>',str(rust))]={'expected':x['html_sha256'],'record':'I23/vr_nodes_source_16/_run_records/BINDING.json'}
for x in json.loads((r/'I23/wrapped_error_artifacts_18/_run_records/SOURCE_BINDINGS.json').read_text())['bindings']:
 docs[x['original_path']]={'expected':x.get('html_sha256',x.get('sha256')),'record':'I23/wrapped_error_artifacts_18/_run_records/SOURCE_BINDINGS.json'}
docrows=[]
for path,x in docs.items():
 m=meta(path);m.update(x,match=m['sha256']==x['expected']);docrows.append(m)
assert all(x['match'] for x in docrows)
pkg=json.loads((r/'I23/serde_binding_07/PACKAGE_BINDING.json').read_text())
package_rows=[]
for p in pkg['packages']:
 root=pathlib.Path(p['installed_root'].replace('<CARGO_HOME>','/Users/ryan/.cargo'))
 archive=pathlib.Path(p['archive']['path'].replace('<CARGO_HOME>','/Users/ryan/.cargo'))
 files=[]
 for item in p['member_comparisons']:
  m=meta(root/item['relative_path']);m['matches_prior_authenticated_member']=m['sha256']==item['archive_sha256'];files.append(m)
 assert all(x['matches_prior_authenticated_member'] for x in files)
 am=meta(archive);assert am['sha256']==p['lock_checksum']
 package_rows.append({'name':p['name'],'version':p['version'],'archive':am,'lock_checksum':p['lock_checksum'],'files':files})
dump('LIBRARY_SOURCE_CORRESPONDENCE.json',{'scope':'Recheck exact existing source/package identities only; no new theorem, decode or archive-reader work.','installed_source_pages':docrows,'authenticated_packages':package_rows})
maps=['I21/layout_04/LABEL_TYPE_MAP.json','I21/layout_04/H_LABEL_TYPE_MAP.json']
old_cache={};commands=[];rows=[]
def oldfile(path,rev,expected):
 if (path,rev) not in old_cache:
  argv=['git','show',rev+':'+path]
  z=subprocess.run(argv,cwd=base,env=dict(os.environ,GIT_OPTIONAL_LOCKS='0'),capture_output=True)
  commands.append({'argv':argv,'cwd':str(base),'environment_set':{'GIT_OPTIONAL_LOCKS':'0'},'exit':z.returncode,'stdout_sha256':hashlib.sha256(z.stdout).hexdigest(),'stderr':z.stderr.decode()})
  assert z.returncode==0 and hashlib.sha256(z.stdout).hexdigest()==expected
  old_cache[path,rev]=z.stdout.decode()
 return old_cache[path,rev]
def declaration(s,anchor):
 start=s.index(anchor);brace=s.index('{',start);depth=0
 for i in range(brace,len(s)):
  if s[i]=='{':depth+=1
  if s[i]=='}':
   depth-=1
   if depth==0:return s[start:i+1]
 raise AssertionError(anchor)
for f in maps:
 d=json.loads((r/f).read_text())
 for x in d['rows']:
  p=q/'source'/x['origin'];row={'label':x['label'],'type':x['type'],'origin_record':f,'source_origin':x['origin']}
  if p.is_file() and x.get('source_sha256'):
   row['current_file']=meta(p);row['prior_file_sha256']=x['source_sha256']
   if row['current_file']['sha256']==x['source_sha256']:row['correspondence']='whole originating source file byte-identical'
   else:
    old=oldfile(x['origin'],d['basis'],x['source_sha256']);new=p.read_text();anchor=x['anchor']
    if anchor.startswith(('pub struct','pub enum','struct','enum')):
     a,b=declaration(old,anchor),declaration(new,anchor);assert a==b
     row['declaration']=a;row['declaration_sha256']=hashlib.sha256(a.encode()).hexdigest();row['correspondence']='exact nominal declaration unchanged in changed containing file; attributes reviewed separately in source diff'
    else:
     assert anchor in old and anchor in new
     row['anchor']=anchor;row['correspondence']='exact anonymous tuple/source-type anchor unchanged'
  else:row['correspondence']='standard/serde expression; installed compiler/sysroot/package/features bound separately, no fresh layout04 run'
  rows.append(row)
assert len(rows)==159
dump('TYPE_SOURCE_CORRESPONDENCE.json',{'rows':rows,'old_source_reads':commands,'status':'Source/type identity evidence, not159 fresh size observations. No layout04/08 overlay built.'})
dump('SOURCE_TYPE_DIFFS.json',commands)
print(json.dumps({'source_pages':len(docrows),'package_member_files':sum(len(p['files']) for p in package_rows),'types':len(rows),'whole_source_matches':sum(x['correspondence'].startswith('whole') for x in rows),'declaration_or_tuple_matches':sum(x['correspondence'].startswith('exact') for x in rows),'standard_expression_rows':sum(x['correspondence'].startswith('standard') for x in rows)}))

