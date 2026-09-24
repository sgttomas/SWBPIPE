from pathlib import Path
import subprocess,json,concurrent.futures,zipfile,hashlib,datetime
p=Path(__file__).parent
run_id=36037700066
prefix="repos/sgttomas/chirality/actions/"
def acquire(endpoint,path):
    with path.open("wb") as out:
        subprocess.run(["gh","api",prefix+endpoint],stdout=out,check=True)
    return {"endpoint":prefix+endpoint,"path":str(path.relative_to(p)),"sha256":hashlib.sha256(path.read_bytes()).hexdigest()}
requests=[(f"runs/{run_id}",p/"run.json"),(f"runs/{run_id}/jobs?per_page=100",p/"jobs.json"),(f"runs/{run_id}/artifacts?per_page=100",p/"artifacts.json")]
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as pool: records=list(pool.map(lambda x:acquire(*x),requests))
run=json.loads((p/"run.json").read_text());assert run["head_sha"]=="a34c9d819d62b54b74d7f07edde87738b97b87cf" and run["conclusion"]=="success"
arts=json.loads((p/"artifacts.json").read_text())["artifacts"]
jobs=json.loads((p/"jobs.json").read_text())["jobs"]
requests=[(f"artifacts/{a['id']}/zip",p/(a['name']+".zip")) for a in arts]+[(f"jobs/{j['id']}/logs",p/f"job-{j['id']}.log") for j in jobs]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool: records.extend(pool.map(lambda x:acquire(*x),requests))
for a in arts:
    target=p/"extracted"/a["name"];target.mkdir(parents=True,exist_ok=True)
    with zipfile.ZipFile(p/(a["name"]+".zip")) as z:
        assert all((target/name).resolve().is_relative_to(target.resolve()) for name in z.namelist())
        z.extractall(target)
(p/"ACQUISITION.json").write_text(json.dumps({"actor":"/root","role":"HELP_HUMAN","time":datetime.datetime.now(datetime.timezone.utc).isoformat(),"failed_descendant_resume":"hosted_evidence refused agent thread limit; ROOT performed collection","requests":records},indent=2)+"\n")
print(json.dumps({"artifacts":len(arts),"jobs":len(jobs),"run":run_id,"head":run["head_sha"]}))
