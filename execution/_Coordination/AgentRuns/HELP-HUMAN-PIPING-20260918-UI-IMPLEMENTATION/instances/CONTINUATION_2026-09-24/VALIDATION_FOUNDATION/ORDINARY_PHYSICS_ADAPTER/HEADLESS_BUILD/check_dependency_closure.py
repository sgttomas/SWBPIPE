"""Post-build source-input and cached registry checksum crosscheck; no solve."""
from pathlib import Path
import json,hashlib,subprocess,shlex,tarfile,tomllib
E=Path(__file__).resolve().parent
S={x['path']:x for x in json.loads((E/'SOURCE_BEFORE.json').read_text())}
line=(E/'runner.dep-info.txt').read_text().splitlines()[0]
inputs=shlex.split(line.split(': ',1)[1])
# Cargo dep-info retains literal ../ include paths; source binding is resolved.
lexical=[x for x in inputs if x not in S]
missing=[x for x in inputs if str(Path(x).resolve()) not in S]
assert not missing,missing
meta=json.loads((E/'metadata.json').read_text()); records=[]
lock=tomllib.loads(Path('/private/tmp/piping-engine-integration-20260925/projects/chirality-piping/core/runner/headless/Cargo.lock').read_text())
lock_checks={(row['name'],row['version']):row['checksum'] for row in lock['package'] if 'checksum' in row}
for pkg in meta['packages']:
 if pkg['source']:
  p=Path(pkg['manifest_path']).parent
  archive=p.parents[2]/'cache'/p.parent.name/(p.name+'.crate')
  archive_sha=hashlib.sha256(archive.read_bytes()).hexdigest()
  assert archive_sha==lock_checks[(pkg['name'],pkg['version'])]
  fails=[]; count=0
  with tarfile.open(archive,'r:gz') as packed:
   for member in packed.getmembers():
    if not member.isfile(): continue
    relative=Path(member.name).relative_to(p.name)
    current=p/relative
    if not current.is_file() or hashlib.sha256(current.read_bytes()).digest()!=hashlib.sha256(packed.extractfile(member).read()).digest(): fails.append(str(relative))
    count+=1
  assert not fails,(pkg['id'],fails)
  records.append({'package_id':pkg['id'],'package_checksum':archive_sha,'archive_path':str(archive),'checked_files':count,'matches':True})
toolchain={}
for name in ('cargo','rustc'):
 p=Path(subprocess.check_output(['rustup','which',name],text=True).strip())
 toolchain[name]={'path':str(p),'sha256':hashlib.sha256(p.read_bytes()).hexdigest()}
record={'dep_info_dependencies':len(inputs),'all_dep_info_inputs_bound_before_build':not missing,'lexical_paths_requiring_resolution':lexical,'registry_package_checksums':records,'actual_toolchain_binaries':toolchain,'scope':'Post-build dependency closure crosscheck; source-before/source-after records bind actual supplied bytes. No solver execution or independent build attestation.','initial_probe':'A preliminary in-memory lexical-only set comparison refused ten literal ../ include paths. Resolving those paths finds all ten already present in SOURCE_BEFORE; no build/input changed. A subsequent optional .cargo-checksum.json probe found the current cache has no such files, so the retained check verifies original .crate archive checksums against Cargo.lock and every regular extracted source file against its archive bytes instead. The first archive-path construction had one parent too few and was corrected before the successful check.'}
(E/'DEPENDENCY_CROSSCHECK.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps({'dep_info_inputs':len(inputs),'registry_packages':len(records),'checks':'pass'}))
