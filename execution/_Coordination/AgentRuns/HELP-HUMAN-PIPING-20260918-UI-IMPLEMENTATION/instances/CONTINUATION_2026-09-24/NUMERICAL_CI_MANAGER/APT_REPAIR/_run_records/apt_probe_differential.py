from pathlib import Path
import urllib.request,urllib.error,hashlib,json,time,re
out=Path('/private/tmp/piping-numerical-ci-manager-20260924/apt-diagnostics');rows=[]
def get(name,url,headers=None):
 row={'name':name,'url':url,'request_headers':headers or {},'at':time.strftime('%Y-%m-%dT%H:%M:%SZ',time.gmtime())};data=b''
 try:
  with urllib.request.urlopen(urllib.request.Request(url,headers=headers or {}),timeout=15) as r:
   data=r.read(4000000);row.update(status=r.status,response_headers=dict(r.headers),length=len(data),sha256=hashlib.sha256(data).hexdigest());(out/(name+'.body')).write_bytes(data)
 except Exception as e:row['error']=str(e)
 rows.append(row);print(name,row.get('status'),row.get('length'),row.get('sha256'),row.get('error'),flush=True);(out/'http-differential.json').write_text(json.dumps(rows,indent=2)+'\n');return data
no_cache={'Cache-Control':'no-cache','Pragma':'no-cache'}
for name,base,path in [('security','https://security.ubuntu.com/ubuntu/dists/noble-security/','main/dep11/Components-amd64.yml.xz'),('updates','https://archive.ubuntu.com/ubuntu/dists/noble-updates/','universe/binary-amd64/Packages.xz')]:
 body=get(name+'-fresh-inrelease',base+'InRelease',no_cache).decode(errors='replace')
 if not body:continue
 lines=re.findall(r'^ ([a-f0-9]{64})\s+(\d+)\s+'+re.escape(path)+r'$',body,re.M);assert len(lines)==1,lines
 sha,size=lines[0];url=base+path.rsplit('/',1)[0]+'/by-hash/SHA256/'+sha
 get(name+'-fresh-byhash',url,no_cache);get(name+'-stable-nocache',base+path,no_cache)
get('apt-2.8.3-source','https://salsa.debian.org/apt-team/apt/-/raw/2.8.3/apt-pkg/acquire-item.cc')
get('apt-repository-format','https://wiki.debian.org/DebianRepository/Format')
get('noble-transport-http','https://manpages.ubuntu.com/manpages/noble/man1/apt-transport-http.1.html')
get('noble-transport-https','https://manpages.ubuntu.com/manpages/noble/man1/apt-transport-https.1.html')
