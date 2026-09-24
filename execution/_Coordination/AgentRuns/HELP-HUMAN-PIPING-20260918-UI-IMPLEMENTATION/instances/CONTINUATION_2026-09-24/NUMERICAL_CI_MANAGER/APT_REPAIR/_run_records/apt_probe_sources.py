from pathlib import Path
import urllib.request,hashlib,json,time
out=Path('/private/tmp/piping-numerical-ci-manager-20260924/apt-diagnostics');rows=[]
for name,url in [('ubuntu-apt-by-hash','https://wiki.ubuntu.com/AptByHash'),('runner-image-apt','https://raw.githubusercontent.com/actions/runner-images/ubuntu24%2F20260920.314/images/ubuntu/scripts/build/configure-apt.sh')]:
 row={'name':name,'url':url,'at':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())}
 try:
  with urllib.request.urlopen(url,timeout=15) as r:
   b=r.read(500000);row.update(status=r.status,headers=dict(r.headers),length=len(b),sha256=hashlib.sha256(b).hexdigest());(out/(name+'.body')).write_bytes(b)
 except Exception as e:row['error']=str(e)
 rows.append(row);print(row,flush=True)
(out/'http-primary-sources.json').write_text(json.dumps(rows,indent=2)+'\n')
