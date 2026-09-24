from pathlib import Path
import urllib.request,hashlib,json,time
out=Path('/private/tmp/piping-numerical-ci-manager-20260924/apt-diagnostics');out.mkdir(parents=True,exist_ok=True)
urls={'noble-sources':'https://manpages.ubuntu.com/manpages/noble/man5/sources.list.5.html','noble-aptconf':'https://manpages.ubuntu.com/manpages/noble/man5/apt.conf.5.html','apt-by-hash':'https://wiki.debian.org/AptByHash','security-inrelease':'https://security.ubuntu.com/ubuntu/dists/noble-security/InRelease','updates-inrelease':'https://archive.ubuntu.com/ubuntu/dists/noble-updates/InRelease','security-observed-hash':'https://security.ubuntu.com/ubuntu/dists/noble-security/main/dep11/by-hash/SHA256/97b1c360f55cc33b9bc5b51019c29be4d060e64c6d4004dcea0e3b2bad855ef5','updates-observed-hash':'https://archive.ubuntu.com/ubuntu/dists/noble-updates/universe/binary-amd64/by-hash/SHA256/5914f242dd289eb47e4e592db25970499a713b7624661ec76f67a8ea5f896ff5'}
rows=[]
for name,url in urls.items():
 row={'name':name,'url':url,'observed_at_utc':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())}
 try:
  with urllib.request.urlopen(url,timeout=15) as r:
   data=r.read(4000000);row.update(status=r.status,headers=dict(r.headers),length=len(data),sha256=hashlib.sha256(data).hexdigest());(out/(name+'.body')).write_bytes(data)
 except Exception as e:row['error']=str(e)
 rows.append(row);print(name,row.get('status'),row.get('length'),row.get('sha256'),row.get('error'),flush=True)
(out/'http-observations-unsandboxed.json').write_text(json.dumps(rows,indent=2)+'\n')
