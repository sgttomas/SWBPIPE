import pathlib,json,subprocess,hashlib,re
p=pathlib.Path(__file__).parent; repo=pathlib.Path('/private/tmp/piping-numerical-ci-20260924');head='a34c9d819d62b54b74d7f07edde87738b97b87cf';project='projects/chirality-piping/'
load=lambda f:json.loads(f.read_text());sha=lambda b:hashlib.sha256(b).hexdigest()
git=lambda *a:subprocess.check_output(['git','-C',str(repo),*a])
r=load(p/'run.json');j=next(j for j in load(p/'jobs.json')['jobs'] if j['name']=='Numerical cargo suite');nroot=p/'extracted/piping-numerical-36037700066-1';n=load(nroot/'numerical.json')
assert r['head_sha']==j['head_sha']==n['head']==head
assert j['conclusion']=='success' and n['status']=='success' and n['exit_code']==0 and n['plan_validated'] is True
assert next(s for s in j['steps'] if s['name']=='Fetch locked dependencies and run offline numerical tests')['conclusion']=='success'
tracked=git('ls-tree','-r','--name-only',head,project+'core',project+'validation/benchmarks').decode().splitlines();manifests=sorted(x[len(project):] for x in tracked if x.endswith('/Cargo.toml') and 'target' not in pathlib.PurePosixPath(x).parts)
assert len(manifests)==38 and manifests==n['manifests']
inputs={m for m in manifests}|{str(pathlib.PurePosixPath(m).with_name('Cargo.lock')) for m in manifests};assert set(n['input_sha256'])==inputs
for name in sorted(inputs):
 b=git('show',head+':'+project+name);assert sha(b)==n['input_sha256'][name]
 f=p/'candidate-inputs'/name;f.parent.mkdir(parents=True,exist_ok=True);f.write_bytes(b)
for name in ['tools/ci/numerical_ci.py','tools/release/check_release_readiness.py']:
 b=git('show',head+':'+project+name);(p/pathlib.Path(name).name).write_bytes(b)
fetches=[['cargo','fetch','--locked','--manifest-path',m] for m in manifests];tests=[['cargo','test','--offline','--manifest-path',m,'--locked'] for m in manifests]
expected=[['rustc','--version'],['cargo','--version']]+fetches+tests
assert [x['argv'] for x in n['commands']]==expected and n['planned_commands']==fetches+tests
assert len(n['commands'])==78 and all(x['exit_code']==0 for x in n['commands'])
for c in n['commands']:assert sha((nroot/c['output']).read_bytes())==c['output_sha256']
results=[]
for c in n['commands'][40:]:
 text=(nroot/c['output']).read_text();s=re.findall(r'test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured; (\d+) filtered out',text)
 assert s and all(int(x[1])==0 for x in s)
 results.append({'manifest':c['argv'][4],'exit_code':c['exit_code'],'summary_groups':len(s),'passed':sum(int(x[0]) for x in s),'ignored':sum(int(x[2]) for x in s),'filtered_out':sum(int(x[4]) for x in s)})
log=(p/f"job-{j['id']}.log").read_text();assert all('Running: '+' '.join(a) in log for a in expected)
report=dict(candidate=head,run_id=r['id'],run_url=r['html_url'],job_id=j['id'],job_url=j['html_url'],conclusion='success',discovered_manifests=38,locked_fetches=38,locked_offline_tests=38,all_command_exit_codes_zero=True,tool_version_commands=2,commands_total=78,input_hashes_verified_against_exact_git_objects=76,output_hashes_verified=78,missing_manifests=0,duplicate_manifests=0,unexpected_manifests=0,tool_versions={x['argv'][0]:(nroot/x['output']).read_text().strip() for x in n['commands'][:2]},environment=n['environment'],crate_results=results,test_summary_passed=sum(x['passed'] for x in results),test_summary_ignored=sum(x['ignored'] for x in results),boundary='PR887 main-based numerical CI repair exact candidate only; not M03 new solver branch qualification. Successful full hosted run; browser identities checked separately. No engineering acceptance/release claim.')
(p/'NUMERICAL_EXECUTION_CHECK.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({k:report[k] for k in ['discovered_manifests','locked_fetches','locked_offline_tests','commands_total','test_summary_passed','test_summary_ignored']}))
